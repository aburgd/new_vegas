use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::Rng;
use std::collections::HashMap;

/// A player character is here
pub struct Courier {
    /// Couriers must have a name...
    pub name: String,
    /// a Map of SPECIAL attributes...
    pub special: HashMap<String, i8>,
    /// an array of skills...
    pub skills: Vec<String>,
    /// an array of traits...
    pub traits: Vec<String>,
    /// and last but not last, a preferred soda.
    pub drink_of_choice: String,
}

impl Courier {
    /// A SPECIAL Map has to be made for a new `Courier`.
    fn assign_special() -> HashMap<String, i8> {
        // Declare a HashMap<String, i8> to store player's special
        let mut player_special: HashMap<String, i8> = HashMap::new();
        // List the keys of SPECIAL as a Vector to iterate over later
        let special_keys: Vec<String> = vec![
            String::from("strength"),
            String::from("perception"),
            String::from("endurance"),
            String::from("charisma"),
            String::from("intelligence"),
            String::from("agility"),
            String::from("luck"),
        ];
        // Assign each key (attribute) a value from from 1 to 11 (excl)
        for key in special_keys {
            player_special.insert(key, rand::thread_rng().gen_range(1, 11));
        }
        // Check if the generated SPECIAL is valid (sum of all values is 40)
        let mut valid_special: bool = Courier::check_special(&player_special);
        // Check if special is valid
        if !valid_special {
            // If not, while not make a new one and check
            while !valid_special {
                player_special = Courier::assign_special();
                valid_special = Courier::check_special(&player_special);
            }
        }
        return player_special;
    }

    /// The logic for validating a the SPECIAL Map.
    /// # Arguments
    ///
    /// * `special` - a reference to the SPECIAL `HashMap<>` to be check
    fn check_special(special: &HashMap<String, i8>) -> bool {
        let special_max: i8 = 40;
        let mut special_sum: i8 = 0;
        for value in special.values() {
            special_sum += value;
        }
        return special_max == special_sum;
    }

    /// Despite what they may think, every `Courier` has a set of skills.
    fn tag_skills() -> Vec<String> {
        let available_skills: [&str; 9] = [
            "energy weapons",
            "melee weapons",
            "guns",
            "barter",
            "repair",
            "speech",
            "explosives",
            "unarmed",
            "medicine",
        ];
        let mut player_skills: Vec<String> = Vec::new();
        for _ in 0..2 {
            player_skills.push(String::from(
                available_skills[rand::thread_rng().gen_range(0, 9)],
            ));
        }
        player_skills.sort();
        player_skills.dedup();
        while player_skills.len() < 2 {
            player_skills.push(String::from(
                available_skills[rand::thread_rng().gen_range(0, 9)],
            ));
            player_skills.dedup();
        }
        return player_skills;
    }

    /// A `Courier`'s unique almalgamation of attributes lends themself to a unique set of traits.
    fn pick_traits() -> Vec<String> {
        let available_traits: [&str; 10] = [
            "built to destroy",
            "fast shot",
            "four eyes",
            "good natured",
            "heavy handed",
            "kamikaze",
            "loose cannon",
            "small frame",
            "trigger discipline",
            "wild wasteland",
        ];
        let mut player_traits: Vec<String> = Vec::new();
        for _ in 0..3 {
            player_traits.push(String::from(
                available_traits[rand::thread_rng().gen_range(0, 10)],
            ));
        }
        player_traits.sort();
        player_traits.dedup();
        while player_traits.len() < 3 {
            player_traits.push(String::from(
                available_traits[rand::thread_rng().gen_range(0, 10)],
            ));
            player_traits.dedup();
        }
        return player_traits;
    }

    /// Being a `Courier` is thirsty work. Pick one.
    fn pick_drink() -> String {
        let drinks: [&str; 3] = ["sunset sarsparilla", "nuka cola", "alcohol"];
        String::from(drinks[rand::thread_rng().gen_range(0, drinks.len())])
    }

    /// A `Courier` needs a name. How else can they get their caps?
    fn generate_name() -> String {
        Name(EN).fake()
    }

    /// Returns a new `Courier` for the Mojave Express. No arguments are needed - everything is random.
    pub fn new() -> Self {
        Self {
            name: Courier::generate_name(),
            special: Courier::assign_special(),
            skills: Courier::tag_skills(),
            traits: Courier::pick_traits(),
            drink_of_choice: Courier::pick_drink(),
        }
    }
}
