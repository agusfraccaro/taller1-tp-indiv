use crate::pieza;

use pieza::color::Color;
use pieza::tipo::Tipo;

pub struct Pieza {
    color: Color,
    tipo: Tipo,
    fila: usize,
    col: usize,
}

impl Pieza {
    pub fn new(color: Color, tipo: Tipo, fila: usize, col: usize) -> Self {
        Pieza {
            color,
            tipo,
            fila,
            col,
        }
    }

    /// La función recibe una pieza y dependiendo su tipo determina si la pieza sobre la que se llamó
    /// la función puede capturar o no a esa otra pieza. Devuelve true en caso de que se pueda, false si no.
    ///
    /// ejemplo: let pieza : Pieza = Pieza::new(Color::Blanco, Tipo::Rey, 1, 2);
    ///          let otra_pieza : Pieza = Pieza::new(color, tipo_pieza, 3, 4);
    ///          pieza.puede:capturar(otra_pieza);
    pub fn puede_capturar(&self, otra_pieza: &Self) -> bool {
        if self.color == otra_pieza.color {
            return false;
        }

        match self.tipo {
            Tipo::Peon => self.peon_puede_capturar(otra_pieza),
            Tipo::Torre => self.puede_capturar_lineal(otra_pieza),
            Tipo::Caballo => self.caballo_puede_capturar(otra_pieza),
            Tipo::Alfil => self.puede_capturar_diagonal(otra_pieza),
            Tipo::Dama => {
                self.puede_capturar_diagonal(otra_pieza) || self.puede_capturar_lineal(otra_pieza)
            }
            Tipo::Rey => self.rey_puede_capturar(otra_pieza),
        }
    }

    /// Calcula la diferencia de las columnas y filas de ambas piezas y devuelve true si las piezas están a un casillero de distancia
    /// en forma diagonal, que es la forma en la que un peón captura. Devuelve false en caso contrario.
    fn peon_puede_capturar(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = self.fila as i32 - otra_pieza.fila as i32;
        let col_dif = (self.col as i32 - otra_pieza.col as i32).abs();

        if (self.color == Color::Blanco && fila_dif == 1 && col_dif == 1)
            || (self.color == Color::Negro && fila_dif == -1 && col_dif == 1)
        {
            return true;
        }

        false
    }

    /// Calcula la diferencia de las columnas y filas de las piezas y devuelve true si las piezas
    /// están en la misma fila o la misma columna, false en caso contrario.
    fn puede_capturar_lineal(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = self.fila as i32 - otra_pieza.fila as i32;
        let col_dif = self.col as i32 - otra_pieza.col as i32;

        if fila_dif == 0 || col_dif == 0 {
            return true;
        }

        false
    }

    /// Calcula la diferencia de las columnas y filas de las piezas, devuelve true si las posiciones se encuentran
    /// en forma de L, que es la forma en la que captura un caballo. Devuelve false en caso contrario.
    fn caballo_puede_capturar(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = (self.fila as i32 - otra_pieza.fila as i32).abs();
        let col_dif = (self.col as i32 - otra_pieza.col as i32).abs();

        if (fila_dif == 2 && col_dif == 1) || (fila_dif == 1 && col_dif == 2) {
            return true;
        }

        false
    }

    /// Calcula la diferencia de las filas y columnas de las piezas y devuelve true si se encuentran en
    /// diagonal, false en caso contrario.
    fn puede_capturar_diagonal(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = (self.fila as i32 - otra_pieza.fila as i32).abs();
        let col_dif = (self.col as i32 - otra_pieza.col as i32).abs();

        if fila_dif == col_dif && fila_dif != 0 && col_dif != 0 {
            return true;
        }

        false
    }

    /// Calcula la diferencia de las filas y columnas de las piezas y devuelve true si se encuentran a una posición
    /// de distancia en cualquier dirección, que es la forma en la que captura un rey. Devuelve false en caso contrario.
    fn rey_puede_capturar(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = self.fila as i32 - otra_pieza.fila as i32;
        let col_dif = self.col as i32 - otra_pieza.col as i32;

        if (fila_dif.abs() == 1 && col_dif == 0)
            || (fila_dif == 0 && col_dif.abs() == 1)
            || (fila_dif.abs() == 1 && col_dif.abs() == 1)
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieza::color::Color;
    use crate::pieza::tipo::Tipo;

    #[test]
    fn rey_puede_capturar() {
        let pieza1 = Pieza::new(Color::Blanco, Tipo::Rey, 2, 2);
        let pieza2 = Pieza::new(Color::Negro, Tipo::Peon, 2, 3);
        assert!(pieza1.puede_capturar(&pieza2));
    }

    #[test]
    fn dama_puede_capturar() {
        let pieza1 = Pieza::new(Color::Blanco, Tipo::Dama, 2, 2);
        let pieza2 = Pieza::new(Color::Negro, Tipo::Alfil, 8, 2);
        assert!(pieza1.puede_capturar(&pieza2));
    }

    #[test]
    fn alfil_puede_capturar() {
        let pieza1 = Pieza::new(Color::Blanco, Tipo::Alfil, 2, 2);
        let pieza2 = Pieza::new(Color::Negro, Tipo::Peon, 3, 3);
        assert!(pieza1.puede_capturar(&pieza2));
    }

    #[test]
    fn igual_color_no_captura() {
        let pieza1 = Pieza::new(Color::Blanco, Tipo::Rey, 2, 2);
        let pieza2 = Pieza::new(Color::Blanco, Tipo::Peon, 2, 3);
        assert_eq!(false, pieza1.puede_capturar(&pieza2));
    }

    #[test]
    fn caballo_puede_capturar() {
        let pieza1 = Pieza::new(Color::Blanco, Tipo::Caballo, 2, 2);
        let pieza2 = Pieza::new(Color::Negro, Tipo::Peon, 3, 4);
        assert!(pieza1.puede_capturar(&pieza2));
    }

    #[test]
    fn torre_puede_capturar() {
        let pieza1 = Pieza::new(Color::Blanco, Tipo::Torre, 2, 2);
        let pieza2 = Pieza::new(Color::Negro, Tipo::Peon, 2, 8);
        assert!(pieza1.puede_capturar(&pieza2));
    }

    #[test]
    fn peon_puede_capturar() {
        let pieza1 = Pieza::new(Color::Blanco, Tipo::Peon, 7, 2);
        let pieza2 = Pieza::new(Color::Negro, Tipo::Dama, 6, 1);
        assert!(pieza1.puede_capturar(&pieza2));
    }
}
