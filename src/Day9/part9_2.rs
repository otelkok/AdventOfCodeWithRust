pub mod part9_2{
    use std::collections::HashMap;
    use crate::part9_1::part9_1::AdjacentLocation;
    pub fn read_input_9_as_map(mut lines : Vec<String>) -> HeightMap{
        let mut cursor_1 : Vec<char> = Vec::new();
        let mut cursor_2 : Vec<char> = Vec::new();
        let mut cursor_3 : Vec<char> = lines[0].chars().collect();
        let mut up : (Position,u8);
        let mut down: (Position,u8);
        let mut left : (Position,u8);
        let mut right : (Position,u8);
        let mut position : (Position,u8);
        let mut result = HeightMap::new();
        for j in 1..=lines.len(){
            cursor_1 = cursor_2;
            cursor_2 = cursor_3;
            cursor_3 = match lines.get(j){
                Some(element) => element.chars().collect(),
                None => Vec::new(),
            };
            for i in 0..cursor_2.len(){
                left = match i.checked_sub(1){
                    Some(element) => (Position::new((i-1,j-1)),cursor_2[element].to_digit(10).unwrap() as u8),
                    None => (Position::new((usize::MAX,j)),u8::MAX),
                };
                up = match cursor_1.get(i){
                    Some(element) =>  (Position::new((i,j-2)),element.to_digit(10).unwrap() as u8),
                    None => (Position::new((i,usize::MAX)),u8::MAX),
                };
                right = match cursor_2.get(i+1){
                    Some(element) => (Position::new((i+1,j-1)),element.to_digit(10).unwrap() as u8),
                    None => (Position::new((usize::MAX,j)),u8::MAX),
                };
                down = match cursor_3.get(i){
                    Some(element) => (Position::new((i,j)),element.to_digit(10).unwrap() as u8),
                    None => (Position::new((usize::MAX,j)),u8::MAX),
                };
                position = (Position::new((i,j-1)),cursor_2[i].to_digit(10).unwrap() as u8);

                let new_location = AdjacentApparentLocation::new(position,[up,down,left,right]);
                result.insert(Position::new((i,j-1)),new_location);
            }

        }
        result
    }
    pub struct HeightMap{
        pub map : HashMap<Position,AdjacentApparentLocation>
    }
    #[derive(Eq,PartialEq,Hash,Debug,Clone)]
    pub struct Position{
        position : (usize,usize),
    }

    #[derive(Debug)]
    pub struct Basin{
        pub content : HashMap<Position,AdjacentApparentLocation>
    }
    #[derive(Debug,Eq,PartialEq,Hash,Clone)]
    pub struct AdjacentApparentLocation{
        height : (Position,u8),
        adjacent_locations : [(Position,u8); 4],
    }
    impl AdjacentApparentLocation{
        pub fn new(height : (Position,u8), adjacent_locations : [(Position,u8);4]) -> Self{
            AdjacentApparentLocation{height, adjacent_locations}
        }
        pub fn is_low_point(&self) -> bool{
            let mut result = true;
            for (position,adjacent) in &self.adjacent_locations{
                if position.position.0 < usize::MAX && position.position.1 < usize::MAX{
                    result = match adjacent <= &self.height.1 {
                        true => false,
                        false =>  result,
                    }
                }
            }
            result
        }
    }
    impl HeightMap{
        pub fn new() -> Self{
            HeightMap{map : HashMap::new()}
        }
        pub fn insert(&mut self,key : Position, value : AdjacentApparentLocation){
            self.map.insert(key,value);
        }
        pub fn low_points(&self) -> Vec<Position>{
            let mut result = Vec::new();
            for (key,value) in &self.map{
                if value.is_low_point(){
                    result.push((*key).clone());
                }
            }
            result
        }
    }
    impl Position {
        pub fn new(position : (usize,usize)) -> Self{
            Position{position}
        }
    }
    impl Basin {
        pub fn new() -> Self{
           Basin{content : HashMap::new()}
        }
        fn insert(&mut self, key : Position, value : AdjacentApparentLocation){
            self.content.insert(key,value);
        }
        pub fn from_low_point(low_points : Vec<Position>,map : &HeightMap) -> Vec<Self>{
            let mut result = Vec::new();
            for position in low_points{
                let mut to_insert = Basin::new();
                to_insert.from_low_point_recursive(position,map,);
                result.push(to_insert);
            }
            result
        }
        fn from_low_point_recursive(&mut self,low_point : Position, map: &HeightMap) {
            let start_point = map.map.get(&low_point).unwrap();
            let current = map.map.get(&low_point).unwrap();
            if current.height.1 < 9 && !self.content.contains_key(&low_point){
                self.content.insert(low_point,(*current).clone());
                for (adjacent,height) in &start_point.adjacent_locations{
                    if adjacent.position.0 < usize::MAX && adjacent.position.1 < usize::MAX && *height < 9{
                        self.from_low_point_recursive(adjacent.clone(), map);
                    }
                }
            }

        }
    }
}