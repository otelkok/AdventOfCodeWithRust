pub mod part14_2{
    use std::collections::{HashMap};
    use std::hash::Hash;
    use itertools::Itertools;
    use crate::part14_1::part14_1::InsertionRule;
    use crate::PolymerPair;
    pub struct ExtendedPolymer{
        pub(crate) pair_map : HashMap<PolymerPair,u64>,
        count_map : HashMap<char,u64>,
        rule_map : HashMap<PolymerPair,char>,
    }
    impl ExtendedPolymer{
        pub fn new(initial : String,rules : Vec<InsertionRule>) -> Self{
            let mut pair_map = HashMap::new();
            let mut count_map = HashMap::new();
            let mut rule_map = HashMap::new();
            for rule in rules{
                rule_map.insert(rule.to,rule.insert);
            }
            for character in initial.chars(){
                let value = count_map.entry(character).or_insert(0);
                *value += 1;
            }
            let as_char : Vec<char> = initial.chars().collect();
            for pair in as_char.windows(2){
                let value = pair_map.entry(PolymerPair::new(pair[0],pair[1])).or_insert(0);
                *value += 1;
            }
            ExtendedPolymer{pair_map,count_map,rule_map}
        }
        pub fn step(&mut self){
            let mut new_pair_map = HashMap::new();
            for (key,value) in &self.pair_map{
                let lookup = self.rule_map.get(&key);
                match lookup{
                    Some(content) => {
                        let value_1 = new_pair_map.entry(PolymerPair::new(key.first,content.clone())).or_insert(0);
                        *value_1 += value;
                        let value_2 = new_pair_map.entry(PolymerPair::new(content.clone(),key.second)).or_insert(0);
                        *value_2 += value;
                        let char_count = self.count_map.entry(content.clone()).or_insert(0);
                        *char_count += value;
                    }
                    None => {

                    }
                }
            }
            self.pair_map = new_pair_map;
        }
        pub fn count_chars(&self) -> HashMap<char, u64>{
            self.count_map.clone()
        }

    }


}