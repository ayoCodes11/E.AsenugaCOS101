use std::io;

fn main() {

    let mut dev : Vec<(String, i32)> = Vec::new();

    println!("Welcome to the EY Nigeria program for Developer Scouting");
    println!("How Many developers are on the queue to use this program?");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let dev_count:i32 = input1.trim().parse().expect("Invalid input");

    for count in 0..dev_count {
        let mut input2 = String::new();
        println!("Developer {}, enter your name :", count+1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_name:String = input2.trim().parse().expect("Invalid input");

        let mut input3 = String::new();
        println!("Developer {}, enter the number of years of programming experiance you currently have :", count+1);
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let new_experience:i32 = input3.trim().parse().expect("Invalid input");

        dev.push((new_name, new_experience));
    }


    let mut max_experience = 0;
    let mut most_experienced = "";

    for (name, exp) in &dev {

        if *exp > max_experience{
            max_experience = *exp;
            most_experienced = name;
        }
    }


    println!("The Developer with the Highest number of years of programming experience is :\n ");
    println!("{} with {} years of experience.", most_experienced, max_experience);




}
