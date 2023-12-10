use crate::days::Part;
use crate::days::{read_lines};


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Eq, PartialEq)]
enum Pipe {
    Start,
    Ground,
    NorthAndSouth,
    EastAndWest,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Pipe>>,
    dists: Vec<Vec<Option<u32>>>,
    inside: Vec<Vec<Option<bool>>>, 
    width: usize,
    height: usize,
    start_pos: (usize, usize)
}


pub fn run(file_name: &str, part: Part) -> Result<u32, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => part2(file_name),
    }
}


fn part1(file_name: &str) -> Result<u32, &'static str> {
    let mut graph = parse_graph(file_name)?;
    let Some((mut p1,mut p2)) = starting_poses(&mut graph) else {
        return Err("Invalid start position");
    };
    let mut distnace = 1;

    println!("{:?}, \nStartin Directions: {:?}, {:?}", graph, p1, p2);

    loop {
        println!("P1: {:?}, P2: {:?}", p1, p2);

        graph.dists[p1.0.1][p1.0.0] = Some(distnace);
        graph.dists[p2.0.1][p2.0.0] = Some(distnace);
        distnace += 1;

        // print_graph(&graph);

        if p1.0 == p2.0 {
            break;
        }

        let Some(p1_next) = graph.connecting_point(p1.0.0, p1.0.1, p1.1) else {
            return Err("failed to advance p1");
        };

        let Some(p2_next) = graph.connecting_point(p2.0.0, p2.0.1, p2.1) else {
            return Err("failed to advance p2");
        };

        p1 = p1_next;
        p2 = p2_next;
    }

    Ok(distnace - 1)
}


fn part2(file_name: &str) -> Result<u32, &'static str> {
    let mut graph = parse_graph(file_name)?;
    let Some((mut p1,mut p2)) = starting_poses(&mut graph) else {
        return Err("Invalid start position");
    };
    let mut distnace = 1;

    graph.dists[graph.start_pos.1][graph.start_pos.0] = Some(0);

    loop {
        graph.dists[p1.0.1][p1.0.0] = Some(distnace);
        graph.dists[p2.0.1][p2.0.0] = Some(distnace);
        distnace += 1;

        if p1.0 == p2.0 {
            break;
        }

        let Some(p1_next) = graph.connecting_point(p1.0.0, p1.0.1, p1.1) else {
            return Err("failed to advance p1");
        };

        let Some(p2_next) = graph.connecting_point(p2.0.0, p2.0.1, p2.1) else {
            return Err("failed to advance p2");
        };

        p1 = p1_next;
        p2 = p2_next;
    }

    // check_inside(&mut graph, 0, 1)?;

    for y in 0..graph.height {
        for x in 0..graph.width {
            if graph.inside[y][x] == None {
                check_inside(&mut graph, x, y)?;
            }
        }
    }

    let mut inside_count = 0;

    for y in 0..graph.height {
        for x in 0..graph.width {
            if graph.inside[y][x] == Some(true) {
                inside_count += 1;
            }
        }
    }

    _print_graph(&graph);
    

    Ok(inside_count)
}


fn starting_poses(graph: &mut Graph) -> Option<(((usize, usize), Direction), ((usize, usize), Direction))> {
    let mut p1 = None;
    let mut p2 = None;

    for heading in [Direction::North, Direction::South, Direction::East, Direction::West] {
        let Some(p) = graph.connecting_point(graph.start_pos.0, graph.start_pos.1, heading) else {
            continue;
        };

        if p1 == None {
            p1 = Some(p);
        } else {
            p2 = Some(p);
            break;
        }
    }

    if p1 == None || p2 == None {
        None 
    } else {
        let pipe = match (p1.unwrap().1, p2.unwrap().1) {
            (Direction::North, Direction::South) => Pipe::NorthAndSouth,
            (Direction::North, Direction::West) => Pipe::NorthAndWest,
            (Direction::North, Direction::East) => Pipe::NorthAndEast,
            (Direction::South, Direction::West) => Pipe::SouthAndWest,
            (Direction::South, Direction::East) => Pipe::SouthAndEast,
            (Direction::East, Direction::West) => Pipe::EastAndWest,
            _ => panic!(),
        };

        graph.nodes[graph.start_pos.1][graph.start_pos.0] = pipe;

        Some((p1.unwrap(), p2.unwrap()))
    }
}


fn parse_graph(file_name: &str) -> Result<Graph, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to parse file");
    };

    let mut elements = Vec::new();
    let mut start_pos = (0, 0);
    let mut dists = Vec::new();
    let mut inside = Vec::new();

    for l in lines {
        let Ok(line) = l else {
            return Err("Error in reading line");
        };

        let mut row = Vec::new();
        let mut row_dists = Vec::new();
        let mut row_inside = Vec::new();

        for ch in line.chars() {
            let el = match ch {
                '|' => Pipe::NorthAndSouth,
                '-' => Pipe::EastAndWest,
                'L' => Pipe::NorthAndEast,
                'J' => Pipe::NorthAndWest,
                '7' => Pipe::SouthAndWest,
                'F' => Pipe::SouthAndEast,
                '.' => Pipe::Ground,
                'S' => Pipe::Start,
                _ => return Err("Invalid char found when parsging")
            };

            if el == Pipe::Start {
                start_pos = (row.len(), elements.len());
            }

            row.push(el);
            row_dists.push(None);
            row_inside.push(None);
        }

        elements.push(row);
        dists.push(row_dists);
        inside.push(row_inside);
    }

    let width = elements[0].len();
    let height = elements.len();

    Ok(Graph {
        nodes: elements,
        dists,
        inside,
        width,
        height,
        start_pos
    })
}


