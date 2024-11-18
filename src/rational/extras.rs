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

use flint_sys::{flint, fmpz, fmpq};
use libc::{c_int, c_long, c_ulong};
use std::mem::MaybeUninit;


#[inline]
pub unsafe fn fmpq_set_ui_den1(res: *mut flint::fmpq, f: c_ulong) {
    fmpq::fmpq_set_ui(res, f, 1);
}

#[inline]
pub unsafe fn fmpq_set_si_den1(res: *mut flint::fmpq, f: c_long) {
    fmpq::fmpq_set_si(res, f, 1);
}

#[inline]
pub unsafe fn fmpq_fmpz_add(res: *mut flint::fmpq, x: *const flint::fmpz, f: *const flint::fmpq) {
    fmpq::fmpq_add_fmpz(res, f, x);
}

#[inline]
pub unsafe fn fmpq_fmpz_sub(res: *mut flint::fmpq, x: *const flint::fmpz, f: *const flint::fmpq) {
    fmpq::fmpq_sub_fmpz(res, f, x);
    fmpq::fmpq_neg(res, res);
}

#[inline]
pub unsafe fn fmpq_fmpz_mul(res: *mut flint::fmpq, x: *const flint::fmpz, f: *const flint::fmpq) {
    fmpq::fmpq_mul_fmpz(res, f, x);
}

#[inline]
pub unsafe fn fmpq_fmpz_div(res: *mut flint::fmpq, x: *const flint::fmpz, f: *const flint::fmpq) {
    fmpq::fmpq_div_fmpz(res, f, x);
    fmpq::fmpq_inv(res, res);
}

#[inline]
pub unsafe fn fmpq_ui_add(res: *mut flint::fmpq, x: c_ulong, f: *const flint::fmpq) {
    fmpq::fmpq_add_ui(res, f, x);
}

#[inline]
pub unsafe fn fmpq_si_add(res: *mut flint::fmpq, x: c_long, f: *const flint::fmpq) {
    fmpq::fmpq_add_si(res, f, x);
}

#[inline]
pub unsafe fn fmpq_ui_sub(res: *mut flint::fmpq, x: c_ulong, f: *const flint::fmpq) {
    fmpq::fmpq_sub_ui(res, f, x);
    fmpq::fmpq_neg(res, res);
}

#[inline]
pub unsafe fn fmpq_si_sub(res: *mut flint::fmpq, x: c_long, f: *const flint::fmpq) {
    fmpq::fmpq_sub_si(res, f, x);
    fmpq::fmpq_neg(res, res);
}

#[inline]
pub unsafe fn fmpq_ui_mul(res: *mut flint::fmpq, f: c_ulong, g: *const flint::fmpq) {
    fmpq::fmpq_mul_ui(res, g, f);
}

#[inline]
pub unsafe fn fmpq_si_mul(res: *mut flint::fmpq, f: c_long, g: *const flint::fmpq) {
    fmpq::fmpq_mul_si(res, g, f);
}

#[inline]
pub unsafe fn fmpq_div_ui(res: *mut flint::fmpq, f: *const flint::fmpq, g: c_ulong) {
    fmpq::fmpq_set_ui(res, g, 1);
    fmpq::fmpq_div(res, f, res);
}

#[inline]
pub unsafe fn fmpq_div_si(res: *mut flint::fmpq, f: *const flint::fmpq, g: c_long) {
    fmpq::fmpq_set_si(res, g, 1);
    fmpq::fmpq_div(res, f, res);
}

#[inline]
pub unsafe fn fmpq_ui_div(res: *mut flint::fmpq, f: c_ulong, g: *const flint::fmpq) {
    fmpq::fmpq_set_ui(res, f, 1);
    fmpq::fmpq_div(res, res, g);
}

#[inline]
pub unsafe fn fmpq_si_div(res: *mut flint::fmpq, f: c_long, g: *const flint::fmpq) {
    fmpq::fmpq_set_si(res, f, 1);
    fmpq::fmpq_div(res, res, g);
}

#[inline]
pub unsafe fn fmpq_pow_ui(res: *mut flint::fmpq, f: *const flint::fmpq, g: c_ulong) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpq::fmpq_pow_fmpz(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpq_mod_ui(res: *mut flint::fmpz, f: *const flint::fmpq, g: c_ulong) {
    fmpz::fmpz_set_ui(res, g);
    fmpq::fmpq_mod_fmpz(res, f, res);
}

#[inline]
pub unsafe fn fmpq_mod_si(res: *mut flint::fmpz, f: *const flint::fmpq, g: c_long) {
    fmpz::fmpz_set_si(res, g);
    fmpq::fmpq_mod_fmpz(res, f, res);
}

#[inline]
pub unsafe fn fmpq_equal_fmpz(f: *const flint::fmpq, g: *const flint::fmpz) -> c_int {
    if fmpq::fmpq_cmp_fmpz(f, g) == 0 {
        1
    } else {
        0
    }
}

