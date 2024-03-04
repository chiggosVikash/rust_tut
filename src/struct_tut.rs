
struct Player{
    name: String,
    health: u64,
    attack: u64,
    defense: u64,
}

impl Player{

    fn new(name:&str, health:u64, attack:u64, defense:u64) -> Player{
        Player{
            name: name.to_string(),
            health,
            attack,
            defense,
        }
    }

    fn reduce_health(&mut self, deducable_no:u64){
        self.health -= deducable_no;
    }
}


pub fn struct_tut(){
    let mut player1 = Player::new("Vikash Kumar",100,10,5);
    // let player2 = Player::new("Rahul Kumar",100,10,6);

     player1.reduce_health(10);

    println!("{} has {} health left",player1.name,player1.health);
}