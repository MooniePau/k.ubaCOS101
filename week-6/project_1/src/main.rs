use std::io;

fn main() {

    loop {

        let p:f32 = 3_200.00;
        let f:f32 = 3_000.00;
        let a:f32 = 2_500.00;
        let e:f32 = 2_000.00;
        let w:f32 = 2_500.00;

        println!("WELCOME TO MO'S KITCHEN!");

        //Print menu

        println!("POUNDO YAM AND EDINKAIKO SOUP     CODE: P     PRICE: {}", p);
        println!("FRIED RICE AND CHICKEN    CODE: F     PRICE: {}", f);
        println!("AMALA AND EWEDU SOUP      CODE: A     PRICE: {}", a);
        println!("EBA AND EGUSI SOUP    CODE: E     PRICE: {}", e);
        println!("WHITE RICE AND STEW   CODE: W     PRICE: {}\n", w);

        println!("Please place your order below:\n\n");

        //ask for code
        println!("Food code? ");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Unable to read input");
        let food_code = input1.trim();

        //ask for quantity
        println!("Quantity? ");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Unable to read input");
        let quantity:f32 = input2.trim().parse().expect("Invalid number");

        //declare and empty quantity variable that fills 
        //with a new value if one of the statements is met
        let price:f32;

        //calculate for price (before discount)
        if food_code == "p" || food_code == "P" {
            price = p * quantity;
        }
        else if food_code == "f" || food_code == "F" {
            price = f * quantity;
        }

        //stopped here on 8th nov 2025
        //next to do is finish up the if and if else statements,
        //probably turn it into a switch statement for faster implementation

        else if food_code == "a" || food_code == "A" {
            price = a * quantity;
        }

        else if food_code == "e" || food_code == "E" {
            price = e * quantity;
        }

        else if food_code == "w" || food_code == "W" {
            price = w * quantity;
        }

        else {
            println!("You haven't entered a valid food code. PLease order again and enter one. (P/F/A/E/W)\n\n");
            continue;
        }

        let mut discount:f32 = 0.05;
        let mut price_after_discount:f32 = price;

        if price > 10_000.00 {
            price_after_discount = price - (price * 0.05);
        }
        else {
            discount = 0.0;
        }


        //then, implement an if statement that prints out a certain thing depending 
        //on the food code the customer enters

        if food_code == "p" || food_code == "P" {
            println!("You ordered: POUNDO YAM AND EDINKAIKO SOUP");
            println!("Price: N{}", p);
            println!("Quantity: {}", quantity);
            println!("Price before discount: {}", price);
            println!("Discount: {}", discount);
            println!("Price after discount: {}\n", price_after_discount);
        }

        else if food_code == "f" || food_code == "F" {
            println!("You ordered: FRIED RICE AND CHICKEN");
            println!("Price: N{}", f);
            println!("Quantity: {}", quantity);
            println!("Price before discount: N{}", price);
            println!("Discount: {}", discount);
            println!("Price after discount: {}\n", price_after_discount);
        }

        else if food_code == "a" || food_code == "A" {
            println!("You ordered: AMALA AND EWEDU SOUP");
            println!("Price: N{}", a);
            println!("Quantity: {}", quantity);
            println!("Price before discount: N{}", price);
            println!("Discount: {}", discount);
            println!("Price after discount: {}\n", price_after_discount);
        }

        else if food_code == "e" || food_code == "E" {
            println!("You ordered: EBA AND EGUSI SOUP");
            println!("Price: N{}", e);
            println!("Quantity: {}", quantity);
            println!("Price before discount: N{}", price);
            println!("Discount: {}", discount);
            println!("Price after discount: {}\n", price_after_discount);
        }

        else if food_code == "w" || food_code == "W" {
        println!("You ordered: WHITE RICE AND STEW");
        println!("Price: N{}", w);
        println!("Quantity: {}", quantity);
        println!("Price before discount: N{}", price);
        println!("Discount: {}", discount);
        println!("Price after discount: {}\n", price_after_discount);

        }

        println!("Would you like to order again? (yes/no), (y/n)");

        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Invalid string");
        let yes_no = input3.trim();

        //if yes; continue loop
        if yes_no == "yes" || yes_no == "YES" || yes_no == "y" || yes_no == "Y" {
            continue;
        }
        
        //if no,print below line and break loop
        else if yes_no == "no" || yes_no == "NO" || yes_no == "n" || yes_no == "N" {
            println!("Thanks for your patronage. Come again next time!");
            break;
        }

        else {
            println!("Please enter valid input (yes/no)");
            break;
        }
    }
}
