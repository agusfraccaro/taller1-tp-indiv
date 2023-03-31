mod pieza;

use pieza::Pieza;
use pieza::Color;
use pieza::Tipo;
use pieza::ResultadoCaptura;

use std::fs;
use std::env;

fn leer_tablero(file_path: &str) -> Result<(Pieza,Pieza),String> {
    let file_content = match fs::read_to_string(file_path) {
        Ok(contenido) => contenido,
        Err(_) => {
            return Err(String::from("Error: no se pudo leer el archivo."));
        }
    };

    let mut pieza_blanca: Option<Pieza> = None;
    let mut pieza_negra: Option<Pieza> = None;

    let mut is_pieza_negra = false;
    let mut is_pieza_blanca = false;

    for (i, line) in file_content.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '_' || c == ' ' {
                continue;
            }

            let color : Color = if c.is_lowercase() { Color::Blanco } else { Color::Negro };
            let tipo_pieza = match c.to_ascii_lowercase() {
                'p' => Tipo::Peon,
                't' => Tipo::Torre,
                'c' => Tipo::Caballo,
                'a' => Tipo::Alfil,
                'd' => Tipo::Dama,
                'r' => Tipo::Rey,
                _ => {
                    return Err(String::from("Error: se encontró una pieza inválida en el archivo"));
                }
            };

            match color {
                Color::Blanco => {
                    if is_pieza_blanca {
                        return Err(String::from("Error: Se encontraron más de una pieza blanca en el archivo."));
                    }
                    is_pieza_blanca = true;
                    pieza_blanca = Some(Pieza::new(color, tipo_pieza, i, j));
                }
                Color::Negro => {
                    if is_pieza_negra {
                        return Err(String::from("Error: Se encontraron más de una pieza negra en el archivo"));
                    }
                    is_pieza_negra = true;
                    pieza_negra = Some(Pieza::new(color, tipo_pieza, i, j));
                }
            }
        }    
    }

    if !is_pieza_blanca {
        return Err(String::from("Error: No hay pieza blanca en el archivo"));
    }

    if !is_pieza_negra {
        return Err(String::from("Error: No hay una pieza negra en el archivo"));
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