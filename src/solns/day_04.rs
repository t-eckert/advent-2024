pub fn run(puzzle: &str) -> anyhow::Result<()> {
    println!("Part 1: {}", part1(puzzle)?);
    println!("Part 2: {}", part2(puzzle)?);

    Ok(())
}

fn part1(puzzle: &str) -> anyhow::Result<i32> {
    let grid = Grid::from_puzzle(puzzle);
    Ok(grid
        .find_all(&'X')
        .into_iter()
        .map(|pos| {
            // We look around each X on the grid to gather all neighbors.
            grid.around(pos)
                .into_iter()
                // Filter down to just the neighbors who are 'M's, this gives us a direction to look in.
                .filter(|neighbor| grid.at(neighbor.pos) == Some('M'))
                // For each of these, search in that direction to find the remaining 'A' and 'S'.
                .filter_map(|neighbor| grid.search(neighbor.pos, neighbor.relation, vec!['A', 'S']))
                // Count how many successfully find the whole word.
                .count() as i32
        })
        .sum())
}

fn part2(puzzle: &str) -> anyhow::Result<i32> {
    let grid = Grid::from_puzzle(puzzle);

    let mut count = 0;
    let a_locations = grid.find_all(&'A');
    for a_location in a_locations {
        let neighbors = grid.around(a_location);
        let x_chars: Vec<(char, Relation)> = neighbors
            .into_iter()
            .filter(|n| {
                vec![
                    Relation::AboveLeft,
                    Relation::AboveRight,
                    Relation::BelowLeft,
                    Relation::BelowRight,
                ]
                .contains(&n.relation)
            })
            .map(|n| (grid.at(n.pos).unwrap(), n.relation))
            .filter(|x| vec!['M', 'S'].contains(&x.0))
            .collect();

        let mut al = None;
        let mut ar = None;
        let mut bl = None;
        let mut br = None;
        for x_char in x_chars {
            match x_char.1 {
                Relation::AboveLeft => al = Some(x_char.0),
                Relation::AboveRight => ar = Some(x_char.0),
                Relation::BelowLeft => bl = Some(x_char.0),
                Relation::BelowRight => br = Some(x_char.0),
                _ => {}
            }
        }

        if al.is_none() || ar.is_none() || bl.is_none() || br.is_none() {
            continue;
        }

        let x_wing = XWing {
            al: al.map(|c| XChar::from_char(c)).flatten().unwrap(),
            ar: ar.map(|c| XChar::from_char(c)).flatten().unwrap(),
            bl: bl.map(|c| XChar::from_char(c)).flatten().unwrap(),
            br: br.map(|c| XChar::from_char(c)).flatten().unwrap(),
        };

        if x_wing.is_legit() {
            count += 1;
        }
    }

    Ok(count)
}

type Pos = (usize, usize);
fn left(pos: Pos) -> Pos {
    (pos.0 - 1, pos.1)
}
fn right(pos: Pos) -> Pos {
    (pos.0 + 1, pos.1)
}
fn above(pos: Pos) -> Pos {
    (pos.0, pos.1 - 1)
}
fn below(pos: Pos) -> Pos {
    (pos.0, pos.1 + 1)
}

#[derive(Debug)]
struct Grid {
    chars: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn from_puzzle(puzzle: &str) -> Self {
        let chars: Vec<Vec<char>> = puzzle.lines().map(|line| line.chars().collect()).collect();

        Self {
            height: *&chars.len(),
            width: *&chars[0].len(),
            chars,
        }
    }

    fn find_all(&self, character: &char) -> Vec<Pos> {
        let mut indices = vec![];
        for (y, row) in self.chars.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if c == character {
                    indices.push((x, y));
                }
            }
        }

        indices
    }

    fn at(&self, pos: Pos) -> Option<char> {
        self.chars
            .get(pos.1)
            .map(|row| row.get(pos.0).map(|c| c.clone()))
            .flatten()
    }

    fn search(&self, pos: Pos, direction: Relation, mut chars: Vec<char>) -> Option<()> {
        if chars.len() == 0 {
            return Some(());
        }

        if !self.relations(pos).contains(&direction) {
            return None;
        }

        let neighbor = Neighbor::new(&pos, direction);
        if self.at(neighbor.pos) == Some(chars[0]) {
            chars.remove(0);
            return self.search(neighbor.pos, direction, chars);
        }

        None
    }

    fn relations(&self, pos: Pos) -> Vec<Relation> {
        let is_at_top_edge = pos.1 == 0;
        let is_at_bottom_edge = pos.1 == self.height - 1;
        let is_at_left_edge = pos.0 == 0;
        let is_at_right_edge = pos.0 == self.width - 1;

        let mut relations = vec![];

        if !is_at_top_edge {
            relations.push(Relation::Above);
            if !is_at_left_edge {
                relations.push(Relation::AboveLeft);
            }
            if !is_at_right_edge {
                relations.push(Relation::AboveRight);
            }
        }

        if !is_at_left_edge {
            relations.push(Relation::Left);
        }
        if !is_at_right_edge {
            relations.push(Relation::Right);
        }

        if !is_at_bottom_edge {
            relations.push(Relation::Below);
            if !is_at_left_edge {
                relations.push(Relation::BelowLeft);
            }
            if !is_at_right_edge {
                relations.push(Relation::BelowRight);
            }
        }

        relations
    }

    fn around(&self, pos: Pos) -> Vec<Neighbor> {
        self.relations(pos)
            .into_iter()
            .map(|rel| Neighbor::new(&pos, rel))
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Relation {
    AboveLeft,
    Above,
    AboveRight,
    Left,
    Right,
    BelowLeft,
    Below,
    BelowRight,
}

#[derive(Debug)]
struct Neighbor {
    pos: Pos,
    relation: Relation,
}

impl Neighbor {
    fn new(pos_from: &Pos, relation: Relation) -> Self {
        let p = *pos_from;
        Self {
            pos: match relation {
                Relation::AboveLeft => above(left(p)),
                Relation::Above => above(p),
                Relation::AboveRight => above(right(p)),
                Relation::Left => left(p),
                Relation::Right => right(p),
                Relation::BelowLeft => below(left(p)),
                Relation::Below => below(p),
                Relation::BelowRight => below(right(p)),
            },
            relation,
        }
    }
}

#[derive(PartialEq, Eq)]
enum XChar {
    M,
    S,
}

impl XChar {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'M' => Some(Self::M),
            'S' => Some(Self::S),
            _ => None,
        }
    }
}

struct XWing {
    al: XChar,
    ar: XChar,
    bl: XChar,
    br: XChar,
}

impl XWing {
    fn is_legit(self) -> bool {
        (self.al == XChar::M && self.br == XChar::S || self.al == XChar::S && self.br == XChar::M)
            && (self.ar == XChar::M && self.bl == XChar::S
                || self.ar == XChar::S && self.bl == XChar::M)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        println!("{}", EXAMPLE);
        assert_eq!(part1(EXAMPLE).unwrap(), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 9);
    }
}
