
use rufl::*;

fn main() {
    let a = Integer::new(17);
    println!("a = {}", a);

    let b = Rational::new([3, 4]);
    println!("b = {}", b);
    
    let c = a*b;
    println!("c = {}", c);

    let f = IntPoly::new([0,1,1,0,1]);
    println!("f = {}", f);

    let g = f + c;
    println!("f + c = {}", g);

    let zz = GenericCtx::integer_ring();
    println!("{}", zz);

    let mat = IntMat::new([1,0,0,1], 2, 2);
    println!("mat = {}", mat);
}
