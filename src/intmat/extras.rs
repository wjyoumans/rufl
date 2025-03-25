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

use flint_sys::{
    flint,
    fmpz,
    fmpz_mat
};
use flint_sys::fmpz_types::fmpz_mat_struct;
use libc::{c_long, c_ulong};
use std::mem::MaybeUninit;


#[inline]
pub unsafe fn fmpz_mat_fmpz_scalar_mul(
    res: *mut fmpz_mat_struct,
    f: *const flint::fmpz,
    g: *const fmpz_mat_struct,
) {
    fmpz_mat::fmpz_mat_scalar_mul_fmpz(res, g, f);
}

#[inline]
pub unsafe fn fmpz_mat_ui_scalar_mul(
    res: *mut fmpz_mat_struct,
    f: c_ulong,
    g: *const fmpz_mat_struct,
) {
    fmpz_mat::fmpz_mat_scalar_mul_ui(res, g, f);
}

#[inline]
pub unsafe fn fmpz_mat_si_scalar_mul(
    res: *mut fmpz_mat_struct,
    f: c_long,
    g: *const fmpz_mat_struct,
) {
    fmpz_mat::fmpz_mat_scalar_mul_si(res, g, f);
}

#[inline]
pub unsafe fn fmpz_mat_scalar_mod_ui(
    res: *mut fmpz_mat_struct,
    f: *const fmpz_mat_struct,
    g: c_ulong,
) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpz_mat::fmpz_mat_scalar_mod_fmpz(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpz_mat_scalar_mod_si(
    res: *mut fmpz_mat_struct,
    f: *const fmpz_mat_struct,
    g: c_long,
) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), g);
    fmpz_mat::fmpz_mat_scalar_mod_fmpz(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}