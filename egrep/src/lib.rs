//! Una implementación rústica de egrep.
//!
//! # Funcionalidad
//!
//! egrep deberá ser invocado solo con la expresión regular y la ruta del archivo a evaluar
//! (semejante al ejemplo brindado para grep). Estos serán pasados como argumentos de línea de comando. El resultado es
//! impreso por terminal.
//!
//! Se implementa la funcionalidad para la expresiones que contienen caracteres normales y
//! los siguientes metacaracteres:
//!
//! * Period: .
//! * Bracket expression: \[]
//! * Bracket expression negada: \[^]
//! * Character Classes: \[:alnum:], \[:alpha:], \[:digit:], \[:lower:], \[:upper:], \[:space:], \[:punct:],
//! * Anchoring: ^, $
//! * Repetition: ?, *, +, {n}, {n,}, {,m}, {n,m}
//!
//! Además, la implementación permite la concatenación, la alternancia ('|'), y la precedencia de expresiones regulares.
//!
//! Esta implementación no admite el uso de metacaracteres de repeticion concatenados o al principio de una expresión.
//! Tampoco admite el uso de range expressions dentro de una bracket expression.
//!
//! # Herramientas
//!
//! * [Lectura de archivos de texto][lectura_archivos] y la obtención de las líneas en dicho archivo.
//! * Creación de [expresiones regulares][regex] a través de una cadena de caracteres.
//!   Estas expresiones regulares pueden recibir una cadena de caracteres y reponder si coinciden o no con si misma.
//!   Cada expresión regular puede ser una [expresión única][expresion] o una alternación de estas.
//! * Creación de [tokens][regex_token] particulares que forman parte de una expresión regular.
//!   Para ello, puede crearse el [valor][valor_regex_token] de un token, que puede responder si coincide o no con el primer
//!   caracter de esta; y la [repetición][repeticion_regex_token] del mismo, que puede modificarse una única vez.
//!
//! # grep y expresiones regulares
//!
//! grep(1) (Globally Search For Regular Expression and Print out) es una herramienta de la línea de
//! comando de los sistemas Linux y Unix para la búsqueda de líneas que coincidan con un patrón específico
//! en un archivo o grupo de archivos.
//!
//! La sintaxis básica de grep requiere dos parámetros fundamentales:
//! una expresión regular y la ruta hacia el archivo (relativa o absoluta).
//! ```toml
//! grep "regular_expression" path/to/file
//! ```
//!
//! ### Expresiones Regulares
//!
//! Una expresión regular (o regex) es una cadena de caracteres basadas en reglas sintácticas
//! que permiten describir secuencias de caracteres. Las expresiones regulares se construyen
//! análogamente a las expresiones aritméticas, mediante el uso de varios operadores para
//! combinar expresiones más pequeñas.
//!
//! Una expresión regular puede estar compuesta, o bien solo por caracteres normales,
//! o bien por una combinación de caracteres normales y metacaracteres.
//! Los metacaracteres describen ciertas construcciones o disposiciones de caracteres.

/// Funcionalidad de egrep. Recepción de parámetros e impresión por pantalla de los resultados.
pub mod egrep;

/// Lectura de líneas de archivos de texto.
pub mod lectura_archivos;

/// Errores en lecturas de archivo.
pub mod error_lectura_archivos;

/// Expresiones regulares. Creación a través de un patrón y matcheos con texto.
pub mod regex;

/// Expresiones regulares, sin alternancia de expresiones ('|').
pub mod expresion;

/// Errores en la creación de expresiones regulares (provenientes del patrón de caracteres recibido, sin considerar errores de bracket expresiones).
pub mod error_regex;

/// Creación de bracket expresión a través de un iterador de caracteres.
mod expresion_bracket;

/// Errores en la creación de bracket expresiones (provenientes del patrón de caracteres recibido).
pub mod error_expresion_bracket;

/// Tokens de una expresión regular.
pub mod regex_token;

/// Valor de un token. Literales, comodines, bracket expresiones y matcheos con estos tipos.
pub mod valor_regex_token;

/// Clases de caracter de una bracket expresión. Matcheos con estas.
pub mod clase_caracter;

/// Repeticiones de un token de una expresión regular.
pub mod repeticion_regex_token;

/// Creación de una repetición de un token a través de un iterador de caracteres.
mod rango_repeticion;

/// Iterador para un vector que añade la funcionalidad de iterar hacia atrás.
mod iterador_vec;
