pub mod part7_1{
    use std::iter::Sum;

    pub fn read_input_7(lines : Vec<String>) -> Vec<u32>{
        let mut crab = Vec::new();
        for line in lines{//actually input consists only 1 line
            line.split(",").map(|token| token.parse::<u32>().unwrap()).for_each(|parsed| crab.push(parsed));
        }
        crab
    }
    pub fn fuel_spent(crabs : &Vec<u32>, to : u64) -> u64{
        crabs.iter().fold(0,|sum,current| sum + to_position(*current, to))
    }
    pub fn center_of_mass(crabs : Vec<u32>) -> (u32,u64) {
        let mut min = (&crabs).iter().fold(&crabs[0],|min,element| element.min(&min));
        let mut max = (&crabs).iter().fold(&crabs[0],|min,element| element.max(&min));
        let mut min_fuel = u64::MAX;
        let mut min_position = u32::MIN;
        for position in *min..=*max{
            (min_fuel,min_position) = match fuel_spent(&crabs, position as u64){
                x if x < min_fuel => (x, position),
                _ => (min_fuel,min_position),
            }
        }
        (min_position,min_fuel)
    }
    pub fn to_position(from : u32, to : u64) -> u64{
        i64::abs(from as i64 - to as i64) as u64
    }

}
