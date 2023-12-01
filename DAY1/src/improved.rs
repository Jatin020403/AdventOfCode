use std::fs;

fn main() {
    const FILE_PATH: &str = "inp.txt";

    let contents = fs::read_to_string(FILE_PATH).expect("File not read");

    let content = contents.split("\n");

    let mut sum = 0;

    for line in content {
        let v = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut str_min_idx = usize::MAX;
        let mut str_max_idx = usize::MIN;
        let mut lnum: String = "".to_string();
        let mut rnum: String = "".to_string();

        for &num in &v {
            if let Some(x) = line.find(num) {
                str_min_idx = std::cmp::min(str_min_idx, x);
                if str_min_idx == x {
                    lnum = (v.iter().position(|&x| x == num).unwrap_or(0) + 1).to_string();
                }
            }
        }

        for &num in &v {
            if let Some(x) = line.rfind(num) {
                str_max_idx = std::cmp::max(str_max_idx, x);
                if str_max_idx == x {
                    rnum = (v.iter().position(|&x| x == num).unwrap_or(0) + 1).to_string();
                }
            }
        }

        for (idx, ch) in line.char_indices() {
            if ch.is_numeric() {
                str_min_idx = std::cmp::min(str_min_idx, idx);
                if str_min_idx == idx {
                    lnum = ch.to_string()
                }
                break;
            }
        }

        // Got first number
        // Now second number

        for (idx, ch) in line.to_string().char_indices().rev() {
            if ch.is_numeric() {
                str_max_idx = std::cmp::max(str_max_idx, idx);
                if str_max_idx == idx {
                    rnum = ch.to_string()
                }
                break;
            }
        }

        let str_num = lnum + &rnum;

        // println!("\n{}\n", str_num);

        let int_val: i32 = str_num.parse().unwrap_or(-1);

        sum += int_val;
    }

    println!("{}\n", sum);
}
