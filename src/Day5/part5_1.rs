pub mod part5_1{
    pub fn read_line_segments(lines : Vec<String>) -> Vec<LineSegment>{
        let mut result = Vec::new();
        for line in lines{
            let reduced_line = line.replace("->","");
            let mut segmented_line = reduced_line.split_whitespace();
            let mut seg = segmented_line.next().unwrap().split(",");
            let x1 = seg.next().unwrap().parse::<u32>().unwrap();
            let y1 = seg.next().unwrap().parse::<u32>().unwrap();
            seg = segmented_line.next().unwrap().split(",");
            let x2 = seg.next().unwrap().parse::<u32>().unwrap();
            let y2 = seg.next().unwrap().parse::<u32>().unwrap();
            result.push(LineSegment::new(x1,y1,x2,y2));
        }
        result
    }
    #[derive(Debug)]
    pub struct Coordinate{
        pub(crate) x : u32,
        pub(crate) y : u32,
    }
    impl Coordinate{
        fn new(x : u32, y : u32) -> Self{
            Coordinate{x,y}
        }
    }
    #[derive(Debug)]
    pub struct LineSegment{
        pub(crate) start_point : Coordinate,
        pub(crate) end_point : Coordinate,
    }
    impl LineSegment{
        fn new(x1 : u32, y1 : u32, x2 : u32, y2 : u32) -> Self{
            LineSegment{start_point : Coordinate::new(x1,y1), end_point : Coordinate ::new(x2,y2)}
        }
    }
    pub struct SeaField{
        pub(crate) sea_field : Vec<Vec<u32>>,
    }
    impl SeaField{
        pub fn new() -> Self{
            let mut row = Vec::new();
            for i in 0..1000{
                row.push(0);
            }
            let mut result = Vec::new();
            for i in 0..1000{
                result.push(row.clone());
            }
            SeaField{sea_field : result }
        }
        pub fn mark_vertical_horizontal_line(&mut self, to_mark : &LineSegment){
            let difference_x : i32 = to_mark.end_point.x as i32  - to_mark.start_point.x as i32 ;
            let difference_y : i32 = to_mark.end_point.y as i32 - to_mark.start_point.y as i32;
            let mut range;
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
                return;
            }
        }
        pub fn count_existing_intersection(&self) -> u32 {
            let mut count = 0;
            for i in 0..1000{
                for y in 0..1000{
                    count = match self.sea_field[i][y]{
                        x if x > 1 => count + 1,
                        _ => count,
                    }
                }
            }
            count
        }
    }

}