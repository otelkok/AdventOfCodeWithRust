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
#[path = "Day7/part7_1.rs"] mod part7_1;
#[path = "Day7/part7_2.rs"] mod part7_2;
#[path = "Day8/part8_1.rs"] mod part8_1;
#[path = "Day8/part8_2.rs"] mod part8_2;
#[path = "Day9/part9_1.rs"] mod part9_1;
#[path = "Day9/part9_2.rs"] mod part9_2;
#[path = "Day10/part10_1.rs"] mod part10_1;
#[path = "Day10/part10_2.rs"] mod part10_2;
#[path = "Day11/part11_1.rs"] mod part11_1;
#[path = "Day11/part11_2.rs"] mod part11_2;

use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use crate::part1_1::part1_1::count_increase;
use crate::part4_1::part4_1::BingoBoard;

use std::borrow::Borrow;
use std::collections::HashMap;
use crate::part6_1::part6_1::LanternFish;
use crate::part8_2::part8_2::{FixedSevenSegmentMap, PotentialMatch};
use crate::part8_2::part8_2::FixedSevenSegmentMap::ONE;
use crate::part9_2::part9_2::Basin;

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
//DAY7 START
    //DAY7 PART1 START
#[test]
fn control_constant_crab_fuel(){
        let lines = common::common::read_file_as_string(&"input7.txt".to_string());
        let mut crabs = part7_1::part7_1::read_input_7(lines);

        let (min_position,min_fuel)  = part7_1::part7_1::center_of_mass(crabs.clone());
        assert_eq!(min_position,323);
        assert_eq!(min_fuel,336701);
}
    //DAY7 PART1 END
    //DAY7 PART2 START
#[test]
fn control_increasing_crab_fuel(){
        let lines = common::common::read_file_as_string(&"input7.txt".to_string());
        let mut crabs = part7_1::part7_1::read_input_7(lines);
        let (min_position,min_fuel)  = part7_2::part7_2::center_of_mass_increasing(crabs.clone());
        assert_eq!()(min_position,461);
        assert_eq!()(min_fuel,95167302);
    }
    //DAY7 PART2 END
//DAY7 END
//DAY8 START
    //DAY8 PART1 START
#[test]
fn control_1_4_7_8(){
        let lines = common::common::read_file_as_string(&"input8.txt".to_string());
        let mut map = part8_1::part8_1::read_input_8_seg_2(lines);
        assert_eq!(map.get(&1_u8),118);
        assert_eq!(map.get(&4_u8),143);
        assert_eq!(map.get(&7_u8),136);
        assert_eq!(map.get(&8_u8),142);
        let sum =map.values().fold(0,|sum,element| sum + element);
        assert_eq!(sum,539);
}
    //DAY8 PART1 END
    //DAY8 PART2 START
#[test]
fn control_all_codes(){
        let lines = common::common::read_file_as_string(&"input8.txt".to_string());
        let entries = part8_2::part8_2::read_input_8_complete(lines);
        let mut sum: u64 = 0;
        for entry in entries{
            let mut potential_match = PotentialMatch::new(entry.seg_1.clone());
            potential_match.initial_belongings(entry.clone());
            let mut solve = potential_match.imply_mapping_coding_possibility();
            let solved = solve.solve_maze();
            let decoder = solved.unwrap().to_decoded();
            let decoded = decoder.decode(entry);
            let number = decoded.iter().fold(0,|sum,element| (sum * 10) + element);
            sum += number as u64;
        }
        assert_eq!(sum,1084606);
    }
    //DAY8 PART2 END
//DAY8 END
//DAY9 START
    //DAY9 PART1 START
#[test]
fn control_risk_point(){
    let lines = common::common::read_file_as_string(&"input9.txt".to_string());
    let entries = part9_1::part9_1::read_input_9(lines);
    let risk_point = part9_1::part9_1::risk_points(part9_1::part9_1::low_points(entries));
    assert_eq!(risk_point,900);
}
    //DAY9 PART1 END
    //DAY9 PART2 START
fn control_basin_sizes(){
        let lines = common::common::read_file_as_string(&"input9.txt".to_string());
        let entries= part9_2::part9_2::read_input_9_as_map(lines);
        let low_points = entries.low_points();
        let basins = Basin::from_low_point(low_points,&entries);
        let (mut index1, mut count1) = (0, 0);
        let (mut index2, mut count2) = (0, 0);
        let (mut index3, mut count3) = (0, 0);
        for (index,basin) in basins.iter().enumerate(){
            match basin.content.len(){
                x if x > count1 => {
                    index3 = index2;
                    index2 = index1;
                    index1 = index;
                    count3 = count2;
                    count2 = count1;
                    count1 = x;
                }
                x if x > count2 => {
                    index3 = index2;
                    index2 = index;
                    count3 = count2;
                    count2 = x;
                }
                x if x > count3 => {
                    index3 = index;
                    count3 = x;
                }
                _ =>{},
            }
        }
        let size1 = basins[index1].content.len();
        let size2 = basins[index2].content.len();
        let size3 = basins[index3].content.len();
        assert_eq!(size1,105);
        assert_eq!(size2,98);
        assert_eq!(size3,96);
        assert_eq!(size1*size2*size3,987840);
    }
    //DAY9 PART2 END
//DAY9 END
//DAY10 START
    //DAY10 PART1 START
#[test]
fn control_corruption_point(){
        let lines = common::common::read_file_as_string(&"input10.txt".to_string());
        let corruptions= part10_1::part10_1::find_corrupted_characters(lines);
        let mut sum = 0;
        for corruption in corruptions{
            match corruption{
                ')' => sum += 3,
                ']' => sum += 57 ,
                '}' => sum += 1197,
                '>' => sum += 25137,
                _ => {}
            }
        }
        assert_eq!(sum,370407);
    }
    //DAY10 PART1 END
    //DAY10 PART2 START
#[test]
fn control_incomplete(){
        let lines = common::common::read_file_as_string(&"input10.txt".to_string());
        let mut removed_lines = Vec::new();
        for line in lines{
            if !part10_2::part10_2::is_line_corrupted(line.clone()){
                removed_lines.push(line);
            }
        }
        let mut sum : u64 = 0;
        let mut sums = Vec::new();
        let mut sub_sum:u64  = 0;
        for line in removed_lines{
            let to_append = part10_2::part10_2::fill_incomplete_line(part10_2::part10_2::remove_paired_closing_characters(line));
            sub_sum = 0;
            for character in to_append{
                sub_sum *= 5;
                match character{
                    ')' => sub_sum+= 1,
                    ']' => sub_sum+= 2,
                    '}' => sub_sum+= 3,
                    '>' => sub_sum+= 4,
                    _ => {},
                }
            }
            sum += sub_sum;
            sums.push(sub_sum);
        }
        sums.sort();
        let mid_element = sums[(sums.len() / 2) ];
        assert_eq!(mid_element,3249889609);
        assert_eq!(sum,350352132259);
}
    //DAY10 PART2 END
//DAY10 END
//DAY11 START
    //DAY11 PART1 START
    //DAY11 PART1 END
    //DAY11 PART2 START
    //DAY11 PART2 END
//DAY11 END
fn main() {
        let lines = common::common::read_file_as_string(&"input_test.txt".to_string());
        let mut game_map = part11_1::par11_1::read_input_11(lines);
        for i in 0..1{
            game_map.pass_step();
        }
        println!("Flash count:{}",game_map.flash_count);
}