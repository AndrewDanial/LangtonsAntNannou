#![allow(unused)]
use nannou::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum TileType {
    Black,
    White,
}

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<TileType>>,
    ant: Ant,
}

#[derive(Debug, Clone)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Ant {
    pos_x: i32,
    pos_y: i32,
    direction: Directions,
}

const RECTWIDTH: f32 = 4.;
const RECTHEIGHT: f32 = 4.;

impl Board {
    fn new(width: usize, height: usize) -> Self {
        let mut grid =
            vec![vec![TileType::White; 512 / RECTWIDTH as usize]; 512 / RECTHEIGHT as usize];
        let mut ant = Ant {
            pos_x: grid.len() as i32 / 2,
            pos_y: grid[0].len() as i32 / 2,
            direction: Directions::Up,
        };
        Board { grid, ant }
    }

    fn display(&self, draw: &Draw, app: &App) {
        let offset_x = RECTWIDTH / 2.;
        let offset_y = RECTHEIGHT / 2.;
        let window = app.window_rect();
        let mut x = 0;
        let mut y = 0;
        for i in (window.x.start as i32..window.x.end as i32).step_by(RECTWIDTH as usize) {
            for j in (window.y.start as i32..window.y.end as i32).step_by(RECTHEIGHT as usize) {
                draw.rect()
                    .x_y(i as f32 + offset_x, j as f32 + offset_y)
                    .w_h(RECTWIDTH, RECTHEIGHT)
                    .color(if self.grid[x][y] == TileType::White {
                        WHITE
                    } else {
                        BLACK
                    });
                y += 1;
            }
            x += 1;
            y = 0;
        }
    }

    fn generate(&mut self) {
        let (mut pos_x, mut pos_y, mut direction) =
            (self.ant.pos_x, self.ant.pos_y, self.ant.direction.clone());
        let mut next = self.grid.clone();
        if pos_x >= next.len() as i32 {
            pos_x = 0;
        } else if pos_x < 0 {
            pos_x = next.len() as i32 - 1;
        }

        if pos_y >= next[0].len() as i32 {
            pos_y = 0;
        } else if pos_y < 0 {
            pos_y = next[0].len() as i32 - 1;
        }
        match next[pos_x as usize][pos_y as usize] {
            TileType::Black => {
                next[pos_x as usize][pos_y as usize] = TileType::White;
                match direction {
                    Directions::Up => {
                        direction = Directions::Right;
                        pos_x += 1
                    }
                    Directions::Down => {
                        direction = Directions::Left;
                        pos_x -= 1
                    }
                    Directions::Left => {
                        direction = Directions::Up;
                        pos_y += 1
                    }
                    Directions::Right => {
                        direction = Directions::Down;
                        pos_y -= 1
                    }
                }
            }

            TileType::White => {
                next[pos_x as usize][pos_y as usize] = TileType::Black;
                match direction {
                    Directions::Up => {
                        direction = Directions::Left;
                        pos_x -= 1;
                    }
                    Directions::Down => {
                        direction = Directions::Right;
                        pos_x += 1;
                    }
                    Directions::Left => {
                        direction = Directions::Down;
                        pos_y -= 1;
                    }
                    Directions::Right => {
                        direction = Directions::Up;
                        pos_y += 1
                    }
                }
            }
        }
        self.grid = next;
        self.ant = Ant {
            pos_x,
            pos_y,
            direction,
        };
    }
}

struct Model {
    board: Board,
    step: i32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _ = app.new_window().view(view).size(512, 512).build().unwrap();
    let mut board = Board::new(
        app.window_rect().w() as usize,
        app.window_rect().h() as usize,
    );
    Model { board, step: 0 }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    model.board.display(&draw, app);
    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.board.generate();
    model.step += 1;
}
