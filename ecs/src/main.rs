//! A CLI game to demonstrate [ECS] system.
//!
//! [ecs]: https://specs.amethyst.rs/docs/tutorials/

use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use ecs::data;
use ecs::system;
use ecs::{EcsStore, GenManager, VecStore};

fn main() {
    // get keyboard input in a thready manner.
    let (tx, rx) = mpsc::channel();

    // a terminal reader.
    thread::spawn(move || {
        let stdin = io::stdin();
        for k in stdin.keys() {
            // termion::TermRead
            tx.send(k).ok();
        }
    });

    // termion::IntoRawMode
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    let mut screen = io::stdout().into_raw_mode().unwrap();
    let mut gen = GenManager::new();
    let mut strengths = VecStore::new();
    let mut dirs = VecStore::new();
    let mut pos = VecStore::new();
    let mut pass = 0;

    loop {
        // create one element per loop
        let g = gen.next_gen();
        strengths.add(g, data::Strength { s: 1, h: 5 });
        dirs.add(g, data::Dir { vx: 0, vy: 0 });
        pos.add(
            g,
            data::Pos {
                x: (rand::random::<i32>() % w),
                y: (rand::random::<i32>() % h),
            },
        );

        system::dir_sys(&mut dirs, &pos);
        system::move_sys(&dirs, &mut pos);
        system::collision_sys(&pos, &mut strengths);
        system::death_sys(&mut gen, &mut strengths, &mut pos, &mut dirs);
        system::render_sys(&mut screen, &pos, &strengths);

        write!(&mut screen, "{}Pass={}", termion::cursor::Goto(1, 1), pass).ok();
        pass += 1;
        screen.flush().ok();

        while let Ok(Ok(k)) = rx.try_recv() {
            if let Key::Char('q') = k {
                return;
            }
        }
        thread::sleep(Duration::from_millis(200));
    }
}
