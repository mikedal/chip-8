
pub mod chip8{
    use rand::{thread_rng, Rng};

    const MEM_SIZE: usize = 4096;
    const REGISTER_COUNT: usize = 16;
    const DISPLAY_HEIGHT: usize = 32;
    const DISPLAY_WIDTH: usize = 64;
    const STACK_SIZE: usize = 16;
    const KEY_COUNT: usize = 16;
    const FONT_SIZE: usize = 80;

    pub struct Chip8 {
        memory: [u8; MEM_SIZE],
        // general purpose registers
        V: [u8; REGISTER_COUNT],
        // index register
        I: usize,
        pc: usize,
        // monochrome, so use bool
        gfx: [bool; DISPLAY_HEIGHT * DISPLAY_WIDTH],
        delay_timer: u8,
        sound_timer: u8,
        stack: [usize; STACK_SIZE],
        sp: usize,
        keys: [bool; KEY_COUNT],
        opcode: Opcode,
        // pitch:
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
        fn fetch(&self) -> u16 {
            (self.memory[self.pc] as u16).rotate_left(8) | self.memory[self.pc + 1] as u16
        }

        fn execute(&mut self) {
            match self.opcode {
                Opcode::OP_0000 => {
                    self.pc += 2; // NOP
                }
                Opcode::OP_00E0 => {
                    self.clear_screen();
                    self.pc += 2;
                }
                Opcode::OP_00EE => {
                    // return
                    self.sp -= 1;
                    self.pc = self.stack[self.sp] + 2;
                }
                Opcode::OP_1MMM(mmm) => {
                    // goto (not considered harmful}
                    self.pc = mmm;
                }
                Opcode::OP_2MMM(mmm) => {
                    // call subroutine
                    self.stack[self.sp] = self.pc;
                    self.sp += 1;
                    self.pc = mmm;
                }
                Opcode::OP_3XKK(x, kk) => {
                    // skip if VX = KK
                    if self.V[x] == kk {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                Opcode::OP_4XKK(x, kk) => {
                    // skip if VX != KK
                    if self.V[x] != kk {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                Opcode::OP_5XY0(x, y) => {
                    if self.V[x] == self.V[y] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                Opcode::OP_6XKK(x, kk) => {
                    self.V[x] = kk;
                    self.pc += 2;
                }
                Opcode::OP_7XKK(x, kk) => {
                    self.V[x] += kk;
                    self.pc += 2;
                }
                Opcode::OP_8XY0(x, y) => {
                    self.V[x] = self.V[y];
                    self.pc += 2;
                }
                Opcode::OP_8XY1(x, y) => {
                    self.V[x] |= self.V[y];
                    self.pc += 2;
                }
                Opcode::OP_8XY2(x, y) => {
                    self.V[x] &= self.V[y];
                    self.pc += 2;
                }
                Opcode::OP_8XY3(x, y) => {
                    self.V[x] ^= self.V[y];
                    self.pc += 2;
                }
                Opcode::OP_8XY4(x, y) => {
                    // this is assuming it overflows. need to test
                    let result = self.V[x] + self.V[y];
                    if result < self.V[x] {
                        self.V[0xF] = 1;
                    }
                    self.V[x] = result;
                    self.V[x] += self.V[y];
                    self.pc += 2;
                }
                Opcode::OP_8XY5(x, y) => {
                    // this is assuming it overflows. need to test
                    let result = self.V[x] - self.V[y];
                    if self.V[x] < self.V[y] {
                        self.V[0xF] = 0;
                    }
                    self.V[x] = result;
                    self.V[x] += self.V[y];
                    self.pc += 2;
                }
                Opcode::OP_9XY0(x, y) => {
                    if self.V[x] != self.V[y] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                Opcode::OP_AMMM(mmm) => {
                    self.I = mmm;
                    self.pc += 2;
                }
                Opcode::OP_BMMM(mmm) => {
                    self.pc = mmm + (self.V[0] as usize);
                }
                Opcode::OP_CXKK(x, kk) => {
                    // AND kk w/ a random value
                    let mut rng = thread_rng();
                    let rnd: u8 = rng.gen_range(0..255);
                    self.V[x] = rnd & kk;
                    self.pc += 2;
                }
                Opcode::OP_DXYN(x, y, n) => {
                    //TODO: do something
                }
                Opcode::OP_EX9E(x) => {
                    // skip if key[Vx] is down
                    let key = self.V[x] as usize;
                    if self.keys[key] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                Opcode::OP_EXA1(x) => {
                    // skip if key[Vx] is down
                    let key = self.V[x] as usize;
                    if !self.keys[key] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                Opcode::OP_F000 => {
                    // TODO: implement
                }
                Opcode::OP_FX07(x) => {
                    self.V[x] = self.delay_timer;
                    self.pc += 2;
                }
                Opcode::OP_FX0A(x) => {
                    // TODO: implement
                    // wait for keypress and save value to Vx
                }
                Opcode::OP_FX15(x) => {
                    self.delay_timer = self.V[x];
                    self.pc += 2;
                }
                // Opcode::OP_FX17(x) => {
                //     self.pitch = self.V[x];
                // }
                Opcode::OP_FX18(x) => {}
                Opcode::OP_FX1E(x) => {}
                Opcode::OP_FX29(x) => {}
                Opcode::OP_FX33(x) => {}
                Opcode::OP_FX55(x) => {}
                Opcode::OP_FX65(x) => {}
                Opcode::OP_FX70(x) => {}
                Opcode::OP_FX71(x) => {}
                Opcode::OP_FX72(x) => {}
            }
        }

        pub fn emulate_cycle(&mut self) {
            let raw_opcode = self.fetch();
            self.opcode = decode(raw_opcode)
        }

        fn clear_screen(&mut self) {
            // TODO: do something
        }
    }

    pub fn create_chip8() -> Chip8 {
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
    #[allow(non_camel_case_types)]
    enum Opcode {
        OP_0000,
        OP_00E0,
        OP_00EE,
        OP_1MMM(usize),
        OP_2MMM(usize),
        OP_3XKK(usize, u8),
        OP_4XKK(usize, u8),
        OP_5XY0(usize, usize),
        OP_6XKK(usize, u8),
        OP_7XKK(usize, u8),
        OP_8XY0(usize, usize),
        OP_8XY1(usize, usize),
        OP_8XY2(usize, usize),
        OP_8XY3(usize, usize),
        OP_8XY4(usize, usize),
        OP_8XY5(usize, usize),
        OP_9XY0(usize, usize),
        OP_AMMM(usize),
        OP_BMMM(usize),
        OP_CXKK(usize, u8),
        OP_DXYN(usize, usize, u8),
        OP_EX9E(usize),
        OP_EXA1(usize),
        OP_F000,
        OP_FX07(usize),
        OP_FX0A(usize),
        OP_FX15(usize),
        // OP_FX17(usize),
        OP_FX18(usize),
        OP_FX1E(usize),
        OP_FX29(usize),
        OP_FX33(usize),
        OP_FX55(usize),
        OP_FX65(usize),
        OP_FX70(usize),
        OP_FX71(usize),
        OP_FX72(usize),
    }

    fn decode(instruction: u16) -> Opcode {
        match instruction & 0xF000 {
            0x0000 => {
                if instruction == 0x0000 {
                    Opcode::OP_0000
                } else if instruction == 0x00E0 {
                    Opcode::OP_00E0
                } else if instruction == 0x00EE {
                    Opcode::OP_00EE
                } else {
                    panic!()
                }
            }
            0x1000 => Opcode::OP_1MMM((instruction & 0x0FFF) as usize),
            0x2000 => Opcode::OP_2MMM((instruction & 0x0FFF) as usize),
            0x3000 => {
                let (x, kk) = decode_xkk(instruction);
                Opcode::OP_3XKK(x, kk)
            }
            0x4000 => {
                let (x, kk) = decode_xkk(instruction);
                Opcode::OP_4XKK(x, kk)
            }
            0x5000 => match instruction & 0x000F {
                0x0000 => {
                    let (x, y) = decode_xy(instruction);
                    Opcode::OP_5XY0(x, y)
                }
                _ => panic!("unknown opcode"),
            },
            0x6000 => {
                let (x, kk) = decode_xkk(instruction);
                Opcode::OP_6XKK(x, kk)
            }
            0x7000 => {
                let (x, kk) = decode_xkk(instruction);
                Opcode::OP_7XKK(x, kk)
            }
            0x8000 => match instruction & 0x000F {
                0x0000 => {
                    let (x, y) = decode_xy(instruction);
                    Opcode::OP_8XY0(x, y)
                }
                0x0001 => {
                    let (x, y) = decode_xy(instruction);
                    Opcode::OP_8XY1(x, y)
                }
                0x0002 => {
                    let (x, y) = decode_xy(instruction);
                    Opcode::OP_8XY2(x, y)
                }
                0x0003 => {
                    let (x, y) = decode_xy(instruction);
                    Opcode::OP_8XY3(x, y)
                }
                0x0004 => {
                    let (x, y) = decode_xy(instruction);
                    Opcode::OP_8XY4(x, y)
                }
                0x0005 => {
                    let (x, y) = decode_xy(instruction);
                    Opcode::OP_8XY5(x, y)
                }
                _ => panic!("unknown opcode"),
            },
            0x9000 => match instruction & 0x000F {
                0x0000 => {
                    let (x, y) = decode_xy(instruction);
                    Opcode::OP_9XY0(x, y)
                }
                _ => panic!("unknown opcode"),
            },
            0xA000 => Opcode::OP_AMMM((instruction & 0x0FFF) as usize),
            0xB000 => Opcode::OP_BMMM((instruction & 0x0FFF) as usize),
            0xC000 => {
                let (x, kk) = decode_xkk(instruction);
                Opcode::OP_CXKK(x, kk)
            }
            0xD000 => {
                let (x, y) = decode_xy(instruction);
                let n = (instruction & 0x000F) as u8;
                Opcode::OP_DXYN(x, y, n)
            }
            0xE000 => match instruction & 0x00FF {
                0x009E => Opcode::OP_EX9E(decode_x(instruction)),
                0x00A1 => Opcode::OP_EXA1(decode_x(instruction)),
                _ => panic!("unknown opcode"),
            },
            0xF000 => {
                if instruction == 0xF000 {
                    Opcode::OP_F000
                } else {
                    match instruction & 0x00FF {
                        0x0007 => Opcode::OP_FX07(decode_x(instruction)),
                        0x000A => Opcode::OP_FX0A(decode_x(instruction)),
                        0x0015 => Opcode::OP_FX15(decode_x(instruction)),
                        // 0x0017 => Opcode::OP_FX17(decode_x(instruction)),
                        0x0018 => Opcode::OP_FX18(decode_x(instruction)),
                        0x001E => Opcode::OP_FX1E(decode_x(instruction)),
                        0x0029 => Opcode::OP_FX29(decode_x(instruction)),
                        0x0033 => Opcode::OP_FX33(decode_x(instruction)),
                        0x0055 => Opcode::OP_FX55(decode_x(instruction)),
                        0x0065 => Opcode::OP_FX65(decode_x(instruction)),
                        0x0070 => Opcode::OP_FX70(decode_x(instruction)),
                        0x0071 => Opcode::OP_FX71(decode_x(instruction)),
                        0x0072 => Opcode::OP_FX72(decode_x(instruction)),
                        _ => panic!("unknown opcode"),
                    }
                }
            }
            _ => panic!("unknown opcode"),
        }
    }

    fn decode_xkk(instruction: u16) -> (usize, u8) {
        let x = (instruction.rotate_right(12) & 0x000F) as usize;
        let kk = (instruction & 0x00FF) as u8;
        (x, kk)
    }

    fn decode_xy(instruction: u16) -> (usize, usize) {
        let x = (instruction.rotate_right(12) & 0x000F) as usize;
        let y = (instruction.rotate_right(4) & 0x000F) as usize;
        (x, y)
    }
    fn decode_x(instruction: u16) -> usize {
        (instruction.rotate_right(12) & 0x000F) as usize
    }

}
