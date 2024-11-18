
use rufl::*;

fn main() {
    let a = Integer::new(17);
    println!("a = {}", a);

    let b = Rational::new([3, 4]);
    let res = a*b;

    println!("a*b = {}", res);

    let zz = GenericCtx::integer_ring();
    println!("{}", zz);
}
