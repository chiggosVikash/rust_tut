mod person_struct;
mod resturent;
mod tranit_exm;
use std::{fs::{File, Permissions}, io::ErrorKind};

mod enums_example;

fn main(){
    let test = enums_example::BasicDetails::V4;
    let sec_test = enums_example::BasicDetails::V6;
    println!("current enum status is  {:?} {:?}",test,sec_test);
}

// use crate::tranit_exm::{Tweet, Summary};
// // use person_struct::Person;


// // #[derive(PartialEq)]
// // enum IpAddrKind {
// //     V4(u8,u8,u8,u8),
// //     V6(String),
// // }

// // #[derive(Debug)]
// // enum Message{
// //     Quit,
// //     Move{x:i32,y:i32,z:i32},
// //     Write(String),
// //     ChangeColor(i32,i32,i32),
// // }

// // impl Message {
// //     fn call(&self){
// //         println!("Message is {:?}",self);
// //     }
// // }





// fn main(){

//     // error handling
//     // let v = vec![1,2,3,4,5];
    
//     // error handling with match
//     // let file_result = File::open("hello.txt");
//     // let result = match file_result {
//     //     Ok(file) => {
//     //          file
//     //     },
//     //     Err(err) => {
//     //         match err.kind() {
//     //             ErrorKind::NotFound => {
//     //                 match File::create("hello.txt") {
//     //                     Ok(fs) => fs,
//     //                     Err(err) => panic!("Failed to create file {:?}",err),
//     //                 }
//     //             }other_error=>{
//     //                 panic!("Failed to read file {:?}",other_error);
//     //             }
                
//     //         }
//     //     },
//     // };

//     // let metadata = result.metadata();

//     // match metadata {
//     //     Ok(data) => {
//     //         println!("metadata permission {:?}",data.permissions());
//     //     },
//     //     Err(err) => {
//     //         panic!("Meta data error {:?}",err);
//     //     },
//     // }

//     //ERROR HANDLING WITH UNWRAP OR ELSE

//     // let read_file = File::open("hello.txt").unwrap_or_else(|error|{
//     //     if error.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|error|{
//     //             panic!("Failed to open file {:?}",error);
//     //         })
           
//     //     }else{
//     //         panic!("Faile to read file {:?}",error);
//     //     }
//     // });

//     // let metadata = read_file.metadata().unwrap_or_else(|error|{
//     //     panic!("failed to read metadata {:?}",error)
//     // });

//     // metadata.permissions().set_readonly(true);
    

//     // // hasmap
//     // let mut scores: HashMap<&str, i32> = HashMap::new();
//     // scores.insert("Blue",10);
//     // scores.insert("Yellow",50);
//     // scores.insert("Red",100);

//     // scores.iter().for_each(|(key,value)|{
//     //     println!("{}: {}",key,value);
//     // });
    


//     // hosting::add_to_waitlist();
//     // let four: IpAddrKind = IpAddrKind::V4(127,0,0,1);
//     // let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));
//     // let message: Message = Message::Write(String::from("This is enum message"));
//     // let quit = Message::Quit;
//     // quit.call();
//     // let move_message = Message::Move{x:10,y:20,z:30};
//     // move_message.call();
//     // message.call();

//     // let change_color = Message::ChangeColor(102,244, 255);
//     // change_color.call();
//     // if four == six{
//     //     println!("Both are equal");
//     // }else{
//     //     println!("Both are not equal");
//     // }
//     // // struct implementation example
//     // let rect: Rectangle = Rectangle{
//     //     width: 30,
//     //     height: 50,
    
//     // };
//     // let rect2 = Rectangle{
//     //     width: 10,
//     //     height: 40,
    
//     // };
//     // let rect3 = Rectangle{
//     //     width: 60,
//     //     height: 45,
    
//     // };
//     // let sq: Rectangle = Rectangle::square(10);
//     // println!("Square function in {:?}",sq);
//     // println!("Can rect hold rect2? {}",rect.can_hold(&rect2));
//     // println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
//     // println!("Area of rectangle {} ",rect.area());

//     // println!("Perimeter of rectangle {} ",rect.perimeter());

   
//     // let s: String = String::from("hello world");

//     // let person: Person = build_person(String::from("Vikash Kumar"), String::from("2004-01-02"));

  
//     // println!("{} is {} years old, born on {}", person.name, person.age, person.date_of_birth);

//     // let person2 = Person{
//     //     name: String::from("Vikash Kumar"),
//     //     ..person
//     // };

//     // let _person3 = Person::new(String::from("Vikash Kumar"), 19, String::from("2004-01-02"));

   

//     // println!("{} is {} years old, born on {}", person2.name, person2.age, person2.date_of_birth);
    


//     // let sentence = &s;
//     // println!("{}",sentence);    

//     // Generic type
//     let char_list = vec!['a','v','x','q','i'];
//     println!("Largest char {}",get_largest(&char_list));
//     println!("Largest num {}",get_largest(&[1,44,5,3,56,4,22,444,908]));
    
//         let tweet = Tweet{
//             tweet_by: String::from("Vikash Kumar"),
//             message: String::from("This is tweet message"),
//             date:String::from("today")
//         };

//         println!("{}",tweet.summarize());
// }

// fn get_largest<T:std::cmp::PartialOrd>(elements:&[T])->&T{
//     if elements.is_empty() {
//         panic!("list can't be empty");
//     }

//     let mut largest = &elements[0];

//     for element in elements{
//         if element > largest{
//             largest = element;
//         }
//     }

//     return largest;

    

// }
// // fn build_person(name: String, date_of_birth: String) -> Person{
// //     return Person{
// //         name,
// //         age:19,
// //         date_of_birth
// //     }
// // }



