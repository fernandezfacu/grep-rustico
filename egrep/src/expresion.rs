use crate::{
    iterador_vec::IteradorVecHaciaAtras, regex_token::RegexToken,
    valor_regex_token::ValorRegexToken,
};

/// Representación de una expresión particular (sólo contiene tokens).
#[derive(Debug)]
pub struct Expresion {
    tokens: Vec<RegexToken>,
}

impl Expresion {
    /// Creación de expresión a partir de un vector de tokens válidos.
    pub fn new(tokens: Vec<RegexToken>) -> Self {
        Self { tokens }
    }

    /// Matcheo con de un valor recibido como string slice con una expresión.
    /// Devuelve true si matchea, false en caso contrario.
    ///
    /// Matchea desde el inicio del valor si el primer token es de anclaje,
    /// sino desde cada caracter del valor.
    ///
    /// Si self es una expresión vacía, devuelve true.
    pub fn matchea(&self, valor: &str) -> bool {
        if let Some(token) = self.tokens.first() {
            if token.valor == ValorRegexToken::AnclajeInicio {
                self.matchea_desde_inicio(valor)
            } else {
                self.matchea_desde_cualquier_lugar(valor)
            }
        } else {
            true
        }
    }

    /// Matcheo desde cualquier caracter de un valor.
    fn matchea_desde_cualquier_lugar(&self, valor: &str) -> bool {
        let indices = valor.char_indices();
        for i in indices {
            if self.matchea_desde_inicio(&valor[i.0..]) {
                return true;
            }
        }
        false
    }

