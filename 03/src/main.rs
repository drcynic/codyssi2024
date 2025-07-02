fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let p1: i32 = input
        .trim()
        .lines()
        .map(|l| l.split_once(" ").unwrap().1.parse::<i32>().unwrap())
        .sum();
    println!("part1: {}", p1);

    let p2 = input
        .trim()
        .lines()
        .map(|l| {
            let (vs, bs) = l.split_once(" ").unwrap();
            let radix = bs.parse::<u32>().unwrap();
            u64::from_str_radix(vs, radix).unwrap()
        })
        .sum();
    println!("part2: {}", p2);

    println!("part3 {:#}", to_base_65(p2));
}

fn to_base_65(mut x: u64) -> String {
    let radix: u64 = 65;
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#";
    let mut result = vec![];

    loop {
        let d = x % radix;
        x = x / radix;
        result.push(alphabet.chars().nth(d as usize).unwrap());

        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
