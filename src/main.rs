mod person_struct;
use person_struct::Person;
fn main(){
   
    let s = String::from("hello world");

    let person = build_person(String::from("Vikash Kumar"), String::from("2004-01-02"));

    println!("{} is {} years old, born on {}", person.name, person.age, person.date_of_birth);


    let sentence = &s;
    println!("{}",sentence);    

}

fn build_person(name: String, date_of_birth: String) -> Person{
    return Person{
        name,
        age:19,
        date_of_birth
    }
}

