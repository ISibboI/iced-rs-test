use crate::game_state::player_actions::PlayerActions;
use crate::game_state::story::quests::init_quests;

#[test]
fn test_init_quests() {
    let actions = PlayerActions::new();
    init_quests(&actions);
}
