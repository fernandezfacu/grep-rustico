/// Representación de la repetición de un token de una expresión regular.
///
/// La repetición de un token está dada por un rango.
///
/// El mínimo (y el máximo) de este rango puede ser numérico o no existir.
/// Si no existe (representado por None), significa que el rango no está limitado inferiormente (o superiormente).
#[derive(Debug, PartialEq)]
pub struct RepeticionRegexToken {
    min: Option<usize>,
    max: Option<usize>,
}

impl RepeticionRegexToken {
    /// Creación de una repetición con un mínimo y un máximo.
    pub fn new(min: Option<usize>, max: Option<usize>) -> Self {
        Self { min, max }
    }

    /// Devuelve el mínimo de la repetición.
    pub fn min(&self) -> Option<usize> {
        self.min
    }

    /// Devuelve el máximo de la repetición.
    pub fn max(&self) -> Option<usize> {
        self.max
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn creacion_de_repeticion_token() {
        let min = None;
        let max = Some(1);
        let rep = RepeticionRegexToken::new(min, max);

        assert_eq!(RepeticionRegexToken { min, max }, rep);
    }

    #[test]
    fn obtener_min_max_repeticion() {
        let min = None;
        let max = Some(1);
        let rep = RepeticionRegexToken::new(min, max);

        assert_eq!(min, rep.min());
        assert_eq!(max, rep.max());
    }
}
