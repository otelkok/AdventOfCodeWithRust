pub mod part3_1{
    pub fn count_bit_for_digits(numbers : Vec<String>) -> (Vec<i32>,i32){
        let mut counts = vec![0;numbers[0].len()];
        let mut line_count = 0;
        for number in numbers{
            line_count+=1;
            for (index,character) in number.chars().enumerate(){
                counts[index] = match character{
                    x if x == '0' => counts[index] + 1,
                    _ => counts[index],
                }
            }
        }
        (counts,line_count)
    }
    pub fn evaluate_epsilon_gamma(counts : Vec<i32>, line_count : i32) -> (String,String){
        let mut epsilon= String::new();
        let mut gamma = String::new();
        for count in counts{
            if line_count - count > count{
                epsilon.push_str("1");
                gamma.push_str("0");
            }
            else {
                epsilon.push_str("0");
                gamma.push_str("1");
            }
        }
        (epsilon,gamma)
    }

}