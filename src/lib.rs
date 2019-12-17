pub mod day_1;
pub mod day_2;


pub mod reader {
    use std::fs;

    pub fn read_file(filename: &str) -> String {
        fs::read_to_string(filename).unwrap()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file() {
        let str = reader::read_file("test.txt");
        assert_eq!("test_1", str.lines().nth(0).unwrap());
    }
}

