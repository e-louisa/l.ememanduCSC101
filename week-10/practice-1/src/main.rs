fn main() {

    let v = vec![101, 250, 330, 400];
    //vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let v2 = v.clone();
    //here 2 variables owns the heap value
    //two pointers to the same content is not allowed in rust

    //rust is smart in terms of memonery access, so it detects a race condition
    //as 2 variables point to same heap

    println!("{:?}",v);

}
