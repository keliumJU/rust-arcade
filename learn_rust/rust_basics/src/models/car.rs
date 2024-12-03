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


pub fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Black"];
    
    let mut color = order as usize;

    while color > 4 {
        color = color - 4;
    }

    let mut motor = Transmission::Manual;
    let mut roof = true;

    if order % 3 == 0 {
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }


    Car {
        color: String::from(colors[(color-1) as usize]), 
        motor: motor, 
        roof: roof, 
        age: car_quality(miles),
    }
}

pub fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    }

    let quality: (Age, u32) = (Age::New, 0);
    quality
}


use std::collections::HashMap;


pub fn show_car_ex() {

    let mut car: Car;
    let mut orders: HashMap<i32, Car> = HashMap::new();
    let day_orders = 12;
    let mut miles = 0;

    for order in 1..day_orders {

        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car Order {}: {:?}", order, orders.get(&order));

        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}
