//! Algebraics
//!
//! Definiton of basic algebraic structures (Ring, Field, Polynomial, Vector, Matrix)

mod matrix;
mod polynomial;
mod polyvec;

pub use matrix::Matrix;
pub use polynomial::Polynomial;
pub use polyvec::PolyVec;


//Finite Group Element (cyclic)
    pub trait FiniteGroup: Sized+Eq { //sized, so the size must be known at compile time
    //check if the element is the additive identity
    fn is_zero(&self)-> bool;

    //returns the additive identity
    fn zero()-> Self;

    //returns the additive inverse of the element
    fn neg(&self)-> Self;
     // defines the addition of 2 elements
    fn add(&self, other: &Self) -> Self;

    //defines the subtraction of 2 elements
    fn sub(&self, other: &Self)-> Self;

}

//finite ring element

    pub trait FiniteRing: Sized+Eq {
        //check if the element is the additive identity
        fn is_zero(&self)-> bool;

        //returns the additive identity
        fn zero()-> Self;

        //returns the additive inverse of the element
        fn neg(&self)-> Self;

        // defines the addition of 2 elements
        fn add(&self, other: &Self) -> Self;

        //defines the subtraction of 2 elements
        fn sub(&self, other: &Self)-> Self;

        //returns the multiplicative identity
        fn one()-> Self;

        //defines the multiplication of 2 elements
        fn mul(&self, other: &Self) -> Self;

    }

    pub trait FiniteField: Sized+ Eq {
        //check if the element is the additive identity
        fn is_zero(&self)-> bool;

        //returns the additive identity
        fn zero()-> Self;

        //returns the additive inverse of the element
        fn neg(&self)-> Self;

        // defines the addition of 2 elements
        fn add(&self, other: &Self) -> Self;

        //defines the subtraction of 2 elements
        fn sub(&self, other: &Self)-> Self;

        //returns the multiplicative identity
        fn one()-> Self;

        //defines the multiplication of 2 elements
        fn mul(&self, other: &Self) -> Self;

        //returns the dimension of the finite field
        fn dimension() -> usize;

        //returns the multiplicative inverse of the element
        fn inv(&self) -> Result<Self, String>;

        //defines the division of 2 elements
        fn div(&self, other: &Self) -> Result<Self, String>;

    }

    /// the vectorSpace trait describes the general properties of an element in a vector space, how to handle and perform operations
    pub trait VectorSpace<T: FiniteField> {
        /// Check if the element is the additive identity
        fn is_zero(&self) -> bool;

        /// Returns the additive identity
        fn zero() -> Self;

        /// Returns the additive inverse of the element
        fn neg(&self) -> Self;

        /// Defines the addition of two elements
        fn add(&self, other: &Self) -> Self;

        /// Defines the substraction of two elements
        fn sub(&self, other: &Self) -> Self;

        /// Returns the vector's dimension
        fn dimension() -> usize;

        /// Initialise vector type
        fn init() -> Self;

        /// Scalar multiplication
        fn mulf(&self, other: &T) -> Self;

        /// Basis vector
        fn basis_vector(position: usize) -> Self;

        /// Set coefficient
        fn set(&mut self, position: usize, value: T);

        /// Get coefficient
        fn get(&self, position: usize) -> T;

        /// Dot product
        fn dot(&self, other: &Self) -> T;
    }

/// The `Ring` trait describes the general properties of an element in a module.
pub trait RingModule<T: FiniteRing> {
    /// Check if the element is the additive identity
    fn is_zero(&self) -> bool;

    /// Returns the additive identity
    fn zero() -> Self;

    /// Returns the additive inverse of the element
    fn neg(&self) -> Self;

    /// Defines the addition of two elements
    fn add(&self, other: &Self) -> Self;

    /// Defines the substraction of two elements
    fn sub(&self, other: &Self) -> Self;

    /// Returns the vector's dimension
    fn dimension() -> usize;

    /// Initialise vector type
    fn init() -> Self;

    /// Scalar multiplication
    fn mulf(&self, other: &T) -> Self;

    /// Basis vector
    fn basis_vector(position: usize) -> Self;

    /// Set coefficient
    fn set(&mut self, position: usize, value: T);

    /// Get coefficient
    fn get(&self, position: usize) -> T;

    /// Dot product
    fn dot(&self, other: &Self) -> T;
}
