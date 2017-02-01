use std::cell::{Cell};

use nalgebra::{Vector2, Point2, FloatPoint, Norm};

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub position: Cell<Point2<f32>>,
    pub velocity: Cell<Vector2<f32>>,
    pub mass: f32,
    pub radius: f32,
}

impl Body {
    fn new(mass: f32, radius: f32) -> Body {
        return Body {
            position: Cell::new(Point2::new(0.0, 0.0)),
            velocity: Cell::new(Vector2::new(0.0, 0.0)),
            mass: mass,
            radius: radius
        };
    }

    fn apply_velocity(&self) {
        self.position.set(self.position.get() + self.velocity.get());
    }
}

fn apply_gravity(b1: &Body, b2: &Body) {
    let diff = b1.position.get() - b2.position.get();
    let norm = diff.normalize();
    let dist = b1.position.get().distance(&b2.position.get());
    let force = norm * b1.mass * b2.mass / dist;
    b1.velocity.set(b1.velocity.get() - force * b1.mass);
    b2.velocity.set(b2.velocity.get() + force * b2.mass);
}

/*
fn apply_gravity_mutual(b1: Body, b2: Body) {
    let dist: f32 = b1.position.get().dist(b2.position.get());
    let force: f32 = b1.mass * b2.mass / dist;
}
*/

#[cfg(test)]
mod tests {
    use std::cell::{Cell};
    use nalgebra::{Vector2, Point2, FloatPoint, Norm};
    use astro::{Body, apply_gravity};

    #[test]
    fn it_works() {
        let b1: Body = Body::new(1.0, 1.0);
        let b2: Body = Body::new(1.0, 1.0);
        b2.position.set(b2.position.get() + Vector2 { x: 0.0, y: 5.0 });
        println!("{:?}", b2);

        apply_gravity(&b1, &b2);
        println!("{:?}", b1);
        println!("{:?}", b2);

        b1.apply_velocity();
        b2.apply_velocity();
        println!("{:?}", b1);
        println!("{:?}", b2);
    }
}
