extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops::{Rem};

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
        if self.coeffs.is_empty(){
            return 0
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
    

        
    fn generate_exceptional_set(){
        let exceptional_set = vec![];
        let modulus = BigInt::from(2);
        let generator = Polynomial::new(vec![BigInt::from(0),BigInt::from(1)]);
        for i in (1..2^)
    }
    
   
    
    

    
}

fn polynomial_long_division(dividend: &Polynomial, divisor: &Polynomial, modulus: &BigInt) -> (Polynomial, Polynomial) {
    println!("dividend: {:?}, divisor: {:?}", dividend.coeffs, divisor.coeffs);

    // Ensure that the division is only attempted if the dividend's degree is >= divisor's degree
    if dividend.degree() < divisor.degree() {
        return (Polynomial::new(vec![BigInt::zero()]), dividend.clone());
    }

    let mut quotient = Polynomial::new(vec![BigInt::zero(); dividend.degree() - divisor.degree() + 1]);
    let mut remainder = dividend.clone();
    let mut i = 0;

    while remainder.degree() >= divisor.degree() && !remainder.is_zero() && i <= 5  {
        let degree_diff = remainder.degree() - divisor.degree();

        // Leading coefficients
        let leading_coeff_remainder = remainder.coeffs.last().unwrap().clone();
        let leading_coeff_divisor = divisor.coeffs.last().unwrap().clone();

        // Compute the quotient coefficient: leading_coeff_remainder / leading_coeff_divisor (mod modulus)
        //let quotient_coeff = (leading_coeff_remainder.clone() * divisor.mod_inverse(modulus).unwrap()).rem(modulus);

        let quotient_coeff = (leading_coeff_remainder.clone() * mod_inverse(leading_coeff_divisor,modulus).unwrap()).rem(modulus);

        // Construct the quotient term (align the degree with degree_diff)
        let mut quotient_term_coeffs = vec![BigInt::zero(); degree_diff + 1];
        quotient_term_coeffs[degree_diff] = quotient_coeff.clone();
        let quotient_term = Polynomial::new(quotient_term_coeffs);

        println!("remainder: {:?}, Leading coeff remainder: {:?}, Leading coeff divisor: {:?}, Quotient coefficient: {:?}",
                 remainder.coeffs, leading_coeff_remainder, divisor.coeffs.last().unwrap(), quotient_coeff);

        // Update quotient
        quotient = quotient.add(&quotient_term, modulus);

        // Subtract the product of quotient term and divisor from remainder
        let subtrahend = divisor.mul(&quotient_term, modulus);
        remainder = remainder.sub_mod(&subtrahend, modulus);

        remainder.trim();

        println!("Updated remainder: {:?}", remainder.coeffs);
        i += 1;
    }

    println!("Final quotient: {:?}", quotient.coeffs);
    println!("Final remainder: {:?}", remainder.coeffs);

    (quotient, remainder)
}

/// Extended Euclidean algorithm to compute the GCD and coefficients
fn extended_euclidean(a: &Polynomial, b: &Polynomial, modulus: &BigInt) -> (Polynomial, Polynomial, Polynomial) {
    let mut s = Polynomial::new(vec![BigInt::zero()]);
    let mut old_s = Polynomial::new(vec![BigInt::one()]);
    let mut t = Polynomial::new(vec![BigInt::one()]);
    let mut old_t = Polynomial::new(vec![BigInt::zero()]);
    let mut r = b.clone();
    let mut old_r = a.clone();
    let i = 0;
    let mut j = 0;
    while !r.coeffs.is_empty() && r.coeffs[0] != BigInt::zero() && j<=10{
    //    let quotient = old_r.clone().rem(&r);
    println!("j:{:?}",j);
    println!("r_i: {:?}, r_i+1: {:?}", old_r.coeffs,r.coeffs);

        let (quotient,remainder) = polynomial_long_division(&old_r,&r, modulus);

        old_r = r;
        r = remainder;
        println!("after update. r_i: {:?}, r_i+1: {:?}", old_r.coeffs,r.coeffs);

        let new_s = old_s.sub_mod(&quotient.mul(&s, modulus), modulus);
        old_s = std::mem::replace(&mut s, new_s);

        let new_t = old_t.sub_mod(&quotient.mul(&t, modulus), modulus);
        old_t = std::mem::replace(&mut t, new_t);
        j += 1;
        println!("i:{:?}",i);
    }


    (old_r, old_t, old_s)
}

