/*
 *  Copyright (C) 2024 William Youmans
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod ops;
mod conv;
mod extras;

//#[cfg(feature = "serde")]
//mod serde;

use crate::Integer;
use flint_sys::{flint, fmpz, fmpq};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};


#[derive(Debug)]
pub struct Rational {
    inner: flint::fmpq,
}

impl AsRef<Rational> for Rational {
    #[inline]
    fn as_ref(&self) -> &Rational {
        self
    }
}

impl Clone for Rational {
    #[inline]
    fn clone(&self) -> Self {
        let mut res = Rational::default();
        unsafe {
            fmpq::fmpq_set(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
}

impl Default for Rational {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpq::fmpq_init(z.as_mut_ptr());
            Rational::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for Rational {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.numerator();
        let d = self.denominator();
        if d == 1 {
            write!(f, "{}", n)
        } else {
            write!(f, "{}/{}", n, d)
        }
    }
}

impl Drop for Rational {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpq::fmpq_clear(self.as_mut_ptr()) }
    }
}

impl Hash for Rational {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator().hash(state);
        self.denominator().hash(state);
    }
}

impl Rational {
    /// Returns a pointer to the inner [FLINT fmpq][flint::fmpq].
    #[inline]
    pub const fn as_ptr(&self) -> *const flint::fmpq {
        &self.inner
    }

    /// Returns a mutable pointer to the inner [FLINT fmpq][flint::fmpq].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut flint::fmpq {
        &mut self.inner
    }

    /// Create a Rational from an initialized [FLINT fmpq][flint::fmpq].
    ///
    /// # Safety
    ///
    ///   * The function must *not* be used to create a constant [`Rational`],
    ///     though it can be used to create a static [`Rational`]. This is
    ///     because constant values are *copied* on use, leading to undefined
    ///     behavior when they are dropped.
    ///   * The value must be initialized.
    ///   * The [`flint::fmpq`] type can be considered as a kind of pointer, so there
    ///     can be multiple copies of it. Since this function takes over
    ///     ownership, no other copies of the passed value should exist.
    #[inline]
    pub const unsafe fn from_raw(raw: flint::fmpq) -> Rational {
        Rational { inner: raw }
    }
    
    /// The returned object should be freed to avoid memory leaks.
    #[inline]
    pub const fn into_raw(self) -> flint::fmpq {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }
}

impl Rational {
    #[inline]
    pub fn new<T: Into<Rational>>(src: T) -> Self {
        src.into()
    }

    /// Return zero.
    ///
    /// ```
    /// use rufl::rational::Rational;
    ///
    /// assert_eq!(Rational::zero(), 0);
    /// ```
    #[inline]
    pub fn zero() -> Rational {
        Rational::default()
    }

    /// Return one.
    ///
    /// ```
    /// use rufl::rational::Rational;
    ///
    /// assert_eq!(Rational::one(), 1);
    /// ```
    #[inline]
    pub fn one() -> Rational {
        let mut res = Rational::default();
        unsafe { fmpq::fmpq_one(res.as_mut_ptr()); }
        res
    }

    /// Set the rational to zero.
    ///
    /// ```
    /// use rufl::rational::Rational;
    /// 
    /// let mut a = Rational::new(5);
    /// a.zero_mut();
    /// assert!(a.is_zero());
    /// ```
    #[inline]
    pub fn zero_mut(&mut self) {
        unsafe { fmpq::fmpq_zero(self.as_mut_ptr()) }
    }
    
    /// Set the rational to one.
    ///
    /// ```
    /// use rufl::rational::Rational;
    /// 
    /// let mut a = Rational::new(5);
    /// a.one_mut();
    /// assert!(a.is_one());
    /// ```
    #[inline]
    pub fn one_mut(&mut self) {
        unsafe { fmpq::fmpq_one(self.as_mut_ptr()) }
    }
    
    /// Return true if the `Rational` is zero.
    ///
    /// ```
    /// use rufl::rational::Rational;
    ///
    /// let x = Rational::from(0u32);
    /// assert!(x.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpq::fmpq_is_zero(self.as_ptr()) == 1 }
    }

    /// Return true if the `Rational` is one.
    ///
    /// ```
    /// use rufl::rational::Rational;
    ///
    /// let x = Rational::from(1i16);
    /// assert!(x.is_one());
    /// ```
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpq::fmpq_is_one(self.as_ptr()) == 1 }
    }

    /// Returns the numerator of a rational number as an [Integer].
    ///
    /// ```
    /// use rufl::rational::Rational;
    ///
    /// let q = Rational::from([3, 4]);
    /// assert_eq!(q.numerator(), 3);
    /// ```
    #[inline]
    pub fn numerator(&self) -> Integer {
        let mut res = Integer::zero();
        unsafe {
            fmpz::fmpz_set(res.as_mut_ptr(), &self.inner.num)
        }
        res
    }

    /// Returns the denominator of a rational number as an [Integer].
    ///
    /// ```
    /// use rufl::rational::Rational;
    ///
    /// let q = Rational::from([3, 4]);
    /// assert_eq!(q.denominator(), 4);
    /// ```
    #[inline]
    pub fn denominator(&self) -> Integer {
        let mut res = Integer::zero();
        unsafe {
            fmpz::fmpz_set(res.as_mut_ptr(), &self.inner.den)
        }
        res
    }

    /*
    #[inline]
    pub fn floor(&self) -> Integer {
        let mut res = self.numerator();
        res.fdiv_q_assign(self.denominator());
        res
    }

    #[inline]
    pub fn ceil(&self) -> Integer {
        let mut res = self.numerator();
        res.cdiv_q_assign(self.denominator());
        res
    }
    
    #[inline]
    pub fn round(&self) -> Integer {
        let mut res = self.numerator();
        res.tdiv_q_assign(self.denominator());
        res
    }
    
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe {
            fmpq::fmpq_sgn(self.as_ptr())
        }
    }

    #[inline]
    pub fn abs(&self) -> Rational {
        unsafe {
            let mut res = Rational::default();
            fmpq::fmpq_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
    
    #[inline]
    pub fn abs_assign(&mut self) {
        unsafe {
            fmpq::fmpq_abs(self.as_mut_ptr(), self.as_ptr());
        }
    }

    #[inline]
    pub fn height(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            fmpq::fmpq_height(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
    */
}

