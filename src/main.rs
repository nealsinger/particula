
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


struct Distance_or_Coordinate {
    x: f32,
    y: f32,
    number: i32,
}


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
		
    fn get_distances( self, distances: &mut Vec<Distance_or_Coordinate>, landmarks: Vec<Distance_or_Coordinate> ) ->  &mut Vec<Distance_or_Coordinate> {
		
		//let mut distances: Vec<Distance_or_Coordinate> = Vec::new();
		for lm in landmarks.iter() {    	
			distances.push(Distance_or_Coordinate {x: (self.x - lm.x).abs(), y: (self.y - lm.y).abs(), number: lm.number });
			//println!("distance between particle and landmark {} is ({}, {}),", 
			//lm.number, distances[(lm.number - 1) as usize].x , distances[(lm.number - 1) as usize].y );	
			//Here's what's going on in the above line:
			//1: vectors can only be indexed by usize types, so there's a cast.
			//2: since landmarks start from one, there will be an off-by-one error unless you subtract 1.
		}
		return distances;
    }
}


fn take_measurement(true_particle: Particle, landmarks: Vec<Distance_or_Coordinate>) -> Vec<Distance_or_Coordinate> {
	
	//let mut test_particle = Particle { x: 0., y: 0., heading: (3.141/2.) };
	let mut samples: Vec<Distance_or_Coordinate> = Vec::new();

	true_particle.get_distances(&mut samples, landmarks);
	let	sensor_noise = 0.02;
	let noise = sensor_noise*gaussian_sample();
	println!("noise is {}", noise);
	for s in samples.iter_mut() {
		println!("sample.x = {}", s.x);
		(*s).x += noise;
		(*s).y += noise;
	}

	return samples;
}

fn generate_landmarks() -> Vec<Distance_or_Coordinate> {
	let mut ls: Vec<Distance_or_Coordinate> = Vec::new();
	ls.push(Distance_or_Coordinate{x: 0.0, y: 0.0, number: 1});
	ls.push(Distance_or_Coordinate{x: 0.0, y: 10.0, number: 2});
	ls.push(Distance_or_Coordinate{x: 10.0, y: 10.0, number: 3});
	ls.push(Distance_or_Coordinate{x: 10.0, y: 0.0, number: 4});
	return ls;
}

fn main() {

    let mut v: Vec<Particle> = generate_particles(3);	
        
	println!("length of particle vector is {}",v.len());      

	predict(&mut v, 10.1, 3.141);	//particles, distance, heading.

    let mut test_particle = Particle { x: 0., y: 0., heading: (3.141/2.) };
    test_particle.move_particle(1.5, 1.3, 1.2);	

	let landmarks: Vec<Distance_or_Coordinate> = generate_landmarks();
	let mut measurements: Vec<Distance_or_Coordinate> = take_measurement( test_particle,landmarks);
	
	for m in measurements.iter() {
		println!("distances: ({},{})", measurements[(m.number-1) as usize].x,measurements[(m.number-1) as usize].y);	
	}
	


    let y = rand::random::<f32>();
    println!("{}", y);
    
}


fn generate_particles(n: i32) -> Vec<Particle> {
	
//    let mut test_particle = Particle { x: 0., y: 0., heading: (3.141/2.) };
//    test_particle.move_particle(1.5, 1.3, 1.2);
//    println!("Particle is at ({}, {}), with heading {} radians",
//               test_particle.x, test_particle.y, test_particle.heading);

	
    let mut v: Vec<Particle> = Vec::new();
	
    let mut rng = rand::thread_rng();    
    for i in 0..n {
		let mut temp_x: f32 = rng.gen_range(-10.0f32, 4.0e1f32);
		let mut temp_y: f32 = rng.gen_range(-10.0f32, 4.0e1f32);
		let mut temp_heading: f32 = rng.gen_range(-10.0f32, 10.0e1f32) % 3.14159;				
		
        v.push(Particle { x: temp_x, y: temp_y, heading: temp_heading});
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

// 






