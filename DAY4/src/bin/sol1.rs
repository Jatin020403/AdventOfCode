fn main() {
    let contents = include_str!("./inp.txt");

    // println!("{}", contents);

    let content = contents.split("\n").collect::<Vec<&str>>();
    // println!("{:?}", content);

    let mut ans = 0;
    for line in content {
        let mut count = 0;

        // let mut counts = 0;

        let pick = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>();

        let win_no = pick[0]
            .split_whitespace()
            .map(|i| i.trim())
            .collect::<Vec<&str>>();

        // println!("{:?}", win_no);

        let elf_no = pick[1]
            .split_whitespace()
            .map(|i| i.trim())
            .collect::<Vec<&str>>();

        // println!("{:?}", elf_no);

        for n in win_no {
            if elf_no.contains(&n) {
                {
                    count += 1;
                }
            }
        }
        if count != 0 {
            ans += 2i32.pow(count - 1);
        }
    }
    println!("{}", ans);
}
