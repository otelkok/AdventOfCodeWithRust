#[path = "part11_2.rs"] mod part11_2;
pub mod part11_1{
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, HashMap};
    pub fn read_input_11(lines : Vec<String>) -> OctopusMap{
        let mut result = OctopusMap::new();
        for (j,line) in lines.iter().enumerate(){
            for (i,character) in line.chars().enumerate(){
                result.insert(Position::new((i,j)),Octopus::new(character.to_digit(10).unwrap() as u8));
            }
        }
        for i in 0..10 {
            for j in 0..10 {
                let corresponding_position = Position::new((j,i));
            }
        }

        result
    }
    #[derive(Debug,Clone)]
    pub struct Octopus {
        energy: u8,
    }
    #[derive(Eq,PartialEq,Hash,Debug,Clone)]
    pub struct Position{
        position: (u8,u8),
    }
    pub struct OctopusMap{
        pub map : BTreeMap<Position,Octopus>,
        pub flash_count : u64,
        pub first_synchronization : u32,
    }
    impl Octopus{
        pub fn new(energy : u8) -> Self{
            Octopus{energy}
        }
        pub fn is_flashing(&self) -> bool{
            self.energy > 9
        }
    }
    impl Position{
        pub fn new(position : (usize,usize)) -> Self{
            Position{position : (position.0 as u8,position.1 as u8)}
        }
    }

    impl PartialOrd<Self> for Position {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.position.1{
                x if x > other.position.1 => Some(Ordering::Greater),
                x if x == other.position.1 => {
                    match self.position.0{
                        y if y > other.position.0 => Some(Ordering::Greater),
                        y if y == other.position.0 => Some(Ordering::Equal),
                        _ => Some(Ordering::Less),

                    }
                }
                _ => {Some(Ordering::Less)}
            }
        }
    }

    impl Ord for Position{
        fn cmp(&self, other: &Self) -> Ordering {
            match self.position.1{
                x if x > other.position.1 => Ordering::Greater,
                x if x == other.position.1 => {
                    match self.position.0{
                        y if y > other.position.0 => Ordering::Greater,
                        y if y == other.position.0 => Ordering::Equal,
                        _ => Ordering::Less,

                    }
                }
                _ => Ordering::Less
            }
        }
    }
    impl OctopusMap{
        pub fn new() -> Self{
            OctopusMap{map : BTreeMap::new(), flash_count : 0, first_synchronization : u32::MAX}
        }
        fn insert(&mut self,key : Position, value : Octopus){
            self.map.insert(key,value);
        }
        pub fn pass_step(&mut self, step : u32) {
            for i in 0..10 {
                for j in 0..10 {
                    let corresponding_position = Position::new((i,j));
                    self.increment_individual(&corresponding_position);
                }
            }
            for i in 0..10 {
                for j in 0..10 {
                    let corresponding_position = Position::new((i,j));
                    self.try_flashing(corresponding_position.clone());
                }
            }
            if self.is_all_flashing() && step < self.first_synchronization{
                self.first_synchronization = step + 1;
            }
            for i in 0..10 {
                for j in 0..10 {
                    let corresponding_position = Position::new((i,j));
                    self.fade_octopus(&corresponding_position);
                }
            }
            for i in 0..10 {
                for j in 0..10 {
                    let corresponding_position = Position::new((j,i));
                }
            }

        }
        fn try_flashing(&mut self, position : Position){
            let mut recurse_flag = false;
            self.map.entry(position.clone()).and_modify(|element| match element{
                x if x.energy > 9 && x.energy < 100 => {
                    x.energy=100;
                    recurse_flag = true;

                }
                _ => {},
            });
            if recurse_flag{
                self.increment_neighbors(&position);
            }
        }
        fn increment_neighbors(&mut self, position : &Position){
            let x = position.position.0;
            let y = position.position.1;
            let current_position = Position::new((x as usize, y as usize));
            let x_start = match x.checked_sub(1){
                Some(result) => result,
                None => x,
            };
            let y_start = match y.checked_sub(1){
                Some(result) => result,
                None => y,
            };
            for i in x_start..=x+1{
                for j in y_start..=y+1{
                    let neighbor_position = Position::new((i as usize, j as usize));
                    if !(i == x && j == y){
                        self.increment_individual(&neighbor_position);
                        self.try_flashing(neighbor_position);
                    }
                }
            }
        }
        fn increment_individual(&mut self, position : &Position){
            let x = position.position.0;
            let y = position.position.1;
            let current_position = Position::new((x as usize, y as usize));
            let result = self.map.entry(current_position);
            result.and_modify(|element| element.energy+=1);

        }
        fn fade_octopus(&mut self, position : &Position){
            let x = position.position.0;
            let y = position.position.1;
            let current_position = Position::new((x as usize, y as usize));
            self.map.entry(current_position).and_modify(|element| match element{
               x if x.energy > 9 => {
                   x.energy = 0;
                   self.flash_count += 1;
               }
                _ => {},
            });
        }
        fn is_all_flashing(&self) -> bool{
            let mut result = true;
            for i in 0..10{
                for j in 0..10{
                    let current_position = Position::new((i as usize, j as usize));
                    match self.map.get(&current_position){
                        Some(x) if !x.is_flashing() => {
                            result = false;
                            break
                        }
                        _ => {},
                    }
                }
            }
            result
        }
    }
}