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

use crate::{Integer, Rational, IntMod, IntPoly, RatPoly};
use crate::ops::*;
use crate::intpoly::extras::*;

use flint_sys::fmpz_poly;

impl_assign_unsafe! {
    None
    IntPoly, IntPoly
    fmpz_poly::fmpz_poly_set
}

impl_assign_unsafe! {
    None
    IntPoly, IntMod
    fmpz_poly::fmpz_poly_set_fmpz
}

impl_assign_unsafe! {
    None
    IntPoly, Integer
    fmpz_poly::fmpz_poly_set_fmpz
}

impl_assign_unsafe! {
    None
    IntPoly, u64 {u64 u32 u16 u8}
    fmpz_poly::fmpz_poly_set_ui
}

impl_assign_unsafe! {
    None
    IntPoly, i64 {i64 i32 i16 i8}
    fmpz_poly::fmpz_poly_set_si
}

impl_cmp_unsafe! {
    eq
    IntPoly
    fmpz_poly::fmpz_poly_equal
}

impl_cmp_unsafe! {
    partial_eq
    IntPoly, u64 {u64 u32 u16 u8}
    fmpz_poly_equal_ui
}

impl_cmp_unsafe! {
    partial_eq
    IntPoly, i64 {i64 i32 i16 i8}
    fmpz_poly_equal_si
}

impl_cmp_unsafe! {
    partial_eq
    IntPoly, Integer
    fmpz_poly::fmpz_poly_equal_fmpz
}

impl_cmp_unsafe! {
    partial_eq
    IntPoly, Rational
    fmpz_poly_equal_fmpq
}

impl_unop_unsafe! {
    None
    IntPoly
    Neg {neg}
    NegAssign {neg_assign}
    fmpz_poly::fmpz_poly_neg
}

// Inv: requires rational function

impl_binop_unsafe! {
    None
    IntPoly, IntPoly, IntPoly

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly::fmpz_poly_add;

    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly::fmpz_poly_sub;

    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly::fmpz_poly_mul;

    Rem {rem}
    RemAssign {rem_assign}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly::fmpz_poly_rem;
}

impl_binop_unsafe! {
    None
    IntPoly, Rational, RatPoly

    Add {add}
    AssignAdd {assign_add}
    fmpz_poly_add_fmpq;

    Sub {sub}
    AssignSub {assign_sub}
    fmpz_poly_sub_fmpq;

    Mul {mul}
    AssignMul {assign_mul}
    fmpz_poly_scalar_mul_fmpq;

    Div {div}
    AssignDiv {assign_div}
    fmpz_poly_scalar_div_fmpq;
}

impl_binop_unsafe! {
    None
    Rational, IntPoly, RatPoly

    Add {add}
    AssignAdd {assign_add}
    fmpz_poly_fmpq_add;

    Sub {sub}
    AssignSub {assign_sub}
    fmpz_poly_fmpq_sub;

    Mul {mul}
    AssignMul {assign_mul}
    fmpz_poly_fmpq_scalar_mul;
}

impl_binop_unsafe! {
    None
    op_assign
    IntPoly, Integer, IntPoly

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_poly::fmpz_poly_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_poly::fmpz_poly_sub_fmpz;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_poly::fmpz_poly_scalar_mul_fmpz;

    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_poly::fmpz_poly_scalar_mod_fmpz;
}

impl_binop_unsafe! {
    None
    op_from
    Integer, IntPoly, IntPoly

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly::fmpz_poly_fmpz_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_fmpz_scalar_mul;

    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_fmpz_scalar_mod;
}

impl_binop_unsafe! {
    None
    op_assign
    IntPoly, u64 {u64 u32 u16 u8}, IntPoly

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_poly_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_poly_sub_ui;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_poly::fmpz_poly_scalar_mul_ui;

    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_poly_scalar_mod_ui;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz_poly::fmpz_poly_pow;
}

impl_binop_unsafe! {
    None
    op_assign
    IntPoly, i64 {i64 i32 i16 i8}, IntPoly

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_poly::fmpz_poly_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_poly::fmpz_poly_sub_si;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_poly::fmpz_poly_scalar_mul_si;

    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_poly_scalar_mod_si;
}

impl_binop_unsafe! {
    None
    op_from
    u64 {u64 u32 u16 u8}, IntPoly, IntPoly

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly_ui_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_ui_scalar_mul;

    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_ui_scalar_mod;
}

impl_binop_unsafe! {
    None
    op_from
    i64 {i64 i32 i16 i8}, IntPoly, IntPoly

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_poly_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_poly_si_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_poly_si_scalar_mul;

    Rem {rem}
    RemFrom {rem_from}
    AssignRem {assign_rem}
    fmpz_poly_si_scalar_mod;
}
