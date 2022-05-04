pub mod part1_2{
    pub fn count_increase_window(depths: Vec<u32>) -> u32 {
        let mut count = 0;
        let mut sum1 = depths[0] + depths[1] + depths[2];
        let mut sum2 = depths[1] + depths[2];
        for i in 3..depths.len(){
            sum2 += depths[i];
            count = match sum2{
                x if x > sum1 => count + 1,
                _ => count,
            };
            sum2 -= depths[i-2];
            sum1 += depths[i];
            sum1 -= depths[i-3];
        }
        count
    }
}