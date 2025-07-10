use std::{error, fmt, path::PathBuf};

/// Errores posibles en la lectura de archivos
#[derive(Debug, PartialEq)]
pub enum ErrorArchivo {
    Path(PathBuf),
    Lectura(PathBuf, usize),
}

impl fmt::Display for ErrorArchivo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorArchivo::Path(ref nombre_archivo) => write!(
                f,
                "{}: No existe el archivo o el directorio",
                nombre_archivo.display()
            ),
            ErrorArchivo::Lectura(ref nombre_archivo, ref numero_linea) => write!(
                f,
                "No se puede leer el archivo '{}' en la línea {}",
                nombre_archivo.display(),
                numero_linea
            ),
        }
    }
}

impl error::Error for ErrorArchivo {}
