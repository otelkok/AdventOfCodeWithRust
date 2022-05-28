pub mod part15_2{
    use std::collections::{BTreeMap, HashMap};
    use std::fmt::{Display, Formatter};
    use std::hash::Hash;
    use std::ops::Add;
    use crate::part15_1::part15_1::GameMap;
    use crate::part15_1::part15_1::Position;
    pub struct ExtendedRiskMap{
        pub(crate) map : BTreeMap<(u32, u32),Option<u64>>,
        pub(crate) game_map : ExtendedGameMap,
    }
    impl Display for ExtendedRiskMap{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut result = String::new();
            let (mut max_x, mut max_y) = (u32::MIN, u32::MIN);
            for ((i,j),value) in &self.map{
                if *i > max_x {
                    max_x = *i;
                }
                if *j > max_y {
                    max_y = *j;
                }
            }
            for i in 0..=max_x{
                for j in 0..=max_y{
                    result = result.add(&*format!("{}", self.game_map.game_map.get(&(i, j)).unwrap().clone()));
                }
                result.push('\n');
            }
            write!(f,"{}",result)
        }
    }
    impl ExtendedRiskMap{
        pub fn new(game_map : ExtendedGameMap) -> Self{
            let mut risk_map = BTreeMap::new();
            for (key,value) in &game_map.game_map{
                risk_map.insert(key.clone(),None);
            }
            ExtendedRiskMap{map: risk_map,game_map}
        }
        pub fn evaluate_position(&mut self, position : (u32,u32)) -> u64{
            let risk = self.map.get(&position).unwrap();
            let positional_risk = *self.game_map.game_map.get(&position).unwrap() as u64;
            match risk{
                Some(content) => *content,
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
                                Some(content_2) if content_2 < content =>content_2 + positional_risk,
                                _ => {
                                    content + positional_risk
                                },
                            }
                        }
                        None => {
                            match risk_from_right{
                                Some(content) => {
                                    content + positional_risk
                                } ,
                                None => {
                                    positional_risk
                                },
                            }
                        }
                    };
                    self.map.insert(position,Some(risk));
                    risk
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ExtendedGameMap{
        pub(crate) game_map : BTreeMap<(u32, u32),u8>,
        initial_x_border : u16,
        initial_y_border : u16,
    }
    impl ExtendedGameMap{
        pub fn new(from : GameMap) -> Self{
            let mut extended_map = BTreeMap::new();
            for ((i,j),value) in from.map{
                extended_map.insert((i as u32, j as u32),value);
            }
            let new_y_border = (from.y_max + 1) * 5 - 1;
            let new_x_border = (from.x_max + 1) * 5 - 1;
            for j in 0..=new_y_border{
                for i in 0..=new_x_border{
                    ExtendedGameMap::get_value_from_narrower_map(&mut extended_map,(i as u32,j as u32),from.y_max,from.x_max);
                }
            }
        ExtendedGameMap{game_map : extended_map,initial_x_border : from.x_max, initial_y_border : from.y_max}
        }
        fn get_value_from_narrower_map(extended_map : &mut BTreeMap<(u32,u32),u8>, position : (u32, u32), initial_y_border : u16, initial_x_border : u16) -> u8{
            match extended_map.get(&position){
                Some(content) => *content,
                None => {
                    match position.0{
                        x if x > initial_x_border as u32 => {
                            let result = ExtendedGameMap::get_value_from_narrower_map(extended_map,(position.0 - initial_x_border as u32 - 1,position.1),initial_y_border,initial_x_border);
                            let derived_value = match result {
                                x if x == 9 => 1,
                                x => x + 1,
                            };
                            extended_map.insert(position,derived_value);
                            derived_value
                        }
                        _ => {
                            match position.1{
                                y if y > initial_y_border as u32 => {
                                    let result =ExtendedGameMap::get_value_from_narrower_map(extended_map,(position.0,position.1-initial_y_border as u32 - 1),initial_y_border,initial_x_border);
                                    let derived_value = match result {
                                        x if x == 9 => 1,
                                        x => x + 1,
                                    };
                                    extended_map.insert(position,derived_value);
                                    derived_value
                                }
                                _ => {
                                    panic!("Unexpected case, position:{:?} should have been existed",position);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}