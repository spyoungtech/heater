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
    #[arg(short, long, help = "the duration, in seconds, to run the heater. If a negative value is provided, runs forever")]
    duration: i32,

    #[arg(short, long, help = "the number of threads per core to spawn", default_value = "2")]
    threads_per_core: u32,

    #[arg(short, long, help = "the number of cores to occupy. If unspecified, uses all cores. If the number specified is higher than the number of CPU cores available, all cores will be used.")]
    cores: Option<u32>
}


fn main() {
    // Get a list of the available CPU cores
    let args = Cli::parse();


    let cores = core_affinity::get_core_ids().expect("Failed to get core IDs");
    let mut handles = Vec::new();

    let mut duration_input: i32 = args.duration;
    if duration_input.is_negative() {
        duration_input = i32::MAX;
    }
    if duration_input == 0 {
        return // TODO: warn?
    }
    assert!(duration_input.is_positive());
    let duration = Duration::from_secs(duration_input as u64);
    let threads_per_core: u32 = args.threads_per_core;
    if threads_per_core == 0 {
        return // TODO: warn?
    }
    let target_time = SystemTime::now() + duration;
    let num_cores = match args.cores {
        Some(core_arg) => {
            core_arg as usize
        },
        None => {
            usize::MAX
        }
    };
    if num_cores == 0 {
        return // TODO: warn?
    }

    for (core_index, core_id) in cores.into_iter().enumerate() {
        if core_index + 1 > num_cores {
            break
        }
        for _ in 0..threads_per_core {
            let handle = thread::spawn(move || {
                // Pin this thread to the specified core
                core_affinity::set_for_current(core_id);
                burn_cpu_until(target_time);
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
