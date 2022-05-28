pub mod part12_1{
    use std::collections::{HashMap, HashSet};
    use std::fmt::{Debug, Formatter};
    use std::path::Path;
    use std::ptr::write;

    pub fn read_input_12(lines : Vec<String>) -> Vec<DirectPath>{
        let mut result = Vec::new();
        for line in lines{
            let split : Vec<&str> = line.split('-').collect();
            result.push(DirectPath::new(Location::new(split[0].to_string()),Location::new(split[1].to_string())));
        }
        result
    }
    pub fn construct_path_string(paths : Vec<DirectPath>) -> Vec<PathString>{
        let mut result = Vec::new();
        let mut initial_candidates = Vec::new();
        for path in &paths{
            match path.is_start(){
                true => initial_candidates.push(DirectedPath::redirect_start(path.clone())),
                false => {},
            }
        }
        let mut candidates = Vec::new();
        initial_candidates.into_iter().for_each(|element| candidates.push(PathString::new(element)));
        loop{
            let mut new_candidates = Vec::new();
            for candidate in &candidates{
                match candidate.is_finished(){
                    true => {
                        new_candidates.push(candidate.clone());
                    },
                    false => {
                        let mut possible_progress = candidate.progress(&paths);
                        possible_progress.retain(|element| element.is_progressing_valid() || element.is_finished());
                        new_candidates.append(&mut possible_progress);
                    }
                }

            }
            match new_candidates.len(){
                x if x == candidates.len() => {
                    result = candidates;
                    break;
                }
                _ => {
                    candidates = new_candidates;
                }
            }
        }
        result
    }
    #[derive(Clone,Eq,PartialEq,Debug,Hash)]
    pub enum Location{
        START,
        END,
        BigCave(String),
        SmallCave(String),
    }
    #[derive(Clone,Eq,PartialEq,Debug,Hash)]
    pub struct DirectPath{
        pub from : Location,
        pub to : Location,
    }
    #[derive(Clone,Debug,Hash,Eq,PartialEq)]
    pub struct DirectedPath{
        pub from : Location,
        pub to : Location,
    }
    #[derive(Clone,Debug)]
    pub struct PathString{
        pub(crate) travel : Vec<DirectedPath>,
    }
    impl Location{
        pub fn new(name : String) -> Self{
            let start = "start".to_string();
            let end = "end".to_string();
            match name{
                x if x == start => Location::START,
                x if x == end => Location::END,
                x if x.to_uppercase() == x => Location::BigCave(x),
                x if x.to_lowercase() == x => Location::SmallCave(x),
                _ => panic!("Unexpected case while constructing Location"),
            }
        }
    }
    impl DirectPath{
        pub fn new(from : Location, to : Location) -> Self{
            DirectPath { from, to }
        }
        fn match_forwards(&self, source : &Vec<DirectPath>) -> Vec<DirectedPath>{
            let mut result = Vec::new();
            for path in source{
                match self.to.clone() {
                    x if x == path.to => result.push(DirectedPath{from : path.to.clone(), to : path.from.clone()}),
                    x if x == path.from => result.push(DirectedPath{from :path.from.clone(), to : path.to.clone()}),
                    _ => {}
                }
            }
            result
        }
        fn match_backwards(&self, source : &Vec<DirectPath>) -> Vec<DirectedPath>{
            let mut result = Vec::new();
            for path in source{
                match self.from.clone(){
                    x if x == path.to => result.push(DirectedPath{from : path.from.clone(), to : path.from.clone()}),
                    x if x == path.from => result.push(DirectedPath{from : path.to.clone(), to : path.from.clone()}),
                    _ => {},
                }
            }
            result
        }
        pub(crate) fn is_start(&self) -> bool{
            if self.from == Location::START || self.to == Location::START{
                true
            }
            else {
                false
            }
        }
        fn is_end(&self) -> bool{
            if self.from == Location::END || self.to == Location::END{
                true
            }
            else {
                false
            }
        }
        fn is_connected(&self, other : &Self) -> bool{
            let mut result = false;
            if self.from == other.from || self.from == other.to{
                result = true;
            }
            else if self.to == other.to || self.to == other.from{
                result = true;
            }
            result
        }
    }
    impl DirectedPath{
        pub fn new(from : Location, to : Location) -> Self{
            DirectedPath { from, to }
        }
        fn is_start(&self) -> bool{
            match self.from{
                Location::START => true,
                _ => false,
            }
        }
        fn is_end(&self) -> bool{
            match self.to{
                Location::END => true,
                _ => false,
            }
        }
        fn redirect_path(&self, other : &DirectPath) -> Option<Self>{
            if self.to == other.to{
                Option::Some(DirectedPath{from : other.to.clone(), to : other.from.clone()})
            }
            else if self.to == other.from{
                Option::Some(DirectedPath{from : other.from.clone(), to : other.to.clone()})
            }
            else {
                Option::None
            }
        }
        pub(crate) fn redirect_start(to_redirect : DirectPath) -> Self{
            match to_redirect{
                x if x.to == Location::START => DirectedPath{from : x.to, to : x.from},
                x if x.from == Location::START => DirectedPath{from : x.from, to : x.to},
                _ => panic!("This function is not expected to redirect paths not containing the START"),
            }
        }
        pub(crate) fn populate_both_ways(to_populate : &Vec<DirectPath>) -> Vec<Self>{
            let mut result = Vec::new();
            for direct_path in to_populate{
                result.push(DirectedPath{to : direct_path.to.clone(), from : direct_path.from.clone()});
                result.push(DirectedPath{to : direct_path.from.clone(), from : direct_path.to.clone()});
            }
            result
        }
        pub(crate) fn is_connected(&self, other : &Self) -> bool {
            if self.to == other.from{
                true
            }
            else{
                false
            }
        }
    }
    impl PathString{
        pub fn new(initial_path : DirectedPath) -> Self{
            let mut travel = vec!(initial_path);
            PathString{travel}
        }
        pub(crate) fn clone_from_append(&self, to_append : DirectedPath) -> Self{
            let mut result = self.clone();
            if self.travel.contains(&to_append){
                panic!("Unexpected case while appending!");
            }
            else {
                result.travel.push(to_append);
            }
            result
        }
        pub fn is_progressing_valid(&self) -> bool{
            let mut flag = true;
            let mut set = HashSet::new();
            let mut small_cave_map = HashMap::new();
            for element in &self.travel{
                set.insert(element.clone());
                match element.from.clone(){
                    Location::SmallCave(content) => {
                        let mut count = small_cave_map.entry(content).or_insert(0);
                        *count += 1;
                    },
                    Location::START => {
                        let mut count = small_cave_map.entry("START".to_string()).or_insert(0);
                        *count += 2;
                    }
                    Location::END => {
                        let mut count = small_cave_map.entry("END".to_string()).or_insert(0);
                        *count += 2;
                    }
                    _ => {}
                }
                match element.to.clone(){
                    Location::SmallCave(content) => {
                        let mut count = small_cave_map.entry(content).or_insert(0);
                        *count += 1;
                    },
                    Location::START => {
                        let mut count = small_cave_map.entry("START".to_string()).or_insert(0);
                        *count += 2;
                    }
                    Location::END => {
                        let mut count = small_cave_map.entry("END".to_string()).or_insert(0);
                        *count += 2;
                    }
                    _ => {}
                }
                flag = match element.to{
                    Location::END => false,
                    _ => flag,
                }
            }
            if set.len() != self.travel.len(){
                flag = false;
            }
            for (key,value) in small_cave_map{
                flag = match value{
                    x if x > 2 => false,
                    _ => flag,
                }
            }
            flag
        }
        pub fn is_finished(&self) -> bool {
            match self.travel.last(){
                Some(element) if element.to == Location::END => true,
                _ => false,
            }
        }
        /*pub fn is_valid(&self) -> bool{
            let mut flag = true;
            flag = self.is_progressing_valid();
            flag = match self.end.clone(){
                Some(content) => content == Location::END,
                None => false,
            };
            flag = match self.does_contain_cycle(){
                true => false,
                false => flag,
            };
            flag
        }*/
        pub fn does_contain_cycle(&self) -> bool{
            let mut control = HashSet::new();
            for path in self.travel.clone(){
                control.insert(path.clone());
            }
            match self.travel.len(){
                x if x == control.len() => true,
                _ => false,
            }
        }
        pub fn progress(&self, mut source: &Vec<DirectPath>) -> Vec<PathString> {
            let mut result = Vec::new();
            let mut candidates = DirectedPath::populate_both_ways(source);
            candidates.retain(|element|  !self.travel.contains(element) && self.travel.last().unwrap().is_connected(element));
            candidates.iter().for_each(|element| result.push(self.clone_from_append(element.clone())));
            result
        }
    }
}
