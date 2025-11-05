fn main() {
    let fullname = "Kenechukwu OLuwanifemi Uba";
    let department = "Data Science";
    let uni = "pan-Atlantic University";

    let mut school = "School of Science".to_string();
    //push string
    school.push_str(" and Technoogy");

    println!("My name is: {}", fullname);
    //check length
    println!("The length of my fullname is {}", fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}", school);
    println!("{}", uni);
}
