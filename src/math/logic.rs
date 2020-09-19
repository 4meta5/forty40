//! Boolean Logic Gates
#![allow(dead_code)]
/// Propositional logic
pub trait Logic {
    /// `AND`, which corresponds to the `all` operator.
    fn and(self, other: Self) -> Self;

    /// `OR`, which corresponds to the `any` operator.
    fn or(self, other: Self) -> Self;

    /// `NOT`, which corresponds to the `not` operator.
    fn not(self) -> Self;
}

impl Logic for bool {
    #[inline]
    fn and(self, other: Self) -> Self {
        self && other
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        self || other
    }

    #[inline]
    fn not(self) -> Self {
        !self
    }
}

impl Logic for Option<()> {
    #[inline]
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (a, b) if a.is_none() || b.is_none() => None,
            _ => Some(()),
        }
    }

    #[inline]
    fn or(self, other: Self) -> Self {
        match (self, other) {
            (a, b) if a.is_some() || b.is_some() => Some(()),
            _ => None,
        }
    }

    #[inline]
    fn not(self) -> Self {
        if self.is_none() {
            Some(())
        } else {
            None
        }
    }
}

/// An expression that can be evaluated
/// -> each Item is a three-valued logic where None represents Unknown
#[derive(Debug)]
pub struct Expression<'a, T: 'a + Sized + Logic> {
    pub(crate) expr: &'a [Option<T>; 5],
}

impl<T: Sized + Logic> Expression<'_, T> {
    /// An iterator over each predicate in the expression (all logics that are known)
    pub fn predicates(&self) -> impl Iterator<Item = &T> {
        self.expr.iter().filter_map(move |item| item.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bool_checks() {
        assert_eq!(true.and(false), false);
        assert_eq!(false.and(true), false);
        assert_eq!(true.or(false), true);
        assert_eq!(false.or(true), true);
        assert_eq!(true.not(), false);
        assert_eq!(false.not(), true);
    }
    #[test]
    fn option_checks() {
        assert_eq!(Some(()).and::<Option<()>>(None), None);
        assert_eq!(Some(()).or(None), Some(()));
        assert_eq!(None.or(Some(())), Some(()));
        assert_eq!(Some(()).not(), None);
        assert_eq!(None.not(), Some(()));
    }
}
