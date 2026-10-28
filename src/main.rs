use std::io::{self, Write};

const HEIGHT: u32 = 300;
const WIDTH:  u32 = 400;

fn main() {
    println!("BILLIARD\n========\n");
    


    let particles: Vec<Vector> = Vec::new();



    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // ========== POSSIBLY GONNA CHANGE LATER ===========
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××

    let num_particles = loop {
        print!("Number of particles: ");
        io::stdout().flush().expect("Failed to flush.");

        if let Some(num) = input::<u32>() {
            break num;
        } else {
            println!("Not a valid number. Please try again.");
        }
    };


    println!("Number of particles = {}", num_particles);
    


    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // ========== POSSIBLY GONNA CHANGE LATER ===========
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××

    // positions and velocities of two particles

    let mut particle1 = Particle::new(
        Vector::new(-4.0, 0.0),
        Vector::new(4.0, 1.0),
        5.0,
        1.0,
        );

    let mut particle2 = Particle::new(
        Vector::new(4.0, 2.0),
        Vector::new(-2.0, -1.0),
        1.0,
        1.0,
        );

    let mut i = 0;
    loop { 
        println!("Intial parameters:\n");
        println!("PARTICLE1: {:#?}", particle1);
        println!("PARTICLE2: {:#?}", particle2);
        println!("---------------------------------------");


        if let Some((time, new_particle1, new_particle2)) = detect_collision(particle1, particle2) {
            particle1 = new_particle1;
            particle2 = new_particle2;

            println!("time = {}", time);
            println!("PARTICLE1: {:#?}", particle1);
            println!("PARTICLE2: {:#?}", particle2);

            println!("\n==============================\n\n");
        } else {
            break;
        }

        if i > 3 {
            break;
        };
        i += 1;
    }

    println!("\n\nI AM i {}\n", i);
}


// ===========================================================
// ----------- Functions, Structs and Other Stuff ------------
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

fn detect_collision(mut particle1: Particle, mut particle2: Particle) 
    -> Option<(f64, Particle, Particle)>
{
    let time = time_at_collision(&particle1, &particle2)?;
    let r_t1 = (particle1.position)
        .add(&(particle1.velocity).mul(time));
    let r_t2 = (particle2.position)
        .add(&(particle2.velocity).mul(time));
    particle1.position = r_t1;
    particle2.position = r_t2;

    let (v1, v2) = velocity_f(&mut particle1, &mut particle2)?;
    particle1.velocity = v1;
    particle2.velocity = v2;

    Some((time, particle1, particle2))
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


fn input<T: std::str::FromStr>() -> Option<T> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read number."); 

    match buffer.trim().parse::<T>() {
        Ok(value) => Some(value),
        Err(_)  => {
            None
        }
    }
}
