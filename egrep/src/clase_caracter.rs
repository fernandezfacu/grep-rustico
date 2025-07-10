use crate::error_expresion_bracket::ErrorExpresionBracket;

/// Representación de una clase de caractes de una expresión bracket.
#[derive(Debug, PartialEq)]
pub enum ClaseCaracter {
    Literal(char),
    Alfanumerico,
    Alfabetico,
    Digito,
    Minuscula,
    Mayuscula,
    Espacio,
    Puntuacion,
}

impl ClaseCaracter {
    /// Creación de una clase de caracter a través de un string.
    ///
    /// # Errores
    ///
    /// Si el string no corresponde con el nombre asignado a la creación de una clase, se devuelve
    /// el error correspondienrte.
    pub fn new(nombre_clase: &str) -> Result<Self, ErrorExpresionBracket> {
        let clase = match nombre_clase {
            "alnum" => Self::Alfanumerico,
            "alpha" => Self::Alfabetico,
            "digit" => Self::Digito,
            "lower" => Self::Minuscula,
            "upper" => Self::Mayuscula,
            "space" => Self::Espacio,
            "punct" => Self::Puntuacion,
            _ => return Err(ErrorExpresionBracket::NombreClaseInvalido),
        };
        Ok(clase)
    }

    /// Matcheo de una clase de caracter con un valor, recibido como string slice.
    ///
    /// En todos los casos, se matchea con el primer caracter del valor evaluado.
    /// Si el valor evaluado está vacío, se devuelve 0.
    ///
    /// Si se matchea con el caracter evaluado, se devuelve su largo en utf8.
    /// Caso contrario, se devuelve 0.
    ///
    /// # Matcheos:
    ///
    /// * Un literal matchea con un caracter si son iguales.
    ///
    /// * Las demás clases matchean con un caracter según si
    ///   este pertenece a la clase (según indica el nombre de la misma).
    pub fn matchea(&self, valor: &str) -> usize {
        if let Some(c) = valor.chars().next() {
            match self {
                Self::Alfanumerico => matchear_con_alfanumerico(c),
                Self::Alfabetico => matchear_con_alfabetico(c),
                Self::Digito => matchear_con_digito(c),
                Self::Minuscula => matchear_con_minuscula(c),
                Self::Mayuscula => matchear_con_mayuscula(c),
                Self::Espacio => matchear_con_espacio(c),
                Self::Puntuacion => matchear_con_puntuacion(c),
                Self::Literal(l) => matchear_con_literal(*l, c),
            }
        } else {
            0
        }
    }
}

fn matchear_con_alfanumerico(c: char) -> usize {
    if c.is_ascii_alphanumeric() {
        c.len_utf8()
    } else {
        0
    }
}

fn matchear_con_alfabetico(c: char) -> usize {
    if c.is_ascii_alphabetic() {
        c.len_utf8()
    } else {
        0
    }
}

fn matchear_con_digito(c: char) -> usize {
    if c.is_ascii_digit() {
        c.len_utf8()
    } else {
        0
    }
}

fn matchear_con_minuscula(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c.len_utf8()
    } else {
        0
    }
}

fn matchear_con_mayuscula(c: char) -> usize {
    if c.is_ascii_uppercase() {
        c.len_utf8()
    } else {
        0
    }
}

fn matchear_con_espacio(c: char) -> usize {
    if c.is_ascii_whitespace() {
        c.len_utf8()
    } else {
        0
    }
}

fn matchear_con_puntuacion(c: char) -> usize {
    if c.is_ascii_punctuation() {
        c.len_utf8()
    } else {
        0
    }
}

fn matchear_con_literal(l: char, c: char) -> usize {
    if c == l {
        c.len_utf8()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creacion_de_clases() {
        let alnum = ClaseCaracter::new("alnum");
        let alpha = ClaseCaracter::new("alpha");
        let digit = ClaseCaracter::new("digit");
        let punct = ClaseCaracter::new("punct");
        let space = ClaseCaracter::new("space");
        let lower = ClaseCaracter::new("lower");
        let upper = ClaseCaracter::new("upper");
        let clase_inexistente = ClaseCaracter::new("clase_inexistente");

        assert_eq!(alnum, Ok(ClaseCaracter::Alfanumerico));
        assert_eq!(alpha, Ok(ClaseCaracter::Alfabetico));
        assert_eq!(digit, Ok(ClaseCaracter::Digito));
        assert_eq!(punct, Ok(ClaseCaracter::Puntuacion));
        assert_eq!(space, Ok(ClaseCaracter::Espacio));
        assert_eq!(lower, Ok(ClaseCaracter::Minuscula));
        assert_eq!(upper, Ok(ClaseCaracter::Mayuscula));
        assert_eq!(
            clase_inexistente,
            Err(ErrorExpresionBracket::NombreClaseInvalido)
        );
    }

    #[test]
    fn matcheos_con_clases() {
        let valor = "abcd".chars().next().unwrap();

        assert_eq!(valor.len_utf8(), matchear_con_alfabetico(valor));
        assert_eq!(valor.len_utf8(), matchear_con_alfanumerico(valor));
        assert_eq!(0, matchear_con_digito(valor));
        assert_eq!(0, matchear_con_espacio(valor));
        assert_eq!(valor.len_utf8(), matchear_con_minuscula(valor));
        assert_eq!(0, matchear_con_mayuscula(valor));
        assert_eq!(0, matchear_con_puntuacion(valor));
    }
}
