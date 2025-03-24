/*
 *  Copyright (C) 2021 William Youmans
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

use crate::*;
use flint_sys::fmpz_mat;
//use std::mem::MaybeUninit;


/*
impl_from! {
    IntMat, IntModMat
    {
        fn from(x: &IntModMat) -> IntMat {
            unsafe {
                let mut z = MaybeUninit::uninit();
                fmpz_mat::fmpz_mat_init_set(z.as_mut_ptr(), &(*x.as_ptr()).mat[0]);
                IntMat::from_raw(z.assume_init())
            }
        }
    }
}
*/

/*
impl_tryfrom! {
    IntMat, RatMat
    {
        fn try_from(x: &RatMat) -> Result<Self,Self::Error> {
            let zm = IntMatSpace::init(x.nrows(), x.ncols());
            let mut res = zm.default();
            unsafe {
                let b = fmpq_mat::fmpq_mat_get_fmpz_mat(res.as_mut_ptr(), x.as_ptr());
                if b != 0 {
                    Ok(res)
                } else {
                    Err("RatMat could not be coerced to an IntMat.")
                }
            }
        }
    }
}
*/

impl<T, const CAP: usize> From<[T; CAP]> for IntMat 
where
    T: Into<Integer>
{
    fn new(src: [T; CAP], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = IntMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.into_iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x.into());
        }
        res
    }
}


impl<const CAP: usize> From<[&Integer; CAP]> for IntMat {
    fn new(src: [&Integer; CAP], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = IntMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.into_iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x);
        }
        res
    }
}

impl<'a, T> From<&'a [T]> for IntMat
where
    &'a T: Into<Integer>
{
    fn new(src: &'a [T], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = IntMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x.into());
        }
        res
    }
}

impl From<&[Integer]> for IntMat {
    fn new(src: &[Integer], nrows: i64, ncols: i64) -> Self {
        let nrows_ui: usize = nrows.try_into().expect(
            "Cannot convert signed long to usize.");
        let ncols_ui: usize = ncols.try_into().expect(
            "Cannot convert signed long to usize.");
        
        assert_eq!(src.len(), nrows_ui * ncols_ui);
        let mut res = IntMat::zero(nrows, ncols);

        let mut col;
        let mut row = 0usize;
        for (i, x) in src.iter().enumerate() {
            col = i % ncols_ui;
            if col == 0 && i != 0 {
                row += 1;
            }
            res.set_entry(row, col, x);
        }
        res
    }
}

