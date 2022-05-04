pub mod part1_1{
    pub(crate) fn count_increase(depths : Vec<u32>) -> u32 {
        let mut count = 0;
        let mut previous_depth = depths[0];
        for depth in depths{
            count = match depth{
                x if x > previous_depth => count+1,
                _ => count,
            };
            previous_depth = depth;
        }
        count
    }
}
