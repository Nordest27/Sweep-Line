use std::time::Instant;
use std::vec;
use ggez::{Context, GameError, GameResult};
use ggez::event::{self, EventHandler};
use ggez::input::mouse::{MouseContext, MouseButton};
use ggez::input::keyboard::KeyCode;
use ggez::glam::Vec2;
use ggez::conf::WindowMode;
use ggez::graphics::{self, DrawMode};
use ggez::input::keyboard::KeyInput;
use crate::domain::{Point, Segment, SweepLineProblem, Direction, distance};
use crate::solvers::{naive_intersection_solver, sweep_line_solver};

struct MainState {
    sweep_line_problem: SweepLineProblem,
    intersection_alpha: f32,
    init_time: Instant,
    mouse_button: MouseButton,
    mouse_position: Point,
    highlight_point_index: Option<(usize, usize)>,
    grid_size: f64,
}

impl MainState {
    fn new() -> Self {
        // Initialize your SweepLineProblem here
        let mut sweep_line_problem = SweepLineProblem::load("problems/sweep_line_problem_2.txt");
        MainState {
            sweep_line_problem,
            intersection_alpha: 0.0,
            init_time: Instant::now(),
            mouse_button: MouseButton::Other(0),
            mouse_position: Point { x: 0.0, y: 0.0 },
            highlight_point_index: None,
            grid_size: 10.0,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.intersection_alpha = (((self.init_time.elapsed().as_secs_f64()*2.0).sin() + 1.0) / 2.0 + 0.1) as f32;
        sweep_line_solver(&mut self.sweep_line_problem);
        //naive_intersection_solver(&mut self.sweep_line_problem);
        if self.mouse_button != MouseButton::Left {
            self.highlight_point_index = None;
            for i in 0..self.sweep_line_problem.segments.len() {
                let segment = &self.sweep_line_problem.segments[i];
                if distance(&self.mouse_position, &segment.ini) < 5.0 {
                    self.highlight_point_index = Some((i, 0));
                    break;
                }
                if distance(&self.mouse_position, &segment.end) < 5.0 {
                    self.highlight_point_index = Some((i, 1));
                    break;
                }
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Get the window size
        let (width, height) = ctx.gfx.size();

        //Draw grid
        for i in 0..(width / self.grid_size as f32) as i32 {
            canvas.draw(
                &graphics::Mesh::new_line(
                    ctx,
                    &[
                        Vec2::new((i as f64 * self.grid_size) as f32, 0.0),
                        Vec2::new((i as f64 * self.grid_size) as f32, height)
                    ],
                    1.0,
                    graphics::Color::from([0.5, 0.5, 0.5, 0.2]),
                )?,
                graphics::DrawParam::default(),
            );
        }
        for i in 0..(height / self.grid_size as f32) as i32 {
            canvas.draw(
                &graphics::Mesh::new_line(
                    ctx,
                    &[
                        Vec2::new(0.0, (i as f64 * self.grid_size) as f32),
                        Vec2::new(width, (i as f64 * self.grid_size) as f32),
                    ],
                    1.0,
                    graphics::Color::from([0.5, 0.5, 0.5, 0.2]),
                )?,
                graphics::DrawParam::default(),
            );
        }

        // Draw segments
        for segment in &self.sweep_line_problem.segments {
            canvas.draw(
                &graphics::Mesh::new_line(
                    ctx,
                    &[
                        Vec2::new(segment.ini.x as f32, segment.ini.y as f32),
                        Vec2::new(segment.end.x as f32, segment.end.y as f32),
                    ],
                    2.0,
                    graphics::Color::from([1.0, 1.0, 1.0, 1.0]),
                )?,
                graphics::DrawParam::default(),
            );

            canvas.draw(
                &graphics::Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    Vec2::new(segment.ini.x as f32, segment.ini.y as f32),
                    2.0,
                    0.1,
                    graphics::Color::from([1.0, 1.0, 1.0, 1.0]),
                )?,
                graphics::DrawParam::default(),
            );

            canvas.draw(
                &graphics::Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    Vec2::new(segment.end.x as f32, segment.end.y as f32),
                    2.0,
                    0.1,
                    graphics::Color::from([1.0, 1.0, 1.0, 1.0]),
                )?,
                graphics::DrawParam::default(),
            );
        }
        for segment in &self.sweep_line_problem.result {
            canvas.draw(
                &graphics::Mesh::new_line(
                    ctx,
                    &[
                        Vec2::new(segment.ini.x as f32, segment.ini.y as f32),
                        Vec2::new(segment.end.x as f32, segment.end.y as f32),
                    ],
                    5.0,
                    graphics::Color::from([1.0, 0.0, 0.0, self.intersection_alpha]),
                )?,
                graphics::DrawParam::default(),
            );

            canvas.draw(
                &graphics::Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    Vec2::new(segment.ini.x as f32, segment.ini.y as f32),
                    5.0,
                    0.1,
                    graphics::Color::from([1.0, 0.0, 0.0, self.intersection_alpha]),
                )?,
                graphics::DrawParam::default(),
            );

            canvas.draw(
                &graphics::Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    Vec2::new(segment.end.x as f32, segment.end.y as f32),
                    5.0,
                    0.1,
                    graphics::Color::from([1.0, 0.0, 0.0, self.intersection_alpha]),
                )?,
                graphics::DrawParam::default(),
            );

            if self.highlight_point_index.is_some() {
                let segment = &self.sweep_line_problem.segments[self.highlight_point_index.unwrap().0];
                let point = if self.highlight_point_index.unwrap().1 == 0 {
                    &segment.ini
                } else {
                    &segment.end
                };
                canvas.draw(
                    &graphics::Mesh::new_circle(
                        ctx,
                        DrawMode::fill(),
                        Vec2::new(point.x as f32, point.y as f32),
                        5.0,
                        0.1,
                        graphics::Color::from([0.0, 0.0, 1.0, 1.0]),
                    )?,
                    graphics::DrawParam::default(),
                );
            }

        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
        if _button == MouseButton::Left {
            self.mouse_button = _button;
        }
        else if _button == MouseButton::Right {
            let mut i_to_delete = -1;
            for i in 0..self.sweep_line_problem.segments.len() {
                let segment = &self.sweep_line_problem.segments[i];
                if distance(&self.mouse_position, &segment.ini) < 5.0
                    || distance(&self.mouse_position, &segment.end) < 5.0 {
                    i_to_delete = i as i32;
                    break;
                }
            }
            if i_to_delete != -1 {
                self.sweep_line_problem.segments.remove(i_to_delete as usize);
            }
            else {
                self.sweep_line_problem.segments.push(
                    Segment {
                        ini: Point { x: _x as f64, y: _y as f64 }.to_grid(self.grid_size),
                        end: Point { x: _x as f64, y: _y as f64 }.to_grid(self.grid_size),
                    }
                );
            }
        }
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
        if _button == MouseButton::Left {
            self.mouse_button = MouseButton::Other(0);
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> Result<(), GameError> {
        self.mouse_position = Point { x: x as f64, y: y as f64 };
        if self.mouse_button == MouseButton::Left {
            if let Some((i, j)) = self.highlight_point_index {
                if j == 0 {
                    self.sweep_line_problem.segments[i].ini = Point { x: x as f64, y: y as f64 }.to_grid(self.grid_size);
                } else {
                    self.sweep_line_problem.segments[i].end = Point { x: x as f64, y: y as f64 }.to_grid(self.grid_size);
                }
            }
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        match input.keycode {
            Some(KeyCode::S) => {
                let mut i = 0;
                loop {
                    let file_name = format!("problems/sweep_line_problem_{}.txt", i);
                    if std::path::Path::new(&file_name).exists() {
                        i += 1;
                    } else {
                        self.sweep_line_problem.save(&file_name);
                        break;
                    }
                }
            }
            _ => (),
        }
        Ok(())
    }
}

pub fn visualization() -> GameResult<()> {
    let (mut ctx, event_loop) =
        ggez::ContextBuilder::new("segments", "your_name").build()?;
    ctx.gfx.set_window_title("Segments Intersection");
    ctx.gfx.set_mode(WindowMode {
        width: 800.0,
        height: 600.0,
        resizable: true,
        ..Default::default()
    }).expect("Error setting window mode");
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}