struct Rectangle {
    w: f64,
    h: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}


fn main() {
    let r = Rectangle {
        w: 2.0,
        h: 3.0,
    };

    println!("{}", r.area());
}
