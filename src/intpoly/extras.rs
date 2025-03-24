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
    fmpz_types,
    fmpz_poly,
    fmpq_types,
    fmpq_poly
};
use libc::{c_int, c_long, c_ulong};
use std::mem::MaybeUninit;


#[inline]
pub unsafe fn fmpz_poly_equal_fmpq(
    f: *const fmpz_types::fmpz_poly_struct,
    x: *const flint::fmpq,
) -> c_int {
    if fmpz::fmpz_is_one(&(*x).den) == 1 {
        fmpz_poly::fmpz_poly_equal_fmpz(f, &(*x).num)
    } else {
        0
    }
}

#[inline]
pub unsafe fn fmpz_poly_equal_ui(f: *const fmpz_types::fmpz_poly_struct, x: c_ulong) -> c_int {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), x);
    let b = fmpz_poly::fmpz_poly_equal_fmpz(f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
    b
}

#[inline]
pub unsafe fn fmpz_poly_equal_si(f: *const fmpz_types::fmpz_poly_struct, x: c_long) -> c_int {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), x);
    let b = fmpz_poly::fmpz_poly_equal_fmpz(f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
    b
}

#[inline]
pub unsafe fn fmpz_poly_add_fmpq(
    res: *mut fmpq_types::fmpq_poly_struct,
    f: *const fmpz_types::fmpz_poly_struct,
    x: *const flint::fmpq,
) {
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, f);
    fmpq_poly::fmpq_poly_add_fmpq(res, res, x);
}

#[inline]
pub unsafe fn fmpz_poly_sub_fmpq(
    res: *mut fmpq_types::fmpq_poly_struct,
    f: *const fmpz_types::fmpz_poly_struct,
    x: *const flint::fmpq,
) {
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, f);
    fmpq_poly::fmpq_poly_sub_fmpq(res, res, x);
}

#[inline]
pub unsafe fn fmpz_poly_scalar_mul_fmpq(
    res: *mut fmpq_types::fmpq_poly_struct,
    f: *const fmpz_types::fmpz_poly_struct,
    x: *const flint::fmpq,
) {
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, f);
    fmpq_poly::fmpq_poly_scalar_mul_fmpq(res, res, x);
}

#[inline]
pub unsafe fn fmpz_poly_scalar_div_fmpq(
    res: *mut fmpq_types::fmpq_poly_struct,
    f: *const fmpz_types::fmpz_poly_struct,
    x: *const flint::fmpq,
) {
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, f);
    fmpq_poly::fmpq_poly_scalar_div_fmpq(res, res, x);
}

#[inline]
pub unsafe fn fmpz_poly_add_ui(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: *const fmpz_types::fmpz_poly_struct,
    x: c_ulong,
) {
    fmpz_poly::fmpz_poly_set_ui(res, x);
    fmpz_poly::fmpz_poly_add(res, f, res);
}

#[inline]
pub unsafe fn fmpz_poly_sub_ui(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: *const fmpz_types::fmpz_poly_struct,
    x: c_ulong,
) {
    fmpz_poly::fmpz_poly_set_ui(res, x);
    fmpz_poly::fmpz_poly_sub(res, f, res);
}

#[inline]
pub unsafe fn fmpz_poly_scalar_mod_ui(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: *const fmpz_types::fmpz_poly_struct,
    x: c_ulong,
) {
    fmpz_poly::fmpz_poly_set_ui(res, x);
    fmpz_poly::fmpz_poly_rem(res, f, res);
}

#[inline]
pub unsafe fn fmpz_poly_scalar_mod_si(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: *const fmpz_types::fmpz_poly_struct,
    x: c_long,
) {
    fmpz_poly::fmpz_poly_set_si(res, x);
    fmpz_poly::fmpz_poly_rem(res, f, res);
}

#[inline]
pub unsafe fn fmpz_poly_fmpq_add(
    res: *mut fmpq_types::fmpq_poly_struct,
    f: *const flint::fmpq,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, g);
    fmpq_poly::fmpq_poly_add_fmpq(res, res, f);
}

#[inline]
pub unsafe fn fmpz_poly_fmpq_sub(
    res: *mut fmpq_types::fmpq_poly_struct,
    f: *const flint::fmpq,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, g);
    fmpq_poly::fmpq_poly_sub_fmpq(res, res, f);
    fmpq_poly::fmpq_poly_neg(res, res);
}

#[inline]
pub unsafe fn fmpz_poly_fmpq_scalar_mul(
    res: *mut fmpq_types::fmpq_poly_struct,
    f: *const flint::fmpq,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, g);
    fmpq_poly::fmpq_poly_scalar_mul_fmpq(res, res, f);
}

#[inline]
pub unsafe fn fmpz_poly_fmpz_add(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: *const flint::fmpz,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_add_fmpz(res, g, f);
}

#[inline]
pub unsafe fn fmpz_poly_fmpz_scalar_mul(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: *const flint::fmpz,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_scalar_mul_fmpz(res, g, f);
}

#[inline]
pub unsafe fn fmpz_poly_fmpz_scalar_mod(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: *const flint::fmpz,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_set_fmpz(res, f);
    fmpz_poly::fmpz_poly_rem(res, res, g);
}

#[inline]
pub unsafe fn fmpz_poly_ui_add(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: c_ulong,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_set_ui(res, f);
    fmpz_poly::fmpz_poly_add(res, res, g);
}

#[inline]
pub unsafe fn fmpz_poly_ui_sub(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: c_ulong,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_set_ui(res, f);
    fmpz_poly::fmpz_poly_sub(res, res, g);
}

#[inline]
pub unsafe fn fmpz_poly_ui_scalar_mul(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: c_ulong,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_scalar_mul_ui(res, g, f);
}

#[inline]
pub unsafe fn fmpz_poly_ui_scalar_mod(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: c_ulong,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_set_ui(res, f);
    fmpz_poly::fmpz_poly_rem(res, res, g);
}

#[inline]
pub unsafe fn fmpz_poly_si_add(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: c_long,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_add_si(res, g, f);
}

#[inline]
pub unsafe fn fmpz_poly_si_sub(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: c_long,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_sub_si(res, g, f);
    fmpz_poly::fmpz_poly_neg(res, res);
}

#[inline]
pub unsafe fn fmpz_poly_si_scalar_mul(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: c_long,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_scalar_mul_si(res, g, f);
}

#[inline]
pub unsafe fn fmpz_poly_si_scalar_mod(
    res: *mut fmpz_types::fmpz_poly_struct,
    f: c_long,
    g: *const fmpz_types::fmpz_poly_struct,
) {
    fmpz_poly::fmpz_poly_set_si(res, f);
    fmpz_poly::fmpz_poly_rem(res, res, g);
}

