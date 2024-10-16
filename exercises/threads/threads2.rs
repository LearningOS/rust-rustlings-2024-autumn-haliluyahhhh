use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            
            // Lock the mutex before modifying the shared data
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // After all threads have joined, print the final value
    println!("jobs completed: {}", status.lock().unwrap().jobs_completed);
}
