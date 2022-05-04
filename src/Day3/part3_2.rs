pub mod part3_2{
    pub fn count_bits_for_index(numbers: &Vec<String>, index: usize) -> (i32,usize){
        let mut count = 0;
        for number in numbers{
            count = match number.chars().collect::<Vec<char>>()[index]{
                '1' => count + 1,
                _ => count,
            }
        }
        (count,numbers.len())
    }
    pub fn evaluate_o2_rating(mut numbers: Vec<String>) -> String{
        let o2_bits = String::new();
        for i in 0..numbers[0].len(){
            if numbers.len() == 1 {
                break;
            }
            let (count,number_count) = count_bits_for_index(&numbers,i);
            if number_count - count as usize > count as usize {//case where most common bit is 0
                numbers.retain(|number| number.chars().collect::<Vec<char>>()[i] == '1');//most common bit is 0, thus removing 1s
            }
            else {// case where most common bit is 1 or 0 and 1 is equal
                numbers.retain(|number| number.chars().collect::<Vec<char>>()[i] =='0');//most common bits is 1, thus removing 0s
            }
        }
        match numbers.len(){
            1 => numbers[0].clone(),
            _ => o2_bits,
        }
    }
    pub fn evaluate_co2_rating(mut numbers: Vec<String>) -> String{
        let co2_bits = String::new();
        for i in 0..numbers[0].len(){
            if numbers.len() == 1 {
                break;
            }
            let (count,number_count) = count_bits_for_index(&numbers,i);
            if (number_count - count as usize) > count as usize {//case where most common bit is 0
                numbers.retain(|number| number.chars().collect::<Vec<char>>()[i] == '0');//most common bits is 1, thus removing 1s
            }
            else {//case where most common bit is 1 or 1 and 0 are equal
                numbers.retain(|number| number.chars().collect::<Vec<char>>()[i] =='1');//most common bit is 1, thus removing 1s
            }
        }
        match numbers.len(){
            1 => numbers[0].clone(),
            _ => co2_bits,
        }
    }
}