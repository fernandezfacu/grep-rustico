use egrep::egrep::egrep;
use egrep::egrep::imprimir;
use std::env;

const ARGS_GREP: usize = 3;
const PATRON: usize = 1;
const NOMBRE_ARCHIVO: usize = 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Si no se recibe la cantidad de argumentos correcta, se imprime un mensaje de error.
    if args.len() == ARGS_GREP {
        let patron = &args[PATRON];
        let nombre_archivo = &args[NOMBRE_ARCHIVO];
        imprimir(egrep(patron, nombre_archivo));
    } else {
        eprintln!("No se ingresaron los argumentos necesarios");
    }
}
