mod courier;
use courier::Courier;
use std::collections::HashMap;

fn main() {
    println!("Mojave Express Courier Application 13-A");
    let player_courier: Courier = Courier::new();
    let player_name: String = player_courier.name;
    println!("\nname\n====\n{}", player_name);
    let player_special: HashMap<String, i8> = player_courier.special;
    println!("\nspecial\n=======");
    for attribute in player_special.keys() {
        println!("{}: {}", attribute, player_special.get(attribute).unwrap());
    }
    let player_skills: Vec<String> = player_courier.skills;
    println!("\nskills\n======");
    for skill in player_skills {
        println!("{}", skill);
    }
    let player_traits: Vec<String> = player_courier.traits;
    println!("\ntraits\n======");
    for item in player_traits {
        println!("{}", item);
    }
    let player_sun_nuka: String = player_courier.sunset_nuka;
    println!("\ndrink of choice\n===============");
    println!("{}", player_sun_nuka);
}
