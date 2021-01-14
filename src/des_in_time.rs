use std::collections::VecDeque;

/**
 * Simulate Caffe in Fixed Time Progression
 * 
 * this works by checking for each timesteps
 * is there any event or not
 */
fn simulate_caffe(num_baristas: usize, mut arrival_times: VecDeque<i32>,
                make_coffe_time: i32) -> Vec<i32>
{
    let mut queue_length = 0;
    let mut barista_time_until_free = vec![0; num_baristas];

    // save the queue data for analyze
    let mut queue_length_over_time: Vec<i32> = Vec::new();

    // Simulate 8 hours in a day
    // and convert to minutes
    for timestep in 0..(8*60) {
        // check if there is customer arrived
        if arrival_times.iter().any(|&time| time == timestep) {
            queue_length += 1;
            arrival_times.pop_front();
            println!("Customer arrived!");
            println!("queue length {:?}", &queue_length);
        }

        // check available baristas
        for barista in 0..num_baristas {
            if barista_time_until_free[barista] == 0 {
                if queue_length > 0 {
                    queue_length =- 1;
                    barista_time_until_free[barista] = make_coffe_time;
                }
            }
            else {
                barista_time_until_free[barista] -= 1;
            }
        }

        queue_length_over_time.push(queue_length);
    }

    queue_length_over_time
}
