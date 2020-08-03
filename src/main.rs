use crate::sweet_mod::Coordinate;

mod funny_generators;
mod sweet_mod;

fn main(){
    println!("Hello");

    let dot_a = Coordinate::new(1.0, 0.0);
    let dot_b = Coordinate::new(0.0, 1.0);

    println!("angle <AB = {}; in degrees <AB = {}",
             dot_a.rad_angle(&dot_b),
             dot_a.deg_angle(&dot_b),
    );
    println!("distance AB = {}", dot_a.distance_to(&dot_b));


}