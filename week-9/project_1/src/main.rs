use std::io::Write;

fn main() {
    
    let cat = "Lager    Stout   Non-Alcoholic
33 Export   Legend  Maltina
Desperados  Turbo King  Amstel Malta
Goldberg    Williams    Malta Gold
Gulder  blank  Fayrouz
Heineken    blank  blank
Star    blank  blank\n";
    
    let mut file = std::fs::File::create("Category of drinks").expect("Create failed");
    file.write_all("Categories of drinks produced by Nigerian Breweries Plc\n".as_bytes()).expect("Write failed");
    file.write_all(cat.as_bytes()).expect("Write failed");
    println!("\nData successfully written to file");
}
