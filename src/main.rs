
/*

//Some Rust references:
------------------------------------------------
	let x: &[(usize, usize)] = &[(1,2), (3,9)];
	for &(a,b) in x.iter() {
		println!("a is {}, b is {}", a, b);
	}
------------------------------------------------
let x: &[(f32, f32)] = &[(1.0,2.0), (3.0,9.0)];
test_particle.get_distance(x);
------------------------------------------------	
<niconii> neals: .pow() on integers, .powi() and .powf() on floats
*/


extern crate rand;
use rand::Rng;
use rand::distributions::normal::StandardNormal;
use std::mem;
use std::f32;


struct Coordinate {
    x: f32,
    y: f32,
}

struct Noise {
	distance_noise: f32,
	turning_noise: f32,
	sensor_noise: f32,
}

struct Particle {
    x: f32,
    y: f32,
    heading: f32,
}



impl Particle {

    fn move_particle(&mut self, mut distance: f32, turning_angle: f32) {
		
	    self.heading = self.heading % (2.0*f32::consts::PI);
	    self.x += (distance * self.heading.cos()) as f32;
	    self.y += (distance * self.heading.sin()) as f32;
	    println!("particle is at ({}, {}), facing {} rads", self.x, self.y, self.heading);
    
	}

    fn get_distance(&self, lm: &Coordinate) -> f32 {
			 return ( ((self.x - lm.x).powf(2.0) + (self.y - lm.y).powf(2.0)).sqrt() );	
    }
   
}


fn predict(particles: &mut Vec<Particle>, mut distance: f32, turning_angle: f32, noise_values: &Noise) {

	for p in particles.iter_mut() {

		let distance_with_noise: f32 = distance + noise_values.distance_noise * gaussian_sample();
		let turning_angle_with_noise: f32 = turning_angle + (noise_values.turning_noise * gaussian_sample());

		p.move_particle(distance_with_noise, turning_angle_with_noise);
        		
	}

}





fn take_measurement(particle_at_actual_location: Particle, landmarks: &Vec<Coordinate>) -> Vec<f32> {
	
	let mut sensor_noise: f32 = 0.005;
	let mut samples: Vec<f32> = Vec::new();

	for lm in landmarks.iter() {
		let mut distance_measurement: f32 = ( (particle_at_actual_location.x - lm.x).powf(2.0) + (particle_at_actual_location.y - lm.y).powf(2.0) ).sqrt();
		distance_measurement += sensor_noise*gaussian_sample();
		
		samples.push(distance_measurement);
		println!("distance is {}", distance_measurement);
	}
	return samples;
}

fn generate_landmarks() -> Vec<Coordinate> {
	let mut ls: Vec<Coordinate> = Vec::new();
	ls.push(Coordinate{x: 0.0, y: 0.0});
	ls.push(Coordinate{x: 0.0, y: 10.0});
	ls.push(Coordinate{x: 10.0, y: 10.0});
	ls.push(Coordinate{x: 10.0, y: 0.0});
	return ls;
}



fn generate_particles(n: i32) -> Vec<Particle> {
	
//    let mut test_particle = Particle { x: 0., y: 0., heading: (3.141/2.) };
//    test_particle.move_particle(1.5, 1.3, 1.2);
//    println!("Particle is at ({}, {}), with heading {} radians",
//               test_particle.x, test_particle.y, test_particle.heading);

	
    let mut v: Vec<Particle> = Vec::new();
	
    let mut rng = rand::thread_rng();    
    for i in 0..n {
		let mut temp_x: f32 = rng.gen_range(0.0f32, 11.0e0f32);
		let mut temp_y: f32 = rng.gen_range(0.0f32, 11.0e0f32);
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




fn update(particles: &mut Vec<Particle>, landmarks: Vec<Coordinate>, measurements: Vec<f32>, noise_values: &Noise) -> Vec<f32> {

	let mut particle_weights: Vec<f32> = Vec::new();
	
//	println!("particle weights are {} {} {}", particle_weights[0], particle_weights[1], particle_weights[2]);
	for (p_i,p) in particles.iter().enumerate() {
		let mut weight: f32 = 1.0;
		for (i, lm) in landmarks.iter().enumerate() {
			//println!("i is {}", i);
			p.get_distance(lm);
			println!("get distance is {}", p.get_distance(lm));
			println!("gaussian values are {}, {}, {}", p.get_distance(lm), noise_values.sensor_noise, measurements[i]);
			weight *= gaussian_distribution(p.get_distance(lm), noise_values.sensor_noise, measurements[i]);
					println!("weight is {}", weight);
		}
		println!("particle i is {}", p_i);
		println!("weight is {}", weight);
		particle_weights.push(weight);
	}
	return particle_weights;
}



fn main() {

    let mut v: Vec<Particle> = generate_particles(1000);	
        
	println!("length of particle vector is {}",v.len());      

	let particle_noise = Noise {distance_noise: 3.0, turning_noise: 1.0, sensor_noise: 1.0};


    let mut test_particle = Particle { x: 0., y: 0., heading: (0.0) };
    test_particle.move_particle(10.0, 3.14159/2.0);	

	

	let landmarks: Vec<Coordinate> = generate_landmarks();
	let mut measurements: Vec<f32> = take_measurement( test_particle, &landmarks);

	predict(&mut v, 10.0, 3.14159/2.0, &particle_noise);	//args: array of particles, distance, heading.*

//	for m in measurements.iter() { println!("m is {}", m); }

	let mut probs: Vec<f32> = update(&mut v, landmarks, measurements, &particle_noise);
	
	for i in 0..probs.len() {
		if  probs[i] > 0.00000001 {
			println!("got one! {}", probs[i]);
			println!("location is ({},{})", v[i].x, v[i].y);
		}
	}
	
	
	println!("manual gaussian is {}", gaussian_distribution(28.213606, 0.001, 14.135057));
/*
-------> Next steps to make update function work:
	1. Make it so that function calls to p.move() method us same principles as predict() function
			Better yet, take funky code out of predict(), copy it into p.move() instead, and make predict() call p.move().
			
	2. move true_particle to (10, pi/2), get measurements, then call predict with (10, pi/2), and feed these into update().
	

	println!("gaussian is {}", gaussian_distribution(1., 1.0, 0.));


    let y = rand::random::<f32>();
    println!("{}", y);
*/    
}


fn gaussian_distribution(mu: f32, sigma: f32, x: f32) -> f32 {

	return f32::consts::E.powf(-1.0*((mu - x).powf(2.0)) / (sigma.powf(2.0)) / 2.0) / (2.0 * f32::consts::PI * (sigma.powf(2.0))).powf(0.5)

}

