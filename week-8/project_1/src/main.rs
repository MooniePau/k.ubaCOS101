use std::io;

// Function that asks the user for input and returns it as a String.
fn ask_info_string(prompt:&str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read input");

    input.trim().to_string()
}

// Function that asks the user for input and returns it as a number (f32).
fn ask_info_int(prompt:&str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read input");

    input.trim().parse().expect("Invalid input")
}

fn main() {

    // Collect user info
    let name:String = ask_info_string("Please enter your name: ");
    let position_held:String = ask_info_string("Please enter your current position: ");
    let years_of_experience:f32 = ask_info_int("Please enter your years of experience");

    //Job roles grouped into vectors,
    // matching each APS/EL/SES level
    let aps12 = vec!["Intern", "-", "Paralegal", "Placement"];
    let aps35 = vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"];
    let aps58 = vec!["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"];
    let el1_8_10 = vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"];
    let el2_10_13 = vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"];
    let ses = vec!["CEO", "Dean", "Partner", "Principal"];

    //matching with a bool
    match true {

        // APS 1–2 → experience 0–3 years
        _ if aps12.contains(&position_held.as_str()) && years_of_experience <= 3.0 => {
            println!("Hello {}, your APS level is: APS 1–2", name);
        }

        // APS 3–5 → experience 3–5 years
        _ if aps35.contains(&position_held.as_str()) && years_of_experience >= 3.0 && years_of_experience <= 5.0 => {
            println!("Hello {}, your APS level is: APS 3–5", name);
        }

        // APS 5–8 → experience 5–8 years
        _ if aps58.contains(&position_held.as_str()) && years_of_experience >= 5.0 && years_of_experience <= 8.0 => {
            println!("Hello {}, your APS level is: APS 5–8", name);
        }

        // EL1 (8–10)
        _ if el1_8_10.contains(&position_held.as_str()) && years_of_experience >= 8.0 && years_of_experience <= 10.0 => {
            println!("Hello {}, your APS level is: EL1 (8–10)", name);
        }

        // EL2 (10–13)
        _ if el2_10_13.contains(&position_held.as_str()) && years_of_experience >= 10.0 && years_of_experience <= 13.0 => {
            println!("Hello {}, your APS level is: EL2 (10–13)", name);
        }

        // SES → 13+ years
        _ if ses.contains(&position_held.as_str()) && years_of_experience >= 13.0 => {
            println!("Hello {}, your APS level is: SES", name);
        }

        // If no match happens, the role or experience is not in the table
        _ => {
            println!("Sorry {}, your role or experience does not match any APS level.", name);
        }
    }
}
