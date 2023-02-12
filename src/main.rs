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

// struct Model {}

fn main() {
    let grid = Grid::new(3, 3, 0.01);
    grid.print();
    // nannou::app(model).event(event).simple_window(view).run();
}

// fn model(_app: &App) -> Model {
//     Model {}
// }

// fn event(_app: &App, _model: &mut Model, _event: Event) {}

// fn view(_app: &App, _model: &Model, _frame: Frame) {}
