#[cfg(test)]
mod tests {
    use crate::io;

    #[test]
    fn test() {
        let test = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_1(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);
        let count = input.read_i128();
        assert_eq!(4277556, count);

        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_2(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);

        let count = input.read_i128();
        assert_eq!(3263827, count);
    }
}

use crate::io::input::*;
use crate::io::output::*;
use crate::io::string::str::StrReader;

fn get_dpc(ops: &String) -> Vec<usize> {
    let mut dpc = vec![];

    let mut cur = ops.chars().rev();
    let mut c_dpc = 0;
    while let Some(c) = cur.next() {
        c_dpc += 1;
        match c {
            '*' | '+' => {
                dpc.push(c_dpc);
                cur.next();
                c_dpc = 0;
            }
            _ => {}
        }
    }

    dpc.into_iter().rev().collect()
}

fn parse_nums(lines: Vec<String>, dpc: &Vec<usize>, ops_s: Vec<String>) -> i128 {
    let mut grid: Vec<Vec<String>> = vec![];
    let mut s = 0;
    let mut e = 0;

    for (idx, dc) in dpc.iter().enumerate() {
        s = e;
        e = s + dc;

        let mut arr = vec![];
        lines.iter().enumerate().for_each(|(idx, l)| {
            let tnum = &l.as_str()[s..e];
            arr.push(format!("{tnum}"));
        });

        e += 1;

        grid.push(arr);
    }

    let mut total = 0;
    for ((row, dp), op) in dpc.iter().enumerate().zip(ops_s) {
        let row = &grid[row];
        let mut nums = vec![0; *dp];
        row.iter().for_each(|s| {
            s.chars().enumerate().for_each(|(idx, c)| {
                let add = match c {
                    ' ' => 0,
                    _ => c.to_digit(10).unwrap() as i128,
                };
                if add != 0 {
                    nums[idx] = nums[idx] * 10 + add;
                }
            });
        });

        let op = match op.chars().next() {
            Some('+') => |a, b| a + b,
            Some('*') => |a, b| a * b,
            _ => unreachable!(),
        };

        let row_total = nums.iter().skip(1).fold(nums[0], |acc, v| op(acc, *v));
        total += row_total;
    }

    total
}

fn solve_2(is: &mut Input, os: &mut Output) {
    let mut lines = is.read_lines();
    let ops_s = String::from_utf8(lines.pop().unwrap().to_vec()).unwrap();

    let dpc = get_dpc(&ops_s);
    let ops: Vec<String> = ops_s.split_whitespace().map(|x| x.to_owned()).collect();

    let total = parse_nums(
        lines
            .into_iter()
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
            .collect(),
        &dpc,
        ops,
    );

    os.print_line(total);
}

fn solve_1(is: &mut Input, os: &mut Output) {
    let mut lines = is.read_lines();
    let ops_s = String::from_utf8(lines.pop().unwrap().to_vec()).unwrap();

    let nums: Vec<Vec<i128>> = lines
        .into_iter()
        .map(|line| {
            String::from_utf8(line.to_vec())
                .unwrap()
                .split_whitespace()
                .map(|s| i128::from_str_radix(s, 10).unwrap())
                .collect()
        })
        .collect();

    let total = ops_s
        .split_whitespace()
        .enumerate()
        .fold(0, |acc, (idx, c)| {
            let mut x = nums[0][idx];
            let op = match c.as_bytes().iter().next().unwrap() {
                b'+' => |a, b| a + b,
                b'*' => |a, b| a * b,
                _ => unreachable!(),
            };

            for i in 1..nums.len() {
                x = op(x, nums[i][idx]);
            }

            acc + x
        });

    os.print_line(total);
}
