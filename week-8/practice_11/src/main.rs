fn main() {
    //an array of numbers
    let numbers = [1,2,3,4,5];
    println!("Original array = {:?}", numbers);

    //create a slice of 2nd and 3rd element
    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    //ommit the start index
    let slice2 = &numbers[..3];
    //the slice starts from index 0 to index 3 exclusive
    println!("index o to 3 sliced = {:?}", slice2);

    //omit the end index
    let slice3 = &numbers[2..];
    //the slice starts from index 2 up to 5 exclusive
    println!("Index 2 to 5 sliced = {:?}", slice3);

    //omit the start and end indices
    //reference the whole array
    let slice4 = &numbers[..];
    //the slice starts from index o to index 5 exclusive
    println!("Index 0 to 5 sliced = {:?}", slice4);
}
