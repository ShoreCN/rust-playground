#[derive(Debug)]

pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub enum State {
    Idle,
    Moving,
    Stopped,
    Overload(u8),
}

pub enum Event {
    Call(u8, Direction),
    FloorReached(u8),
    Stop,
}

#[derive(Debug)]
pub struct Elevator {
    pub current_floor: u8,
    pub direction: Direction,
    pub destination: u8,
    pub state: State,
    pub current_weight: u32,
    pub weight_limit: u32,
}