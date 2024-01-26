use std::io;

struct Company{
    company: String,
    shares: f32,
    liabilities: f32,
    year: String,

}

fn main() {
    let _economy = vec![
        Company {
            company: String::from("\nCadbury Nigeria Plc"),
            shares: 150000000.0,
            liabilities: 5500000.0,
            year: String::from("\t1965"),
        },
        Company {
            company: String::from("\nChampion Breweries Plc"),
            shares: 25000000.0,
            liabilities: 8000000.0,
            year: String::from("\t1974"),
        },
        Company {
            company: String::from("\nDangote Sugar Refinery Plc"),
            shares: 18000000.0,
            liabilities: 10000000.0,
            year: String::from("\t1970"),
        },
        Company {
            company: String::from("\nFlour Mills Nigeria Plc"),
            shares: 32000000.0,
            liabilities: 4000000.0,
            year: String::from("\t1960"),
        },
        Company {
            company: String::from("\nNestle Nigeria Plc"),
            shares: 8000000.0,
            liabilities: 1500000.0,
            year: String::from("\t1961"),
        },
        Company {
            company: String::from("\nUnilever Nigeria Plc"),
            shares: 37000000.0,
            liabilities: 11000000.0,
            year: String::from("\t1923"),
        },
        Company {
            company: String::from("\nHoneywell Nigeria Plc"),
            shares: 34000000.0,
            liabilities: 9000000.0,
            year: String::from("\t1906"),
        },
        Company {
            company: String::from("\nNigerian Breweries Plc"),
            shares: 30000000.0,
            liabilities: 12000000.0,
            year: String::from("\t1940"),
        }
    ];
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your Username");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let username:String = input1.trim().parse().expect("Not a valid user");

    println!("Enter your Password");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let password:String = input2.trim().parse().expect("Not a valid Password");

    if username.len() < 3 || username.len() > 8{
        println!("Invalid Username")
    }
    for c in password.chars() {
        if password.len() < 3 || password.len() > 8{
            println!("Invalid password length")
        }else if !c.is_ascii_alphabetic() && !c.is_ascii_digit() && !c.is_ascii_uppercase(){
            println!("Invalid Password")
        }

    }
}


