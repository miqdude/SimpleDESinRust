/**
 * Caffe Simulation using Descrete Event Simulation
 * this code is reimplementation of Python code from
 * Discrete-Event Simulation with Lewis Bobbermen https://youtu.be/0KvA92ykPKI
 * 
 * author : miqdadabdurhman@gmail.com
 */
use rand::Rng;
use std::collections::VecDeque;

fn simulate_caffe(num_baristas: usize, mut arrival_times: VecDeque<i32>,
                make_coffe_time: i32) -> Vec<i32>
{
    let mut queue_length = 0;

    // store when the baristas are free instead of how long left
    let mut barista_free_at = vec![0; num_baristas];

    // timestamps for chart
    let mut timesteps = Vec::new();
    let mut queue_length_over_time = Vec::new();
    
    let mut time_queue = arrival_times.clone();

    while time_queue.len() > 0{
        let time_step = time_queue.pop_front().unwrap();
        if time_step > (8 * 60) {
            break;
        }

        // Check if anyone arrive
        if arrival_times.iter().any(|&time| time == time_step) {
            queue_length += 1;
            arrival_times.pop_front();
        }

        for barista in 0..num_baristas {
            // if there is any barista free
            if barista_free_at[barista] <= time_step {
                // server customer
                if queue_length > 0 {
                    queue_length -= 1;
                    
                    // set time when barista is free
                    barista_free_at[barista] = time_step + make_coffe_time;
                    time_queue.push_back(time_step + make_coffe_time);
                }
            }
        }

        timesteps.push(time_step);
        queue_length_over_time.push(queue_length);
    }

    queue_length_over_time
}

fn main() {
    let mut arrival_times = VecDeque::new();
    let mut rng = rand::thread_rng();

    // Generate the time when the customer arrived
    const NUM_COSTOMER:i32 = 50;
    for time in 0..NUM_COSTOMER {
        if rng.gen_range(0..=1) == 1 {
            arrival_times.push_back(time);
        }
    }

    const NUM_BARISTAS:usize = 2;
    const MAKE_COFFE_TIME:i32 = 5;

    let queue_length_over_time = simulate_caffe(NUM_BARISTAS, arrival_times, MAKE_COFFE_TIME);
    println!("{:?}", queue_length_over_time);
}