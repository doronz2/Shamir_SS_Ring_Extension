extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use core::num;
use std::{arch::aarch64::int8x16_t, ops::Rem, vec};
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

    fn trimmed_poly(&self)->Polynomial {
        let mut divisor_poly = Polynomial::new(self.coeffs.clone());
        while let Some(true) = divisor_poly.coeffs.last().map(|x| x.is_zero()) {
            divisor_poly.coeffs.pop();
        }
        return divisor_poly
    }
    


        /// Check if a polynomial is zero.
        pub fn is_zero(&self) -> bool {
            self.coeffs.iter().all(|c| c.is_zero())
        }

}

#[derive(Debug, Clone)]
struct GaloisRing {
            modulus: BigInt,
            irreducible: Polynomial
}
        

impl GaloisRing{


    
    fn new(modulus: BigInt, irreducible: Polynomial) -> Self {
        GaloisRing {modulus, irreducible}
    }

    fn add(&self, poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {
        let mut result = vec![BigInt::zero(); poly1.coeffs.len().max(poly2.coeffs.len())];
        let zero = BigInt::zero(); // Avoid temporary value issues

        for i in 0..result.len() {
            let a = if i < poly1.coeffs.len() { &poly1.coeffs[i] } else { &zero };
            let b = if i < poly2.coeffs.len() { &poly2.coeffs[i] } else { &zero };

            result[i] = (a + b).rem(&self.modulus).clone();
        }

        Polynomial::new(result)
    }


    fn add_ring(&self, poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {
        let mut result = vec![BigInt::zero(); poly1.coeffs.len().max(poly2.coeffs.len())];
        let zero = BigInt::zero(); // Avoid temporary value issues

        for i in 0..result.len() {
            let a = if i < poly1.coeffs.len() { &poly1.coeffs[i] } else { &zero };
            let b = if i < poly2.coeffs.len() { &poly2.coeffs[i] } else { &zero };

            result[i] = (a + b).rem(&self.modulus).clone();
        }

        let poly_result = Polynomial::new(result);
        let (_,result_coeffs_ring) = self.polynomial_long_division( &poly_result, &self.irreducible);

        Polynomial::new(result_coeffs_ring.coeffs)
    }

    fn sub_mod(&self, poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {
        let mut result = vec![BigInt::zero(); poly1.coeffs.len().max(poly2.coeffs.len())];
        let zero = BigInt::zero(); // Avoid temporary value issues

        for i in 0..result.len() {
            let a = if i < poly1.coeffs.len() { &poly1.coeffs[i] } else { &zero };
            let b = if i < poly2.coeffs.len() { &poly2.coeffs[i] } else { &zero };

            result[i] = (a - b).rem(&self.modulus).clone();
            if result[i].sign() == num_bigint::Sign::Minus {
                result[i] += self.modulus.clone();
            }
        }

        Polynomial::new(result)
    }


    fn mul(&self, poly1: &Polynomial, poly2: &Polynomial) -> Polynomial {
        let mut result_coeffs = vec![BigInt::zero(); poly1.degree() + poly2.degree() + 1];

        for i in 0..=poly1.degree() {
            for j in 0..=poly2.degree() {
                let mut result = result_coeffs[i + j].clone(); // Clone the current value
                result += &poly1.coeffs[i] * &poly2.coeffs[j];
                result_coeffs[i + j] = result.rem(&self.modulus).clone(); // Apply the modulus and store back
            }
        }

        Polynomial::new(result_coeffs)
    }
    
        fn mul_ring(&self, poly1:  &Polynomial, poly2: &Polynomial) -> Polynomial {
            let mut result_coeffs = vec![BigInt::zero(); poly1.degree() + poly2.degree() + 1];
    
            for i in 0..=poly1.degree() {
                for j in 0..=poly2.degree() {
                    let mut result = result_coeffs[i + j].clone(); // Clone the current value
                    result += &poly1.coeffs[i] * &poly2.coeffs[j];
                    result_coeffs[i + j] = result.rem(&self.modulus).clone(); // Apply the modulus and store back
                }
            }
            let poly_result = Polynomial::new(result_coeffs);
            let (_,result_coeffs_ring) = self.polynomial_long_division( &poly_result, &self.irreducible);
            return Polynomial::new(result_coeffs_ring.coeffs);
        }

    

        fn power_in_ring(&self, poly:  &Polynomial, exponent:usize)-> Polynomial{
            if exponent == 0 {
                return Polynomial::new(vec![BigInt::from(1)]);
            }

            if (poly.is_zero()){
                return GaloisRing::zero();
            }
            
            
            let mut res_power = poly.clone();
                for _ in 1..exponent{
                    res_power = self.mul_ring(&res_power, &poly);
            }
            res_power
        }








fn zero()-> Polynomial{
    Polynomial::new(vec![BigInt::from(0)])
}

fn polynomial_long_division(&self, dividend: &Polynomial, divisor: &Polynomial) -> (Polynomial, Polynomial) {
   // println!("dividend: {:?}, divisor: {:?}", dividend.coeffs, divisor.coeffs);

    // Ensure that the division is only attempted if the dividend's degree is >= divisor's degree
    
    let divisor_trimmed= divisor.trimmed_poly(); //in case the leasing coeeficient/s is 0
   // println!("divisor {:?}, divisor trimmed {:?}", divisor.coeffs, divisor_trimmed.coeffs);


    if dividend.degree() < divisor_trimmed.degree() {
        return (Polynomial::new(vec![BigInt::zero()]), dividend.clone());
    }


    let mut quotient = Polynomial::new(vec![BigInt::zero(); dividend.degree() - divisor_trimmed.degree() + 1]);
    let mut remainder = dividend.clone();
    let mut i = 0;

    while remainder.degree() >= divisor_trimmed.degree() && !remainder.is_zero() && i <= 5  {
        let degree_diff = remainder.degree() - divisor_trimmed.degree();

        // Leading coefficients
        let leading_coeff_remainder = remainder.coeffs.last().unwrap().clone();
        let leading_coeff_divisor = divisor_trimmed.coeffs.last().unwrap().clone();

        // Compute the quotient coefficient: leading_coeff_remainder / leading_coeff_divisor (mod modulus)
        //let quotient_coeff = (leading_coeff_remainder.clone() * divisor.mod_inverse(modulus).unwrap()).rem(modulus);
       // println!("divisor: {:?}, leading coefficient divisor: {:?}", divisor, leading_coeff_divisor);
        let quotient_coeff = (leading_coeff_remainder.clone() * self.mod_inverse(leading_coeff_divisor).unwrap()).rem(&self.modulus);

        // Construct the quotient term (align the degree with degree_diff)
        let mut quotient_term_coeffs = vec![BigInt::zero(); degree_diff + 1];
        quotient_term_coeffs[degree_diff] = quotient_coeff.clone();
        let quotient_term = Polynomial::new(quotient_term_coeffs);

   //     println!("remainder: {:?}, Leading coeff remainder: {:?}, Leading coeff divisor: {:?}, Quotient coefficient: {:?}",
      //           remainder.coeffs, leading_coeff_remainder, divisor.coeffs.last().unwrap(), quotient_coeff);

        // Update quotient
        quotient = self.add(&quotient,&quotient_term);

        // Subtract the product of quotient term and divisor from remainder
        let subtrahend = self.mul(&divisor,&quotient_term);
        remainder = self.sub_mod(&remainder,&subtrahend);

        remainder.trim();

//        println!("Updated remainder: {:?}", remainder.coeffs);
        i += 1;
    }

 //   println!("Final quotient: {:?}", quotient.coeffs);
 //   println!("Final remainder: {:?}", remainder.coeffs);

    (quotient, remainder)
}

/// Extended Euclidean algorithm to compute the GCD and coefficients
fn extended_euclidean(&self, a: &Polynomial, b: &Polynomial) -> (Polynomial, Polynomial, Polynomial) {
    let mut s = Polynomial::new(vec![BigInt::zero()]);
    let mut old_s = Polynomial::new(vec![BigInt::one()]);
    let mut t = Polynomial::new(vec![BigInt::one()]);
    let mut old_t = Polynomial::new(vec![BigInt::zero()]);
    let mut r = b.clone();
    let mut old_r = a.clone();
    let i = 0;
    let mut j = 0;
   // println!("a");
    while !r.coeffs.is_empty() {
  //  println!("b");
    //    let quotient = old_r.clone().rem(&r);
   // println!("j:{:?}",j);
   // println!("r_i: {:?}, r_i+1: {:?}", old_r.coeffs,r.coeffs);

        let (quotient,remainder) = self.polynomial_long_division(&old_r,&r);
     //   println!("quotient: {:?}, remainder: {:?}", quotient, remainder);


        old_r = r;
        r = remainder;
      //  println!("after update. r_i: {:?}, r_i+1: {:?}", old_r.coeffs,r.coeffs);

        let new_s = self.sub_mod(&old_s,&self.mul(&quotient,&s));
        old_s = std::mem::replace(&mut s, new_s);

        let new_t = self.sub_mod(&old_t,&self.mul(&quotient,&t));
        old_t = std::mem::replace(&mut t, new_t);

     //   println!("i:{:?}",i);
    }


    (old_r, old_t, old_s)
}

fn mod_inverse(&self, divisor: BigInt) -> Option<BigInt> {
    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();
    let mut r = self.modulus.clone(); // Owned BigInt
    let mut new_r = divisor.clone();  // Owned BigInt
  
    while !new_r.is_zero() {
        let quotient = &r / &new_r; // Use references to avoid moving r and new_r

        let temp_t = t.clone(); // Clone t to create a temporary BigInt
        t = new_t.clone();      // Move new_t into t
        new_t = temp_t - &quotient * &new_t;

        let temp_r = r.clone(); // Clone r to create a temporary BigInt
        r = new_r.clone();      // Clone new_r to avoid moving it
        new_r = temp_r - quotient * new_r; // Now this works because new_r is cloned
    }

    if r > BigInt::one() {
        None
    } else {
        Some((t.rem(&self.modulus) + &self.modulus).rem(&self.modulus)) // Ensure result is positive and within modulus
    }
}

 
/// Find the inverse of a polynomial in the Galois ring (Z[X] / modulus)
fn find_inverse_in_galois_ring(&self, elem: &Polynomial) -> Option<Polynomial> {
//    println!("0");

    let (g, u, _) = self.extended_euclidean(&self.irreducible, elem);
  //  println!("1");
    // If g is not 1, divide u by g
    if g.coeffs != vec![BigInt::one()] {
        let g_inv = self.mod_inverse(g.coeffs[0].clone()).unwrap(); 
       //     println!("t = {:?}, g_inv: {:?}",u.coeffs, g_inv);
            let inverse_coeffs: Vec<BigInt> = u.coeffs.iter().map(|coeff| (coeff *  &g_inv).rem(&self.modulus)).collect();
            let inverse_poly = Polynomial::new(inverse_coeffs);
            return Some(inverse_poly);

    }

    // If g is 1, just return u mod irreducible
    Some(u)
}


fn generate_exceptional_set(&self) -> Vec<Polynomial> {
    let mut exceptional_set = Vec::new();

    // Loop over all non zero elements in F_2
    let max_value = (1 << self.irreducible.degree());
    for i in 1..max_value  {
        let mut coeffs = Vec::new();
        let mut value = i;

        // Convert integer i to a polynomial by interpreting the bits as coefficients
        for _ in 0..self.irreducible.degree() {
            coeffs.push(BigInt::from(value % 2));
            value /= 2;
        }

        exceptional_set.push(Polynomial::new(coeffs));
        
    }

    exceptional_set
}




fn random_ring_element(&self)->Polynomial{
    let d = self.irreducible.degree();
    let coeffs: Vec<BigInt> = (0..d).map(|_| rand::thread_rng().gen_bigint_range(&BigInt::from(0),&BigInt::from(self.modulus.clone()))).collect();
    Polynomial{coeffs}
}

 
fn generate_random_polynomial_with_secret(&self,secret: Polynomial)->Vec<Polynomial>{
    let mut rand_poly: Vec<Polynomial> = vec![Polynomial::new(secret.coeffs)];
    for _ in 1..self.irreducible.degree(){
        rand_poly.push(self.random_ring_element());
    }
    rand_poly
}


fn evaluate_polynomial(&self, point: &Polynomial, rand_polynomial_ring: &Vec<Polynomial>)-> Polynomial{
    let mut eval_poly = Polynomial::new(vec![BigInt::from(0)]);
    for i in 0..self.irreducible.degree(){
        eval_poly = self.add_ring(&eval_poly,  &self.mul_ring(&rand_polynomial_ring[i],&self.power_in_ring(point,i)));
      //  println!("------ term {:?}, current mul is {:?}, power of point {:?} is {:?}", i, self.mul_ring(&rand_polynomial_ring[i],&self.power_in_ring(point,i)), point, &self.power_in_ring(point,i));
    }
 //   println!("------term of point {:?} is {:?}", point, eval_poly.clone());
  
    eval_poly
}


fn shamir_secret_sharing(&self, secret: Polynomial,  number_of_parties: usize, t: usize) -> Vec<(Polynomial,Polynomial)>{
    assert!(number_of_parties <= (1 << &self.irreducible.degree()) -1);// check that  n <= 2^d
    assert!(number_of_parties <= (1 << &self.irreducible.degree()) -1);// check that  n <= 2^d

    let random_polynomial_ring =  &self.generate_random_polynomial_with_secret(secret);
    let mut shares: Vec<(Polynomial,Polynomial)> = vec![];
    let mut evaluated_element: Polynomial;
    let points = self.generate_exceptional_set();
    for i in 0..number_of_parties{
        evaluated_element = self.evaluate_polynomial(&points[i], &random_polynomial_ring.clone());
        shares.push((points[i].clone(),evaluated_element));
    }
   // println!("random polynomial ring: {:?}", random_polynomial_ring);

    shares
}



fn shamir_secret_sharing_non_random(&self, poly_vec: Vec<Polynomial>,  number_of_parties: usize) -> Vec<(Polynomial,Polynomial)>{
    assert!(number_of_parties <= (1 << &self.irreducible.degree()) -1);// check that  n <= 2^d
    assret!(number_of_parties >= )
    let mut shares: Vec<(Polynomial,Polynomial)> = vec![];
    let mut evaluated_element: Polynomial;
    let points = self.generate_exceptional_set();
    for i in 0..number_of_parties{
        evaluated_element = self.evaluate_polynomial(&points[i], &poly_vec.clone());
        shares.push((points[i].clone(),evaluated_element));
    }
    println!("random polynomial ring: {:?}", poly_vec);

    shares
}

fn reconstruct_secret(&self, shares: Vec<(Polynomial,Polynomial)>) -> Polynomial{
    let mut res = GaloisRing::zero();
    for  (xi, yi) in shares.clone(){
    let mut li = Polynomial::new(vec![BigInt::from(1)]);
        for (xj, _) in shares.clone(){
            if xi != xj {
                let numerator =   xj.clone();
                let denominator = self.sub_mod(&xj,&xi);
                let denominator_inv = self.find_inverse_in_galois_ring(&denominator);
                let frac = self.mul_ring(&numerator,&denominator_inv.clone().unwrap());
               //`` println!("num: {:?}, denom {:?}, denom_inv {:?}, frac = {:?}", numerator.coeffs, denominator.coeffs, denominator_inv, frac.coeffs);
                println!("li {:?}, frac = {:?}", li.coeffs, frac.coeffs);

                li  = self.mul_ring(&li,&frac);
            }
        }
        res = self.add_ring(&res,&self.mul(&yi,&li));
        println!("partial sum: {:?}, yi:{:?}, li:{:?}", res, yi,li);
    }
 res
}


}

fn irreducible_polynomial(degree: usize) -> Option<Polynomial> {
    match degree {
        1 => {
            // Define the irreducible polynomial r(x) = x + 1
            Some(Polynomial::new(vec![
                BigInt::from(1), // Coefficient for x^0
                BigInt::from(1), // Coefficient for x^1
            ]))
        }
        2 => {
            // Define the irreducible polynomial r(x) = x^2 + x + 1
            Some(Polynomial::new(vec![
                BigInt::from(1), // Coefficient for x^0
                BigInt::from(1), // Coefficient for x^1
                BigInt::from(1), // Coefficient for x^2
            ]))
        }
        3 => {
            // Define the irreducible polynomial r(x) = x^3 + x + 1
            Some(Polynomial::new(vec![
                BigInt::from(1), // Coefficient for x^0
                BigInt::from(1), // Coefficient for x^1
                BigInt::from(0), // Coefficient for x^2
                BigInt::from(1), // Coefficient for x^3
            ]))
        }
        4 => {
            // Define the irreducible polynomial r(x) = x^4 + x + 1
            Some(Polynomial::new(vec![
                BigInt::from(1), // Coefficient for x^0
                BigInt::from(1), // Coefficient for x^1
                BigInt::from(0), // Coefficient for x^2
                BigInt::from(0), // Coefficient for x^3
                BigInt::from(1), // Coefficient for x^4
            ]))
        }
        5 => {
            // Define the irreducible polynomial r(x) = x^5 + x^2 + 1
            Some(Polynomial::new(vec![
                BigInt::from(1), // Coefficient for x^0
                BigInt::from(0), // Coefficient for x^1
                BigInt::from(1), // Coefficient for x^2
                BigInt::from(0), // Coefficient for x^3
                BigInt::from(0), // Coefficient for x^4
                BigInt::from(1), // Coefficient for x^5
            ]))
        }
        _ => None, // Return None for any other degree
    }
}


fn main() {
    let modulus = BigInt::from(7);

    let degree = 3;
    let number_of_parties = 3;
    //let number_of_parties = 2^(irreducible.degree() - 1);
 
    let irreducible = irreducible_polynomial(degree).unwrap();
    let ring = GaloisRing::new(modulus, irreducible);
    let secret = Polynomial::new(vec![
        BigInt::from(6), // Coefficient for x^0
        BigInt::from(4), // Coefficient for x^1
       ]);

    let a1 = Polynomial::new(vec![
        BigInt::from(5), // Coefficient for x^0
        BigInt::from(6), // Coefficient for x^1
       ]);

       let a2 = Polynomial::new(vec![
        BigInt::from(0), // Coefficient for x^0
        BigInt::from(5), // Coefficient for x^1
       ]);
    
       let mut non_rand_poly: Vec<Polynomial> = vec![secret.clone()];
       non_rand_poly.push(a1);
       non_rand_poly.push(a2);

       println!("NON random polynomial ring: {:?}", non_rand_poly);

    
       let shares = ring.shamir_secret_sharing_non_random(non_rand_poly, number_of_parties);
       println!("shares: {:?}",shares);

    // Define the polynomial to invert new_r(x) = x^2 + 1
 //   let divisor = Polynomial::new(vec![BigInt::from(1), BigInt::from(0), BigInt::from(1)]);
    //let (remainder,quotient) = ring.polynomial_long_division(&ring.irreducible, &divisor);
    /* 
    match find_inverse_in_galois_ring(&elem, &modulus, &irreducible) {
        Some(inverse) => println!("Inverse in Galois Ring: {:?}", inverse),
        None => println!("No inverse exists."),
    }
    */
   //let (r,s,t) = ring.extended_euclidean(&ring.irreducible, &divisor);
//   println!("r: {:?}, t: {:?}, s:{:?}",r,s,t);
 //  let result = ring.find_inverse_in_galois_ring(&divisor);
 //  println!("{:?}", result);

   let exeptional_set = ring.generate_exceptional_set();
   println!("Exceptional set: {:?}", exeptional_set);
   let secret = ring.random_ring_element();
   println!("secret: {:?}", secret);
    println!("random polynomial ring (i.e., a list of polynomials): {:?}", ring.generate_random_polynomial_with_secret(secret.clone()));
         // Example polynomials: x^3 + 2x^2 + 3x + 4 and x^2 + 1
         let a = Polynomial::new(vec![
            BigInt::from(0), // Coefficient for x^0
            BigInt::from(1), // Coefficient for x^1
            ]);
        let b = Polynomial::new(vec![
            BigInt::from(6), // Coefficient for x^0
            BigInt::from(3),  // Coefficient for x^1
            BigInt::from(5), // Coefficient for x^2
            BigInt::from(2), // Coefficient for x^3
        ]);
      //  println!("mul ring: a: {:?}, b:{:?}, a*b = {:?}", a,b, a.mul_ring(&b, &modulus, &irreducible) );
        println!("****** a^3 = {:?}", ring.power_in_ring(&a, 2));
        //let rand_polynomial_ring = generate_random_polynomial_with_secret(secret.clone(), &modulus, &irreducible);
        //let evaluated_poly = evaluate_polynomial(&zero(), rand_polynomial_ring.clone(), &modulus, &irreducible);
        //println!("evaluated polynnomial: {:?} at 0: {:?}, with secret {:?}, gives {:?}", rand_polynomial_ring, 0, secret, evaluated_poly );
        let shares = ring.shamir_secret_sharing(secret.clone(), number_of_parties);
        println!("shares: {:?}",shares);
        let reconstructed_secret = ring.reconstruct_secret(shares);
        println!("original secret: {:?}. Reconstrcuted secret: {:?}", secret,reconstructed_secret);

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
