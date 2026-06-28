pub fn variable() {
    println!("================================== VARIABLES ==================================");
    // VARIABLES
    // EXAMPLE VARIABLE IMMUTABLE
    let name = "John Doe";
    println!("The value of name is: {}", name);
    // EXAMPLE VARIABLE MUTABLE
    let mut age = 30;
    println!("The value of age is first mutable: {}", age);
    age = 31;
    println!("The value of age is changed: {}", age);
    println!("================================== VARIABLES ==================================");

    println!("================================== STATICS TYPES ==================================");
    // EXAMPLE STATIC TYPES
    let x: i32 = 5;
    let y: f64 = 3.14;
    let z: bool = true;
    println!("The value of x is: {} integer 32", x);
    println!("The value of y is: {} float 64", y);
    println!("The value of z is: {} boolean", z);
    // EXAMPLE STATIC TYPES ERROR
    // let mut z: bool = 1;
    // println!("The value of z is: {} boolean", z);
    println!("================================== STATICS TYPES ==================================");

    println!("================================== SHADOWING ==================================");
    let name1 = "Alex Smith";
    println!("The value of name1 is: {}", name1);
    let name1 = "Jane Doe";
    println!("The value of name1 is: {}", name1);
    println!("================================== SHADOWING ==================================");
}
