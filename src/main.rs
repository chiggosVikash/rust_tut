// use std::io;

fn main() {
    let mut num:i8 = 100;

    println!("Signed interger of 8bit {}", num);

    let tup = (12,2.5,"Vikash");
    println!("Tuple is {:?}", tup);

   let loop_result = loop{
        println!("Value of num is {}", num);
        if num == 0 {
            break num;
        }
        num = num - 1;
    };

    println!("Loop result is {}", loop_result);

    

}