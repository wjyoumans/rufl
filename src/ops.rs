pub use std::ops::*;

/*
pub trait NegOps:
    //Neg<Output=Self>
    NegAssign
{}

impl<T> NegOps for T
where
    T: //Neg<Output=T>
    NegAssign
{}

pub trait InvOps:
    //Inv<Output=Self>
    InvAssign
{}

impl<T> InvOps for T
where
    T: //Inv<Output=T>
    InvAssign
{}

pub trait AddOps: Sized
    //+ Add<Output=Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> AddFrom<&'a Self>
{}

impl<T> AddOps for T
where
    T: Sized 
    //+ Add<Output=T>
    + for<'a> AddAssign<&'a T>
    + for<'a> AddFrom<&'a T>
{}

pub trait SubOps: Sized
    //+ Sub<Output=Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> SubFrom<&'a Self>
{}

impl<T> SubOps for T
where
    T: Sized 
    //+ Sub<Output=T>
    + for<'a> SubAssign<&'a T>
    + for<'a> SubFrom<&'a T>
{}

pub trait MulOps: Sized
    //+ Mul<Output=Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> MulFrom<&'a Self>
{}

impl<T> MulOps for T
where
    T: Sized 
    //+ Mul<Output=T>
    + for<'a> MulAssign<&'a T>
    + for<'a> MulFrom<&'a T>
{}

pub trait DivOps: Sized
    //+ Div<Output=Self>
    + for<'a> DivAssign<&'a Self>
    + for<'a> DivFrom<&'a Self>
{}

impl<T> DivOps for T
where
    T: Sized 
    //+ Div<Output=T>
    + for<'a> DivAssign<&'a T>
    + for<'a> DivFrom<&'a T>
{}


// Constructors

pub trait New<T> {
    fn new(src: T) -> Self;
}

pub trait NewCtx<T, Ctx> {
    fn new(src: T, ctx: &Ctx) -> Self;
}

pub trait NewMatrix<T> {
    fn new(src: T, nrows: i64, ncols: i64) -> Self;
}

pub trait NewElement<T>: Parent {
    fn new(&self, src: T) -> Self::Element;
}
*/

/// Assign to self. Not meant for expensive conversion.
pub trait Assign<T = Self> {
    fn assign(&mut self, other: T);
}

pub trait NewCtx<T, Ctx> {
    fn new(src: T, ctx: &Ctx) -> Self;
}

pub trait NewMatrix<T> {
    fn new(src: T, nrows: i64, ncols: i64) -> Self;
}

///////////////////////////////////////////////////////////////////
// Unary Ops
///////////////////////////////////////////////////////////////////

/// Inverse as a unary operation.
pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

/// Inverse with assignment.
pub trait InvAssign {
    fn inv_assign(&mut self);
}

/// Inverse with assignment into a separate argument.
pub trait AssignInv<Arg = Self> {
    fn assign_inv(&mut self, arg: Arg);
}

/// Negation with assignment.
pub trait NegAssign {
    fn neg_assign(&mut self);
}

/// Negation with assignment into a separate argument.
pub trait AssignNeg<Arg = Self> {
    fn assign_neg(&mut self, arg: Arg);
}

/// Complement with assignment.
pub trait NotAssign {
    fn not_assign(&mut self);
}

///////////////////////////////////////////////////////////////////
// Binary Ops
///////////////////////////////////////////////////////////////////

/// Complement with assignment into a separate argument.
pub trait AssignNot<Arg = Self> {
    fn assign_not(&mut self, arg: Arg);
}

/// Bitwise `and` with assignment to the rhs operand.
pub trait BitAndFrom<Lhs = Self> {
    fn bitand_from(&mut self, lhs: Lhs);
}

