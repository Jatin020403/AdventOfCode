fn main() {
    let contents = include_str!("./inp.txt");

    let mut arr: Vec<Vec<char>> = vec![];
    let mut ch_arr: Vec<Vec<i32>> = vec![];
    let content = contents.split("\n").collect::<Vec<&str>>();

    for l in content {
        let line: Vec<char> = l.chars().collect();
        arr.push(line.clone());
        ch_arr.push(vec![0; line.len()]);
    }

    for (i, line) in arr.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if !ch.is_numeric() && ch != &'.' {
                if i > 0 {
                    if j > 0 {
                        ch_arr[i - 1][j - 1] = 1;
                    }
                    if j < line.len() - 1 {
                        ch_arr[i - 1][j + 1] = 1;
                    }
                    ch_arr[i - 1][j] = 1;
                }
                if i < arr.len() - 1 {
                    if j > 0 {
                        ch_arr[i + 1][j - 1] = 1;
                    }
                    if j < line.len() - 1 {
                        ch_arr[i + 1][j + 1] = 1;
                    }
                    ch_arr[i + 1][j] = 1;
                }
                if j > 0 {
                    ch_arr[i][j - 1] = 1;
                }
                if j < line.len() - 1 {
                    ch_arr[i][j + 1] = 1;
                }
            }
        }
    }

    for (i, line) in arr.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if ch.is_numeric() && ch_arr[i][j] == 1 {
                ch_arr = mark_2(&i, &j, &arr, ch_arr);
            }
        }
    }

    let mut sum = 0;

    for (i, line) in arr.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            if ch_arr[i][j] == 2 {
                let mut num: String = "0".to_string();
                while j < line.len() && ch_arr[i][j] == 2 {
                    num = num + &arr[i][j].to_string();
                    j += 1;
                }
                sum += num.parse::<i32>().unwrap_or(0);
                // println!("{}", num);
            } else {
                j += 1;
            }
        }
    }

    for line in arr {
        println!("{:?}", line);
    }
    for line in ch_arr {
        println!("{:?}", line);
    }

    println!("{}", sum)
}

fn mark_2(i: &usize, j: &usize, arr: &Vec<Vec<char>>, mut ch_arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let c = i.clone();
    let mut r = j.clone();

    while arr[c][r].is_numeric() {
        ch_arr[c][r] = 2;
        if r == 0 {
            break;
        }
        r -= 1;
    }
    r = j.clone();
    while r < arr[c].len() && arr[c][r].is_numeric() {
        ch_arr[c][r] = 2;
        r += 1;
    }
    ch_arr
}
