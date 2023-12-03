use {std::collections::HashMap, std::fs};

fn main() {
    let file_path = "inp.txt";
    let contents = fs::read_to_string(file_path).expect("File not read");

    let content = contents.split("\n");

    let mut ans = 0;
    for line in content {
        let mut counts = HashMap::new();

        let pick = line.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>();
        // println!("{:?}", pick);

        for p in pick {
            let cube = p.split(",").collect::<Vec<&str>>();
            for val in cube {
                let n_c = val.split_whitespace().collect::<Vec<&str>>();
                // println!("{:?}", n_c);
                let temp = n_c[0].parse::<i32>().unwrap_or(0);
                let n = std::cmp::max(counts.get(n_c[1]).unwrap_or(&0), &temp);
                counts.insert(n_c[1], *n);
            }
        }
        let mut mul = 1;
        for (_, val) in &counts {
            mul *= val;
        }
        ans += mul;
    }
    println!("{}", ans);
}
