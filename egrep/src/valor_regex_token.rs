use crate::clase_caracter::ClaseCaracter;

/// Representación de un valor de un token de una expresión regular.
#[derive(Debug, PartialEq)]
pub enum ValorRegexToken {
    Literal(char),
    Comodin,
    AnclajeInicio,
    AnclajeFinal,
    ExpresionBracket(Vec<ClaseCaracter>),
    ExpresionBracketNegada(Vec<ClaseCaracter>),
}

impl ValorRegexToken {
    /// Matcheo de un valor de un token con otro valor, recibido como string slice.
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
    /// * Un comodín matchea con cualquier caracter.
    ///
    /// * Un token de anclaje no matchea con ningún caracter.
    ///
    /// * Una expresión bracket matchea con un caracter si este matchea con alguna de las
    ///   clases de caracter de la expresión.
    ///
    /// * Una expresión bracket negada matchea con un caracter si este no matchea con ninguna de las
    ///   clases de caracter de la expresión.
    pub fn matchea(&self, valor: &str) -> usize {
        match self {
            Self::Literal(l) => matchear_con_literal(*l, valor),
            Self::Comodin => matchear_con_comodin(valor),
            Self::AnclajeInicio => matchear_con_anclaje(),
            Self::AnclajeFinal => matchear_con_anclaje(),
            Self::ExpresionBracket(clases_caracter) => {
                matchear_con_expresion_bracket(clases_caracter, valor)
            }
            Self::ExpresionBracketNegada(clases_caracter) => {
                matchear_con_expresion_bracket_negada(clases_caracter, valor)
            }
        }
    }
}

fn matchear_con_literal(literal: char, valor: &str) -> usize {
    if valor.starts_with(literal) {
        literal.len_utf8()
    } else {
        0
    }
}

fn matchear_con_comodin(valor: &str) -> usize {
    if let Some(c) = valor.chars().next() {
        c.len_utf8()
    } else {
        0
    }
}

fn matchear_con_anclaje() -> usize {
    0
}

fn matchear_con_expresion_bracket(clases_caracter: &Vec<ClaseCaracter>, valor: &str) -> usize {
    let mut matcheo = 0;
    for clase_caracter in clases_caracter {
        if clase_caracter.matchea(valor) > 0 {
            matcheo = clase_caracter.matchea(valor);
            break;
        }
    }
    matcheo
}

fn matchear_con_expresion_bracket_negada(
    clases_caracter: &Vec<ClaseCaracter>,
    valor: &str,
) -> usize {
    if let Some(c) = valor.chars().next() {
        let mut matcheo = c.len_utf8();
        for clase_caracter in clases_caracter {
            if clase_caracter.matchea(valor) > 0 {
                matcheo = 0;
                break;
            }
        }
        matcheo
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_matchea_con_valor() {
        let valor_que_matchea = "lddf";
        let valor_que_no_matchea = "ddf";
        let literal = 'l';

        let result1 = matchear_con_literal(literal, valor_que_matchea);
        let result2 = matchear_con_literal(literal, valor_que_no_matchea);

        assert_eq!(result1, literal.len_utf8());
        assert_eq!(result2, 0)
    }

    #[test]
    fn comodin_matchea_con_todo() {
        let valor = "ddf";
        let primera_letra_valor = 'd';

        let result = matchear_con_comodin(valor);

        assert_eq!(result, primera_letra_valor.len_utf8());
    }

    #[test]
    fn anclaje_no_matchea_con_nada() {
        let result = matchear_con_anclaje();

        assert_eq!(result, 0);
    }

    #[test]
    fn bracket_expresion_matchea_con_valor() {
        let valor = "#ddf";
        let primer_caracter_valor = '#';
        let clases_caracteres_que_matchean = vec![ClaseCaracter::Literal('#')];
        let clases_caracteres_que_no_matchean =
            vec![ClaseCaracter::Literal('?'), ClaseCaracter::Alfabetico];

        let result1 = matchear_con_expresion_bracket(&clases_caracteres_que_matchean, valor);
        let result2 = matchear_con_expresion_bracket(&clases_caracteres_que_no_matchean, valor);

        assert_eq!(result1, primer_caracter_valor.len_utf8());
        assert_eq!(result2, 0);
    }

    #[test]
    fn bracket_expresion_negada_matchea_con_valor() {
        let valor = "#ddf";
        let primer_caracter_valor = '#';
        let clases_caracteres_que_no_matchean = vec![ClaseCaracter::Literal('#')];
        let clases_caracteres_que_matchean =
            vec![ClaseCaracter::Literal('?'), ClaseCaracter::Alfabetico];

        let result1 = matchear_con_expresion_bracket(&clases_caracteres_que_matchean, valor);
        let result2 = matchear_con_expresion_bracket(&clases_caracteres_que_no_matchean, valor);

        assert_eq!(result1, 0);
        assert_eq!(result2, primer_caracter_valor.len_utf8());
    }
}
