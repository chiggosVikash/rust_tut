use std::fmt::Display;


pub struct  Tweet{
    pub message:String,
    pub date:String,
    pub tweet_by:String
}


impl Summary for Tweet {
    fn summarize(&self)->String {
        format!("@{} : {}",self.message,self.tweet_by)
    }
}


pub trait Summary {
    
    fn summarize(&self)->String;
}


pub fn notify(summay:&impl Summary){
    
}

pub fn notify_<T:Summary + Display>(s:&T){
    
}