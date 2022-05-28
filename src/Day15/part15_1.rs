pub mod part15_1{
    use std::collections::HashMap;
    use itertools::min;

    pub struct Position {
        position : (u16,u16),
        risk_value : Option<u32>,
    }
    impl Position{
        pub fn new(position : (u16,u16)) -> Self{
            Position{position,risk_value : Option::None}
        }
    }
    pub struct RiskMap{
        map : HashMap<(u16,u16),Option<u32>>,
        game_map : GameMap,
    }
    impl RiskMap{
        pub fn new(game_map : GameMap) -> Self{
            let mut risk_map = HashMap::new();
            for (key,value) in &game_map.map{
                risk_map.insert(key.clone(),None);
            }
            RiskMap{map: risk_map,game_map}
        }
        pub fn evaluate_position(&mut self, position : (u16,u16)) -> u32{
            let risk = self.map.get(&position).unwrap();
            let positional_risk = *self.game_map.map.get(&position).unwrap() as u32;
            match risk{
                Some(content) => {
                    *content
                }
                None => {
                    let down_position =  (position.0,position.1 + 1 );
                    let right_position = (position.0 + 1, position.1);
                    let risk_form_down = match self.map.contains_key(&down_position){
                        true => Some(self.evaluate_position(down_position)),
                        false => None,
                    };
                    let risk_from_right = match self.map.contains_key(&right_position){
                        true => Some(self.evaluate_position(right_position)),
                        false => None,
                    };
                    let risk = match risk_form_down{
                        Some(content) => {
                            match risk_from_right{
                                Some(content_2) if content_2 < content => content_2 + positional_risk,
                                _ => content + positional_risk,
                            }
                        }
                        None => {
                            match risk_from_right{
                                Some(content) => content + positional_risk ,
                                None => *self.game_map.map.get(&position).unwrap() as u32,
                            }
                        }
                    };
                    self.map.insert(position,Some(risk));
                    risk
                }
            }
        }
    }
    pub struct GameMap{
        pub(crate) map : HashMap<(u16, u16),u8>,
        pub(crate) x_max : u16,
        pub(crate) y_max : u16,
    }
    impl GameMap{
        pub fn new(lines : Vec<String>) -> Self{
            let mut map = HashMap::new();
            for j in 0..lines.len(){
                let current_string : Vec<char> = lines.get(j).unwrap().chars().collect();
                for i in 0..current_string.len(){
                    map.insert((i as u16,j as u16),current_string.get(i).unwrap().to_digit(10).unwrap() as u8);
                }
            }
            GameMap{map,y_max : lines.len() as u16 - 1, x_max : lines[0].len() as u16 -1}
        }
    }
}