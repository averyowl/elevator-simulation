/// PersonId newtype, should be unique for each person
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PersonId(pub u32);

/// CarId newtype, should be unique for each car
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CarId(pub u32);

/// Direction enum used for exterior buttons
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
}

/// It's less important that floor is type safe, so I made it a type alias
pub type Floor = u32;
