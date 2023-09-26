use std::collections::HashMap;

use ast::analyzed::{Expression, Identity, PolynomialReference, Reference};
use ast::parsed::visitor::ExpressionVisitable;

/// Checks if an expression is
/// - a polynomial
/// - not part of a polynomial array
/// - not shifted with `'`
/// and return the polynomial if so
pub fn try_to_simple_poly<T>(expr: &Expression<T>) -> Option<&PolynomialReference> {
    if let Expression::Reference(Reference::Poly(
        p @ PolynomialReference {
            index: None,
            next: false,
            ..
        },
    )) = expr
    {
        Some(p)
    } else {
        None
    }
}

pub fn try_to_simple_poly_ref<T>(expr: &Expression<T>) -> Option<&PolynomialReference> {
    if let Expression::Reference(Reference::Poly(poly_ref)) = expr {
        if poly_ref.index.is_none() && !poly_ref.next {
            return Some(poly_ref);
        }
        None
    } else {
        None
    }
}

pub fn is_simple_poly_of_name<T>(expr: &Expression<T>, poly_name: &str) -> bool {
    if let Expression::Reference(Reference::Poly(PolynomialReference {
        name,
        index: None,
        next: false,
        ..
    })) = expr
    {
        name == poly_name
    } else {
        false
    }
}

pub fn substitute_constants<T: Copy>(
    identities: &[Identity<T>],
    constants: &HashMap<String, T>,
) -> Vec<Identity<T>> {
    identities
        .iter()
        .cloned()
        .map(|mut identity| {
            identity.pre_visit_expressions_mut(&mut |e| {
                if let Expression::Constant(name) = e {
                    *e = Expression::Number(constants[name])
                }
            });
            identity
        })
        .collect()
}
