use std::io::Write;

fn main() {
    
    let datasets = "S/N NAME OF COMMISSIONER  |  MINISTRY  |  GEOPOLITICAL ZONE";
    let name_of_commisioner = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geopolitical_zone = vec!["South West","North East","South South","South West","South East"];

    let mut count = 1;

    let mut file = std::fs::File::create("Convicted Ministers List EFCC.txt").expect("Create failed");

    file.write_all("Here is the list of the Convicted Ministers from different Geopolitical zones in the country\n"
        .as_bytes()).expect("Write Failed");

    file.write_all("\n".as_bytes()).expect("Write failed");

    file.write_all(datasets.as_bytes()).expect("Write Failed");

    file.write_all("\n".as_bytes()).expect("Write failed");

    for i in 0..name_of_commisioner.len()
    {
        let formatted = format!("\n {}. {} | {} | {}\n",count, name_of_commisioner[i],ministry[i],geopolitical_zone[i]);
        file.write_all(formatted.as_bytes()).expect("Write failed");

        count+=1;
    }

    println!("\nData written to file");
}
