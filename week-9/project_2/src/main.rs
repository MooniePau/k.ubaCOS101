use std::fs::File;
use std::io::Write;

fn main(){
    //Data stored in vectors
    let student_names = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matric_number = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001"
    ];

    let department = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical"
    ];

    let level = vec![
        "300",
        "100",
        "200",
        "200",
        "100"
    ];

    //Create the file
    let mut file = File::create("PAU-SMIS.txt").unwrap();

    //Write the title
    let title = format!("{:<20} {:<15} {:<15} {:<10}\n","Student Name", "Matric Number", "Department","Level");
    file.write_all(title.as_bytes()).unwrap();

    //loop through each row of data
    for i in 0..student_names.len() {
        let col1 = student_names.get(i).unwrap_or(&"");
        let col2 = matric_number.get(i).unwrap_or(&"");
        let col3 = department.get(i).unwrap_or(&"");
        let col4 = level.get(i).unwrap_or(&"");

        //Put in a fixed column width to align everything
        let row = format!("{:<20} {:<15} {:<15} {:<5}\n", col1, col2, col3, col4);
        file.write_all(row.as_bytes()).unwrap();
    }

    println!("Project II complete!");

}