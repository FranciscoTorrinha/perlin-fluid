use std::ops;

use nannou::noise::{NoiseFn, Perlin};

struct Cell {
    angle: f64,
}

struct Grid {
    width: usize,
    height: usize,
    cell_size: f64,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(width: usize, height: usize, cell_size: f64) -> Grid {
        let perlin = Perlin::new();

        let cells: Vec<Vec<Cell>> = (0..height)
            .into_iter()
            .map(|y| {
                (0..width)
                    .into_iter()
                    .map(|x| Cell {
                        angle: perlin.get([x as f64 * cell_size, y as f64 * cell_size]),
                    })
                    .collect()
            })
            .collect();

        Grid {
            width,
            height,
            cell_size,
            cells,
        }
    }

    fn print(&self) {
        self.cells.iter().for_each(|line| {
            line.iter().for_each(|cell| print!("{:.3} ", cell.angle));
            println!("")
        })
    }
}

#[derive(Default, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Default)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    accelaration: Vec2,
}

impl Particle {
    fn new(position: Vec2) -> Particle {
        Particle {
            position,
            ..Default::default()
        }
    }

    fn print(&self) {
        println!(
            "Postion: {}\nVelocity: {}\nAccelataion: {}",
            self.position.to_string(),
            self.velocity.to_string(),
            self.accelaration.to_string()
        );
    }

    fn apply_force(&mut self, force: Vec2) {
        self.accelaration += force;
    }

    fn update(&mut self) {
        self.velocity += self.accelaration;
        self.position += self.velocity;
    }
}

fn main() {
    let grid = Grid::new(3, 3, 0.01);
    grid.print();

    let mut particle = Particle::new(Vec2 { x: 1.0, y: 1.0 });
    particle.apply_force(Vec2 { x: 0.2, y: 0.2 });
    particle.update();
    particle.print();
}
