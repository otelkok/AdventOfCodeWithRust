pub mod part7_2{
    use crate::part7_1::part7_1::to_position;
    pub fn center_of_mass_increasing(crabs : Vec<u32>) -> (u32,u64) {
        let mut min = (&crabs).iter().fold(&crabs[0],|min,element| element.min(&min));
        let mut max = (&crabs).iter().fold(&crabs[0],|min,element| element.max(&min));
        let mut min_fuel = u64::MAX;
        let mut min_position = u32::MIN;
        for position in *min..=*max{
            (min_fuel,min_position) = match fuel_spent_increasing(&crabs, position as u64){
                x if x < min_fuel => (x, position),
                _ => (min_fuel,min_position),
            }
        }
        (min_position,min_fuel)
    }
    pub fn fuel_spent_increasing(crabs: &Vec<u32>, to : u64) -> u64{
        crabs.iter().fold(0,|sum,current| sum + to_position_increasing(*current, to))
    }
    pub fn to_position_increasing(from : u32, to : u64) -> u64{
        let difference = i64::abs(from as i64 - to as i64) as u64;
        difference * (difference + 1) / 2
    }
}