use std::io;
fn main() {
    let mut number_of_candidates = String::new();
    println!("How many candidates have registered?");
    io::stdin().read_line(&mut number_of_candidates).expect("Invalid Input");
    let number_of_candidates:i32 = number_of_candidates.trim().parse().expect("Invalid Integer");

    if number_of_candidates > 150{
        println!("Sorry, you aren't eligible to vote; The maximum number of candidates has already been reached");
    }else{
        let mut level = String::new();
        loop{
            let mut level_forloop = String::new();
            println!("What level is the candidate?");
            io::stdin().read_line(&mut level_forloop).expect("Invalid Input");
            level_forloop = level_forloop.trim().to_string();

            if level_forloop != "100"&& level_forloop!= "200" && level_forloop != "300" && level_forloop != "400" && level_forloop != "500"{
                println!("Enter a valid level eg, 100, 200, 300, 400 or 500");
                println!("You typed the level is ({})", level_forloop);
            }else{
                level = level_forloop.to_string();
                break;
            }
        }
        if level == "100"{
            println!("Sorry, You aren't eligible.");
        }else{
            let mut cgpa = 0.0;
            loop{
                let mut cgpa_forloop = String::new();
                println!("Please enter your cgpa");
                io::stdin().read_line(&mut cgpa_forloop).expect("Invalid Input");
                let cgpa_forloop:f32 = cgpa_forloop.trim().parse().expect("Not a valid float");
                if cgpa_forloop < 0.0 || cgpa_forloop > 5.0{
                    if cgpa_forloop < 0.0{
                        println!("You have provided an invalid cgpa");
                    }else{
                        println!("You have provided an invalid cgpa");
                    }
                }else{
                    cgpa = cgpa_forloop;
                    break;
                }
            }
            
            if cgpa < 4.0{
                println!("Sorry your cgpa is too low.");
            }else{
                loop{
                    let mut response = String::new();
                    println!("Are you a class rep, (y/n)");
                    io::stdin().read_line(&mut response).expect("Invalid Integer");

                    if response.to_lowercase().trim() != "y" && response.to_lowercase().trim() != "n"{
                        println!("You've entered an invalid response. Type y for yes, n for no");
                    }else if response.to_lowercase().trim() == "y"{
                        println!("You can vote. But we need to collect some more information first");

                        let mut name = String::new();
                        println!("What is your name");
                        io::stdin().read_line(&mut name).expect("Invalid input");

                        let mut email = String::new();

                        loop{
                            let mut email_forloop = String::new();
                            println!("What is your email");
                            io::stdin().read_line(&mut email_forloop).expect("Invalid input");

                            if email_forloop.trim().contains("@") && email_forloop.trim().ends_with(".com"){
                                email = email_forloop;
                                break;
                            }else{
                                println!("You've entered an invalid email.");
                            }

                        }