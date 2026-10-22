#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector {x, y}
    }

    pub fn add(&self, b: &Vector) -> Self {
        Vector {x: self.x + b.x, y: self.y + b.y}
    }

    pub fn sub(&self, b: &Vector) -> Self {
        Vector {x: self.x - b.x, y: self.y - b.y}
    }

    pub fn mul(&self, c: f64) -> Self {
        Vector {x: c * self.x, y: c * self.y}
    }

    pub fn dot(&self, b: &Vector) -> f64 {
        self.x * b.x + self.y * b.y
    }
    
    pub fn len(&self) -> f64 {
        ((self.x).powf(2.0) + (self.y).powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.mul(1.0 / self.len())
    }
}

// ======================================================

#[derive(Debug, Clone, Copy)]
struct Particle {
    position: Vector,
    velocity: Vector,
    mass: f64,
    radius: f64,
}

impl Particle {
    pub fn new(
        position: Vector, velocity: Vector,
        mass: f64, radius: f64
        ) -> Self {
            Particle {
                position,
                velocity,
                mass,
                radius,
            }
    }
}

// =================== MAIN FUNCTION ====================

fn main() {
    println!("BILLIARD\n========\n");
    
    // positions and velocities of two particles

    let mut particle1 = Particle::new(
        Vector::new(3.0, 4.0),
        Vector::new(4.4, 2.5),
        23.0,
        5.6,
        );

    let mut particle2 = Particle::new(
        Vector::new(3.0, 4.0),
        Vector::new(4.4, 2.5),
        23.0,
        5.6,
        );

    // let mut time = 0.0;
    let mut i = 0;
    loop {
        let time = time_at_collision(&particle1, &particle2);

        // Find positions for the two particles at t.
        let r_t1 = (particle1.position).add(&(particle1.velocity).mul(time));
        let r_t2 = (particle2.position).add(&(particle2.velocity).mul(time));
        particle1.position = r_t1;
        particle2.position = r_t2;

        let velocities = velocity_f(&mut particle1, &mut particle2);
        particle1.velocity = velocities.0;
        particle2.velocity = velocities.1;

        if i > 6 {
            break;
        };
        // time = 0.0;
        i += 1;
    }
}

// ======================================================

fn time_at_collision(
    particle1: &Particle, particle2: &Particle,
    ) -> f64 {
    
    let r = &particle2.position.sub(&particle1.position);
    let v = &particle2.velocity.sub(&particle1.velocity);
    let rad = modulus(&particle2.radius - &particle1.radius);
    // I feel like I am forgetting to take a modulus for R.

    // Setting up the quadratic equation.
    let a = v.dot(v);
    let b = 2.0 * r.dot(v);
    let c = r.dot(r) - rad * rad;

    // let roots = quadratic(a, b, c);
    match quadratic(a, b, c) {
        Some(roots) => {
            let root1 = roots.unwrap().0;
            let root2 = roots.unwrap().1;

            if root1 < root2 {
                return root1;
            }
            root2
        }
        None => {
            println!("Particles are not colliding.");
            None
        }
    }
}

///////////////////
//
fn velocity_f(
    particle1: &Particle, particle2: &Particle,
    ) -> (Vector, Vector) {

    let r_i1 = &particle1.position;
    let r_i2 = &particle2.position;
    let contact = r_i1.sub(&r_i2);
    let contact = contact.mul(1.0 / contact.len());
    // contact normal found

    // find normal closing speed, u = (v1 - v2).n
    let v_i1 = &particle1.velocity;
    let v_i2 = &particle2.velocity;
    let u = (v_i1.sub(&v_i2)).dot(&contact);

    let m1 = particle1.mass;
    let m2 = particle2.mass;

    if u > 0.0 {
        let v_f1 = v_i1.sub(&contact.mul((2.0*m2/(m1+m2))*u));
        let v_f2 = v_i2.add(&contact.mul((2.0*m1/(m1+m2))*u));
        return (v_f1, v_f2);

        // *particle1 = Particle {
        //     position: r_i1,
        //     velocity_i: v_f1,
        //     mass: m1,  
        // };

        // *particle2 = Particle {
        //     position: r_i2,
        //     velocity_i: v_f2,
        //     mass: m2,
        // };

    }
    println!("Particles are not colliding.");
    return ((*v_i1).clone(), (*v_i2).clone());

    // ==========================================÷===÷=
    // Let's look at the position at collision now
    //
    // |(r2-r1) + (v2-v1)t| = R1 + R2
    //
/*
    let r = r_i2 - r_i1;
    let v = v_i2 - v_i1;
    let rad = &particle1.radius + &particle2.radius;

    let a = v.dot(&v);
    let b = 2 * r.dot(&v);
    let c = r.dot(r) - rad*rad;

    let time = 
*/
}

fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b*b - 4.0*a*c;
    
    if discriminant < 0.0 || a == 0.0 {
        return None;
    }
    
    let real = -b / (2.0 * a);
    let imag = discriminant / (2.0 * a);
    let root1 = real + imag;
    let root2 = real - imag;

    Some((root1, root2))
}

fn modulus(num: f64) -> f64 {
    if num < 0.0 {
        return (-1.0) * num;
    }
    num
}
