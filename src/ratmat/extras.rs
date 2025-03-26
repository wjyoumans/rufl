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

use flint_sys::flint::fmpz;
use flint_sys::fmpz::*;
use flint_sys::fmpq_mat::*;
use flint_sys::fmpz_types::fmpz_mat_struct;
use flint_sys::fmpq_types::fmpq_mat_struct;
use libc::{c_long, c_ulong};
use std::mem::MaybeUninit;


#[inline]
pub unsafe fn fmpq_mat_scalar_mul_ui(
    res: *mut fmpq_mat_struct,
    f: *const fmpq_mat_struct,
    g: c_ulong,
) {
    let mut z = MaybeUninit::uninit();
    fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpq_mat_scalar_mul_fmpz(res, f, z.as_ptr());
    fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpq_mat_scalar_mul_si(
    res: *mut fmpq_mat_struct,
    f: *const fmpq_mat_struct,
    g: c_long,
) {
    let mut z = MaybeUninit::uninit();
    fmpz_init_set_si(z.as_mut_ptr(), g);
    fmpq_mat_scalar_mul_fmpz(res, f, z.as_ptr());
    fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpq_mat_fmpz_scalar_mul(
    res: *mut fmpq_mat_struct,
    f: *const fmpz,
    g: *const fmpq_mat_struct,
) {
    fmpq_mat_scalar_mul_fmpz(res, g, f);
}

/*
#[inline]
unsafe fn fmpq_mat_fmpq_scalar_mul(
    res: *mut fmpq_mat_struct,
    f: *const fmpq,
    g: *const fmpq_mat_struct,
) {
    fmpq_mat_scalar_mul_fmpq(res, g, f);
}*/

#[inline]
pub unsafe fn fmpq_mat_ui_scalar_mul(
    res: *mut fmpq_mat_struct,
    f: c_ulong,
    g: *const fmpq_mat_struct,
) {
    fmpq_mat_scalar_mul_ui(res, g, f);
}

#[inline]
pub unsafe fn fmpq_mat_si_scalar_mul(
    res: *mut fmpq_mat_struct,
    f: c_long,
    g: *const fmpq_mat_struct,
) {
    fmpq_mat_scalar_mul_si(res, g, f);
}

#[inline]
pub unsafe fn fmpq_mat_get_fmpz_mat_mod_ui(
    res: *mut fmpz_mat_struct,
    f: *const fmpq_mat_struct,
    g: c_ulong,
) {
    let mut z = MaybeUninit::uninit();
    fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpq_mat_get_fmpz_mat_mod_fmpz(res, f, z.as_ptr());
    fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpq_mat_get_fmpz_mat_mod_si(
    res: *mut fmpz_mat_struct,
    f: *const fmpq_mat_struct,
    g: c_long,
) {
    let mut z = MaybeUninit::uninit();
    fmpz_init_set_si(z.as_mut_ptr(), g);
    fmpq_mat_get_fmpz_mat_mod_fmpz(res, f, z.as_ptr());
    fmpz_clear(z.as_mut_ptr());
}