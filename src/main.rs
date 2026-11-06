// use std::io::{self, Write};
use std::collections::HashMap;

// const HEIGHT: u32 = 300;
// const WIDTH:  u32 = 400;

fn main() {
    println!("BILLIARD\n========\n");
    


    let mut particles: Vec<Particle> = Vec::new();



    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // ========== POSSIBLY GONNA CHANGE LATER ===========
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××

    // let num_particles = loop {
    //     print!("Number of particles: ");
    //     io::stdout().flush().expect("Failed to flush.");

    //     if let Some(num) = input::<u32>() {
    //         break num;
    //     } else {
    //         println!("Not a valid number. Please try again.");
    //     }
    // };


    // println!("Number of particles = {}", num_particles);
    


    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // ========== POSSIBLY GONNA CHANGE LATER ===========
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××
    // %>÷;[#,[#>#&>@^^@^<@:@&!^!<!^@@@&@>;@>×<×&×>×;×&××

    // positions and velocities of two particles

    let num = 7;

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


    loop {
        let delta_time: f64     = 1.0 / 60.0;

        let mut colliding_pairs = HashMap::new();
        let mut pairs           = HashMap::new();

        for i in 0..num {
            for j in 0..num {
                if i == j {
                    continue;
                }
                // HANDLE ERROR!
                let time: f64;
                if let Some(t) = time_at_collision(&particles[i], &particles[j]) {
                    time = t;
                }
                pairs.insert(time.to_string(), particles[j]);
            }

            if let Some(t_min) = pairs.keys().min() {
                let t_min_key  = t_min.to_string();
                colliding_pairs.insert(t_min_key, (particles[i], pairs.get(t_min_key)));
            } else {
                println!("Not colliding. From the time Hashmap.");
            }
            pairs.clear();
        }
        // End of i loop ^

        // FUCK! Just realized I can't use a float as a key.
        // Fine. Let's write the rest of the logic then.
        
        let t_min: f64;
        let mut t_min_key = String::new();

        // Smallest of the smallest t.
        if let Some(key) = colliding_pairs.keys().min() {
            t_min_key = key;
            t_min     = key.parse().unwrap(); 
        }

        // if t_min < delta_time {
        
        // Take the two particles from the hashmap and send to
        // the detect_collision function to get updated data.
        for (key, val) in colliding_pairs.iter() {
            let p1 = (colliding_pairs[&key]).0;
            let p2 = (colliding_pairs[&key]).1;
            
            if t_min < delta_time {
                let particles = if let Some((particle1, particle2))
                    = detect_collision(p1, p2, t_min) {
                        (particle1, particle2)
                };

                // >>>> DRAW particle1 AND particle2 >>>>
            } else {
    /**/



                
                let p1 = (colliding_pairs[&key]).0;
                let p2 = (colliding_pairs[&key]).1;

                let r_t1 = (p1.position)
                    .add(&(p1.velocity).mul(delta_time));
                let r_t2 = (p2.position)
                    .add(&(p2.velocity).mul(delta_time));
                p1.position = r_t1;
                p2.position = r_t2;




    /**/
            }
        }
            
        // But here we need to loop through colliding_pairs
        // and do these calculations with every pairs of
        // particles and draw them. I haven't added a loop.

        // } else {
        //     
        // }

    }







    // for _ in 0..num {
    //     let (rx, ry) = loop {
    //         print!("Enter the position separated by whitespace: ");
    //         io::flush().expect("Failed to flush.");
    //         if let Some((x, y)) = input::<(f64, f64)>() {
    //             break (x, y);
    //         } else {
    //             println!("Invalid input. Please try again.");
    //         }
    //     };

    //     let (vx, vy) = loop {
    //         print!("Enter the velocity separated by whitespace: ");
    //         io::flush().expect("Failed to flush.");
    //         if let Some((x, y)) = input::<(f64, f64)>() {
    //             break (x, y);
    //         } else {
    //             println!("Invalid input. Please try again.");
    //         }
    //     };

    //     let mass = loop {
    //         print!("Enter the mass of the particle: ");
    //         io::flush().expect("Failed to flush.");
    //         if let Some(mass) = input::<f64>() {
    //             break mass;
    //         } else {
    //             println!("Invalid input. Please try again.");
    //         }
    //     };

    //     let radius = loop {
    //         print!("Enter the radius of the particle: ");
    //         io::flush().expect("Failed to flush.");
    //         if let Some(radius) = input::<f64>() {
    //             break radius;
    //         } else {
    //             println!("Invalid input. Please try again.");
    //         }
    //     };
    //     
    // // particles.push(particle1);
    // // particles.push(particle2);
    // }















    // let mut i = 0;
    // loop { 
    //     println!("Intial parameters:\n");
    //     println!("PARTICLE1: {:#?}", particles[0]);
    //     println!("PARTICLE2: {:#?}", particles[1]);
    //     println!("---------------------------------------");


    //     if let Some((time, new_particle1, new_particle2)) = detect_collision(particles[0], particles[1]) {
    //         particles[0] = new_particle1;
    //         particles[1] = new_particle2;

    //         println!("time = {}", time);
    //         println!("PARTICLE1: {:#?}", particles[0]);
    //         println!("PARTICLE2: {:#?}", particles[1]);

    //         println!("\n==============================\n\n");
    //     } else {
    //         break;
    //     }

    //     if i > 3 {
    //         break;
    //     };
    //     i += 1;
    // }

    // println!("\n\nI AM i {}\n", i);
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


// fn input<T: std::str::FromStr>() -> Option<T> {
//     let mut buffer = String::new();
//     io::stdin()
//         .read_line(&mut buffer)
//         .expect("Failed to read number."); 
// 
//     match buffer.trim().parse::<T>() {
//         Ok(value) => Some(value),
//         Err(_)  => {
//             None
//         }
//     }
// }
