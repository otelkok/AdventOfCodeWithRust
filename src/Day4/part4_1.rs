pub mod part4_1{
    use crate::common;
    use crate::read_file_as_string;

    #[derive(Copy,Clone,Debug,PartialEq)]
    pub struct BingoEntry{
        value : u32,
        is_marked : bool,
    }
    impl BingoEntry{
        pub fn new(value : u32) -> Self{
            BingoEntry{ value, is_marked : false}
        }
    }
    #[derive(Debug,Clone,Copy,PartialEq)]
    pub struct BingoBoard{
        rows : [[BingoEntry;5] ; 5],
    }
    impl BingoBoard{
        pub fn new(entries : &[Vec<u32>]) -> Self{
            let mut initial = BingoBoard{ rows : [[BingoEntry::new(0);5] ; 5]};
            for i in 0..5{
                for y in 0..5{
                    initial.rows[i][y].value = entries[i][y];
                }
            }
            initial
        }
        pub fn is_complete(&self) -> bool{
            let mut lasting_flag_column  = true;
            let mut lasting_flag_row  = true;
            for i in 0..5{
                lasting_flag_column = true;
                lasting_flag_row = true;
                for y in 0..5{
                    lasting_flag_row = match self.rows[i][y].is_marked {
                        true => lasting_flag_row,
                        false => false,
                    };
                    lasting_flag_column = match self.rows[y][i].is_marked{
                        true => lasting_flag_column,
                        false => false,
                    }
                }
                if lasting_flag_row || lasting_flag_column {
                    break;
                }
            }
            lasting_flag_row || lasting_flag_column
        }
        pub fn mark(&mut self, value : u32){
            for i in 0..5{
                for y in 0..5{
                    self.rows[i][y].is_marked = match self.rows[i][y].value{
                        x if x == value => true,
                        _ => self.rows[i][y].is_marked,
                    };
                }
            }
        }
        pub fn sum_of_unmarked(&self) -> u32 {
            let mut sum = 0;
            for i in 0..5{
                for y in 0..5{
                    sum = match self.rows[i][y].is_marked{
                        true => sum,
                        false => sum + self.rows[i][y].value,
                    }
                }
            }
            sum
        }
    }
    pub(crate) fn read_input_4(filename : &str) -> (Vec<u32>, Vec<BingoBoard>){
        let mut lines : Vec<String> = read_file_as_string(&filename.to_string());
        let moves = lines[0].split(",").map(|value| value.parse::<u32>().unwrap()).collect();
        lines.remove(0);
        lines.retain(|line| !line.is_empty());
        let mut parsed : Vec<Vec<u32>> = Vec::new();
        for line in lines.iter(){
            parsed.push(line.split_whitespace().map(|element| element.parse::<u32>().unwrap()).collect());
        }
        let mut boards : Vec<BingoBoard> = Vec::new();
        for i in 0..parsed.len()/ 5{
            boards.push(BingoBoard::new(&parsed[i*5..i*5+5]));
        }
        (moves,boards)
    }
    pub fn execute_move(to_execute : u32, boards : &mut Vec<BingoBoard>){
        for mut board in boards{
            board.mark(to_execute);
        }
    }
    pub fn control_finished(boards : &Vec<BingoBoard>) -> Option<Vec<BingoBoard>>{
        let mut result = Vec::new();
        for board in boards{
            if board.is_complete(){
                result.push(board.clone());
            }
        }
        match result.len() {
            0 => None,
            _ => Some(result),
        }
    }
}