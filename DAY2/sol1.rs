use {std::collections::HashMap, std::fs};

fn main() {
    let file_path = "inp.txt";
    let contents = fs::read_to_string(file_path).expect("File not read");

    let content = contents.split("\n");

    let mut id = 1;
    let mut ans = 0;
    for line in content {

        let pick = line.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();
        println!("{:?}", pick);

        let mut flag = true;
        for p in pick {
            let cube = p.split(",").collect::<Vec<&str>>();
            for val in cube {
                let n_c = val.split_whitespace().collect::<Vec<&str>>();
                // println!("{:?}", n_c);
                let n = n_c[0].parse::<i32>().unwrap_or(0);
                let c = n_c[1];
                if c == "red" && n > 12 {
                    flag = false;
                    println!("red violates");
                } else if c == "green" && n > 13 {
                    flag = false;
                    println!("green violates");
                } else if c == "blue" && n > 14 {
                    flag = false;
                    println!("blue violates");
                }
            }
        }
        if flag {
            ans += id;
        }
        id += 1;
    }
    println!("{}", ans);
}
