//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.cells(), &expected_universe.cells());
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new(64, 64);
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[1, 2, 3, 3, 3], &[2, 3, 1, 2, 3]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new(64, 64);
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[2, 2, 3, 3, 4], &[1, 3, 2, 3, 2]);
    universe
}