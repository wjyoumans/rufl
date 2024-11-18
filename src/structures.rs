
pub trait Finite {
    fn is_finite(&self) -> bool;    
}

pub trait MultiplicativeGroup {
    fn is_multiplicative_group(&self) -> bool;
}

pub trait Ring {
    fn is_ring(&self) -> bool;
}

pub trait CommutativeRing {
    fn is_commutative_ring(&self) -> bool;
}

pub trait IntegralDomain {
    fn is_integral_domain(&self) -> bool;
}

pub trait UniqueFactorizationDomain {
    fn is_unique_factorization_domain(&self) -> bool;
    fn is_ufd(&self) -> bool {
        self.is_unique_factorization_domain()
    }
}

pub trait Field {
    fn is_field(&self) -> bool;
}

pub trait AlgebraicallyClosed {
    fn is_algebraically_closed(&self) -> bool;
}

pub trait FiniteCharacteristic {
    fn is_finite_characteristic(&self) -> bool;
}

pub trait OrderedRing {
    fn is_ordered_ring(&self) -> bool;
}

pub trait ZeroRing {
    fn is_zero_ring(&self) -> bool;
}

pub trait Exact {
    fn is_exact(&self) -> bool;
}

pub trait Canonical {
    fn is_canonical(&self) -> bool;
}

pub trait RealPrecision {
    fn has_real_precision(&self) -> bool;
}
