#[cfg(test)]
mod tests {
    use crate::io;

    #[test]
    fn test() {
        let test = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_1(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);
        let count = input.read_u64();
        assert_eq!(3, count);

        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_2(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);

        let count = input.read_u64();
        assert_eq!(14, count);
    }
}

use crate::io::input::*;
use crate::io::output::*;
use crate::io::string::str::StrReader;

fn solve_2(is: &mut Input, os: &mut Output) {
    let mut segs = vec![];
    loop {
        let line = is.read_line().to_string();

        println!("\"{line}\"");
        if line.is_empty() {
            break;
        }
        let mut nums = line.split("-").map(|s| u64::from_str_radix(s, 10).unwrap());
        segs.push((nums.next().unwrap(), nums.next().unwrap()));
    }

    // sort by Left than right
    segs.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let mut segs_new = segs.clone();
    segs_new.clear();
    segs_new.push(segs[0]);
    let mut p1 = segs.iter().peekable().skip(1);
    while let Some(right) = p1.next() {
        let left = segs_new.last_mut().unwrap();
        // overlap
        if left.0 <= right.0 && right.0 <= left.1 {
            left.1 = u64::max(left.1, right.1);
        } else {
            segs_new.push(*right);
        }
    }

    println!("segs: {segs:?}");
    println!("segs_new: {segs_new:?}");

    let count = segs_new.iter().fold(0, |acc, iv| acc + iv.1 - iv.0 + 1);

    while !is.read_line().is_empty() {}
    os.print_line(count);
}

fn solve_1(is: &mut Input, os: &mut Output) {
    let mut segs = vec![];
    loop {
        let line = is.read_line().to_string();

        if line.is_empty() {
            break;
        }
        let mut nums = line.split("-").map(|s| u64::from_str_radix(s, 10).unwrap());
        segs.push((nums.next().unwrap(), nums.next().unwrap()));
    }

    let mut count = 0;
    loop {
        if is.is_empty() {
            break;
        }
        let a = is.read_u64();
        for s in segs.iter() {
            if a >= s.0 && a <= s.1 {
                count += 1;
                break;
            }
        }
    }

    os.print_line(count);
}
