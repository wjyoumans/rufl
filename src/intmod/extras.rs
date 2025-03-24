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

use flint_sys::{flint, fmpz, fmpq, fmpz_mod, fmpz_mod_types};
use libc::{c_int, c_long, c_ulong};

#[inline]
pub unsafe fn fmpz_mod_ui_add(
    res: *mut flint::fmpz,
    f: c_ulong,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_add_ui(res, g, f, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_si_add(
    res: *mut flint::fmpz,
    f: c_long,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_add_si(res, g, f, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_ui_mul(
    res: *mut flint::fmpz,
    f: c_ulong,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_mul_ui(res, g, f, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_si_mul(
    res: *mut flint::fmpz,
    f: c_long,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_mul_si(res, g, f, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_div(
    res: *mut flint::fmpz,
    f: *const flint::fmpz,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, g);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_add_fmpq(
    res: *mut flint::fmpz,
    f: *const flint::fmpz,
    g: *const flint::fmpq,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*g).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*g).num, ctx);
    fmpz_mod::fmpz_mod_add(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_sub_fmpq(
    res: *mut flint::fmpz,
    f: *const flint::fmpz,
    g: *const flint::fmpq,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*g).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*g).num, ctx);
    fmpz_mod::fmpz_mod_sub(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_mul_fmpq(
    res: *mut flint::fmpz,
    f: *const flint::fmpz,
    g: *const flint::fmpq,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*g).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*g).num, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_div_fmpq(
    res: *mut flint::fmpz,
    f: *const flint::fmpz,
    g: *const flint::fmpq,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*g).num);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*g).den, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_fmpq_add(
    res: *mut flint::fmpz,
    f: *const flint::fmpq,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*f).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*f).num, ctx);
    fmpz_mod::fmpz_mod_add(res, res, g, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_fmpq_sub(
    res: *mut flint::fmpz,
    f: *const flint::fmpq,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*f).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*f).num, ctx);
    fmpz_mod::fmpz_mod_sub(res, res, g, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_fmpq_mul(
    res: *mut flint::fmpz,
    f: *const flint::fmpq,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*f).den);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*f).num, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, g, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_fmpq_div(
    res: *mut flint::fmpz,
    f: *const flint::fmpq,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set(res, &(*f).num);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, &(*f).den, ctx);
    fmpz_mod::fmpz_mod_mul(res, res, g, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_div_ui(
    res: *mut flint::fmpz,
    f: *const flint::fmpz,
    g: c_ulong,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set_ui(res, g);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_ui_div(
    res: *mut flint::fmpz,
    f: c_ulong,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_inv(res, g, ctx);
    fmpz_mod_ui_mul(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_div_si(
    res: *mut flint::fmpz,
    f: *const flint::fmpz,
    g: c_long,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz::fmpz_set_si(res, g);
    fmpz_mod::fmpz_mod_inv(res, res, ctx);
    fmpz_mod::fmpz_mod_mul(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_si_div(
    res: *mut flint::fmpz,
    f: c_long,
    g: *const flint::fmpz,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    fmpz_mod::fmpz_mod_inv(res, g, ctx);
    fmpz_mod_si_mul(res, f, res, ctx);
}

#[inline]
pub unsafe fn fmpz_mod_pow_si(
    res: *mut flint::fmpz,
    f: *const flint::fmpz,
    g: c_long,
    ctx: *const fmpz_mod_types::fmpz_mod_ctx,
) {
    if g < 0 {
        fmpz_mod::fmpz_mod_inv(res, f, ctx);
    } else {
        fmpz::fmpz_set(res, f);
    }

    fmpz_mod::fmpz_mod_pow_ui(res, res, g.abs() as u64, ctx);
}

#[inline]
pub unsafe fn fmpz_equal_fmpq(f: *const flint::fmpz, g: *const flint::fmpq) -> c_int {
    if fmpq::fmpq_cmp_fmpz(g, f) == 0 {
        1
    } else {
        0
    }
}

