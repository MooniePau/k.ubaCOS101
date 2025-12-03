use std::fs::File;
use std::io::Write;

fn main(){
    //Store table data in vectors
    //store lager data
    let lager = vec! [
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star"
    ];

    //store stout data
    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams"
    ];

    //store non alcoholic data
    let non_alcoholic = vec! [
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz"
    ];

    //create a file to write column headers
    let mut file = File::create("Nigerian_Bewery_Limited_drinks_table").unwrap();

    //Write the column title with proper spacing
    let header = format!("{:<15} {:<15} {:<20}\n","Lager", "Stout", "Non-Alcoholic");
    file.write_all(header.as_bytes()).unwrap();

    //Loop through each row of data
    for i in 0..6 {
        let col1 = lager.get(i).unwrap_or(&"");
        let col2 = stout.get(i).unwrap_or(&"");
        let col3 = non_alcoholic.get(i).unwrap_or(&"");

        //Put in a fixed column width to align everything
        let row = format!("{:<15} {:<15} {:<20}\n", col1,col2, col3);
        file.write_all(row.as_bytes()).unwrap();
    }

    println!("Project I complete!");

}