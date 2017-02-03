use nalgebra::{Vector2, Point2, FloatPoint, Norm};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
}

impl RigidBody {
    fn new(mass: f32) -> RigidBody {
        return RigidBody {
            position: Point2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            mass: mass,
        };
    }

    pub fn apply_velocity(&mut self) {
        self.position = self.position + self.velocity;
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.velocity += 0.001 * force / self.mass;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub rigidbody: RigidBody,
    pub radius: f32,
    pub color: [f32; 4]
}

impl Ball {
    pub fn new(mass: f32, radius: f32) -> Ball {
        return Ball {
            rigidbody: RigidBody::new(mass),
            radius: radius,
            color: [1.0, 1.0, 1.0, 0.5]
        };
    }
}

pub fn apply_gravity_mutual(b1: &mut RigidBody, b2: &mut RigidBody) {
    let diff = b1.position - b2.position;
    let norm = diff.normalize();
    let dist = b1.position.distance(&b2.position);
    let force = norm * b1.mass * b2.mass / (dist + 20.0);
    b1.apply_force(-force);
    b2.apply_force(force);
}

#[no_mangle]
pub extern fn create_ball() -> Ball {
    return Ball::new(0.0, 0.0);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use nalgebra::{Vector2, Point2, FloatPoint, Norm};
    use astro::{RigidBody, apply_gravity_mutual};

    #[test]
    fn it_works() {
        let mut b1 = RigidBody::new(1.0, 1.0);
        let mut b2 = RigidBody::new(1.0, 1.0);
        b2.position = b2.position + Vector2 { x: 0.0, y: 5.0 };
        println!("{:?}", b2);

        apply_gravity_mutual(&mut b1, &mut b2);

        b1.apply_velocity();
        b2.apply_velocity();

        println!("{:?}", b1);
        println!("{:?}", b2);
    }
}
