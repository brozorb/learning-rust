use std::time::Instant;



fn main() {
    let now = Instant::now();
    for i in 0..1_000_000 {
        println!("{}", i);
    }
    let elapsed = now.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", elapsed);
}