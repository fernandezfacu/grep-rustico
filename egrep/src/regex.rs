use std::error;

use crate::{
    error_regex::ErrorRegex, expresion::Expresion,
    expresion_bracket::nuevo_token_expresion_bracket, iterador_vec::IteradorVecHaciaAtras,
    rango_repeticion::obtener_repeticion, regex_token::RegexToken,
    repeticion_regex_token::RepeticionRegexToken, valor_regex_token::ValorRegexToken,
};

/// Representación de una expresión regular.
#[derive(Debug)]
pub struct Regex {
    expresiones: Vec<Expresion>,
}

impl Regex {
    /// Creación de una nueva expresión regular recibiendo un patrón.
    ///
    /// # Errores
    ///
    /// * Si se recibe un \\ al final del patrón.
    ///
    /// * Si se recibe un metacaracter de repetición al inicio del patrón o luego de otro del mismo tipo.
    ///
    /// * Si se recibe un contenido inválido dentro de un rango de repetición (ej.: {2,1}, {1,2,_}).
    ///
    /// * Si se abre una bracket expresión y no cierra (ej.: \[ab, \[\[:space:]).
    ///
    /// * Si se intenta crear una clase de caracter con sintaxis invalida (ej.: \[:space:], sin doble corchete).
    ///
    /// * Si el nombre de la clase de caracter es inválido (ej.: \[\[:spac:]]).
    ///
    ///
    pub fn new(patron: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut tokens: Vec<RegexToken> = Vec::new();
        let mut expresiones: Vec<Expresion> = Vec::new();
        let chars = patron.chars().collect();
        let mut iter_chars = IteradorVecHaciaAtras::new(&chars);
        while let Some(c) = iter_chars.next() {
            if Self::es_metacaracter_de_repeticion(*c) {
                let modified =
                    Self::modificar_repeticion_token(tokens.last_mut(), *c, &mut iter_chars)?;
                if modified {
                    continue;
                }
            }
            if *c == '|' {
                Self::guardar_expresion(&mut expresiones, tokens);
                tokens = Vec::new();
                continue;
            }
            let token = Self::nuevo_token(*c, &mut iter_chars)?;
            if let Some(t) = token {
                tokens.push(t);
            }
        }
        Self::guardar_expresion(&mut expresiones, tokens);
        Ok(Regex { expresiones })
    }

    /// Crea un nuevo token según un caracter y los que le siguen (guardados en un iterador).
    ///
    /// Devuelve errores que luego devolverá new().
    fn nuevo_token(
        c: char,
        iter_chars: &mut IteradorVecHaciaAtras<char>,
    ) -> Result<Option<RegexToken>, Box<dyn error::Error>> {
        let token = match c {
            '.' => Some(Self::nuevo_token_comodin()),
            '\\' => Self::siguiente_como_literal(iter_chars)?,

            '^' => Self::nuevo_token_de_anclaje_inicio(iter_chars)?,
            '$' => Some(Self::nuevo_token_de_anclaje_final()),
            '[' => {
                let t = nuevo_token_expresion_bracket(iter_chars)?;
                Some(t)
            }
            _ => Some(Self::nuevo_token_literal(c)),
        };
        Ok(token)
    }

    fn nuevo_token_comodin() -> RegexToken {
        RegexToken::new(
            ValorRegexToken::Comodin,
            RepeticionRegexToken::new(Some(1), Some(1)),
        )
    }

    fn nuevo_token_literal(literal: char) -> RegexToken {
        RegexToken::new(
            ValorRegexToken::Literal(literal),
            RepeticionRegexToken::new(Some(1), Some(1)),
        )
    }

