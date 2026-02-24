use std::fmt::{self, Debug, Display};

/// Transforma a exibição de Debug em Display.
/// ### Exemplo
/// ```
/// use common_crate::fmt::DebugToDisplay;
/// let vec = vec![1, 2, 3];
/// let texto = "texto";
/// println!("{}", DebugToDisplay(&vec)); // [1, 2, 3]
/// println!("{}", DebugToDisplay(&texto)); // "texto"
/// println!("{:?} {}", vec, texto); // [1, 2, 3] texto
/// ```
pub struct DebugToDisplay<'a, T>(pub &'a T);

impl<'a, T: Debug> Display for DebugToDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.0, f)
    }
}

pub trait DebugAsDisplay {
	/// Transforma a exibição de Debug em Display.
	/// ### Exemplo
	/// ```
	/// use common_crate::fmt::DebugAsDisplay;
	/// let vec = vec![1, 2, 3];
	/// let texto = "texto";
	/// println!("{}", vec.debug_as_display()); // [1, 2, 3]
	/// println!("{}", texto.debug_as_display()); // "texto"
	/// println!("{:?} {}", vec, texto); // [1, 2, 3] texto
	/// ```
    fn debug_as_display(&self) -> DebugToDisplay<'_, Self> where Self: Sized;
}

impl<T: Debug> DebugAsDisplay for T {
    fn debug_as_display(&self) -> DebugToDisplay<'_, Self> {
        DebugToDisplay(self)
    }
}