pub mod part8_2{
    use std::collections::{HashMap, HashSet};
    use crate::FixedSevenSegmentMap::{FIVE, FOUR, THREE};
    use crate::part8_1::part8_1::{CodedChar, SevenSegmentDigit};
    use crate::part8_1::part8_1::SevenSegmentDigit::ONE;
    use itertools::Itertools;

    #[derive(Clone)]
    pub struct InputEntry{
        pub seg_1 : Vec<String>,
        pub seg_2 : Vec<String>
    }
    pub struct OutputEntry{
        decoded : Vec<String>,
    }
    pub struct SevenSegmentOutput {
        output : Vec<u32>,
    }
    #[derive(Debug,Clone)]
    pub struct PotentialMatch{
        candidate_belonging : HashMap<char,Vec<FixedSevenSegmentMap>>,
        target_codes : Vec<String>,
        // If there is only one candidate for a FixedSevenSegment, then other chars in that match has to belong to that FixedSevenSegment too.
        //this relation may cause some candidates to be eliminated. If it is possible to be a char to exist in a single-owned chars group and
        //some other belonging, than candidate should not be eliminated.
    }
    #[derive(Clone,Debug)]
    pub struct PotentialCoding{
        map : HashMap<char,Vec<char>>,
        tries : Vec<char>,
        target_codes : Vec<String>,
    }
    #[derive(Debug)]
    pub struct ConcreteDecoding{
        decoded : Vec<CodedChar>,
    }
    impl InputEntry{
        pub fn new(seg1 : Vec<&str>, seg2 : Vec<&str>) -> Self{
            InputEntry{seg_1 : seg1.iter().map(|element| element.to_string()).collect(), seg_2 : seg2.iter().map(|element| element.to_string()).collect()}
        }
        pub fn seg1_as_coded_char(&self) -> Vec<Vec<char>>{
            let mut result = Vec::new();
            self.seg_1.iter().for_each(|element| result.push(element.chars().collect()));
            result
        }
        pub fn seg2_as_coded_char(&self) -> Vec<CodedChar>{
            let mut result = Vec::new();
            self.seg_2.iter().for_each(|element| element.chars().for_each(|character| result.push(CodedChar::new(&character))));
            result
        }
    }
    pub fn read_input_8_complete(lines : Vec<String>) -> Vec<InputEntry>{
        let mut result = Vec::new();
        for line in lines{
            let segments : Vec<&str> = line.split("|").collect();
            result.push(InputEntry::new(segments[0].split_whitespace().collect(),segments[1].split_whitespace().collect()));
        }
        result
    }

