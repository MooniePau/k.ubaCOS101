use std::io;

// Function to get string input
fn ask_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Function to get number input
fn ask_float(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}

fn main() {
    // Vector to store developers (name, years_of_experience)
    // Each item is a tuple: (String, f32)
    let mut developers: Vec<(String, f32)> = Vec::new();

    // Ask how many developers will be interviewed
    let total = ask_float("How many developers are being interviewed?") as usize;

    // Loop to collect developer info
    for _ in 0..total {
        let name = ask_string("Enter developer name:");
        let years = ask_float("Enter years of programming experience:");

        // Push the tuple into the vector
        developers.push((name, years));
    }

    // Check if vector is empty just in case
    if developers.is_empty() {
        println!("No developers entered.");
        return;
    }

    // Find the developer with the highest experience
    // We start by assuming the first person is the max
    let mut highest = &developers[0];

    for dev in &developers {
        if dev.1 > highest.1 {
            highest = dev;
        }
    }

    // Print result
    println!(
        "\nThe most experienced developer is {} with {} years of programming experience.",
        highest.0, highest.1
    );
}
