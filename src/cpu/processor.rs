use log;

pub struct Processor {
    // registers
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub status: u8,
    pub sp: u8,
    pub pc: u16,
    // memory
    pub memory: [u8; 1 << 16],
}

#[allow(dead_code)]
impl Processor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            a: 0,
            status: 0,
            sp: 0,
            pc: 0,
            memory: [0; 1 << 16],
        }
    }
    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.a = 0;
        self.status = 0;
        self.sp = 0;
        self.pc = self.mem_read_u16(0xfffc);
    }
    /// Read from memory; assuming `address` is big-endian
    fn mem_read_u8(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    /// Read two bytes from memory; assuming the `address` is big-endian
    /// And the bytes are assumed to be little-endian
    fn mem_read_u16(&self, address: u16) -> u16 {
        let bytes = [self.memory[address as usize], self.memory[(address + 1) as usize]];
        u16::from_le_bytes(bytes)
    }
    /// Write to memory; assuming `address` is big-endian
    fn mem_write_u8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
    /// Write to memory; assuming `address` is big-endian
    /// And `value` is also big-endian
    /// Internally, will write `value` in _little-endian_ mode.
    fn mem_write_u16(&mut self, address: u16, value: u16) {
        let le_bytes = u16::to_le_bytes(value);
        self.memory[address as usize] = le_bytes[0];
        self.memory[(address + 1) as usize] = le_bytes[1];
    }
    pub fn load_program(&mut self, program: &[u8]) {
        // programs are loaded at 0x8000
        // but after reset interrupt
        // CPU reads the two bytes at 0xfffc
        // it then jumps to that address
        // so we load the program at 0x8000
        // and write the value `0x8000` at the address `0xfffc`
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xfffc, 0x8000);
    }
    fn inx(&mut self) -> bool {
        self.x = self.x.wrapping_add(1);
        self.set_zero_and_neg(self.x);
        false
    }
    fn iny(&mut self) -> bool {
        self.y = self.y.wrapping_add(1);
        self.set_zero_and_neg(self.y);
        false
    }
    fn lda_immediate(&mut self, param: u8) -> bool {
        self.a = param;
        self.set_zero_and_neg(param);
        false
    }
    fn ldx_immediate(&mut self, param: u8) -> bool {
        self.x = param;
        self.set_zero_and_neg(param);
        false
    }
    fn ldy_immediate(&mut self, param: u8) -> bool {
        self.y = param;
        self.set_zero_and_neg(param);
        false
    }
    fn tax(&mut self) -> bool {
        self.x = self.a;
        self.set_zero_and_neg(self.x);
        false
    }
    fn set_zero_and_neg(&mut self, value: u8) {
        if value == 0 {
            log::info!("[{:x}] Setting Zero Flag", self.pc);
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }
        if value & 0b1000_0000 != 0 {
            log::info!("[{:x}] Setting Negative Flag", self.pc);
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
    pub fn start_exec(&mut self, program: &[u8]) {
        self.load_program(program);
        self.reset();
        self.run_program();
    }
    pub fn run_program(&mut self) {
        loop {
            // TODO : Add fetch cycles and implement bus
            // Fetch opcode
            let opcode = self.memory[self.pc as usize];
            // Increment PC
            self.pc += 1;
            if match opcode {
                0xa9 => self.lda_immediate(self.memory[self.pc as usize]),
                0xa2 => self.ldx_immediate(self.memory[self.pc as usize]),
                0xa0 => self.ldy_immediate(self.memory[self.pc as usize]),
                0x00 => { return; }
                0xe8 => self.inx(),
                0xc8 => self.iny(),
                0xaa => self.tax(),
                opcode => {
                    log::error!("Reached unmatched opcode : {:x}", opcode);
                    false
                }
            } {
                log::info!("Program Counter changed to : {:x}", self.pc);
            }
        }
    }
    pub fn interpret(&mut self, instr_stream: Vec<u8>) {
        self.start_exec(&instr_stream);
    }
}