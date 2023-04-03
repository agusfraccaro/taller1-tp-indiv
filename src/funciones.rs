use crate::pieza;

use pieza::color::Color;
use pieza::pieza::Pieza;
use pieza::tipo::Tipo;

use std::fs::File;
use std::io::{self, prelude::*};

/// Recibe el path del archivo a leer, devuelve error si el archivo contiene dos piezas del mismo color, o menos de dos piezas
/// o algún caracter inválido, en caso contrario devuelve una tupla con las dos piezas cargadas con sus respectivos
/// datos (color, tipo, fila y columna).
pub fn leer_tablero(file_path: &str) -> io::Result<(Pieza, Pieza)> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Error: no se pudo abrir el archivo.",
            ));
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut is_pieza_negra = false;
    let mut is_pieza_blanca = false;

    let mut piezas = Vec::new();

    let mut i = 1;
    for line in contents.lines() {
        let mut j = 1;
        for c in line.chars() {
            if c == '_' {
                j += 1;
                continue;
            }

            if c == ' ' {
                continue;
            }

            let color: Color = if c.is_lowercase() {
                Color::Blanco
            } else {
                Color::Negro
            };
            let tipo_pieza = match c.to_ascii_lowercase() {
                'p' => Tipo::Peon,
                't' => Tipo::Torre,
                'c' => Tipo::Caballo,
                'a' => Tipo::Alfil,
                'd' => Tipo::Dama,
                'r' => Tipo::Rey,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Error: caracter inválido '{}' en fila {}", c, i),
                    ));
                }
            };

            match color {
                Color::Blanco => {
                    if is_pieza_blanca {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Error: hay dos piezas blancas en el archivo.",
                        ));
                    }
                    is_pieza_blanca = true;
                    piezas.push(Pieza::new(color, tipo_pieza, i, j + 1));
                }
                Color::Negro => {
                    if is_pieza_negra {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Error: hay dos piezas negras en el archivo.",
                        ));
                    }
                    is_pieza_negra = true;
                    piezas.push(Pieza::new(color, tipo_pieza, i, j + 1));
                }
            }
        }
        i += 1;
    }

    if !is_pieza_blanca || !is_pieza_negra {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Error: el archivo debería contener dos piezas.",
        ));
    }

    Ok((piezas.remove(0), piezas.remove(0)))
}

/// Devuelve la letra correspondiente dependiendo si ambas piezas se pueden capturar mutuamente,
/// ninguna se puede capturar, o capturan solo blancas o solo negras.
pub fn captura(piezas: (Pieza, Pieza)) -> char {
    let pieza_negra = piezas.0;
    let pieza_blanca = piezas.1;

    if pieza_negra.puede_capturar(&pieza_blanca) {
        if pieza_blanca.puede_capturar(&pieza_negra) {
            return 'E';
        } else {
            return 'N';
        }
    } else if pieza_blanca.puede_capturar(&pieza_negra) {
        return 'B';
    } else {
        return 'P';
    }
}
