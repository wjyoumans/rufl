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

use crate::{IntMod, Integer, Rational};
use crate::ops::*;
use crate::intmod::extras::*;

use flint_sys::{fmpz, fmpz_mod};

impl_assign_unsafe! {
    ctx
    IntMod, IntMod
    fmpz_mod::fmpz_mod_set_fmpz
}

impl_assign_unsafe! {
    ctx
    IntMod, Integer
    fmpz_mod::fmpz_mod_set_fmpz
}

impl_assign_unsafe! {
    ctx
    IntMod, u64 {u64 u32 u16 u8}
    fmpz_mod::fmpz_mod_set_ui
}

impl_assign_unsafe! {
    ctx
    IntMod, i64 {i64 i32 i16 i8}
    fmpz_mod::fmpz_mod_set_si
}

impl_assign! {
    IntMod, Rational
    {
        fn assign(&mut self, src: &Rational) {
            if let Some(den_inv) = src.denominator().invmod(self.modulus()) {
                let temp = src.numerator() * den_inv;
                unsafe {
                    fmpz_mod::fmpz_mod_set_fmpz(
                        self.as_mut_ptr(), 
                        temp.as_ptr(),
                        self.ctx_as_ptr()
                    );
                }
            } else {
                panic!("Denominator not invertible!");
            }
        }
    }
}

// TODO: probably dont use these Eq impls? Needs some thought

impl_cmp_unsafe! {
    eq
    IntMod
    fmpz::fmpz_equal
}

impl_cmp_unsafe! {
    partial_eq
    IntMod, Integer
    fmpz::fmpz_equal
}

impl_cmp_unsafe! {
    partial_eq
    IntMod, Rational
    fmpz_equal_fmpq
}

impl_cmp_unsafe! {
    partial_eq
    IntMod, u64 {u64 u32 u16 u8}
    fmpz::fmpz_equal_ui
}

impl_cmp_unsafe! {
    partial_eq
    IntMod, i64 {i64 i32 i16 i8}
    fmpz::fmpz_equal_si
}

impl_unop_unsafe! {
    ctx
    IntMod
    Neg {neg}
    NegAssign {neg_assign}
    fmpz_mod::fmpz_mod_neg
}

impl_unop_unsafe! {
    ctx
    IntMod
    Inv {inv}
    InvAssign {inv_assign}
    fmpz_mod::fmpz_mod_inv
}

impl_binop_unsafe! {
    ctx
    IntMod, IntMod, IntMod

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add;

    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub;

    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul;

    Div {div}
    DivAssign {div_assign}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_div;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, u64 {u64 u32 u16 u8}, IntMod

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_ui;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_ui;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mod_div_ui;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz_mod::fmpz_mod_pow_ui;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, i64 {i64 i32 i16 i8}, IntMod

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_si;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_si;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mod_div_si;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz_mod_pow_si;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, Integer, IntMod

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_fmpz;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_fmpz;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mod_div;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    fmpz_mod::fmpz_mod_pow_fmpz;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntMod, Rational, IntMod

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod_add_fmpq;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod_sub_fmpq;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod_mul_fmpq;

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mod_div_fmpq;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    u64 {u64 u32 u16 u8}, IntMod, IntMod

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_ui_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_ui_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_ui_div;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    i64 {i64 i32 i16 i8}, IntMod, IntMod

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_si_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_si_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_si_div;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    Integer, IntMod, IntMod

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod::fmpz_mod_add_fmpz;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod::fmpz_mod_sub_fmpz;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod::fmpz_mod_mul_fmpz;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_div;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    Rational, IntMod, IntMod

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_fmpq_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod_fmpq_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_fmpq_mul;

    Div {div}
    DivFrom {div_from}
    AssignDiv {assign_div}
    fmpz_mod_fmpq_div;
}
