fn main() {}

fn run1() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;
    //println!("{} {}", var1, var2);
}

fn run2() {
    let mut x;
    //assert_eq!(x, 42);
    x = 42;
    let y = &x;
    x = 43;
    //assert_eq!(*y, 43);
}
