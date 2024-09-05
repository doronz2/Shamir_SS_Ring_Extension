extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use std::{f64::consts::E, ops::Rem, vec};
use rand::Rng;

#[derive(Debug, Clone)]
struct Polynomial {
    coeffs: Vec<BigInt>,
}

// Implement PartialEq for Polynomial
impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        if self.coeffs.len() != other.coeffs.len() {
            return false;
        }
        for (a, b) in self.coeffs.iter().zip(other.coeffs.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

impl Polynomial {
    fn new(coeffs: Vec<BigInt>) -> Self {
        Polynomial { coeffs }
    }

    fn degree(&self) -> usize {
        if self.coeffs.is_empty() {
            return 0;
        }
        self.coeffs.len() - 1
    }

    fn trim(&mut self) {
        while let Some(true) = self.coeffs.last().map(|x| x.is_zero()) {
            self.coeffs.pop();
        }
    }

    fn add(&self, other: &Polynomial, modulus: &BigInt) -> Polynomial {
        let mut result = vec![BigInt::zero(); self.coeffs.len().max(other.coeffs.len())];
        let zero = BigInt::zero(); // Avoid temporary value issues

        for i in 0..result.len() {
            let a = if i < self.coeffs.len() { &self.coeffs[i] } else { &zero };
            let b = if i < other.coeffs.len() { &other.coeffs[i] } else { &zero };

            result[i] = (a + b).rem(modulus).clone();
        }

        Polynomial::new(result)
    }

    fn sub_mod(&self, other: &Polynomial, modulus: &BigInt) -> Polynomial {
        let mut result = vec![BigInt::zero(); self.coeffs.len().max(other.coeffs.len())];
        let zero = BigInt::zero(); // Avoid temporary value issues

        for i in 0..result.len() {
            let a = if i < self.coeffs.len() { &self.coeffs[i] } else { &zero };
            let b = if i < other.coeffs.len() { &other.coeffs[i] } else { &zero };

            result[i] = (a - b).rem(modulus).clone();
            if result[i].sign() == num_bigint::Sign::Minus {
                result[i] += modulus.clone();
            }
        }

        Polynomial::new(result)
    }

    fn mul(&self, other: &Polynomial, modulus: &BigInt) -> Polynomial {
        let mut result_coeffs = vec![BigInt::zero(); self.degree() + other.degree() + 1];

        for i in 0..=self.degree() {
            for j in 0..=other.degree() {
                let mut result = result_coeffs[i + j].clone(); // Clone the current value
                result += &self.coeffs[i] * &other.coeffs[j];
                result_coeffs[i + j] = result.rem(modulus).clone(); // Apply the modulus and store back
            }
        }

        Polynomial::new(result_coeffs)
    }

    /// Check if a polynomial is zero.
    pub fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|c| c.is_zero())
    }
}

#[derive(Debug, Clone)]
struct GaloisRing {
    modulus: BigInt,
    irreducible: Polynomial,
}

