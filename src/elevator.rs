use crate::types::{CarId, Direction, Floor};

/// The state of an entire building, which contains a vector of the state of each floor,
/// along with a vector of the state of each elevator car
#[derive(Clone, Debug, PartialEq)]
pub struct BuildingState {
    pub floors: Vec<FloorState>,
    pub cars: Vec<ElevatorCarState>,
}

/// The state of each floor, which contains its floor number, and outer buttons
#[derive(Clone, Debug, PartialEq)]
pub struct FloorState {
    pub floor: Floor,
    pub out_up: bool,
    pub out_down: bool,
}

/// The state of each elevator car, which contains its id number, current floor/location as a
/// float, target floor if it exists, whether the door is open, and a vector of car buttons
#[derive(Clone, Debug, PartialEq)]
pub struct ElevatorCarState {
    pub id: CarId,
    pub current_floor: f32,
    pub target_floor: Option<Floor>,
    pub door_open: bool,
    pub car_buttons: Vec<bool>,
}

/// A list of possible elevator commands
pub enum ElevatorCommand {
    MoveCarTo { car_id: CarId, floor: Floor },
    PressOutButton { floor: Floor, direction: Direction },
    PressCarButton { car_id: CarId, floor: Floor },
}

/// an elevatorsim struct contains a building state, and an impl to change that state based on
/// ElevatorCommands
#[derive(Debug)]
pub struct ElevatorSim {
    state: BuildingState,
}

/// Implement the required functions to modify the building's state
/// new - create a new building
/// applycommand - apply an ElevatorCommand to the building state
/// car_mut - return a mutable referance to a given elevator car state
/// tick - move elevators, open doors, dismiss call buttons
impl ElevatorSim {
    /// create a new building, with number of floors and number of cars
    pub fn new(floor_num: usize, cars_num: usize) -> Self {
        let mut floors_vec = Vec::new();
        for i in 0..floor_num {
            let floor_state = FloorState {
                floor: i as Floor,
                out_up: false,
                out_down: false,
            };
            floors_vec.push(floor_state)
        }
        let mut cars_vec = Vec::new();
        for i in 0..cars_num {
            let car_state = ElevatorCarState {
                id: CarId(i as u32),
                current_floor: 0.,
                target_floor: None,
                door_open: false,
                car_buttons: vec![false; floor_num], //create in each elevator car the correct
                                                     //number of buttons
            };
            cars_vec.push(car_state)
        }
        ElevatorSim {
            state: BuildingState {
                floors: floors_vec,
                cars: cars_vec,
            },
        }
    }

    /// Apply an ElevatorCommand to the BuildingState
    pub fn apply_command(&mut self, cmd: ElevatorCommand) {
        match cmd {
            // pressing the outer button on a specific floor pointing in a direction
            ElevatorCommand::PressOutButton { floor, direction } => {
                if let Some(f) = self.state.floors.get_mut(floor as usize) {
                    match direction {
                        Direction::Up => f.out_up = true,
                        Direction::Down => f.out_down = true,
                    }
                }
            }
            // pressing the button inside an elevator car
            ElevatorCommand::PressCarButton { car_id, floor } => {
                if let Some(car) = self.car_mut(car_id) {
                    if let Some(slot) = car.car_buttons.get_mut(floor as usize) {
                        *slot = true;
                    }
                }
            }
            // setting the target floor of an elevator car, which also closes its door
            ElevatorCommand::MoveCarTo { car_id, floor } => {
                if let Some(car) = self.car_mut(car_id) {
                    car.target_floor = Some(floor);
                    car.door_open = false;
                }
            }
        }
    }

    /// get a mutable referance to a particular elevator car, based on its id.
    /// With more time, I would impl functions on the elevator car to do everything
    /// necessary here
    fn car_mut(&mut self, car_id: CarId) -> Option<&mut ElevatorCarState> {
        self.state.cars.get_mut(car_id.0 as usize)
    }

    /// move elevator cars, if they are at their target floor, open their doors
    pub fn tick(&mut self, dt: f32) {
        for car in &mut self.state.cars {
            if let Some(target) = car.target_floor {
                //for each car with a target floor
                let target_f = target as f32;
                //get the difference between its target and current location
                let diff = target_f - car.current_floor;
                let speed = 1.0;
                if diff.abs() < 0.01 {
                    // if the elevator is close to its target floor, say we're there and open the
                    // door
                    car.current_floor = target_f;
                    car.target_floor = None;
                    car.door_open = true;

                    let floor_index = target as usize;

                    // reset the outer buttons on the floor
                    if let Some(floor_state) = self.state.floors.get_mut(floor_index) {
                        floor_state.out_up = false;
                        floor_state.out_down = false;
                    }

                    // reset the button inside the elevator for this floor
                    if let Some(button) = car.car_buttons.get_mut(floor_index) {
                        *button = false;
                    }
                } else {
                    // move the elevator car down or up based on the direction it needs to move
                    let step = speed * dt * (if diff > 0. { 1. } else { -1. });
                    car.current_floor += step;
                }
            }
        }
    }

    // return a referance to the entire building state, used in render and PeopleSim
    pub fn state(&self) -> &BuildingState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CarId, Direction};

    #[test]
    fn press_out_button() {
        let mut sim = ElevatorSim::new(3, 1);

        sim.apply_command(ElevatorCommand::PressOutButton {
            floor: 1,
            direction: Direction::Up,
        });

        assert!(sim.state().floors[1].out_up);
        assert!(!sim.state().floors[1].out_down);
    }

    #[test]
    fn press_car_button() {
        let mut sim = ElevatorSim::new(3, 1);
        sim.apply_command(ElevatorCommand::PressCarButton {
            car_id: CarId(0),
            floor: 2,
        });

        assert!(sim.state().cars[0].car_buttons[2])
    }

    #[test]
    fn tick_moves_car() {
        let mut sim = ElevatorSim::new(3, 1);
        sim.apply_command(ElevatorCommand::MoveCarTo {
            car_id: CarId(0),
            floor: 1,
        });
        sim.tick(1.0);
        let car = &sim.state().cars[0];
        assert!(car.target_floor == Some(1));
        assert!(car.current_floor != 0.0);
    }
}
