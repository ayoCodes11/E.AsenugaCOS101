use std::io::Write;

fn main() {
 
    let lager = "\nLager : 33 Export, Desperados, Goldberg, Gulder, Heineken, Star.";
    let stout = "\nStout : Legend, Turbo King, Williams.";
    let non_alcoholic = "\nNon-Alcoholic : Maltina, Amstel Malta, Malta Gold, Fayrouz.";

    let mut file = std::fs::File::create("portfolio.txt").expect("Create failed");
    file.write_all("Nigerian Brewries Plc Drinks in their categories\n".as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");

    println!("\nData written to file");

}
