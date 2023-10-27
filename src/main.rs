use std::io;

fn main() {
    // println!("Hello, world!");
    // let tuple_x:(i32,f32,bool) = (12,4.0,false);
    // println!("type {}",tuple_x.1);
    // let mut _num = 10;
    // let name = "Vikash Kumar";
    // let length = name.len();
    // println!("Hello this is no: {_num}");
    // _num = 23;
    // println!("Length of name {name} is :{length}");

    // println!("value after assigned :{_num}");

    // take user input
    // 
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("User entered {}",input);

    let add_result = add_two_number(10, 32);
    println!("Added value is {}",add_result);
    
    let sub_result:i32 = sub_two_number(10, 32);
    let mult_result:i32 = mult_two_number(10, 32);
    let div_result:i32 = div_two_number(10, 32);

    println!("Multiplication value is {}",mult_result);
    println!("Division value is {}",div_result);
    println!("Subtraction value is {}",sub_result);
    }

    fn add_two_number(a:i32,b:i32)->i32{
        return a+b;
    }

    fn mult_two_number(a:i32,b:i32)->i32{
        return a*b;
    }


    fn div_two_number(a:i32,b:i32)->i32{
        return a/b;
    }

    // This is fuction to subtract two number and return result
    fn sub_two_number(a:i32,b:i32)->i32{
        return a-b;
    }

    
