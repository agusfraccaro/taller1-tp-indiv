use tpindividual::funciones::captura;
use tpindividual::funciones::leer_tablero;
use tpindividual::pieza::color::Color;
use tpindividual::pieza::pieza::Pieza;
use tpindividual::pieza::tipo::Tipo;

#[test]
fn tablero_dos_piezas_negras_error() {
    assert!(leer_tablero("tablero_test.txt").is_err());
}

#[test]
fn tablero_dos_piezas_blancas_error() {
    assert!(leer_tablero("tablero_test2.txt").is_err());
}

#[test]
fn tablero_caracter_invalido_error() {
    assert!(leer_tablero("tablero_test3.txt").is_err());
}

#[test]
fn tablero_solo_una_pieza_error() {
    assert!(leer_tablero("tablero_test4.txt").is_err());
}

#[test]
fn tablero_correcto() {
    assert!(leer_tablero("tablero_test5.txt").is_ok());
}

#[test]
fn archivo_no_existe() {
    assert!(leer_tablero("tablero_test139.txt").is_err());
}

#[test]
fn captura_negra() {
    let pieza1 = Pieza::new(Color::Negro, Tipo::Dama, 8, 9);
    let pieza2 = Pieza::new(Color::Blanco, Tipo::Peon, 8, 1);

    assert_eq!('N', captura((pieza1, pieza2)));
}

#[test]
fn captura_blanca() {
    let pieza1 = Pieza::new(Color::Negro, Tipo::Peon, 3, 4);
    let pieza2 = Pieza::new(Color::Blanco, Tipo::Rey, 3, 3);

    assert_eq!('B', captura((pieza1, pieza2)));
}

#[test]
fn ambas_capturan() {
    let pieza1 = Pieza::new(Color::Negro, Tipo::Peon, 1, 1);
    let pieza2 = Pieza::new(Color::Blanco, Tipo::Rey, 2, 2);

    assert_eq!('E', captura((pieza1, pieza2)));
}

#[test]
fn ninguna_captura() {
    let pieza1 = Pieza::new(Color::Negro, Tipo::Caballo, 5, 7);
    let pieza2 = Pieza::new(Color::Blanco, Tipo::Torre, 2, 2);

    assert_eq!('P', captura((pieza1, pieza2)));
}
