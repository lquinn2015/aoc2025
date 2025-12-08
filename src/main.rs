use crate::io::input::*;
use crate::io::output::*;
use crate::io::string::str::StrReader;

mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod io;

fn main() {
    //let test = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let test = "123 328  51 64 
    45 64  387 23 
    6 98  215 314
*   +   *   + ";
    //let mut input = io::input::Input::slice(test.as_bytes());
    let mut input = io::input::Input::stdin(); //slice(test.as_bytes());
    let mut output = io::output::Output::stdout();

    //solve_1(&mut input, &mut output); // 357
    solve_2(&mut input, &mut output); // 3121910778619
    output.flush();
}

fn convert_line(line: String) -> Vec<i128> {
    line.split_whitespace()
        .map(|s| i128::from_str_radix(s, 10).unwrap())
        .collect()
}

fn convert_tline(line: Vec<String>) -> Vec<i128> {
    let mut nums = vec![];

    for dstr in line {
        while nums.len() < dstr.len() {
            nums.push(0);
        }

        for (idx, d) in dstr
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i128)
            .enumerate()
        {
            nums[idx] = nums[idx] * 10 + d;
        }
    }

    nums
}

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

fn parse_nums(lines: Vec<String>, dpc: &Vec<usize>) -> Vec<Vec<i128>> {
    let mut s = 0;
    let mut e = 0;

    for (idx, dc) in dpc.iter().enumerate() {
        s = e;
        e = s + dc;

        println!("dp: {idx}");
        lines.iter().enumerate().for_each(|(idx, l)| {
            println!("slice[{s}..{e}], l.len = {}: {l}", l.len());
            let tnum = &l.as_str()[s..e];
            println!("line{idx}: {tnum}");
        });

        e += 1;
    }

    vec![]
}

fn solve_2(is: &mut Input, os: &mut Output) {
    let mut lines = is.read_lines();
    let ops_s = String::from_utf8(lines.pop().unwrap().to_vec()).unwrap();

    let dpc = get_dpc(&ops_s);
    println!("{dpc:?}");

    let nums = parse_nums(
        lines
            .into_iter()
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
            .collect(),
        &dpc,
    );

    println!("{nums:?}");

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
            println!("Op{idx} = {c} , subtotal: {x}");

            acc + x
        });

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

    println!("{nums:?}");

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
            println!("Op{idx} = {c} , subtotal: {x}");

            acc + x
        });

    os.print_line(total);
}
