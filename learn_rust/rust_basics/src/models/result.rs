#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(divided: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    }else {
        Ok(divided / divisor)
    }
}

pub fn show_struct_result_ex() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(9.0, 2.0));
}
