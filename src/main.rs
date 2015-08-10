
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

extern crate gnuplot;
use gnuplot::*;



struct Coordinate_ {  //Added underscore, because otherwise it conflicts with gnuplot.
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
		
	    self.heading = turning_angle % (2.0*f32::consts::PI);
	    self.x += (distance * self.heading.cos()) as f32;
	    self.y += (distance * self.heading.sin()) as f32;
//	    println!("particle is at ({}, {}), facing {} rads", self.x, self.y, self.heading);
	}

    fn get_distance(&self, lm: &Coordinate_) -> f32 {
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




fn take_measurement(particle_at_actual_location: &Particle, landmarks: &Vec<Coordinate_>) -> Vec<f32> {
	
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

fn generate_landmarks() -> Vec<Coordinate_> {
	let mut ls: Vec<Coordinate_> = Vec::new();
	ls.push(Coordinate_{x: 0.0, y: 0.0});
	ls.push(Coordinate_{x: 0.0, y: 10.0});
	ls.push(Coordinate_{x: 10.0, y: 10.0});
	ls.push(Coordinate_{x: 10.0, y: 0.0});
	return ls;
}



fn generate_particles(n: u32) -> Vec<Particle> {
		
    let mut v: Vec<Particle> = Vec::new();
	
    let mut rng = rand::thread_rng();    
    for i in 0..n {
		let mut temp_x: f32 = rng.gen_range(-30.0f32, 30.0e0f32);
		let mut temp_y: f32 = rng.gen_range(-30.0f32, 30.0e0f32);
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




fn update(particles: &mut Vec<Particle>, landmarks: &Vec<Coordinate_>, measurements: Vec<f32>, noise_values: &Noise) {

	for (p_i,p) in particles.iter_mut().enumerate() {
		for (i, lm) in landmarks.iter().enumerate() {
			p.get_distance(lm);
//			println!("get distance is {}", p.get_distance(lm));
//			println!("gaussian values are {}, {}, {}", p.get_distance(lm), noise_values.sensor_noise, measurements[i]);
			p.probability *= gaussian_distribution(p.get_distance(lm), noise_values.sensor_noise, measurements[i]);
		}
	}
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


fn main() {
	let number_of_particles: usize = 5000;
    let mut v: Vec<Particle> = generate_particles(number_of_particles as u32);	
        
	println!("length of particle vector is {}",v.len());      

	let particle_noise = Noise {distance_noise: 0.50, turning_noise: 0.50, sensor_noise: 1.0};
	let landmarks: Vec<Coordinate_> = generate_landmarks();

    let mut test_particle = Particle { x: 0.0, y: 0.0, heading: (0.0), probability: 1.0 };

	let mut forward_movement: f32 = 10.0;
	let mut heading_angle: f32 = f32::consts::PI/5.0;
	let mut rng = rand::thread_rng();    
	
	for i in 0..5 {
	    test_particle.move_particle(forward_movement, heading_angle);

		let mut measurements: Vec<f32> = take_measurement(&test_particle, &landmarks);

		predict(&mut v, forward_movement, heading_angle, &particle_noise);	//args: array of particles, distance, heading.*
		update(&mut v, &landmarks, measurements, &particle_noise);
		v = resample(&v);
//		plot_positions(&v);		//Generate a frame for each resampling, 
								//very cool - shows how the samples follow and
								//cluster around the particle.								

		forward_movement = rng.gen_range(0.0f32, 20.0e0f32);
		heading_angle = rng.gen_range(0.0f32, 3.141e0f32);			
	}
    
	for p in 0..number_of_particles {
		println!("resampled prob is {}", v[p].probability);
	}
	plot_positions(&v);
	
	println!("particle's actual position is ({}, {})", test_particle.x, test_particle.y);	
	println!("size of particle struct is {}",  mem::size_of::<Particle>());
	
}


fn gaussian_distribution(mu: f32, sigma: f32, x: f32) -> f32 {
	return f32::consts::E.powf(-1.0*((mu - x).powf(2.0)) / (sigma.powf(2.0)) / 2.0) / (2.0 * f32::consts::PI * (sigma.powf(2.0))).powf(0.5)
}


fn plot_positions(v: &Vec<Particle>) {

	let mut x: Vec<f32> = Vec::new();
	let mut y: Vec<f32> = Vec::new();
	for p in v.iter() {
		x.push(p.x);
		y.push(p.y);
	}

	let mut fg = Figure::new();
	fg.axes2d()
	.points(x.iter(), y.iter(), &[Caption("Particles"), PointSymbol('D'), Color("#ffaa77"), PointSize(1.0)])
	.set_y_range(Fix(-200.0), Fix(200.0))
	.set_x_range(Fix(-200.0), Fix(200.0))
	.set_title("Particle Locations", &[])
	.set_x_label("X", &[Font("Arial", 12.0), TextColor("red"), Rotate(45.0)])
	.set_y_label("Y", &[Font("Arial", 12.0), TextColor("red"), Rotate(90.0)]);

	fg.show();
}



