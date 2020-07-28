use std::time::{Duration, Instant};
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq)]
struct Coordinate{
    x_latitude: f64,
    y_longitude: f64,
}

struct TripSegment{
    start: Coordinate,
    next: Option<TripSegment>,
}

trait Human{

}

struct CarKey{
    key_image: f64,
}

struct Engine{
    amount_of_cylinders: u32,
    performance: f32,
    ignition_placeholder: f64,
}

enum SeatState{
    Vacant,
    Taken(dyn Human)
}

struct Seat{
    state: SeatState,
}

struct Car{
    engine: Engine,
    seats: [Seat; 4],
}


struct CarDriver{
    driver_name: String,
    car_key: CarKey,
}

impl Human for CarDriver{

}

struct Trip{
    segment: TripSegment,
}

struct MetaRoute{
    cluster: Vec<Trip>,
    representative: Trip,
}

#[derive(Copy, Clone, PartialEq)]
struct GeoPosition {
    coordinate: Coordinate,
    t_timestamp: Instant,
}

struct CarData {
    dude: CarDriver,
    gps_data: Vec<GeoPosition>
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