    /// Matcheo desde el inicio de un valor.
    fn matchea_desde_inicio(&self, valor: &str) -> bool {
        let mut indice = 0;
        let mut iter_tokens = IteradorVecHaciaAtras::new(&self.tokens);
        // Pila con tuplas que indican el tamaño de match en utf8 de un token con un valor y
        // si se puede hacer backtrack sobre ese token.
        let mut pila: Vec<(usize, bool)> = Vec::new();

        // recorre los tokens de la expresión con un iterador.
        'tokens: while let Some(token) = iter_tokens.next() {
            // Si el token es de anclaje final, devuelve 1 o 0 según si se llegó al final del valor que se evalúa.
            if token.valor == ValorRegexToken::AnclajeFinal {
                return valor.len() == indice;
            }
            // Si el token tiene un límite inferior de repeticiones, se chequea si matchea n veces con el valor.
            // Si matchea n veces, se sigue con el siguiente token. Si no, devuelve 0.
            if let Some(n) = token.min_repeticiones() {
                let (tamaño_match, continuar) = Self::matchear_n_veces(
                    n,
                    token,
                    valor,
                    &mut indice,
                    &mut iter_tokens,
                    &mut pila,
                );
                if continuar {
                    continue 'tokens;
                }
                if (tamaño_match == 0) & (n != 0) {
                    return false;
                }
            }
            // Si el token tiene un límite superior de repeticiones, se avanza sobre el valor
            // hasta m veces, según si se macthea con el token. Si no, se puede avanzar sobre
            //  el valor hasta el final.
            if let Some(m) = token.max_repeticiones() {
                Self::matchear_entre_n_y_m_veces(m, token, valor, &mut indice, &mut pila);
            } else {
                Self::matchear_hasta_el_final(token, valor, &mut indice, &mut pila);
            }
        }
        true
    }

    /// Función auxiliar para el matcheo de una expresion con un valor desde su inicio.
    ///
    /// Verifica si matchea n veces un token con un valor desde determinado índice.
    /// Devuelve una tupla con el tamaño del matcheo en utf8 (o 0 si no hay match) en
    /// la primera posición y un booleano que indica si se debe continuar en el loop desde el que
    /// la función fue llamada.
    fn matchear_n_veces(
        n: usize,
        token: &RegexToken,
        valor: &str,
        indice: &mut usize,
        tokens: &mut IteradorVecHaciaAtras<RegexToken>,
        pila: &mut Vec<(usize, bool)>,
    ) -> (usize, bool) {
        let mut tamaño_match = 0;
        let mut continuar = false;
        for _ in 0..n {
            let s = token.valor.matchea(&valor[*indice..]);
            if s == 0 {
                // Si no matchea, backtrackeo lo que me indica backtrack mas
                // hasta donde matchee en el for. Si se backtrackeo, indico que
                // se debe continuar en el loop donde se llamo a la función y salgo del for.
                if let Some(size) = Self::backtrack(pila, tokens) {
                    *indice -= size + tamaño_match;
                    continuar = true;
                }
                tamaño_match = 0;
                break;
            } else {
                tamaño_match += s;
                *indice += s;
            }
        }
        if tamaño_match != 0 {
            // Guardo en la pila de backtrackeables el tamaño por el que se matcheo
            // e indico que no se puede backtrackear (matcheo n veces, no se vuelve atrás).
            let backtrackeable = false;
            pila.push((tamaño_match, backtrackeable));
        }
        (tamaño_match, continuar)
    }

    /// Función para hacer backtrack sobre un iterador de tokens.
    ///
    /// Dada una pila que guarda tamaños de matcheos y si es posible backtrackear
    /// sobre un token que matcheó, backtrackea sobre el iterador de tokens todo lo que se pueda,
    /// devolviendo el tamaño en usize por el que se backtrackeo (o None en caso de no haberlo hecho).
    fn backtrack(
        evaluated: &mut Vec<(usize, bool)>,
        tokens: &mut IteradorVecHaciaAtras<RegexToken>,
    ) -> Option<usize> {
        let mut back_size = 0;
        tokens.prev();

        while let Some(e) = evaluated.pop() {
            let tamaño_match = e.0;
            let backtrackeable = e.1;
            back_size += tamaño_match;
            if backtrackeable {
                return Some(back_size);
            } else {
                tokens.prev();
            }
        }
        None
    }

    /// Función auxiliar para el matcheo de una expresion con un valor desde su inicio.
    ///
    /// Verifica si matchea entre n y m veces un token con un valor desde determinado índice.
    fn matchear_entre_n_y_m_veces(
        m: usize,
        token: &RegexToken,
        valor: &str,
        indice: &mut usize,
        pila: &mut Vec<(usize, bool)>,
    ) {
        let mut tamaño_match = token.valor.matchea(&valor[*indice..]);
        let mut valores_evaluados = 0;
        // Si el token tenía un mínimo de repeticiones y se llegó al llamado de esta función,
        // significa que el token ya matcheó n veces con el valor.
        if let Some(n) = token.min_repeticiones() {
            valores_evaluados = n;
        }
        while (tamaño_match != 0) & (valores_evaluados < m) {
            *indice += tamaño_match;
            // Guardo en la pila de backtrackeables el tamaño por el que se matcheo
            // e indico que se puede backtrackear (ya matcheo n veces, se puede volver atrás).
            let backtrackeable = true;
            pila.push((tamaño_match, backtrackeable));
            tamaño_match = token.valor.matchea(&valor[*indice..]);
            valores_evaluados += 1;
        }
    }

    /// Función auxiliar para el matcheo de una expresion con un valor desde su inicio.
    ///
    /// Verifica si matchea entre un token con un valor desde determinado índice, hasta el final de este.
    fn matchear_hasta_el_final(
        token: &RegexToken,
        valor: &str,
        indice: &mut usize,
        pila: &mut Vec<(usize, bool)>,
    ) {
        let mut tamaño_match = token.valor.matchea(&valor[*indice..]);
        while tamaño_match != 0 {
            *indice += tamaño_match;
            // Guardo en la pila de backtrackeables el tamaño por el que se matcheo
            // e indico que se puede backtrackear (ya matcheo n veces o no hay n, se puede volver atrás).
            let backtrackeable = true;
            pila.push((tamaño_match, backtrackeable));
            tamaño_match = token.valor.matchea(&valor[*indice..]);
        }
    }
}
