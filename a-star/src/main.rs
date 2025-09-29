use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn manhattan_distance(&self, other: &Position) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

// A* uses f(n) = g(n) + h(h) where:
// g(n) is the actual cost from start to n
// h(n) is the estimated cost from n to goal (heuristic)
// f(n) is the total estimated cost
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    f_cost: i32,
    g_cost: i32,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.cmp(&self.f_cost)
            .then_with(|| other.g_cost.cmp(&self.g_cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    width: i32,
    height: i32,
    obstacles: HashSet<Position>,
}

impl Grid {
    fn new(width: i32, height: i32) -> Self {
        Grid {
            width,
            height,
            obstacles: HashSet::new(),
        }
    }

    fn add_obstacle(&mut self, pos: Position) {
        self.obstacles.insert(pos);
    }

    fn is_valid(&self, pos: &Position) -> bool {
        pos.x >= 0
            && pos.x < self.width
            && pos.y >= 0
            && pos.y < self.height &&
            !self.obstacles.contains(pos)
    }

    fn get_neighbors(&self, pos: &Position) -> Vec<(Position, usize)> {
        let directions = [
            (0, 1), // Down
            (1, 0), // Right
            (0, -1), // Up
            (-1, 0), // Left
        ];

        directions.iter()
            .map(|(dx, dy)| Position::new(pos.x + dx, pos.y + dy))
            .filter(|p| self.is_valid(p))
            .map(|p| (p, 1))
            .collect()
    }
}

fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
    start: Position,
    goal: Position,
) -> Vec<Position> {
    let mut path = Vec::new();

    let mut current = goal;

    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap();
    }

    path.push(start);
    path.reverse();

    path
}

fn astar(grid: &Grid, start: Position, goal: Position) -> Option<(Vec<Position>, i32)> {
    let mut open_set = BinaryHeap::new();

    let mut g_costs = HashMap::new();
    g_costs.insert(start, 0);

    let mut came_from = HashMap::new();

    let mut closed_set = HashSet::new();

    let h_start = start.manhattan_distance(&goal);

    open_set.push(State {
        f_cost: h_start as i32,
        g_cost: 0,
        position: start,
    });

    while let Some(State { f_cost: _, g_cost, position }) = open_set.pop() {
        if position == goal {
            return Some((reconstruct_path(&came_from, start, goal), g_cost));
        }

        if closed_set.contains(&position) {
            continue
        }

        if g_cost > *g_costs.get(&position).unwrap_or(&i32::MAX) {
            continue
        }

        closed_set.insert(position);

        let neighbors = grid.get_neighbors(&position);

        for (neighbor, move_cost) in neighbors {
            if closed_set.contains(&neighbor) {
                continue;
            }

            let tentative_g = g_cost + (move_cost as i32);

            if tentative_g < *g_costs.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, position);
                g_costs.insert(neighbor, tentative_g);

                let h = neighbor.manhattan_distance(&goal) as i32;
                let f = tentative_g + h;

                open_set.push(State {
                    f_cost: f,
                    g_cost: tentative_g,
                    position: neighbor,
                })
            }
        }
    }

    None
}

fn main() {
    let mut grid = Grid::new(10, 10);

    for y in 2..8 {
        grid.add_obstacle(Position::new(5, y));
    }

    let start = Position::new(1, 5);
    let goal = Position::new(8, 5);

    println!("Finding path from ({}, {}) to ({}, {})", start.x, start.y, goal.x, goal.y);

    match astar(&grid, start, goal) {
        Some((path, cost)) => {
            println!("Path found! Length: {} steps, Cost: {}", path.len(), cost);

            println!("Path coordinates:");
            for (i, pos) in path.iter().enumerate() {
                println!("Step {} ({}, {})", i, pos.x, pos.y);
            }
        },
        None => {
            println!("No path found");
        }
    }
}
