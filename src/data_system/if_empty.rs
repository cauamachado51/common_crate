/// Trait que adiciona o método if_empty para tipos que podem ser verificados se estão vazios
pub trait IfEmpty<T> {
    /// Executa uma closure e retorna seu resultado se o valor estiver vazio, caso contrário retorna o próprio valor.
    /// Exemplo:
    /// ```
    /// use common_crate::IfEmpty;
    /// 
    /// let texto = String::new();
    /// let resultado = texto.if_empty(|| "valor padrão".to_string());
    /// assert_eq!(resultado, "valor padrão");
    /// 
    /// let texto = "não vazio".to_string();
    /// let resultado = texto.if_empty(|| "valor padrão".to_string());
    /// assert_eq!(resultado, "não vazio");
    /// ```
    fn if_empty<F>(self, f: F) -> T
    where
        F: FnOnce() -> T;
}

/// Implementação do trait IfEmpty para String
impl IfEmpty<String> for String {
    fn if_empty<F>(self, f: F) -> String
    where
        F: FnOnce() -> String,
    {
        if self.is_empty() {
            f()
        } else {
            self
        }
    }
}

/// Implementação do trait IfEmpty para &str
impl<'a> IfEmpty<String> for &'a str {
    fn if_empty<F>(self, f: F) -> String
    where
        F: FnOnce() -> String,
    {
        if self.is_empty() {
            f()
        } else {
            self.to_string()
        }
    }
}

/// Implementação do trait IfEmpty para Option<String>
impl IfEmpty<String> for Option<String> {
    fn if_empty<F>(self, f: F) -> String
    where
        F: FnOnce() -> String,
    {
        match self {
            Some(s) if !s.is_empty() => s,
            _ => f(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_empty() {
        let empty_string = String::new();
        let result = empty_string.if_empty(|| "default".to_string());
        assert_eq!(result, "default");
    }

    #[test]
    fn test_string_not_empty() {
        let non_empty_string = "hello".to_string();
        let result = non_empty_string.if_empty(|| "default".to_string());
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_str_empty() {
        let empty_str = "";
        let result = empty_str.if_empty(|| "default".to_string());
        assert_eq!(result, "default");
    }

    #[test]
    fn test_str_not_empty() {
        let non_empty_str = "hello";
        let result = non_empty_str.if_empty(|| "default".to_string());
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_option_some_not_empty() {
        let some_string = Some("hello".to_string());
        let result = some_string.if_empty(|| "default".to_string());
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_option_some_empty() {
        let some_empty_string = Some(String::new());
        let result = some_empty_string.if_empty(|| "default".to_string());
        assert_eq!(result, "default");
    }

    #[test]
    fn test_option_none() {
        let none_string: Option<String> = None;
        let result = none_string.if_empty(|| "default".to_string());
        assert_eq!(result, "default");
    }

    #[test]
    fn test_closure_with_panic() {
        let empty_string = String::new();
        let result = std::panic::catch_unwind(|| {
            empty_string.if_empty(|| panic!("algo vazio"))
        });
        assert!(result.is_err());
    }
}