    /// Devuelve un token de anclaje de inicio, dependiendo del caracter que le antecede en el patrón
    /// recibido en new().
    ///
    /// * Si no le antecede nada o un |, devuelve un anclaje válido (sin repetición para que no falle el matcheo).
    /// * Si le antecede un ^, devuelve None (se ignora).
    /// * Si le antecede otra cosa, se devuelve un error de expresión inválida.
    fn nuevo_token_de_anclaje_inicio(
        iter_chars: &mut IteradorVecHaciaAtras<char>,
    ) -> Result<Option<RegexToken>, ErrorRegex> {
        if let Some(c) = iter_chars.prev() {
            iter_chars.next();
            match c {
                '^' => Ok(None),
                '|' => Ok(Some(RegexToken::new(
                    ValorRegexToken::AnclajeInicio,
                    RepeticionRegexToken::new(Some(0), Some(0)),
                ))),
                _ => Err(ErrorRegex::ExpresionInvalidaAnclajeInicio),
            }
        } else {
            Ok(Some(RegexToken::new(
                ValorRegexToken::AnclajeInicio,
                RepeticionRegexToken::new(Some(0), Some(0)),
            )))
        }
    }

    fn nuevo_token_de_anclaje_final() -> RegexToken {
        RegexToken::new(
            ValorRegexToken::AnclajeFinal,
            RepeticionRegexToken::new(Some(0), Some(0)),
        )
    }

    /// Dado un iterador con los chars restantes de un patrón, devuelve
    /// el siguiente caracter como literal, si este existe.
    ///
    /// # Errores
    ///
    /// Si no hay caracter siguiente, significa que se encontró una barra invertida
    /// al final del patrón y se devuelve el error corresponidiente.
    fn siguiente_como_literal(
        iter_chars: &mut IteradorVecHaciaAtras<char>,
    ) -> Result<Option<RegexToken>, ErrorRegex> {
        if let Some(literal) = iter_chars.next() {
            Ok(Some(Self::nuevo_token_literal(*literal)))
        } else {
            Err(ErrorRegex::BarraInvertidaAlFinal)
        }
    }

    fn guardar_expresion(expresiones: &mut Vec<Expresion>, tokens: Vec<RegexToken>) {
        let expresion = Expresion::new(tokens);
        expresiones.push(expresion);
    }

    fn es_metacaracter_de_repeticion(c: char) -> bool {
        (c == '?') | (c == '*') | (c == '+') | (c == '{')
    }

    /// Modifica la repetición de un token. Devuelve true o false, según si se pudo realizar
    /// la modificación.
    ///
    /// Se recibe un iterador con los tokens del patrón para poder obtener un rango de repetición.
    /// Si se tenía una llave de apertura '{' sin sintaxis de repetición, se devuelve false.
    /// En otro caso (que no represente un error), se devuelve true.
    ///
    /// # Errores
    ///
    /// * Errores que pueden darse al querer generar un rango de repetición con sintaxis inválida.
    ///
    /// * Si el token recibido es inexistente, significa que había un rango de repetición al principio
    ///   del patrón.
    ///
    /// * Si no se puede modificar la repetición del token, significa que ya había un rango de repetición
    ///   luego de otro.
    fn modificar_repeticion_token(
        token: Option<&mut RegexToken>,
        c: char,
        iter_chars: &mut IteradorVecHaciaAtras<char>,
    ) -> Result<bool, ErrorRegex> {
        let repeticion = obtener_repeticion(c, iter_chars)?;
        if let Some(rep) = repeticion {
            if let Some(ultimo_token) = token {
                if !ultimo_token.modificar_repeticion(rep) {
                    Err(ErrorRegex::RepeticionInvalida)
                } else {
                    Ok(true)
                }
            } else {
                Err(ErrorRegex::RepeticionInvalida)
            }
        } else {
            Ok(false)
        }
    }

    /// Matcheo de un valor recibido como string slice con una expresión regular.
    ///
    /// Si alguna de las expresiones particulares que la conforman matchea con el valor,
    /// devuelve true. Caso contrario, devuelve false.
    pub fn matchea(&self, valor: &str) -> bool {
        for expresion in &self.expresiones {
            if expresion.matchea(valor) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn error_al_querer_añadir_repeticion() {
        let resultado1 = Regex::new("hol{3}{5}a");
        let resultado2 = Regex::new("?hola");
        assert!(resultado1.is_err());
        assert!(resultado2.is_err());
    }

    #[test]
    fn error_al_enviar_barra_invertida_al_final() {
        let resultado = Regex::new("hola\\");
        assert!(resultado.is_err());
    }

    #[test]
    fn error_anclaje_inicio_en_medio() {
        let resultado = Regex::new("ho^la");
        assert!(resultado.is_err());
    }
}
