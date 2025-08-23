
mod utils;
mod math;
mod core;
mod materials;
mod sdf;
mod demos;

use demos::cornell_box::cornell_box;
use demos::spheres::spheres;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <demo_name>");
        eprintln!("Available demos: cornell_box, spheres");
        return;
    }

    match args[1].as_str() {
        "cornell_box" => cornell_box(),
        "spheres" => spheres(),
        _ => {
            eprintln!("Unknown demo: {}", args[1]);
            eprintln!("Available demos: cornell_box, spheres");
        }
    }
}
