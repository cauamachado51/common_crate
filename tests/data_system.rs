use common_crate::{key_value, parse_array};

#[test]
fn test_parse_array() {
    // é pra dar certo
    match parse_array::<u8, 3>("[1, 2, 3]") {
        Ok(array) => println!("{:?}", array), // [1, 2, 3]
        Err(error) => println!("{}", error),
    }
    match parse_array::<bool, 3>("true, false, true") {
        Ok(array) => println!("{:?}", array), // [true, false, true]
        Err(error) => println!("{}", error),
    }
    match parse_array::<String, 3>("a, b, c") {
        Ok(array) => println!("{:?}", array), // ["a", "b", "c"]
        Err(error) => println!("{}", error),
    }
    // é pra dar errado
    match parse_array::<u8, 3>("[1, 2, 3, 4]") {
        Ok(array) => println!("{:?}", array),
        Err(error) => println!("{}", error), // Esperado 3 elementos, encontrado 4
    }
    match parse_array::<u8, 4>("[1, 2, 3]") {
        Ok(array) => println!("{:?}", array),
        Err(error) => println!("{}", error), // Esperado 4 elementos, encontrado 3
    }
    match parse_array::<u8, 3>("[1, b, 3]") {
        Ok(array) => println!("{:?}", array),
        Err(error) => println!("{}", error), // Falha ao parsear elemento 'b'
    }
    /* é pra panicar
    let array: [u8; 5] = parse_array("[1, 2, 3]").unwrap();
    println!("{:?}", array);
    let array: [u8; 3] = parse_array("[1, b, 3]").unwrap();
    println!("{:?}", array);
    */
}

#[test]
fn test_key_value() {
    let algo = key_value("chave:valor");
    match algo {
        Some((chave, valor)) => { assert_eq!(chave, "chave"); assert_eq!(valor, "valor") },
        None => assert!(false),
    }
}