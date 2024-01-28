extern crate sdl2;
extern crate specs;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use specs::{Builder, World, WorldExt, DispatcherBuilder, Join};


// Components
struct Position { x: f32, y: f32 }
struct Velocity { x: f32, y: f32 }
struct Renderable { width: u32, height: u32, color: Color }

// Specs implementations
impl specs::Component for Position {
    type Storage = specs::VecStorage<Self>;
}
impl specs::Component for Velocity {
    type Storage = specs::VecStorage<Self>;
}
impl specs::Component for Renderable {
    type Storage = specs::VecStorage<Self>;
}


// Systems
struct MovementSystem;
impl<'a> specs::System<'a> for MovementSystem {
    type SystemData = (specs::WriteStorage<'a, Position>, specs::ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}

struct RenderSystem;
impl<'a> specs::System<'a> for RenderSystem {
    type SystemData = (specs::ReadStorage<'a, Position>, specs::ReadStorage<'a, Renderable>);

    fn run(&mut self, (pos, rend): Self::SystemData) {
        // Rendering is handled in the main loop in this case
    }
}

fn render(canvas: &mut WindowCanvas, pos: &specs::ReadStorage<Position>, rend: &specs::ReadStorage<Renderable>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for (pos, rend) in (pos, rend).join() {
        let rectangle = Rect::new(pos.x as i32, pos.y as i32, rend.width, rend.height);
        canvas.set_draw_color(rend.color);
        canvas.fill_rect(rectangle).expect("Could not fill rect");
    }

    canvas.present();
}

fn main() {
    // SDL2 setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Game Window", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Specs world setup
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Renderable>();

    // Create player entity
    let player_entity = world.create_entity()
        .with(Position { x: 100.0, y: 100.0 })
        .with(Velocity { x: 0.0, y: 0.0 })
        .with(Renderable { width: 50, height: 50, color: Color::RGB(255, 0, 0) })
        .build();

    // Systems setup
    let mut dispatcher = DispatcherBuilder::new()
        .with(MovementSystem, "movement_system", &[])
        .with_thread_local(RenderSystem)
        .build();

    // Main loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let mut vel_storage = world.write_storage::<Velocity>();
                    let player_vel = vel_storage.get_mut(player_entity).unwrap();
                    player_vel.x = -5.0;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let mut vel_storage = world.write_storage::<Velocity>();
                    let player_vel = vel_storage.get_mut(player_entity).unwrap();
                    player_vel.x = 5.0;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let mut vel_storage = world.write_storage::<Velocity>();
                    let player_vel = vel_storage.get_mut(player_entity).unwrap();
                    player_vel.y = -5.0;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let mut vel_storage = world.write_storage::<Velocity>();
                    let player_vel = vel_storage.get_mut(player_entity).unwrap();
                    player_vel.y = 5.0;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    let mut vel_storage = world.write_storage::<Velocity>();
                    let player_vel = vel_storage.get_mut(player_entity).unwrap();
                    player_vel.x = 0.0;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    let mut vel_storage = world.write_storage::<Velocity>();
                    let player_vel = vel_storage.get_mut(player_entity).unwrap();
                    player_vel.y = 0.0;
                },
                _ => {}
            }
        }

        dispatcher.dispatch(&world);
        world.maintain();

        let positions = world.read_storage::<Position>();
        let renderables = world.read_storage::<Renderable>();
        render(&mut canvas, &positions, &renderables);
    }
}