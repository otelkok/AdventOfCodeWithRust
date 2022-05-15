pub mod part2_2{
    pub fn evaluate_planned_course_with_aim(commands : Vec<String> ) -> (i32,i32){
        let mut depth : i32 = 0;
        let mut horizontal : i32 = 0;
        let mut aim : i32 = 0;
        let mut int_part = 0;
        for command in commands{
            let mut split_tokens : Vec<&str> = command.split(" ").collect();
            int_part = split_tokens[1].parse::<i32>().unwrap();
            (depth,horizontal,aim) = match &split_tokens[0] {
                x if x.contains("forward") => (depth + (aim * int_part),horizontal + int_part,aim),
                x if x.contains("down") => (depth, horizontal,aim + int_part),
                x if x.contains("up") => (depth,horizontal,aim - int_part),
                _ => (depth,horizontal,aim),
            };
        }
        (depth,horizontal)
    }
}