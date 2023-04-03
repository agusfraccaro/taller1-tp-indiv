use tpindividual::funciones;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: debe enviar como parámetro el nombre del archivo.");
        println!("USO: cargo run -- <nombre_archivo>.txt");
        return;
    }

    let file_path = &args[1];

    let piezas = match funciones::leer_tablero(file_path) {
        Ok(piezas) => piezas,
        Err(error) => {
            println!("{}", error);
            exit(-1);
        }
    };

    println!("{}", funciones::captura(piezas));
}
