extern crate ffitest;
extern crate rand;
extern crate nalgebra;
extern crate clap;

extern crate piston_window;

use std::vec::Vec;

use rand::distributions::{Range, IndependentSample};
use nalgebra::{Vector2, Point2, distance};
use clap::{Arg, App};

use piston_window::{PistonWindow, WindowSettings, Window, clear, ellipse, RenderEvent, UpdateEvent};

use ffitest::astro::*;

fn create_balls(n: u32) -> Vec<Ball> {
    let mut vec = vec![];
    for _ in 0..n {
        let ball = Ball::new(1.0, 1.0);
        vec.push(ball);
    }
    return vec;
}

fn place_balls_randomly(balls: &mut Vec<Ball>, width: u32, height: u32) {
    let between = Range::new(0.2 as f32, 0.8 as f32);
    let mut rng = rand::thread_rng();
    for ball in balls.iter_mut() {
        ball.rigidbody.position.x = between.ind_sample(&mut rng) * width as f32;
        ball.rigidbody.position.y = between.ind_sample(&mut rng) * height as f32;
    }
}

fn create_balls_randomly(n: u32, window_width: u32, window_height: u32) -> Vec<Ball> {
    let mut balls = create_balls(n);
    place_balls_randomly(&mut balls, window_width, window_height);
    return balls;
}

fn create_orbit_system(center_x: f32, center_y: f32) -> Vec<Ball> {
    let mut balls = create_balls(2);

    balls[0].radius = 10.0;
    balls[0].rigidbody.mass = 100.0;
    balls[0].rigidbody.position = Point2::new(center_x, center_y);
    balls[0].color = [0.2, 0.8, 0.2, 1.0];

    balls[1].rigidbody.position = Point2::new(center_x + 200.0, center_y);
    balls[1].rigidbody.apply_force(Vector2::new(0.0, 10.0));
    balls[1].radius = 2.0;

    return balls;
}

struct Game {
    pub window: PistonWindow,
    pub balls: Vec<Ball>,
    pub merge_on_collision: bool,
}

impl Game {
    fn mainloop(&mut self) {
        while let Some(e) = self.window.next() {
            //println!("{:?}", e);

            e.render(|_| {
                // TODO: Figure out how to move into self.render (I don't know how to pass the
                // event into the render method)
                let ref balls = self.balls;
                self.window.draw_2d(&e, |c, g| {
                    clear([0.0; 4], g);

                    for ball in balls.iter() {
                        ellipse(ball.color,
                                  [
                                   ball.rigidbody.position.x as f64 - ball.radius as f64,
                                   ball.rigidbody.position.y as f64 - ball.radius as f64,
                                   2.0*ball.radius as f64,
                                   2.0*ball.radius as f64
                                  ],
                                  c.transform, g);
                    }
                });
            });

            e.update(|update_args| {
                self.update(update_args.dt);
            });
        }
    }

    /*
    fn render(&mut self, e: &Event) {
    }
    */

    fn update(&mut self, dt: f64) {
        let mut to_remove = vec![];
        let time_factor = 10.0;

        for i1 in 0..self.balls.len() {
            for i2 in i1+1..self.balls.len() {
                let slice = &mut self.balls[i1..i2+1];
                let (ball1, slice) = slice.split_first_mut().unwrap();
                let ball2 = slice.last_mut().unwrap();

                apply_gravity_mutual(&mut ball1.rigidbody, &mut ball2.rigidbody, dt * time_factor);

                if self.merge_on_collision {
                    // TODO: All this is very much not-at-all realistic
                    // TODO: Merge-logic should probably be moved to Ball.add() or similar
                    // TODO: We might want to make merging step-by-step (as in the game Osmos)
                    // TODO: We might want to set the merged ball position to a weighted average of the two
                    if distance(&ball1.rigidbody.position, &ball2.rigidbody.position) < ball1.radius + ball2.radius
                    && !to_remove.contains(&i1) {
                        to_remove.push(i1);

                        let ball1_area = ball1.radius.powi(2) * 3.14;
                        let ball2_area = ball2.radius.powi(2) * 3.14;
                        ball2.radius = ((ball1_area + ball2_area) / 3.14).sqrt();

                        ball2.rigidbody.mass += ball1.rigidbody.mass;
                        ball2.rigidbody.apply_force(ball1.rigidbody.velocity * ball1.rigidbody.mass);
                    }
                }
            }
        }

        // This is simply to avoid the tedious task of removing the balls within the pair-loop above
        if to_remove.len() > 0 {
            println!("Removing {} balls", to_remove.len());
            for i in to_remove.iter().rev() {
                self.balls.remove(*i);
                println!("Removed ball #{}", i)
            }
        }

        for ball in self.balls.iter_mut() {
            ball.rigidbody.apply_velocity(dt * time_factor);
        }
    }
}

fn main() {
    let matches = App::new("gravitysim")
                      .version("0.1")
                      .about("A simple gravity simulator written by a Rust newbie.")
                      .author("Erik Bjäreholt <erik.bjareholt@gmail.com>")
                      .arg(Arg::with_name("scene")
                               .short("s")
                               .long("scene")
                               .help("Specify scene to load")
                               .value_name("SCENE_NAME")
                               .takes_value(true)
                               .possible_values(&["orbit", "random"]))
                      .arg(Arg::with_name("collide-merge")
                               .long("collide-merge")
                               .help("When set balls will merge upon collision"))
                      .get_matches();

    let window: PistonWindow =
        WindowSettings::new("GravitySim", [640, 480]).samples(16)
        .exit_on_esc(true).build().unwrap();

    let window_size = window.size();

    let balls: Vec<Ball>;
    let scene = matches.value_of("scene").unwrap_or("random");
    match scene {
        "orbit" =>
            balls = create_orbit_system((window_size.width/2) as f32, (window_size.height/2) as f32),
        "random" | _ =>
            // Try values up to:
            //  - 100  (without --release)
            //  - 1000 (with --release)
            // These work fine on a 6th gen i7 (Skylake) even with merge_on_collision set to false
            balls = create_balls_randomly(50, window_size.width, window_size.height),
    }

    let mut game = Game {
        window: window,
        balls: balls,
        merge_on_collision: matches.is_present("collide-merge"),
    };
    game.mainloop();
}
