extern crate ff;
use ff::{Field, PrimeField, FieldExt, Polynomial};
use num_bigint::BigInt;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct GF2dField {
    poly: Polynomial,
    modulus: Polynomial,
}

impl GF2dField {
    fn new(modulus: Polynomial) -> Self {
        GF2dField {
            poly: Polynomial::zero(),
            modulus,
        }
    }

    fn add(&self, other: &Self) -> Self {
        GF2dField {
            poly: &self.poly + &other.poly,
            modulus: self.modulus.clone(),
        }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut result = &self.poly * &other.poly;
        result = result % &self.modulus;
        GF2dField {
            poly: result,
            modulus: self.modulus.clone(),
        }
    }
}

fn generate_exceptional_set(d: usize, modulus: Polynomial) -> HashSet<GF2dField> {
    let mut exceptional_set = HashSet::new();
    let max_value = 1 << d;
    
    for i in 1..max_value {
        let poly = Polynomial::from_coeffs((0..d).map(|j| ((i >> j) & 1).into()).collect());
        let element = GF2dField::new(poly.clone());
        exceptional_set.insert(element);
    }
    exceptional_set
}

fn main() {
    // Define the irreducible polynomial for the field extension GF(2^d)
    let irreducible_poly = Polynomial::from_coeffs(vec![1, 0, 1, 1]); // Example for x^3 + x + 1
    
    let d = 3; // Degree of the extension
    let exceptional_set = generate_exceptional_set(d, irreducible_poly);
    
    // Print the exceptional set
    for elem in exceptional_set {
        println!("{:?}", elem);
    }
}
