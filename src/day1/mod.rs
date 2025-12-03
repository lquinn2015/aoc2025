#[cfg(test)]
mod test {

    use crate::io;

    #[test]
    fn test() {
        //let test = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let test = "9191896883-9191940271,457499-518693,4952-6512,960-1219,882220-1039699,2694-3465,3818-4790,166124487-166225167,759713819-759869448,4821434-4881387,7271-9983,1182154-1266413,810784-881078,802-958,1288-1491,45169-59445,25035-29864,379542-433637,287-398,75872077-75913335,653953-689335,168872-217692,91-113,475-590,592-770,310876-346156,2214325-2229214,85977-112721,51466993-51620441,8838997-8982991,534003-610353,32397-42770,17-27,68666227-68701396,1826294188-1826476065,1649-2195,141065204-141208529,7437352-7611438,10216-13989,33-44,1-16,49-74,60646-73921,701379-808878";
        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_1(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);
        let count = input.read_u64();
        assert_eq!(5398419778u64, count);

        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_2(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);

        let count = input.read_u64();
        assert_eq!(15704845910u64, count);
    }
}

use crate::io::input::*;
use crate::io::output::*;
use crate::io::string::str::StrReader;

fn solve_2(is: &mut Input, os: &mut Output) {
    let data = is.read_str();
    let s = data.as_slice();

    let mut count = 0;
    for x in s.split(|&b| b == b',') {
        let mut iter = x
            .split(|&b| b == b'-')
            .map(|x| String::from_utf8(x.to_vec()))
            .map(|x| x.unwrap().parse::<u64>().unwrap());

        let (lo, hi) = (iter.next().unwrap(), iter.next().unwrap());
        for (x, s) in (lo..=hi).map(|x| (x, format!("{x}"))) {
            let mut valid_sub = false;
            for subseq_len in (1..s.len()).filter(|l| s.len() % l == 0) {
                let mut sub_c = true;
                let sub = &s[0..subseq_len];
                let mut cursor = &s[subseq_len..];
                while !cursor.is_empty() {
                    let b = &cursor[..subseq_len];
                    cursor = &cursor[subseq_len..];
                    if sub != b {
                        sub_c = false;
                        break;
                    }
                }

                if sub_c {
                    //println!("{sub} in {s}");
                    valid_sub = true;
                    break;
                }
            }
            if valid_sub {
                count += x;
            }
        }
    }
    os.print_line(count);
}

fn solve_1(is: &mut Input, os: &mut Output) {
    let data = is.read_str();
    let s = data.as_slice();

    let mut count = 0;
    for x in s.split(|&b| b == b',') {
        let mut iter = x
            .split(|&b| b == b'-')
            .map(|x| String::from_utf8(x.to_vec()))
            .map(|x| x.unwrap().parse::<u64>().unwrap());

        let (lo, hi) = (iter.next().unwrap(), iter.next().unwrap());
        for (x, s) in (lo..=hi)
            .map(|x| (x, format!("{x}")))
            .filter(|(_n, s)| s.len() % 2 != 1)
        {
            let m = s.len() >> 1;
            if s[0..m] == s[m..] {
                count += x;
            }
        }
    }
    os.print_line(count);
}
