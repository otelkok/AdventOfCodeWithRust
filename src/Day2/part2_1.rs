pub mod part2_1{
    pub fn evaluate_planned_course(commands : Vec<String> ) -> (i32,i32){
        let mut depth : i32 = 0;
        let mut horizontal : i32 = 0;
        let mut int_part = 0;
        for command in commands{
            let mut split_tokens : Vec<&str> = command.split(" ").collect();
            int_part = split_tokens[1].parse::<i32>().unwrap();
            (depth,horizontal) = match &split_tokens[0] {
                x if x.contains("forward") => (depth,horizontal + int_part),
                x if x.contains("down") => (depth - int_part, horizontal),
                x if x.contains("up") => (depth + int_part,horizontal),
                _ => (depth,horizontal),
            };
        }
        (depth,horizontal)
    }
}