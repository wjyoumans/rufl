/*
 *  Copyright (C) 2021 William Youmans
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

use crate::integer::Integer;
use crate::error::{Error, Result};
use crate::util::is_digit;
use flint_sys::fmpz;
use std::ffi::CString;
use std::str::FromStr;


// FIXME: panics from negative sign...
// FIXME: Valgrind sometimes complains about possibly lost bytes.
// Probably false negative, how can we be sure?
impl FromStr for Integer {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if !s.chars().all(is_digit) {
            return Err(Error::Msg("Input is not an integer.".to_string()));
        }

        if let Ok(c_str) = CString::new(s) {
            let mut z = Integer::default();
            unsafe {
                let res = fmpz::fmpz_set_str(
                    z.as_mut_ptr(), 
                    c_str.as_ptr(), 
                    10
                );
                if res == 0 {
                    Ok(z)
                } else {
                    Err(Error::Msg("Error in conversion.".to_string()))
                }
            }
        } else {
            Err(Error::Msg("String contains 0 byte.".to_string()))
        }
    }
}

///////////////////////////////////////////////////////////////////
// From
///////////////////////////////////////////////////////////////////

impl_from_unsafe! {
    None
    Integer, u64 {usize u64 u32 u16 u8}
    fmpz::fmpz_set_ui
}

impl_from_unsafe! {
    None
    Integer, i64 {isize i64 i32 i16 i8}
    fmpz::fmpz_set_si
}

/*
impl_from_unsafe! {
    None
    Integer, IntMod
    fmpz::fmpz_set
}

///////////////////////////////////////////////////////////////////
// TryFrom
///////////////////////////////////////////////////////////////////

impl TryFrom<Rational> for Integer {
    type Error = Error;
    fn try_from(src: Rational) -> Result<Self> {
        if src.denominator().is_one() {
            Ok(src.numerator())
        } else {
            Err(ConversionError {
                val: src.to_string(),
                in_type: "Rational".to_string(),
                out_type: "Integer".to_string(),
            })
        }
    }
}
*/
