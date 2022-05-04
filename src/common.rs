pub mod common{
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use std::path::Path;

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    pub fn read_file(filename : &String) -> Vec<u32>{
        let mut result = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            for line in lines{
                if let Ok(depth) = line{
                    result.push(depth.parse::<u32>().unwrap())
                }
            }
        }
        result
    }
    pub fn read_file_as_string(filename:&String) -> Vec<String>{
        let mut result = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            for line in lines{
                if let Ok(depth) = line{
                    result.push(depth.to_string());
                }
            }
        }
        result
    }
}