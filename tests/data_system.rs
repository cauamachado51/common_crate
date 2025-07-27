use common_crate::{key_value, parse_array};

#[test]
fn test_parse_array() {
    let array1: [u8; 3] = parse_array("[1, 2, 3]");
    let array2 = parse_array::<bool, 3>("true, false, true");
    let array3: [String; 3] = parse_array("a, b, c");
    assert_eq!(array1, [1, 2, 3]);
    assert_eq!(array2, [true, false, true]);
    assert_eq!(array3, ["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[test]
fn test_key_value() {
    let algo = key_value("chave:valor");
    match algo {
        Some((chave, valor)) => { assert_eq!(chave, "chave"); assert_eq!(valor, "valor") },
        None => assert!(false),
    }
}