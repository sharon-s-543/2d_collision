fn main() {
    println!("BILLIARD\n========\n");
    
    // positions and velocities of two particles

    // let mut particle1 = Particle::new(
    //     Vector::new(23.0, 24.2),
    //     Vector::new(4.4, 2.5),
    //     23.04,
    //     5.6,
    //     );

    // let mut particle2 = Particle::new(
    //     Vector::new(6.7, 9.6),
    //     Vector::new(7.1, 8.8),
    //     31.94,
    //     7.9,
    //     );

    let mut particle1 = Particle::new(
        Vector::new(0.0, 0.0),
        Vector::new(5.0, 0.0),
        2.2,
        1.0,
        );

    let mut particle2 = Particle::new(
        Vector::new(50.0, 0.0),
        Vector::new(-5.0, 0.0),
        1.57,
        1.0,
        );

    // let mut time = 0.0;
    let mut i = 0;
    loop {
        match time_at_collision(&particle1, &particle2) {
            Some (time) => {



// ------------------------------------------------------
// ₹%₹%₹%%₹%₹%₹%₹% For testing purposes ₹%₹%₹₹%₹%₹^₹^%%%₹%
// ------------------------------------------------------
                let p_i1 = particle1.velocity.mul(particle1.mass);
                let p_i2 = particle2.velocity.mul(particle2.mass);
                let p_i  = &p_i1.add(&p_i2);

                let e_k1 = (&p_i1.dot(&p_i1)) / (2.0 * particle1.mass);
                let e_k2 = (&p_i2.dot(&p_i2)) / (2.0 * particle2.mass);
                let e_k  = e_k1 + e_k2;

                println!("Initial Position 1: {:#?}", particle1.position);
                println!("Initial Velocity 1: {:#?}", particle1.velocity);

                println!("Initial Position 2: {:#?}", particle2.position);
                println!("Initial Velocity 2: {:#?}", particle2.velocity);

                println!("-----------------------------------------");
// ------------------------------------------------------
// ₹%₹%₹%₹%₹%#%₹%#%₹%₹%₹^₹%₹₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%
// ------------------------------------------------------





                // Find positions for the two particles at t.
                let r_t1 = (particle1.position)
                    .add(&(particle1.velocity).mul(time));
                let r_t2 = (particle2.position)
                    .add(&(particle2.velocity).mul(time));
                particle1.position = r_t1;
                particle2.position = r_t2;

                // let (v1, v2) = velocity_f(&mut particle1, &mut particle2);
                match velocity_f(&mut particle1, &mut particle2) {
                    Some((v1, v2)) => {
                        particle1.velocity = v1;
                        particle2.velocity = v2;
                    }
                    None => {
                        println!("It's over.");
                        break;
                    }
                };

                // println!("\nInitial momentum: {:#?}\nFinal momentum: {:#?}\n", p_i, P_i);
                println!("PARTICLE1: {:#?}", particle1);
                println!("PARTICLE2: {:#?}", particle2);




// ------------------------------------------------------
// ₹&%&%&%&%^%₹%₹%₹% For testing purposes ₹%₹%₹₹%₹%₹%₹%%₹%
// ------------------------------------------------------
                let P_i1 = particle1.velocity.mul(particle1.mass);
                let P_i2 = particle2.velocity.mul(particle2.mass);
                let P_i  = &P_i1.add(&P_i2);
                println!("\nInitial momentum: {:#?}\nFinal momentum: {:#?}\n", p_i, P_i);
                // println!("{:#?}\n{:#?}\n", p_i, P_i);


                let E_k1 = (&P_i1.dot(&P_i1)) / (2.0 * particle1.mass);
                let E_k2 = (&P_i2.dot(&P_i2)) / (2.0 * particle2.mass);
                let E_k  = E_k1 + E_k2;
                println!("Initial KE: {:#?}\nFinal KE: {:#?}\n", e_k, E_k);
// ------------------------------------------------------
// ₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹%₹₹%₹%₹%₹^₹₹
                println!("---------------------------------");
                println!("#################₹₹₹₹#############₹₹\n\n");
// ------------------------------------------------------
// ------------------------------------------------------




            }
            None => {
                println!("No more collisions.");
                break;
            }
        }

        if i > 6 {
            break;
        };
        // time = 0.0;
        i += 1;
    }

    println!("\n\nI AM i {}\n", i);
}



// =======================================================
// ---------- Functions, Structs and Other Stuff ---------
// =======================================================

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
    pub fn new(position: Vector, velocity: Vector, mass: f64, radius: f64) -> Self {
            Particle {
                position,
                velocity,
                mass,
                radius,
            }
    }
}

// ======================================================
// ======================================================


fn time_at_collision(particle1: &Particle, particle2: &Particle) -> Option<f64>
{
    let r   = &particle1.position.sub(&particle2.position);
    let v   = &particle1.velocity.sub(&particle2.velocity);
    let rad = &particle2.radius + &particle1.radius;

    // Setting up the quadratic equation.
    let a = v.dot(v);
    let b = 2.0 * r.dot(v);
    let c = r.dot(r) - rad * rad;

    if b >= 0.0 {
        return None;
    }
    let (_, root) = quadratic(a, b, c)?;
    if root >= 0.0 {
        Some(root)
    } else {
        None
    }
}



fn velocity_f(particle1: &Particle, particle2: &Particle) -> Option<(Vector, Vector)>
{
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

    if u < 0.0 {
        let v_f1 = v_i1.sub(&contact.mul((2.0*m2/(m1+m2))*u));
        let v_f2 = v_i2.add(&contact.mul((2.0*m1/(m1+m2))*u));
        return Some((v_f1, v_f2));

    }
    println!("Particles are not colliding.");
    // return ((*v_i1).clone(), (*v_i2).clone());
    None
}



fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)>
{
    let discriminant = b*b - 4.0*a*c;
    
    if discriminant < 0.0 || a == 0.0 {
        return None;
    }
    
    let real = -b / (2.0 * a);
    let imag = discriminant.sqrt() / (2.0 * a);
    let root1 = real + imag;
    let root2 = real - imag;

    Some((root1, root2))
}

