fn main() {
    let v = vec![20, 40, 60, 80];

    let v2 = &v; //transferred ownership, then fixed by borrowing
    let v2_return = display(&v2); //passed v2 into display which returns 
    //it into v2_return
    println!("In main {:?}", v);
}

fn display(v:&Vec<i32>) -> &Vec<i32> {
    //returning the same vector
    println!("Inside display {:?}", v);
    return v;
}