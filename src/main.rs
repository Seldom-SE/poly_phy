//! An Asteroids-ish example game to show off ggez.
//! The idea is that this game is simple but still
//! non-trivial enough to be interesting.

use ggez;
use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::Mesh;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use nalgebra as na;
use mint;

type Point2 = na::Point2<f32>;

// Helper functions

// Actors

#[derive(Debug)]
enum ActorType {
    Floating,
    Polygon,
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    vertices: Vec<Point2>,
}

// Constructor Constants

// Constructors

fn default_polygons () -> Vec<Actor> {
    let mut polygons = Vec::new();
    polygons.push(Actor {
        tag: ActorType::Polygon,
        vertices: {
            let mut vertices = Vec::new();
            vertices.push(Point2::new(100., 100.));
            vertices.push(Point2::new(200., 200.));
            vertices.push(Point2::new(300., 100.));
            vertices.push(Point2::new(300., 300.));
            vertices.push(Point2::new(100., 300.));
            vertices
        }
    });
    polygons
}

// Physics Constants

// Physics

// Input State

#[derive(Debug)]
struct InputState {
    mouse_x: f32,
    mouse_y: f32,
    action: bool,
    draw: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            mouse_x: 0.0,
            mouse_y: 0.0,
            action: false,
            draw: false,
        }
    }
}

// Main State

struct MainState {
    floating: Option<Actor>,
    polygons: Vec<Actor>,
    screen_width: f32,
    screen_height: f32,
    input: InputState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        print_instructions();

        let (width, height) = graphics::drawable_size(ctx);
        let s = MainState {
            floating: None,
            polygons: default_polygons(),
            screen_width: width,
            screen_height: height,
            input: InputState::default(),
        };

        Ok(s)
    }
}

// Utility

fn print_instructions() {
    println!();
    println!("Welcome to poly_phy.");
    println!();
    println!("How to use:");
    println!("No features yet!");
    println!();
}

fn draw_actor(ctx: &mut Context, actor: &Actor,) -> GameResult {
    let polygon = Mesh::new_polygon(ctx,
        DrawMode::fill(),
        &actor.vertices.iter().map(|point| mint::Point2::from_slice(&[point.x, point.y])).collect::<Vec<mint::Point2<f32>>>()[..],
        Color::new(1., 1., 1., 1.));
    match &polygon {
        Ok(polygon) => graphics::draw(ctx,
            polygon,
            graphics::DrawParam::new()),
        Err(err) => Err(err.clone()),
    }
}

// Event Handling

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            // Handle Input

            // Update Physics

            // Handle Collisions
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // Loop over all objects drawing them...
        {
            let floating = &self.floating;
            if let Some(actor) = floating {
                draw_actor(ctx, actor)?;
            }

            for polygon in &self.polygons {
                draw_actor(ctx, polygon)?;
            }
        }

        // Draw UI

        // Flip Screen
        graphics::present(ctx)?;

        // Yield
        timer::yield_now();
        Ok(())
    }

    // Key Events

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _: KeyMods,
        _: bool,
    ) {
        match keycode {
            KeyCode::LShift => {
                self.input.draw = true;
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _: KeyMods) {
        match keycode {
            KeyCode::LShift => {
                self.input.draw = false;
            }
            _ => (), // Do nothing
        }
    }
}

// Main

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("poly_phy", "ggez")
        .window_setup(conf::WindowSetup::default().title("poly_phy"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0));

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}