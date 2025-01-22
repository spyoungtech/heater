use std::thread;
use std::time::{Duration, SystemTime};
use core_affinity;
use clap::Parser;
fn burn_cpu_until(target_time: SystemTime) {
    // Busy-loop until we reach the target time
    while SystemTime::now() < target_time {
        // Perform some trivial work to keep the CPU busy
        let mut sum = 0u64;
        for i in 0..100_000 {
            sum = sum.wrapping_add(i ^ (i >> 1));
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, help = "the duration, in seconds, to run the heater")]
    duration: u64
}


fn main() {
    // Get a list of the available CPU cores
    let args = Cli::parse();


    let cores = core_affinity::get_core_ids().expect("Failed to get core IDs");
    let mut handles = Vec::new();


    for (core_index, core_id) in cores.into_iter().enumerate() {
        // For each core, we'll spawn 2 threads pinned to that same core
        for sub_thread_id in 0..2 {
            let handle = thread::spawn(move || {
                // Pin this thread to the specified core
                core_affinity::set_for_current(core_id);
                let duration = Duration::from_secs(args.duration);
                let target_time = SystemTime::now() + duration;
                burn_cpu_until(target_time);
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
