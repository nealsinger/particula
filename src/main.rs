
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
use std::f32;

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

fn gaussian_sample() -> f32 {
	let StandardNormal(random_noise) = rand::random();
	println!("gaussian sample is {}", random_noise);
	return random_noise as f32;
}


fn predict(particles: &mut Vec<Particle>, mut distance: f32, turning_angle: f32) {

	let distance_noise: f32 = 0.25;
	let turning_noise: f32 = 0.25;

	distance += distance_noise * gaussian_sample();	
	
	let d_heading: f32 = turning_angle + (turning_noise * gaussian_sample());
    for p in particles {        
        (*p).heading = (p.heading + d_heading) % (2.0*f32::consts::PI);
      	let d_x: f32 = distance * p.heading.cos();	//all this reassignment is intentional,
		let d_y: f32 = distance * p.heading.sin();	//it helps the compiler infer types.
        (*p).x += d_x;
        (*p).y += d_y; 
        println!("particle is at ({}, {}), facing {} rads", p.x, p.y, p.heading);
    }
}

/*
fn update(particles: &mut Vec<Particle>, measurements: &mut Vec<Measurement>) {

	

}
*/

fn main() {

    let mut v: Vec<Particle> = generate_particles(3);	
        
	println!("length of particle vector is {}",v.len());      

	predict(&mut v, 10.1, 3.141);	//particles, distance, heading.


    
    let y = rand::random::<f32>();
    println!("{}", y);
    
}
