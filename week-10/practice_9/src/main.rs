//defining dimensions of a rectangle
struct Rectangle {
    width:u32,
    height:u32
}

//logic to calc area of a rectangel
impl Rectangle {
    fn area(&self) ->u32 {
        //use the . operator to fetch the val of a field via the self keyword
        self.width * self.height
    }
}

fn main () {
    //instanatiate the struct
    let small = Rectangle {
        width:10,
        height:20
    };

    //print the area
    println!("Wdith is {} \n height is {} \n area is {}", small.width, small.height, small.area());
}