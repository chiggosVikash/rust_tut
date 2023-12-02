mod person_struct;
mod resturent;

use person_struct::Person;
mod rectangle;
use rectangle::Rectangle;

use crate::resturent::front_of_house::hosting;


#[derive(PartialEq)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32,y:i32,z:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self){
        println!("Message is {:?}",self);
    }
}





fn main(){
    hosting::add_to_waitlist();
    let four: IpAddrKind = IpAddrKind::V4(127,0,0,1);
    let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));
    let message: Message = Message::Write(String::from("This is enum message"));
    let quit = Message::Quit;
    quit.call();
    let move_message = Message::Move{x:10,y:20,z:30};
    move_message.call();
    message.call();

    let change_color = Message::ChangeColor(102,244, 255);
    change_color.call();
    if four == six{
        println!("Both are equal");
    }else{
        println!("Both are not equal");
    }
    // struct implementation example
    let rect: Rectangle = Rectangle{
        width: 30,
        height: 50,
    
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    
    };
    let rect3 = Rectangle{
        width: 60,
        height: 45,
    
    };
    let sq: Rectangle = Rectangle::square(10);
    println!("Square function in {:?}",sq);
    println!("Can rect hold rect2? {}",rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    println!("Area of rectangle {} ",rect.area());

    println!("Perimeter of rectangle {} ",rect.perimeter());

   
    let s: String = String::from("hello world");

    let person: Person = build_person(String::from("Vikash Kumar"), String::from("2004-01-02"));

  
    println!("{} is {} years old, born on {}", person.name, person.age, person.date_of_birth);

    let person2 = Person{
        name: String::from("Vikash Kumar"),
        ..person
    };

    let person3 = Person::new(String::from("Vikash Kumar"), 19, String::from("2004-01-02"));

   

    println!("{} is {} years old, born on {}", person2.name, person2.age, person2.date_of_birth);
    


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

