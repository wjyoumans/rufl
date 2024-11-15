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

//#[cfg(feature = "serde")]
//mod serde;

//pub mod macros;

use flint_sys::{flint, fmpz};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};

#[derive(Debug)]
pub struct Integer {
    inner: flint::fmpz,
}

impl AsRef<Integer> for Integer {
    #[inline]
    fn as_ref(&self) -> &Integer {
        self
    }
}

impl Clone for Integer {
    #[inline]
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init_set(z.as_mut_ptr(), self.as_ptr());
            Integer::from_raw(z.assume_init())
        }
    }
}

impl Default for Integer {
    #[inline]
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init(z.as_mut_ptr());
            Integer::from_raw(z.assume_init())
        }
    }
}

impl fmt::Display for Integer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str_radix(10))
    }
}

impl Drop for Integer {
    #[inline]
    fn drop(&mut self) {
        unsafe { fmpz::fmpz_clear(self.as_mut_ptr()) }
    }
}

impl Hash for Integer {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ui_vector().hash(state);
    }
}


impl Integer {
    /// Returns a pointer to the inner [FLINT integer][fmpz::fmpz].
    #[inline]
    pub const fn as_ptr(&self) -> *const flint::fmpz {
        &self.inner

    }

    /// Returns a mutable pointer to the inner [FLINT integer][fmpz::fmpz].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut flint::fmpz {
        &mut self.inner

    }

    /// Create an Integer from an initialized [FLINT integer][fmpz::fmpz].
    ///
    /// # Safety
    ///
    ///   * The function must *not* be used to create a constant [`Integer`],
    ///     though it can be used to create a static [`Integer`]. This is
    ///     because constant values are *copied* on use, leading to undefined
    ///     behavior when they are dropped.
    ///   * The value must be initialized.
    ///   * The [`fmpz::fmpz`] type can be considered as a kind of pointer, so there
    ///     can be multiple copies of it. Since this function takes over
    ///     ownership, no other copies of the passed value should exist.
    #[inline]
    pub const unsafe fn from_raw(raw: flint::fmpz) -> Integer {
        Integer { inner: raw }
    }

    /// The returned object should be freed to avoid memory leaks.
    #[inline]
    pub const fn into_raw(self) -> flint::fmpz {
        let ret = self.inner;
        let _ = ManuallyDrop::new(self);
        ret
    }    
}


impl Integer {
    pub fn new<T: Into<Integer>>(x: T) -> Integer {
        x.into()
    }
    
