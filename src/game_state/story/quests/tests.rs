use crate::game_state::actions::Actions;
use crate::game_state::story::quests::init_quests;

#[test]
fn test_init_quests() {
    let actions = Actions::new();
    init_quests(&actions);
}
