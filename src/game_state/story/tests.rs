use crate::game_state::player_actions::PlayerActions;
use crate::game_state::story::Story;

#[test]
fn test_story_serde() {
    let actions = PlayerActions::new();
    let story = Story::new(&actions);
    let serialized = serde_json::to_vec(&story).unwrap();
    let deserialized = serde_json::from_slice(&serialized).unwrap();
    assert_eq!(story, deserialized);
}
