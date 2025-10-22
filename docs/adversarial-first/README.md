# Paper 3: Adversarial-First Design

**A Framework for Building Systems That Are Secure by Default**

**Version:** 1.0
**Status:** Complete

---

## 1. Abstract

For decades, the prevailing security model has been "penetrate and patch"—a reactive stance where systems are built for functionality first and secured later. This paper introduces **Adversarial-First Design**, a paradigm that treats potential attacks as primary design constraints. We explore the theory of **hash-flooding denial-of-service attacks**, a common vector against naive hash table implementations. We then present a secure-by-design solution using a keyed hash function inspired by **SipHash**, implemented in our Rust `SecureHashMap`. Finally, we demonstrate the effectiveness of this defense in our `edge_simulation`, where the system successfully detects and mitigates a targeted collision attack.

---

## 2. The Missing Fundamental: Assume a Hostile World

In classical computer science, data structures are often analyzed under an assumption of benign, or at least random, inputs. A hash table, for example, is expected to perform in O(1) time on average, with the rare O(n) worst case being dismissed as astronomically unlikely.

This assumption is dangerously naive in a networked world. An adversary will not provide random inputs; they will craft the specific inputs needed to trigger the worst-case behavior. This leads to critical vulnerabilities:
-   A web server's core data structures can be crippled by a few carefully crafted HTTP requests.
-   A database can be slowed to a crawl by queries that exploit algorithmic weaknesses.
-   A DNS resolver can be taken offline by a flood of colliding domain lookups.

The missing fundamental is a design philosophy that **assumes all external inputs are potentially malicious** and builds in defenses from the start.

---

## 3. Foundational Research: Hash-Flooding and SipHash

A classic example of this vulnerability is the **hash-flooding (or hash-DoS) attack**. Most languages have historically used simple, deterministic hash functions for their hash tables. An attacker who knows the hash function can pre-compute a large number of keys that all hash to the same bucket. By flooding a server with requests involving these keys, they can degrade the hash table's performance from O(1) to O(n), effectively disabling the application.

To solve this, researchers Jean-Philippe Aumasson and Daniel J. Bernstein developed **SipHash**, a family of pseudorandom functions (PRFs) optimized for this exact scenario. The key insight of SipHash is to use a **secret, randomly generated key** to initialize the hash function.

With SipHash, the hash of a given input depends on this secret key. An attacker, not knowing the key, can no longer pre-compute collisions. The hash function is no longer a fixed target. This approach has become the industry standard and is now used in the hash table implementations of Rust, Python, Ruby, and many other languages.

---

## 4. Rust Implementation: `SecureHashMap`

Our `SecureHashMap` implements the core principles of SipHash to create a collision-resistant hash table.

### Core Mechanism: Randomized Hashing and Dynamic Defense

1.  **Random Seeding:** When a `SecureHashMap` is created, it initializes two secret seeds with cryptographically secure random numbers. These seeds act as the key for our hash function.

    ```rust
    // from src/adversarial_first.rs
    let mut rng = thread_rng();
    SecureHashMap {
        seed1: rng.gen(),
        seed2: rng.gen(),
        // ...
    }
    ```

2.  **Keyed Hash Function:** The `hash` method is a simplified implementation of the SipHash compression function. The result depends on both the input `key` and the secret seeds. An attacker without the seeds cannot predict the hash output.

3.  **Attack Detection and Rehashing:** As an additional layer of defense, the hash map monitors its own performance. If it detects that a single bucket's chain length has exceeded a threshold, it assumes an attack is underway. It then triggers the `rehash_with_new_seed` function, which generates new random seeds and completely re-builds the hash table. This invalidates the attacker's old collision keys and forces them to start over.

---

## 5. Verification and Demonstration

The effectiveness of this design is proven in our `edge_simulation`. The simulation includes a phase dedicated to a **targeted adversarial attack**:

1.  **Reconnaissance:** The simulation first uses the public `hash` method to "brute-force" a list of 50 unique keys that are guaranteed to collide into the same bucket. This simulates an attacker who has found a weakness or has the time to probe the system.

2.  **Attack:** The simulation then floods the server with tasks using these colliding keys.

The output of the simulation clearly shows the `SecureHashMap`'s defenses kicking in. After a few colliding keys are inserted, the console prints:
`🔄 Rehash #1 with new random seed`

This message confirms that the attack was detected and that the system automatically initiated its defensive protocol, demonstrating a robust, secure-by-design system in action.

---

### References

1.  Aumasson, J., and Bernstein, D. J. 2012. *SipHash: a fast short-input PRF*.
2.  Crosby, S. A., and Wallach, D. S. 2003. *Denial of Service via Algorithmic Complexity Attacks*. In *Proceedings of the 12th USENIX Security Symposium*.
