use std::io;

fn main() {

    let mut input1 = String::new();

    println!("\nSelect an equation");
    println!("Area of Trapezium");
    println!("Area of Rhombus");
    println!("Area of Parallelogram");
    println!("Area of Cube");
    println!("Volume of Cylinder");
    println!();
    println!("(Enter TRAPEZIUM, RHOMBUS, PARALLELOGRAM, CUBE OR CYLINDER)");

    io::stdin().read_line(&mut input1).expect("Invalid input");
    let input1 = input1.to_uppercase();


    if input1.trim() == "TRAPEZIUM"
    {
        trapezium();
    }
    else if input1.trim() == "RHOMBUS"
    {
        rhombus();
    }
    else if input1.trim() == "PARALLELOGRAM"
    {
        parallelogram();
    }
    else if input1.trim() == "CUBE"
    {
       cube();
    }
    else if input1.trim() == "CYLINDER"
    {
        cylinder();
    }
    else 
    {
        println!("invalid input");
    }


}

fn trapezium()
{
    let mut in1 = String::new();
    let mut in2 = String::new();
    let mut in3 = String::new();


    println!("Enter the height");
    io::stdin().read_line(&mut in1).expect("Invalid string");
    let hght:f64 = in1.trim().parse().expect("Invalid input");

    println!("Enter the base 1");
    io::stdin().read_line(&mut in2).expect("Invalid string");
    let bs1:f64 = in2.trim().parse().expect("Invalid input");

    println!("Enter the base 2");
    io::stdin().read_line(&mut in3).expect("Invalid string");
    let bs2:f64 = in3.trim().parse().expect("Invalid input");

    let ans:f64 = hght / 2.0 * (bs1 + bs2);

    println!("Your answer is : {}", ans);
}

fn rhombus()
{
    let mut in1 = String::new();
    let mut in2 = String::new();

    println!("Enter the diagonal 1");
    io::stdin().read_line(&mut in1).expect("Invalid string");
    let d1:f64 = in1.trim().parse().expect("Invalid input");

    println!("Enter the diagonal 2");
    io::stdin().read_line(&mut in2).expect("Invalid string");
    let d2:f64 = in2.trim().parse().expect("Invalid input");

    let ans:f64 = (d1 * d2) / 2.0;

    println!("Your answer is : {}", ans);
}

fn parallelogram()
{
    let mut in1 = String::new();
    let mut in2 = String::new();

    println!("Enter the base");
    io::stdin().read_line(&mut in1).expect("Invalid string");
    let bs:f64 = in1.trim().parse().expect("Invalid input");

    println!("Enter the altitude");
    io::stdin().read_line(&mut in2).expect("Invalid string");
    let alt:f64 = in2.trim().parse().expect("Invalid input");

    let ans:f64 = bs*alt; 

    println!("Your answer is : {}", ans);
}

fn cube()
{
    let mut in1 = String::new();

    println!("Enter the length of a side");
    io::stdin().read_line(&mut in1).expect("Invalid string");
    let ls:f64 = in1.trim().parse().expect("Invalid input");

    let ans:f64 = 6.0 * (ls.powf(2.0));

    println!("Your answer is : {}", ans);
}

fn cylinder()
{
    let mut in1 = String::new();
    let mut in2 = String::new();

    println!("Enter the radius");
    io::stdin().read_line(&mut in1).expect("Invalid string");
    let rd:f64 = in1.trim().parse().expect("Invalid input");

    println!("Enter the height");
    io::stdin().read_line(&mut in2).expect("Invalid string");
    let hght:f64 = in2.trim().parse().expect("Invalid input");

    let ans:f64 = 3.14159 * (rd.powf(2.0)) * hght;

    println!("Your answer is : {}", ans);
}
