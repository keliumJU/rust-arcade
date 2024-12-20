use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn show_point_ex() {
    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 3, y: 5};

        println!("equal!");
        if p1 == p2 {
    }else {
        println!("no equal!");
    }

    println!("{}", p1);
    println!("{:?}", p1);
}
