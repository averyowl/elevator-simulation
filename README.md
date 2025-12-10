Elevator Simulation
Author: Jacob Purvis

How to build:

To build this project, you can run

cargo build

in the project directory.

How to run:

To run this project, you can use

cargo run

in the project directory.

This project uses positional arguments to allow for customization of parameters.
The positional arguments may be used by running a command of the following form:

`cargo run -- [floors] [num_elevators] [steps]`

floors - This is the number of floors that will be simulated.
         When people appear, they will appear on a random floor, and be headed
         to a random target floor.

         Default: 10

num_elevators - This is the number of individual elevators that will be
                simulated. Each elevator operates independently, and will not
                head to pick up the same person as another elevator.

                Default: 2

steps - This is the number of steps the simulation will run for. Each step
        prints the BuildingState, and then pauses for 25 ms. One new person
        is spawned every 30 steps.

        Default: 2000

Overview:

This project simulates people using an elevator system in a building with a
variable number of floors, and a variable number of elevators. The elevator
controller uses dependency injection, which allows it to be swapped out, should
a more efficient elevator controller algorithm be designed. 

Output:

Floor: 9 [..] Waiting: 0 |   .    .
Floor: 8 [..] Waiting: 0 |   .    .
Floor: 7 [..] Waiting: 0 |   .    .
Floor: 6 [..] Waiting: 0 |   .  1(1)
Floor: 5 [^v] Waiting: 2 | 0(1)   .
Floor: 4 [..] Waiting: 0 |   .    .
Floor: 3 [..] Waiting: 0 |   .    .
Floor: 2 [..] Waiting: 0 |   .    .
Floor: 1 [..] Waiting: 0 |   .    .
Floor: 0 [..] Waiting: 0 |   .    .

Here is an example output step. Two people are currently waiting on floor 5.
They have each pressed a different elevator button, so one is heading up and
the other is heading down. There are currently two elevators, with IDs of 0 and
1 respectively. They both contain a single passenger.

Design:

I initially started with designing the building's state itself. My idea was to
use Rust's variable passing system as literally as possible to simulate people.
I was going to have the people objects *moved* into a vector inside of the
ElevatorCarState, as if they were actually getting onto the elevator. In an
earlier iteration, elevator cars contained a vector to allow them to carry
Persons. 

As the project developed, I realized this was in direct conflict with
my overall goal for the project, which was to design an elevator system that
could be used in the real world. The coupling of Persons and ElevatorCarState
had to be removed in favor of Persons keeping track of where in the
BuildingState they were. I aimed to keep the elevator module completely unaware
that people existed at all. This lead to my main being used as a translation
layer between people.rs and elevator.rs.

I was less concerned about the people module using elevator information, since
*people* were just used in this project to prove that the elevator module was
working as intended, and likely wouldn't have any real world use.

Another big part of this project was the dependency injection aspect of
ElevatorController. The ability to use different controllers on the same
BuildingState object was extremely important to me, and although I didn't get
to writing additional ElevatorControllers beyond BasicController, it would
allow in the future for multiple controllers to be contrasted against eachother.

As I was beginning this project, I initially thought I could combine multiple
library crates into one project, so that I could build something modular. This
was what was outlined in my proposal. As I explored how to begin work on this
project, it came to my attention that managing that would be a nightmare, so
I reframed the project to be 3 separate modules in one library crate instead.

AI usage disclosure:
ChatGPT was used for final code review, and asked to search for "Weird or bad
decisions". It brought up unused enum elements, PersonAction::TryEnterCar and
PersonAction::TryExitCar, which were removed. These were from the original plan
for this project, which was to have people objects moved in and out of elevator
cars. It also brought up that one of my unit tests relied on RNG to pass, which
I fixed.
