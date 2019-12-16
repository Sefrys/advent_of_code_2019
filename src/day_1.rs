use crate::reader::read;

fn calculate_fuel_req(x: u32) -> u32 {
    (x / 3) as u32 - 2
}

pub fn calculate_sum_of_fuel_req(filename: &str) -> u32 {
   read(filename).lines().map(|x| calculate_fuel_req(x.parse::<u32>().unwrap())).sum()
}



#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_fuel_req() {
        assert_eq!(654, calculate_fuel_req(1969));
        assert_eq!(33583, calculate_fuel_req(100756))
    }

    #[test]
    fn test_sum_fuel_req() {
        assert_eq!(34237, calculate_sum_of_fuel_req("inputs/day_1_test.txt"));
    }
}