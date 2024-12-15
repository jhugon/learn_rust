use rand::prelude::*;
use rand_distr::StandardNormal;

fn main() {

    let n = 10;
    let dist = StandardNormal;
    println!("Hello, world! {}",n);
    for _ in 0..n {
        let val: f64 = thread_rng().sample(dist);
        println!("{}",val);
    }
}
