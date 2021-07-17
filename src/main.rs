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

// implement_buffer_content!(ObservedHeight);
// The above macro doesn't implement glium::buffer::Content
// correctly. Here is a partial implementation (just enough to trigger the problem)
unsafe impl glium::buffer::Content for ObservedHeight {
    type Owned = Box<ObservedHeight>;

    #[inline]
    fn read<F, E>(size: usize, _: F) -> Result<Self::Owned, E> where F: FnOnce(&mut Self) -> Result<(), E> {
        todo!()
    }

    #[inline]
    fn get_elements_size() -> usize {
        use std::mem;
        let fake_ptr: &ObservedHeight = unsafe { mem::transmute((0usize, 0usize)) };
        let size = mem::size_of_val(fake_ptr);
        println!("Size of val: {}", size);
        size
    }

    #[inline]
    fn to_void_ptr(&self) -> *const () {
        println!("to_void_ptr");
        &self as *const _ as *const ()
    }

    #[inline]
    fn ref_from_ptr<'a>(ptr: *mut (), size: usize) -> Option<*mut Self> {
        todo!()
    }

    #[inline]
    fn is_size_suitable(size: usize) -> bool {
        println!("is_size_suitable");
        use std::mem;

        let fake_ptr: &ObservedHeight = unsafe { mem::transmute((0usize, 0usize)) };
        // The idea here is that min_size should become the smallest possible size
        // of the runtime sized object. In this example-program that size is 0
        // (since the array of ObservedHeight can be 0-length).
        let min_size = mem::size_of_val(fake_ptr);

        let fake_ptr: &ObservedHeight = unsafe { mem::transmute((0usize, 1usize)) };
        // The idea here is that step should become the size of each array element
        // in the final array of the dynamically sized object. In this case 4,
        // because the array in ObservedHeight is of f32.
        let step = mem::size_of_val(fake_ptr) - min_size;

        // The calculation of min_size and step here works in debug mode,
        // but not in release.

        println!("Min size: {}, step: {} ", min_size, step);
        size > min_size && (size - min_size) % step == 0

    }
}



pub fn main() {
    let mut watcher = TkWatcher::start();
    let agent_count = 1;
    let events_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    println!("About to create display");
    let display: glium::Display = glium::Display::new(window, context, &events_loop).unwrap();
    let s = 512 * 512 * 4;
    println!("About to create uniform buffer");
    let obsbuffer : Result<glium::uniforms::UniformBuffer<ObservedHeight>,_> = glium::uniforms::UniformBuffer::empty_unsized_persistent(&display, s as usize);
    println!("empty_unsized_persistent returned. Calling unwrap");
    obsbuffer.unwrap();
    println!("Done");

}
