
use rufl::*;

fn main() {
    let a = Integer::new(17);
    println!("a = {}", a);

    let ZZ = GenericCtx::integer_ring();
    println!("{}", ZZ);    
}
