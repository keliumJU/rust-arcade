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
