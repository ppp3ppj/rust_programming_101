// for demo code can use allow this = disble warn
#![allow(unused_variables)]

use std::collections::HashMap;
//use basic_rust::Person;
//use basic_rust::person::Person;
//use basic_rust::customers::Customer;

use basic_rust::{
    person::Person,
    customers::Customer,
    speaking::Speaking
};

fn main() {

    let x = 10;
    let (x, y) = (10, 20);

    const PI: f64 = 3.14;

    //Tuple
    let x =  (1, 3.14, 1000);
    let x: (u8, f64, i32) = (1, 3.14, 1000);
    let a = x.0;
    let b = x.1;
    let c = x.2;

    // Array
    let x: [i32; 5];
    let x = [1, 2, 3, 4, 5];
    let x = [0;5];

    let score = 50;
    let mut grade = "";
    if score >= 80 {
        grade = "A";
    } else if score >= 70 {
        grade = "B";
    } else if score >= 60 {
        grade = "C";
    } else if score >= 50 {
        grade = "D";
    } else {
        grade = "F";
    }

    let grade = if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else if score >= 60 {
        "C"
    } else if score >= 50 {
        "D"
    } else {
        "F"
    };

    // Ternary Operator
    let result = if score >= 50 { "Pass" } else { "Fail" };

    // Loop
    //while true {
     //   break;
    //}

    // can set name of loop
    /*
    'label1: loop {
        'label2: loop {
            break 'label1;
            continue 'label2;
        }
    }
    */

    // .. = 0 - 9
    // ..= = 0-10
    for i in 0..3 {
       println!("{}", i);
    }

    for i in 0..=3 {
       println!("{}", i);
    }

    let numbers = [10, 20, 30];
    for n in numbers.iter() {
        println!("{}", n)
    }

    for n in [10, 20, 30].iter() {
        println!("{}", n)
    }
    let numbers = [(1, 2), (3, 4)];
    for (i, j) in numbers.iter() {
        println!("{} {}", i, j);
    }

    // String
    let x = "hello"; // string slice
    let x = String::from("Hello");
    let x = "Hello".to_string();

    // Array is fix size
    // Collection can add resize
    let mut x: Vec<i32> = Vec::new();
    x.push(10);
    x.push(29);
    x.push(30);
    let y = x.pop();

    // Macro
    let mut x = vec![1, 2, 3];

    //Hash map
    let mut x: HashMap<&str, &str> = HashMap::new();
    x.insert("th", "thailand");
    x.insert("us", "united state");
    let y = x.get("th");
    // must unwrap() will get data from key hashmap
    println!("{}", y.unwrap());

    // Struct
     /*let p = Person{
        name: "Bone".to_string(),
        age: 18,
    };
    */
    let new_access = Person::new("pp".to_string(), 18);
    new_access.hello();
    new_access.speak();

    //Trait same interface

    // Enum
    let x = Colors::Red;
    let mut color = "";
    match x {
        Colors::Red => color = "red",
        Colors::Green => { color = "green" },
        _ => color = "blue"
    };

    let color = match x {
        Colors::Red => "red",
        Colors::Green => "green",
        _ => "blue"
    };

    let x = checkGrade(100);
    match x {
        GradeResult::Error(e) => println!("{}", e),
        GradeResult::Value(g) => println!("{}", g),

    };

    let x = checkGrade2(-1);
    match x {
        None => println!("error!"),
        Some(v) => println!("{}", v),
    }

    let x = checkGrade3(-1);
    match x {
        Err(e) => println!("{}", e),
        Ok(v) => println!("{}", v),
    }

    let x = checkGrade3(100);

    if x.is_err() {
        return;
    }
    let y = x.unwrap(); // not recommend please check err real use guard pattern befor unwarp()

    if let Ok(v) = x {
       println!("{}", v);
    }

    // Best
    let x = checkGrade3(100);
    let y = match x {
        Err(e) => {
            println!("{}", e);
            return;
        },
        Ok(v) => v,
    };

    // Closures is mean lamda or anonymus fn

    //println!("{}", p.name);
    let x = |a:i32, b:i32| a + b;
    let y = x(10, 20);
    // function type
    let y = cal(10, 20, x);
    let y = cal(10, 20, |a,b| a - b);


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

fn cal<F: Fn(i32, i32) -> i32>(a: i32, b: i32, f:F) -> i32 {
    f(a, b)
}

// short hand
fn cal2<F>(a: i32, b: i32, f:F) -> i32
where
        F: Fn(i32, i32) -> i32,
    {
        f(a,b)
    }


fn checkGrade(score: i32) -> GradeResult {
    if score < 0 || score > 100 {
       return GradeResult::Error("Score is not correct".to_string());
    }
    return GradeResult::Value("A".to_string());
}

fn checkGrade2(score: i32) -> Option<String>{
   if score < 0 || score > 100 {
      return None;
   }
    //return Some("A".to_string());
   Some("A".to_string())
}

fn checkGrade3(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("Score is not correct".to_string());
    }
    Ok("A".to_string())
}

enum GradeResult {
   Value(String),
   Error(String),
}

enum Colors where{
    Red,
    Green,
    Blue
}



fn get_number() -> i32 {
    let a = 10;
    let b = 20;
    //return a + b;
    // trail return same ruby
    a + b
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
