// use std::io;

fn main() {
   
    // let mut input = String::new();

    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // println!("User entered {}",input);

    let add_result = add_two_number(10, 32);
    println!("Added value is {}",add_result);
    
    let sub_result:i32 = sub_two_number(10, 32);
    let mult_result:i32 = mult_two_number(10, 32);
    let div_result:i32 = div_two_number(10, 32);

    println!("Multiplication value is {}",mult_result);
    println!("Division value is {}",div_result);
    println!("Subtraction value is {}",sub_result);

    const PI:f32 = 3.14;
    let area_of_circle_result = area_of_circle(10.0,PI);
    println!("Area of circle is {}",area_of_circle_result);

    shadowing();
    }

    fn area_of_circle(radius:f32,pi:f32)-> f32{
        return pi*radius*radius;
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


    /// shadowing
    /// Shadowing is different from marking a variable as mut, because we’ll get a compile-time 
    /// error if we accidentally try to reassign to this variable without using the let keyword.
    ///  By using let, we can perform a few transformations on a value but have the variable be
    ///  immutable after those transformations have been completed.
    fn shadowing(){
       let x:i32 = 10;
       let x:i32 = x + 12;

       {
            let x:i32 = x + 12;
             println!("Value of x is in inner scopre is {}",x);
       }
       println!("value of x in outer scope is {}",x);

    }

   

    
