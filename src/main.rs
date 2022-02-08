use signal_processing::CircularVector;

fn main() {
    let mut cv = CircularVector::new(5);
    cv.print();
    print!("popped: {}\n", cv.push(1.));
    cv.print();
    print!("popped: {}\n", cv.push(2.));
    cv.print();
    print!("popped: {}\n", cv.push(3.));
    cv.print();
    print!("popped: {}\n", cv.push(4.));
    cv.print();
    print!("popped: {}\n", cv.push(5.));
    cv.print();
    print!("popped: {}\n", cv.push(6.));
    cv.print();
}
