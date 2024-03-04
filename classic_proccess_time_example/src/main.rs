use std::{thread, time};
use std::thread::JoinHandle;

fn sixty_nine(num: u64, thread: i8) -> u64 {
    println!("Thread {} will sleep for {} seconds", thread, num);
    let sex = time::Duration::new(num.try_into().unwrap(), 0);
    thread::sleep(sex);
    num
}

fn main() {
    // parallel executuion
    println!("Total running time: {} seconds", sixty_nine(4, 1) + sixty_nine(2, 2) + sixty_nine(0, 3));

    // now using threads
    let now = time::Instant::now();

    let thread_1: JoinHandle<u64> = thread::spawn( || {
        sixty_nine(6, 1)
    });
    let thread_2: JoinHandle<u64> = thread::spawn( || {
        sixty_nine(6, 2)
    });
    let thread_3: JoinHandle<u64> = thread::spawn( || {
        sixty_nine(6, 3)
    });

    let res_1 = thread_1.join();
    let res_2 = thread_2.join();
    let res_3 = thread_3.join();
    let sequential_sum:u64 = res_1.unwrap() + res_2.unwrap() + res_3.unwrap();
    let time_elapsed = now.elapsed().as_secs();
    println!("Time elapsed: {} ", &time_elapsed);
    println!("Individual times added together: {} seconds", &sequential_sum);
    println!("You saved {} seconds using threads", &sequential_sum  - &time_elapsed);
}