/// Bitwise `and` with assignment into a separate argument.
pub trait AssignBitAnd<Lhs = Self, Rhs = Self> {
    fn assign_bitand(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Bitwise `or` with assignment to the rhs operand.
pub trait BitOrFrom<Lhs = Self> {
    fn bitor_from(&mut self, lhs: Lhs);
}

/// Bitwise `or` with assignment into a separate argument.
pub trait AssignBitOr<Lhs = Self, Rhs = Self> {
    fn assign_bitor(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Bitwise `xor` with assignment to the rhs operand.
pub trait BitXorFrom<Lhs = Self> {
    fn bitxor_from(&mut self, lhs: Lhs);
}

/// Bitwise `xor` with assignment into a separate argument.
pub trait AssignBitXor<Lhs = Self, Rhs = Self> {
    fn assign_bitxor(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Addition with assignment to the rhs operand.
pub trait AddFrom<Lhs = Self> {
    fn add_from(&mut self, lhs: Lhs);
}

/// Addition with assignment into a separate argument.
pub trait AssignAdd<Lhs = Self, Rhs = Self> {
    fn assign_add(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Subtraction with assignment to the rhs operand.
pub trait SubFrom<Lhs = Self> {
    fn sub_from(&mut self, lhs: Lhs);
}

/// Subtraction with assignment into a separate argument.
pub trait AssignSub<Lhs = Self, Rhs = Self> {
    fn assign_sub(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Multiplication with assignment to the rhs operand.
pub trait MulFrom<Lhs = Self> {
    fn mul_from(&mut self, lhs: Lhs);
}

/// Multiplication with assignment into a separate argument.
pub trait AssignMul<Lhs = Self, Rhs = Self> {
    fn assign_mul(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Division with assignment to the rhs operand.
pub trait DivFrom<Lhs = Self> {
    fn div_from(&mut self, lhs: Lhs);
}

/// Division with assignment into a separate argument.
pub trait AssignDiv<Lhs = Self, Rhs = Self> {
    fn assign_div(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Remainder with assignment to the rhs operand.
pub trait RemFrom<Lhs = Self> {
    fn rem_from(&mut self, lhs: Lhs);
}

/// Remainder with assignment into a separate argument.
pub trait AssignRem<Lhs = Self, Rhs = Self> {
    fn assign_rem(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Exponentiation.
pub trait Pow<Rhs = Self> {
    type Output;
    fn pow(self, rhs: Rhs) -> Self::Output;
}

/// Exponentiation with assignment.
pub trait PowAssign<Rhs = Self> {
    fn pow_assign(&mut self, rhs: Rhs);
}

/// Exponentiation with assignment to the rhs operand.
pub trait PowFrom<Lhs = Self> {
    fn pow_from(&mut self, lhs: Lhs);
}

/// Exponentiation with assignment into a separate argument.
pub trait AssignPow<Lhs = Self, Rhs = Self> {
    fn assign_pow(&mut self, lhs: Lhs, rhs: Rhs);
}

/// Evaluation of an expression.
pub trait Evaluate<X> {
    type Output;
    fn evaluate(&self, x: X) -> Self::Output;
}

/// Evaluation of an expression with assignment into a separate argument.
pub trait AssignEvaluate<Expr, X> {
    fn assign_evaluate(&mut self, expr: Expr, x: X);
}

/// Modular evaluation of an expression.
pub trait EvaluateMod<X, M> {
    type Output;
    fn evaluate_mod(&self, x: X, modulus: M) -> Self::Output;
}

/// Modular evaluation of an expression with assignment into a separate argument.
pub trait AssignEvaluateMod<Expr, X, M> {
    fn assign_evaluate_mod(&mut self, expr: Expr, x: X, modulus: M);
}


/* Alternative to implementing ops for OwnedScalar/BorrowedScalar wrappers
///////////////////////////////////////////////////////////////////
// Scalar ops
///////////////////////////////////////////////////////////////////

/// Addition with a scalar.
pub trait AddScalar<Rhs> {
    type Output;
    fn add_scalar(self, rhs: Rhs) -> Self::Output;
}

pub trait ScalarAdd<Rhs> {
    type Output;
    fn add_scalar(self, rhs: Rhs) -> Self::Output;
}

/// Addition with a scalar with assignment.
pub trait AddAssignScalar<Rhs> {
    fn add_assign_scalar(&mut self, rhs: Rhs);
}

/// Subtraction by a scalar.
pub trait SubScalar<Rhs> {
    type Output;
    fn sub_scalar(self, rhs: Rhs) -> Self::Output;
}

/// Multiplication by a scalar.
pub trait MulScalar<Rhs> {
    type Output;
    fn mul_scalar(self, rhs: Rhs) -> Self::Output;
}

/// Division by a scalar.
pub trait DivScalar<Rhs> {
    type Output;
    fn div_scalar(self, rhs: Rhs) -> Self::Output;
}

/// Modular reduction by a scalar.
pub trait RemScalar<Rhs> {
    type Output;
    fn rem_scalar(self, rhs: Rhs) -> Self::Output;
}
*/

