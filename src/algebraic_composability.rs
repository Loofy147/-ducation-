/// A struct representing a Monoid, an algebraic structure with a single associative binary operation and an identity element.
pub struct Monoid<T, F>
where
    F: Fn(T, T) -> T,
{
    /// The identity element of the monoid.
    identity: T,
    /// The binary operation of the monoid.
    operation: F,
}

impl<T, F> Monoid<T, F>
where
    T: Clone + PartialEq,
    F: Fn(T, T) -> T,
{
    /// Creates a new Monoid.
    pub fn new(identity: T, operation: F) -> Self {
        Monoid { identity, operation }
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
