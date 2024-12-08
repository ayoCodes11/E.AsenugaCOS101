use std::io;

fn main() {
    
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("Welcome to the Nigerian Federal Government's APS Level Checker. Please enter your Occupation");
    println!("(Enter either : Office Administrator, Academic, Lawyer, Teacher");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    input1 = input1.to_lowercase();

    if input1.trim() == "office administrator" {

        println!("What is your role in your Occupation?");
        println!("(Enter either : Intern, Administrator, Senior Administrator, Office Manager, Director, CEO");

        io::stdin().read_line(&mut input2).expect("Failed to read input");
        input2 = input2.to_lowercase();

    }
    else if input1.trim() == "academic" {

        println!("What is your role in your Occupation?");
        println!("(Enter either : Research Assistant, PhD Candidate, Post-Doc Researcher, Senior Lecturer, Dean");

        io::stdin().read_line(&mut input2).expect("Failed to read input");
        input2 = input2.to_lowercase();

    }
    else if input1.trim() == "lawyer" {

        println!("What is your role in your Occupation?");
        println!("(Enter either : Paralegal, Junior Associate, Associate, Senior Associate 1-2, Senior Associate 3-4, Partner");

        io::stdin().read_line(&mut input2).expect("Failed to read input");
        input2 = input2.to_lowercase();

    }
    else if input1.trim() == "teacher" {

        println!("What is your role in your Occupation?");
        println!("(Enter either : Placement, Classroom Teacher, Senior Teacher, Leading Teacher, Deputy Principal, Principal");

        io::stdin().read_line(&mut input2).expect("Failed to read input");
        input2 = input2.to_lowercase();

    } 
    else 
    {
        println!("Invalid input");
    }

    println!("How many years of Experience do you have?");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let exp:i32 = input3.trim().parse().expect("Invalid input");

    let ans:(String,i32) = (input2, exp);
    get_aps_level(ans);
}

fn get_aps_level(x:(String,i32)) {

    let (role,exp) = x;


    if role.trim() == "intern" || role.trim() == "paralegal" || role.trim() == "placement"
    {
        if exp >= 1 && exp <= 2
        {
            println!("Your APS Level is : APS 1-2");
        }
        else 
        {
            println!("The role you entered requires more/less years of experience");
        }
    }

    if role.trim() == "administrator" || role.trim() == "research assistant" || role.trim() == "junior associate" || role.trim() == "classroom teacher"
    {
        if exp >= 3 && exp <= 5
        {
            println!("Your APS Level is : APS 3-5");
        }
        else 
        {
            println!("The role you entered requires more/less years of experience");
        }
    }

    if role.trim() == "senior administrator" || role.trim() == "phd candidate" || role.trim() == "associate" || role.trim() == "senior teacher"
    {
        if exp >= 5 && exp <= 8
        {
            println!("Your APS Level is : APS 5-8");
        }
        else 
        {
            println!("The role you entered requires more/less years of experience");
        }
    }

    if role.trim() == "office manager" || role.trim() == "post-doc researcher" || role.trim() == "senior associate 1-2" || role.trim() == "leading teacher"
    {
        if exp >= 8 && exp <= 10
        {
            println!("Your APS Level is : EL1 8-10");
        }
        else 
        {
            println!("The role you entered requires more/less years of experience");
        }
    }

    if role.trim() == "director" || role.trim() == "senior lecturer" || role.trim() == "senior associate 3-4" || role.trim() == "deputy principal"
    {
        if exp >= 10 && exp <= 13
        {
            println!("Your APS Level is : EL2 10-13");
        }
        else 
        {
            println!("The role you entered requires more/less years of experience");
        }
    }

    if role.trim() == "ceo" || role.trim() == "dean" || role.trim() == "partner" || role.trim() == "principal"
    {
        if exp >= 13
        {
            println!("Your APS Level is : SES");
        }
        else 
        {
            println!("The role you entered requires more/less years of experience");
        }
    }

}
