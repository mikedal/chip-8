const MEM_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;
const STACK_SIZE: usize = 16;
const KEY_COUNT: usize = 16;
const FONT_SIZE: usize = 80;

struct Chip8 {
    memory: [u8; MEM_SIZE],
    // general purpose registers
    V: [u8; REGISTER_COUNT],
    // index register
    I: u16,
    pc: usize,
    // monochrome, so use bool
    gfx: [bool; DISPLAY_HEIGHT * DISPLAY_WIDTH],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; STACK_SIZE],
    sp: u8,
    keys: [bool; KEY_COUNT],
    opcode: Opcode,
}

impl Chip8 {
    fn init_font(&mut self) {
        // could we do this without allocating a new array? probably
        let font: [u8; FONT_SIZE] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for i in 0..FONT_SIZE {
            self.memory[i] = font[i];
        }
    }

    // load 2 bytes starting at pc
    fn fetch (&self) -> u16 {
        (self.memory[self.pc] as u16).rotate_left(8) | self.memory[self.pc + 1] as u16
    }

    fn emulate_cycle(&mut self){
        let raw_opcode = self.fetch();
        self.opcode = decode(raw_opcode)
    }
}

fn create_chip8() -> Chip8 {
    let mut instance = Chip8 {
        memory: [0; MEM_SIZE],
        V: [0; REGISTER_COUNT],
        I: 0,
        pc: 0x200,
        gfx: [false; DISPLAY_HEIGHT * DISPLAY_WIDTH],
        delay_timer: 0,
        sound_timer: 0,
        stack: [0; STACK_SIZE],
        sp: 0,
        keys: [false; KEY_COUNT],
        opcode: Opcode::OP_0000,
    };
    instance.init_font();
    instance
}

enum Opcode {
    OP_0000,
    OP_00E0,
    OP_00EE,
    OP_1MMM (u32),
    OP_2MMM (u32),
    OP_3XKK (u8, u16),
    OP_4XKK (u8, u16),
    OP_5XY0 (u8, u8),
    OP_6XKK (u8, u16),
    OP_7XKK (u8, u16),
    OP_8XY0 (u8, u8),
    OP_8XY1 (u8, u8),
    OP_8XY2 (u8, u8),
    OP_8XY3 (u8, u8),
    OP_8XY4 (u8, u8),
    OP_8XY5 (u8, u8),
    OP_9XY0 (u8, u8),
    OP_AMMM (u32),
    OP_BMMM (u32),
    OP_CXKK (u8, u16),
    OP_DXYN (u8, u8, u8),
    OP_EX9E (u8),
    OP_EXA1 (u8),
    OP_F000,
    OP_FX07 (u8),
    OP_FX0A (u8),
    OP_FX17 (u8),
    OP_FX18 (u8),
    OP_FX1E (u8),
    OP_FX29 (u8),
    OP_FX33 (u8),
    OP_FX55 (u8),
    OP_FX65 (u8),
    OP_FX70 (u8),
    OP_FX71 (u8),
    OP_FX72 (u8),
    
}

fn decode(instruction: u16) -> Opcode {
    // todo: fill this in
    panic!()
}

fn main() {
    let mut chip8 = create_chip8();
    chip8.emulate_cycle()
}
