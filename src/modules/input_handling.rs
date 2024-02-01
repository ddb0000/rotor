use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct InputHandler {
    event_pump: EventPump,
}

impl InputHandler {
    pub fn new(event_pump: EventPump) -> Self {
        InputHandler { event_pump }
    }

    pub fn process_input(&mut self) -> Vec<InputEvent> {
        let mut input_events = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                // Handle key press events
                Keycode::Space => return InputEvent::Attack,
                Keycode::E => return InputEvent::Interact,
                Keycode::Q => input_events.push(InputEvent::UseItem("Health Potion".to_string())),
                Keycode::W => input_events.push(InputEvent::Move(Direction::Up)),
                Keycode::S => input_events.push(InputEvent::Move(Direction::Down)),
                Keycode::D => input_events.push(InputEvent::Move(Direction::Right)),
                Keycode::A => input_events.push(InputEvent::Move(Direction::Left)),

                // Add more controls as needed
                _ => {}
            }
        }
        InputEvent::None
    }
}

pub enum InputEvent {
    None,
    Move(Direction),
    Attack,
    Interact,
    UseItem(String),
    // Add more input events as needed
}
