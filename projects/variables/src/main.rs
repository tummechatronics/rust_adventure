const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x: u8 = 25;
    println!("value: {x}");
    x = 6;
    println!("value: {x}");
    println!("value: {THREE_HOURS_IN_SECONDS}");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let t = ([1; 2], [3; 4]);
    println!("{:?}", t);
    //The syntax [x; y] declares an array with y copies of the value x

    let (a, b) = t;
    println!("{:?}", a);
    println!("{:?}", b);

    println!("{}", a[0] + t.1[0]);

    // println!("The value of y is: {y}");
    //creating error
    //THREE_HOURS_IN_SECONDS = 2
}
