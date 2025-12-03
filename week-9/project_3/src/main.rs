use std::fs::File;
 use std::io::Write;

fn main() { 
    // Prepare the three datasets 
    let commissioners = vec![
        "Ajibogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Collins Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Ekiye",
    ];


    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defence",
        "Power & Steel",
        "Petroleum"
    ];

    let zone = vec![
        "South West",
        "North East",
        "South East",
        "South South",
        "South West"
    ];

    //Create file
    let mut file = File::create("efcc_records.txt").unwrap();

     //Write headers with fixed column widths
    let header = format!("{:<5} {:<15} {:<15} {:<15}\n", "S/N", "Commissioner", "Ministry", "Zone");
    file.write_all(header.as_bytes()).unwrap();

    //Merge using loop
    let header = format!("{:<5} {:<15} {:<15} {:<15}\n", "S/N", "Commissioner", "Ministry", "Zone");
    file.write_all(header.as_bytes()).unwrap();

    //Loop through each row of data
    for i in 0..commissioners.len() {
        let col1 = format!("{}", i + 1);
        let col2 = commissioners.get(i).unwrap_or(&"");
        let col3 = ministries.get(i).unwrap_or(&"");
        let col4 = zone.get(i).unwrap_or(&"");

        // Align everything with fixed column widths
        let line = format!("{:<15} {:<15} {:<15} {:<15}\n", col1, col2, col3, col4);
        file.write_all(line.as_bytes()).unwrap();
    }


    println!("Project III complete");
}