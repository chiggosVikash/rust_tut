
pub fn borrowing(){
    let name = String::from("Vikash");
    let name2: &String = &name;

    println!("{}",name2);

}