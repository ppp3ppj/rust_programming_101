// for demo code can use allow this = disble warn
#![allow(unused_variables)]
fn main() {
    //let mut n1 = 1;
    //let n2 = &mut n1;
    //*n2 = 20;
    //let n3 = n2;
    //let n4 = &n2;
    //println!();
    //println!("{}", n1);
    //println!("{}", n2);

//    let n1 = 1;
//    let n2 = 2;
//    ex1_1();
//    println!();
//    println!();
    //let mut n1 = 1;
    //hello(&mut n1);
    //println!("{}", n1);

    //heap learn
    let n1 = Box::new(1);
    let n2 = Box::new(2);
    ex4_1();
    println!();
    println!();
}

fn ex4_1() {
    let n3 = Box::new(3);
    ex4_2();
    println!();
    println!();
}

fn ex4_2() {
    let n4 = Box::new(4);
    println!();
}

fn hello(a:&mut i32) {
    *a = 30;
}

//fn ex1_1() {
//    let n3 = 3;
//    ex1_2();
//    println!();
//}

//fn ex1_2() {
//    let n4 = 4;
//    println!();
//}
