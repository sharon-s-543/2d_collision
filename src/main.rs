use std::collections::HashMap;

// const HEIGHT: u32 = 300;
// const WIDTH:  u32 = 400;

fn main() {
    println!("BILLIARD\n========\n");

    let mut particles: Vec<Particle> = Vec::new();

    // positions and velocities of two particles

    let particle1 = Particle::new(Vector::new(-4.0, 0.0), Vector::new(4.0, 1.0), 5.0, 1.0,);
    let particle2 = Particle::new(Vector::new(4.0, 2.0), Vector::new(-2.0, -1.0), 1.0, 1.0,);
    let particle3 = Particle::new(Vector::new( 18.0,  12.0), Vector::new(-3.0,  3.0), 1.5, 1.0);
    let particle4 = Particle::new(Vector::new(-10.0,  16.0), Vector::new( 6.0,  1.0), 2.0, 2.0);
    let particle5 = Particle::new(Vector::new( -3.0,  30.0), Vector::new( 1.0, -5.0), 1.5, 1.5);
    let particle6 = Particle::new(Vector::new(-21.0,  -5.0), Vector::new( 5.0, -3.0), 2.0, 2.0);
    let particle7 = Particle::new(Vector::new(-11.0,  -8.0), Vector::new(-3.0,  2.0), 2.0, 2.0);

    particles.push(particle1);
    particles.push(particle2);
    particles.push(particle3);
    particles.push(particle4);
    particles.push(particle5);
    particles.push(particle6);
    particles.push(particle7);


// Get a vector of particles. Loop through them using a two
// level nested loop to calculate the time of collision for
// all the possible combinations of particles. For example,
// find the time at collision for all possible combinations
// with particke A. Take the smallest of that time since that
// happens first. Repeat the same for particle B and so on.
// Collect these smallest values and their corresponding
// particle pairs. Find the smallest value of time from this
// collection. We have a delta_time. If this smallest time is
// lesser than delta_time, calculate its value after collision
// and update the position of every particle after that small
// time. Draw the particles. Sleep for that small time. Else
// change the position of the particles by delta_time. Draw
// the particles. Sleep for delta_time. Then go back to the 
// nested loop and this cycle goes on.


    let delta_time: f64     = 1.0 / 60.0;

// 
// DRAW THE BACKGROUND AND PARTICLES AT INITIAL POSITIONS HERE
//

    loop {
        let mut pairs: Vec<(&Particle,&Particle)> =Vec::new();
        let mut times_at_collision = vec![delta_time];

        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {

                let time: f64;
                if let Some(t) = time_at_collision(&particles[i], &particles[j]) {
                    time = t;
                }

                if time < delta_time {    
                    pairs.push((&particles[i],&particles[j]));
                    times_at_collision.push(time);
                }
            }
        }

        // Find the smallest global time and its index.
        // Find the particles at the index.
        // Update the position by smallest_time and apply
        // detect_collison for each pairs.
        
        // !!!
        // !!! WARNING !!!
        // POSSIBLE PROBLEMS WITH SIMULTANEOUS COLLISIONS!!!
        // !!!

        let mut smallest_time = f64::INFINITY;
        let mut index: i32;
        for i in 0..times_at_collision.len() {
            if times_at_collision[i] < smallest_time {
                smallest_time = time_at_collision[i];
                index         = i;
            }
        }

        // Draw the particle at pairs[index].
        
        for pair in &pairs {
            detect_collision();
        }

 
        // But here we need to loop through colliding_pairs
        // and do these calculations with every pairs of
        // particles and draw them. I haven't added a loop.

        // } else {
        //     
        // }

        std::thread::sleep(std::time::Duration::from_secs(delta_time));
    }
}


// ===========================================================
// ----------- Functions, Structs and Other Stuff ------------
// ===========================================================

struct Pair {
    particles: Vec<(Particle, Particle)>,
    time: Vec<f64>,
}

impl Pair {
    pub fn new(particle1: &Particle, particle2: &Particle, time: f64) 
        -> (Self, Vec<f64>, Vec<f64>) {
        let mut time      = Vec::new();
        let mut particles = Vec::new();
        let pair          = Pair {
            particles: particles.push((particle1, particle2)),
            time: time,
        };

        (pair, particles, time)
    }
}

// ===========================================================

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

// We need the initial positions, final positions and time to
// animate the collision. Initial positions can be found from
// the struct. Time can be returned. Final position can be 
// found from the modified struct.

fn detect_collision(mut particle1: Particle, mut particle2: Particle, time: f64) 
    -> Option<(Particle, Particle)>
{
    // let time = time_at_collision(&particle1, &particle2)?;
    let r_t1 = (particle1.position)
        .add(&(particle1.velocity).mul(time));
    let r_t2 = (particle2.position)
        .add(&(particle2.velocity).mul(time));
    particle1.position = r_t1;
    particle2.position = r_t2;

    let (v1, v2) = velocity_f(&mut particle1, &mut particle2)?;
    particle1.velocity = v1;
    particle2.velocity = v2;

    Some((particle1, particle2))
}



fn time_at_collision(particle1: &Particle, particle2: &Particle) -> Option<f64>
{
    let r   = &particle2.position.sub(&particle1.position);
    let v   = &particle2.velocity.sub(&particle1.velocity);
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
    let contact = r_i2.sub(&r_i1);
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
        return Some((v_f1, v_f2));

    }
    println!("Particles are not colliding.");
    None
}



fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)>
{
    let discriminant = b*b - 4.0*a*c;
    
    if discriminant < 0.0 || a == 0.0 {
        return None;
    }
    
    let rational = -b / (2.0 * a);
    let radical  = discriminant.sqrt() / (2.0 * a);
    let root1    = rational + radical;
    let root2    = rational - radical;

    println!("From quadratic fn (line 304), t = {}", root1);

    Some((root1, root2))
}