    impl PotentialMatch{
        pub fn new(target_codes : Vec<String>) -> Self{
            let mut result = HashMap::new();
            result.insert('a',Vec::new());
            result.insert('b',Vec::new());
            result.insert('c',Vec::new());
            result.insert('d',Vec::new());
            result.insert('e',Vec::new());
            result.insert('f',Vec::new());
            result.insert('g',Vec::new());
            PotentialMatch{candidate_belonging : result,target_codes}
        }
        pub fn initial_belongings(&mut self, sequence : InputEntry){
            let mut secondary_candidates = Vec::new();
            for token in sequence.seg1_as_coded_char(){
                match token.len(){
                    2 => { token.iter().for_each(|element| self.apply_belonging(FixedSevenSegmentMap::ONE,element.clone()))},
                    3 => {token.iter().for_each(|element| self.apply_belonging(FixedSevenSegmentMap::SEVEN,element.clone()))},
                    4 => {token.iter().for_each(|element| self.apply_belonging(FixedSevenSegmentMap::FOUR,element.clone()))},
                    5 => {secondary_candidates.push(token)},
                    6 => {secondary_candidates.push(token)},
                    7 => {token.iter().for_each(|element| self.apply_belonging(FixedSevenSegmentMap::EIGHT,element.clone()))},
                    _ => {},
                }
            }
            for secondary_candidate in secondary_candidates.clone(){
                match secondary_candidate.len(){
                    5 => {
                        secondary_candidate.iter().for_each(|element| self.apply_secondary_belonging([FixedSevenSegmentMap::TWO,FixedSevenSegmentMap::THREE,FixedSevenSegmentMap::FIVE],element.clone()));
                    },
                    6 => {
                        secondary_candidate.iter().for_each(|element| self.apply_secondary_belonging([FixedSevenSegmentMap::ZERO,FixedSevenSegmentMap::SIX,FixedSevenSegmentMap::NINE],element.clone()));
                    },
                    _ => {panic!("UNEXPECTED CASE");}
                }
            }
        }
        pub fn apply_belonging(&mut self, candidate : FixedSevenSegmentMap, to_own : char){
                let mut before = self.candidate_belonging.remove(&to_own).unwrap();
                if !before.contains(&candidate){
                    before.push(candidate);
                }
                self.candidate_belonging.insert(to_own,before);

        }
        pub fn apply_secondary_belonging(&mut self, candidates: [FixedSevenSegmentMap ; 3], to_own : char){
            for candidate in candidates{
                self.apply_belonging(candidate,to_own);
            }
        }
        pub fn is_possible_to_own(&self, sequence : &FixedSevenSegmentMap, to_own : char) -> bool{
            if self.candidate_belonging.get(&to_own).unwrap().contains(&sequence){
                false
            }
            else {
                self.does_contain_common_chars_with_current_owners(sequence,to_own)
            }
        }
        pub fn does_contain_common_chars_with_current_owners(&self, candidate : &FixedSevenSegmentMap, to_own : char) -> bool{
            let mut inital = vec!('a','b','c','d','e','f','g');
            for owner in self.candidate_belonging.get(&to_own).unwrap(){
                inital.retain(|element| owner.value().contains(element));
            }
            inital.len() != 0
        }
        pub fn imply_mapping_coding_possibility(&self) -> PotentialCoding{
            let mut result = PotentialCoding::new(self.target_codes.clone());
            for (to_char,value) in &self.candidate_belonging{
                for segment in value{
                    for from_char in segment.value(){
                        let mut canon_map = result.map.remove(&from_char).unwrap();
                        if !canon_map.contains(&to_char){
                            canon_map.push(to_char.clone())
                        }
                        result.map.insert(from_char,canon_map);
                    }
                }
            }
            result
        }
    }

