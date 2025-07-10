use crate::{
    clase_caracter::ClaseCaracter, error_expresion_bracket::ErrorExpresionBracket,
    iterador_vec::IteradorVecHaciaAtras, regex_token::RegexToken,
    repeticion_regex_token::RepeticionRegexToken, valor_regex_token::ValorRegexToken,
};

/// Creación de un token de expresión bracket dado un iterador de los caracteres de un patrón
/// de creación de una expresión regular con índice en el caracter apertura de bracket.
///
/// Devuelve un token con repetición {1,1} y una expresión bracket como valor.
///
/// # Errores
///
/// Si el contenido del bracket es inválido, se devuelve el error correspondiente:
///
/// * Si se abre una bracket expresión y no cierra (ej.: \[ab, \[\[:space:]).
///
/// * Si se intenta crear una clase de caracter con sintaxis invalida (ej.: \[:space:], sin doble corchete).
///
/// * Si el nombre de la clase de caracter es inválido (ej.: \[\[:spac:]]).
///
/// Los ejemplos son dados como strings, pero debe interpretarse que los caracteres siguientes en el
/// iterador recibido por parámetro son los que le siguen al primer bracket de apertura ([) en cada caso.
pub fn nuevo_token_expresion_bracket(
    iter_chars: &mut IteradorVecHaciaAtras<char>,
) -> Result<RegexToken, ErrorExpresionBracket> {
    let mut clases_caracter: Vec<ClaseCaracter> = Vec::new();
    let mut negada = false;
    let contenido_bracket = obtener_contenido_de_bracket(iter_chars)?;

    if tiene_error_sintaxis(&contenido_bracket) {
        return Err(ErrorExpresionBracket::SintaxisClaseInvalida);
    }

    let chars_contenido: Vec<char> = contenido_bracket.chars().collect();
    let mut iter_contenido_bracket = IteradorVecHaciaAtras::new(&chars_contenido);

    manejar_primer_caracter_del_contenido(
        &mut iter_contenido_bracket,
        &mut negada,
        &mut clases_caracter,
    )?;
    manejar_el_resto_de_contenido(&mut iter_contenido_bracket, &mut clases_caracter)?;

    if negada {
        Ok(RegexToken::new(
            ValorRegexToken::ExpresionBracketNegada(clases_caracter),
            RepeticionRegexToken::new(Some(1), Some(1)),
        ))
    } else {
        Ok(RegexToken::new(
            ValorRegexToken::ExpresionBracket(clases_caracter),
            RepeticionRegexToken::new(Some(1), Some(1)),
        ))
    }
}

/// Dado un iterador de los caracteres de un patrón de creación de una expresión regular
/// con índice en el caracter apertura de bracket, devuelve el contenido de este bracket si es válido.
///
/// Si es inválido (es decir, el bracket no cierra), devuelve el error que corresponde.
fn obtener_contenido_de_bracket(
    iter_chars: &mut IteradorVecHaciaAtras<char>,
) -> Result<String, ErrorExpresionBracket> {
    let mut contenido = String::new();
    let mut contenido_valido = false;
    let mut clase_caracter_abierta = false;
    while let Some(c) = iter_chars.next() {
        if *c == ']' {
            if cerrar_contenido(clase_caracter_abierta, &contenido) {
                contenido_valido = true;
                break;
            } else {
                contenido.push(*c);
                clase_caracter_abierta = false;
                continue;
            }
        } else if *c == '[' {
            abrir_o_no_clase_caracter(&mut clase_caracter_abierta, iter_chars);
            contenido.push(*c);
        } else {
            contenido.push(*c);
        }
    }
    if !contenido_valido {
        Err(ErrorExpresionBracket::BracketNoMatchea)
    } else {
        Ok(contenido)
    }
}

/// Función auxiliar para la obtención del contenido de un bracket
///
/// Dado un booleano y un string slice que representa el contenido parcial
/// de un bracket, devuelve true o false según si se puede cerrar el contenido o no.
///
/// Esta función se llama cuando se lee un ']' dentro del bracket.
///
/// Si no hay una clase de caracter abierta, se cierra el contenido.
///
/// Si hay una clase de caracter abierta, se cierra el contenido solo si el caracter anterior a
/// ']' no es ':'.
fn cerrar_contenido(clase_caracter_abierta: bool, contenido: &str) -> bool {
    if clase_caracter_abierta {
        if let Some(anterior_c) = contenido.chars().last() {
            return anterior_c != ':';
        }
    }
    true
}

