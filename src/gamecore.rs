/// Overall states for the game
pub enum GameState {
    Loading,
    MainMenu,
}

/// This structure contains the entire game state, and should be passed around to various logic functions.
pub struct GameCore {

    /// The game's overall state
    pub state: GameState

}
