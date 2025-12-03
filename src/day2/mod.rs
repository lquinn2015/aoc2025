#[cfg(test)]
mod test {

    use crate::io;

    #[test]
    fn test() {
        //let test = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let test = "987654321111111
811111111111119
234234234234278
818181911112111";
        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_1(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);
        let count = input.read_u64();
        assert_eq!(357, count);

        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_2(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);

        let count = input.read_u64();
        assert_eq!(3121910778619u64, count);
    }
}

use crate::io::input::*;
use crate::io::output::*;
use crate::io::string::str::StrReader;

fn solve_2(is: &mut Input, os: &mut Output) {
    let mut count = 0;
    loop {
        let line = String::from_utf8(is.read_str().to_vec()).unwrap();
        if line.is_empty() {
            break;
        }

        let max_seq = |(midx, mc), (idx, c)| {
            let d = (c as u8) - b'0';
            if d >= mc {
                (idx, d)
            } else {
                (midx, mc)
            }
        };

        let mut s = line.as_str();
        let mut sub_sum = 0;
        let mut n = 12;
        while !s.is_empty() {
            let (bidx, d) = s
                .chars()
                .into_iter()
                .rev()
                .enumerate()
                .skip(n - 1)
                .fold((0, 0), max_seq);

            println!("sub_Seq: {s}, b_idx: {bidx}, nex_c {d}, n: {n}");

            if d == 0 {
                let sum = s.chars().into_iter().fold((0, n), |(acc, n), c| {
                    let d = (c as u8 - b'0') as u64;
                    (d * u64::pow(10u64, (n - 1) as u32) + acc, n - 1)
                });

                sub_sum += sum.0;
                break;
            }

            if n == 1 {
                let last_c = s.chars().fold(0, |max_c, c| u8::max(c as u8 - b'0', max_c)) as u64;
                sub_sum += last_c;
                break;
            } else {
                let fwd_idx = s.len() - bidx - 1;
                let d = d as u64;
                sub_sum += d * u64::pow(10u64, (n - 1) as u32);

                s = &s[fwd_idx + 1..];
                n -= 1;
            }
        }
        println!("{line}: sub_sum: {sub_sum}");
        count += sub_sum;
    }

    os.print_line(count);
}

fn max_sum(s: &str, n: usize) -> u64 {
    // if we don't have enough digits bail 12 is always better
    if s.len() < n || n == 0 {
        return 0;
    }

    let n = n;

    let d = ((s.chars().next().unwrap() as u8) - b'0') as u64;
    let dexp = d * u64::pow(10, (n - 1) as u32);
    u64::max(dexp + max_sum(&s[1..], n - 1), max_sum(&s[1..], n))
}

fn solve_1(is: &mut Input, os: &mut Output) {
    let mut count = 0;

    loop {
        let line = String::from_utf8(is.read_str().to_vec()).unwrap();
        if line.is_empty() {
            break;
        }
        let max_seq = |(midx, mc), (idx, c)| {
            let d = (c as u8) - b'0';
            if d >= mc {
                (idx, d)
            } else {
                (midx, mc)
            }
        };

        let mut iter = line.chars().into_iter().rev().enumerate();
        iter.next();

        let (back_idx, lead_d) = iter.fold((0, 0), max_seq);
        let fwd_idx = line.len() - back_idx - 1;
        let mut iter = line[fwd_idx..].chars().into_iter().enumerate();
        iter.next();
        let (_trail, nd) = iter.fold((0, 0), max_seq);

        /*
        println!("line: {line}");
        println!("s: {fwd_idx}, e: {_trail}");
        println!(
            "sub_line:  {}, fd: {}, sd:{}",
            &line[fwd_idx..][.._trail + 1],
            lead_d as usize * 10,
            nd
        );
        */
        count += lead_d as usize * 10 + nd as usize;
    }
    os.print_line(count);
}
