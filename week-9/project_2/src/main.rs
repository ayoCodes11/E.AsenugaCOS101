use std::io::Write;

fn main() {
    let title = "PAU SMIS\n";
    let headers = "Student Name, Matric Number, Department, Level\n";
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
        ("Shania Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
        ("Bianca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];

    // Open file for writing
    let mut file = std::fs::File::create("PAU_SMIS.txt").expect("Create failed");

    // Write title and headers
    file.write_all(title.as_bytes()).expect("Write failed");
    file.write_all(headers.as_bytes()).expect("Write failed");

    // Write student data
    for student in students {
        let formatted = format!("{}, {}, {}, {}\n", student.0, student.1, student.2, student.3);
        file.write_all(formatted.as_bytes()).expect("Write failed");
    }

    println!("\nData Written to file");
}