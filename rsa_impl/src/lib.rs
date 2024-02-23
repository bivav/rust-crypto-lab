use num_bigint::BigUint;
use num_traits::{One, Zero};

pub struct RSAAlgo {
    pub p: BigUint,
    pub q: BigUint,
}

impl RSAAlgo {
    pub fn new(p: &BigUint, q: &BigUint) -> Self {
        Self {
            p: p.clone(),
            q: q.clone(),
        }
    }

    /**
    #### Formula
    ```markdown
    n = p * q
    ```
    where `p` and `q` are prime numbers.
     **/
    pub fn generate_modulus(&self) -> BigUint {
        &self.p * &self.q
    }

    /**
    #### Formula
    ```markdown
    phi = (p - 1) * (q - 1)
    ```
    where `p` and `q` are prime numbers.<br>
    This calculates Euler's totient.
     **/
    pub fn eulers_totient(&self) -> BigUint {
        (&self.p - 1u32) * (&self.q - 1u32)
    }

    /**
    #### Formula
    ```markdown
    e = 65537
    ```
     **/
    pub fn choose_public_exponent() -> BigUint {
        BigUint::from(65537u32)
    }

    /*
        The `extended_gcd` function is used to calculate the greatest common divisor (gcd) of `e`
        and `phi` using the Extended Euclidean Algorithm.
        It also calculates the Bézout's coefficients of `e` and `phi` i.e: `x` and `y` such that `e*x + phi*y = gcd(e, phi)`.
        If `e` is zero, the function returns `phi` as the gcd, and 0 and 1 as the Bézout's coefficients.
        Otherwise, it recursively calls itself with `phi % e` and `e`, and calculates the new Bézout's coefficients.
    */
    // pub fn extended_gcd(e: &BigUint, phi: &BigUint) -> (BigUint, BigUint, BigUint) {
    //     if e.is_zero() {
    //         // When e is 0, gcd is phi, x is 0, & y is 1 according to the algorithm's base case
    //         (phi.clone(), BigUint::zero(), BigUint::one())
    //     } else {
    //         let (gcd, x, y) = Self::extended_gcd(&(phi % e), e);
    //
    //         // Ensuring that the result is always positive by adding phi to the result and taking the modulus of phi
    //         let new_x = (y + phi) - ((phi / e) * &x) % phi;
    //         let new_y = x;
    //
    //         (gcd, new_x, new_y)
    //     }
    // }

    // pub fn extended_gcd(e: &BigUint, phi: &BigUint) -> (BigUint, BigUint, BigUint) {
    //     if e.is_zero() {
    //         (phi.clone(), BigUint::zero(), BigUint::one())
    //     } else {
    //         let (gcd, x, y) = Self::extended_gcd(&(phi % e), e);
    //
    //         // Correcting the calculation of new_x to ensure proper operation order
    //         let new_x = ((y + phi) - (((phi / e) * &x) % phi) + phi) % phi; // Adding phi before the final modulus to ensure positivity
    //         let new_y = x;
    //
    //         println!(
    //             "a: {}, b: {}, gcd: {}, x: {}, y: {}",
    //             &e, &phi, &gcd, &new_x, &new_y
    //         ); // Debugging print
    //
    //         (gcd, new_x, new_y)
    //     }
    // }

    /*
        The `mod_inverse` function is part of the `RSAAlgo` struct implementation and is used to
        calculate the modular multiplicative inverse of two numbers.
        The modular multiplicative inverse of `a mod m` is an integer `b` such that `(a*b) mod m = 1`.
        This function takes two parameters, `e` and `phi`, both of type `BigUint`.
        - It starts by calling the `extended_gcd` function with `e` and `phi` as arguments to find the greatest
        common divisor (gcd) of `e` and `phi` and the Bézout's coefficient of `e`.
        - If `gcd` is one, it means that `e` and `phi` are coprime (i.e., their greatest common divisor is 1).
        In this case, the function calculates the modular multiplicative inverse of `e` under `phi`
        using the formula `(x % phi + phi) % phi`, and returns it.
        - If `gcd` is not one, it means that `e` and `phi` are not coprime, and hence,
        the modular multiplicative inverse does not exist.
        - In this case, the function panics with the message "Inverse does not exist."
        This function is a key part of the RSA encryption algorithm, used to calculate the
        modular multiplicative inverse of two numbers, which is necessary for generating the private key in RSA.
    */
    // pub fn mod_inverse(e: BigUint, phi: BigUint) -> BigUint {
    //     let (gcd, x, _) = Self::extended_gcd(&e, &phi);
    //     if gcd.is_one() {
    //         (x % phi.clone() + phi.clone()) % phi.clone()
    //     } else {
    //         panic!("Inverse does not exist.")
    //     }
    // }

