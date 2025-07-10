/// Iterador de un vector, que admite la iteración hacia atrás y adelante de sus elementos.
#[derive(Debug, PartialEq)]
pub struct IteradorVecHaciaAtras<'a, T> {
    indice_actual: Option<usize>,
    vector: &'a Vec<T>,
}

impl<'a, T> IteradorVecHaciaAtras<'a, T> {
    /// Creación del iterador.
    ///
    /// Recibe una referencia a un vector de elementos de cualquier tipo.
    pub fn new(vector: &'a Vec<T>) -> IteradorVecHaciaAtras<'a, T> {
        IteradorVecHaciaAtras {
            indice_actual: None,
            vector,
        }
    }

    pub fn prev(&mut self) -> Option<&'a T> {
        let nuevo_indice = match self.indice_actual {
            Some(0) | None => return None,
            Some(i) => i - 1,
        };

        self.indice_actual = Some(nuevo_indice);
        self.vector.get(nuevo_indice)
    }
}

impl<'a, T> Iterator for IteradorVecHaciaAtras<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let nuevo_indice = match self.indice_actual {
            Some(i) => i + 1,
            None => 0,
        };

        self.indice_actual = Some(nuevo_indice);
        self.vector.get(nuevo_indice)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn itera_hacia_adelante_y_atras() {
        let v = vec![1, 2, 3, 4, 5];

        let mut iter = IteradorVecHaciaAtras::new(&v);

        assert_eq!(None, iter.prev());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&4), iter.next());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(Some(&4), iter.prev());
        assert_eq!(Some(&5), iter.next());
        assert_eq!(None, iter.next());
    }
}
