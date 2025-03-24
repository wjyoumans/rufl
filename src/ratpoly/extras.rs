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
    fmpq,
    fmpq_poly
};
use flint_sys::fmpq_types::fmpq_poly_struct;
use flint_sys::fmpz_types::fmpz_poly_struct;
use libc::{c_int, c_long, c_ulong};
use std::mem::MaybeUninit;


#[inline]
pub unsafe fn fmpq_poly_equal_fmpq(
    f: *const fmpq_poly_struct,
    x: *const flint::fmpq,
) -> c_int {
    if fmpq_poly::fmpq_poly_length(f) == 1 {
        let mut z = MaybeUninit::uninit();
        fmpq::fmpq_init(z.as_mut_ptr());
        fmpq_poly::fmpq_poly_get_coeff_fmpq(z.as_mut_ptr(), f, 0);
        let b = fmpq::fmpq_equal(z.as_ptr(), x);
        fmpq::fmpq_clear(z.as_mut_ptr());
        b
    } else {
        0
    }
}

#[inline]
pub unsafe fn fmpq_poly_equal_fmpz(
    f: *const fmpq_poly_struct,
    x: *const flint::fmpz,
) -> c_int {
    if fmpq_poly::fmpq_poly_length(f) == 1 {
        let mut z = MaybeUninit::uninit();
        fmpq::fmpq_init(z.as_mut_ptr());
        fmpq_poly::fmpq_poly_get_coeff_fmpq(z.as_mut_ptr(), f, 0);
        let b = fmpq::fmpq_cmp_fmpz(z.as_ptr(), x);
        fmpq::fmpq_clear(z.as_mut_ptr());
        
        if b == 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[inline]
pub unsafe fn fmpq_poly_equal_ui(
    f: *const fmpq_poly_struct,
    x: c_ulong,
) -> c_int {
    if fmpq_poly::fmpq_poly_length(f) == 1 {
        let mut z = MaybeUninit::uninit();
        fmpq::fmpq_init(z.as_mut_ptr());
        fmpq_poly::fmpq_poly_get_coeff_fmpq(z.as_mut_ptr(), f, 0);
        let b = fmpq::fmpq_cmp_ui(z.as_ptr(), x);
        fmpq::fmpq_clear(z.as_mut_ptr());
        
        if b == 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[inline]
pub unsafe fn fmpq_poly_equal_si(
    f: *const fmpq_poly_struct,
    x: c_long,
) -> c_int {
    if fmpq_poly::fmpq_poly_length(f) == 1 {
        let mut z = MaybeUninit::uninit();
        fmpq::fmpq_init(z.as_mut_ptr());
        fmpq_poly::fmpq_poly_get_coeff_fmpq(z.as_mut_ptr(), f, 0);
        let b = fmpq::fmpq_cmp_si(z.as_ptr(), x);
        fmpq::fmpq_clear(z.as_mut_ptr());
        
        if b == 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[inline]
pub unsafe fn fmpq_poly_add_ui(
    res: *mut fmpq_poly_struct,
    f: *const fmpq_poly_struct,
    x: c_ulong,
    )
{
    fmpq_poly::fmpq_poly_set_ui(res, x);
    fmpq_poly::fmpq_poly_add(res, f, res);
}

#[inline]
pub unsafe fn fmpq_poly_sub_ui(
    res: *mut fmpq_poly_struct,
    f: *const fmpq_poly_struct,
    x: c_ulong,
    )
{
    fmpq_poly::fmpq_poly_set_ui(res, x);
    fmpq_poly::fmpq_poly_sub(res, f, res);
}

#[inline]
pub unsafe fn fmpq_poly_add_fmpz_poly(
    res: *mut fmpq_poly_struct,
    f: *const fmpq_poly_struct,
    x: *const fmpz_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_add(res, f, res);
}

#[inline]
pub unsafe fn fmpq_poly_sub_fmpz_poly(
    res: *mut fmpq_poly_struct,
    f: *const fmpq_poly_struct,
    x: *const fmpz_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_sub(res, f, res);
}

#[inline]
pub unsafe fn fmpq_poly_mul_fmpz_poly(
    res: *mut fmpq_poly_struct,
    f: *const fmpq_poly_struct,
    x: *const fmpz_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_mul(res, f, res);
}

#[inline]
pub unsafe fn fmpq_poly_rem_fmpz_poly(
    res: *mut fmpq_poly_struct,
    f: *const fmpq_poly_struct,
    x: *const fmpz_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_rem(res, f, res);
}

#[inline]
pub unsafe fn fmpq_poly_fmpz_poly_add(
    res: *mut fmpq_poly_struct,
    x: *const fmpz_poly_struct,
    f: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_add(res, res, f);
}

#[inline]
pub unsafe fn fmpq_poly_fmpz_poly_sub(
    res: *mut fmpq_poly_struct,
    x: *const fmpz_poly_struct,
    f: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_sub(res, res, f);
}

#[inline]
pub unsafe fn fmpq_poly_fmpz_poly_mul(
    res: *mut fmpq_poly_struct,
    x: *const fmpz_poly_struct,
    f: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_mul(res, res, f);
}

#[inline]
pub unsafe fn fmpq_poly_fmpz_poly_rem(
    res: *mut fmpq_poly_struct,
    x: *const fmpz_poly_struct,
    f: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_fmpz_poly(res, x);
    fmpq_poly::fmpq_poly_rem(res, res, f);
}

#[inline]
pub unsafe fn fmpq_poly_ui_add(
    res: *mut fmpq_poly_struct,
    f: c_ulong,
    g: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_ui(res, f);
    fmpq_poly::fmpq_poly_add(res, res, g);
}

#[inline]
pub unsafe fn fmpq_poly_ui_sub(
    res: *mut fmpq_poly_struct,
    f: c_ulong,
    g: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_ui(res, f);
    fmpq_poly::fmpq_poly_sub(res, res, g);
}

#[inline]
pub unsafe fn fmpq_poly_ui_scalar_mul(
    res: *mut fmpq_poly_struct,
    f: c_ulong,
    g: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_scalar_mul_ui(res, g, f);
}

#[inline]
pub unsafe fn fmpq_poly_si_add(
    res: *mut fmpq_poly_struct,
    f: c_long,
    g: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_add_si(res, g, f);
}

#[inline]
pub unsafe fn fmpq_poly_si_sub(
    res: *mut fmpq_poly_struct,
    f: c_long,
    g: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_set_si(res, f);
    fmpq_poly::fmpq_poly_sub(res, res, g);
}

pub unsafe fn fmpq_poly_si_scalar_mul(
    res: *mut fmpq_poly_struct,
    f: c_long,
    g: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_scalar_mul_si(res, g, f);
}

#[inline]
pub unsafe fn fmpq_poly_fmpz_add(
    res: *mut fmpq_poly_struct,
    f: *const flint::fmpz,
    g: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_add_fmpz(res, g, f);
}

#[inline]
pub unsafe fn fmpq_poly_fmpz_scalar_mul(
    res: *mut fmpq_poly_struct,
    f: *const flint::fmpz,
    g: *const fmpq_poly_struct,
    )
{
    fmpq_poly::fmpq_poly_scalar_mul_fmpz(res, g, f);
}



