extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("Roguelike", 640, 400)
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {}
            }
        }
    }
}
