#[derive(Debug, Clone, Copy)]
pub enum Players {
    Player1,
    Player2,
}

pub enum PlayerType {
    Human,
    AI { difficulty: Difficulty },
}

// Various difficulties the AI can be set to.
pub enum Difficulty {
    Random,
}
