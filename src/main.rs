#![feature(slice_patterns)]
#![feature(step_by)]

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

    let mut ball1 = Ball::new(1.0, 10.0);
    let mut ball2 = Ball::new(1.0, 10.0);
    let mut ball3 = Ball::new(1.0, 10.0);

    let mut balls = create_balls(3);
    {
        //let mut vec = vec![&mut ball1, &mut ball2, &mut ball3];

        let between = Range::new(0.0 as f32, 1.0 as f32);
        let mut rng = rand::thread_rng();
        for ball in balls.iter_mut() {
            ball.rigidbody.position.x = between.ind_sample(&mut rng) * window_size.width as f32;
            ball.rigidbody.position.y = between.ind_sample(&mut rng) * window_size.height as f32;
        }
    }

    /*
    ball1.rigidbody.position.x = 0.3*window_size.width as f32;
    ball1.rigidbody.position.y = 0.1*window_size.height as f32;

    ball2.rigidbody.position.x = 0.5*window_size.width as f32;
    ball2.rigidbody.position.y = 0.7*window_size.height as f32;

    ball3.rigidbody.position.x = 0.8*window_size.width as f32;
    ball3.rigidbody.position.y = 0.8*window_size.height as f32;
    */

    /*
    let mut balls = Vec::new();
    balls.push(&mut ball1);
    balls.push(&mut ball2);
    balls.push(&mut ball3);
    */

    while let Some(e) = window.next() {

        /*
        {
            let mut pairs = vec![];
            for i in 0..balls.len() {
                for j in i+1..balls.len() {
                    pairs.push(&mut balls[i]);
                    pairs.push(&mut balls[j]);
                }
            }

            for pair in pairs.chunks_mut(2) {
                println!("{:?}", pair[0]);
                let ref mut ball1 = pair[0];
                let ref mut ball2 = pair[1];
                //println!("({:?}, {:?})", pair.0, pair.1);
                apply_gravity_mutual(&mut ball1.rigidbody, &mut ball2.rigidbody);
            }
        }
        */

         for i1 in 0..balls.len() {
            for i2 in i1+1..balls.len() {
                let slice = &mut balls[i1..i2+1];
                let (ball1, slice) = slice.split_first_mut().unwrap();
                let ball2 = slice.last_mut().unwrap();

                apply_gravity_mutual(&mut ball1.rigidbody, &mut ball2.rigidbody);
                //println!("({}, {})", el1, el2);
            }
        }

        /*
        apply_gravity_mutual(&mut ball1.rigidbody, &mut ball2.rigidbody);
        apply_gravity_mutual(&mut ball2.rigidbody, &mut ball3.rigidbody);
        apply_gravity_mutual(&mut ball1.rigidbody, &mut ball3.rigidbody);
        */

        //for ball in &[&mut ball1, &mut ball2, &mut ball3] {
        /*
        ball1.rigidbody.apply_velocity();
        ball2.rigidbody.apply_velocity();
        ball3.rigidbody.apply_velocity();
        */
        //}

        for ball in balls.iter_mut() {
            ball.rigidbody.apply_velocity();
        }

        //let balls = balls.clone();

        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);

            /*
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [
                      50.0,
                      50.0,
                      50.0,
                      50.0,
                      ],
                      c.transform, g);
            */

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