impl GaloisRing {
    fn mul_ring(&self, poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {
        let length = poly1.coeffs.len();
    //  assert_eq!(poly2.coeffs.len(), length);
        if poly1.is_zero() || poly2.is_zero(){
            let zero_poly = vec![BigInt::zero(); length]; 
            return Polynomial::new(zero_poly);
        }
        let mut result_coeffs = vec![BigInt::zero(); poly1.degree() + poly2.degree() + 1];
        
        for i in 0..=poly1.degree() {
            for j in 0..=poly2.degree() {
                let mut result = result_coeffs[i + j].clone(); // Clone the current value
                result += &poly1.coeffs[i] * &poly2.coeffs[j];
                result_coeffs[i + j] = result.rem(&self.modulus).clone(); // Apply the modulus and store back
            }
        }
        let poly_result = Polynomial::new(result_coeffs);
        let (_, result_coeffs_ring) = self.polynomial_long_division(&poly_result, &self.irreducible);
        Polynomial::new(result_coeffs_ring.coeffs)
    }

    fn power_in_ring(&self, poly: &Polynomial, exponent: usize) -> Polynomial {
        if exponent == 0 {
            return Polynomial::new(vec![BigInt::from(1)]);
        }
        let mut res_power = poly.clone();
        for _ in 1..exponent {
            res_power = self.mul_ring(&res_power, poly);
        }
        res_power
    }

    fn polynomial_long_division(&self, dividend: &Polynomial, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        if dividend.degree() < divisor.degree() {
            return (
                Polynomial::new(vec![BigInt::zero()]),
                dividend.clone(),
            );
        }

        let mut quotient = Polynomial::new(vec![BigInt::zero(); dividend.degree() - divisor.degree() + 1]);
        let mut remainder = dividend.clone();

        while remainder.degree() >= divisor.degree() && !remainder.is_zero() {
            let degree_diff = remainder.degree() - divisor.degree();
            let leading_coeff_remainder = remainder.coeffs.last().unwrap().clone();
            let leading_coeff_divisor = divisor.coeffs.last().unwrap().clone();

            let quotient_coeff = (leading_coeff_remainder.clone() * self.mod_inverse(leading_coeff_divisor).unwrap()).rem(&self.modulus);

            let mut quotient_term_coeffs = vec![BigInt::zero(); degree_diff + 1];
            quotient_term_coeffs[degree_diff] = quotient_coeff.clone();
            let quotient_term = Polynomial::new(quotient_term_coeffs);

            quotient = quotient.add(&quotient_term, &self.modulus);

            let subtrahend = divisor.mul(&quotient_term, &self.modulus);
            remainder = remainder.sub_mod(&subtrahend, &self.modulus);
            remainder.trim();
        }

        (quotient, remainder)
    }

    fn extended_euclidean(&self, a: &Polynomial, b: &Polynomial) -> (Polynomial, Polynomial, Polynomial) {
        let mut s = Polynomial::new(vec![BigInt::zero()]);
        let mut old_s = Polynomial::new(vec![BigInt::one()]);
        let mut t = Polynomial::new(vec![BigInt::one()]);
        let mut old_t = Polynomial::new(vec![BigInt::zero()]);
        let mut r = b.clone();
        let mut old_r = a.clone();

        while !r.coeffs.is_empty() && r.coeffs[0] != BigInt::zero() {
            let (quotient, remainder) = self.polynomial_long_division(&old_r, &r);

            old_r = r;
            r = remainder;

            let new_s = old_s.sub_mod(&quotient.mul(&s, &self.modulus), &self.modulus);
            old_s = std::mem::replace(&mut s, new_s);

            let new_t = old_t.sub_mod(&quotient.mul(&t, &self.modulus), &self.modulus);
            old_t = std::mem::replace(&mut t, new_t);
        }

        (old_r, old_t, old_s)
    }

    fn mod_inverse(&self, divisor: BigInt) -> Option<BigInt> {
        let mut t = BigInt::zero();
        let mut new_t = BigInt::one();
        let mut r = self.modulus.clone();
        let mut new_r = divisor.clone();

        while !new_r.is_zero() {
            let quotient = &r / &new_r;
            let temp_t = t.clone();
            t = new_t.clone();
            new_t = temp_t - &quotient * &new_t;

            let temp_r = r.clone();
            r = new_r.clone();
            new_r = temp_r - &quotient * &new_r;
        }

        if r > BigInt::one() {
            None
        } else {
            Some((t.rem(&self.modulus) + &self.modulus) % &self.modulus)
        }
    }

    fn find_inverse_in_galois_ring(&self, elem: &Polynomial) -> Option<Polynomial> {
        let (g, u, _) = self.extended_euclidean(&self.irreducible, elem);

        if g.coeffs != vec![BigInt::one()] {
            let g_inv = self.mod_inverse(g.coeffs[0].clone()).unwrap();
            let inverse_coeffs: Vec<BigInt> = u.coeffs.iter().map(|coeff| (coeff * &g_inv).rem(&self.modulus)).collect();
            let inverse_poly = Polynomial::new(inverse_coeffs);
            return Some(inverse_poly);
        }

        Some(u)
    }
}

fn generate_exceptional_set(irreducible: &Polynomial) -> Vec<Polynomial> {
    let mut exceptional_set = Vec::new();

    let max_value = 2_u64.pow(irreducible.degree() as u32);
    for i in 0..max_value {
        let mut coeffs = Vec::new();
        let mut value = i;

        for _ in 0..irreducible.degree() {
            coeffs.push(BigInt::from(value % 2));
            value /= 2;
        }

        exceptional_set.push(Polynomial::new(coeffs));
    }

    exceptional_set
}

fn random_ring_element(modulus: &BigInt, irreducible: &Polynomial) -> Polynomial {
    let d = irreducible.degree();
    let coeffs: Vec<BigInt> = (0..d).map(|_| rand::thread_rng().gen_bigint_range(&BigInt::from(0), modulus)).collect();
    Polynomial { coeffs }
}

fn generate_random_polynomial_with_secret(secret: Polynomial, ring: &GaloisRing) -> Vec<Polynomial> {
    let mut rand_poly: Vec<Polynomial> = vec![Polynomial::new(secret.coeffs)];
    for _ in 0..ring.irreducible.degree() - 1 {
        rand_poly.push(random_ring_element(&ring.modulus, &ring.irreducible));
    }
    rand_poly
}

fn evaluate_polynomial(point: &Polynomial, rand_polynomial_ring: Vec<Polynomial>, ring: &GaloisRing) -> Polynomial {
    let mut eval_poly = Polynomial::new(vec![]);
    for i in 1..ring.irreducible.degree() {
        eval_poly = eval_poly.add(&ring.mul_ring(&rand_polynomial_ring[i], &ring.power_in_ring(point, i)), &ring.modulus);
        println!("1: evaluated poly {:?}", eval_poly);
    }
    eval_poly
}

fn shamir_secret_sharing(secret: Polynomial, number_of_parties: usize, ring: &GaloisRing) -> Vec<(Polynomial, Polynomial)> {
    println!("{:?},{:?}", number_of_parties, 2_usize.pow(ring.irreducible.degree() as u32 -1 ));
    assert!(number_of_parties >= 2_usize.pow(ring.irreducible.degree() as u32) - 1);
    let random_polynomial_ring = generate_random_polynomial_with_secret(secret, ring);
    let mut shares: Vec<(Polynomial, Polynomial)> = vec![];
    let points = generate_exceptional_set(&ring.irreducible);

    for i in 0..number_of_parties {
        let evaluated_element = evaluate_polynomial(&points[i], random_polynomial_ring.clone(), ring);
        shares.push((points[i].clone(), evaluated_element.clone()));
        println!("evalluated: {:?}", evaluated_element.clone());
    }
    shares
}

fn reconstruct_secret(shares: Vec<(Polynomial, Polynomial)>, ring: &GaloisRing) -> Polynomial {
    let mut res = Polynomial::new(vec![BigInt::zero()]);
    for (xi, yi) in shares.clone() {
        let mut li = Polynomial::new(vec![BigInt::from(1)]);
        for (xj, _) in shares.clone() {
            if xi != xj {
                let numerator = xj.clone();
                let denominator = &xj.sub_mod(&xi, &ring.modulus);
                let denominator_inv = ring.find_inverse_in_galois_ring(denominator).unwrap();
                let frac = ring.mul_ring(&numerator, &denominator_inv);
                li = ring.mul_ring(&li, &frac);
            }
        }
        res = res.add(&ring.mul_ring(&yi, &li), &ring.modulus);
    }
    res
}

fn main() {
    let modulus = BigInt::from(7);
    let irreducible = Polynomial::new(vec![
        BigInt::from(1),
        BigInt::from(1),
        BigInt::from(0),
        BigInt::from(0),
        BigInt::from(1),
    ]);

    let number_of_parties = 2_usize.pow(irreducible.degree() as u32);
    println!("number of parties: {:?}", number_of_parties);
    let ring = GaloisRing { modulus, irreducible };

    // Example usage
    let secret = random_ring_element(&ring.modulus, &ring.irreducible);
    println!("secret: {:?}", secret);

    let shares = shamir_secret_sharing(secret.clone(), number_of_parties, &ring);
    println!("shares: {:?}",shares);

    let reconstructed_secret = reconstruct_secret(shares, &ring);
    println!("Original secret: {:?}, Reconstructed secret: {:?}", secret, reconstructed_secret);
}
