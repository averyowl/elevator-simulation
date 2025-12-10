use elevator_simulation::control::{ElevatorController, BasicController};
use elevator_simulation::elevator::ElevatorSim;
use elevator_simulation::elevator::{BuildingState, ElevatorCommand};
use elevator_simulation::people::{PeopleSim, Person, PersonAction, PersonState};
use std::{env, thread, time::Duration};

///ties together PeopleSim, ElevatorSim, and ElevatorController
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut floors: u32 = 10;
    let mut num_elevators = 2;
    let mut steps = 2000;
    
    if args.len() > 4 {
        eprintln!("Too many arguments.
Usage: cargo run -- [floors] [num_elevators] [steps]");
        std::process::exit(1);
    }

    if args.len() > 1 {
        match args[1].parse() {
            Ok(floor_num) => floors = floor_num,
            Err(e) => eprintln!("Error: {e}: Floors must be a positive integer"),
        };
    }

    if args.len() > 2 {
        match args[2].parse() {
            Ok(elevator_num) => num_elevators = elevator_num,
            Err(e) => eprintln!("Error: {e}: num_elevators must be a positive integer"),
        };
    }

    if args.len() > 3 {
        match args[3].parse() {
            Ok(steps_num) => steps = steps_num,
            Err(e) => eprintln!("Error: {e}: steps must be a positive integer"),
        };
    }

    let mut people = PeopleSim::new(floors, 3.);
    let mut building = ElevatorSim::new(floors as usize, num_elevators);
    let mut controller = BasicController;

    //amount to advance the simulation by
    let timestep = 0.1;

    for _ in 0..steps {
        // step PeopleSim, and get the vector of PersonActions
        let person_action = people.tick(timestep, building.state());
        for act in person_action {
            //translate those PersonActions into ElevatorCommands
            if let Some(cmd) = person_action_to_cmd(act) {
                building.apply_command(cmd);
            }
        }

        //get the building state and pass it to the controller to get ElevatorCommands
        let state = building.state();
        let control_cmds = controller.tick(state);
        for cmd in control_cmds {
            //apply all elevator commands
            building.apply_command(cmd);
        }

        building.tick(timestep);

        render(building.state(), people.people());

        thread::sleep(Duration::from_millis(25));
    }
}

/// Translate PersonActions to ElevatorCommands
fn person_action_to_cmd(action: PersonAction) -> Option<ElevatorCommand> {
    match action {
        //If a person tries to call an elevator, press the outer button
        PersonAction::CallElevator { floor, direction } => {
            Some(ElevatorCommand::PressOutButton { floor, direction })
        }
        //If a person tries to press an interior car button, press the interior car button
        PersonAction::PressCarButton { car_id, floor } => {
            Some(ElevatorCommand::PressCarButton { car_id, floor })
        }
    }
}

/// Render the BuildingState and Person locations
fn render(state: &BuildingState, people: &[Person]) {
    let num_floors = state.floors.len();
    let num_elevators = state.cars.len();

    let mut waiting_counts = vec![0; num_floors];
    let mut riding_counts = vec![0; num_elevators];

    //get the number of people waiting at each floor and in each elevator car
    for person in people {
        match person.state {
            PersonState::Waiting => {
                //for each person waiing, add 1 to waiting_counts
                let index = person.current_floor;
                waiting_counts[index as usize] += 1;
            }
            PersonState::Riding => {
                if let Some(car_id) = person.in_car {
                    //for each person in an elevator car, add 1 to riding_counts
                    let index = car_id.0;
                    riding_counts[index as usize] += 1;
                }
            }
            //other states, New, Done, don't matter in rendering
            _ => {}
        }
    }

    //for each floor
    for floor_index in (0..num_floors).rev() {
        let floor_state = &state.floors[floor_index];

        //create up and down arrow buttons
        let up = if floor_state.out_up { '^' } else { '.' };
        let down = if floor_state.out_down { 'v' } else { '.' };

        let waiting = waiting_counts[floor_index];

        let mut elevator_cells = Vec::new();
        //for each elevator car
        for car in &state.cars {
            let car_floor = car.current_floor.round() as u32;
            let here = car_floor == floor_state.floor;

            //determine if the car is on this floor
            if here {
                let riders = riding_counts[car.id.0 as usize];
                let id = car.id.0;
                //create elevator car print text
                elevator_cells.push(format!("{id}({riders})"));
            } else {
                //if the elevator is not here, replace with .
                elevator_cells.push("  . ".to_string());
            }
        }

        let join_cells = elevator_cells.join(" ");

        let floor = floor_state.floor;
        //print each floor in this format
        println!("Floor: {floor} [{up}{down}] Waiting: {waiting} | {join_cells}")
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use elevator_simulation::types::{CarId, Direction};

    #[test]
    fn call_elevator_to_press_out_button() {
        let cmd = person_action_to_cmd(PersonAction::CallElevator {
            floor: 3,
            direction: Direction::Up,
        });
        match cmd {
            Some(ElevatorCommand::PressOutButton { floor, .. }) => {
                assert_eq!(floor, 3)
            }
            _ => panic!(),
        }
    }

    #[test]
    fn press_car_button_to_press_car_button() {
        let cmd = person_action_to_cmd(PersonAction::PressCarButton {
            car_id: CarId(0),
            floor: 3,
        });
        match cmd {
            Some(ElevatorCommand::PressCarButton { car_id, floor }) => {
                assert_eq!(car_id, CarId(0));
                assert_eq!(floor, 3)
            }
            _ => panic!(),
        }
    }
}
