extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone)]
struct Polynomial {
    coeffs: Vec<BigInt>,
}

impl Polynomial {
    fn new(coeffs: Vec<BigInt>) -> Self {
        Polynomial { coeffs }
    }

    fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    fn trim(&mut self) {
        while let Some(true) = self.coeffs.last().map(|x| x.is_zero()) {
            self.coeffs.pop();
        }
    }

    fn change_ring(&self, modulus: &BigInt) -> Polynomial {
        let coeffs = self
            .coeffs
            .iter()
            .map(|c| c.rem(modulus).clone())
            .collect();
        Polynomial::new(coeffs)
    }

    fn sub_mod(&self, other: &Polynomial, modulus: &BigInt) -> Polynomial {
        let mut result = vec![BigInt::zero(); self.coeffs.len().max(other.coeffs.len())];

        for i in 0..result.len() {
            let zero = BigInt::zero();
            let a = if i < self.coeffs.len() { &self.coeffs[i] } else { &zero };
            let b = if i < other.coeffs.len() { &other.coeffs[i] } else { &zero };

            result[i] = (a - b).rem(modulus).clone();
            if result[i].sign() == num_bigint::Sign::Minus {
                result[i] += modulus.clone();
            }
        }

        Polynomial::new(result)
    }

    fn mul(&self, other: &Polynomial) -> Polynomial {
        let mut result_coeffs = vec![BigInt::zero(); self.degree() + other.degree() + 1];

        for i in 0..=self.degree() {
            for j in 0..=other.degree() {
                result_coeffs[i + j] += &self.coeffs[i] * &other.coeffs[j];
            }
        }

        Polynomial::new(result_coeffs)
    }

    fn rem(&self, modulus: &Polynomial) -> Polynomial {
        let mut result = self.clone();

        while result.degree() >= modulus.degree() {
            let degree_diff = result.degree() - modulus.degree();
            let leading_coeff = result.coeffs.last().unwrap().clone();

            for i in 0..=modulus.degree() {
                result.coeffs[degree_diff + i] -= &leading_coeff * &modulus.coeffs[i];
            }

            result.trim();
        }

        result
    }
}

/// Extended Euclidean algorithm to compute the GCD and coefficients
fn extended_euclidean(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let (mut s, mut old_s) = (BigInt::zero(), BigInt::one());
    let (mut t, mut old_t) = (BigInt::one(), BigInt::zero());
    let (mut r, mut old_r) = (b.clone(), a.clone());

    while !r.is_zero() {
        let quotient = &old_r / &r;
        old_r = std::mem::replace(&mut r, &old_r - &quotient * &r);
        old_s = std::mem::replace(&mut s, &old_s - &quotient * &s);
        old_t = std::mem::replace(&mut t, &old_t - &quotient * &t);
    }

    (old_r, old_s, old_t)
}

/// Find the inverse of a polynomial in the Galois ring (Z[X] / modulus)
fn find_inverse_in_galois_ring(elem: &Polynomial, modulus: &Polynomial, irreducible: &Polynomial) -> Option<Polynomial> {
    let (g, u, _) = extended_euclidean(&elem.coeffs[0], &modulus.coeffs[0]);

    let inverse_poly = if !g.is_one() {
        // If g is not 1, we need to divide u by g
        let inverse_coeffs = u.div(&g).rem(&modulus.coeffs[0]);
        Polynomial::new(vec![inverse_coeffs])
    } else {
        // If g is 1, u is already the inverse
        let inverse_coeffs = u.rem(&modulus.coeffs[0]);
        Polynomial::new(vec![inverse_coeffs])
    };

    // Embed in the Galois ring by reducing modulo the irreducible polynomial
    Some(inverse_poly.rem(irreducible))
}

fn main() {
    let modulus = BigInt::from(7);

    // Define the irreducible polynomial r(x) = x^4 + x + 1
    let irreducible = Polynomial::new(vec![
        BigInt::from(1), // Coefficient for x^0
        BigInt::from(0), // Coefficient for x^1
        BigInt::from(0), // Coefficient for x^2
        BigInt::from(1), // Coefficient for x^3
        BigInt::from(1), // Coefficient for x^4
    ]);

    // Define the polynomial to invert new_r(x) = x^2 + 1
    let elem = Polynomial::new(vec![BigInt::from(3), BigInt::from(0), BigInt::from(1)]);
    
    match find_inverse_in_galois_ring(&elem, &modulus, &irreducible) {
        Some(inverse) => println!("Inverse in Galois Ring: {:?}", inverse),
        None => println!("No inverse exists."),
    }
}
