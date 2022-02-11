use (crate)::FIRFilter;
use signal_processing::CircularVector;
#[test]
fn delay_line_test() {
    let mut cv = CircularVector::new(25).unwrap();
    let n = 32;
    let mut pop;
    cv.print();
    println!();
    for i in 1..n {
        pop = cv.push(i as f64);
        print!("{} <- ", pop);
        cv.print();
        print!("\n");
    }

    //assert_eq!(cv.get(2), 4.) // Third item is 4.0
}
#[test]
fn filter_test() {
    let size = 5;
    let n_samples = 20;
    let mut f = FIRFilter::new(size, vec![1.0 / (size as f64); size]).unwrap();
    //cv.print();
    println!();
    for _i in 1..n_samples {
        println!("{}", f.filter(1.0));
    }

    //assert_eq!(cv.get(2), 4.) // Third item is 4.0
}
