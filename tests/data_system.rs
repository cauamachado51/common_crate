use common_crate::ds::{parse_array, parse_vec};

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
    match parse_array::<String, 3>("[a, , c]") {
        Ok(array) => println!("{:?}", array), // ["a", "", "c"]
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
    match parse_array::<u8, 3>("[1, , 3]") {
        Ok(array) => println!("{:?}", array),
        Err(error) => println!("{}", error), // Falha ao parsear elemento ''
    }
}

#[test]
fn test_parse_vec() {
    // é pra dar certo
    match parse_vec::<u8>("[1, 2, 3]") {
        Ok(vec) => println!("{:?}", vec), // [1, 2, 3]
        Err(error) => println!("{}", error),
    }
    match parse_vec::<bool>("true, false, true") {
        Ok(vec) => println!("{:?}", vec), // [true, false, true]
        Err(error) => println!("{}", error),
    }
    match parse_vec::<String>("a, , c") {
        Ok(vec) => println!("{:?}", vec), // ["a", "", "c"]
        Err(error) => println!("{}", error),
    }
    // é pra dar errado
    match parse_vec::<u8>("1, , 3") {
        Ok(vec) => println!("{:?}", vec), 
        Err(error) => println!("{}", error), // Falha ao parsear elemento ''
    }
    match parse_vec::<u8>("[1, b, 3]") {
        Ok(vec) => println!("{:?}", vec),
        Err(error) => println!("{}", error), // Falha ao parsear elemento 'b'
    }
}