
mod utils;
mod math;
mod core;
mod materials;
mod sdf;
mod demos;

use demos::cornell_box::cornell_box;
use demos::spheres::spheres;

fn main() {
   cornell_box();
   //spheres();
}
