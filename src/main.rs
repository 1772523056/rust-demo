use std::env;
use std::fs;
// use std::intrinsics::powf32;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, p: &Point) -> f64 {
        self.x + self.y
    }
}

fn main() {
    let x = Point {
        x: 1.0,
        y: 2.0,
    };
    let y = Point {
        x: 1.0,
        y: 2.0,
    };
    println!("value : {:?}", x.distance(&y));
    // let args = env::args().collect::<Vec<String>>();
    //
    // let (query, filepath) = parse_config(&args);
    //
    // let content = fs::read_to_string(filepath).expect("something went wrong!");
    // println!("value : {:?}", content);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filepath = &args[2];
    (query, filepath)
}