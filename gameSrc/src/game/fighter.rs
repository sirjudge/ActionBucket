use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub enum PlayerType {
    Human,
    CPU,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FighterStats {
    pub character_id: i16,
    pub base_damage: i8,
    //NOTE: Might bump this down to i8 in the future
    pub base_weight: i16,
    pub base_speed_multiplier: i8,
    pub health_percent: i16,
    pub lives_remaining: i16,
}

#[derive(Component, Clone, Debug)]
pub struct Fighter {
    pub player_type: PlayerType,
    pub fighter_stats: FighterStats,
}

//NOTE: Using this default for now to get everything working but need to change
// to the player's selected character from the lobby menu
impl Default for Fighter {
    fn default() -> Self {
        Fighter {
            player_type: PlayerType::Human,
            fighter_stats: FighterStats {
                health_percent: 0,
                lives_remaining: 3,
                base_damage: 10,
                base_weight: 100,
                base_speed_multiplier: 1,
                character_id: 30
            },
        }
    }
}

impl Fighter {
    fn set_heatlh(&mut self, health_percent: i16) {
        if health_percent < 0 {
            self.fighter_stats.health_percent = 0;
            return;
        }

        self.fighter_stats.health_percent = health_percent;
    }

    fn increment_health_percent(&mut self, health_percent: i16) {
        self.fighter_stats.health_percent += health_percent;
    }

    fn decrement_health_percent(&mut self, health_percent: i16) {
        if self.fighter_stats.health_percent <= health_percent {
            self.fighter_stats.health_percent = 0;
        } else {
            self.fighter_stats.health_percent -= health_percent;
        }
    }

    fn decrement_lives(&mut self) {
        match self.fighter_stats.lives_remaining {
            0 => {
                //Log that there are no more lives
                // send game over event
            }
            _ => self.fighter_stats.lives_remaining -= 1,
        };
    }

    fn increment_lives(&mut self) {
        match self.fighter_stats.lives_remaining {
            0 => {
                // Ressurection and move lives to 1?
                // Seems like edge case but should handle it anyways
            }
            _ => self.fighter_stats.lives_remaining += 1,
        };
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn health_increments_correctly() {
        let mut fighter = create_fighter();
        assert!(fighter.fighter_stats.health_percent == 0);
        fighter.increment_health_percent(10);
        assert!(fighter.fighter_stats.health_percent == 10);
    }

    #[test]
    fn health_decrements_correctly() {
        let mut fighter = create_fighter();
        fighter.increment_health_percent(10);
        assert!(fighter.fighter_stats.health_percent == 10);
        fighter.decrement_health_percent(5);
        assert!(fighter.fighter_stats.health_percent == 5);
        fighter.decrement_health_percent(6);
        assert!(fighter.fighter_stats.health_percent == 0);
    }

    fn create_fighter() -> Fighter {
        Fighter::default()
    }
}
