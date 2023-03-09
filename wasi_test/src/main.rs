extern "C" {
    fn yield_now();
}

fn main() {
    println!("Hello, world!");
    for i in 0..50 {
        println!("i: {i}");
        unsafe { yield_now(); }
    }
    panic!("Panic!");
}
