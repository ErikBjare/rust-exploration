use nalgebra::{Vector2, Point2, FloatPoint, Norm};

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
}

impl Body {
    fn new(mass: f32, radius: f32) -> Body {
        return Body {
            position: Point2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            mass: mass,
            radius: radius
        };
    }

    fn apply_velocity(&mut self) {
        self.position = self.position + self.velocity;
    }

    fn apply_force(&mut self, force: Vector2<f32>) {
        self.velocity += force * self.mass;
    }
}

fn apply_gravity(b1: &mut Body, b2: &mut Body) {
    let diff = b1.position - b2.position;
    let norm = diff.normalize();
    let dist = b1.position.distance(&b2.position);
    let force = norm * b1.mass * b2.mass / dist;
    b1.apply_force(-force);
    b2.apply_force(force);
}

#[no_mangle]
pub extern fn create_body() -> Body {
    return Body::new(0.0, 0.0);
}

/*
fn apply_gravity_mutual(b1: Body, b2: Body) {
    let dist: f32 = b1.position.get().dist(b2.position.get());
    let force: f32 = b1.mass * b2.mass / dist;
}
*/

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use nalgebra::{Vector2, Point2, FloatPoint, Norm};
    use astro::{Body, apply_gravity};

    #[test]
    fn it_works() {
        let mut b1: Body = Body::new(1.0, 1.0);
        let mut b2: Body = Body::new(1.0, 1.0);
        b2.position = b2.position + Vector2 { x: 0.0, y: 5.0 };
        println!("{:?}", b2);

        apply_gravity(&mut b1, &mut b2);

        b1.apply_velocity();
        b2.apply_velocity();

        println!("{:?}", b1);
        println!("{:?}", b2);
    }
}
