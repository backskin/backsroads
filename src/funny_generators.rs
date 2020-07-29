use crate::sweet_mod::{CarRawData, GeoPosition, CarDriver};
use rand::Rng;

pub(crate) trait GeoGenerator {
    fn next_point(&mut self) -> GeoPosition;
}

pub(crate) enum Direction{
    Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft,
}

pub struct LineGenerator {
    seed: u32,
    last_x_point: f64,
    last_y_point: f64,
    range_value: f64,
    direction: Direction,
}

impl LineGenerator{
    pub fn new(start_x: f64, start_y: f64, range: f64, direction: Direction) -> LineGenerator{
        LineGenerator{
            seed: 0,
            last_x_point: start_x,
            last_y_point: start_y,
            range_value: range,
            direction,
        }
    }
}

impl GeoGenerator for LineGenerator {
    fn next_point(&mut self) -> GeoPosition {
        self.last_x_point += 1.0;
        let y = rand::thread_rng().gen_range(-self.range_value, self.range_value);
        GeoPosition::new_and_now(self.last_x_point, y)
    }
}

pub fn generate_car_movement(meta_generator: &mut dyn GeoGenerator, points_amount: u32) -> CarRawData {

    let some_dude = CarDriver::new("Dude".to_string());

    let mut output = CarRawData::new(some_dude);

    for i in 0..points_amount {
        let point = meta_generator.next_point();
        output.add(point);
    };

    output
}

pub fn print_out_movement(raw_data: &CarRawData){

    for (index, chunk) in raw_data.get_iterator().enumerate(){
        let (x, y) = &chunk.coordinate.get_coors();
        println!("{}-th point: {}, {}, {}", index, x, y, chunk.t_timestamp.elapsed().as_secs());
    }
}