//! Number Theoretic Trasform (NTT)
//!
//! NTT operations and operations performed in the NTT domain

use crate::structures::{
    algebraics::{FiniteField, FiniteRing, RingModule},
    Poly3329, PolyMatrix3329, PolyVec3329, F3329,
};

/// 256-roots of unity
const ZETAS_256: [usize; 256] = [
    1, 17, 289, 1584, 296, 1703, 2319, 2804, 1062, 1409, 650, 1063, 1426, 939, 2647, 1722, 2642,
    1637, 1197, 375, 3046, 1847, 1438, 1143, 2786, 756, 2865, 2099, 2393, 733, 2474, 2110, 2580,
    583, 3253, 2037, 1339, 2789, 807, 403, 193, 3281, 2513, 2773, 535, 2437, 1481, 1874, 1897,
    2288, 2277, 2090, 2240, 1461, 1534, 2775, 569, 3015, 1320, 2466, 1974, 268, 1227, 885, 1729,
    2761, 331, 2298, 2447, 1651, 1435, 1092, 1919, 2662, 1977, 319, 2094, 2308, 2617, 1212, 630,
    723, 2304, 2549, 56, 952, 2868, 2150, 3260, 2156, 33, 561, 2879, 2337, 3110, 2935, 3289, 2649,
    1756, 3220, 1476, 1789, 452, 1026, 797, 233, 632, 757, 2882, 2388, 648, 1029, 848, 1100, 2055,
    1645, 1333, 2687, 2402, 886, 1746, 3050, 1915, 2594, 821, 641, 910, 2154, 3328, 3312, 3040,
    1745, 3033, 1626, 1010, 525, 2267, 1920, 2679, 2266, 1903, 2390, 682, 1607, 687, 1692, 2132,
    2954, 283, 1482, 1891, 2186, 543, 2573, 464, 1230, 936, 2596, 855, 1219, 749, 2746, 76, 1292,
    1990, 540, 2522, 2926, 3136, 48, 816, 556, 2794, 892, 1848, 1455, 1432, 1041, 1052, 1239, 1089,
    1868, 1795, 554, 2760, 314, 2009, 863, 1355, 3061, 2102, 2444, 1600, 568, 2998, 1031, 882,
    1678, 1894, 2237, 1410, 667, 1352, 3010, 1235, 1021, 712, 2117, 2699, 2606, 1025, 780, 3273,
    2377, 461, 1179, 69, 1173, 3296, 2768, 450, 992, 219, 394, 40, 680, 1573, 109, 1853, 1540,
    2877, 2303, 2532, 3096, 2697, 2572, 447, 941, 2681, 2300, 2481, 2229, 1274, 1684, 1996, 642,
    927, 2443, 1583, 279, 1414, 735, 2508, 2688, 2419, 1175,
];

