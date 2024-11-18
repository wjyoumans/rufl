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


// Use traits to generate method tables?
// if user defines a struct and implements traits for it (like init, clear, add, sub, etc.)
// then generate a method table and init a gr_method_tab.
//
// Then we need to pass rust function pointers back to flint... see libffi crate



//mod ops;
//mod conv;

use crate::*;
use flint_sys::gr;
use std::ffi::{CStr, CString};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct GrCtx(gr::gr_ctx_struct);

impl Drop for GrCtx {
    fn drop(&mut self) {
        unsafe {
            gr::gr_ctx_clear(&mut self.0);
        }
    }
}

impl GrCtx {
    #[inline]
    pub fn integer_ring() -> Self {
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            gr::gr_ctx_init_fmpz(ctx.as_mut_ptr()); 
            GrCtx(ctx.assume_init())
        }
    }    
    
    #[inline]
    pub fn rational_field() -> Self {
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            gr::gr_ctx_init_fmpz(ctx.as_mut_ptr()); 
            GrCtx(ctx.assume_init())
        }
    }    
}



#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum GrType {
    IntegerRing,
    RationalField,
    Other,
}

impl fmt::Display for GrType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match &self {
            GrType::IntegerRing => "integer ring",
            _ => "something else?"
        };
        write!(f, "{}", out)
    }
}

#[derive(Debug, Clone)]
pub struct GenericCtx {
    gr_type: GrType,
    inner: Rc<GrCtx>,
}

impl Eq for GenericCtx {}

// TODO: use GenericType for fine grained check
impl PartialEq for GenericCtx {
    fn eq(&self, rhs: &GenericCtx) -> bool {
        Rc::ptr_eq(&self.inner, &rhs.inner) 
    }
}

impl fmt::Display for GenericCtx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Generic context for {}",
            self.gr_type()
        )
    }
}

/*
impl Hash for GenericCtx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.modulus().hash(state)
    }
}*/

impl GenericCtx {
    /// Returns a pointer to the inner [gr::gr_ctx_struct].
    #[inline]
    pub fn as_ptr(&self) -> &gr::gr_ctx_struct {
        &self.inner.0
    }

    pub fn gr_type(&self) -> GrType {
        self.gr_type
    }
}

impl GenericCtx {
    #[inline]
    pub fn integer_ring() -> Self {
        GenericCtx {
            gr_type: GrType::IntegerRing,
            inner: Rc::new(GrCtx::integer_ring())
        }
    }
}

#[derive(Debug)]
pub struct GenericElem {
    inner: gr::gr_ptr,
    ctx: GenericCtx,
}

impl Drop for GenericElem {
    fn drop(&mut self) {
        unsafe {gr::gr_clear(self.as_mut_ptr(), self.ctx.as_ptr())}
    }
}

impl fmt::Display for GenericElem {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut s = MaybeUninit::uninit();
            let success = gr::gr_get_str(s.as_mut_ptr(), self.as_ptr(), self.ctx_as_ptr());
            if success != 0 {
                panic!("Error in gr::gr_get_str")
            }

            match CStr::from_ptr(s.assume_init()).to_str() {
                Ok(s) => write!(f, "{}", s),
                Err(_) => panic!("Flint returned invalid UTF-8!"),
            }
        }

    }
}

impl GenericElem {
    /// Returns a pointer to the inner [gr::gr_ptr].
    #[inline]
    pub const fn as_ptr(&self) -> gr::gr_srcptr {
        self.inner
    }

    /// Returns a mutable pointer to the inner [gr::gr_ptr].
    #[inline]
    pub fn as_mut_ptr(&mut self) -> gr::gr_ptr {
        self.inner
    }

    /// Returns a pointer to the [FLINT context][gr::gr_ctx_struct].
    #[inline]
    pub fn ctx_as_ptr(&self) -> &gr::gr_ctx_struct {
        self.context().as_ptr()
    }
    
    #[inline]
    pub const unsafe fn from_raw(inner: gr::gr_ptr, ctx: GenericCtx) -> GenericElem {
        GenericElem { inner, ctx }
    }
  
    #[inline]
    pub const fn into_raw(self) -> gr::gr_ptr {
        let inner = self.inner;
        let _ = ManuallyDrop::new(self);
        inner
    }
    
    #[inline]
    pub const fn context(&self) -> &GenericCtx {
        &self.ctx
    }

}


// GenericElem<T: Ring>
// GenericRing<T: Ring>
// GenericGroup<T: Group>
// impl GenericElem<IntegerRing> {
//   
//}
