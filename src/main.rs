fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("******the value of s1 : {s1}******** the value of s2 : {s2}*******");

    /*
    let  s1_address = s1.as_ptr();
    let  s2_address = s2.as_ptr();
    println!("******the value of s1_address : {s1_address}");
    println!("******the value of s2_address : {s2_address}");
     */
}
