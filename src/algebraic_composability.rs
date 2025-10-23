/// A struct representing a Monoid, an algebraic structure with a single associative binary operation and an identity element.
pub struct Monoid<T, F>
where
    F: Fn(T, T) -> T,
{
    identity: T,
    /// The binary operation of the monoid.
    pub operation: F,
}

impl<T, F> Monoid<T, F>
where
    T: Clone + PartialEq,
    F: Fn(T, T) -> T,
{
    /// Creates a new `Monoid`.
    pub fn new(identity: T, operation: F) -> Self {
        Monoid {
            identity,
            operation,
        }
    }

    /// Returns the identity element of the monoid.
    pub fn identity(&self) -> T {
        self.identity.clone()
    }

    /// Checks if the identity law holds for a given set of values.
    /// The identity law states that for any element `x`, `op(identity, x) == x` and `op(x, identity) == x`.
    pub fn check_identity_law(&self, values: &[T]) -> bool {
        values.iter().all(|v| {
            let left_identity = (self.operation)(self.identity.clone(), v.clone());
            let right_identity = (self.operation)(v.clone(), self.identity.clone());
            &left_identity == v && &right_identity == v
        })
    }

    /// Checks if the associativity law holds for a given set of values.
    /// The associativity law states that for any elements `x`, `y`, and `z`, `op(op(x, y), z) == op(x, op(y, z))`.
    pub fn check_associativity_law(&self, values: &[T]) -> bool {
        if values.len() < 3 {
            return true;
        }
        values.windows(3).all(|w| {
            let a = w[0].clone();
            let b = w[1].clone();
            let c = w[2].clone();
            let left_assoc = (self.operation)((self.operation)(a.clone(), b.clone()), c.clone());
            let right_assoc = (self.operation)(a, (self.operation)(b, c));
            left_assoc == right_assoc
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
/// A struct to hold statistics about processed tasks.
pub struct TaskStats {
    /// The number of tasks processed.
    pub tasks_processed: u64,
    /// The amount of data processed.
    pub data_processed: f64,
}

/// Creates a `Monoid` for `TaskStats`.
pub fn task_stats_monoid() -> Monoid<TaskStats, fn(TaskStats, TaskStats) -> TaskStats> {
    Monoid::new(
        TaskStats {
            tasks_processed: 0,
            data_processed: 0.0,
        },
        |a, b| TaskStats {
            tasks_processed: a.tasks_processed + b.tasks_processed,
            data_processed: a.data_processed + b.data_processed,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_algebraic_laws() {
        let sum_monoid = Monoid::new(0, |a, b| a + b);
        let sum_values = vec![1, 2, 3, 4, 5];
        let sum_identity = sum_monoid.check_identity_law(&sum_values);
        let sum_associativity = sum_monoid.check_associativity_law(&sum_values);
        assert!(sum_identity, "Sum monoid should satisfy the identity law");
        assert!(sum_associativity, "Sum monoid should satisfy the associativity law");

        let list_monoid = Monoid::new(Vec::<i32>::new(), |mut a, mut b| { a.append(&mut b); a });
        let list_values = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
        let list_identity = list_monoid.check_identity_law(&list_values);
        let list_associativity = list_monoid.check_associativity_law(&list_values);
        assert!(list_identity, "List monoid should satisfy the identity law");
        assert!(list_associativity, "List monoid should satisfy the associativity law");
    }
}