    /// Initialize a new `Integer` with the given number of limbs.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let x = Integer::with_capacity(2);
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    pub fn with_capacity(limbs: u64) -> Integer {
        let mut z = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init2(z.as_mut_ptr(), limbs);
            Integer::from_raw(z.assume_init())
        }
    }

    /// Return zero.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// assert_eq!(Integer::zero(), 0);
    /// ```
    #[inline]
    pub fn zero() -> Integer {
        Integer::default()
    }

    /// Return one.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// assert_eq!(Integer::one(), 1);
    /// ```
    #[inline]
    pub fn one() -> Integer {
        let mut res = Integer::default();
        unsafe { fmpz::fmpz_one(res.as_mut_ptr()); }
        res
    }

    /// Set the integer to zero.
    ///
    /// ```
    /// use rufl::integer::Integer;
    /// 
    /// let mut a = Integer::new(5);
    /// a.zero_mut();
    /// assert!(a.is_zero());
    /// ```
    #[inline]
    pub fn zero_mut(&mut self) {
        unsafe { fmpz::fmpz_zero(self.as_mut_ptr()) }
    }
    
    /// Set the integer to one.
    ///
    /// ```
    /// use rufl::integer::Integer;
    /// 
    /// let mut a = Integer::new(5);
    /// a.one_mut();
    /// assert!(a.is_one());
    /// ```
    #[inline]
    pub fn one_mut(&mut self) {
        unsafe { fmpz::fmpz_one(self.as_mut_ptr()) }
    }


    /// Return an `Option` containing the input as a signed long if possible.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(-1234);
    /// assert_eq!(z.get_si().unwrap(), -1234);
    /// ```
    #[inline]
    pub fn get_si(&self) -> Option<i64> {
        if self.fits_si() {
            unsafe { Some(fmpz::fmpz_get_si(self.as_ptr())) }
        } else {
            None
        }
    }

    /// Return an `Option` containing the input as an unsigned long if possible.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(1234);
    /// assert_eq!(z.get_ui().unwrap(), 1234);
    ///
    /// let z = Integer::from(-1234);
    /// assert!(z.get_ui().is_none());
    /// ```
    #[inline]
    pub fn get_ui(&self) -> Option<u64> {
        if self.sign() < 0 {
            return None;
        }
        if self.abs_fits_ui() {
            unsafe { Some(flint_sys::fmpz::fmpz_get_ui(self.as_ptr())) }
        } else {
            None
        }
    }

    /// Return a vector `A` of unsigned longs such that the original [Integer] can
    /// be written as `a[0] + a[1]*x + ... + a[n-1]*x^(n-1)` where `x = 2^FLINT_BITS`.
    ///
    /// ```
    /// use rufl::integer::Integer;
    /// use rufl::ops::Pow;
    ///
    /// let z = Integer::from(2).pow(65u8);
    /// let v = z.get_ui_vector();
    /// assert!(v == vec![0, 2]);
    ///
    /// let mut t = Integer::default();
    /// t.set_ui_vector(v);
    /// assert_eq!(z, t);
    ///
    /// let x: Integer = "18446744073709551616".parse().unwrap();
    /// let v = x.get_ui_vector();
    /// let mut t = Integer::default();
    /// t.set_ui_vector(v);
    /// assert_eq!(x, t);
    /// ```
    #[inline]
    pub fn get_ui_vector(&self) -> Vec<u64> {
        if self.is_zero() {
            vec![]
        } else {
            let n = self.size();
            let mut out = Vec::with_capacity(n as usize);
            unsafe {
                fmpz::fmpz_get_ui_array(out.as_mut_ptr(), n, self.as_ptr());
                out.set_len(n as usize);
            }
            out
        }
    }

    /// Set `self` to the nonnegative [Integer]
    /// `vec[0] + vec[1]*x + ... + vec[n-1]*x^(n-1)` where `x = 2^FLINT_BITS`.
    ///
    /// ```
    /// use rufl::integer::Integer;
    /// use rufl::ops::Pow;
    ///
    /// let mut z = Integer::default();
    /// z.set_ui_vector(vec![0,2]);
    /// assert_eq!(z, Integer::from(2).pow(65u8));
    /// ```
    #[inline]
    pub fn set_ui_vector(&mut self, vec: Vec<u64>) {
        if vec.is_empty() {
            self.zero_mut();
        } else {
            unsafe {
                fmpz::fmpz_set_ui_array(
                    self.as_mut_ptr(), vec.as_ptr(), vec.len() as i64);
            }
        }
    }

    /// Convert the `Integer` to a string in base `base`.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let x = Integer::from(1024);
    /// assert_eq!(x.to_str_radix(2), "10000000000")
    /// ```
    pub fn to_str_radix(&self, base: u8) -> String {
        unsafe {
            // Extra two bytes are for possible minus sign and null terminator
            let len = fmpz::fmpz_sizeinbase(self.as_ptr(), base as i32) as usize + 2;

            // Allocate and write into a raw *c_char of the correct length
            let mut vector: Vec<u8> = Vec::with_capacity(len);
            fmpz::fmpz_get_str(vector.as_mut_ptr() as *mut _, 
                               base as i32, self.as_ptr());
            
            vector.set_len(len);
            let mut first_nul = None;
            let mut index: usize = 0;
            for elem in &vector {
                if *elem == 0 {
                    first_nul = Some(index);
                    break;
                }
                index += 1;
            }
            let first_nul = first_nul.unwrap_or(len);

            vector.truncate(first_nul);
            match String::from_utf8(vector) {
                Ok(s) => s,
                Err(_) => panic!("FLINT returned invalid UTF-8!"),
            }
        }
    }

    /// Determines the size of the absolute value of an `Integer` in base `base`
    /// in terms of number of digits. The base can be between 2 and 62, inclusive.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(1000001);
    /// assert_eq!(8, z.sizeinbase(7));
    /// ```
    #[inline]
    pub fn sizeinbase(&self, base: i32) -> usize {
        assert!(1 < base && base < 63);
        unsafe { fmpz::fmpz_sizeinbase(self.as_ptr(), base) }
    }

    /// Returns the number of bits required to store the absolute value of an
    /// `Integer`. Returns zero if the `Integer` is zero.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let x = Integer::from(16);
    /// assert_eq!(x.bits(), 5);
    /// ```
    #[inline]
    pub fn bits(&self) -> u64 {
        unsafe { fmpz::fmpz_bits(self.as_ptr()) }
    }

    /// Returns the number of limbs required to store the absolute value of an
    /// `Integer`. Returns zero if the `Integer` is zero.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z: Integer = "18446744073709551616".parse().unwrap();
    /// assert_eq!(2, z.size());
    /// ```
    #[inline]
    pub fn size(&self) -> i64 {
        unsafe { flint_sys::fmpz::fmpz_size(self.as_ptr()) }
    }
    
    /// Returns -1 if the `Integer` is negative, +1 if the `Integer`
    /// is positive, and 0 otherwise.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(-12);
    /// assert_eq!(z.sign(), -1);
    ///
    /// let z = Integer::from(0);
    /// assert_eq!(z.sign(), 0);
    ///
    /// let z = Integer::from(12);
    /// assert_eq!(z.sign(), 1);
    /// ```
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe { fmpz::fmpz_sgn(self.as_ptr()) }
    }

    /// Determine if the `Integer` fits in a signed long.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z: Integer = "18446744073709551616".parse().unwrap();
    /// assert_eq!(z.fits_si(), false);
    /// ```
    #[inline]
    pub fn fits_si(&self) -> bool {
        unsafe { fmpz::fmpz_fits_si(self.as_ptr()) == 1 }
    }

    /// Determine if the absolute value of an `Integer` fits in an unsigned long.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z: Integer = "18446744073709551614".parse().unwrap();
    /// assert_eq!(z.abs_fits_ui(), true);
    /// ```
    #[inline]
    pub fn abs_fits_ui(&self) -> bool {
        unsafe { fmpz::fmpz_abs_fits_ui(self.as_ptr()) == 1 }
    }
    
    /// Sets the bit index `bit_index` of an `Integer`.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let mut z = Integer::from(1024);
    /// z.setbit(0);
    /// assert_eq!(1025, z);
    /// ```
    #[inline]
    pub fn setbit(&mut self, bit_index: u64) {
        unsafe { fmpz::fmpz_setbit(self.as_mut_ptr(), bit_index) }
    }

    /// Test the bit index `bit_index` of an `Integer`. Return `true` if it is 1,
    /// `false` if it is zero.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(1025);
    /// assert!(z.testbit(0));
    /// ```
    #[inline]
    pub fn testbit(&self, bit_index: u64) -> bool {
        unsafe { fmpz::fmpz_tstbit(self.as_ptr(), bit_index) == 1 }
    }

    // Comparison //
    
    /// Return true if the `Integer` is zero.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let x = Integer::from(0u32);
    /// assert!(x.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        unsafe { fmpz::fmpz_is_zero(self.as_ptr()) == 1 }
    }

    /// Return true if the `Integer` is one.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let x = Integer::from(1i16);
    /// assert!(x.is_one());
    /// ```
    #[inline]
    pub fn is_one(&self) -> bool {
        unsafe { fmpz::fmpz_is_one(self.as_ptr()) == 1 }
    }
    
    /// Return true if the `Integer` is 1 or -1.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let x = Integer::from(-1i32);
    /// assert!(x.is_pm1());
    /// ```
    #[inline]
    pub fn is_pm1(&self) -> bool {
        unsafe { fmpz::fmpz_is_pm1(self.as_ptr()) == 1 }
    }

    /// Check if the `Integer` is even.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(102);
    /// assert!(z.is_even());
    /// ```
    #[inline]
    pub fn is_even(&self) -> bool {
        unsafe { fmpz::fmpz_is_even(self.as_ptr()) == 1 }
    }

    /// Check if the `Integer` is odd.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(103);
    /// assert!(z.is_odd());
    /// ```
    #[inline]
    pub fn is_odd(&self) -> bool {
        unsafe { fmpz::fmpz_is_odd(self.as_ptr()) == 1 }
    }

    // Basic arithmetic //

    /// Returns the absolute value of an `Integer`
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(-99);
    /// assert_eq!(z.abs(), Integer::from(99));
    /// ```
    #[inline]
    pub fn abs(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            fmpz::fmpz_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }

    /// Set the input to its absolute value.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let mut z = Integer::from(-99);
    /// z.abs_mut();
    /// assert_eq!(z, Integer::from(99));
    /// ```
    #[inline]
    pub fn abs_mut(&mut self) {
        unsafe {
            fmpz::fmpz_abs(self.as_mut_ptr(), self.as_ptr());
        }
    }

    /// Outputs `self * x * y` where `x, y` can be converted to unsigned longs.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let f = Integer::from(-1);
    /// assert_eq!(f.mul2_uiui(10u32, 3u32), -30i32);
    /// ```
    #[inline]
    pub fn mul2_uiui<S>(&self, x: S, y: S) -> Integer
    where
        S: Into<u64>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_mul2_uiui(res.as_mut_ptr(), self.as_ptr(), 
                                 x.into(), y.into());
        }
        res
    }

    /// Set `self` to `self * x * y` where `x, y` can be converted to unsigned longs.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let mut f = Integer::from(-1);
    /// f.mul2_uiui_mut(10u8, 3u8);
    /// assert_eq!(f, -30);
    /// ```
    #[inline]
    pub fn mul2_uiui_mut<S>(&mut self, x: S, y: S)
    where
        S: Into<u64>,
    {
        unsafe {
            fmpz::fmpz_mul2_uiui(self.as_mut_ptr(), self.as_ptr(), 
                                 x.into(), y.into());
        }
    }

    /// Output `self * 2^exp`.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let g = Integer::from(2);
    /// assert_eq!(g.mul_2exp(3u64), 16);
    /// ```
    #[inline]
    pub fn mul_2exp<S>(&self, exp: S) -> Integer
    where
        S: Into<u64>,
    {
        let mut res = Integer::default();
        unsafe {
            fmpz::fmpz_mul_2exp(res.as_mut_ptr(), self.as_ptr(), exp.into());
        }
        res
    }

    /// Compute `self * 2^exp` in place.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let mut g = Integer::from(2);
    /// g.mul_2exp_mut(3u32);
    /// assert_eq!(g, 16);
    /// ```
    #[inline]
    pub fn mul_2exp_mut<S>(&mut self, exp: S)
    where
        S: Into<u64>,
    {
        unsafe {
            fmpz::fmpz_mul_2exp(self.as_mut_ptr(), self.as_ptr(), exp.into());
        }
    }
    
    /// Return `self + (x * y)`.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(2);
    /// assert_eq!(z.addmul(Integer::from(3), Integer::from(4)), 14);
    /// ```
    #[inline]
    pub fn addmul<T>(&self, x: T, y: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_addmul(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
        res
    }

    /// Compute `self + (x * y)` in place.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let mut z = Integer::from(2);
    /// z.addmul_mut(Integer::from(3), Integer::from(4));
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_mut<T>(&mut self, x: T, y: T)
    where
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_addmul(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
    }

    /// Return `self + (x * y)` where `y` can be converted to an unsigned long.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(2);
    /// assert_eq!(z.addmul_ui(Integer::from(3), 4u32), 14);
    /// ```
    #[inline]
    pub fn addmul_ui<S, T>(&self, x: T, y: S) -> Integer
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_addmul_ui(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
        res
    }

    /// Compute `self + (x * y)` in place where `y` can be converted to an
    /// unsigned long.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let mut z = Integer::from(2);
    /// z.addmul_ui_mut(Integer::from(3), 4u8);
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_ui_mut<S, T>(&mut self, x: T, y: S)
    where
        S: Into<u64>,
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_addmul_ui(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
    }
    
    /// Return `self + (x * y)` where `y` can be converted to a signed long.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(2);
    /// assert_eq!(z.addmul_si(Integer::from(3), 4i32), 14);
    /// ```
    #[inline]
    pub fn addmul_si<S, T>(&self, x: T, y: S) -> Integer
    where
        S: Into<i64>,
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_addmul_si(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
        res
    }

    /// Compute `self + (x * y)` in place where `y` can be converted to an
    /// signed long.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let mut z = Integer::from(2);
    /// z.addmul_si_mut(Integer::from(3), 4i8);
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_si_mut<S, T>(&mut self, x: T, y: S)
    where
        S: Into<i64>,
        T: AsRef<Integer>,
    {
        unsafe {
            fmpz::fmpz_addmul_si(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.into()
            );
        }
    }
    
    /// Return `self - (x * y)`.
    ///
    /// ```
    /// use rufl::integer::Integer;
    ///
    /// let z = Integer::from(13);
    /// assert_eq!(z.submul(Integer::from(3), Integer::from(4)), 1);
    /// ```
    #[inline]
    pub fn submul<T>(&self, x: T, y: T) -> Integer
    where
        T: AsRef<Integer>,
    {
        let mut res = self.clone();
        unsafe {
            fmpz::fmpz_submul(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
        res
    }
}
