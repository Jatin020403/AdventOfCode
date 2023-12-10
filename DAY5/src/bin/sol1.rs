fn main() {
    let contents = include_str!("./inp.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let seed = &(contents[0]
        .split_whitespace()
        .map(|i| i.trim())
        .collect::<Vec<&str>>())[1..];

    // let conv = &contents[1..];

    let mut ans = i64::MAX;

    for num in seed {
        let mut n = num.parse().unwrap_or(0);
        for conv in &contents[1..] {
            let temp_map = &(conv.split("\n").collect::<Vec<&str>>())[1..];
            for map in temp_map {
                let m = map.split(" ").collect::<Vec<&str>>();
                let dest = m[0].parse().unwrap_or(0);
                let start = m[1].parse().unwrap_or(0);
                let ran = m[2].parse().unwrap_or(0);

                if n >= start && n < start + ran {
                    n = dest + (n - start);
                    // println!("{}:{}:{}->{}", dest, start, ran, n);
                    break;
                }
                // println!("->{}", n);
            }
        }
        println!("{}", n);
        ans = std::cmp::min(ans, n);
    }

    println!("ans is : {}", ans);

    // println!("{:?}", seed);
    // println!("{:?}", contents);
}
