
/*

//Some Rust references:
------------------------------------------------
	let x: &[(usize, usize)] = &[(1,2), (3,9)];
	for &(a,b) in x.iter() {
		println!("a is {}, b is {}", a, b);
	}
------------------------------------------------
let x: &[(f32, f32)] = &[(1.0,2.0), (3.0,9.0)];
test_particle.get_distances(x);
------------------------------------------------	
*/


extern crate rand;
use rand::Rng;
use rand::distributions::normal::StandardNormal;
use std::mem;

struct Particle {
    x: f32,
    y: f32,
    heading: f32,
    
}

impl Particle {
    fn move_particle(&mut self, d_x: f32, d_y: f32, d_heading: f32) {
        self.x += d_x;
        self.y += d_y;
        self.heading += d_heading;
    }
		
    fn get_distances( &mut self, landmarks: &[(f32, f32)] ) {
		for (i, &(x, y)) in landmarks.iter().enumerate() {    	
			println!("distance between particle and landmark {} is ({}, {}),", i, self.x - x, self.y - y);		
		}
    }
}

fn generate_particles(n: i32) -> Vec<Particle> {
	
//    let mut test_particle = Particle { x: 0., y: 0., heading: (3.141/2.) };
//    test_particle.move_particle(1.5, 1.3, 1.2);
//    println!("Particle is at ({}, {}), with heading {} radians",
//               test_particle.x, test_particle.y, test_particle.heading);
	
    let mut v: Vec<Particle> = Vec::new();
//    v.push(test_particle);

    let mut rng = rand::thread_rng();    
    for i in 0..n {
		let mut temp_x: f32 = rng.gen_range(-10.0f32, 4.0e1f32);
		let mut temp_y: f32 = rng.gen_range(-10.0f32, 4.0e1f32);
		let mut temp_heading: f32 = rng.gen_range(-10.0f32, 10.0e1f32) % 3.14159;				
		
        v.push(Particle { x: temp_x, y: temp_y, heading: temp_heading });
    }

    return v;
	    	
}

fn predict(particles: &mut Vec<Particle>) {

    for p in particles {
        (*p).x += 1.3;
        println!("x location of particle is {}", p.x);
    }

}


fn main() {

	let StandardNormal(x) = rand::random();
	println!("gaussian is {}", x);
	
    let mut rng = rand::thread_rng();

    let mut v: Vec<Particle> = generate_particles(10);
	println!("length of particle vector is {}",v.len());      
    let movement_noise: f32 = 0.2;
	let mut forward_movement = movement_noise * rng.gen_range(-5.0e-1f32, 5.0e-1f32);

	predict(&mut v);

	
    for particle in &mut v {
        (*particle).x += forward_movement + 1.3;
        println!("x location of particle is {}", particle.x);
    }
    
    
    let y = rand::random::<f32>();
    println!("{}", y);
    
}
