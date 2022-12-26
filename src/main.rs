use ggez::*;
use rand::Rng;
use std::collections::HashSet;
use std::default;

const P_SIZE: f32 = 10.0; // "pixel" size
const SCREEN_WIDTH: f32 = 700.0;
const SCREEN_HEIGTH: f32 = 500.0;
const UPDATE_RATE: u128 = 30; // n millis for updating (fixed Update)
const SPW: f32 = SCREEN_WIDTH / P_SIZE;
const SPH: f32 = SCREEN_HEIGTH / P_SIZE;

enum Status {
    Over,
    Ongoing
}

struct State {
dt: std::time::Duration,
    curr_ms: u128,
    snake: Vec<(f32, f32)>,
    dir: (f32, f32),
    lock_dir: (f32, f32),
    food: (f32, f32),
    rng: rand::rngs::ThreadRng,
    status: Status,
    score: u32
}

impl default::Default for State {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        State {
dt: std::time::Duration::new(0, 0),
        snake: (0..30).map(|e| (0.0, e as f32)).collect(),
        curr_ms: 0,
        dir: (1.0, 0.0),
        lock_dir : (-1.0, 0.0),
        food: (
                rng.gen_range(0..SPW as usize) as f32,
                rng.gen_range(0..SPH as usize) as f32
              ),
        rng,
        score: 0,
        status: Status::Ongoing
        }
    }
}


fn update_pos(pos: (f32, f32), dir: (f32, f32)) -> (f32, f32) {
    let (dir_x, dir_y) = dir;
    let (pos_x, pos_y) = pos;
    if pos_x + dir_x >= SPW {
        return (0.0, pos_y)
    }
    if pos_x + dir_x < 0.0 {
        return (SPW - 1.0, pos_y)
    }
    if pos_y + dir_y == SPH {
        return (pos_x, 0.0)
    }
    if pos_y + dir_y < 0.0 {
        return (pos_x, SPH - 1.0)
    }
    (pos_x + dir_x, pos_y + dir_y)
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        self.curr_ms += ctx.time.delta().as_millis();

        match self.status {
            Status::Over =>  {
                Ok(())
            }
            Status::Ongoing => {
                if self.curr_ms > UPDATE_RATE {
                    self.curr_ms = 0;
                    let head = self.snake[0];
                    let tail = &self.snake[0..(self.snake.len() - 1)];
                    let mut updated_snake: Vec<(f32, f32)> = vec![];
                    updated_snake.push(update_pos(head, self.dir));
                    for pos in tail {
                        updated_snake.push(*pos);
                    }
                    self.snake = updated_snake;
                }
                if self.snake[0].0 == self.food.0 && self.snake[1].1 == self.food.1 {
                    self.score += 10;
                    self.snake.insert(0, update_pos(self.snake[0], self.dir));
                    self.food = (
                            self.rng.gen_range(0..SPW as usize) as f32,
                            self.rng.gen_range(0..SPH as usize) as f32
                            ); 
                }
                let mut uniq = HashSet::new();
                let alive = self.snake.iter().all(move |x| uniq.insert(format!("{}{}", x.0, x.1)));
                if !alive {
                    self.status = Status::Over;
                }
                Ok(())
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(&ctx.gfx, graphics::Color::WHITE);
        self.snake.iter()
            .map(|(x, y)| graphics::Rect::new(*x * P_SIZE, *y * P_SIZE, P_SIZE, P_SIZE))
            .for_each(|rect| {
                    let mesh = graphics::Mesh::new_rectangle(&ctx.gfx, graphics::DrawMode::fill(), rect, graphics::Color::BLUE).unwrap();
                    canvas.draw(&mesh, glam::vec2(0.0, 0.0));
                    });
        let rect = graphics::Rect::new(self.food.0 * P_SIZE, self.food.1 * P_SIZE, P_SIZE, P_SIZE);
        let mesh = graphics::Mesh::new_rectangle(&ctx.gfx, graphics::DrawMode::fill(), rect, graphics::Color::RED).unwrap();
        let score_text = graphics::Text::new(format!("ESC - QUIT\nSCORE: {}", self.score));
        let score_params = graphics::DrawParam::default().dest(glam::vec2(25.0, 25.0)).color(graphics::Color::MAGENTA);
        canvas.draw(&score_text, score_params);
        canvas.draw(&mesh, glam::vec2(0.0, 0.0));
        match self.status {
            Status::Over => {
                let text = graphics::Text::new(format!("Game over\nScore: {}\ntype 1 to replay", self.score));
                let dest = glam::vec2(25.0, 75.0);
                let params = graphics::DrawParam::default().dest(dest).color(graphics::Color::MAGENTA);
                canvas.draw(&text, params);
                canvas.finish(ctx)?;
                Ok(())
            }
            _ => {
                canvas.finish(ctx)?;
                Ok(())
            }
        }
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: input::keyboard::KeyInput, _repeated: bool) -> GameResult {
        match self.status {
            Status::Ongoing => {
                match input.scancode {
                    1 => ctx.request_quit(),
                      103 => {
                          if self.lock_dir != (0.0, -1.0) {
                              self.dir = (0.0, -1.0);
                              self.lock_dir = (0.0, 1.0);
                          }
                      }
                    106 => {
                        if self.lock_dir != (1.0, 0.0) {
                            self.dir = (1.0, 0.0);
                            self.lock_dir = (-1.0, 0.0);
                        }
                    }
                    108 => {
                        if self.lock_dir != (0.0, 1.0) {
                            self.dir = (0.0, 1.0);
                            self.lock_dir = (0.0, -1.0);
                        }
                    }
                    105 => {
                        if self.lock_dir != (-1.0, 0.0) {
                            self.dir = (-1.0, 0.0);
                            self.lock_dir = (1.0, 0.0);
                        }
                    }
                    _ => {}
                }
                Ok(())
            }
            Status::Over => {
                match input.scancode {
                    1 => ctx.request_quit(),
                      2 => {
                          *self = State::default();
                      }
                    _ => {}
                }
                Ok(())
            }
        }
    }
}


fn main() {
    let state = State::default();

    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGTH))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
