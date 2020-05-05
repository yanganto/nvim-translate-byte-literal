use super::*;

#[test]
fn test_str_preprocessor_return_type() {
    let n = Engine::new();
    assert_eq!(n.str_preprocessor("[]"), vec![]);
}

#[test]
fn test_str_preprocessor_return_u64_numbers() {
    let n = Engine::new();
    assert_eq!(
        n.str_preprocessor("[65, 110, 116, 111, 110, 105, 111]"),
        vec![65, 110, 116, 111, 110, 105, 111]
    );
}

#[test]
fn test_translate_number_array() {
    let n = Engine::new();
    assert_eq!(
        n.translate_number_array(vec![65, 110, 116, 111, 110, 105, 111]),
        "Antonio"
    );
}

// TODO
// #[test]
// fn test_str_preprocessor_with_leading_space() {
//     let n = Engine::new();
//     assert_eq!(
//         n.str_preprocessor("  [65, 110, 116, 111, 110, 105, 111]"),
//         vec![65, 110, 116, 111, 110, 105, 111]
//     );
// }
