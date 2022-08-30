use crate::game_state::story::Story;

#[test]
fn test_story_serde() {
    let story = Story::default();
    let serialized = serde_json::to_vec(&story).unwrap();
    let deserialized = serde_json::from_slice(&serialized).unwrap();
    assert_eq!(story, deserialized);
}
