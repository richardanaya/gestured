use input::event::gesture::GestureEventCoordinates;
use input::event::gesture::GestureEventTrait;
use input::{Libinput, LibinputInterface};
use std::fs::{File, OpenOptions};
use std::os::unix::{
    fs::OpenOptionsExt,
    io::{FromRawFd, IntoRawFd, RawFd},
};
use std::path::Path;
use std::str::FromStr;

extern crate libc;
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use clap::Parser;

/// Gesture daemon for launching applications based on gestures from libinput.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Allows you to bind a gesture
    #[clap(short, long, required = true)]
    gestures: Vec<String>,

    #[clap(short, long, default_value_t = 125.0)]
    threshold: f64,
}

fn main() {
    let args = Args::parse();

    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    let mut x = 0.0;
    let mut y = 0.0;

    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            if let input::event::Event::Gesture(ev) = &event {
                if let input::event::gesture::GestureEvent::Swipe(s) = &ev {
                    if let input::event::gesture::GestureSwipeEvent::Begin(_b) = &s {
                        x = 0.0;
                        y = 0.0;
                    }
                    if let input::event::gesture::GestureSwipeEvent::Update(u) = &s {
                        x += u.dx();
                        y += u.dy();
                    }
                    if let input::event::gesture::GestureSwipeEvent::End(e) = &s {
                        let fingers = e.finger_count();
                        let len = (x * x + y * y).sqrt();
                        let ang = y.atan2(x).to_degrees();
                        let dir = if ang < 45.0 && ang > -45.0 {
                            Direction::Right
                        } else if ang > 45.0 && ang < 135.0 {
                            Direction::Down
                        } else if ang < -45.0 && ang > -135.0 {
                            Direction::Up
                        } else {
                            Direction::Left
                        };

                        if len > args.threshold {
                            for g in &args.gestures {
                                let parts = g.split(",");
                                let gesture_cmd = parts.collect::<Vec<&str>>();
                                let expect_fingers = i32::from_str(gesture_cmd[0]).unwrap();
                                if expect_fingers == fingers {
                                    if (dir == Direction::Up
                                        && gesture_cmd[1] == "D"
                                        && gesture_cmd[2] == "U")
                                        || (dir == Direction::Right
                                            && gesture_cmd[1] == "L"
                                            && gesture_cmd[2] == "R")
                                        || (dir == Direction::Down
                                            && gesture_cmd[1] == "U"
                                            && gesture_cmd[2] == "D")
                                        || (dir == Direction::Left
                                            && gesture_cmd[1] == "R"
                                            && gesture_cmd[2] == "L")
                                    {
                                        let gesture_cmd_command = gesture_cmd[3].to_owned();
                                        std::thread::spawn(move || {
                                            if let Some(argv) = shlex::split(&gesture_cmd_command) {
                                                let cmd = &argv[0];
                                                let args = &argv[1..];
                                                std::process::Command::new(cmd)
                                                    .args(args)
                                                    .spawn()
                                                    .expect("failed to start");
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
