pub mod part5_2{
    use std::iter::Zip;
    use std::ops::RangeInclusive;
    use crate::part5_1::part5_1::{LineSegment, SeaField};

    impl SeaField{
        pub fn mark_vertical_horizontal_diagonal_line(&mut self, to_mark : &LineSegment){
            let difference_x : i32 = to_mark.end_point.x as i32  - to_mark.start_point.x as i32 ;
            let difference_y : i32 = to_mark.end_point.y as i32 - to_mark.start_point.y as i32;
            let mut range = RangeInclusive::<u32>::new(0,0);
            let mut range_y = RangeInclusive::<u32>::new(0,0);
            if difference_x == 0{
                range = match difference_y{
                    x if x > 0 => to_mark.start_point.y..=to_mark.end_point.y,
                    _ => to_mark.end_point.y..=to_mark.start_point.y,
                };
                let constant = to_mark.start_point.x;
                for i in range{
                    self.sea_field[constant as usize ][i as usize] += 1;
                }
            }
            else if difference_y == 0{
                range = match difference_x{
                    x if x > 0 => to_mark.start_point.x..=to_mark.end_point.x,
                    _ => to_mark.end_point.x..=to_mark.start_point.x,
                };
                let constant = to_mark.start_point.y;
                for i in range{
                    self.sea_field[i as usize][constant as usize] += 1;
                }
            }
            else {

                let (  iter_x , iter_y) = match (difference_x,difference_y){
                    (x,y) if x > 0  && y > 0 => (1,1),
                    (x,y) if x > 0 && y < 0 => (1,-1),
                    (x,y) if x < 0 && y > 0 => (-1,1),
                    _ => (-1,-1),
                };
                let mut i = to_mark.start_point.x as i32;
                let mut y = to_mark.start_point.y as i32;
                while i - iter_x != to_mark.end_point.x as i32 && y - iter_y != to_mark.end_point.y as i32 {
                    self.sea_field[i as usize][y as usize] += 1;
                    y += iter_y;
                    i += iter_x;
                }
            }
        }
    }
}