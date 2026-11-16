use rand::Rng; //using rand dependency 0.8

fn main() {
    //diceroll();

    player_attack(); //call player_attack function
}

//Displays a random number between 1 and 20
//to represent a d20 dice roll
fn diceroll() {
    let num = rand::thread_rng().gen_range(1..20);

    if num == 20 {
        println!("You rolled a nat 20, Critical hit!");
    } else if num == 1 {
        println!("You rolled a nat 1, Critical fail!");
    } else {
        println!("You rolled a(n) {}", num);
    }
}

//Displays the result of the attack_roll function
fn player_attack() {
    let attack_total = attack_roll();
    println!("Does a {} hit?", attack_total);
}

//Returns the player's attack roll as an integer. 
//A random number between 1 and 20.
fn attack_roll() -> i32 {
    let num = rand::thread_rng().gen_range(1..20);
    let attk_mod = 6; 
    let attack_total: i32 = num + attk_mod;       
    

    return attack_total;
}