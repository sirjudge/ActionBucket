use crate::game::fighter::FighterStats;
use std::error::Error;
use std::fs;
use std::io;
use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;


pub const FIGHTER_TOML_STRING: &str = r#"
[action_bucket1]
character_id = 1
base_damage = 10
base_weight = 100
base_speed_multiplier = 4
lives_remaining = 0
health_percent = 0

[action_bucket2]
character_id = 2
base_damage = 20
base_weight = 200
lives_remaining = 0
health_percent = 0
base_speed_multiplier = 3

[action_bucket3]
character_id = 3
base_damage = 30
base_weight = 300
lives_remaining = 0
health_percent = 0
base_speed_multiplier = 2

[action_bucket4]
character_id = 4
base_damage = 40
base_weight = 400
lives_remaining = 0
health_percent = 0
base_speed_multiplier = 1

"#;

#[derive(Debug, serde::Deserialize,bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct FighterData {
    pub action_bucket1: FighterStats,
    pub action_bucket2: FighterStats,
    pub action_bucket3: FighterStats,
    pub action_bucket4: FighterStats,
}

pub fn load_fighter_data_to_string(asset_server: Res<AssetServer>) -> Result<String, io::Error> {
    //TODO: Bevy not letting me do this as easily as I thought. For some reason
    // there's no "bevy_thing.extract_text_from_file()
    // let toml_text: Handle<bevy = asset_server.load("data/fighter_list.toml");
    // let content = fs::read_to_string("data/fighter_list.toml");
    // match content {
    //     Ok(toml_string) => {
    //         Ok(toml_string)
    //     }
    //     Err(error) => {
    //         error!("Error occurred trying to extract fighter_list.toml:{:?}", error);
    //         Err(error)
    //     }
    // }
    //
    Ok(FIGHTER_TOML_STRING.to_string())
}

pub fn get_fighter_data_from_toml(toml_string: &str) -> Result<FighterData, Box<dyn Error>> {
    match toml::from_str::<FighterData>(toml_string) {
        Ok(fighter_data) => {
            Ok(fighter_data)
        }
        Err(error) => {
            error!("Error ocurred parsing given toml_string:{:?}", error);
            Err(Box::new(error))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hardcoded_toml_string_parses_successfully(){
        let fighter_data = get_fighter_data_from_toml(FIGHTER_TOML_STRING);
        match fighter_data {
            Ok(_) => (),
            Err(error) =>{
                panic!("Hardocded data should have parsed but did not:{:?}", error);
            }
        }
    }

    #[test]
    fn valid_toml_parses_successfully() {
        let toml_string = r#"
[action_bucket1]
character_id = 1
base_damage = 10
base_weight = 100
base_speed_multiplier = 4
lives_remaining = 0
health_percent = 0

[action_bucket2]
character_id = 2
base_damage = 20
base_weight = 200
lives_remaining = 0
health_percent = 0
base_speed_multiplier = 3

[action_bucket3]
character_id = 3
base_damage = 30
base_weight = 300
lives_remaining = 0
health_percent = 0
base_speed_multiplier = 2

[action_bucket4]
character_id = 4
base_damage = 40
base_weight = 400
lives_remaining = 0
health_percent = 0
base_speed_multiplier = 1
        "#;
        let fighter_data = get_fighter_data_from_toml(toml_string);
        match fighter_data {
            Ok(data) => {
                assert_eq!(data.action_bucket1.character_id, 1);
                assert_eq!(data.action_bucket1.base_damage, 10);
                assert_eq!(data.action_bucket1.base_weight, 100);
                assert_eq!(data.action_bucket1.base_speed_multiplier, 4);

                assert_eq!(data.action_bucket2.character_id, 2);
                assert_eq!(data.action_bucket2.base_damage, 20);
                assert_eq!(data.action_bucket2.base_weight, 200);
                assert_eq!(data.action_bucket2.base_speed_multiplier, 3);

                assert_eq!(data.action_bucket3.character_id, 3);
                assert_eq!(data.action_bucket3.base_damage, 30);
                assert_eq!(data.action_bucket3.base_weight, 300);
                assert_eq!(data.action_bucket3.base_speed_multiplier, 2);

                assert_eq!(data.action_bucket4.character_id, 4);
                assert_eq!(data.action_bucket4.base_damage, 40);
                assert_eq!(data.action_bucket4.base_weight, 400);
                assert_eq!(data.action_bucket4.base_speed_multiplier, 1);
            }
            Err(error) => {
                panic!("Error orcurred during fighter toml data parsing{:?}", error);
            }
        }
    }
}