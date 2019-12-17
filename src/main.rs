use advent_of_code_2019::day_1;
use advent_of_code_2019::day_2;

fn main() {
    let day_1p1_answer = day_1::d1p1::calculate_sum_of_fuel_req("inputs/day_1.txt");
    let day_1p2_answer = day_1::d1p2::calculate_sum_of_fuel_req("inputs/day_1.txt");
    println!("Day_1_1 answer: {}", day_1p1_answer);
    println!("Day_1_2 answer: {}", day_1p2_answer);

    let day_2p1_answer = day_2::intcode::calculate_intcode("inputs/day_2.txt");
    let day_2p2_answer = day_2::intcode::calculate_pairs("inputs/day_2.txt");
    println!("Day_2_1 answer: {}", day_2p1_answer);
    println!("Day_2_2 answer: {}", day_2p2_answer);
}