#[derive(Debug)]

pub enum Direction {
    Up,
    Down,
}

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

pub struct Elevator {
    pub current_floor: u8,
    pub direction: Direction,
    pub destination: u8,
    pub state: State,
}