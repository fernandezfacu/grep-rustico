use crate::{
    error_regex::ErrorRegex, iterador_vec::IteradorVecHaciaAtras,
    repeticion_regex_token::RepeticionRegexToken,
};

/// Obtención de un rango de repetición a partir de un caracter de repetición y
/// un iterador de los caracteres de un patrón de creación de una expresión regular,
/// con índice  en el caracter de repetición.
///
/// Si el caracter de repetición es ?, + ó *, devuelve la repetición que corresponda.
///
/// Si es {, decide si devuelve una repetición o nada.
/// Si no devuelve nada, no modifica el iterador.
/// Si devuelve un rango, avanza en el iterador para que el siguiente caracter sea el que le sigue a }.
///
/// # Errores
///
/// Devuelve error si el contenido de un rango de repetición es inválido. Los casos son:
///
/// * Si se tiene un número y luego uno menor. Por ejemplo: '2', ',' '1' '}' son los cuatro
///   caracteres siguientes en el iterador.
///
/// * Si se tiene dos números corectamente y luego una coma. Ejemplo: '1', ',' '2' ',', '}' son los cinco
///   caracteres siguientes en el iterador.
pub fn obtener_repeticion(
    c: char,
    iter_chars: &mut IteradorVecHaciaAtras<char>,
) -> Result<Option<RepeticionRegexToken>, ErrorRegex> {
    let repeticion = match c {
        '*' => Some(RepeticionRegexToken::new(None, None)),
        '?' => Some(RepeticionRegexToken::new(None, Some(1))),
        '+' => Some(RepeticionRegexToken::new(Some(1), None)),
        '{' => obtener_rango_repeticion(iter_chars)?,
        _ => None,
    };
    Ok(repeticion)
}

/// Función auxiliar para la obtención de rangos de repetición.
///
/// Recibe un iterador de caracteres.
///
/// Si se puede obtener un rango, lo devuelve y adelanta el iterador.
/// Si no, devuelve None y deja el iterador donde estaba.
///
/// Si hay un error en la sintaxis del rango, se devuelve el error de contenido inválido de una
/// repetición en una expresión regular.
fn obtener_rango_repeticion(
    iter_chars: &mut IteradorVecHaciaAtras<char>,
) -> Result<Option<RepeticionRegexToken>, ErrorRegex> {
    let contenido_rango = obtener_contenido_de_rango_repeticion(iter_chars)?;
    if let Some(contenido) = contenido_rango {
        let params_rango: Vec<&str> = contenido.split(',').collect();
        let (min, max) = valores_rango(params_rango);
        if (min > max) & (max.is_some()) {
            return Err(ErrorRegex::ContenidoInvalidoRepeticion);
        }
        Ok(Some(RepeticionRegexToken::new(min, max)))
    } else {
        Ok(None)
    }
}

/// Se obtiene el contenido del rango dado un iterador de caracteres.
///
/// Ejemplo: Si se tiene como caracteres siguientes '1', '}'; el contenido es '1'.
///
/// Si no es un rango, devuelve None y deja el iterador como estaba.
///
/// Si es, devuelve el contenido y deja el iterador avanzado hasta después de }.
///
/// Devuelve error si el contenido del rango es inválido.
fn obtener_contenido_de_rango_repeticion(
    iter_chars: &mut IteradorVecHaciaAtras<char>,
) -> Result<Option<String>, ErrorRegex> {
    let mut pasos_adelante = 0;
    let mut comas = 0;
    let es_rango = es_rango(iter_chars, &mut pasos_adelante, &mut comas)?;
    if !es_rango {
        for _ in 0..pasos_adelante {
            iter_chars.prev();
        }
        Ok(None)
    } else {
        if !(0..2).contains(&comas) | (pasos_adelante == 1) {
            return Err(ErrorRegex::ContenidoInvalidoRepeticion);
        }
        let mut contenido_rango_rev = String::new();
        for _ in 0..pasos_adelante - 1 {
            if let Some(c) = iter_chars.prev() {
                contenido_rango_rev.push(*c);
            }
        }
        for _ in 0..pasos_adelante - 1 {
            iter_chars.next();
        }
        let contenido_rango: String = contenido_rango_rev.chars().rev().collect();
        Ok(Some(contenido_rango))
    }
}

fn es_rango(
    iter_chars: &mut IteradorVecHaciaAtras<char>,
    pasos_adelante: &mut usize,
    comas: &mut usize,
) -> Result<bool, ErrorRegex> {
    for c in iter_chars.by_ref() {
        if *comas > 1 {
            return Err(ErrorRegex::ContenidoInvalidoRepeticion);
        }
        *pasos_adelante += 1;
        if c.is_ascii_digit() {
            continue;
        } else if *c == ',' {
            *comas += 1;
        } else if *c == '}' {
            return Ok(true);
        } else {
            break;
        }
    }
    Ok(false)
}

