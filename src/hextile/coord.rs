#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub q: i32,
    pub r: i32,
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

impl Direction {
    // Define the direction offsets for a flat-topped hexagon
    fn offset(self) -> Coord {
        match self {
            Direction::North => Coord { q: 0, r: -1 },
            Direction::Northeast => Coord { q: 1, r: -1 },
            Direction::Southeast => Coord { q: 1, r: 0 },
            Direction::South => Coord { q: 0, r: 1 },
            Direction::Southwest => Coord { q: -1, r: 1 },
            Direction::Northwest => Coord { q: -1, r: 0 },
        }
    }

    // Get all the directions as an array
    pub const ALL: [Direction; 6] = [
        Direction::North,
        Direction::Northeast,
        Direction::Southeast,
        Direction::South,
        Direction::Southwest,
        Direction::Northwest,
    ];
}

impl Coord {
    // Method to get a neighbor in a specific direction
    pub fn neighbor(&self, direction: Direction) -> Self {
        let offset = direction.offset();
        Coord {
            q: self.q + offset.q,
            r: self.r + offset.r,
        }
    }

    // Method to get all neighbors
    pub fn neighbors(&self) -> [Coord; 6] {
        let mut neighbors = [Coord { q: 0, r: 0 }; 6];
        for (i, direction) in Direction::ALL.iter().enumerate() {
            neighbors[i] = self.neighbor(*direction);
        }
        neighbors
    }
}