fn mod_inverse(divisor: BigInt, modulus: &BigInt) -> Option<BigInt> {
    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();
    let mut r = modulus.clone();
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
        Some(t.rem(modulus) + modulus)
    }
}

 
/// Find the inverse of a polynomial in the Galois ring (Z[X] / modulus)
fn find_inverse_in_galois_ring(elem: &Polynomial, modulus: &BigInt, irreducible: &Polynomial) -> Option<Polynomial> {
    let (g, u, _) = extended_euclidean(irreducible, elem, modulus);

    // If g is not 1, divide u by g
    if g.coeffs != vec![BigInt::one()] {
        let g_inv = mod_inverse(g.coeffs[0].clone(),modulus).unwrap(); 
            println!("t = {:?}, g_inv: {:?}",u.coeffs, g_inv);
            let inverse_coeffs: Vec<BigInt> = u.coeffs.iter().map(|coeff| (coeff *  &g_inv).rem(modulus)).collect();
            let inverse_poly = Polynomial::new(inverse_coeffs);
            return Some(inverse_poly);

    }

    // If g is 1, just return u mod irreducible
    Some(u)
}


fn main() {
    let modulus = BigInt::from(8);

    // Define the irreducible polynomial r(x) = x^4 + x + 1
    let irreducible = Polynomial::new(vec![
        BigInt::from(1), // Coefficient for x^0
        BigInt::from(1), // Coefficient for x^1
        BigInt::from(0), // Coefficient for x^2
        BigInt::from(0), // Coefficient for x^3
        BigInt::from(1), // Coefficient for x^4
    ]);

    // Define the polynomial to invert new_r(x) = x^2 + 1
    let divisor = Polynomial::new(vec![BigInt::from(1), BigInt::from(0), BigInt::from(1)]);
    let (remainder,quotient) = polynomial_long_division(&irreducible, &divisor, &modulus);
    /* 
    match find_inverse_in_galois_ring(&elem, &modulus, &irreducible) {
        Some(inverse) => println!("Inverse in Galois Ring: {:?}", inverse),
        None => println!("No inverse exists."),
    }
    */
   let (r,s,t) = extended_euclidean(&irreducible, &divisor, &modulus);
   println!("r: {:?}, t: {:?}, s:{:?}",r,s,t);
   let result = find_inverse_in_galois_ring(&divisor, &modulus, &irreducible);
   println!("{:?}", result);
}



  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_long_division() {
        // Define the modulus: 7
        let modulus = BigInt::from(7);

        // Define the dividend: x^3 + 2x^2 + 3x + 4
        let dividend = Polynomial::new(vec![
            BigInt::from(1), // Coefficient for x^0
            BigInt::from(1), // Coefficient for x^1
            BigInt::from(0), // Coefficient for x^2
            BigInt::from(0), // Coefficient for x^3
            BigInt::from(1), // Coefficient for x^2
        ]);

        // Define the divisor: x^2 + 1
        let divisor = Polynomial::new(vec![
            BigInt::from(1), // Coefficient for x^0
            BigInt::from(0), // Coefficient for x^1
            BigInt::from(1), // Coefficient for x^2
        ]);

        // Perform polynomial long division
        let (quotient, remainder) = polynomial_long_division(&dividend, &divisor, &modulus);

        // Expected quotient: x + 2
        let expected_quotient = Polynomial::new(vec![
            BigInt::from(6), // Coefficient for x^1
            BigInt::from(0), // Coefficient for x^0
            BigInt::from(1), // Coefficient for x^2
        ]);

        // Expected remainder: x + 2
        let expected_remainder = Polynomial::new(vec![
            BigInt::from(2), // Coefficient for x^0
            BigInt::from(1), // Coefficient for x^1
        ]);

        assert_eq!(quotient.coeffs, expected_quotient.coeffs, "Quotient does not match expected value");
        assert_eq!(remainder.coeffs, expected_remainder.coeffs, "Remainder does not match expected value");


    }
 

    #[test]
    fn test_extended_euclidean() {
        let modulus = BigInt::from(7);

        // Example polynomials: x^3 + 2x^2 + 3x + 4 and x^2 + 1
        let a = Polynomial::new(vec![
            BigInt::from(1), // Coefficient for x^0
            BigInt::from(1), // Coefficient for x^1
            BigInt::from(0), // Coefficient for x^2
            BigInt::from(1), // Coefficient for x^3
            BigInt::from(1), // Coefficient for x^3

        ]);
        let b = Polynomial::new(vec![
            BigInt::from(1), // Coefficient for x^0
            BigInt::from(0),  // Coefficient for x^1
            BigInt::from(1), // Coefficient for x^2
        ]);

        let (gcd, s, t) = extended_euclidean(&a, &b, &modulus);

        // Verify that s*a + t*b = gcd
        let left_side = s.mul(&a, &modulus).add(&t.mul(&b, &modulus), &modulus);
      //  assert_eq!(left_side.rem(&modulus), gcd.rem(&modulus), "s*a + t*b should equal gcd");

        // Verify that gcd is indeed the greatest common divisor
        assert!(gcd.degree() <= b.degree(), "GCD should have a degree less than or equal to the smaller polynomial");
    }

}
