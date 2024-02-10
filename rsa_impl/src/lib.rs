//
// use num_bigint::BigUint;
// use num_traits::{One, Zero};
//
// pub struct RSAAlgo {
//     pub p: BigUint,
//     pub q: BigUint,
// }
//
// impl RSAAlgo {
//     pub fn new(p: &BigUint, q: &BigUint) -> Self {
//         Self {
//             p: p.clone(),
//             q: q.clone(),
//         }
//     }
//
//     pub fn generate_modulus(&self) -> BigUint {
//         &self.p * &self.q
//     }
//
//     pub fn eulers_totient(&self) -> BigUint {
//         (&self.p - 1u32) * (&self.q - 1u32)
//     }
//
//     pub fn choose_public_exponent() -> BigUint {
//         BigUint::from(65537u32)
//     }
//
//     /*
//         The `extended_gcd` function is used to calculate the greatest common divisor (gcd) of `e`
//         and `phi` using the Extended Euclidean Algorithm.
//         It also calculates the Bézout's coefficients of `e` and `phi` i.e: `x` and `y` such that `e*x + phi*y = gcd(e, phi)`.
//         If `e` is zero, the function returns `phi` as the gcd, and 0 and 1 as the Bézout's coefficients.
//         Otherwise, it recursively calls itself with `phi % e` and `e`, and calculates the new Bézout's coefficients.
//     */
//     // pub fn extended_gcd(e: &BigUint, phi: &BigUint) -> (BigUint, BigUint, BigUint) {
//     //     if e.is_zero() {
//     //         // When e is 0, gcd is phi, x is 0, & y is 1 according to the algorithm's base case
//     //         (phi.clone(), BigUint::zero(), BigUint::one())
//     //     } else {
//     //         let (gcd, x, y) = Self::extended_gcd(&(phi % e), e);
//     //
//     //         // Ensuring that the result is always positive by adding phi to the result and taking the modulus of phi
//     //         let new_x = (y + phi) - ((phi / e) * &x) % phi;
//     //         let new_y = x;
//     //
//     //         (gcd, new_x, new_y)
//     //     }
//     // }
//
//     pub fn extended_gcd(e: &BigUint, phi: &BigUint) -> (BigUint, BigUint, BigUint) {
//         if e.is_zero() {
//             (phi.clone(), BigUint::zero(), BigUint::one())
//         } else {
//             let (gcd, x, y) = Self::extended_gcd(&(phi % e), e);
//
//             // Correcting the calculation of new_x to ensure proper operation order
//             let new_x = ((y + phi) - (((phi / e) * &x) % phi) + phi) % phi; // Adding phi before the final modulus to ensure positivity
//             let new_y = x;
//
//             println!("a: {}, b: {}, gcd: {}, x: {}, y: {}", &e, &phi, &gcd, &new_x, &new_y); // Debugging print
//
//
//             (gcd, new_x, new_y)
//         }
//     }
//
//
//     /*
//         The `mod_inverse` function is part of the `RSAAlgo` struct implementation and is used to
//         calculate the modular multiplicative inverse of two numbers.
//         The modular multiplicative inverse of `a mod m` is an integer `b` such that `(a*b) mod m = 1`.
//         This function takes two parameters, `e` and `phi`, both of type `BigUint`.
//         - It starts by calling the `extended_gcd` function with `e` and `phi` as arguments to find the greatest
//         common divisor (gcd) of `e` and `phi` and the Bézout's coefficient of `e`.
//         - If `gcd` is one, it means that `e` and `phi` are coprime (i.e., their greatest common divisor is 1).
//         In this case, the function calculates the modular multiplicative inverse of `e` under `phi`
//         using the formula `(x % phi + phi) % phi`, and returns it.
//         - If `gcd` is not one, it means that `e` and `phi` are not coprime, and hence,
//         the modular multiplicative inverse does not exist.
//         - In this case, the function panics with the message "Inverse does not exist."
//         This function is a key part of the RSA encryption algorithm, used to calculate the
//         modular multiplicative inverse of two numbers, which is necessary for generating the private key in RSA.
//     */
//     // pub fn mod_inverse(e: BigUint, phi: BigUint) -> BigUint {
//     //     let (gcd, x, _) = Self::extended_gcd(&e, &phi);
//     //     if gcd.is_one() {
//     //         (x % phi.clone() + phi.clone()) % phi.clone()
//     //     } else {
//     //         panic!("Inverse does not exist.")
//     //     }
//     // }
//
//     pub fn mod_inverse(e: BigUint, phi: BigUint) -> BigUint {
//         let (gcd, x, _) = Self::extended_gcd(&e, &phi);
//
//         println!("GCD: {}", &gcd); // Debugging print
//
//         if gcd.is_one() {
//             // Simplifying the final adjustment to ensure x is positive
//             (x + phi.clone()) % phi.clone()
//         } else {
//             panic!("Inverse does not exist, as gcd is not 1.");
//         }
//     }
//
//     pub fn encrypt(message: &BigUint, e: &BigUint, n: &BigUint) -> BigUint {
//         // c = m^e mod n
//         message.modpow(e, n)
//     }
//
//     pub fn decrypt(encrypted: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
//         // m = c^d mod n
//         encrypted.modpow(d, n)
//     }
//
//     pub fn modular_exponentiation(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
//         let mut result = BigUint::one();
//         let mut base = base.clone() % modulus;
//         let mut exponent = exponent.clone();
//
//         while exponent > BigUint::zero() {
//             if &exponent % 2u32 == BigUint::one() {
//                 result = (result * &base) % modulus;
//             }
//             exponent >>= 1; // Divide the exponent by 2
//             base = (&base * &base) % modulus; // Square the base
//         }
//
//         result
//     }
//
// }
