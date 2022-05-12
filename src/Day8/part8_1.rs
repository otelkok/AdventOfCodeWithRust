pub mod part8_1{
    use std::borrow::Borrow;
    use std::collections::{HashMap, HashSet};
    pub fn read_input_8_seg_2(lines : Vec<String>) -> HashMap<u8,u64>{
        let mut count_1 = 0;
        let mut count_4 = 0;
        let mut count_7 = 0;
        let mut count_8 = 0;
        for line in lines{
            let spli : &str = line.split("|").collect::<Vec<&str>>()[1];
            let tokens : Vec<&str> = spli.split_whitespace().collect();
            for token in tokens{
                match CodedChar::try_and_update(token.chars().collect()){
                    Some(SevenSegmentDigit::ONE(..)) => count_1 += 1,
                    Some(SevenSegmentDigit::FOUR(..)) => count_4 += 1,
                    Some(SevenSegmentDigit::SEVEN(..)) => count_7 += 1,
                    Some(SevenSegmentDigit::EIGHT(..)) => count_8 += 1,
                    _ => {},
                }
            }
        }
        let mut result = HashMap::new();
        result.insert(1,count_1);
        result.insert(4,count_4);
        result.insert(7,count_7);
        result.insert(8,count_8);
        result
    }
    pub enum SevenSegmentDigit{
        ZERO(Vec<CodedChar>),
        ONE(Vec<CodedChar>),
        TWO(Vec<CodedChar>),
        THREE(Vec<CodedChar>),
        FOUR(Vec<CodedChar>),
        FIVE(Vec<CodedChar>),
        SIX(Vec<CodedChar>),
        SEVEN(Vec<CodedChar>),
        EIGHT(Vec<CodedChar>),
        NINE(Vec<CodedChar>),
    }
    #[derive(PartialEq,Eq,Hash,Debug,Clone)]
    pub struct CodedChar{
        pub body : char,
        pub meaning : Option<char>,
    }
    fn to_coded_char_arr(char_arr : &[char]) -> Vec<CodedChar> {
        let mut as_vec = Vec::new();
        for char in char_arr.clone(){
            as_vec.push(CodedChar::new(char));
        }
        as_vec
    }
    impl CodedChar{
        pub fn new(body : &char) -> Self{
            CodedChar{body : body.clone(), meaning: Option::None}
        }
        pub fn try_and_update(characters: Vec<char>) -> Option<SevenSegmentDigit>{
            match characters.len(){
                2 =>  Option::Some(SevenSegmentDigit::ONE(characters.iter().map(|element| CodedChar::new(element)).collect())),
                3 =>  Option::Some(SevenSegmentDigit::SEVEN(characters.iter().map(|element| CodedChar::new(element)).collect())),
                4 =>  Option::Some(SevenSegmentDigit::FOUR(characters.iter().map(|element| CodedChar::new(element)).collect())),
                7 =>  Option::Some(SevenSegmentDigit::EIGHT(characters.iter().map(|element| CodedChar::new(element)).collect())),
                _ => Option::None,
            }
        }
    }
}