//! Assignment 6: Mastering advanced types (1/2).
//!
//! The primary goal of this assignment is to understand generics, traits, and lifetimes.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-06.sh` works fine.
//! See `assignment06_grade.rs` and `/scripts/grade-06.sh` for the test script.

use std::{collections::HashMap, fmt::Debug};

/// Semiring.
///
/// Consult <https://en.wikipedia.org/wiki/Semiring>.
pub trait Semiring: Debug + Clone + PartialEq {
    /// Additive identity.
    fn zero() -> Self;
    /// Multiplicative identity.
    fn one() -> Self;
    /// Addition operation.
    fn add(&self, rhs: &Self) -> Self;
    /// Multiplication operation.
    fn mul(&self, rhs: &Self) -> Self;
}

/// Converts integer to semiring value.
pub fn from_usize<T: Semiring>(value: usize) -> T {
    let mut result = T::zero();
    let one = T::one();

    for _ in 0..value {
        result = T::add(&result, &one);
    }

    result
}

impl Semiring for u64 {
    fn zero() -> Self {
        0_u64
    }

    fn one() -> Self {
        1_u64
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Semiring for i64 {
    fn zero() -> Self {
        0_i64
    }

    fn one() -> Self {
        1_i64
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Semiring for f64 {
    fn zero() -> Self {
        0_f64
    }

    fn one() -> Self {
        1_f64
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

/// Polynomials with coefficient in `C`.
///
/// For example, polynomial `x^2 + 5x + 6` is represented in `Polynomial<u64>` as follows:
///
/// ```
/// Polynomial {
///     coefficients: {
///         2: 1,
///         1: 5,
///         0: 6,
///     },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial<C: Semiring> {
    coefficients: HashMap<u64, C>,
}

impl<C: Semiring> Semiring for Polynomial<C> {
    fn zero() -> Self {
        Self {
            coefficients: HashMap::new(),
        }
    }

    fn one() -> Self {
        let mut map = HashMap::new();
        let _unuse = map.insert(0_u64, C::one());
        Self { coefficients: map }
    }

    fn add(&self, rhs: &Self) -> Self {
        let mut map = self.coefficients.clone();
        for (k, v) in &rhs.coefficients {
            let entry = map.entry(*k).or_insert_with(C::zero);
            *entry = v.add(entry);
        }
        map.retain(|_, v| *v != C::zero());
        Self { coefficients: map }
    }

    fn mul(&self, rhs: &Self) -> Self {
        let mut pol = Self {
            coefficients: HashMap::new(),
        };
        for (k, v) in &rhs.coefficients {
            let mut inner_map = HashMap::new();
            for key in self.coefficients.keys() {
                let entry = self.coefficients.get(key).unwrap();
                let _unuse = inner_map.insert(k + key, v.mul(entry));
            }
            pol = pol.add(&Self {
                coefficients: inner_map,
            });
        }
        pol.coefficients.retain(|_, v| *v != C::zero());
        pol
    }
}

impl<C: Semiring> From<C> for Polynomial<C> {
    fn from(value: C) -> Self {
        let mut map = HashMap::new();
        let _unuse = map.insert(0_u64, value);
        Self { coefficients: map }
    }
}

impl<C: Semiring> Polynomial<C> {
    /// Constructs polynomial `x`.
    pub fn x() -> Self {
        let mut map = HashMap::new();
        let _unused = map.insert(1_u64, C::one());
        Self { coefficients: map }
    }

    /// Evaluates the polynomial with the given value.
    pub fn eval(&self, value: C) -> C {
        let mut out = C::zero();
        for (k, v) in &self.coefficients {
            let mut inner = C::one();
            for _ in 0..*k {
                inner = inner.mul(&value);
            }
            inner = inner.mul(v);
            out = out.add(&inner);
        }
        out
    }
}

struct FindIter<'s, T: Eq> {
    query: &'s [T],
    base: &'s [T],
    curr: usize,
}

impl<T: Eq> Iterator for FindIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.curr;
        while i + self.query.len() <= self.base.len() {
            let mut ans = 0;
            for j in 0..self.query.len() {
                if self.query[j] != self.base[i + j] {
                    break;
                } else {
                    ans += 1;
                }
            }
            if ans == self.query.len() {
                break;
            }
            i += 1;
        }
        self.curr = i + 1;
        if i + self.query.len() > self.base.len() {
            None
        } else {
            Some(i)
        }
    }
}

/// Returns an iterator over substring query indexes in the base.
pub fn find<'s, T: Eq>(query: &'s [T], base: &'s [T]) -> impl 's + Iterator<Item = usize> {
    FindIter {
        query,
        base,
        curr: 0,
    }
}
