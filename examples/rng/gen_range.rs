use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen_range(0..10);
    println!("random 0..10 {}",n1);
    let n2: f32 = rng.gen_range(0.0..10.0);
    println!("random 0.0..10.0 {}",n2);
}