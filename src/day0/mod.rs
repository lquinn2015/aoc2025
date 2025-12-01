#[cfg(test)]
mod test {
    use crate::io::{input, output};

    #[test]
    fn solve_1() {
        let buf = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let input = input::Input::slice(buf.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let out = output::Output::buf(&mut obuf);
        super::solve_1(input, out);
        let mut input = input::Input::slice(&obuf);
        let count = input.read_u32();

        assert_eq!(3, count);

        let input = input::Input::slice(buf.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let out = output::Output::buf(&mut obuf);
        super::solve_2(input, out);
        let mut input = input::Input::slice(&obuf);
        let count = input.read_u32();

        assert_eq!(6, count);
    }
}

use crate::io::input::*;
use crate::io::output::*;

fn solve_2(mut input: Input, mut output: Output) {
    let mut count = 0;
    let mut dial = 50;
    while !input.is_empty() {
        let dir = match input.read_char() {
            b'L' => -1,
            b'R' => 1,
            _ => unreachable!(),
        };
        let dist = input.read_i32();

        // count times we do full spins;
        let spins = dist / 100;

        // range [-99,99]
        let dist = (dist % 100) * dir;

        let cross = if dial != 0 && ((dist + dial) < 0 || (dist + dial) > 100) {
            1
        } else {
            0
        };

        dial = (dist + dial + 100) % 100;
        let exact = if dial == 0 { 1 } else { 0 };

        count += spins + cross + exact;
    }

    output.print_line(count);
    output.flush();
}

fn solve_1(mut input: Input, mut output: Output) {
    let mut count = 0;
    let mut dial = 50;
    while !input.is_empty() {
        let dir = match input.read_char() {
            b'L' => -1,
            b'R' => 1,
            _ => unreachable!(),
        };
        let dist = input.read_i32() % 100;

        dial = (dial + (dist * dir + 100)) % 100;
        if dial == 0 {
            count += 1;
        }
    }

    output.print_line(count);
    output.flush();
}
