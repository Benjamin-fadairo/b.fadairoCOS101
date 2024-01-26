use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your Age:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Are you experienced?(y/n)");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let e = input2.trim();

    let mut experience = false;
    if e == "y" {
        experience = true;
    }

    if !experience{ 
        println!("Your incentive is 100000");
    }else if a >= 40{
        println!("Your incentive is 1560000");
    }else if a >= 30{
        println!("Your incentive is 1480000");
    }else{
        println!("Your incentive is 1300000");
    }
}
