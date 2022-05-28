pub mod part14_1{
    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};

    pub fn read_input14(lines : Vec<String>) -> (String, Vec<InsertionRule>){
        let mut result_string = String::new();
        let mut result_insertion = Vec::new();
        for line in lines{
            if !line.is_empty(){
                if line.contains("->"){
                    let mut tokens : Vec<&str> = line.split("->").collect();
                    let pair : Vec<char> = tokens[0].chars().collect();
                    let insert : Vec<char> = tokens[1].strip_prefix(" ").unwrap().chars().collect();
                    result_insertion.push(InsertionRule::new(pair[0],pair[1],insert[0]));
                }
                else {
                    result_string = line;
                }
            }
        }
        (result_string,result_insertion)
    }
    #[derive(Clone,Eq,PartialEq,Hash)]
    pub struct PolymerPair{
        pub(crate) first : char,
        pub(crate) second : char,
    }
    impl Debug for PolymerPair{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return write!(f,"[{}{}]",self.first,self.second);
        }
    }
    impl Display for PolymerPair{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return write!(f,"[{}{}]",self.first,self.second);
        }
    }
    #[derive(Debug,Clone,Eq,PartialEq)]
    pub struct InsertionRule{
        pub(crate) to : PolymerPair,
        pub(crate) insert : char,
    }
    pub struct Polymer{
        cursor : usize,
        pairs : Vec<PolymerPair>,
        single_polymer : Option<char>,
    }
    impl Display for Polymer{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut result = String::new();
            for pair in &self.pairs{
                result.push(pair.first);
                result.push(pair.second);
            }
            match self.single_polymer{
                Some(content) => {
                    result.push(content);
                }
                None => {},
            }
            write!(f,"{}",result)
        }
    }
    impl Display for InsertionRule{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return write!(f,"[{} -> {}]",self.to,self.insert);
        }
    }
    impl PolymerPair{
        pub fn new(first : char, second : char) -> Self {
            PolymerPair{first,second}
        }

        pub fn vec_from_string(string : String) -> Polymer {
            let mut result = Vec::new();
            let mut char_vec : Vec<char> = string.chars().collect();
            let mut last_char = char_vec[0];
            let mut single_polymer = None;
            for i in (0..char_vec.len()).step_by(2){
                match char_vec.get(i + 1){
                    Some(content) => {
                        result.push(PolymerPair::new(char_vec[i],char_vec[i+1]));
                    }
                    None => {
                        single_polymer = Some(char_vec[i]);
                    }
                }
            }
            Polymer::new(result,single_polymer)
        }
    }
    impl InsertionRule{
        pub fn new(first : char, second : char, third : char) -> Self{
            InsertionRule{to : PolymerPair::new(first, second), insert : third}
        }
    }
    impl Polymer{
        pub fn new(pairs : Vec<PolymerPair>,single_polymer : Option<char>) -> Self{
            let mut occurence_map = HashMap::new();
            for polymer in &pairs{
                let first_value = occurence_map.entry(polymer.first).or_insert(0);
                *first_value += 1;
                let second_value = occurence_map.entry(polymer.second).or_insert(0);
                *second_value += 1;
            }
            match single_polymer{
                Some(content) => {
                    let last_element = occurence_map.entry(content).or_insert(0);
                    *last_element += 1;
                }
                None => {},
            }
            Polymer{cursor : 0,pairs,single_polymer}
        }
        pub fn refresh_cursor(&mut self){
            self.cursor = 0;
        }
        fn insert_char(&mut self, to_insert : char){
            if self.cursor < self.pairs.len() * 2 {
                let (to_replace,poly_1) = match self.cursor % 2{
                    1 => {
                        self.cursor += 1;
                        let to_replace = self.pairs.remove(self.cursor / 2);
                        (to_replace.clone(),PolymerPair{first : to_insert, second : to_replace.first})
                    }
                    0 => {
                        let to_replace = self.pairs.remove(self.cursor / 2);
                        self.cursor += 1;
                        (to_replace.clone(), PolymerPair{first: to_replace.first, second : to_insert})
                    }
                    _ => {
                        panic!("Unexpected case!");
                    }
                };
                self.pairs.insert(self.cursor / 2,poly_1);
                self.cursor += 1;
                let mut cut_section = Vec::new();
                while self.pairs.len() > self.cursor / 2{
                    cut_section.insert(0,self.pairs.remove(self.pairs.len()-1));
                }
                let mut last_char = to_replace.second;
                for pair in cut_section{
                    self.pairs.push(PolymerPair{first: last_char, second: pair.first});
                    last_char = pair.second;
                }
                match self.single_polymer{
                    Some(content) => {
                        self.pairs.push(PolymerPair{first:last_char,second : content});
                        self.single_polymer = None;
                    }
                    None =>{
                        self.single_polymer = Some(last_char);
                    }
                }


            }
            else if self.cursor == self.pairs.len() * 2{
                match self.single_polymer{
                    Some(content) => {
                        self.pairs.push(PolymerPair{first : content,second : to_insert});
                        self.cursor += 1;

                    },
                    None => {
                        self.single_polymer = Some(to_insert);
                    }
                }
            }
            else {
                panic!("Unexpected case while inserting entry");
            }
        }
        pub fn step(&mut self, rules : &Vec<InsertionRule>){
            'outer : while self.cursor < self.len() {
                let to_match = self.get(self.cursor);
                'inner : for rule in rules{
                    match to_match{
                        Some(content) if content == rule.to => {
                            self.insert_char(rule.insert);
                            break 'inner;
                        }
                        Some(..) => continue,
                        None => break 'outer,
                    }
                }
            }

        }
        pub fn get(&self, index : usize) -> Option<PolymerPair>{
            match index % 2{
                0 => {
                    match index / 2{
                        x if x < self.pairs.len() => {
                            Some(self.pairs.get(index / 2).unwrap().clone())
                        }
                        _ => Option::None
                    }
                }
                1 =>{
                    match index / 2 {
                        x if x == self.pairs.len() - 1 => {
                            Some(PolymerPair{first:self.pairs.get(x).unwrap().second,second : 'Ğ'})
                        }
                        x if x < self.pairs.len() => {
                            Some(PolymerPair{first:self.pairs.get(x).unwrap().second,second: self.pairs.get(x+1).unwrap().first})
                        }
                        _ => {
                            Option::None
                        }
                    }
                }
                _ => Option::None
            }
        }
        pub fn len(&self) -> usize{
            self.pairs.len() * 2 + match self.single_polymer{
                Some(..) => 1,
                None => 0,
            }
        }
        pub fn count_chars(&self) -> HashMap<char,usize>{
            let mut result = HashMap::new();
            for pair in &self.pairs{
                let char_1 = result.entry(pair.first).or_insert(0);
                *char_1 += 1;
                let char_2 = result.entry(pair.second).or_insert(0);
                *char_2 += 1;
            }
            match self.single_polymer{
                Some(content) => {
                    let last_char = result.entry(content).or_insert(0);
                    *last_char += 1;
                }
                None => {},
            }
            result
        }
    }
}