fn valores_rango(params_rango: Vec<&str>) -> (Option<usize>, Option<usize>) {
    let min: Option<usize>;
    let max: Option<usize>;
    if params_rango.len() == 1 {
        match params_rango[0].parse::<usize>() {
            Ok(n) => {
                min = Some(n);
                max = Some(n);
            }
            Err(_) => {
                min = None;
                max = None;
            }
        }
    } else {
        match params_rango[0].parse::<usize>() {
            Ok(n) => min = Some(n),
            Err(_) => min = None,
        }
        match params_rango[1].parse::<usize>() {
            Ok(m) => max = Some(m),
            Err(_) => max = None,
        }
    }
    (min, max)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn obtengo_repeticion_asterisco() {
        let v = Vec::new();
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);

        let repeticion = obtener_repeticion('*', &mut iter_chars).unwrap().unwrap();

        assert_eq!(RepeticionRegexToken::new(None, None), repeticion);
    }

    #[test]
    fn obtengo_repeticion_signo_pregunta() {
        let v = Vec::new();
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);

        let repeticion = obtener_repeticion('?', &mut iter_chars).unwrap().unwrap();

        assert_eq!(RepeticionRegexToken::new(None, Some(1)), repeticion);
    }

    #[test]
    fn obtengo_repeticion_signo_suma() {
        let v = Vec::new();
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);

        let repeticion = obtener_repeticion('+', &mut iter_chars).unwrap().unwrap();

        assert_eq!(RepeticionRegexToken::new(Some(1), None), repeticion);
    }

    #[test]
    fn obtengo_rango_mismos_extremos() {
        let v = vec!['1', '}'];
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);

        let repeticion = obtener_repeticion('{', &mut iter_chars).unwrap().unwrap();

        assert_eq!(RepeticionRegexToken::new(Some(1), Some(1)), repeticion);
    }

    #[test]
    fn obtengo_rango_distintos_extremos() {
        let v = vec!['1', ',', '5', '}'];
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);

        let repeticion = obtener_repeticion('{', &mut iter_chars).unwrap().unwrap();

        assert_eq!(RepeticionRegexToken::new(Some(1), Some(5)), repeticion);
    }

    #[test]
    fn obtengo_rango_sin_extremos() {
        let v1 = vec!['1', ',', '}'];
        let mut iter_chars1 = IteradorVecHaciaAtras::new(&v1);
        let v2 = vec![',', '5', '}'];
        let mut iter_chars2 = IteradorVecHaciaAtras::new(&v2);

        let repeticion1 = obtener_repeticion('{', &mut iter_chars1).unwrap().unwrap();
        let repeticion2 = obtener_repeticion('{', &mut iter_chars2).unwrap().unwrap();

        assert_eq!(RepeticionRegexToken::new(Some(1), None), repeticion1);
        assert_eq!(RepeticionRegexToken::new(None, Some(5)), repeticion2);
    }

    #[test]
    fn no_obtengo_rango_con_llave_que_no_cierra_o_contenido_no_numerico() {
        let v1 = vec!['1', ',', 'n', '}'];
        let v2 = vec!['n', 'o', ' ', 'c', 'i', 'e', 'r', 'r', 'r', 'a'];
        let mut iter_chars1 = IteradorVecHaciaAtras::new(&v1);
        let mut iter_chars2 = IteradorVecHaciaAtras::new(&v2);

        let repeticion1 = obtener_repeticion('{', &mut iter_chars1).unwrap();
        let repeticion2 = obtener_repeticion('{', &mut iter_chars2).unwrap();

        assert_eq!(repeticion1, None);
        assert_eq!(repeticion2, None);
    }

    #[test]
    fn error_con_rango_invalido() {
        let v1 = vec!['5', ',', '1', '}'];
        let mut iter_chars1 = IteradorVecHaciaAtras::new(&v1);
        let v2 = vec!['1', ',', '2', ',', '3', '}'];
        let mut iter_chars2 = IteradorVecHaciaAtras::new(&v2);

        let repeticion1 = obtener_repeticion('{', &mut iter_chars1);
        let repeticion2 = obtener_repeticion('{', &mut iter_chars2);

        assert_eq!(Err(ErrorRegex::ContenidoInvalidoRepeticion), repeticion1);
        assert_eq!(Err(ErrorRegex::ContenidoInvalidoRepeticion), repeticion2);
    }
}