fn check_inside(graph: &mut Graph, start_x: usize, start_y: usize) -> Result<(), &'static str> {
    let mut crossed_count = 0;
    let mut prev_x = start_x;

    if graph.on_line(start_x, start_y) {
        return Ok(());
    }

    loop {
        let Some((new_x, _)) = graph.is_valid(prev_x, start_y, 1, 0, true) else {
            break;
        };

        println!(" - {} {} {} {:?}", new_x, start_y, crossed_count, graph.nodes[start_y][new_x]);

        if !graph.on_line(new_x, start_y) {
            prev_x = new_x;
            continue;
        }

        if graph.nodes[start_y][new_x] == Pipe::NorthAndSouth {
            crossed_count += 1;
            prev_x = new_x;
            continue;
        }

        /* In line with pipe need to follow to see if we loop it */
        let (heading, incoming) = match graph.nodes[start_y][new_x] {
            Pipe::NorthAndEast => (Direction::East, Direction::South),
            Pipe::SouthAndEast => (Direction::East, Direction::North),
            _ => panic!(),
        };

        prev_x = new_x + 1;

        loop {
            println!("   - {} {} {} {:?}", prev_x, start_y, crossed_count, graph.nodes[start_y][prev_x]);

            let Some((point, p_heading)) = graph.connecting_point(
                prev_x, start_y, heading
            ) else {
                return Err("Error in following line");
            };

            println!("       - {:?} {:?}", point, p_heading);

            if p_heading == heading {
                prev_x = point.0;
            } else if p_heading == incoming {
                // prev_x += 1;
                crossed_count += 1;
                break;
            } else {
                // prev_x += 1;
                break;
            }
        }
    }

    graph.inside[start_y][start_x] = Some(crossed_count % 2 != 0);
    Ok(())
}


impl Graph {
    fn connecting_point(&self, x: usize, y: usize, heading: Direction) -> Option<((usize, usize), Direction)> {
        match &self.nodes[y][x] {
            Pipe::Ground => None,
            p @ _ => p.accepts_heading(heading).map(
                |((dx, dy), outgoing)| self.is_valid(x, y, dx, dy, false).map(
                    |point| (point, outgoing)
                )
            ).flatten()
        }
    }


    fn is_valid(&self, x: usize, y:usize, dx: isize, dy: isize, ignore_ground: bool) -> Option<(usize, usize)> {
        let x = x.checked_add_signed(dx)?;
        let y = y.checked_add_signed(dy)?;

        if  x >= self.width || y >= self.height || 
            (self.nodes[y][x] == Pipe::Ground && !ignore_ground) {
            None 
        } else {
            Some((x, y))
        }
    }

    fn on_line(&self, x: usize, y:usize) -> bool {
        self.dists[y][x] != None
    }
}


impl Pipe {
    fn accepts_heading(&self, heading: Direction) -> Option<((isize, isize), Direction)> {
        match (self, heading) {
            (Pipe::Start, Direction::North)         => Some(((0, -1), Direction::North)),
            (Pipe::Start, Direction::South)         => Some(((0, 1), Direction::South)),
            (Pipe::Start, Direction::East)          => Some(((1, 0), Direction::East)),
            (Pipe::Start, Direction::West)          => Some(((-1, 0), Direction::West)),
            (Pipe::NorthAndSouth, Direction::South) => Some(((0, 1), Direction::South)),
            (Pipe::NorthAndSouth, Direction::North) => Some(((0, -1), Direction::North)),
            (Pipe::EastAndWest, Direction::West)    => Some(((-1, 0), Direction::West)),
            (Pipe::EastAndWest, Direction::East)    => Some(((1, 0), Direction::East)),
            (Pipe::NorthAndEast, Direction::West)   => Some(((0, -1), Direction::North)),
            (Pipe::NorthAndEast, Direction::South)  => Some(((1, 0), Direction::East)),
            (Pipe::NorthAndWest, Direction::East)   => Some(((0, -1), Direction::North)),
            (Pipe::NorthAndWest, Direction::South)  => Some(((-1, 0), Direction::West)),
            (Pipe::SouthAndWest, Direction::North)  => Some(((-1, 0), Direction::West)),
            (Pipe::SouthAndWest, Direction::East)   => Some(((0, 1), Direction::South)),
            (Pipe::SouthAndEast, Direction::North)  => Some(((1, 0), Direction::East)),
            (Pipe::SouthAndEast, Direction::West)   => Some(((0, 1), Direction::South)),
            _ => None
        }
    }
}


fn _print_graph(graph: &Graph) {
    for y in 0..graph.height {
        for x in 0..graph.width {
            if graph.dists[y][x] != None {
                let dist = graph.dists[y][x].unwrap();

                print!("{}", dist);

                if dist < 10 {
                    print!(" ");
                }

                continue;
            }

            if graph.inside[y][x] != None {
                let inside = graph.inside[y][x].unwrap();

                if inside {
                    print!("I ");
                } else {
                    print!("O ");
                }

                continue;
            }

            print!(". ");
        }
        println!("");
    }
}