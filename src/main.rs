use std::time::{Duration, Instant};
use std::io::{self};
use std::thread;
use rand::Rng;

fn main() {
    println!("press enter key to test reaction time\n");
    let mut input = String::new();

    let mut rng = rand::thread_rng();
    let n: u64 = rng.gen_range(0 .. 3000);

    let delay = Duration::from_millis(n);
    thread::sleep(delay);

    println!("NVGNFNGSFKLGNSFLKNNSKFJ");
    println!("NCSDJKVNSDKJVNSFKJVNSFKVSFNKV");
    println!("CANDJALSCKNALKCNCNALSDF");

    let start = Instant::now();
    io::stdin().read_line(&mut input).expect("failed to read input");
    let elapsed = start.elapsed();

    println!("reaction time: {:#?}", elapsed);
    println!("(delay: {:#?})", delay);
}