    // pub fn mod_inverse(e: BigUint, phi: BigUint) -> BigUint {
    //     let (gcd, x, _) = Self::extended_gcd(&e, &phi);
    //
    //     println!("GCD: {}", &gcd); // Debugging print
    //
    //     if gcd.is_one() {
    //         // Simplifying the final adjustment to ensure x is positive
    //         (x + phi.clone()) % phi.clone()
    //     } else {
    //         panic!("Inverse does not exist, as gcd is not 1.");
    //     }
    // }

    /**

    The modular multiplicative inverse of <code><span style="color:#87CEEB"><b>`a mod m`</b></span></code> is an integer <code><span style="color:#87CEEB"><b>`b`</b></span></code> such that,
    ```markdown
    (a * b) mod m = 1
    where `a` and `m` are coprime (i.e., their GCD is 1).
    ```
    This function is a key part of the RSA encryption algorithm, used to calculate the modular multiplicative inverse of two numbers,
    which is necessary for generating the private key in RSA.
    ___

    ### The Extended Euclidean Algorithm
    The Extended Euclidean Algorithm is used in the <code><span style="color:#87CEEB"><b>mod_inverse</b></span></code> function to calculate the modular multiplicative inverse of two numbers.
    <br><br>
    In the context of RSA encryption, this is necessary for generating the private key.
    The algorithm helps to find an integer `x` such that
    ```markdown
    (e*x) mod phi = 1
    where `e` is the public exponent and `phi` is Euler's totient.
    ```

    The algorithm works by initializing four variables <code><span style="color:#87CEEB"><b>`a`, `b`, `x0`, `x1`</b></span></code>.
    It then enters a loop that continues as long as <code><span style="color:#87CEEB"><b>`b`</b></span></code> is greater than zero.
    Inside the loop, it calculates the quotient of `a` divided by `b`, then updates `a` and `b` using the Extended Euclidean Algorithm.
    It also updates <code><span style="color:#87CEEB"><b>`x0` </b></span></code> and <code><span style="color:#87CEEB"><b>`x1`</b></span></code> which are used to calculate the Bézout's coefficients.
    <br><br>
    After the loop, the function checks if `a` is equal to <code><span style="color:#87CEEB"><b>`BigUint::one()`</b></span></code>.
    If it is, this means that <code><span style="color:#87CEEB"><b>`e` and `phi` </b></span></code> are coprime (i.e., their greatest common divisor is 1),
    and the function returns <code><span style="color:#87CEEB"><b>`Some(x0)` </b></span></code>, where <code><span style="color:#87CEEB"><b>`x0` </b></span></code> is the modular multiplicative inverse of <code><span style="color:#87CEEB"><b>`e` </b></span></code> under <code><span style="color:#87CEEB"><b>`phi`</b></span></code>.
    <br><br>
    If `a` is not equal to `BigUint::one()`, this means that `e` and `phi` are not coprime, and hence, the modular multiplicative inverse does not exist.
    In this case, the function returns `None`.

     **/
    pub fn mod_inverse(e: &BigUint, phi: &BigUint) -> Option<BigUint> {
        let (mut a, mut b, mut x0, mut x1) = (phi.clone(), e.clone(), BigUint::zero(), BigUint::one());

        while b > BigUint::zero() {
            let quotient = &a / &b;

            let temp = b.clone();
            b = &a - &quotient * &b;
            a = temp;

            let temp = x1.clone();
            x1 = (&x0 + phi - (&quotient * &x1) % phi) % phi; // Ensuring x1 is positive
            x0 = temp;
        }

        // Making sure the inverse exists which means a should be 1 if e and phi are coprime
        if a != BigUint::one() {
            return None; // Inverse doesn't exist if gcd(e, phi) != 1
        }

        Some(x0)
    }

    pub fn encrypt(message: &BigUint, e: &BigUint, n: &BigUint) -> BigUint {
        // implementing c = m^e mod n
        message.modpow(e, n)
    }

    pub fn decrypt(encrypted: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
        // implementing m = c^d mod n
        encrypted.modpow(d, n)
    }
}