/// Función auxiliar para la obtención del contenido de un bracket
///
/// Recibe un booleano que indica si hay una clase de caracter abierta en el contenido
/// (si se encontró "[:") y un iterador de caracteres que es el patrón para crear la expresión regular original.
///
/// Esta función se llama cuando se lee un '[' dentro del bracket.
///
/// Si luego se encuentra un ':', se abre la clase de caracter (se modifica el booleano a true).
/// //No importa si ya se encontraba abierta, eso será un error más adelante
///
/// Si no se encuentra un ':', no se hace nada.
fn abrir_o_no_clase_caracter(
    clase_caracter_abierta: &mut bool,
    iter_chars: &mut IteradorVecHaciaAtras<char>,
) {
    if let Some(siguiente_c) = iter_chars.next() {
        if *siguiente_c == ':' {
            *clase_caracter_abierta = true;
        }
        iter_chars.prev();
    }
}

/// Dado el contenido de una expresión bracket, determina si tiene error de sintaxis
/// y devuelve true o false.
///
/// Error de sintaxis es cuando el contenido es, por ejemplo, ":alnum:" en vez de "[:alnum:]"
fn tiene_error_sintaxis(contenido: &str) -> bool {
    if let Some(primer_c) = contenido.chars().next() {
        if let Some(ultimo_c) = contenido.chars().last() {
            return (primer_c == ':') & (ultimo_c == ':') & !tiene_solo_dos_puntos(contenido);
        }
    }
    false
}

fn tiene_solo_dos_puntos(contenido: &str) -> bool {
    for c in contenido.chars() {
        if c != ':' {
            return false;
        }
    }
    true
}

/// Se maneja el primer caracter del contenido.
///
/// Se recibe como parámetro el iterador con los caracteres del contenido,
/// un booleano que representa si una bracket expresión es negada o no, y
/// un vector que guarda las clases de caracter de la expresión bracket.
///
/// Si el primer caracter es '^', setea en true el booleano.
/// Si el primer caracter es '[', maneja la posibilidad de que sea una clase de caracter
/// (esto puede devolver un error).
///
/// Si es otro caracter o no es una clase de caracter, guarda una clase de caracter con
/// el caracter como literal.
///
/// Si el iterador no tiene contenido, devuelve error (significa que la expresión bracket estaba vacía).
fn manejar_primer_caracter_del_contenido(
    iter_contenido_bracket: &mut IteradorVecHaciaAtras<char>,
    negada: &mut bool,
    clases_caracter: &mut Vec<ClaseCaracter>,
) -> Result<(), ErrorExpresionBracket> {
    if let Some(c) = iter_contenido_bracket.next() {
        match c {
            '^' => *negada = true,
            '[' => manejar_posible_clase_caracter(iter_contenido_bracket, clases_caracter)?,
            _ => clases_caracter.push(ClaseCaracter::Literal(*c)),
        }
    } else {
        return Err(ErrorExpresionBracket::BracketNoMatchea);
    }
    Ok(())
}

/// Se maneja el contenido de una expresión bracket.
///
/// Se recibe como parámetro el iterador con los caracteres del contenido y
/// un vector que guarda las clases de caracter de la expresión bracket.
///
/// Si un primer caracter es '[', maneja la posibilidad de que sea una clase de caracter
/// (esto puede devolver un error).
///
/// Si es otro caracter o no es una clase de caracter, guarda una clase de caracter con
/// el caracter como literal.
fn manejar_el_resto_de_contenido(
    iter_contenido_bracket: &mut IteradorVecHaciaAtras<char>,
    caracteres: &mut Vec<ClaseCaracter>,
) -> Result<(), ErrorExpresionBracket> {
    while let Some(c) = iter_contenido_bracket.next() {
        if *c == '[' {
            manejar_posible_clase_caracter(iter_contenido_bracket, caracteres)?;
        } else {
            caracteres.push(ClaseCaracter::Literal(*c));
        }
    }
    Ok(())
}

/// Maneja la posibilidad de una clase caracter en una expresion bracket.
/// Se recibe como parámetro el iterador con los caracteres del contenido,
/// un vector que guarda las clases de caracter de la expresión bracket y
/// un caracter.
///
/// Es una función auxiliar para la creación de un token con valor bracket expresión,
/// por como se hacen las llamadas el caracter siguiente en el iterador del contenido
/// es uno que le sigue a '['.
///
/// Si el siguiente es ':', significa que se abrió una clase caracter y se busca crearla.
/// Si hay un error en el contenido de la clase de caracter, se devuelve. Si no, se añade al vector
/// esta clase
///
/// Si el siguiente no es ':', simplemente se agrega como clase caracter literal a '['
/// y a este caracter que le seguía.
fn manejar_posible_clase_caracter(
    iter_contenido_bracket: &mut IteradorVecHaciaAtras<char>,
    caracteres: &mut Vec<ClaseCaracter>,
) -> Result<(), ErrorExpresionBracket> {
    if let Some(siguiente_c) = iter_contenido_bracket.next() {
        match siguiente_c {
            ':' => {
                let contenido_clase = obtener_contenido_de_clase_caracter(iter_contenido_bracket)?;
                let clase_caracter = ClaseCaracter::new(&contenido_clase)?;
                caracteres.push(clase_caracter);
            }
            _ => {
                caracteres.push(ClaseCaracter::Literal('['));
                caracteres.push(ClaseCaracter::Literal(*siguiente_c));
            }
        }
    } else {
        caracteres.push(ClaseCaracter::Literal('['));
    }
    Ok(())
}

