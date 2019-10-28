use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use rand::Rng;
use std::collections::HashMap;

pub struct Courier {
    pub name: String,
    pub special: HashMap<String, i8>,
    pub skills: Vec<String>,
    pub traits: Vec<String>,
    pub sunset_nuka: String,
}

impl Courier {
    fn assign_special() -> HashMap<String, i8> {
        // Declare a HashMap<String, i8> to store player's special
        let mut player_special: HashMap<String, i8> = HashMap::new();
        // List the keys of SPECIAL as a Vector to iterate over later
        let special_keys: Vec<String> = vec![
            String::from("special"),
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

    fn check_special(special: &HashMap<String, i8>) -> bool {
        let special_max: i8 = 40;
        let mut special_sum: i8 = 0;
        for value in special.values() {
            special_sum += value;
        }
        return special_max == special_sum;
    }

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

    fn sunset_or_nuka() -> String {
        let sunset_nuka: [&str; 2] = ["sunset sarsparilla", "nuka cola"];
        String::from(sunset_nuka[rand::thread_rng().gen_range(0, 2)])
    }

    fn generate_name() -> String {
        Name(EN).fake()
    }

    pub fn new() -> Self {
        Self {
            name: Courier::generate_name(),
            special: Courier::assign_special(),
            skills: Courier::tag_skills(),
            traits: Courier::pick_traits(),
            sunset_nuka: Courier::sunset_or_nuka(),
        }
    }
}
