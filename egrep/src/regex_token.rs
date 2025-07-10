use crate::{repeticion_regex_token::RepeticionRegexToken, valor_regex_token::ValorRegexToken};

/// Representación de un token de una expresión regular.
///
/// Posee un valor y una rango de repetición.
///
/// El rango de repetición puede ser modificado una única vez.
#[derive(Debug, PartialEq)]
pub struct RegexToken {
    pub valor: ValorRegexToken,
    repeticion: RepeticionRegexToken,
    repeticion_modificable: bool,
}

impl RegexToken {
    /// Creación de un token dado un valor y un rango de repetición.
    ///
    /// * Caso especial: Si se quiere tener un token que no debe ser evaluado al matchear
    ///   con su valor, inicializar con un rango de repetición con mínimo y máximo 0. De
    ///   este modo, se crea un token que posee una repetición nula, no modificable.
    pub fn new(valor: ValorRegexToken, repeticion: RepeticionRegexToken) -> Self {
        let mut modificable = true;
        if (repeticion.min() == Some(0)) & (repeticion.max() == Some(0)) {
            modificable = false;
        }
        Self {
            valor,
            repeticion,
            repeticion_modificable: modificable,
        }
    }

    /// Modifica la repetición de un token, en caso de ser posible.
    ///
    /// Devuelve true o false, según si se pudo realizar la modificación.
    pub fn modificar_repeticion(&mut self, new_repeticion: RepeticionRegexToken) -> bool {
        if !self.repeticion_modificable {
            false
        } else {
            self.repeticion = new_repeticion;
            self.repeticion_modificable = false;
            true
        }
    }

    /// Devuelve el mínimo del rango de repetición.
    pub fn min_repeticiones(&self) -> Option<usize> {
        self.repeticion.min()
    }

    /// Devuelve el máximo del rango de repetición.
    pub fn max_repeticiones(&self) -> Option<usize> {
        self.repeticion.max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creacion_de_token() {
        let valor = ValorRegexToken::Comodin;
        let repeticion = RepeticionRegexToken::new(Some(1), Some(1));

        let token = RegexToken::new(
            ValorRegexToken::Comodin,
            RepeticionRegexToken::new(Some(1), Some(1)),
        );

        assert_eq!(
            RegexToken {
                valor,
                repeticion,
                repeticion_modificable: true
            },
            token
        );
    }

    #[test]
    fn puedo_modificar_repeticion_nuevo_token() {
        let min_inicial = Some(1);
        let max_inicial = Some(1);
        let repeticion = RepeticionRegexToken::new(min_inicial, max_inicial);
        let mut token = RegexToken::new(ValorRegexToken::Comodin, repeticion);

        let min_nuevo = None;
        let max_nuevo = None;
        let nueva_repeticion = RepeticionRegexToken::new(min_nuevo, max_nuevo);
        let modifico = token.modificar_repeticion(nueva_repeticion);

        assert_ne!(min_inicial, token.min_repeticiones());
        assert_ne!(max_inicial, token.max_repeticiones());

        assert_eq!(min_nuevo, token.min_repeticiones());
        assert_eq!(max_nuevo, token.max_repeticiones());

        assert!(modifico);
    }
}
