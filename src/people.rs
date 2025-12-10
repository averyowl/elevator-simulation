use crate::elevator::BuildingState;
use crate::types::{CarId, Direction, Floor, PersonId};
use rand::Rng;

/// enum of actions people can take
#[derive(Debug)]
pub enum PersonAction {
    CallElevator { floor: Floor, direction: Direction },
    PressCarButton { car_id: CarId, floor: Floor },
}

/// enum of states people can be in
#[derive(Debug)]
pub enum PersonState {
    New,
    Waiting,
    Riding,
    Done,
}

/// Person object, contains an id, current floor, target floor, state, and
/// an optional elevator car id
#[derive(Debug)]
pub struct Person {
    pub id: PersonId,
    pub current_floor: Floor,
    pub target_floor: Floor,
    pub state: PersonState,
    pub in_car: Option<CarId>,
}

/// PeopleSim object contains
/// next_person_id - the id of the next person who will spawn
/// num_floors - the number of floors in the building
/// spawn_timer - a timer which increments until it reaches spawn_interval
/// spawn_interval - a value to adjust how often new people are spawned
/// people - a vector of people
pub struct PeopleSim {
    next_person_id: u32,
    num_floors: Floor,
    spawn_timer: f32,
    spawn_interval: f32,
    people: Vec<Person>,
}

/// implement functions for PeopleSim
/// new - create a new peoplesim object, with a certain number of floors
/// people - return a slice of People
/// tick - spawns a person, and then for each person makes decisions and generates PersonActions
impl PeopleSim {
    /// Create a new PeopleSim, with a particular number of floors
    pub fn new(num_floors: Floor, spawn_interval: f32) -> Self {
        Self {
            next_person_id: 0,
            num_floors,
            spawn_timer: 0.,
            spawn_interval,
            people: Vec::new(),
        }
    }

    /// Return a slice of all people
    pub fn people(&self) -> &[Person] {
        &self.people
    }

    /// Take in BuildingState, and return a vector of PersonActions, which main
    /// can translate into ElevatorActions
    pub fn tick(&mut self, dt: f32, building: &BuildingState) -> Vec<PersonAction> {
        let mut actions: Vec<PersonAction> = Vec::new();

        self.spawn_timer += dt;

        if self.spawn_timer >= self.spawn_interval {
            self.spawn_timer = 0.0;

            let id = PersonId(self.next_person_id);
            self.next_person_id += 1;

            // create a person on a random start floor, with a random target floor
            let start_floor = rand::rng().random_range(0..self.num_floors);
            let mut target_floor = rand::rng().random_range(0..self.num_floors);
            while start_floor == target_floor {
                //ensure the target floor is not the same as the start floor
                target_floor = rand::rng().random_range(0..self.num_floors);
            }

            let person = Person {
                id,
                current_floor: start_floor,
                target_floor,
                state: PersonState::New,
                in_car: None,
            };

            self.people.push(person);
        }

        // for each person, make the decisions they need to make
        for person in self.people.iter_mut() {
            match person.state {
                //if someone is new, they need to push the outer buttons as long as there is no
                //elevator there already, or the buttons are already pressed
                PersonState::New => {
                    let mut car_here = false;
                    //check each car in the building
                    for car in &building.cars {
                        //don't worry about cars whose doors aren't open
                        if !car.door_open {
                            continue;
                        }

                        // if it's on the current floor, don't push the outer button
                        let car_floor = car.current_floor.round() as Floor;
                        if car_floor == person.current_floor {
                            car_here = true;
                            break;
                        }
                    }
                    //if there is no car here, push the outer button
                    if !car_here {
                        let direction = if person.target_floor > person.current_floor {
                            Direction::Up
                        } else {
                            Direction::Down
                        };

                        actions.push(PersonAction::CallElevator {
                            floor: person.current_floor,
                            direction,
                        });
                    }

                    //now the new person can start waiting
                    person.state = PersonState::Waiting;
                }
                //if a person is waiting, they need to check if there is a car on their current
                //floor with its door open. If there is, they need to enter that car
                PersonState::Waiting => {
                    let mut car_to_board: Option<CarId> = None;
                    //for each car in the building
                    for car in &building.cars {
                        //don't worry about cars with closed doors
                        if !car.door_open {
                            continue;
                        }

                        //if it's on the current floor
                        let car_floor = car.current_floor.round() as Floor;
                        if car_floor == person.current_floor {
                            car_to_board = Some(car.id); //set it as the car to board
                            break;
                        }
                    }

                    //if we got a car to board
                    if let Some(car_id) = car_to_board {
                        //enter the car and push the interior button
                        actions.push(PersonAction::PressCarButton {
                            car_id,
                            floor: person.target_floor,
                        });

                        //the person is now riding the elevator car
                        person.state = PersonState::Riding;
                        person.in_car = Some(car_id);
                    }
                }
                //if a person is riding an elevator car
                PersonState::Riding => {
                    //make sure they're in a car
                    if let Some(car_id) = person.in_car {
                        //make sure that car is in the building
                        if let Some(car) = building.cars.get(car_id.0 as usize) {
                            let car_floor = car.current_floor.round() as Floor;

                            //if the car is where they want to go, and the door is open
                            if car_floor == person.target_floor && car.door_open {
                                //get out
                                person.current_floor = person.target_floor;
                                person.in_car = None;
                                //the person is now done
                                person.state = PersonState::Done;
                            }
                        }
                    }
                }
                PersonState::Done => {}
            }
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elevator::BuildingState;

    fn empty_building() -> BuildingState {
        BuildingState {
            floors: Vec::new(),
            cars: Vec::new(),
        }
    }

    #[test]
    fn person_spawned_and_calls_elevator() {
        let mut sim = PeopleSim::new(5, 0.1);
        let building = empty_building();

        let actions = sim.tick(1.0, &building);

        assert_eq!(sim.people().len(), 1);
        assert_eq!(actions.len(), 1);
    }
}
