use egrep::egrep::egrep;

#[test]
fn test_1_grep() {
    let lineas_que_matchean =
        egrep("ab.cd", &"tests/data/tests_integracion.txt".to_string()).unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec![
        "abxcd".to_string(),
        "xabxcdx".to_string(),
        "abbcd".to_string(),
    ];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_2_grep() {
    let lineas_que_matchean =
        egrep("ab.*cd", &"tests/data/tests_integracion.txt".to_string()).unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec![
        "abxcd".to_string(),
        "abcd".to_string(),
        "xabxcdx".to_string(),
        "abcd".to_string(),
        "abcccccccccccccd".to_string(),
        "abasdahsdbhasbdhiasdbcddkfdjklfhsdfhj".to_string(),
        "aoksdhjsaddajshabasdahsdbhasbdhiasdbcddkfdjklfhsdfhj".to_string(),
        "abbcd".to_string(),
        "abbbbcd".to_string(),
    ];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_3_grep() {
    let lineas_que_matchean =
        egrep("a[bc]d", &"tests/data/tests_integracion.txt".to_string()).unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec![
        "abd".to_string(),
        "acd".to_string(),
        "xabdx".to_string(),
        "abdeeeeeeeeeeeeeeefc".to_string(),
    ];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_4_grep() {
    let lineas_que_matchean =
        egrep("ab{2,4}cd", &"tests/data/tests_integracion.txt".to_string()).unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec!["abbcd".to_string(), "abbbbcd".to_string()];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_5_grep() {
    let lineas_que_matchean =
        egrep("abc|de+f", &"tests/data/tests_integracion.txt".to_string()).unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec![
        "abcd".to_string(),
        "abcd".to_string(),
        "abcccccccccccccd".to_string(),
        "abcccccccccccccc".to_string(),
        "abc".to_string(),
        "def".to_string(),
        "deeeeeeeeeeeeeeeeeef".to_string(),
        "abdeeeeeeeeeeeeeeefc".to_string(),
        "dfabc".to_string(),
    ];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_6_grep() {
    let lineas_que_matchean = egrep(
        "la [aeiou] es una vocal",
        &"tests/data/tests_integracion.txt".to_string(),
    )
    .unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec![
        "la a es una vocal".to_string(),
        "la u es una vocal".to_string(),
    ];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_7_grep() {
    let lineas_que_matchean = egrep(
        "la [^aeiou] no es una vocal",
        &"tests/data/tests_integracion.txt".to_string(),
    )
    .unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec!["la x no es una vocal".to_string()];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_8_grep() {
    let lineas_que_matchean = egrep(
        "hola [[:alpha:]]+",
        &"tests/data/tests_integracion.txt".to_string(),
    )
    .unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec![
        "hola mama!".to_string(),
        "hola papa!".to_string(),
        "hola mundo".to_string(),
    ];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_9_grep() {
    let lineas_que_matchean = egrep(
        "[[:digit:]] es un numero",
        &"tests/data/tests_integracion.txt".to_string(),
    )
    .unwrap();

    let lineas_que_deben_matchear: Vec<String> =
        vec!["1 es un numero".to_string(), "10 es un numero".to_string()];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_10_grep() {
    let lineas_que_matchean = egrep(
        "el caracter [[:alnum:]] no es un simbolo",
        &"tests/data/tests_integracion.txt".to_string(),
    )
    .unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec![
        "el caracter a no es un simbolo".to_string(),
        "el caracter z no es un simbolo".to_string(),
        "el caracter 8 no es un simbolo".to_string(),
    ];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_11_grep() {
    let lineas_que_matchean = egrep(
        "hola[[:space:]]mundo",
        &"tests/data/tests_integracion.txt".to_string(),
    )
    .unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec!["hola mundo".to_string()];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_12_grep() {
    let lineas_que_matchean = egrep(
        "[[:upper:]]ascal[[:upper:]]ase",
        &"tests/data/tests_integracion.txt".to_string(),
    )
    .unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec!["MascalMase".to_string()];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}

#[test]
fn test_13_grep() {
    let lineas_que_matchean = egrep(
        "es el fin$",
        &"tests/data/tests_integracion.txt".to_string(),
    )
    .unwrap();

    let lineas_que_deben_matchear: Vec<String> = vec!["este es el fin".to_string()];

    assert_eq!(lineas_que_matchean, lineas_que_deben_matchear);
}
