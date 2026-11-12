pub struct Fighter {
    health_percent: i16,
    lives_remaining: i16,
}

impl Fighter {
    fn new() -> Self {
        Fighter {
            health_percent: 0,
            lives_remaining: 3,
        }
    }

    fn set_heatlh(&mut self, health_percent: i16){
        if health_percent < 0 {
            self.health_percent = 0;
            return;
        }
    
        self.health_percent = health_percent;
    }

    fn increment_health_percent(&mut self, health_percent: i16) {
        self.health_percent += health_percent;
    }

    fn decrement_health_percent(&mut self, health_percent: i16) {
        if self.health_percent <= health_percent {
            self.health_percent = 0;
        }
        else {
            self.health_percent -= health_percent;
        }
    }
    
    fn decrement_lives(&mut self) {
        match self.lives_remaining {
            0 => {
                //Log that there are no more lives
                // send game over event
            },
            _ => self.lives_remaining -= 1,
        };
    }
    
    fn increment_lives(&mut self) {
        match self.lives_remaining {
            0 => {
                // Ressurection and move lives to 1?
                // Seems like edge case but should handle it anyways
            },
            _ => self.lives_remaining += 1,
        };
    }
}


#[cfg(test)]
mod test {
    use crate::game::fighter;
    use super::*;

    #[test]
    fn health_increments_correctly(){
        let mut fighter = create_fighter();
        assert!(fighter.health_percent == 0);
        fighter.increment_health_percent(10);
        assert!(fighter.health_percent == 10);
    }
    
    #[test]
    fn health_decrements_correctly(){
        let mut fighter = create_fighter();
        fighter.increment_health_percent(10);
        assert!(fighter.health_percent == 10);
        fighter.decrement_health_percent(5); 
        assert!(fighter.health_percent == 5);
        fighter.decrement_health_percent(6); 
        assert!(fighter.health_percent == 0);
    }

    fn create_fighter() -> Fighter {
        Fighter::new()
    }
}
