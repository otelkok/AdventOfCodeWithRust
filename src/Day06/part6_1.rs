pub mod part6_1{
    pub fn read_input_6(lines : Vec<String>) -> Vec<LanternFish>{
        let mut fish = Vec::new();
        for line in lines{//actually input consists only 1 line
            line.split(",").map(|token| token.parse::<u32>()).for_each(|parsed| fish.push(LanternFish::new(parsed.unwrap())));
        }
        fish
    }
    #[derive(Copy,Clone,Debug)]
    pub struct LanternFish{
        pub time_until_creation : u32,
    }
    impl LanternFish{
        pub fn new(time_until_creation : u32) -> Self{
            LanternFish{time_until_creation}
        }
        pub fn pass_time(&mut self) -> Option<LanternFish>{
            match self.time_until_creation{
                0 => {self.time_until_creation = 6; Option::Some(LanternFish::new(8))},
                _ => {self.time_until_creation-=1; Option::None},
            }
        }
    }
}