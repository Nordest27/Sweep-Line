use std::vec;
use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, DrawMode};
use crate::domain::{Point, Segment, SweepLineProblem, Direction};

struct MainState {
    sweep_line_problem: SweepLineProblem,
}

impl MainState {
    fn new() -> Self {
        // Initialize your SweepLineProblem here
        let segments = vec![
            Segment {
                ini: Point { x: 100.0, y: 100.0 },
                end: Point { x: 200.0, y: 200.0 },
            },
            Segment {
                ini: Point { x: 300.0, y: 100.0 },
                end: Point { x: 400.0, y: 200.0 },
            },
        ];
        let sweep_line_problem = SweepLineProblem {
            segments,
            result: Vec::new(),
            time: 0.0,
            basic_operations: 0,
        };

        MainState { sweep_line_problem }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // Draw segments
        for segment in &self.sweep_line_problem.segments {
            canvas.draw(
                &graphics::Mesh::new_line(
                    ctx,
                    &[
                        Vec2::new(segment.ini.x, segment.ini.y),
                        Vec2::new(segment.end.x, segment.end.y),
                    ],
                    2.0,
                    graphics::Color::from([1.0, 1.0, 1.0, 1.0]),
                )?,
                graphics::DrawParam::default(),
            )
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn visualization() -> GameResult<()> {
    let (ctx, event_loop) =
        ggez::ContextBuilder::new("segments", "your_name").build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}