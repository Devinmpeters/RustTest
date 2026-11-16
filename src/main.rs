use rand::Rng;

fn main() {
    //diceroll();

    player_attack();
}

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

fn player_attack() {
    let attack_total = attack_roll();
    println!("Does a {} hit?", attack_total);
}

fn attack_roll() -> i32 {
    let num = rand::thread_rng().gen_range(1..20);
    let attk_mod = 6; 
    let attack_total: i32 = num + attk_mod;       
    

    return attack_total;
}