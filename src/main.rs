#![feature(slice_patterns)]

extern crate piston_window;
extern crate ffitest;
extern crate rand;

use std::vec::Vec;

use rand::distributions::{Range, IndependentSample};
use piston_window::*;

use ffitest::astro::*;

fn create_balls(n: i32) -> Vec<Ball> {
    let mut vec = vec![];
    for _ in 0..n {
        let ball = Ball::new(1.0, 10.0);
        vec.push(ball);
    }
    return vec;
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let window_size = window.size();
    println!("{:?}", window_size);

    // Try values up to:
    //  - 100  (without --release)
    //  - 1000 (with --release)
    // These work fine on a 6th gen i7 (Skylake)
    let mut balls = create_balls(50);
    {
        let between = Range::new(0.2 as f32, 0.8 as f32);
        let mut rng = rand::thread_rng();
        for ball in balls.iter_mut() {
            ball.rigidbody.position.x = between.ind_sample(&mut rng) * window_size.width as f32;
            ball.rigidbody.position.y = between.ind_sample(&mut rng) * window_size.height as f32;
        }
    }

    while let Some(e) = window.next() {

         for i1 in 0..balls.len() {
            for i2 in i1+1..balls.len() {
                let slice = &mut balls[i1..i2+1];
                let (ball1, slice) = slice.split_first_mut().unwrap();
                let ball2 = slice.last_mut().unwrap();

                apply_gravity_mutual(&mut ball1.rigidbody, &mut ball2.rigidbody);
            }
        }

        for ball in balls.iter_mut() {
            ball.rigidbody.apply_velocity();
        }

        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);

            for ball in &balls {
                //println!("Rendering ball {:?}", ball);
                rectangle([1.0; 4],
                          [
                           ball.rigidbody.position.x as f64,
                           ball.rigidbody.position.y as f64,
                           ball.radius as f64,
                           ball.radius as f64
                          ],
                          c.transform, g);
            }
        });
    }
}
