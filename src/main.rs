use rand::Rng; //using rand dependency 0.8

fn main() {
    player_attack(); //call player_attack function
}

//Return random number between 1 and 20 to simulate a d20 roll
fn diceroll() -> i32 {
    let num = rand::thread_rng().gen_range(1..20);
    return num;
}

//Function to calculate the total attack roll and print it
//Function to calculate the total attack roll and print it
fn player_attack() {
    let attack_total = attack_roll(attk_mod: 6); //call attack_roll function to get total attack roll, pasing in the attack modifier of 6
    println!("Does a(n) {} hit?", attack_total);
}

//Function to calculate the total attack roll by adding the d20 roll and the attack modifier
fn attack_roll(attk_mod: i32) -> i32 {
    let num = diceroll(); //call diceroll function to get random number between 1 and 20
    let attack_total: i32 = num + attk_mod; //add the d20 roll and the attack modifier to get the total attack roll      

    return attack_total;
}
