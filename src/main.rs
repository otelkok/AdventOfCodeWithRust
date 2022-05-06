#![allow(dead_code)]
#![allow(unused)]

#[path = "common.rs"] mod common;
#[path = "Day1/part1_1.rs"] mod part1_1;
#[path = "Day1/part1_2.rs"] mod part1_2;
#[path = "Day2/part2_1.rs"] mod part2_1;
#[path = "Day2/part2_2.rs"] mod part2_2;
#[path = "Day3/part3_1.rs"] mod part3_1;
#[path = "Day3/part3_2.rs"] mod part3_2;
#[path = "Day4/part4_1.rs"] mod part4_1;
#[path = "Day5/part5_1.rs"] mod part5_1;
#[path = "Day5/part5_2.rs"] mod part5_2;
#[path = "Day6/part6_1.rs"] mod part6_1;
#[path = "Day6/part6_2.rs"] mod part6_2;

use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use crate::part1_1::part1_1::count_increase;
use crate::part4_1::part4_1::BingoBoard;

use std::borrow::Borrow;
use std::collections::HashMap;
use crate::part6_1::part6_1::LanternFish;

// DAY1 START
    //DAY1 PART1 START
#[test]
fn control(){
    let mut depths = common::common::read_file(&"input1.txt".to_string());
    let count = part1_1::part1_1::count_increase(depths);
    assert_eq!(count,1342);
}
    //DAY1 PART1 END
    //DAY1 PART2 START

#[test]
fn windowed_control(){
    let mut depths = common::common::read_file(&"input1.txt".to_string());
    let count = crate::part1_2::part1_2::count_increase_window(depths);
    assert_eq!(count,1378);
}
    //DAY1 PART2 END
// DAY1 END
//DAY2 START
    //DAY2 PART1 START
#[test]
fn control_planned_course(){
    let mut commands = common::common::read_file_as_string(&"input2.txt".to_string());
    let (depth,horizontal) = part2_1::part2_1::evaluate_planned_course(commands);
    let multiplied = depth * horizontal;
    assert_eq!(multiplied,-1690020);
}
    //DAY2 PART1 END
    //DAY2 PART 2 START

#[test]
fn control_planned_course_with_aim(){
    let mut commands = common::common::read_file_as_string(&"input2.txt".to_string());
    let (depth,horizontal) = part2_2::part2_2::evaluate_planned_course_with_aim(commands);
    let multiplied = depth * horizontal;
    assert_eq!(multiplied,1408487760);
}
    //DAY2 PART 2 END
//DAY2 END
//DAY3 START
    //DAY3 PART1 START

#[test]
fn control_gamma_epsilon(){
    let numbers = common::common::read_file_as_string(&"input3.txt".to_string());
    let (counts,line_count) = part3_1::part3_1::count_bit_for_digits(numbers);
    let (epsilon,gamma) = part3_1::part3_1::evaluate_epsilon_gamma(counts,line_count);
    assert_eq!(gamma,"001011101010");
    assert_eq!(epsilon,"110100010101");
    let epsilon_as_integer = i32::from_str_radix(epsilon.as_str(),2).unwrap();
    let gamma_as_integer = i32::from_str_radix(gamma.as_str(),2).unwrap();
    assert_eq!(epsilon_as_integer*gamma_as_integer,2498354);
}

    //DAY3 PART1 END
    //DAY3 PART2 START

#[test]
fn control_o2_co2(){
    let numbers = common::common::read_file_as_string(&"input3.txt".to_string());
    let co2_numbers = numbers.clone();
    let o2_string = part3_2::part3_2::evaluate_o2_rating(numbers);
    let co2_string = part3_2::part3_2::evaluate_co2_rating(co2_numbers);
    assert_eq!(o2_string,"001101000100");
    assert_eq!(co2_string,"111101010001");
    let o2_as_integer = i32::from_str_radix(o2_string.as_str(),2).unwrap();
    let co2_as_integer = i32::from_str_radix(co2_string.as_str(),2).unwrap();
    assert_eq!(co2_as_integer*o2_as_integer,3277956);
}
    //DAY3 PART2 END
