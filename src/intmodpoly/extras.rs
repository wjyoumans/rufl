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
use flint_sys::fmpz_mod_poly::*;
use flint_sys::fmpz_mod_types::*;
use libc::{c_long, c_ulong};
use std::mem::MaybeUninit;


#[inline]
pub unsafe fn fmpz_mod_poly_fmpz_add(
    res: *mut fmpz_mod_poly_struct,
    f: *const fmpz,
    g: *const fmpz_mod_poly_struct,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_add_fmpz(res, g, f, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_poly_add_ui(
    res: *mut fmpz_mod_poly_struct,
    f: *const fmpz_mod_poly_struct,
    x: c_ulong,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_set_ui(res, x, ctx);
    fmpz_mod_poly_add(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_poly_ui_add(
    res: *mut fmpz_mod_poly_struct,
    x: c_ulong,
    g: *const fmpz_mod_poly_struct,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_add_ui(res, g, x, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_poly_si_add(
    res: *mut fmpz_mod_poly_struct,
    x: c_long,
    g: *const fmpz_mod_poly_struct,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_add_si(res, g, x, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_poly_sub_ui(
    res: *mut fmpz_mod_poly_struct,
    f: *const fmpz_mod_poly_struct,
    x: c_ulong,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_set_ui(res, x, ctx);
    fmpz_mod_poly_sub(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_poly_ui_sub(
    res: *mut fmpz_mod_poly_struct,
    x: c_ulong,
    g: *const fmpz_mod_poly_struct,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_set_ui(res, x, ctx);
    fmpz_mod_poly_sub(res, res, g, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_poly_fmpz_scalar_mul(
    res: *mut fmpz_mod_poly_struct,
    f: *const fmpz,
    g: *const fmpz_mod_poly_struct,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_scalar_mul_fmpz(res, g, f, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_poly_ui_scalar_mul(
    res: *mut fmpz_mod_poly_struct,
    x: c_ulong,
    g: *const fmpz_mod_poly_struct,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_scalar_mul_ui(res, g, x, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_poly_scalar_mul_si(
    res: *mut fmpz_mod_poly_struct,
    f: *const fmpz_mod_poly_struct,
    x: c_long,
    ctx: *const fmpz_mod_ctx_struct,
) {
    let mut z = MaybeUninit::uninit();
    fmpz_init_set_si(z.as_mut_ptr(), x);
    fmpz_mod_poly_scalar_mul_fmpz(res, f, z.as_ptr(), ctx);
    fmpz_clear(z.as_mut_ptr());
}

#[inline]
pub unsafe fn fmpz_mod_poly_si_scalar_mul(
    res: *mut fmpz_mod_poly_struct,
    x: c_long,
    g: *const fmpz_mod_poly_struct,
    ctx: *const fmpz_mod_ctx_struct,
) {
    fmpz_mod_poly_scalar_mul_si(res, g, x, ctx);
}
