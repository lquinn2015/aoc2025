#[cfg(test)]
mod test {

    use crate::io;

    #[test]
    fn test() {
        //let test = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let test = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_1(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);
        let count = input.read_u64();
        assert_eq!(13, count);

        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_2(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);

        let count = input.read_u64();
        assert_eq!(43, count);
    }
}

use crate::io::input::*;
use crate::io::output::*;
use crate::io::string::str::StrReader;

struct NeighborIter<'a, T> {
    graph: &'a Vec<Vec<T>>,
    center: (isize, isize),
    mode: isize,
}

impl<'a, T> NeighborIter<'a, T> {
    fn new(center: (usize, usize), graph: &'a Vec<Vec<T>>) -> NeighborIter<'a, T> {
        NeighborIter {
            graph,
            center: (center.0 as isize, center.1 as isize),
            mode: 0,
        }
    }
}

impl<T: Copy> Iterator for NeighborIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let max_y = self.graph.len();
        let max_x = self.graph[0].len();

        while self.mode < 9 {
            if self.mode == 4 {
                self.mode += 1;
            }
            let dx = (self.mode % 3) - 1;
            let dy = (self.mode / 3) - 1;
            self.mode += 1;

            let (cy, cx) = self.center;
            let (fy, fx) = (cy + dy, cx + dx);

            if fx < max_x as isize && fy < max_y as isize && fx >= 0 && fy >= 0 {
                return Some(
                    *self
                        .graph
                        .get(fy as usize)
                        .unwrap()
                        .get(fx as usize)
                        .unwrap(),
                );
            }
        }

        None
    }
}

fn solve_1(is: &mut Input, os: &mut Output) {
    let mut grid: Vec<Vec<u32>> = vec![];

    // build grid
    loop {
        let line = String::from_utf8(is.read_str().to_vec()).unwrap();
        if line.is_empty() {
            break;
        }

        let row = line
            .chars()
            .into_iter()
            .map(|c| match c {
                '.' => 0,
                '@' => 1,
                _ => 0,
            })
            .collect();
        grid.push(row);
    }

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let p = (y, x);
            let mut ncount = 0;
            if grid[y][x] == 0 {
                print!(".");
                continue;
            }
            for val in NeighborIter::new(p, &grid) {
                ncount += val;
            }
            if ncount < 4 {
                print!("x");
                count += 1;
            } else {
                print!("@");
            }
        }
        println!();
    }
    os.print_line(count);
}

fn solve_2(is: &mut Input, os: &mut Output) {
    let mut grid: Vec<Vec<u32>> = vec![];

    // build grid
    loop {
        let line = String::from_utf8(is.read_str().to_vec()).unwrap();
        if line.is_empty() {
            break;
        }

        let row = line
            .chars()
            .into_iter()
            .map(|c| match c {
                '.' => 0,
                '@' => 1,
                _ => 0,
            })
            .collect();
        grid.push(row);
    }

    let mut ocount = 0;
    loop {
        let mut count = 0;
        let mut next_grid = vec![];
        for y in 0..grid.len() {
            next_grid.push(vec![]);
            for x in 0..grid[y].len() {
                let p = (y, x);
                let mut ncount = 0;
                if grid[y][x] == 0 {
                    print!(".");
                    next_grid[y].push(0);
                    continue;
                }
                for val in NeighborIter::new(p, &grid) {
                    ncount += val;
                }
                if ncount < 4 {
                    print!("x");
                    next_grid[y].push(0);
                    count += 1;
                } else {
                    print!("@");
                    next_grid[y].push(1);
                }
            }
            println!();
        }
        println!();
        grid = next_grid;
        ocount += count;
        if count == 0 {
            break;
        }
    }

    os.print_line(ocount);
}
