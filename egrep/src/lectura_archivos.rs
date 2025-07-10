use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::error_lectura_archivos::ErrorArchivo;

/// Lectura de archivos. Dado un path a un archivo, devuelve las líneas del mismo, si este existe.
///
/// # Errores
///
/// Si el path al archivo no existe, devuelve un error indicando esto.
///
/// Si una de las lineas no se puede leer, devuelve un error indicando en qué linea del archivo ocurrió.
///  
pub fn leer_lineas_archivo(nombre_archivo: impl AsRef<Path>) -> Result<Vec<String>, ErrorArchivo> {
    let file = File::open(&nombre_archivo)
        .map_err(|_e| ErrorArchivo::Path(nombre_archivo.as_ref().to_path_buf()))?;
    let lector = BufReader::new(file);
    let mut lineas = Vec::new();

    for (i, resultado_lectura) in lector.lines().enumerate() {
        let numero_linea = i + 1;
        let linea = resultado_lectura.map_err(|_e| {
            ErrorArchivo::Lectura(nombre_archivo.as_ref().to_path_buf(), numero_linea)
        })?;
        lineas.push(linea);
    }

    Ok(lineas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_al_leer_archivo_inixistente() {
        let nombre_archivo = "";

        let result = leer_lineas_archivo(nombre_archivo);
        assert_eq!(result, Err(ErrorArchivo::Path(nombre_archivo.into())));
    }

    #[test]
    fn lee_lineas_archivo_existente() {
        let path_archivo = "data/test_lectura_archivo.txt";
        let lineas_archivo: Vec<String> = vec![
            "linea1".to_string(),
            "linea2".to_string(),
            "linea 3".to_string(),
        ];

        let result = leer_lineas_archivo(path_archivo);

        assert_eq!(result, Ok(lineas_archivo));
    }
}
