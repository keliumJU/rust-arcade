#[derive(PartialEq, Debug)]
pub struct Car {
    pub color: String, 
    pub motor: Transmission,
    pub roof: bool,
    pub age: (Age, u32)
}

#[derive(PartialEq, Debug)]
pub enum Age {
    New,
    Used
}

#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}


pub fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color: color, 
        motor: motor, 
        roof: roof, 
        age: car_quality(miles),
    }
}

pub fn car_quality(miles: u32) -> (Age, u32) {
    let quality: (Age, u32) = (Age::New, miles);
    quality
}



pub fn car_main_ex() {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut car: Car = Car {color: String::from(colors[0]), motor: Transmission::Manual, roof: false, age: (Age::New, 0)};

    let mut engine: Transmission = Transmission::Manual;

    car = car_factory(String::from(colors[0]), engine, false, 5);
    println!("Car 1 = {}, {:?} motor, roof: {}, mileage: {:?}", 
        car.color, car.motor, car.roof, car.age);
}
