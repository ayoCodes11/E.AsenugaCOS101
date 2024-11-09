use std::io;

fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("What is your name?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Nice to meet you {}. How old are you?", input1 );
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 18 {

        println!("Thats good to know. Now how many years of experience do you have {}?", input1);
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let exp:i32 = input3.trim().parse().expect("Not a valid number");

        if exp >= 18
        {
            if age >= 40 {
                println!("{}, your annual incentive will be 1,560,000 Naira", input1);
            }
            else if age >= 30 && age < 40 {
                println!("{}, your annual incentive will be 1,480,000 Naira", input1);
            }
            else if age < 28 {
                println!("{}, your annual incentive will be 1,300,000 Naira", input1);
            }else {
                println!("We will get back to you {}", input1); 
            }
        }
        else {

            println!("You need to gain experience {}, so your incentive will be 100,000 Naira", input1);
        }

    }
    else {

        println!("Sorry {} , but you'll have to be older than that to work for this company.", input1);
    }
}
