
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
use rand::{thread_rng, Rng};
use rand::distributions::normal::StandardNormal;
use std::mem;
use std::f32;
use std::i32;

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
    probability: f32,
}



impl Particle {


	fn clone(&self) -> Self {
		return Particle {x: self.x,  y: self.y, heading: self.heading, probability: self.probability}; 
	}

    fn move_particle(&mut self, mut distance: f32, turning_angle: f32) {
		
	    self.heading = self.heading % (2.0*f32::consts::PI);
	    self.x += (distance * self.heading.cos()) as f32;
	    self.y += (distance * self.heading.sin()) as f32;
//	    println!("particle is at ({}, {}), facing {} rads", self.x, self.y, self.heading);
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




fn take_measurement(particle_at_actual_location: &Particle, landmarks: &Vec<Coordinate>) -> Vec<f32> {
	
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
		let mut temp_x: f32 = rng.gen_range(-10.0f32, 20.0e0f32);
		let mut temp_y: f32 = rng.gen_range(-10.0f32, 20.0e0f32);
		let mut temp_heading: f32 = rng.gen_range(-10.0f32, 10.0e1f32) % 3.14159;				
		
        v.push(Particle { x: temp_x, y: temp_y, heading: temp_heading, probability: 1.0});
    }

    return v;
	    	
}

fn gaussian_sample() -> f32 {
	let StandardNormal(random_noise) = rand::random();
//	println!("gaussian sample is {}", random_noise);
	return random_noise as f32;
}




fn update(particles: &mut Vec<Particle>, landmarks: &Vec<Coordinate>, measurements: Vec<f32>, noise_values: &Noise) //-> Vec<f32> 
{

	//let mut particle_weights: Vec<f32> = Vec::new();
	
//	println!("particle weights are {} {} {}", particle_weights[0], particle_weights[1], particle_weights[2]);
	for (p_i,p) in particles.iter_mut().enumerate() {
		//let mut weight: f32 = 1.0;
		for (i, lm) in landmarks.iter().enumerate() {
			//println!("i is {}", i);
			p.get_distance(lm);
//			println!("get distance is {}", p.get_distance(lm));
//			println!("gaussian values are {}, {}, {}", p.get_distance(lm), noise_values.sensor_noise, measurements[i]);
			p.probability *= gaussian_distribution(p.get_distance(lm), noise_values.sensor_noise, measurements[i]);
//					println!("weight is {}", weight);
		}
//		println!("particle i is {}", p_i);
//		println!("weight is {}", weight);
		//particle_weights.push(weight);
	}
	//return particle_weights;
}





fn find_max_probability(unsorted_particles: &Vec<Particle>) -> f32 {
//Rust doesn't support Ord for f32, so you can't use .max() or .cmp()

	let mut max_value: f32 = 0.0;

	for part in unsorted_particles.iter() {
		if (part.probability - max_value) > 0.0 {
			max_value = part.probability.clone();
		}	
	}
	return max_value;
}

fn resample(probable_particles: &Vec<Particle> ) -> Vec<Particle> {
	let max_probability: f32 = find_max_probability(&probable_particles);
	let mut threshold: f32 = 0.0;
	let mut rng = thread_rng();
	let mut n: usize = rng.gen_range(0, probable_particles.len() as usize);
	let mut resampled_probabilities: Vec<Particle> = Vec::new();
	for p in probable_particles.iter() {
		let temp_rand: f32 = rng.gen();	//TODO: move this to line below using type hinting
		threshold += temp_rand * 2.0 * max_probability;
		
		while (threshold > probable_particles[n].probability) {
			threshold -= probable_particles[n].probability;
			n = (n + 1) % probable_particles.len();
		}
		resampled_probabilities.push(probable_particles[n].clone());
	}
	return resampled_probabilities;
}


/*

Todo: 
1. add a probability trait to the particle struct, and modify rest of code accordingly.
2. make loop that resamples particles, moves the true particle, takes a measurement, and feeds resampled particles into predict, and then into resample.
3. make the above loop go for like three iterations, and print stats of particle with highest likelihood.

*/


fn main() {

    let mut v: Vec<Particle> = generate_particles(1000);	
        
	println!("length of particle vector is {}",v.len());      

	let particle_noise = Noise {distance_noise: 1.0, turning_noise: 1.0, sensor_noise: 1.0};
	let landmarks: Vec<Coordinate> = generate_landmarks();

    let mut test_particle = Particle { x: 0., y: 0., heading: (0.0), probability: 1.0 };

	for i in 0..2 {
	    test_particle.move_particle(10.0, 3.14159/2.0);	

		let mut measurements: Vec<f32> = take_measurement(&test_particle, &landmarks);

		predict(&mut v, 10.0, 3.14159/2.0, &particle_noise);	//args: array of particles, distance, heading.*

		update(&mut v, &landmarks, measurements, &particle_noise);
		
		v = resample(&v);
	}

/*	
	for i in 0..probabilities.len() {
		if  probabilities[i] > 1e-50f32 {	//this can be tuned for more or fewer particles
			println!("got one! {}", probabilities[i]);
			println!("location is ({},{})", v[i].x, v[i].y);
		}
	}
*/

//	let max_index: usize = find_index_of_max(&probabilities);
//	println!("max value is {}", max_index);
//	let resampled_probs: Vec<f32> = resample(&probabilities);
	
//	for i in 0..100 {
//		println!("resampled prob is {}", resampled_probs[i]);
//	}
	
	println!("manual gaussian is {}", gaussian_distribution(28.213606, 0.001, 14.135057));
    
//	let max_prob_index: usize = find_index_of_max(&resampled_probs);
//	println!("max value is {}", resampled_probs[max_prob_index]);
//	println!("number of resampled particles is {}", resampled_probs.len());

	for p in 0..100 {
		println!("resampled prob is {}", v[p].probability);
	}
	println!("max probability after three runs is {}", find_max_probability(&v));
}


fn gaussian_distribution(mu: f32, sigma: f32, x: f32) -> f32 {

	return f32::consts::E.powf(-1.0*((mu - x).powf(2.0)) / (sigma.powf(2.0)) / 2.0) / (2.0 * f32::consts::PI * (sigma.powf(2.0))).powf(0.5)

}

