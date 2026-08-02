use gb_core::cpu::Cpu;
use gb_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH};

const SCALE: u32 = 3;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread::sleep;
use std::time::Duration;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Game ROM not specified. Finishing.");
        return;
    }

    if matches!(args[1].as_str(), "--help" | "help" | "-h" | "h") {
        println!("Usage: gb.exe game.gb");
        return;
    } else if matches!(args[1].as_str(), "--version" | "version" | "-v" | "v") {
        println!("gb.exe version 0.1.0\nCopyright (c) 2026 Saulo Henrique. All Rights Reserved.");
        println!(
            "This is free software; see the source for copying conditions.  There is NO\nwarranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE."
        );
        return;
    }

    let mut gb = Cpu::new();
    let filename = &args[1];
    let rom = load_rom(filename);
    gb.load_rom(&rom);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("My Game Boy Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();
    'gameloop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'gameloop;
                }
                _ => {}
            }
        }

        sleep(Duration::from_millis(100));
    }
}

fn load_rom(path: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    let mut f = File::open(path).expect("Error opening ROM file");
    f.read_to_end(&mut buffer).expect("Error loading ROM");
    buffer
}
