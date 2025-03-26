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

use flint_sys::fmpq_mat::*;
use std::mem::MaybeUninit;
use crate::*;

impl_from! {
    RatMat, IntMat
    {
        fn from(x: &IntMat) -> RatMat {
            let mut z = MaybeUninit::uninit();
            unsafe {
                fmpq_mat_init(z.as_mut_ptr(), x.nrows() as i64, x.ncols() as i64);
                fmpq_mat_set_fmpz_mat(z.as_mut_ptr(), x.as_ptr());
                RatMat::from_raw(z.assume_init())
            }
        }
    }
}

// impl_from! {
//     RatMat, IntModMat
//     {
//         fn from(x: &IntModMat) -> RatMat {
//             RatMat::from(IntMat::from(x))
//         }
//     }
// }
