
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

    fn get_distances(&self, lm: &Coordinate) -> f32 {
			 return ( ((self.x - lm.x).powf(2.0) + (self.y - lm.y).powf(2.0)).sqrt() );	
    }
   
}


fn take_measurement(particle_at_actual_location: Particle, landmarks: Vec<Coordinate>) -> Vec<f32> {
	
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
        (*p).x += (distance * p.heading.cos()) as f32;//d_x;
        (*p).y += (distance * p.heading.sin()) as f32;//d_y; 
        println!("particle is at ({}, {}), facing {} rads", p.x, p.y, p.heading);
    }
}


fn update(particles: &mut Vec<Particle>, landmarks: Vec<Coordinate>, sensor_noise: f32, measurements: Vec<f32>) -> Vec<f32> {

	let mut particle_weights: Vec<f32> = Vec::new();
	
	for _ in 0..particles.len() {
		particle_weights.push(1.0);
	}
	println!("particle weights are {} {} {}", particle_weights[0], particle_weights[1], particle_weights[2]);
	for p in particles {
		let mut weight: f32 = 1.0;
		for (i, lm) in landmarks.iter().enumerate() {
			p.get_distances(lm);
			weight *= gaussian_distribution(p.get_distances(lm), sensor_noise, measurements[i]);
		}
		particle_weights.push(weight);
	}
	return particle_weights;
}



fn main() {

    let mut v: Vec<Particle> = generate_particles(10);	
        
	println!("length of particle vector is {}",v.len());      

	predict(&mut v, 10.1, 3.141);	//args: array of particles, distance, heading.

    let mut test_particle = Particle { x: 0., y: 0., heading: (3.141/2.) };
    test_particle.move_particle(1.5, 1.3, 1.2);	

	let landmarks: Vec<Coordinate> = generate_landmarks();
	let mut measurements: Vec<f32> = take_measurement( test_particle, landmarks);

	update(v, landmarks, 0.001, measurements);


	
	for m in measurements.iter() {
		//println!("distances: ({},{})", measurements[(m.number-1) as usize].x,measurements[(m.number-1) as usize].y);	
	}
	//update(&mut v, measurements);


	println!("gaussian is {}", gaussian_distribution(1., 1.0, 0.));


    let y = rand::random::<f32>();
    println!("{}", y);
    
}


fn gaussian_distribution(mu: f32, sigma: f32, x: f32) -> f32 {

	return f32::consts::E.powf(-1.0*((mu - x).powf(2.0)) / (sigma.powf(2.0)) / 2.0) / (2.0 * f32::consts::PI * (sigma.powf(2.0))).powf(0.5)

}

