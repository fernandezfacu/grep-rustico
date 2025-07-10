use std::{error, fmt};

/// Errores posibles en la creación de una expresión regular a través de un patrón (no incluye errores de bracket expresiones).
#[derive(Debug, PartialEq)]
pub enum ErrorRegex {
    RepeticionInvalida,
    ContenidoInvalidoRepeticion,
    BarraInvertidaAlFinal,
    ExpresionInvalidaAnclajeInicio,
}

impl fmt::Display for ErrorRegex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorRegex::RepeticionInvalida => write!(
                f,
                "Metacaracter de repetición al principio o luego de otro que no admite repetición"
            ),
            ErrorRegex::ContenidoInvalidoRepeticion => write!(f, "Contenido de \\{{\\}} inválido"),
            ErrorRegex::BarraInvertidaAlFinal => write!(f, "Barra invertida extra al final «\\»"),
            ErrorRegex::ExpresionInvalidaAnclajeInicio => write!(
                f,
                "Metacaracter de anclaje de inicio ^ en medio de una expresión"
            ),
        }
    }
}

impl error::Error for ErrorRegex {}
