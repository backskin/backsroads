use std::time::{Duration, Instant};
use std::cmp::Ordering;
use rand::{Rng};
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq)]
pub struct Coordinate{
    x_latitude: f64,
    y_longitude: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Coordinate{
        Coordinate{
            x_latitude: x,
            y_longitude: y,
        }
    }

    pub fn get_coors(&self) -> (f64, f64){
        (self.x_latitude, self.y_longitude)
    }
}

pub struct TripSegment{
    pub start: Coordinate,
    pub end: Coordinate,
}
pub struct CarKey{
    key_image: f64,
}

pub struct CarDriver{
    driver_name: String,
    car_key: CarKey,
}

impl CarDriver{
    pub fn new(name: String)-> CarDriver{
        let mut rng = rand::thread_rng();
        let some_random_num: f64 = rng.gen();
        let key = some_random_num * (rng.gen_range(1000, 1000000) as f64);
        CarDriver{
            driver_name: name,
            car_key: CarKey{ key_image: key },
        }
    }

}

pub struct Trip{
    segment: TripSegment,
}

pub struct MetaRoute{
    pub cluster: Vec<Trip>,
    pub representative: Trip,
}

#[derive(Copy, Clone, PartialEq)]
pub struct GeoPosition {
    pub coordinate: Coordinate,
    pub t_timestamp: Instant,
}

impl GeoPosition{
    pub fn new(cor: Coordinate, time: Instant) -> GeoPosition{
        GeoPosition{
            coordinate: cor,
            t_timestamp: time,
        }
    }

    pub fn new_and_now(x_cor: f64, y_cor: f64)->GeoPosition{
        GeoPosition{
            coordinate: Coordinate { x_latitude: x_cor, y_longitude: y_cor },
            t_timestamp: Instant::now(),
        }
    }
}

pub struct CarRawData{
    pub dude: CarDriver,
    gps_data: Vec<GeoPosition>
}

impl CarRawData{

    pub fn new(driver: CarDriver) -> CarRawData{
        CarRawData{
            dude: driver,
            gps_data: vec![],
        }
    }

    pub fn remove(&mut self, position: &GeoPosition){
        let mut index = 0;
        for i in 0..self.gps_data.len() {
            if self.gps_data.get(i) == position{
                index = i;
                break;
            }
        }

        self.gps_data.remove(index);
    }

    pub fn add(&mut self, position: GeoPosition){
        self.gps_data.push(position);
    }

    pub fn get_iterator(&self) -> Iter<'_, GeoPosition> {
        return self.gps_data.iter();
    }
}

struct CarTrips{
    data: Vec<Trip>
}

pub struct TripAdvisor{
    trip_gap: Duration,
    car_trips: CarTrips,
}

impl TripAdvisor{

    fn new(raw_data: &CarRawData) -> TripAdvisor {

    }

    fn segments_cleaner(&self){

    }

    fn min_point_trip_filter(&self, threshold: u16){

    }

    fn min_time_trip_filter(&self, threshold: Duration){

    }

    fn centroid_dist_trip_filter(&self, win_size_in_points: u8, threshold: f32){

    }

    fn direction_dyn_trip_filter(&self, threshold: f32){

    }

    fn most_recent_trip_filter(&self){

    }

    fn apply_all_filters(&self){
        let min_point: u16 = 10;
        let min_time: Duration = Duration::new(30, 0);
        let win_size: u8 = 10;
        let min_dist: f32 = 0.5;
        let max_ratio: f32 = 0.6;

        self.segments_cleaner();
        self.min_point_trip_filter(min_point);
        self.min_time_trip_filter(min_time);
        self.centroid_dist_trip_filter(win_size, min_dist);
        self.direction_dyn_trip_filter(max_ratio);
        self.most_recent_trip_filter();
    }

}

impl GeoPosition {
    fn stamp_now(x: f64, y:f64) -> GeoPosition {
        GeoPosition {coordinate: Coordinate{x_latitude:x, y_longitude: y}, t_timestamp: Instant::now()}
    }

    fn make_from(coordinate: Coordinate, t: Instant) -> GeoPosition {
        GeoPosition{ coordinate, t_timestamp: t }
    }
}

impl PartialOrd for GeoPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.t_timestamp.partial_cmp(&other.t_timestamp)
    }
}

