use crate::game::fighter::Fighter;

pub struct Player {
    name: String,
    current_fighter: Fighter,
}

impl Player {
    pub fn change_name(&mut self,new_name: &str){
        self.name = new_name.to_string();
    }

    pub fn change_fighter(&mut self, new_fighter: Fighter){
        self.current_fighter = new_fighter;
    }
}