    impl PotentialCoding{
        pub fn new(target_codes : Vec<String>) -> Self{
            let mut result = HashMap::new();
            result.insert('a',Vec::new());
            result.insert('b',Vec::new());
            result.insert('c',Vec::new());
            result.insert('d',Vec::new());
            result.insert('e',Vec::new());
            result.insert('f',Vec::new());
            result.insert('g',Vec::new());
            PotentialCoding{map : result, tries : Vec::new(),target_codes}
        }
        pub fn is_complete(&self) -> bool{
            let mut result = true;
            let mut owned = HashSet::new();
            for (key,value) in self.map.clone(){
                result = match value.len(){
                    1 => {owned.insert(value[0]); result},
                    _ => false,
                };
            }
            owned.len() == 7 && result
        }
        pub fn is_stuck(&self) -> bool{
            let mut result = false;
            for (key,value) in &self.map{
                result = match value.len(){
                    0 => true,
                    _ => result,
                };
            }
           result
        }
        pub fn is_able_to_decode_all(&self,mut decoder : PotentialCoding) -> bool{
            let result = true;
            let enums = vec!(FixedSevenSegmentMap::ZERO,
                             FixedSevenSegmentMap::ONE,
                             FixedSevenSegmentMap::TWO,
                             FixedSevenSegmentMap::THREE,
                             FixedSevenSegmentMap::FOUR,
                             FixedSevenSegmentMap::FIVE,
                             FixedSevenSegmentMap::SIX,
                             FixedSevenSegmentMap::SEVEN,
                             FixedSevenSegmentMap::EIGHT,
                             FixedSevenSegmentMap::NINE);
            let digits = vec!(0,1,2,3,4,5,6,7,8,9);
            let decoded = decoder.to_decoded();
            let mut result = true;
            let mut decodings = Vec::new();
            for coding in self.target_codes.clone(){
                let candidate = decoded.decode_test(coding);
                decodings.push(candidate);
                result = match digits.contains(&(candidate as i32)){
                    true => result,
                    false => false,
                }
            }
            result
        }
        pub fn solve_maze(&mut self) -> Option<PotentialCoding>{
            let keys = vec!('a','b','c','d','e','f','g');
            match self.solve_maze_recursive(keys){
                Some(potential_coding) => Some(potential_coding),
                None => None,
            }
        }
        pub fn solve_maze_recursive(&mut self, mut tries: Vec<char>) -> Option<PotentialCoding>{
            if self.is_complete(){
                let decoder = PotentialCoding{map : self.map.clone(), tries : Vec::new(), target_codes:  self.target_codes.clone()};
                if self.is_able_to_decode_all(decoder.clone()){
                    return Some(decoder);
                }
                else {
                    return None;
                }
            }
            else if self.is_stuck(){
                return None;
            }
            let to_try = tries[0];
            let mut next_tries = tries.clone();
            next_tries.remove(0);
            let mut candidates = self.map.remove(&to_try).unwrap();
            for coded_char in candidates.clone(){
                let mut progress = PotentialCoding{map : self.map.clone(), tries : self.tries.clone(),target_codes: self.target_codes.clone()};
                if progress.apply_owning(to_try, coded_char) {
                    progress.apply_coding();
                    match progress.solve_maze_recursive(next_tries.clone()){
                        Some(result) => return Some(result),
                        _ => {},
                    }
                }
            }
            None
        }
        pub fn apply_owning(&mut self, key : char, to_own : char) -> bool{
            let mut result = true;
            self.tries.push(to_own);
            let previous_candidates = self.map.remove(&key);
            let mut new_owner = Vec::new();
            new_owner.push(to_own);
            self.map.insert(key,new_owner);
            result = true;
            result
        }
        pub fn apply_coding(&mut self){
            for (key,value) in &self.map.clone(){
                match value.len(){
                    1  => {
                        self.remove_candidate_except_owner(key.clone(),value.get(0).unwrap().clone());
                    },
                    _ => {},
                }
            }
        }
        pub fn remove_candidate_except_owner(&mut self, owner : char, to_remove : char){
            for key in self.map.clone().keys(){
                if key.clone() != owner{
                    let mut to_remove_from = self.map.remove(key).unwrap();
                    for (index,candidate) in to_remove_from.clone().iter().enumerate(){
                        if candidate.clone() == to_remove{
                            to_remove_from.remove(index);
                        }
                    }
                    self.map.insert(key.clone(),to_remove_from);
                }
                else {
                    let mut to_remove_from = self.map.remove(key).unwrap();
                    let mut to_reinsert = Vec::new();
                    to_reinsert.push(to_remove.clone());
                    self.map.insert(key.clone(), to_reinsert);
                }
            }
        }
        pub fn to_decoded(&mut self) -> ConcreteDecoding{
            let mut result = Vec::new();
            for (key,value) in self.map.clone(){
                if value.len() > 1{
                    panic!("UNEXPECTED CASE");
                }
                else {
                    let meaning = match self.map.remove(&key).unwrap().get(0){
                        Some(coded_char) => {Some(coded_char.clone())},
                        None => None,
                    };
                    result.push(CodedChar{body : key.clone(), meaning});
                }
            }
            ConcreteDecoding::new(result)
        }
    }

