pub mod day_1_1 {
    use crate::reader::read_file;

    pub fn calculate_fuel_req(x: u32) -> u32 {
        (x / 3) as u32 - 2
    }

    pub fn calculate_sum_of_fuel_req(filename: &str) -> u32 {
        read_file(filename).lines().map(|x| calculate_fuel_req(x.parse::<u32>().unwrap())).sum()
    }
}

pub mod day_1_2 {
    use crate::reader::read_file;
    // 1. Get module mass
    // 2. Get fuel req for module mass
    // 3. Save last req as `last_fuel_mass`
    // 4. Get fuel req for last_fuel_mass -> increment fuel req by that amount -> change last fuel_req


    pub struct ShipModule {
        pub module_mass: i32,
        pub module_fuel_mass: i32,
        pub last_fuel_mass: i32,
    }

    pub fn calculate_sum_of_fuel_req(filename: &str) -> i32 {
        read_file(filename).lines().map(|x| {
            let mut ship = ShipModule::new(x.parse::<i32>().unwrap());
            ship.recursive_fuel_req();
            ship.module_fuel_mass
        }).sum()
    }

    fn calculate_fuel_req_2(x: i32) -> i32 {
        let fuel = (x / 3) as i32 - 2;
       if  fuel > 0 {
          fuel
       } else {
           0
       }
    }

    impl ShipModule {
        pub fn new(module_mass: i32) -> ShipModule {
            let mass = calculate_fuel_req_2(module_mass);
            ShipModule {
                module_mass,
                module_fuel_mass: mass,
                last_fuel_mass: mass,
            }

        }

        pub fn recursive_fuel_req(&mut self) {
            let next_fuel_mass = calculate_fuel_req_2(self.last_fuel_mass);
            self.module_fuel_mass += next_fuel_mass;
            self.last_fuel_mass = next_fuel_mass;

            if self.last_fuel_mass > 0 {
                self.recursive_fuel_req();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_1_2;
    use crate::day_1::day_1_2::ShipModule;

    #[test]
    fn test_fuel_req() {
        assert_eq!(654, day_1_1::calculate_fuel_req(1969));
        assert_eq!(33583, day_1_1::calculate_fuel_req(100756))
    }

    #[test]
    fn test_sum_fuel_req() {
        assert_eq!(34237, day_1_1::calculate_sum_of_fuel_req("inputs/day_1_test.txt"));
    }

    #[test]
    fn test_recursive_fuel_req() {
        let mut ship_mod = ShipModule::new(100756);
        ship_mod.recursive_fuel_req();
        assert_eq!(50346, ship_mod.module_fuel_mass);

        let mut ship_mod = ShipModule::new(1969);
        ship_mod.recursive_fuel_req();
        assert_eq!(966, ship_mod.module_fuel_mass);
    }
}