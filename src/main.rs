mod chip8;

use crate::chip8::chip8::create_chip8;


fn main() {
    let mut chip8 = create_chip8();
    chip8.emulate_cycle()
}
