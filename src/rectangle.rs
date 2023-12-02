#[derive(Debug)]
pub struct Rectangle{
    pub width: u32,
    pub height: u32,
}


impl Rectangle{
    pub fn square(size:u32)->Self{
        Self{
            width:size,
            height:size
        }
    }
    pub fn area(&self)->u32{
        return self.width * self.height;
    }

    pub fn perimeter(&self)-> u32{
        return 2 * (self.width + self.height);
    }

    pub fn can_hold(&self,other:&Rectangle)-> bool{
        return self.width > other.width && self.height > other.height;
    }
}
