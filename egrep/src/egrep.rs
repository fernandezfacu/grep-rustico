use crate::lectura_archivos::leer_lineas_archivo;
use crate::regex::Regex;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Impresión por pantalla de los resultados de egrep
pub fn imprimir(resultado: Result<Vec<String>>) {
    match resultado {
        Ok(lineas) => {
            for linea in lineas {
                println!("{}", linea);
            }
        }
        Err(e) => eprintln!("grep: {}", e),
    }
}

/// Dado un patrón y un path a un archivo, devuelve los resultados de egrep como un vector de strings.
pub fn egrep(patron: &str, path_archivo: &String) -> Result<Vec<String>> {
    let lineas_archivo = leer_lineas_archivo(path_archivo)?;
    let regex = Regex::new(patron)?;
    let mut lineas_que_matchean = Vec::new();
    for linea in lineas_archivo {
        if regex.matchea(&linea) {
            lineas_que_matchean.push(linea);
        }
    }
    Ok(lineas_que_matchean)
}
