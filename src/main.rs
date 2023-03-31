mod pieza;

use pieza::Pieza;
use pieza::Color;
use pieza::Tipo;
use pieza::ResultadoCaptura;

use std::fs::File;
use std::env;
use std::io::{self, prelude::*};

fn leer_tablero(file_path: &str) -> io::Result<(Pieza, Pieza)> {
    let mut file = match File::open(file_path){
        Ok(file) => file,
        Err(_) => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Error: no se pudo abrir el archivo."));
        }
    };
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut pieza_blanca: Option<Pieza> = None;
    let mut pieza_negra: Option<Pieza> = None;

    let mut is_pieza_negra = false;
    let mut is_pieza_blanca = false;

    let mut i = 1;
    for line in contents.lines() {
        let mut j = 1;
        for c in line.chars() {
            if c == '_' {
                j += 1;
                continue;
            }

            if c == ' ' { continue; }

            let color : Color = if c.is_lowercase() { Color::Blanco } else { Color::Negro };
            let tipo_pieza = match c.to_ascii_lowercase() {
                'p' => Tipo::Peon,
                't' => Tipo::Torre,
                'c' => Tipo::Caballo,
                'a' => Tipo::Alfil,
                'd' => Tipo::Dama,
                'r' => Tipo::Rey,
                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Error: carácter inválido en el archivo."));
                }
            };

            match color {
                Color::Blanco => {
                    if is_pieza_blanca {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Error: hay dos piezas blancas en el archivo."));
                    }
                    is_pieza_blanca = true;
                    pieza_blanca = Some(Pieza::new(color, tipo_pieza, i, j+1));
                }
                Color::Negro => {
                    if is_pieza_negra {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, "Error: hay dos piezas negras en el archivo."));
                    }
                    is_pieza_negra = true;
                    pieza_negra = Some(Pieza::new(color, tipo_pieza, i, j+1));
                }
            }
        }    
        i += 1;
    }

    if !is_pieza_blanca || !is_pieza_negra{
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Error: el archivo debería contener dos piezas."));
    }

    Ok((pieza_negra.unwrap(), pieza_blanca.unwrap()))
}

fn captura(piezas: (pieza::Pieza, pieza::Pieza)) -> ResultadoCaptura {
    let pieza_negra = piezas.0;
    let pieza_blanca = piezas.1;

    if pieza_negra.puede_capturar(&pieza_blanca) {
        if pieza_blanca.puede_capturar(&pieza_negra) {
            ResultadoCaptura::AmbasCaptura
        } else {
            ResultadoCaptura::NegraCaptura
        }
    } else if pieza_blanca.puede_capturar(&pieza_negra) {
        ResultadoCaptura::BlancaCaptura
    } else {
        ResultadoCaptura::NingunaCaptura
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: debe enviar como parámetro el nombre del archivo.");
        println!("USO: cargo run -- <nombre_archivo>.txt");
        return;
    }

    let file_path = &args[1];

    let piezas = match leer_tablero(file_path){
        Ok(piezas) => piezas,
        Err(error) =>{
            println!("{}", error);
            return;
        }
    };

    let resultado_captura = captura(piezas);

    match resultado_captura {
        ResultadoCaptura::AmbasCaptura => {
            println!("E");
        }
        ResultadoCaptura::NingunaCaptura => {
            println!("P");
        }
        ResultadoCaptura::NegraCaptura => {
            println!("N");
        }
        ResultadoCaptura::BlancaCaptura => {
            println!("B");
        }
    }
}