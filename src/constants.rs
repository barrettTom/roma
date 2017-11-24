#![allow(dead_code)]

pub enum Colors {
    None,
    Grass,
    Tree,
    White,
    BlueUnit,
}

pub enum Orders {
    Wait,
    Move,
    Wander,
}

pub enum Commands {
    Go,
    Grid,
    Finish,
}
