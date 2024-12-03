pub struct Car {
    pub color: String, 
    pub transmission: Transmission,
    pub convertible: bool,
    pub mileage: u32,
}

#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}


pub fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        color: color, 
        transmission: transmission, 
        convertible: convertible, 
        mileage: 0
    }
}

pub fn car_main_ex() {
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
