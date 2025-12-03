fn main() {
    let v = vec![10, 20, 30];
    //vector v owns the object in heap

    let v2 = &v;
    //transferred ownership

    display(v2);

    println!("In main {:?}", v2);
    //v2 can't be used anymore, function has used and dropped it
}

fn display(v:&Vec<i32>) {
    println!("Inside display {:?}", v);
}