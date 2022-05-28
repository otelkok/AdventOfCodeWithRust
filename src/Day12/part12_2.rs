pub mod part12_2{
    use std::collections::{HashMap, HashSet};
    use crate::part12_1::part12_1::{DirectedPath, DirectPath, Location, PathString};

    impl PathString{
        fn clone_from_append_extended(&self, to_append : DirectedPath) -> Self{
            let mut result = self.clone();
            result.travel.push(to_append);
            result
        }
        pub fn progress_extended(&self, mut source: &Vec<DirectPath>) -> Vec<PathString> {
            let mut result = Vec::new();
            let mut candidates = DirectedPath::populate_both_ways(source);
            candidates.retain(|element|  self.travel.last().unwrap().is_connected(element));
            candidates.iter().for_each(|element| result.push(self.clone_from_append_extended(element.clone())));
            result
        }
        pub fn is_progressing_valid_extended(&self) -> bool{
            let mut flag = true;
            let mut small_cave_map = HashMap::new();
            let mut start_end_map = HashMap::new();
            for element in &self.travel{
                match element.from.clone(){
                    Location::SmallCave(content) => {
                        let mut count = small_cave_map.entry(content).or_insert(0);
                        *count += 1;
                    },
                    Location::START => {
                        let mut count = start_end_map.entry("START".to_string()).or_insert(0);
                        *count += 2;
                    }
                    Location::END => {
                        let mut count = start_end_map.entry("END".to_string()).or_insert(0);
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
                        let mut count = start_end_map.entry("START".to_string()).or_insert(0);
                        *count += 2;
                    }
                    Location::END => {
                        let mut count = start_end_map.entry("END".to_string()).or_insert(0);
                        *count += 2;
                    }
                    _ => {}
                }
                flag = match element.to{
                    Location::END => false,
                    _ => flag,
                }
            }
            let mut small_cave_count = 0;
            for (key,value) in small_cave_map{
                small_cave_count = match value{
                    x if x > 2 && x <=4 => small_cave_count + 1,
                    x if x > 4 => small_cave_count + 2,
                    _ => small_cave_count,
                }
            }
            flag = match small_cave_count{
                x if x < 2 => flag,
                _ => false,
            };
            for (key,value) in start_end_map{
                flag = match value{
                    x if x > 2 => false,
                    _ => flag,
                }
            }
            flag
        }
    }
    pub fn construct_path_string_extended(paths : Vec<DirectPath>) -> Vec<PathString>{
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
        let mut append_count = 0;
        loop{
            append_count = 0;
            let mut new_candidates = Vec::new();
            for candidate in &candidates{
                match candidate.is_finished(){
                    true => {
                        new_candidates.push(candidate.clone());
                    },
                    false => {
                        let mut possible_progress = candidate.progress_extended(&paths);
                        possible_progress.retain(|element| element.is_progressing_valid_extended() || element.is_finished());
                        append_count += possible_progress.len();
                        new_candidates.append(&mut possible_progress);
                    }
                }
            }
            match append_count{
                0 => {
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
}