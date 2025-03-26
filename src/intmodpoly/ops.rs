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

use crate::{IntMod, IntModPoly, Integer};

use flint_sys::fmpz_mod_poly;
use crate::ops::*;
use crate::intmodpoly::extras::*;


impl_cmp! {
    eq
    IntModPoly
    {
        fn eq(&self, rhs: &IntModPoly) -> bool {
            unsafe {
                self.context() == rhs.context() && 
                    fmpz_mod_poly::fmpz_mod_poly_equal(
                        self.as_ptr(),
                        rhs.as_ptr(),
                        self.ctx_as_ptr()
                    ) != 0
            }
        }
    }
}

impl_cmp! {
    partial_eq
    IntModPoly, IntMod
    {
        fn eq(&self, rhs: &IntMod) -> bool {
            self.context() == rhs.context() && 
                self.degree() == 0 && &self.get_coeff(0) == rhs
        }
    }
}

impl_unop_unsafe! {
    ctx
    IntModPoly
    Neg {neg}
    NegAssign {neg_assign}
    fmpz_mod_poly::fmpz_mod_poly_neg
}

impl_binop_unsafe! {
    ctx
    IntModPoly, IntModPoly, IntModPoly

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_poly::fmpz_mod_poly_add;

    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod_poly::fmpz_mod_poly_sub;

    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_poly::fmpz_mod_poly_mul;
}

impl_binop_unsafe! {
    ctx
    op_assign
    IntModPoly, Integer, IntModPoly

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod_poly::fmpz_mod_poly_add_fmpz;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod_poly::fmpz_mod_poly_sub_fmpz;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod_poly::fmpz_mod_poly_scalar_mul_fmpz;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntModPoly, u64 {u64 u32 u16 u8}, IntModPoly

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod_poly_add_ui;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod_poly_sub_ui;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod_poly::fmpz_mod_poly_scalar_mul_ui;
}

impl_binop_unsafe! {
    ctx_lhs
    op_assign
    IntModPoly, i64 {i64 i32 i16 i8}, IntModPoly

    Add {add}
    AddAssign {add_assign}
    AssignAdd {assign_add}
    fmpz_mod_poly::fmpz_mod_poly_add_si;

    Sub {sub}
    SubAssign {sub_assign}
    AssignSub {assign_sub}
    fmpz_mod_poly::fmpz_mod_poly_sub_si;

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    fmpz_mod_poly_scalar_mul_si;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    Integer, IntModPoly, IntModPoly

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_poly_fmpz_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod_poly::fmpz_mod_poly_fmpz_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_poly_fmpz_scalar_mul;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    u64 {u64 u32 u16 u8}, IntModPoly, IntModPoly

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_poly_ui_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod_poly_ui_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_poly_ui_scalar_mul;
}

impl_binop_unsafe! {
    ctx_rhs
    op_from
    i64 {i64 i32 i16 i8}, IntModPoly, IntModPoly

    Add {add}
    AddFrom {add_from}
    AssignAdd {assign_add}
    fmpz_mod_poly_si_add;

    Sub {sub}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    fmpz_mod_poly::fmpz_mod_poly_si_sub;

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mod_poly_si_scalar_mul;
}