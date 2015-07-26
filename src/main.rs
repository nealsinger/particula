extern crate rand;
use rand::Rng;

struct Robot {
    x: f32,
    y: f32,
    heading: f32,
}

impl Robot {
    fn move_bot(&mut self, d_x: f32, d_y: f32, d_heading: f32) {
        self.x += d_x;
        self.y += d_y;
        self.heading += d_heading;
    }
}

fn main() {

    let mut rng = rand::thread_rng();
    
    let mut my_bot = Robot { x: 0., y: 0., heading: (3.141/2.) };
    my_bot.move_bot(1.5, 1.3, 1.2);
    println!("My bot is at ({}, {}), with heading {} radians",
               my_bot.x, my_bot.y, my_bot.heading);

    let mut v: Vec<Robot> = Vec::new();
    v.push(my_bot);
    println!("length of vector is {}", v.len());
    

    
    for i in 0 .. 10 {
		let mut temp_x: f32 = rng.gen_range(-10.0f32, 4.0e1f32);
		let mut temp_y: f32 = rng.gen_range(-10.0f32, 4.0e1f32);
		let mut temp_heading: f32 = rng.gen_range(-10.0f32, 10.0e1f32) % 3.14159;				
		
        v.push(Robot { x: temp_x, y: temp_y, heading: temp_heading });
    }
    println!("{}",v.len());   
    
   /*
    for (index, bot) in &mut v.iter().enumerate() {
        (*bot).x += 100.;
        println!("x location of bot is {}, iteration is {}", bot.x, index);
    }
   */ 
    for bot in &mut v {
        (*bot).x += 1.;
        println!("x location of bot is {}", bot.x);
    }
    let y = rand::random::<f32>();
    println!("{}", y);
    
}
