# The Seven Missing Computational Fundamentals: A Rust Implementation

This repository provides production-ready Rust implementations and detailed research papers for seven fundamental principles of modern computing that are often overlooked in classical computer science.

This project is not just a collection of algorithms; it is a demonstration of how to build systems that are **robust, adaptive, secure, and intelligent** by design. It includes both a `verification_suite` to test each principle in isolation and a comprehensive `edge_simulation` to show how they work together in a realistic, production-style scenario.

---

## The Seven Fundamentals

1.  **[Time-Aware Computing](./docs/time-aware/README.md)**
2.  **[Resource-Aware Computing](./docs/resource-aware/README.md)**
3.  **[Adversarial-First Design](./docs/adversarial-first/README.md)**
4.  **[Algebraic Composability](./docs/algebraic-composability/README.md)**
5.  **[Uncertainty Quantification](./docs/uncertainty-quantification/README.md)**
6.  **[Self-Modifying Algorithms](./docs/self-modifying/README.md)**
7.  **[Causal Reasoning](./docs/causal-reasoning/README.md)**

---

## Getting Started

This project is a standard Rust crate with two binaries and a library.

### Running the Test Suite

The project includes a comprehensive test suite that verifies the correctness of each fundamental.

```bash
cargo test
```

### Building the Documentation

The library is fully documented. To build and view the documentation, run:

```bash
cargo doc --open
```

### Running the Edge Server Simulation

The `edge_simulation` is a comprehensive, integrated simulation that shows all seven principles working together in a realistic, production-style scenario.

```bash
cargo run --bin edge_simulation
```

### Running the Smart Load Balancer

The `smart_balancer` is a real, working HTTP load balancer that uses the seven fundamentals to make intelligent routing and security decisions.

```bash
cargo run --bin smart_balancer
```
Once running, you can send requests to it:
```bash
curl http://127.0.0.1:3000/some/path
```

---

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

Before contributing, please ensure that all tests pass and that the code is formatted correctly:

```bash
cargo test
cargo fmt
```
