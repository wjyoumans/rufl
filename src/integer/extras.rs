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
use libc::{c_long, c_ulong};
use std::mem::MaybeUninit;

#[inline]
pub unsafe fn fmpz_and_ui(res: *mut flint::fmpz, f: *const flint::fmpz, x: c_ulong) {
    fmpz::fmpz_init_set_ui(res, x);
    fmpz::fmpz_and(res, f, res);
}

#[inline]
pub unsafe fn fmpz_and_si(res: *mut flint::fmpz, f: *const flint::fmpz, x: c_long) {
    fmpz::fmpz_init_set_si(res, x);
    fmpz::fmpz_and(res, f, res);
}

#[inline]
pub unsafe fn fmpz_ui_and(res: *mut flint::fmpz, x: c_ulong, f: *const flint::fmpz) {
    fmpz_and_ui(res, f, x);
}

#[inline]
pub unsafe fn fmpz_si_and(res: *mut flint::fmpz, x: c_long, f: *const flint::fmpz) {
    fmpz_and_si(res, f, x);
}

#[inline]
pub unsafe fn fmpz_or_ui(res: *mut flint::fmpz, f: *const flint::fmpz, x: c_ulong) {
    fmpz::fmpz_init_set_ui(res, x);
    fmpz::fmpz_or(res, f, res);
}

#[inline]
pub unsafe fn fmpz_or_si(res: *mut flint::fmpz, f: *const flint::fmpz, x: c_long) {
    fmpz::fmpz_init_set_si(res, x);
    fmpz::fmpz_or(res, f, res);
}

#[inline]
pub unsafe fn fmpz_ui_or(res: *mut flint::fmpz, x: c_ulong, f: *const flint::fmpz) {
    fmpz_or_ui(res, f, x);
}

#[inline]
pub unsafe fn fmpz_si_or(res: *mut flint::fmpz, x: c_long, f: *const flint::fmpz) {
    fmpz_or_si(res, f, x);
}

#[inline]
pub unsafe fn fmpz_xor_ui(res: *mut flint::fmpz, f: *const flint::fmpz, x: c_ulong) {
    fmpz::fmpz_init_set_ui(res, x);
    fmpz::fmpz_xor(res, f, res);
}

#[inline]
pub unsafe fn fmpz_xor_si(res: *mut flint::fmpz, f: *const flint::fmpz, x: c_long) {
    fmpz::fmpz_init_set_si(res, x);
    fmpz::fmpz_xor(res, f, res);
}

#[inline]
pub unsafe fn fmpz_ui_xor(res: *mut flint::fmpz, x: c_ulong, f: *const flint::fmpz) {
    fmpz_xor_ui(res, f, x);
}

#[inline]
pub unsafe fn fmpz_si_xor(res: *mut flint::fmpz, x: c_long, f: *const flint::fmpz) {
    fmpz_xor_si(res, f, x);
}

#[inline]
pub unsafe fn fmpz_ui_add(res: *mut flint::fmpz, x: c_ulong, f: *const flint::fmpz) {
    fmpz::fmpz_add_ui(res, f, x);
}

#[inline]
pub unsafe fn fmpz_si_add(res: *mut flint::fmpz, x: c_long, f: *const flint::fmpz) {
    fmpz::fmpz_add_si(res, f, x);
}

#[inline]
pub unsafe fn fmpz_ui_sub(res: *mut flint::fmpz, x: c_ulong, f: *const flint::fmpz) {
    fmpz::fmpz_sub_ui(res, f, x);
    fmpz::fmpz_neg(res, res);
}

#[inline]
pub unsafe fn fmpz_si_sub(res: *mut flint::fmpz, x: c_long, f: *const flint::fmpz) {
    fmpz::fmpz_sub_si(res, f, x);
    fmpz::fmpz_neg(res, res);
}

#[inline]
pub unsafe fn fmpz_ui_mul(res: *mut flint::fmpz, f: c_ulong, g: *const flint::fmpz) {
    fmpz::fmpz_mul_ui(res, g, f);
}

