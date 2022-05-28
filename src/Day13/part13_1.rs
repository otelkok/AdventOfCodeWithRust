pub mod part13_1{
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, HashMap};
    use std::fmt::{Display, Formatter};
    pub fn read_input_13(lines : Vec<String>) -> (Paper,Vec<FoldInstruction>){
        let mut result = Paper::new();
        let mut instruction_result = Vec::new();
        for line in lines{
            if line.contains(","){
                result.mark(Position::new(line));
            }
            else if line.contains("fold along"){
                instruction_result.push(FoldInstruction::new(line));
            }
        }
        (result,instruction_result)
    }
    #[derive(Debug)]
    pub struct FoldInstruction{
        vertical : bool,
        index : u16,
    }
    #[derive(Clone,PartialEq,Eq,Hash,Debug)]
    pub struct Position{
        x : u16,
        y : u16,
    }

    impl PartialOrd<Self> for Position {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.y > other.y {
                Some(Ordering::Greater)
            }
            else if self.y == other.y{
                if self.x > other.x{
                    Some(Ordering::Greater)
                }
                else if self.x == other.x{
                    Some(Ordering::Equal)
                }
                else {
                    Some(Ordering::Less)
                }
            }
            else {
                Some(Ordering::Less)
            }
        }
    }

    impl Ord for Position{
        fn cmp(&self, other: &Self) -> Ordering {
            if self.y > other.y {
                Ordering::Greater
            }
            else if self.y == other.y{
                if self.x > other.x{
                    Ordering::Greater
                }
                else if self.x == other.x{
                    Ordering::Equal
                }
                else {
                    Ordering::Less
                }
            }
            else {
                Ordering::Less
            }
        }
    }
    #[derive(Debug)]
    pub struct Paper{
        dot_map : BTreeMap<Position,bool>,
        x_max : u16,
        y_max : u16,
    }
    impl Display for Paper{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut result = String::new();
            for j in 0..=self.y_max{
                for i in 0..=self.x_max{
                    match self.dot_map.contains_key(&Position { x: i, y: j }){
                        true => result.push('#'),
                        false => result.push('.'),
                    }
                    result.push(' ');
                }
                result.push('\n');
            }
            return write!(f,"{}",result);
        }
    }
    impl FoldInstruction{
        pub fn new(line : String) -> Self{
            let tokens : Vec<&str> = line.split_whitespace().collect();
            let token : Vec<&str>= tokens.last().unwrap().split("=").collect();

            let mut flag = match token[0]{
                "x" => false,
                "y" => true,
                _ => {panic!("Unexpected case while parsing folding instruction")},
            };
            FoldInstruction{vertical : flag, index : token[1].parse::<u16>().unwrap()}
        }
    }
    impl Position{
        pub fn new(line : String) -> Self{
            let tokens : Vec<&str> = line.split(",").collect();
            Position{ x : tokens[0].parse::<u16>().unwrap(), y : tokens[1].parse::<u16>().unwrap()}
        }
    }
    impl Paper{
        pub fn new() -> Self{
            Paper{dot_map : BTreeMap::new(), x_max : 0, y_max : 0}
        }
        pub fn execute_instruction(&mut self, instruction : &FoldInstruction) -> Self{
            let mut result = Paper::new();
            match instruction.vertical{
                true => {
                    let (mut left,right) = self.cut_vertical(instruction.index);
                    left = left.overlap_vertical(&right);
                    result = left;
                }
                false => {
                    let (mut upper, down) = self.cut_horizontal(instruction.index);
                    upper = upper.overlap_horizontal(&down);
                    result = upper;
                }
            }
            result
        }
        pub fn marked_dot(&self) -> usize{
            for (key,value) in &self.dot_map{
            }
            self.dot_map.len()
        }
        pub fn mark(&mut self,to_mark : Position){
            self.dot_map.entry(to_mark.clone()).or_insert(true);
            self.x_max = match to_mark.x{
                x if x > self.x_max => x,
                _ => self.x_max,
            };
            self.y_max = match to_mark.y{
                y if y > self.y_max => y,
                _ => self.y_max,
            };
        }
        fn overlap_horizontal(&mut self, other : &Self) -> Self{
            let mut result = Paper::new();
            let mut mirrored_other = other.mirror_horizontal();
            for (key,value) in &self.dot_map{
                result.mark(key.clone());
            }
            for (key,value) in mirrored_other.dot_map{
                result.mark(key);
            }
            result
        }
        fn cut_horizontal(&self, index : u16) -> (Self,Self){
            let mut cut_1 = Paper::new();
            let mut cut_2 = Paper::new();
            for i in 0..index{
                for j in 0..=self.y_max{
                    let position = Position{x : i, y : j};
                    if self.dot_map.contains_key(&position) {
                        cut_1.mark(position);
                    }

                }
            }
            for i in index+1..=self.x_max{
                for j in 0..=self.y_max{
                    if self.dot_map.contains_key(&Position{x : i, y : j}) {
                        cut_2.mark(Position{x : i - index - 1, y : j});
                    }
                }
            }
            (cut_1,cut_2)
        }
        fn mirror_horizontal(&self) -> Self{
            let mut result = Paper::new();
            for (key, value) in &self.dot_map{
                result.mark(Position{x : self.x_max - key.x, y : key.y});
            }
            result
        }
        pub fn overlap_vertical(&mut self, other : &Self) -> Self{
            let mut result = Paper::new();
            let mirrored_other = other.mirror_vertical();
            for (key,value) in &self.dot_map{
                result.mark(key.clone());
            }
            for (key,value) in mirrored_other.dot_map{
                result.mark(key);
            }
            result

        }
        fn cut_vertical(&self, index : u16) -> (Self,Self){
            let mut cut_1 = Paper::new();
            let mut cut_2 = Paper::new();
            for i in 0..=self.x_max{
                for j in 0..index{
                    let position = Position{x : i, y : j};
                    if self.dot_map.contains_key(&position) {
                        cut_1.mark(position);
                    }
                }
            }
            for i in 0..=self.x_max{
                for j in index+1..=self.y_max{
                    if self.dot_map.contains_key(&Position{x : i, y : j}) {
                        cut_2.mark(Position{x : i, y: j - index - 1});
                    }
                }
            }
            (cut_1,cut_2)
        }
        pub fn mirror_vertical(&self) -> Self{
            let mut result = Paper::new();
            for (key,value) in &self.dot_map{
                result.mark(Position{x : key.x, y : self.y_max - key.y});
            }
            result
        }
    }
}