mod tests {
    use crate::io;

    #[test]
    fn test() {
        let test = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_1(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);
        let count = input.read_u64();
        assert_eq!(21, count);

        let mut input = io::input::Input::slice(test.as_bytes());
        let mut obuf: Vec<u8> = vec![];
        let mut output = io::output::Output::buf(&mut obuf);
        super::solve_2(&mut input, &mut output);
        output.flush();

        let mut input = io::input::Input::slice(&obuf);

        let count = input.read_u64();
        assert_eq!(40, count);
    }
}

use crate::io::input::*;
use crate::io::output::*;
use crate::io::string::str::StrReader;

struct NeighborIter<'a, T> {
    graph: &'a Graph<T>,
    center: (isize, isize),
    mode: isize,
}

struct Graph<T> {
    graph: Vec<Vec<T>>,
}

impl<'a, T> NeighborIter<'a, T> {
    fn new(center: (usize, usize), graph: &'a Graph<T>) -> NeighborIter<'a, T> {
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
        let max_y = self.graph.graph.len();
        let max_x = self.graph.graph[0].len();

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

#[derive(Copy, Clone)]
enum Symbol {
    Start,
    Beam(u128),
    Splitter,
    Empty,
}

impl std::fmt::Display for Graph<Symbol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.graph.len() {
            for col in 0..self.graph[0].len() {
                let _ = match self.graph[row][col] {
                    Symbol::Start => write!(f, "  S  "),
                    Symbol::Beam(val) => {
                        write!(f, "{val:^10}")
                    }
                    Symbol::Splitter => write!(f, "  ^  "),
                    Symbol::Empty => write!(f, "  .  "),
                };
            }
            let _ = writeln!(f, "");
        }

        write!(f, "")
    }
}

fn solve_2(is: &mut Input, os: &mut Output) {
    let mut graph: Vec<Vec<Symbol>> = vec![];

    // build graph
    loop {
        let line = String::from_utf8(is.read_str().to_vec()).unwrap();
        if line.is_empty() {
            break;
        }

        let row = line
            .chars()
            .into_iter()
            .map(|c| match c {
                'S' => Symbol::Start,
                '|' => Symbol::Beam(0),
                '.' => Symbol::Empty,
                '^' => Symbol::Splitter,
                _ => unreachable!(),
            })
            .collect();
        graph.push(row);
    }

    let mut graph = Graph { graph };

    for row in 0..(graph.graph.len() - 1) {
        for col in 0..graph.graph[0].len() {
            match graph.graph[row][col] {
                Symbol::Beam(_) | Symbol::Start => match graph.graph[row + 1][col] {
                    Symbol::Empty => {
                        graph.graph[row + 1][col] = Symbol::Beam(1);
                    }
                    Symbol::Splitter => {
                        graph.graph[row + 1][col - 1] = Symbol::Beam(1);
                        graph.graph[row + 1][col + 1] = Symbol::Beam(1);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    println!("{graph}");

    let mut start_val = 0;
    for row in (0..(graph.graph.len() - 1)).rev() {
        println!("test?");
        // beam prop
        for col in 0..graph.graph[0].len() {
            if let Symbol::Beam(_) = graph.graph[row][col] {
                if let Symbol::Beam(lb) = graph.graph[row + 1][col] {
                    graph.graph[row][col] = Symbol::Beam(lb);
                }
            }

            if let Symbol::Start = graph.graph[row][col] {
                if let Symbol::Beam(total) = graph.graph[row + 1][col] {
                    start_val = total;
                }
            }
        }

        println!("{graph}");

        // split converge
        for col in 0..graph.graph[0].len() {
            if let Symbol::Splitter = graph.graph[row][col] {
                let left = if let Symbol::Beam(lb) = graph.graph[row][col - 1] {
                    lb
                } else {
                    0
                };
                let right = if let Symbol::Beam(rb) = graph.graph[row][col + 1] {
                    rb
                } else {
                    0
                };
                println!("hit at [{row}][{col}] = {left}+{right}={}", left + right);

                graph.graph[row][col] = Symbol::Beam(left + right);
            }
        }
        println!("{graph}");
    }

    os.print_line(start_val);
}

fn solve_1(is: &mut Input, os: &mut Output) {
    let mut graph: Vec<Vec<Symbol>> = vec![];

    // build graph
    loop {
        let line = String::from_utf8(is.read_str().to_vec()).unwrap();
        if line.is_empty() {
            break;
        }

        let row = line
            .chars()
            .into_iter()
            .map(|c| match c {
                'S' => Symbol::Start,
                '|' => Symbol::Beam(0),
                '.' => Symbol::Empty,
                '^' => Symbol::Splitter,
                _ => unreachable!(),
            })
            .collect();
        graph.push(row);
    }

    let mut graph = Graph { graph };

    let mut count = 0;
    for row in 0..(graph.graph.len() - 1) {
        for col in 0..graph.graph[0].len() {
            match graph.graph[row][col] {
                Symbol::Beam(_) | Symbol::Start => match graph.graph[row + 1][col] {
                    Symbol::Empty => {
                        graph.graph[row + 1][col] = Symbol::Beam(1);
                    }
                    Symbol::Splitter => {
                        count += 1;
                        graph.graph[row + 1][col - 1] = Symbol::Beam(1);
                        graph.graph[row + 1][col + 1] = Symbol::Beam(1);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        println!("{graph}");
    }

    os.print_line(count);
}
