use anyhow::Result;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::time::timeout;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use http_body_util::Full;
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use computational_fundamentals::{
    resource_aware::{Budgets, ResourceAwareScheduler, Task},
    self_modifying::SelfOptimizingCache,
    adversarial_first::SecureHashMap,
    uncertainty_quantification::UncertainValue,
    algebraic_composability::{TaskStats, task_stats_monoid},
};

const RATE_LIMIT_THRESHOLD: u64 = 100;
const BACKEND_TIMEOUT_MS: u64 = 50;

struct BackendServer {
    addr: SocketAddr,
    scheduler: ResourceAwareScheduler,
}

struct LoadBalancer {
    backends: Vec<BackendServer>,
    rate_limiter: SecureHashMap,
    cache: SelfOptimizingCache<String, String>,
    stats: TaskStats,
}

async fn handle_request(req: Request<Incoming>, balancer: Arc<Mutex<LoadBalancer>>) -> Result<Response<Full<Bytes>>> {
    let ip = req.headers().get("X-Forwarded-For").map_or("127.0.0.1", |h| h.to_str().unwrap());
    let path = req.uri().path().to_string();

    let mut balancer_guard = balancer.lock().await;
    let count = balancer_guard.rate_limiter.get(ip).map_or(0, |c| c.parse::<u64>().unwrap_or(0));

    if count >= RATE_LIMIT_THRESHOLD {
        println!("🚫 IP {} rate limited.", ip);
        let mut resp = Response::new(Full::new(Bytes::from("Too Many Requests")));
        *resp.status_mut() = StatusCode::TOO_MANY_REQUESTS;
        return Ok(resp);
    }

    balancer_guard.rate_limiter.set(ip, &(count + 1).to_string());

    if let Some(cached_response) = balancer_guard.cache.get(&path) {
        println!("⚡ Cache hit for {}", path);
        balancer_guard.stats = (task_stats_monoid().operation)(balancer_guard.stats.clone(), TaskStats { tasks_processed: 1, data_processed: 0.0 });
        return Ok(Response::new(Full::new(Bytes::from(cached_response.clone()))));
    }
    println!("Cache miss for {}", path);

    let task = Task {
        name: path.clone(),
        operations: UncertainValue::new(1e9, 1e8),
        data_size: 1e8,
        network: true,
        value: 10.0,
    };

    for backend in &mut balancer_guard.backends {
        if backend.scheduler.schedule_task(&task, 0.1) {
            println!("Forwarding to backend {}", backend.addr);

            let backend_response = timeout(Duration::from_millis(BACKEND_TIMEOUT_MS), async {
                tokio::time::sleep(Duration::from_millis(10)).await;
                Ok::<_, hyper::Error>(format!("Response for {} from backend {}", path, backend.addr))
            }).await;

            return match backend_response {
                Ok(Ok(body)) => {
                    balancer_guard.cache.put(path, body.clone());
                    balancer_guard.stats = (task_stats_monoid().operation)(balancer_guard.stats.clone(), TaskStats { tasks_processed: 1, data_processed: task.data_size });
                    Ok(Response::new(Full::new(Bytes::from(body))))
                },
                _ => {
                    println!("Backend {} timed out", backend.addr);
                    let mut resp = Response::new(Full::new(Bytes::from("Gateway Timeout")));
                    *resp.status_mut() = StatusCode::GATEWAY_TIMEOUT;
                    Ok(resp)
                }
            };
        }
    }

    let mut resp = Response::new(Full::new(Bytes::from("Service Unavailable")));
    *resp.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    let backends = vec![
        BackendServer {
            addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
            scheduler: ResourceAwareScheduler::new(Budgets { cpu: 10.0, energy: 100.0, memory: 1e9, bandwidth: 1e8 }),
        },
        BackendServer {
            addr: SocketAddr::from(([127, 0, 0, 1], 8081)),
            scheduler: ResourceAwareScheduler::new(Budgets { cpu: 10.0, energy: 100.0, memory: 1e9, bandwidth: 1e8 }),
        },
    ];

    let balancer = Arc::new(Mutex::new(LoadBalancer {
        backends,
        rate_limiter: SecureHashMap::new(),
        cache: SelfOptimizingCache::new(100),
        stats: task_stats_monoid().identity(),
    }));

    println!("Smart Load Balancer listening on http://{}", addr);
    println!("Press Ctrl+C to shut down.");

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let balancer = balancer.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| handle_request(req, balancer.clone())))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
