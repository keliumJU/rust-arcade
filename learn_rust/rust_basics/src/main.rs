mod models;

use crate::models::car::{
    Transmission,
    car_factory,
};

fn main() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", 
        car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, false);

    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Gold"), Transmission::SemiAuto, true);

    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage);

}