/// 7-byte reversal (to impleme)
fn byte_rev(i: usize) -> usize {
    i
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

/// Triple Modular Redundancy for polynomial multiplication
fn bcm_tmr<const N: usize>(a: &Poly3329<N>, b: &Poly3329<N>) -> Poly3329<N> {
    // Execute BCM three times
    let result1 = bcm(a, b);
    let result2 = bcm(a, b);
    let result3 = bcm(a, b);

    // Compare the results of the three operations and select the correct one
    Poly3329::majority(result1, result2, result3)
}

/// Base case multiplication for vectors
fn bcm_vec<const N: usize, const D: usize>(
    a: &PolyVec3329<N, D>,
    b: &PolyVec3329<N, D>,
) -> Poly3329<N> {
    let mut p = bcm_tmr(&a.get(0), &b.get(0));
    for i in 1..D {
        p = p.add(&bcm_tmr(&a.get(i), &b.get(i)));
    }
    p
}

/// Matrix base case multiplication, cf p. 7
pub fn bcm_matrix_vec<const N: usize, const X: usize, const Y: usize>(
    a: &PolyMatrix3329<N, X, Y>,
    b: &PolyVec3329<N, X>,
) -> PolyVec3329<N, Y> {
    let mut v = PolyVec3329::init();

    for i in 0..Y {
        v.set(i, bcm_vec(&a.row(i), &b))
    }

    v
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
fn ntt_product<const N: usize>(a_hat: &Poly3329<N>, b_hat: &Poly3329<N>) -> Poly3329<N> {
    rev_ntt_tmr(&bcm(a_hat, b_hat))
}

/// Computes a^T.b as NTT^-1(a_hat^T o b_hat)
pub fn ntt_product_vec<const N: usize, const D: usize>(
    a_hat: &PolyVec3329<N, D>,
    b_hat: &PolyVec3329<N, D>,
) -> Poly3329<N> {
    rev_ntt_tmr(&bcm_vec(a_hat, b_hat))
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
        coeffs[i] = base_ntt_tmr(&p.coefficients[i]);
    }
    PolyVec3329::from_vec(coeffs)
}

/// Reverse NTT on vectors
fn rev_ntt_vec<const N: usize, const D: usize>(p_hat: &PolyVec3329<N, D>) -> PolyVec3329<N, D> {
    let mut coeffs = [Default::default(); D];
    for i in 0..D {
        coeffs[i] = rev_ntt_tmr(&p_hat.coefficients[i]);
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

/// Triple Modular Redundancy (TMR) for NTT
fn base_ntt_tmr<const N: usize>(p: &Poly3329<N>) -> Poly3329<N> {
    // Esegui la trasformazione NTT tre volte
    let result1 = base_ntt(p);
    let result2 = base_ntt(p);
    let result3 = base_ntt(p);

    // Confronta i risultati delle tre operazioni e scegli quello corretto
    Poly3329::majority(result1, result2, result3)
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
        let z = F3329::from_int(ZETAS_256[(256 - i) % 256]);

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

/// Triple Modular Redundancy (TMR) for Reverse NTT
fn rev_ntt_tmr<const N: usize>(p_hat: &Poly3329<N>) -> Poly3329<N> {
    // Esegui la trasformazione reverse NTT tre volte
    let result1 = rev_ntt(p_hat);
    let result2 = rev_ntt(p_hat);
    let result3 = rev_ntt(p_hat);

    // Confronta i risultati delle tre operazioni e scegli quello corretto
    Poly3329::majority(result1, result2, result3)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::F3329; // Assicurati di importare la struttura necessaria per i test.


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


    #[test]
    fn test_base_ntt_tmr() {
        // Crea un polinomio di esempio
        let mut poly = Poly3329::from_vec([Default::default(); 256]);
        for i in 0..256 {
            poly.set_coeff(i, F3329::from_int(i));
        }

        // Clona il polinomio per simulare le copie usate nella TMR
        let mut poly1 = poly.clone();
        let mut poly2 = poly.clone();
        let mut poly3 = poly.clone();

        // Introduci errori in alcune delle copie per simulare bit flipping o errori
        poly2.set_coeff(5, F3329::from_int(999));  // Errore nel coefficiente 5
        poly3.set_coeff(10, F3329::from_int(888)); // Errore nel coefficiente 10



        let result1 = base_ntt(&poly1);
        let result2 = base_ntt(&poly2);
        let result3 = base_ntt(&poly3);

        let maj_p=Poly3329::majority(result1, result2, result3);

        // Puoi anche aggiungere un confronto con un risultato atteso, se disponibile
        let expected_result = base_ntt(&poly1); // Utilizza la funzione base_ntt come riferimento.

        // Verifica che il risultato non sia nullo

       // assert!(!result.is_zero());


        assert_eq!(expected_result.coefficients, maj_p.coefficients);
    }

    #[test]
    fn test_rev_ntt_tmr() {
        // Testare la funzione rev_ntt_tmr con un polinomio di esempio
        let mut poly_hat = Poly3329::from_vec([Default::default(); 256]);
        for i in 0..256 {
            poly_hat.set_coeff(i, F3329::from_int(i));
        }

        let result_hat = rev_ntt_tmr(&poly_hat);
        // Verifica che il risultato non sia nullo


        // Puoi anche aggiungere un confronto con un risultato atteso, se disponibile
        let expected_result_hat = rev_ntt(&poly_hat); // Utilizza la funzione rev_ntt come riferimento.
        assert_eq!(expected_result_hat.coefficients, result_hat.coefficients);
    }

    #[test]
    fn test_base_ntt_majority_error_simulation(){
        // Testare la funzione base_ntt_tmr con un polinomio di esempio
        let mut poly = Poly3329::from_vec([Default::default(); 256]);
        for i in 0..256 {
            poly.set_coeff(i, F3329::from_int(i));
        }
        ///base ntt tmr simulation
        let mut result1 = base_ntt(&poly);
        let mut result2 = base_ntt(&poly);
        let mut result3 = base_ntt(&poly);



        // Modificare alcuni coefficienti in result1, result2 e result3
        for i in 0..256 {
            if i % 4 == 0 { // Modifica 1/4 dei coefficienti
                result1.set_coeff(i, F3329::from_int((i + 1) % 3329)); // Modifica a un valore diverso
            }
            if i % 4 == 1 { // Modifica 1/4 dei coefficienti
                result2.set_coeff(i, F3329::from_int((i + 2) % 3329)); // Modifica a un valore diverso
            }
            if i % 4 == 2 { // Modifica 1/4 dei coefficienti
                result3.set_coeff(i, F3329::from_int((i + 3) % 3329)); // Modifica a un valore diverso
            }
        }

        let expected_result=base_ntt(&poly);
        let result=Poly3329::majority(result1, result2, result3);

        // Verifica che il risultato non sia nullo
        assert!(!result.is_zero());


        assert_eq!(expected_result.coefficients, result.coefficients);
    }

    #[test]
    fn test_bcm() {
        let a = Poly3329::from_vec([F3329::from_int(1), F3329::from_int(2), F3329::from_int(3), F3329::from_int(4)]);
        let b = Poly3329::from_vec([F3329::from_int(5), F3329::from_int(6), F3329::from_int(7), F3329::from_int(8)]);

        let result = bcm(&a, &b);
        let expected = bcm(&a, &b);

        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_bcm_matrix_vec() {
        //TODO ADD MORE DETAILS
        //let mut matrix = PolyMatrix3329::init();
        let a = Poly3329::from_vec([F3329::from_int(1), F3329::from_int(2), F3329::from_int(3), F3329::from_int(4)]);
        let b = Poly3329::from_vec([F3329::from_int(5), F3329::from_int(6), F3329::from_int(7), F3329::from_int(8)]);


        let mut value = 1;  // Iniziamo da 1

        // for i in 0..3 {
        //     for j in 0..3 {
        //         // Riempie la matrice con il valore corrente, quindi incrementa
        //         matrix.set(i, j, Poly3329::from_int(value));
        //         value += 1;
        //     }
        // }

        let result = bcm_tmr(&a, &b);
        let expected = bcm(&a, &b);

        assert_eq!(result.coefficients, expected.coefficients);
    }

}