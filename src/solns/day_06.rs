use std::{collections::HashSet, io::Write, thread::sleep, time::Duration};

pub fn run(puzzle: &str) -> anyhow::Result<()> {
    println!("Part 1: {}", part1(puzzle)?);
    println!("Part 2: {}", part2(puzzle)?);

    Ok(())
}

fn part1(puzzle: &str) -> anyhow::Result<i32> {
    let (map, guard_start) = parse(puzzle)?;
    let mut guard = Guard::new(guard_start);

    let mut visited: HashSet<Pos> = HashSet::new();
    while guard.is_within_map(&map) {
        visited.insert(guard.pos);
        guard.step(&map);
    }

    Ok(visited.len() as i32)
}

fn part2(puzzle: &str) -> anyhow::Result<i32> {
    let (map, guard_start) = parse(puzzle)?;
    let mut guard = Guard::new(guard_start);

    // Get all visited locations. These are places we could place an obstacle.
    let mut visited: HashSet<Pos> = HashSet::new();
    while guard.is_within_map(&map) {
        visited.insert(guard.pos);
        guard.step(&map);
    }

    let mut loops = 0;
    for pos in visited.iter() {
        guard.reset();
        if is_inf_loop(&mut guard, &map_with_obstacle(map.clone(), pos)?) {
            loops += 1;
        }
    }

    Ok(loops)
}

fn map_with_obstacle(map: Vec<Vec<bool>>, obs: &Pos) -> anyhow::Result<Vec<Vec<bool>>> {
    let mut with_obstacle = map;
    with_obstacle[obs.1 as usize][obs.0 as usize] = true;

    Ok(with_obstacle)
}

fn is_inf_loop(guard: &mut Guard, map: &Vec<Vec<bool>>) -> bool {
    let mut visited_with_dir: HashSet<(Pos, Dir)> = HashSet::new();

    // If the guard leaves the map, we cannot have an infinite loop...
    while guard.is_within_map(map) {
        // and if the guard returns to a position it has been in before
        // while pointing in the same direction, we must have an infinite loop.
        if visited_with_dir.contains(&(guard.pos, guard.dir)) {
            return true;
        }

        visited_with_dir.insert((guard.pos, guard.dir));
        guard.step(&map);
    }

    return false;
}

#[allow(dead_code)]
fn render(map: &Vec<Vec<bool>>, guard: &Guard, visited: &HashSet<Pos>) {
    let mut buffer = String::new();

    // Move the cursor to the top-left corner without clearing the screen
    buffer.push_str("\x1B[H");

    for (y, row) in map.iter().enumerate() {
        for (x, is_blocked) in row.into_iter().enumerate() {
            if guard.pos.0 == x as i32 && guard.pos.1 == y as i32 {
                match guard.dir {
                    Dir::Up => buffer.push('^'),
                    Dir::Right => buffer.push('>'),
                    Dir::Down => buffer.push('V'),
                    Dir::Left => buffer.push('<'),
                }
            } else if visited.contains(&(x.try_into().unwrap(), y.try_into().unwrap())) {
                buffer.push('X');
            } else if is_blocked == &true {
                buffer.push('#');
            } else {
                buffer.push('.');
            }
        }
        buffer.push('\n');
    }

    // Write the buffer to the terminal in one go
    print!("{}", buffer);
    std::io::stdout().flush().unwrap();

    // Add a small delay for visual effect
    sleep(Duration::from_millis(10));
}

type Pos = (i32, i32);

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Guard {
    start: Pos,
    pos: Pos,
    dir: Dir,
}

impl Guard {
    fn new(origin: Pos) -> Self {
        Self {
            start: origin,
            pos: origin,
            dir: Dir::Up,
        }
    }

    fn step(&mut self, map: &Vec<Vec<bool>>) {
        self.pos = move_forward(self.pos, &self.dir);
        if self.is_within_map(map) && is_blocked(map, &self.pos) {
            self.pos = move_backward(self.pos, &self.dir);
            self.dir = turn_right(&self.dir);
            self.step(map);
        }
    }

    fn is_within_map(&self, map: &Vec<Vec<bool>>) -> bool {
        if self.pos.0 < 0 || self.pos.1 < 0 {
            return false;
        }
        if self.pos.1 >= map.len() as i32 {
            return false;
        }
        if self.pos.0 >= map[0].len() as i32 {
            return false;
        }

        return true;
    }

    fn reset(&mut self) {
        self.pos = self.start;
        self.dir = Dir::Up;
    }
}

fn move_forward(pos: Pos, dir: &Dir) -> Pos {
    match dir {
        Dir::Up => (pos.0, pos.1 - 1),
        Dir::Right => (pos.0 + 1, pos.1),
        Dir::Down => (pos.0, pos.1 + 1),
        Dir::Left => (pos.0 - 1, pos.1),
    }
}

fn move_backward(pos: Pos, dir: &Dir) -> Pos {
    match dir {
        Dir::Up => (pos.0, pos.1 + 1),
        Dir::Right => (pos.0 - 1, pos.1),
        Dir::Down => (pos.0, pos.1 - 1),
        Dir::Left => (pos.0 + 1, pos.1),
    }
}

fn turn_right(dir: &Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

fn is_blocked(map: &Vec<Vec<bool>>, pos: &Pos) -> bool {
    map[pos.1 as usize][pos.0 as usize]
}

fn parse(puzzle: &str) -> anyhow::Result<(Vec<Vec<bool>>, Pos)> {
    let mut map = vec![];
    let mut guard_start = (0, 0);
    for (y, line) in puzzle.lines().enumerate() {
        let mut locs = vec![];
        for (x, loc) in line.chars().enumerate() {
            match loc {
                '.' => locs.push(false),
                '#' => locs.push(true),
                '^' => {
                    guard_start = (x as i32, y as i32);
                    locs.push(false);
                }
                _ => return Err(anyhow::anyhow!("INVALID INPUT!")),
            }
        }
        map.push(locs);
    }

    Ok((map, guard_start))
}

#[cfg(test)]
mod test {
    use super::*;

    pub const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 6);
    }
}
