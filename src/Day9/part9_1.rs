#[path = "part9_2.rs"] mod part9_2;
use part9_2::part9_2::Position;
pub mod part9_1{
    use crate::part9_2::part9_2::Position;

    pub fn read_input_9(mut lines : Vec<String>) -> Vec<AdjacentLocation>{
        let mut cursor_1 : Vec<char> = Vec::new();
        let mut cursor_2 : Vec<char> = Vec::new();
        let mut cursor_3 : Vec<char> = lines[0].chars().collect();
        let mut result = Vec::new();
        let mut up : u8;
        let mut down: u8;
        let mut left : u8;
        let mut right : u8;
        let mut position : u8;
        for i in 1..=lines.len(){
            cursor_1 = cursor_2;
            cursor_2 = cursor_3;
            cursor_3 = match lines.get(i){
                Some(element) => element.chars().collect(),
                None => Vec::new(),
            };
            for y in 0..cursor_2.len(){
                left = match y.checked_sub(1){
                    Some(element) => cursor_2[element].to_digit(10).unwrap() as u8,
                    None => u8::MAX,
                };
                up = match cursor_1.get(y){
                    Some(element) =>  element.to_digit(10).unwrap() as u8,
                    None => u8::MAX,
                };
                right = match cursor_2.get(y+1){
                    Some(element) => element.to_digit(10).unwrap() as u8,
                    None => u8::MAX,
                };
                down = match cursor_3.get(y){
                    Some(element) => element.to_digit(10).unwrap() as u8,
                    None => u8::MAX,
                };
                position = cursor_2[y].to_digit(10).unwrap() as u8;

                let new_location = AdjacentLocation::new(position.clone(),[up.clone(),down.clone(),left.clone(),right.clone()]);
                result.push(new_location);
            }

        }
        result
    }
    pub fn low_points(points : Vec<AdjacentLocation>) -> Vec<AdjacentLocation>{
        let mut result = Vec::new();
        for point in points{
            if point.is_low_point(){
                result.push(point);
            }
        }
        result
    }
    pub fn risk_points(low_points : Vec<AdjacentLocation>) -> u64{
        let mut sum = 0;
        for low_point in low_points{
            sum += low_point.height as u64 + 1;
        }
        sum
    }
    #[derive(Debug)]
    pub struct AdjacentLocation{
        pub height : u8,
        pub adjacent_heights : [u8;4],
    }
    impl AdjacentLocation{
        pub fn new(height : u8, adjacent_heights : [u8;4]) -> Self{
            AdjacentLocation{height, adjacent_heights}
        }
        pub fn is_low_point(&self) -> bool{
            let mut result = true;
            for adjacent in self.adjacent_heights{
                result = match adjacent <= self.height{
                    true => false,
                    false =>  result,
                }
            }
            result
        }
    }
}