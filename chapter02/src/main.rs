fn main() {
    variance1();
}

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

fn run3() {
    let x1 = 42;
    let y1 = Box::new(84);
    {
        let z = (x1, y1);
    }
    let x2 = x1;
    //let y2 = y1;
}

fn call_replace_with_84() {
    let mut s = Box::new(42);
    replace_with_84(&mut s);
}

fn replace_with_84(s: &mut Box<i32>) {
    //let was = *s;
    let was = std::mem::take(s);
    *s = was;

    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}

fn lifetimes_holes() {
    let mut x = Box::new(42);
    let mut z = &x;
    for i in 0..100 {
        println!("{}", z);
        x = Box::new(i);
        z = &x;
    }
    println!("{}", z);
}

fn variance1() {
    struct MutStr<'a, 'b> {
        s: &'a mut &'b str,
    }
    let mut s = "hello";
    *MutStr { s: &mut s }.s = "world";
    println!("{}", s);
}

fn variance2() {
    struct MutStr<'a> {
        s: &'a mut &'a str,
    }
    let mut s = "hello";
    *MutStr { s: &mut s }.s = "world";
    println!("{}", s);
}
