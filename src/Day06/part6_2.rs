pub mod part6_2{
    use std::collections::HashMap;
    use crate::LanternFish;

   pub fn how_many_fish_after(n_days : u64, producer: & mut LanternFish, answer_map: & mut HashMap<(u64, u32),u64>) -> u64{
       match answer_map.get(&(n_days,producer.time_until_creation)){
           None => {},
           Some(answer) => return answer.clone(),
       }
       let mut after_creation : i64 = match n_days as i64{
           x if x <= producer.time_until_creation as i64 => -1,
           _ => (n_days as i64 - 1) - producer.time_until_creation as i64,
       };
       let mut count1 = 0;
       let mut count2 = 0;
       let mut time_until_creation_before_change = producer.time_until_creation;
       producer.time_until_creation = 6;
       let result = match after_creation{
           x if x == -1 => 1,
           x if x < 7 => 2,
           _ => how_many_fish_after(after_creation as u64 ,producer,answer_map) + how_many_fish_after(after_creation as u64, & mut LanternFish::new(8),answer_map),
       };
       answer_map.insert((n_days,time_until_creation_before_change),result);

       result
   }
}