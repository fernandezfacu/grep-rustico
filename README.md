# grep-rustico
Implementación del comando grep en Rust

## Funcionalidad

Deberá ser invocado solo con la expresión regular y la ruta del archivo a evaluar (semejante al ejemplo brindado para grep). Estos serán pasados como argumentos de línea de comando. El resultado es impreso por terminal.

Se implementa la funcionalidad para la expresiones que contienen caracteres normales y los siguientes metacaracteres:

    Period: .
    Bracket expression: []
    Bracket expression negada: [^]
    Character Classes: [:alnum:], [:alpha:], [:digit:], [:lower:], [:upper:], [:space:], [:punct:],
    Anchoring: ^, $
    Repetition: ?, *, +, {n}, {n,}, {,m}, {n,m}

Además, la implementación permite la concatenación, la alternancia (‘|’), y la precedencia de expresiones regulares.

Esta implementación no admite el uso de metacaracteres de repeticion concatenados o al principio de una expresión. Tampoco admite el uso de range expressions dentro de una bracket expression.

## Cómo compilar y ejecutar

Se debe tener instalado Rust para poder probar esta implementación.

Para compilar:

```bash
$ cargo build
```

Para ejecutar:

```bash
$ cargo run "regular_expression" path/to/file
```

Para ver la documentación:

```bash
$ cargo doc --open
```
