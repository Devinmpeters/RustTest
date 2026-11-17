
use rand::Rng; //using rand dependency 0.8
use std::io; //using std::io dependency

//Main function to run the program
fn main() {
    println!("Welcome to the Dungeons and Dragons Attack Roll Simulator!"); //print welcome message
    println!("Please enter your attack modifier: "); //prompt user for attack modifier

    let mut input: String = String::new(); //create variable to hold user input
    io::stdin().read_line(&mut input).expect("Failed to read line"); //read user input

    println!("Press enter to roll a d20 and add your attack modifier of {} to see if you hit your target!", input); //prompt user to press enter

    player_attack(input.parse().unwrap_or(0)); //call player_attack function
}

//Return random number between 1 and 20 to simulate a d20 roll
fn diceroll() -> i32 {
    let num = rand::thread_rng().gen_range(1..20);
    return num;
}

//Function to calculate the total attack roll and print it
fn player_attack(attk_mod: i32) {
    let attack_total: i32 = attack_roll(attk_mod); //call attack_roll function to get total attack roll
    println!("Does a(n) {} hit?", attack_total);
}

//Function to calculate the total attack roll by adding the d20 roll and the attack modifier
fn attack_roll(attk_mod: i32) -> i32 {
    let num = diceroll(); //call diceroll function to get random number between 1 and 20
    let attack_total: i32 = num + attk_mod; //add the d20 roll and the attack modifier to get the total attack roll      

    return attack_total;
}
