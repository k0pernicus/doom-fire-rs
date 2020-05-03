extern crate lazy_static;
extern crate rand;

mod colors;
use colors::COLORS;

use std::io::{stdout, Read, Stdout, Write};
use std::process;
use std::thread;
use std::time::Duration;

extern crate termion;
use termion::async_stdin;
use termion::color;
use termion::raw::IntoRawMode;
use termion::terminal_size;

const MS_WAIT: u64 = 5;

fn render(stdout: &mut Stdout, pixels_fire: &mut Vec<u8>, height: u16, width: u16) {
    for x in 0..width {
        for y in 1..height {
            let src = (y * width + x) as usize;
            let pixel = pixels_fire[src];
            if pixel == 0u8 {
                continue;
            }
            let spread_fire_accelerator: u8 = (rand::random::<f64>() * 3.0).floor() as u8 & 3;
            let dst = src - spread_fire_accelerator as usize + 1;
            if dst >= width as usize {
                pixels_fire[dst - width as usize] =
                    pixels_fire[src] - (spread_fire_accelerator & 1) as u8;
            }
        }
    }

    for pixel in 0..height * width {
        let color_index = pixels_fire[pixel as usize] as usize;
        if color_index == 0 {
            write!(stdout, "{} ", color::Bg(color::Rgb(0u8, 0u8, 0u8))).unwrap();
            continue;
        }
        let (r, g, b) = COLORS[color_index - 1];
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

    let mut pixels_fire = vec![0u8; (width * height) as usize];
    for x in 0..width {
        let index = (height - 1) * width + x;
        pixels_fire[index as usize] = 35u8;
    }

    loop {
        // 100 ms to get the input
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            break;
        }

        // Be prepared to render on screen
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1,),
            termion::cursor::Hide
        )
        .unwrap();

        render(&mut stdout, &mut pixels_fire, height, width);

        thread::sleep(Duration::from_millis(MS_WAIT));
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
