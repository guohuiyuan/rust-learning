use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // 生成一个随机的 Point
    let rand_point: Point = rng.gen();
    println!("Random Point: {:?}", rand_point);

    // 通过类型暗示( hint )生成一个随机的元组
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    println!("Random tuple: {:?}", rand_tuple);
}