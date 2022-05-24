mod restaurant_api;
mod models;
mod usage_simulation;
use std::thread;

fn main() {
    loop {
        thread::spawn(|| { usage_simulation::call_api(); });
        thread::spawn(|| { usage_simulation::call_api(); });
        thread::spawn(|| { usage_simulation::call_api(); });
        thread::spawn(|| { usage_simulation::call_api(); });
        thread::spawn(|| { usage_simulation::call_api(); });
        thread::spawn(|| { usage_simulation::call_api(); });
        thread::spawn(|| { usage_simulation::call_api(); });
        thread::spawn(|| { usage_simulation::call_api(); });
        thread::spawn(|| { usage_simulation::call_api(); });
        let handle = thread::spawn(|| {
            usage_simulation::call_api();
        });
        handle.join().unwrap();
    }
}