use std::io;


fn main() {
    

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the coefficient of x^2 in the Quadratic Equation");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the coefficient of x in the Quadratic Equation");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the constant in the Quadratic Equation");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {

        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);

        println!("The discriminant is positive and roots are : {} and {}.", root1, root2);

    }else if d == 0.0 {

        let root = -b / (2.0 * a);
        println!("The discriminant is zero, and the real root is : {}.", root);

    }else {

        println!("The discriminant is zero, so there are no real roots.");
    }
}