//DAY3 END
//DAY4 START
    //DAY4 PART1 START
#[test]
fn control_unmarked_sum(){
        let (moves, mut boards) = part4_1::part4_1::read_input_4("input4.txt");
        let mut finished= Option::None;
        let mut last_move = 0;
        for to_execute in moves{
            part4_1::part4_1::execute_move(to_execute,&mut boards);
            finished = part4_1::part4_1::control_finished(&boards);
            if finished.is_some(){
                last_move = to_execute;
                break;
            }
        }
        let sum = finished.unwrap()[0].sum_of_unmarked();
        assert_eq!(824,sum);
        assert_eq!(11536,sum * last_move);
    }
    //DAY4 PART1 END
    //DAY4 PART2 START
#[test]
fn control_unmarked_sum_for_last_winner(){
        let (moves, mut boards) = part4_1::part4_1::read_input_4("input4.txt");
        let mut finished= Option::None;
        let mut last_move = 0;
        let mut sum = 0;
        for to_execute in moves{
            part4_1::part4_1::execute_move(to_execute,&mut boards);
            finished = part4_1::part4_1::control_finished(&boards);
            match finished{
                None => continue,
                Some(value) => {
                    if boards.len() == 1 && value.len() == 1{
                        last_move = to_execute;
                        sum = value[0].sum_of_unmarked();
                        break;
                    }
                    else {
                        boards.retain(|element| !value.contains(element));
                    }
                }
            }
        }
        assert_eq!(sum,214);
        assert_eq!(last_move,6)
    }
    //DAY4 PART2 END
//DAY4 END
//DAY5 START
    //DAY5 PART1 START
#[test]
fn control_intersection_horizontal_vertical_lines(){
        let lines = common::common::read_file_as_string(&"input5.txt".to_string());
        let line_segments = part5_1::part5_1::read_line_segments(lines);
        let mut sea_field = part5_1::part5_1::SeaField::new();

        for line_segment in line_segments{
            sea_field.mark_vertical_horizontal_line(&line_segment);
        }
        let count = sea_field.count_existing_intersection();
        assert_eq!(count,7318);
}
    //DAY5 PART1 END
    //DAY5 PART2 START
#[test]
fn control_intersection_horizontal_vertical_diagonal_lines(){
        let lines = common::common::read_file_as_string(&"input5.txt".to_string());
        let line_segments = part5_1::part5_1::read_line_segments(lines);
        let mut sea_field = part5_1::part5_1::SeaField::new();

        for line_segment in line_segments{
            sea_field.mark_vertical_horizontal_diagonal_line(&line_segment);
        }
        let count = sea_field.count_existing_intersection();
        assert_eq!(count,19939);
    }
    //DAY5 PART2 END
//DAY5 END
//DAY6 START
    //DAY6 PART1 START
#[test]
fn control_lantern_fish_80_days(){
        let lines = common::common::read_file_as_string(&"input6.txt".to_string());
        let mut fish = part6_1::part6_1::read_input_6(lines);
        for i in 0..80{
            let mut new_fish = Vec::new();
            for mut fsh in fish{
                match fsh.pass_time(){
                    Some(element) => {new_fish.push(fsh); new_fish.push(element);},
                    _ => new_fish.push(fsh),
                };
            }
            fish =  new_fish;
        }
        assert_eq!(fish.len(),394994);
}
    //DAY6 PART1 END
    //DAY6 PART2 START
#[test]
fn control_lantern_fish_256_days(){
        let lines = common::common::read_file_as_string(&"input6.txt".to_string());
        let mut fish = part6_1::part6_1::read_input_6(lines);
        let mut count = 0;
        let mut answer_map = HashMap::new();
        let mut map_ref = &mut answer_map;
        for mut fsh in fish{
            count += part6_2::part6_2::how_many_fish_after(256,&mut fsh,map_ref);
        }
        assert_eq!(count,1765974267455);
    }
    //DAY6 PART2 END
//DAY6 END
fn main() {

}