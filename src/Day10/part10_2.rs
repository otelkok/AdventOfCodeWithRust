pub mod part10_2{
    use std::collections::HashMap;

    pub fn is_line_corrupted(line : String) -> bool{
        let openings = ['(','[','{','<'];
        let closings = [')',']','}','>'];
        let mut mapping = HashMap::new();
        mapping.insert(')','(');
        mapping.insert(']','[');
        mapping.insert('}','{');
        mapping.insert('>','<');
        let mut stack: Vec<char> = Vec::new();
        for character in line.chars(){

            match character{
                x if openings.contains(&x) => {stack.push(x);}
                x if closings.contains(&x) => {
                    let check = stack.pop().unwrap();
                    match mapping.get(&character){
                        None => return true,
                        x if *x.unwrap() != check => return true,
                        _ => {}
                    }
                },
                _ => {},
            }
        }
        false
    }
    pub fn remove_paired_closing_characters(line : String) -> Vec<char>{
        let openings = ['(','[','{','<'];
        let closings = [')',']','}','>'];
        let mut mapping = HashMap::new();
        mapping.insert(')','(');
        mapping.insert(']','[');
        mapping.insert('}','{');
        mapping.insert('>','<');
        let mut stack: Vec<char> = Vec::new();
        for character in line.chars(){
            match character{
                x if openings.contains(&x) => {stack.push(x);}
                x if closings.contains(&x) => {
                    let check = stack.pop().unwrap();
                    match mapping.get(&character){
                        None => panic!("Unexpected case"),
                        Some(y) if *y == check => {()},
                        Some(y) if openings.contains(y) => {stack.push(check); stack.push(character);}
                        _ => {stack.push(character)},
                    }
                },
                _ => {},
            }
        };
        stack
    }
    pub fn fill_incomplete_line(mut line: Vec<char>) -> Vec<char>{
        let mut mapping = HashMap::new();
        mapping.insert('(',')');
        mapping.insert('[',']');
        mapping.insert('{','}');
        mapping.insert('<','>');
        let mut result = Vec::new();
        for character in line.iter().rev(){
            result.push(*mapping.get(character).unwrap());
        }
        result
    }
}