/// Obtención del contenido de una clase caracter.
///
/// Se recibe como parámetro un iterador con el contenido de una bracket expresión.
///
/// Es una función auxiliar para la creación de un token con valor bracket expresión,
/// por como se hacen las llamadas el caracter siguiente en el iterador del contenido
/// es uno que le sigue a "[:".
///
/// Si no se encuentra un cierre a la clase de caracter (":]"), se devuelve error.
///
/// Si se encuentra el cierre, se devuelve los caracteres desde el primer caracter que devuelve el iterador
/// hasta encontrar ":]" como un String.
fn obtener_contenido_de_clase_caracter(
    iter_contenido_bracket: &mut IteradorVecHaciaAtras<char>,
) -> Result<String, ErrorExpresionBracket> {
    let mut contenido = String::new();
    let mut contenido_valido = false;
    for c in iter_contenido_bracket.by_ref() {
        if *c == ']' {
            if let Some(ultimo_c) = contenido.chars().last() {
                if ultimo_c == ':' {
                    contenido_valido = true;
                }
            }
            break;
        } else {
            contenido.push(*c);
        }
    }

    if !contenido_valido {
        Err(ErrorExpresionBracket::BracketNoMatchea)
    } else {
        //se popea ':' si se cerró el contenido.
        contenido.pop();
        Ok(contenido)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn error_bracket_no_cierra() {
        let v = vec!['5', ',', '1', ' '];
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);
        let resultado = nuevo_token_expresion_bracket(&mut iter_chars);

        assert_eq!(Err(ErrorExpresionBracket::BracketNoMatchea), resultado)
    }

    #[test]
    fn error_bracket_sin_contenido() {
        let v = vec![']'];
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);
        let resultado = nuevo_token_expresion_bracket(&mut iter_chars);

        assert_eq!(Err(ErrorExpresionBracket::BracketNoMatchea), resultado)
    }

    #[test]
    fn error_sintaxis_clase_caracter() {
        let v = vec![':', 's', 'p', 'a', 'c', 'e', ':', ']'];
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);
        let resultado = nuevo_token_expresion_bracket(&mut iter_chars);

        assert_eq!(Err(ErrorExpresionBracket::SintaxisClaseInvalida), resultado)
    }

    #[test]
    fn error_nombre_clase_caracter() {
        let v = vec!['[', ':', 's', 'p', 'a', 'c', ':', ']', ']'];
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);
        let resultado = nuevo_token_expresion_bracket(&mut iter_chars);

        assert_eq!(Err(ErrorExpresionBracket::NombreClaseInvalido), resultado)
    }

    #[test]
    fn obtengo_token_con_clases_caracter() {
        let v = vec![
            'a', 'b', '[', ':', 's', 'p', 'a', 'c', 'e', ':', ']', 'c', ']',
        ];
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);

        let resultado = nuevo_token_expresion_bracket(&mut iter_chars).unwrap();
        let valor = ValorRegexToken::ExpresionBracket(vec![
            ClaseCaracter::Literal('a'),
            ClaseCaracter::Literal('b'),
            ClaseCaracter::Espacio,
            ClaseCaracter::Literal('c'),
        ]);
        let repeticion = RepeticionRegexToken::new(Some(1), Some(1));

        assert_eq!(RegexToken::new(valor, repeticion), resultado);
    }

    #[test]
    fn obtengo_token_con_expresion_negada() {
        let v = vec![
            '^', 'a', 'b', '[', ':', 's', 'p', 'a', 'c', 'e', ':', ']', 'c', ']',
        ];
        let mut iter_chars = IteradorVecHaciaAtras::new(&v);

        let resultado = nuevo_token_expresion_bracket(&mut iter_chars).unwrap();
        let valor = ValorRegexToken::ExpresionBracketNegada(vec![
            ClaseCaracter::Literal('a'),
            ClaseCaracter::Literal('b'),
            ClaseCaracter::Espacio,
            ClaseCaracter::Literal('c'),
        ]);
        let repeticion = RepeticionRegexToken::new(Some(1), Some(1));

        assert_eq!(RegexToken::new(valor, repeticion), resultado);
    }
}
