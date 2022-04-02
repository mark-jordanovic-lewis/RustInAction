use graphics::math::{Vec2d, add, mul_scalar};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, System, Layout};
use std::time::Instant;

struct ReportingAllocator;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity:  Vec2d<f64>,
    acceleration:  Vec2d<f64>,
    colour: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;
        let x_vel = 0.0;
        let y_vel = rng.gen_range(-2.0..0.0);
        let x_acc = 0.0;
        let y_acc = rng.gen_range(0.0..0.15);

        Particle {
            height: 4.0,
            width: 4.0,
            position: [x, y].into(),
            velocity: [x_vel, y_vel].into(),
            acceleration: [x_acc, y_acc].into(),
            colour: [1.0, 1.0, 1.0, 0.99]
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, 0.7);
        self.colour[3] *= 0.995;
    }
}

struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

impl World {
    fn new(height: f64, width: f64) -> Self {
        World {
            current_turn: 0,
            particles: vec![],
            height,
            width,
            rng: thread_rng()
        }
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(&self);
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;
            let particle_iter = self.particles.iter().enumerate();

            for (i, particle) in particle_iter {
                if particle.colour[3] < 0.02 {
                    to_delete = Some(i)
                }
                break
            }

            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            }
        }
    }

    fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);
        if n > 0 {
            self.add_shapes(n)
        } else {
            self.remove_shapes(n)
        };

        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update()
        }
        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
            .exit_on_esc(true)
            .build()
            .expect("Could not create a window");
    let mut world = World::new(width, height);
    world.add_shapes(1000);
    while let Some(event) = window.next() {
        world.update();
        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15,0.17,0.17, 0.9], renderer);
            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.colour, size, ctx.transform, renderer);
            }
        });
    }
}
