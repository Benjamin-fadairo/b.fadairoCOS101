use std::io;

fn main() {
    println!("   menu                          price 
poundo yam/edikaiko soup       -N3,200
fried rice & chicken           -N3,000
Amala and ewedusoup            -N2,500
eba and egusi soup             -N2,000
white rice and stew            -N2.500");
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("How many portions of pounded yam and edinkaikong do you want");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let p:f32 = input1.trim().parse().expect("Not a valid input");

    println!("How many portions of fried rice do you want?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let f:f32 = input2.trim().parse().expect("Not a valid input");

    println!("How many porions of Amala and ewedu soup do you want?");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let a:f32 = input3.trim().parse().expect("Not a valid input");

    println!("How many portions of Eba and egusi soup?");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let e:f32 = input4.trim().parse().expect("Not a valid input");

    println!("How many portions of white rice and stew do you want?");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let w:f32 = input5.trim().parse().expect("Not a valid input");

    let mut p:f32 = 3200.0 * p;
    let mut f:f32 = 3000.0 * f;
    let mut a:f32 = 2500.0 * a;
    let mut e:f32 = 2000.0 * e;
    let mut w:f32 = 2500.0 * w;

    let total = p + f + a + e + w;
    let discount = total * 0.95;

    if total > 10000.0{
        println!("the cost is {}",discount);
    }else{
        println!("The cost is {}",total);
    }
}
