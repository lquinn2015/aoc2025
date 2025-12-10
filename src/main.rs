use crate::io::input::*;
use crate::io::output::*;
use crate::io::string::str::StrReader;

mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod io;

fn main() {
    //let test = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let test = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    //let mut input = io::input::Input::slice(test.as_bytes());
    let mut input = io::input::Input::stdin(); //slice(test.as_bytes());
    let mut output = io::output::Output::stdout();

    solve_2(&mut input, &mut output); // 357
                                      //solve_2(&mut input, &mut output); // 3121910778619
    output.flush();
}

#[derive(Debug)]
struct Node {
    loc: (isize, isize, isize),
    celf: usize,
    adj: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    src: usize,
    dst: usize,
    dist: usize,
}

fn dist(i: usize, j: usize, graph: &Vec<Node>) -> usize {
    let a = &graph[i];
    let b = &graph[j];
    (isize::pow(a.loc.0 - b.loc.0, 2)
        + isize::pow(a.loc.1 - b.loc.1, 2)
        + isize::pow(a.loc.2 - b.loc.2, 2)) as usize
}

fn build_graph(is: &mut Input) -> (Vec<Node>, Vec<Edge>) {
    let mut vertices: Vec<Node> = vec![];
    let mut edges: Vec<Edge> = vec![];

    loop {
        let line = is.read_line();
        if line.is_empty() {
            break;
        }

        let mut iter = line
            .split(|&c| c == b',')
            .map(|s| isize::from_str_radix(&String::from_utf8(s.to_vec()).unwrap(), 10).unwrap());
        let loc = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );
        let celf = vertices.len();
        vertices.push(Node {
            loc,
            adj: vec![],
            celf,
        });
    }

    let n = vertices.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let src = i;
            let dst = j;
            let dist = dist(src, dst, &vertices);

            edges.push(Edge { src, dst, dist });
            //vertices[src].adj.push(dst);
            //vertices[dst].adj.push(src);
        }
    }

    edges.sort_by(|a, b| a.dist.cmp(&b.dist));

    (vertices, edges)
}

fn solve_2(is: &mut Input, os: &mut Output) {
    let (mut vertices, mut edges) = build_graph(is);

    let n = vertices.len();
    println!("n  is {n}");

    let mut v2c: Vec<usize> = vec![0; n];
    let mut components: Vec<Vec<usize>> = vec![vec![]];
    for e in edges.iter() {
        if v2c[e.src] == 0 && v2c[e.dst] == 0 {
            v2c[e.src] = components.len();
            v2c[e.dst] = components.len();
            components.push(vec![e.src, e.dst]);
        } else if v2c[e.src] == 0 && v2c[e.dst] != 0 {
            components[v2c[e.dst]].push(e.src);
            v2c[e.src] = v2c[e.dst];
        } else if v2c[e.src] != 0 && v2c[e.dst] == 0 {
            components[v2c[e.src]].push(e.dst);
            v2c[e.dst] = v2c[e.src];
        } else if v2c[e.src] != v2c[e.dst] {
            // both components must be merged
            let merge_to = v2c[e.dst];

            let merge_from: Vec<usize> = components[v2c[e.src]].iter().copied().collect();
            components[v2c[e.src]] = vec![];
            v2c[e.src] = merge_to;

            for &v in merge_from.iter() {
                components[merge_to].push(v);
                v2c[v] = merge_to;
            }
        }

        if components[v2c[e.src]].len() == n {
            let total = vertices[e.src].loc.0 as usize * vertices[e.dst].loc.0 as usize;
            println!(
                "last edge {:?}, {:?},{:?}",
                e, vertices[e.src], vertices[e.dst]
            );
            os.print_line(total);
            println!("{v2c:?}");
            return;
        }
    }
}

fn solve_1(is: &mut Input, os: &mut Output) {
    let (mut vertices, mut edges) = build_graph(is);
    //println!("{edges:?}");
    //println!("v_count {n}, edges: {}", edges.len());

    let n = vertices.len();
    edges.iter().take(1000).for_each(|e| {
        vertices[e.src].adj.push(e.dst);
        vertices[e.dst].adj.push(e.src);
    });

    let mut components: Vec<Vec<usize>> = vec![];
    let mut visited: Vec<bool> = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            let cid = components.len();
            components.push(vec![i]);

            let mut stack = vec![i];
            while let Some(vert) = stack.pop() {
                visited[i] = true;
                for &dst in vertices[vert].adj.iter() {
                    if !visited[dst] {
                        stack.push(dst);
                        components[cid].push(dst);
                        visited[dst] = true;
                    }
                }
            }
        }
    }

    components.sort_by(|a, b| b.len().cmp(&a.len()));
    let sz = components.iter().take(3).fold(1, |acc, c| c.len() * acc);
    println!("components {:?}", components.iter().take(3));
    os.print_line(sz);
}
