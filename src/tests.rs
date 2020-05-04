use super::*;

#[test]
fn test_str_preprocessor() {
    let n = Engine::new();
    assert_eq!(n.str_preprocessor("[]"), vec![]);
}
