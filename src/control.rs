use crate::elevator::{BuildingState, ElevatorCommand};
use crate::types::Floor;

/// This is a trait which allows you to swap between different methods of elevator control
pub trait ElevatorController {
    fn tick(&mut self, state: &BuildingState) -> Vec<ElevatorCommand>;
}

/// An empty struct on which to implement ElevatorController, to use as an object
/// which will perform elevator control
pub struct BasicController;

impl ElevatorController for BasicController {
    /// Based on the building's state, generate a vector of ElevatorCommands
    /// which tell elevators to go to target floors
    fn tick(&mut self, state: &BuildingState) -> Vec<ElevatorCommand> {
        let mut commands = Vec::new();

        // for each floor process hall buttons
        for floor_state in &state.floors {
            if !floor_state.out_up && !floor_state.out_down {
                continue;
            }

            if state.cars.is_empty() {
                break;
            }

            // check if an elevator is already headed to that floor
            let floor = floor_state.floor;
            let mut already_served = false;
            for car in &state.cars {
                if car.target_floor == Some(floor) {
                    already_served = true;
                    break;
                }

                let car_floor = car.current_floor.round() as Floor;
                if car_floor == floor && car.door_open {
                    already_served = true;
                    break;
                }
            }

            if already_served {
                continue;
            }

            let mut best_car_index: Option<usize> = None;
            let mut best_distance = f32::MAX;

            // for each car
            for (i, car) in state.cars.iter().enumerate() {
                if car.target_floor.is_some() {
                    continue;
                } //if the car doesn't have a target floor already
                // find the car which is the closest to the target floor
                let distance = (car.current_floor - floor_state.floor as f32).abs();
                if distance < best_distance {
                    best_distance = distance;
                    best_car_index = Some(i);
                }
            }

            //if we found a viable car that wasn't busy
            if let Some(car_id) = best_car_index {
                let car_id = state.cars[car_id].id;

                commands.push(ElevatorCommand::MoveCarTo {
                    car_id,
                    floor: floor_state.floor,
                });
            }
        }

        // process interior elevator buttons
        for car in &state.cars {
            for (floor_index, &pressed) in car.car_buttons.iter().enumerate() {
                if !pressed {
                    continue;
                }

                // issue commands to move the car to every pressed interior button
                commands.push(ElevatorCommand::MoveCarTo {
                    car_id: car.id,
                    floor: floor_index as Floor,
                });
            }
        }

        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elevator::{ElevatorCarState, FloorState};
    use crate::types::CarId;

    #[test]
    fn no_commands_when_nothing_pressed() {
        let floors = vec![
            FloorState {
                floor: 0,
                out_up: false,
                out_down: false,
            },
            FloorState {
                floor: 1,
                out_up: false,
                out_down: false,
            },
        ];

        let cars = vec![ElevatorCarState {
            id: CarId(0),
            current_floor: 0.0,
            target_floor: None,
            door_open: false,
            car_buttons: vec![false, false],
        }];

        let state = BuildingState { floors, cars };
        let mut controller = BasicController;

        let commands = controller.tick(&state);
        assert!(commands.is_empty());
    }

    #[test]
    fn no_commands_when_all_cars_busy() {
        let floors = vec![
            FloorState {
                floor: 0,
                out_up: false,
                out_down: false,
            },
            FloorState {
                floor: 1,
                out_up: false,
                out_down: true,
            },
        ];

        let cars = vec![ElevatorCarState {
            id: CarId(0),
            current_floor: 0.0,
            target_floor: Some(1),
            door_open: false,
            car_buttons: vec![false, false],
        }];

        let state = BuildingState { floors, cars };
        let mut controller = BasicController;

        let commands = controller.tick(&state);
        assert!(commands.is_empty());
    }
}
