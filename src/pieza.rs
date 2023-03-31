#[derive(PartialEq)]
pub enum Color {
    Blanco,
    Negro,
}

pub struct Pieza {
    color: Color,
    tipo: Tipo,
    fila: usize,
    col: usize,
}

pub enum ResultadoCaptura {
    NegraCaptura,
    BlancaCaptura,
    AmbasCaptura,
    NingunaCaptura,
}

pub enum Tipo {
    Peon,
    Torre,
    Caballo,
    Alfil,
    Dama,
    Rey,
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

    pub fn puede_capturar(&self, otra_pieza: &Self) -> bool {
        if self.color == otra_pieza.color {
            return false;
        }

        match self.tipo {
            Tipo::Peon => self.peon_puede_capturar(otra_pieza),
            Tipo::Torre => self.puede_capturar_lineal(otra_pieza),
            Tipo::Caballo => self.caballo_puede_capturar(otra_pieza),
            Tipo::Alfil => self.puede_capturar_diagonal(otra_pieza),
            Tipo::Dama => self.puede_capturar_diagonal(otra_pieza) || self.puede_capturar_lineal(otra_pieza),
            Tipo::Rey => self.rey_puede_capturar(otra_pieza),
        }
    }

    fn peon_puede_capturar(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = self.fila as i32 - otra_pieza.fila as i32;
        let col_dif = (self.col as i32 - otra_pieza.col as i32).abs();

        if self.color == Color::Blanco && fila_dif == 1 && col_dif == 1 {
            return true;
        } else if self.color == Color::Negro && fila_dif == -1 && col_dif == 1 {
            return true;
        } 

        false
    }

    fn puede_capturar_lineal(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = self.fila as i32 - otra_pieza.fila as i32;
        let col_dif = self.col as i32 - otra_pieza.col as i32;

        if fila_dif == 0 || col_dif == 0 {
            return true;
        }

        false
    }

    fn caballo_puede_capturar(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = (self.fila as i32 - otra_pieza.fila as i32).abs();
        let col_dif = (self.col as i32 - otra_pieza.col as i32).abs();

        if (fila_dif == 2 && col_dif == 1) || (fila_dif == 1 && col_dif == 2){
            return true;
        }

        false
    }

    fn puede_capturar_diagonal(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = (self.fila as i32 - otra_pieza.fila as i32).abs();
        let col_dif = (self.col as i32 - otra_pieza.col as i32).abs();

        if fila_dif == col_dif && fila_dif != 0 && col_dif != 0 {
            return true;
        }

        false
    }

    fn rey_puede_capturar(&self, otra_pieza: &Pieza) -> bool {
        let fila_dif = self.fila as i32 - otra_pieza.fila as i32;
        let col_dif = self.col as i32 - otra_pieza.col as i32;

        if (fila_dif.abs() == 1 && col_dif == 0) || (fila_dif == 0 && col_dif.abs() == 1) || (fila_dif.abs() == 1 && col_dif.abs() == 1) {
            return true;
        }

        false
    }
}