    impl ConcreteDecoding{
        pub fn new(content : Vec<CodedChar>) -> Self{
            ConcreteDecoding{ decoded : content}
        }
        pub fn decode_test(&self, to_decode : String) -> u32{
            let mut to_insert = String::new();
            for char in to_decode.chars(){
                to_insert.push(self.get_meaning(char));
            }
            self.get_seven_segment_digit(to_insert)
        }
        pub fn decode(&self, to_decode : InputEntry) -> Vec<u32>{
            let mut result = Vec::new();
            for string in to_decode.seg_2{
                let mut to_insert = String::new();
                for char in string.chars(){
                    to_insert.push(self.get_meaning(char));
                }
                result.push(to_insert);
            }
            self.get_seven_segment_digits(OutputEntry{decoded : result})
        }
        fn get_seven_segment_digits(&self, to_search : OutputEntry) -> Vec<u32>{
            let mut result = Vec::new();
            for string in to_search.decoded{
                result.push(self.get_seven_segment_digit(string));
            }
            result
        }
        fn get_seven_segment_digit(&self, to_find : String) -> u32{
            match to_find.len(){
                2 => {1},
                3 => {7},
                4 => {4},
                5 => {self.get_seven_segment_with_len_5(to_find)},
                6 => {self.get_seven_segment_with_len_6(to_find)},
                7 => {8},
                _ => {u32::MAX},
            }
        }
        fn get_seven_segment_with_len_5(&self, to_look : String) -> u32{
            let two = vec!('a','c','d','e','g');
            let three = vec!('a','c','d','f','g');
            let five = vec!('a','b','d','f','g');
            let mut two_bool = true;
            let mut three_bool = true;
            let mut five_bool = true;
            for char in to_look.chars(){
                two_bool = match two.contains(&char){
                    true => two_bool,
                    false => false,
                };
                three_bool = match three.contains(&char){
                    true => three_bool,
                    false => false,
                };
                five_bool = match five.contains(&char){
                    true => five_bool,
                    false => false,
                };
            }
            if two_bool{
                2
            }
            else if three_bool{
                3
            }
            else if five_bool{
                5
            }
            else {
                u32::MAX
            }
        }
        fn get_seven_segment_with_len_6(&self, to_look : String) -> u32{
            let zero = vec!('a','b','c','e','f','g');
            let six = vec!('a','b','d','e','f','g');
            let nine = vec!('a','b','c','d','f','g');
            let mut zero_bool = true;
            let mut six_bool = true;
            let mut nine_bool = true;
            for char in to_look.chars(){
                zero_bool = match zero.contains(&char){
                    true => zero_bool,
                    false => false,
                };
                six_bool = match six.contains(&char){
                    true => six_bool,
                    false => false,
                };
                nine_bool = match nine.contains(&char){
                    true => nine_bool,
                    false => false,
                };
            }
            if zero_bool{
                0
            }
            else if six_bool{
                6
            }
            else if nine_bool{
                9
            }
            else {
                u32::MAX
            }
        }
        fn get_meaning(&self, to_get : char) -> char{
            for decoded in &self.decoded{
                if decoded.body == to_get{
                    return decoded.meaning.unwrap();
                }
            }
            '-'
        }
    }
    #[derive(PartialEq,Debug,Clone,Eq,Hash)]
   pub enum FixedSevenSegmentMap{
        ZERO,
        ONE,
        TWO,
        THREE,
        FOUR,
        FIVE,
        SIX,
        SEVEN,
        EIGHT,
        NINE
    }

    impl FixedSevenSegmentMap{
        pub fn value(&self) -> Vec<char>{
            match self{
                FixedSevenSegmentMap::ZERO =>  vec!('a', 'b', 'c', 'e', 'f', 'g'),
                FixedSevenSegmentMap::ONE => vec!('c','f'),
                FixedSevenSegmentMap::TWO =>  vec!('a','c','d','e','g'),
                FixedSevenSegmentMap::THREE => vec!('a','c','d','f','g'),
                FixedSevenSegmentMap::FOUR => vec!('b','c','d','f'),
                FixedSevenSegmentMap::FIVE => vec!('a','b','d','f','g'),
                FixedSevenSegmentMap::SIX => vec!('a','b','d','e','f','g'),
                FixedSevenSegmentMap::SEVEN => vec!('a','c','f'),
                FixedSevenSegmentMap::EIGHT => vec!('a','b','c','d','e','f','g'),
                FixedSevenSegmentMap::NINE => vec!('a','b','c','d','f','g'),
            }

        }

    }

}