#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn holds(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn transpose(&self) -> Rectangle {
        Rectangle {
            width: self.height,
            height: self.width,
        }
    }

    fn scale(&self, factor: f64) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 1.5,
        height: 4.5,
    };
    println!("{:?}", rectangle);
    println!("area: {}", rectangle.area());
    println!("perimeter: {}", rectangle.perimeter());

    let rectangle2 = Rectangle {
        width: 100.0,
        height: 100.0,
    };
    println!("{}", rectangle2.holds(&rectangle));

    println!("{:?}", rectangle.transpose());
    println!("{:?}", rectangle.scale(1.5));
}
