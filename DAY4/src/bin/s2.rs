fn main() {
    let contents = include_str!("./test_inp.txt");

    // println!("{}", contents);

    let content = contents.split("\n").collect::<Vec<&str>>();
    // println!("{:?}", content);

    let mut ans = 0;
    let mut arr = vec![0; content.len()];
    for (i, line) in content.iter().enumerate() {
        // ans = recurse(i, &content, ans);

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
                    arr[i] += 1;
                    // println!("{}", i + iter);
                }
            }
        }
    }

    for (i, line) in content.iter().enumerate() {
        println!("{}", arr[i]);
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

        let mut iter = 0;
        for n in win_no {
            if elf_no.contains(&n) {
                {
                    iter += 1;
                    ans += arr[i] * arr[i + iter];
                }
            }
        }
        if iter == 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn recurse(i: usize, content: &Vec<&str>, arr: &Vec<i32>, mut ans: i32) -> i32 {
    let line = content[i];
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
    let mut iter = 0;
    for n in win_no {
        if elf_no.contains(&n) {
            {
                iter += 1;
                ans = recurse(i + iter, content, arr, ans);
                println!("{}", i + iter);
            }
        }
    }
    // println!("{}", ans);
    ans + 1
}
