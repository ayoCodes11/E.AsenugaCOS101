use std::io;

fn main() {
<<<<<<< HEAD
    let p:f32 = 3200.0;
    let f:f32 = 3000.0;
    let a:f32 = 2500.0;
    let e:f32 = 2000.0;
    let w:f32 = 2500.0;

    let mut order = String::new();
    let mut input2 = String::new();

    let mut bill:f32 = 0.0;

    println!("Here's the menu
        P = Poundo Yam/Edinkaiko Soup - N{}
        F = Fried Rice & Chicken      - N{}
        A = Amala & Ewedu Soup        - N{}
        E = Eba & Egusi Soup          - N{}
        W = White Rice & Stew         - N{}",
        p,f,a,e,w);

    println!("What would you Like to order today? (Enter either P,F,A,E or W) ");
    io::stdin().read_line(&mut order).expect("Invalid String");
    let order = order.to_uppercase();

    if order.trim() == "P" {
        println!("What Quantity of Poundo Yam/Edinkaiko Soup would you like (Enter any number)");
        io::stdin().read_line(&mut input2).expect("Invalid String");
        let qty:f32 = input2.trim().parse().expect("Invalid number");

        bill = p * qty;
    }
    else if order.trim() == "F" {

        println!("What Quantity of Fried Rice & Chicken would you like (Enter any number)");
        io::stdin().read_line(&mut input2).expect("Invalid String");
        let qty:f32 = input2.trim().parse().expect("Invalid number");

        bill = f * qty;

    }
    else if order.trim() == "A" {

        println!("What Quantity of Amala & Ewedu Soup would you like (Enter any number)");
        io::stdin().read_line(&mut input2).expect("Invalid String");
        let qty:f32 = input2.trim().parse().expect("Invalid number");

        bill = a * qty;

    }
    else if order.trim() == "E" {

        println!("What Quantity of Eba & Egusi Soup would you like (Enter any number)");
        io::stdin().read_line(&mut input2).expect("Invalid String");
        let qty:f32 = input2.trim().parse().expect("Invalid number");

        bill = e * qty;

    }
    else if order.trim() == "W" {

        println!("What Quantity of White Rice & Stew would you like (Enter any number)");
        io::stdin().read_line(&mut input2).expect("Invalid String");
        let qty:f32 = input2.trim().parse().expect("Invalid number");

        bill = w * qty;

    }
    else 
    {
        println!("Invalid order");
    }

    if bill > 10_000.0
    {
        let disc:f32 = (5.0/100.0) * bill;

        let np:f32 = bill - disc;
        println!("Your total is : {}", np);
    }
    else 
    {
        println!("Your total is : {}", bill);
    }
=======

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
>>>>>>> 1aa8d7d85d471ef10404671e8bab0bf4233307b0
}
