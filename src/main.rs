#[macro_use]
extern crate glium;
extern crate notify;

use glium::glutin;
use std::cell::RefCell;
use std::{time, thread};
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::time::Duration;
use std::path::Path;
use std::sync::mpsc::{channel, TryRecvError, Sender};

#[derive(Clone)]
pub struct TkWatcher {
    sender: Arc<RefCell<()>>,
}

impl TkWatcher {
    pub fn start() -> TkWatcher {
        use self::notify::Watcher;
        // Most of this is minimized nonsense, but changing it affects the program behaviour
        let (cmdtx, cmdrx) = channel::<Arc<Mutex<()>>>();
        let (tx, rx) = channel();

        thread::Builder::new().name("TkWatcher".to_string()).spawn(move || {
            // Starting this notify watcher in another thread seems to be needed
            // to trigger the problem.

            let mut watcher: notify::RecommendedWatcher = notify::Watcher::new(tx, Duration::from_secs(1)).unwrap();
            std::thread::park();
        }).unwrap();

        TkWatcher {
            sender: Arc::new(RefCell::new(())),
        }
    }
}


struct ObservedHeight {
    observed_heights: [f32],
}

implement_buffer_content!(ObservedHeight);

pub fn main() {
    let mut watcher = TkWatcher::start();
    let agent_count = 1;
    let events_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display: glium::Display = glium::Display::new(window, context, &events_loop).unwrap();
    let s = 512 * 512 * 4;
    let obsbuffer : glium::uniforms::UniformBuffer<ObservedHeight> = glium::uniforms::UniformBuffer::empty_unsized_persistent(&display, s as usize).unwrap();

}
