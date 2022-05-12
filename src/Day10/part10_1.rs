pub mod part10_1{
    use std::collections::HashMap;

    pub fn find_corrupted_characters(lines : Vec<String>) -> Vec<char>{
        let mut result = Vec::new();
        let openings = ['(','[','{','<'];
        let closings = [')',']','}','>'];
        let mut mapping = HashMap::new();
        mapping.insert(')','(');
        mapping.insert(']','[');
        mapping.insert('}','{');
        mapping.insert('>','<');
        for line in lines{
            let mut stack: Vec<char> = Vec::new();
            for character in line.chars(){
                match character{
                    x if openings.contains(&x) => {stack.push(x);}
                    x if closings.contains(&x) => {
                        let check = stack.pop().unwrap();
                        match mapping.get(&character){
                            None => {result.push(character); break},
                            x if *x.unwrap() != check => {result.push(character); break},
                            _ => {}
                        }
                    },
                    _ => {},
                }
            }
        }
        result
    }
}