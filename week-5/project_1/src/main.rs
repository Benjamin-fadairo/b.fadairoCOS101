use std::io;


fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the x-square coeficient:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the x coeficient:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter constant:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    
    let mut d = b.powf(2.0) - (4.0 * a * c);
    

    if d < 0.0{
        println!("the roots are imaginary");
    }else if d == 0.0{
        let answer = ((-1.0 * b) + d) / 2.0 * a;
        println!("the root is {}",answer);
    }else {
        d = d.sqrt();
        let answer1 = ((-1.0 * b) + d) / 2.0 * a;
        let answer2 = ((-1.0 * b) - d) / 2.0 * a;

        println!("The answers are {},{}",answer1,answer2);
    }
}