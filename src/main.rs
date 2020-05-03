extern crate lazy_static;

mod colors;
use colors::COLORS;

use std::io::{Read, Write, stdout, Stdout};
use std::thread;
use std::process;
use std::time::Duration;

extern crate termion;
use termion::color;
use termion::terminal_size;
use termion::async_stdin;
use termion::raw::IntoRawMode;

const MS_WAIT: u64 = 500;

fn clear_screen(stdout: &mut Stdout) {
    // Clear the screen
}

fn fill(v: &mut Vec<(u8, u8, u8)>, tick: usize, height: u16, width: u16) {
    let mut i = 0;
    for _ in 0..height - 1 {
        for _ in 0..width {
            v[i] = COLORS[tick % 2];
            i += 1;
        }
    }

    for _ in 0..width {
        v[i] = COLORS[tick % 2];
        i += 1;
    }
}

fn render(stdout: &mut Stdout, v: &Vec<(u8, u8, u8)>, height: u16, width: u16) {
    for position in 0..(height * width) as usize {
        let (r, g, b) = v[position];
        write!(stdout, "{} ", color::Bg(color::Rgb(r, g, b))).unwrap();
    }
    stdout.flush().unwrap();
}

fn main() {
    // Listen async user key(s)
    let mut stdin = async_stdin().bytes();

    // Initialize the std output
    let stdout = stdout();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let size = terminal_size();
    if let Err(error) = size {
        println!("cannot get terminal size: {}", error);
        process::exit(1);
    }
    let (width, height): (u16, u16) = size.unwrap();

    let mut buffer: Vec<(u8, u8, u8)> = vec![(255u8, 255u8, 255u8); (width * height) as usize];
    let mut tick: usize = 0;
    
    loop {
        // 100 ms to get the input
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }
        
        // Be prepared to render on screen
        write!(stdout,
            "{}{}",
            termion::cursor::Goto(1, 1,),
            termion::cursor::Hide)
            .unwrap();
            
        fill(&mut buffer, tick, height, width);
        render(&mut stdout, &buffer, height, width);

        thread::sleep(Duration::from_millis(MS_WAIT));
        tick += 1;
    }
}