#[inline]
pub unsafe fn fmpz_si_mul(res: *mut flint::fmpz, f: c_long, g: *const flint::fmpz) {
    fmpz::fmpz_mul_si(res, g, f);
}

#[inline]
pub unsafe fn fmpz_tdiv_r(f: *mut flint::fmpz, g: *const flint::fmpz, h: *const flint::fmpz) {
    let mut q = MaybeUninit::uninit();
    fmpz::fmpz_init(q.as_mut_ptr());
    fmpz::fmpz_tdiv_qr(q.as_mut_ptr(), f, g, h);
    fmpz::fmpz_clear(q.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpz_tdiv_r_ui(f: *mut flint::fmpz, g: *const flint::fmpz, h: c_ulong) {
    let r = fmpz::fmpz_tdiv_ui(g, h);
    fmpz::fmpz_set_ui(f, r);
}

#[inline]
pub unsafe fn fmpz_tdiv_r_si(f: *mut flint::fmpz, g: *const flint::fmpz, h: c_long) {
    let r = fmpz::fmpz_tdiv_ui(g, h as u64);
    fmpz::fmpz_set_ui(f, r);
}

#[inline]
pub unsafe fn fmpz_ui_tdiv_r(res: *mut flint::fmpz, x: c_ulong, h: *const flint::fmpz) {
    fmpz::fmpz_init_set_ui(res, x);
    fmpz_tdiv_r(res, res, h);
}

#[inline]
pub unsafe fn fmpz_si_tdiv_r(res: *mut flint::fmpz, x: c_long, h: *const flint::fmpz) {
    fmpz::fmpz_init_set_si(res, x);
    fmpz_tdiv_r(res, res, h);
}

#[inline]
pub unsafe fn fmpq_inv_fmpz(res: *mut flint::fmpq, f: *const flint::fmpz) {
    fmpq::fmpq_set_fmpz(res, f);
    fmpq::fmpq_inv(res, res);
}

#[inline]
pub unsafe fn fmpz_pow_fmpz(res: *mut flint::fmpq, f: *const flint::fmpz, g: *const flint::fmpz) {
    fmpq::fmpq_set_fmpz(res, f);
    fmpq::fmpq_pow_fmpz(res, res, g);
}

#[inline]
pub unsafe fn fmpz_pow_si(res: *mut flint::fmpq, f: *const flint::fmpz, g: c_long) {
    fmpq::fmpq_set_fmpz(res, f);
    fmpq::fmpq_pow_si(res, res, g);
}

#[inline]
pub unsafe fn fmpz_ui_pow(res: *mut flint::fmpq, f: c_ulong, g: *const flint::fmpz) {
    fmpq::fmpq_set_ui(res, f, 1);
    fmpq::fmpq_pow_fmpz(res, res, g);
}

#[inline]
pub unsafe fn fmpz_si_pow(res: *mut flint::fmpq, f: c_long, g: *const flint::fmpz) {
    fmpq::fmpq_set_si(res, f, 1);
    fmpq::fmpq_pow_fmpz(res, res, g);
}

#[inline]
pub unsafe fn fmpz_div_ui(res: *mut flint::fmpq, f: *const flint::fmpz, g: c_ulong) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    fmpq::fmpq_set_fmpz_frac(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpz_div_si(res: *mut flint::fmpq, f: *const flint::fmpz, g: c_long) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), g);
    fmpq::fmpq_set_fmpz_frac(res, f, z.as_ptr());
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpz_ui_div(res: *mut flint::fmpq, f: c_ulong, g: *const flint::fmpz) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_ui(z.as_mut_ptr(), f);
    fmpq::fmpq_set_fmpz_frac(res, z.as_ptr(), g);
    fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpz_si_div(res: *mut flint::fmpq, f: c_long, g: *const flint::fmpz) {
    let mut z = MaybeUninit::uninit();
    fmpz::fmpz_init_set_si(z.as_mut_ptr(), f);
    fmpq::fmpq_set_fmpz_frac(res, z.as_ptr(), g);
    fmpz::fmpz_clear(z.as_mut_ptr());
}
