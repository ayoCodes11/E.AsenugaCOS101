use std::io;

fn main() {
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
}
