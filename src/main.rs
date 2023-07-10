use rand::Rng;
use rand::seq::SliceRandom;

fn simulate_monty_hall(switch_choice: bool) -> bool {
    let mut rng = rand::thread_rng();

    // Initialize the doors
    let doors = vec![1, 2, 3];

    // Randomly place the prize behind one of the doors
    let prize_door = rng.gen_range(1..=3);

    // Make the initial choice
    let initial_choice = rng.gen_range(1..=3);

    // Determine which door the host opens
    let remaining_doors: Vec<i32> = doors.iter().filter(|&&door| door != initial_choice).cloned().collect();
    let host_open = if initial_choice == prize_door {
        *remaining_doors.choose(&mut rng).unwrap()
    } else {
        remaining_doors.iter().find(|&&door| door != prize_door).unwrap().clone()
    };

    // Switch the choice if requested
    let final_choice = if switch_choice {
        *doors.iter().find(|&&door| door != initial_choice && door != host_open).unwrap()
    } else {
        initial_choice
    };

    // Determine if the final choice is correct
    final_choice == prize_door
}

fn run_simulations(num_simulations: u32, switch_choice: bool) -> f32 {
    let mut successes = 0;

    for _ in 0..num_simulations {
        let success = simulate_monty_hall(switch_choice);
        if success {
            successes += 1;
        }
    }

    let success_rate = successes as f32 / num_simulations as f32;

    success_rate
}

fn main() {
    let num_simulations = 100_000;
    let switch_choice = true; // Set to false if you want to always stick with the initial choice

    let success_rate = run_simulations(num_simulations, switch_choice);
    println!("Success rate: {:.2}%", success_rate * 100.0);
}
