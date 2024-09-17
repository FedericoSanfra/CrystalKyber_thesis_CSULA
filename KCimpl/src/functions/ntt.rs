//! Number Theoretic Trasform (NTT)
//!
//! NTT operations and operations performed in the NTT domain

use crate::structures::{
    algebraics::{FiniteField, FiniteRing, RingModule},
    Poly3329, PolyMatrix3329, PolyVec3329, F3329,
};

/// Funzione per invertire i bit di un numero su 7 bit
fn byte_rev(i: usize) -> usize {
    let mut i = i;
    let mut rev = 0;
    for _ in 0..7 {
        rev = (rev << 1) | (i & 1); // Sposta a sinistra `rev` e aggiungi il bit meno significativo di `i`
        i >>= 1; // Sposta a destra `i` per analizzare il bit successivo
    }
    rev
}


/// Basecase multiplication between polynomials (p 7)
fn bcm<const N: usize>(a: &Poly3329<N>, b: &Poly3329<N>) -> Poly3329<N> {
    // BCM with the zero polynomial is the zero polynomial
    if a.is_zero() || b.is_zero() {
        return Poly3329::zero();
    }

    let mut p = Poly3329::init();

    for i in 0..=(N - 1) / 2 {
        let zeta = F3329::from_int(ZETAS_256[2 * byte_rev(i) + 1]);

        let p01 = a[2 * i].mul(&b[2 * i]);
        let p02 = a[2 * i + 1].mul(&b[2 * i + 1]).mul(&zeta);

        let p11 = a[2 * i].mul(&b[2 * i + 1]);
        let p12 = a[2 * i + 1].mul(&b[2 * i]);

        p.set_coeff(2 * i, p01.add(&p02));
        p.set_coeff(2 * i + 1, p11.add(&p12));
    }
    p
}

/// Base case multiplivation for vectors
fn bcm_vec<const N: usize, const D: usize>(
    a: &PolyVec3329<N, D>,
    b: &PolyVec3329<N, D>,
) -> Poly3329<N> {
    let mut p = bcm(&a.get(0), &b.get(0));
    for i in 1..D {
        p = p.add(&bcm(&a.get(i), &b.get(i)));
    } //product between matrix's rows and b vector
    p
}


/// Computes a.b as NTT^-1(a_hat o b_hat)
fn ntt_product<const N: usize>(a_hat: &Poly3329<N>, b_hat: &Poly3329<N>) -> Poly3329<N> {
    rev_ntt(&bcm(a_hat, b_hat))
}

/// Computes a^T.b as NTT^-1(a_hat^T o b_hat)
pub fn ntt_product_vec<const N: usize, const D: usize>(
    a_hat: &PolyVec3329<N, D>,
    b_hat: &PolyVec3329<N, D>,
) -> Poly3329<N> {
    rev_ntt(&bcm_vec(a_hat, b_hat))
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product_matvec<const N: usize, const X: usize, const Y: usize>(
    a_hat: &PolyMatrix3329<N, X, Y>,
    b_hat: &PolyVec3329<N, X>,
) -> PolyVec3329<N, Y> {
    rev_ntt_vec(&bcm_matrix_vec(a_hat, b_hat))
}

/// Number theoretic Transform on vectors
pub fn ntt_vec<const N: usize, const D: usize>(p: &PolyVec3329<N, D>) -> PolyVec3329<N, D> {
    let mut coeffs = [Default::default(); D];
    for i in 0..D {
        coeffs[i] = base_ntt(&p.coefficients[i]);
    }
    PolyVec3329::from_vec(coeffs)
}

/// Reverse NTT on vectors
fn rev_ntt_vec<const N: usize, const D: usize>(p_hat: &PolyVec3329<N, D>) -> PolyVec3329<N, D> {
    let mut coeffs = [Default::default(); D];
    for i in 0..D {
        coeffs[i] = rev_ntt(&p_hat.coefficients[i]);
    }
    PolyVec3329::from_vec(coeffs)
}

/// Number theoretic Transform
fn base_ntt<const N: usize>(p: &Poly3329<N>) -> Poly3329<N> {
    let mut a = Poly3329::init();

    // Zero polynomial's NTT is zero
    if p.is_zero() {
        return Poly3329::zero();
    }

    // We assume d is even since spec requires operating mod X^2-zeta
    for i in 0..=(N - 1) / 2 {
        let mut p0 = p[0];
        let mut p1 = p[1];

        for j in 1..=(N - 1) / 2 {
            let index = (2 * byte_rev(i) * j + j) % 256;
            let zeta = F3329::from_int(ZETAS_256[index]);
            let mut c0 = p[2 * j];
            let mut c1 = p[2 * j + 1];

            c0 = c0.mul(&zeta);
            c1 = c1.mul(&zeta);

            p0 = p0.add(&c0);
            p1 = p1.add(&c1);
        }
        a.set_coeff(2 * i, p0);
        a.set_coeff(2 * i + 1, p1);
    }

    a
}

/// Reverse NTT
fn rev_ntt<const N: usize>(p_hat: &Poly3329<N>) -> Poly3329<N> {
    let mut a = Poly3329::init();

    // Zero polynomial's NTT is zero
    if p_hat.is_zero() {
        return Poly3329::zero();
    }
    // Unwraps safely since the case None has been tested above
    let d = p_hat.degree().unwrap();

    let coeff = F3329::from_int((d / 2) + 1);

    for i in 0..=(N - 1) / 2 {
        let mut p0 = p_hat[0];
        let mut p1 = p_hat[1];
        let z = F3329::from_int(ZETAS_256[((256 - i) % 256)]);

        for j in 1..=(N - 1) / 2 {
            let index = (2 * byte_rev(i) * j) % 256;
            let zeta = F3329::from_int(ZETAS_256[(256 - index) % 256]);
            let mut c0 = p_hat[2 * j];
            let mut c1 = p_hat[2 * j + 1];

            c0 = c0.mul(&zeta);
            c1 = c1.mul(&zeta);

            p0 = p0.add(&c0);
            p1 = p1.add(&c1);
        }

        // Unwraps safely since coeff is d/2 + 1
        a.set_coeff(2 * i, p0.mul(&z).div(&coeff).unwrap());
        a.set_coeff(2 * i + 1, p1.mul(&z).div(&coeff).unwrap());
    }

    a
}

#[test]
fn rev_then_ntt() {
    let mut u_bold = Poly3329::from_vec([Default::default(); 256]);
    for i in 0..256 {
        u_bold.set_coeff(i, F3329::from_int(i));
    }
    let u = rev_ntt(&u_bold);

    assert_eq!(u_bold.coefficients, base_ntt(&u).coefficients)
}

#[test]
fn ntt_then_rev() {
    let mut u = Poly3329::from_vec([Default::default(); 256]);
    for i in 0..256 {
        u.set_coeff(i, F3329::from_int(i));
    }
    let u_bold = base_ntt(&u);

    assert_eq!(u.coefficients, rev_ntt(&u_bold).coefficients)
}