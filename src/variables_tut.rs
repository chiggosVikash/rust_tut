use std::vec;



    pub fn variables_fn(){
        let x: i32 = 5;
       
        let l1: [i32; 5] = [1, 2, 3, 4, 5];
        let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
        println!("The value of x is: {}",x);
        // x = 6; // this will throw an error because x is immutable
        {
            let x: i32 = 6;
            println!("The value of x is: {}",x);
        }

        for i in &l1 {
            println!("The value of x is: {}",i);
        }
        let mut index:u32 = 0;
        let result = loop {
            let x = vec1[index as usize];
            println!("The value of x is: {}",x);
            index +=1;
            if index == vec1.len() as u32{
                break;
            }
        };
        println!("The value of x is: {:?}",result);

        println!("The value of x is: {}",x);

        
    }

