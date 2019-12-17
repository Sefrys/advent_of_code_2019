pub mod intcode {
    use std::iter::*;
    use crate::reader::read_file;

    pub fn calculate_intcode(filename: &str) -> usize {
        let mut vec = get_input_as_vec(filename);

        vec[1] = 12;
        vec[2] = 2;

        traverse(vec)[0]
    }

    pub fn calculate_pairs(filename: &str) -> i32 {
        for i in 0..=99 {
            for j in 0..=99 {
                let mut vec = get_input_as_vec(filename);
                vec[1] = i;
                vec[2] = j;

                if traverse(vec)[0] == 19690720 {
                    return ((100 * i) + j) as i32;
                }
            }
        }
        0
    }


    fn traverse(mut vec: Vec<usize>) -> Vec<usize> {
        let mut start = 0;

        loop {
            let repl_idx;
            match vec[start] {
                1 => {
                    let tmp = vec[vec[start + 1]] + vec[vec[start + 2]];
                    repl_idx = vec[start + 3];
                    vec[repl_idx] = tmp;
                }
                2 => {
                    let tmp = vec[vec[start + 1]] * vec[vec[start + 2]];
                    repl_idx = vec[start + 3];
                    vec[repl_idx] = tmp;
                }
                99 => break,
                _ => panic!("Unexpected value: {}", vec[start])
            }

            start += 4;
        }
        vec
    }

    fn get_input_as_vec(filename: &str) -> Vec<usize> {
        read_file(filename).split(',').map(|x| x.parse::<usize>().unwrap()).collect()
    }


    #[cfg(test)]
    mod intcode_tests {
        use super::*;

        #[test]
        fn input_as_vec_test() {
            assert_eq!(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], get_input_as_vec("inputs/day_2_test.txt"));
        }

        #[test]
        fn intcode_test_1() {
            assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], traverse(get_input_as_vec("inputs/day_2_test.txt")));
        }

        #[test]
        fn intcode_test_2() {
            assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], traverse(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]));
        }
    }
}

