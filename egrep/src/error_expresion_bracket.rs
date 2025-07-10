use std::{error, fmt};

/// Errores posibles en la creación de un token con una expresión bracket como valor.
#[derive(Debug, PartialEq)]
pub enum ErrorExpresionBracket {
    BracketNoMatchea,
    NombreClaseInvalido,
    SintaxisClaseInvalida,
}

impl fmt::Display for ErrorExpresionBracket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorExpresionBracket::BracketNoMatchea => write!(f, "unmatched [, [^ or [:"),
            ErrorExpresionBracket::NombreClaseInvalido => {
                write!(f, "Nombre de clase de caracter inválido")
            }
            ErrorExpresionBracket::SintaxisClaseInvalida => write!(
                f,
                "La sintaxis de la clase de caracteres es [[:space:]], no [:space:]"
            ),
        }
    }
}

impl error::Error for ErrorExpresionBracket {}
