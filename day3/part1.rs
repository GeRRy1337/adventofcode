use std::fs;

fn main() {
    let nums=1..=52;
    let chars=('a'..='z').chain('A'..='Z');
    let zipped = chars.into_iter().zip(nums.into_iter());

    let contents = fs::read_to_string("/home/gerry/Desktop/Rust/aoc/adventofcode/day3/day3.txt").expect("");

    let mut sum=0;
    for line in contents.lines(){

        println!("{} {} / {}", line.len(), &line[..line.len()/2], &line[line.len()/2..]);
        let mut booth: char = 'a';
        for c in (&line[..line.len()/2]).chars() {
            if (&line[line.len()/2..]).contains(c) {
                booth=c;
                break
            }
        }
        for (c, i) in zipped.clone()
        {
            if c == booth {
                sum+=i;
            }
        }
    }
    println!("{}", sum);

}
