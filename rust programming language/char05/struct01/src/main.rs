struct Rectangle {
    w: f64,
    h: f64
}

fn area(rect: Rectangle) -> f64 {
    rect.w * rect.h
}


fn main() {
    let r = Rectangle {
        w: 2.0,
        h: 3.0,
    };

    println!("{}", area(r));
}
