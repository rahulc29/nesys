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

enum AddressMode {
    /// Puts 8-bit constant into a register
    Immediate(u8),
    /// For accessing $0000 to $00ff
    ZeroPage(u8),
    /// Zero page but `to_access = address + register(X)`
    /// Addition is wrapped; capable of overflow
    ZeroPageX(u8),
    /// `ZeroPageX` but for Y register
    ZeroPageY(u8),
    /// PC + address
    /// where `address` is a **_signed_** 8-bit integer
    Relative(i8),
    /// Access memory at absolute address
    /// Implementations using this enum must ensure that the address is stored big-endian
    Absolute(u16),
    /// Same as absolute but `to_access = address + register(X)`
    /// Addition is wrapped; capable of overflow
    AbsoluteX(u16),
    /// Same as `AbsoluteX` but for Y
    AbsoluteY(u16),
    /// Goes to the address and reads two bytes (little-endian)
    /// These read bytes are the actual address
    /// Only used by the `jmp` instruction
    Indirect(u16),
    IndexedIndirect(u8),
    IndirectIndexed(u8),
}

#[allow(dead_code)]
impl Processor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            a: 0,
            status: 0,
            sp: 0xff,
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
    fn eval_address_mode_u8(&self, address_mode: AddressMode) -> u8 {
        if let AddressMode::Immediate(constant) = address_mode {
            constant
        }
        self.mem_read_u8(self.operand_address(address_mode))
    }
    fn eval_address_mode_u16(&self, address_mode: AddressMode) -> u16 {
        self.mem_read_u16(self.operand_address(address_mode))
    }
    fn operand_address(&self, address_mode: AddressMode) -> u16 {
        match address_mode {
            AddressMode::Immediate(_) => {
                self.pc
            }
            AddressMode::ZeroPage(address) => {
                address as u16
            }
            AddressMode::ZeroPageX(address) => {
                let address = address.wrapping_add(self.x) as u16;
                address
            }
            AddressMode::ZeroPageY(address) => {
                let address = address.wrapping_add(self.y) as u16;
                address
            }
            AddressMode::Relative(offset) => {
                // Refactor : put in separate function
                let address = if offset < 0 {
                    let offset = (0 - offset) as u16;
                    self.pc - offset
                } else {
                    let offset = offset as u16;
                    self.pc + offset
                };
                address
            }
            AddressMode::Absolute(address) => {
                address
            }
            AddressMode::AbsoluteX(address) => {
                let address = address.wrapping_add(self.x as u16);
                address
            }
            AddressMode::AbsoluteY(address) => {
                let address = address.wrapping_add(self.y as u16);
                address
            }
            AddressMode::Indirect(address) => {
                let address = self.mem_read_u16(address);
                address
            }
            AddressMode::IndexedIndirect(address) => {
                let address = self.mem_read_u16((address + self.x) as u16);
                address
            }
            AddressMode::IndirectIndexed(address) => {
                let address = self.mem_read_u16(address as u16) + (self.y as u16);
                address
            }
        }
    }
    /// Read from memory; assuming `address` is native-endian
    fn mem_read_u8(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    /// Read two bytes from memory; assuming the `address` is native-endian
    /// And the bytes are assumed to be little-endian
    /// The returned value is native-endian
    fn mem_read_u16(&self, address: u16) -> u16 {
        let bytes = [self.memory[address as usize], self.memory[(address + 1) as usize]];
        u16::from_le_bytes(bytes)
    }
    /// Write to memory; assuming `address` is native-endian
    fn mem_write_u8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
    /// Write to memory; assuming `address` is native-endian
    /// And `value` is also native-endian
    /// Internally, will write `value` in _little-endian_ mode.
    fn mem_write_u16(&mut self, address: u16, value: u16) {
        let le_bytes = u16::to_le_bytes(value);
        self.memory[address as usize] = le_bytes[0];
        self.memory[(address + 1) as usize] = le_bytes[1];
    }
    pub fn load_program(&mut self, program: &[u8]) {
        // programs are loaded at 0x8000
        // but after reset interrupt
        // CPU reads the two bytes at 0xfffc (little-endian)
        // it then jumps to that address
        // so we load the program at 0x8000
        // and write the value `0x8000` at the address `0xfffc`
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xfffc, 0x8000);
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
            let next_byte = self.memory[self.pc as usize];
            let next_short = self.mem_read_u16(self.pc);
            if match opcode {
                0xa9 => self.lda_immediate(next_byte),
                0xa2 => self.ldx_immediate(next_byte),
                0xa0 => self.ldy_immediate(next_byte),
                0x00 => { return; }
                0xe8 => self.inx(),
                0xc8 => self.iny(),
                0xaa => self.tax(),
                0xa5 => self.lda_zero_page(next_byte),
                0xa6 => self.ldx_zero_page(next_byte),
                0xa4 => self.ldy_zero_page(next_byte),
                0xb5 => self.lda_zero_page_x(next_byte),
                0xb4 => self.ldy_zero_page_x(next_byte),
                0x69 => self.adc(AddressMode::Immediate(next_byte)),
                0x65 => self.adc(AddressMode::ZeroPage(next_byte)),
                0x75 => self.adc(AddressMode::ZeroPageX(next_byte)),
                0x6d => self.adc(AddressMode::Absolute(next_short)),
                0x7d => self.adc(AddressMode::AbsoluteX(next_short)),
                0x79 => self.adc(AddressMode::AbsoluteY(next_short)),
                0x61 => self.adc(AddressMode::IndexedIndirect(next_byte)),
                0x71 => self.adc(AddressMode::IndirectIndexed(next_byte)),
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

#[allow(dead_code)]
impl Processor {
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
    fn lda_zero_page(&mut self, address: u8) -> bool {
        self.a = self.mem_read_u8(self.operand_address(AddressMode::ZeroPage(address)));
        self.set_zero_and_neg(self.a);
        false
    }
    fn lda_zero_page_x(&mut self, address: u8) -> bool {
        self.a = self.mem_read_u8(self.operand_address(AddressMode::ZeroPageX(address)));
        self.set_zero_and_neg(self.a);
        false
    }
    fn lda_zero_page_y(&mut self, address: u8) -> bool {
        self.a = self.mem_read_u8(self.operand_address(AddressMode::ZeroPageY(address)));
        self.set_zero_and_neg(self.a);
        false
    }
    fn ldx_immediate(&mut self, param: u8) -> bool {
        self.x = param;
        self.set_zero_and_neg(param);
        false
    }
    fn ldx_zero_page(&mut self, address: u8) -> bool {
        self.x = self.mem_read_u8(self.operand_address(AddressMode::ZeroPage(address)));
        self.set_zero_and_neg(self.x);
        false
    }
    fn ldx_zero_page_x(&mut self, address: u8) -> bool {
        self.x = self.mem_read_u8(self.operand_address(AddressMode::ZeroPageX(address)));
        self.set_zero_and_neg(self.x);
        false
    }
    fn ldx_zero_page_y(&mut self, address: u8) -> bool {
        self.x = self.mem_read_u8(self.operand_address(AddressMode::ZeroPageX(address)));
        self.set_zero_and_neg(self.x);
        false
    }
    fn ldy_immediate(&mut self, param: u8) -> bool {
        self.y = param;
        self.set_zero_and_neg(param);
        false
    }
    fn ldy_zero_page(&mut self, address: u8) -> bool {
        self.y = self.mem_read_u8(self.operand_address(AddressMode::ZeroPage(address)));
        self.set_zero_and_neg(self.y);
        false
    }
    fn ldy_zero_page_x(&mut self, address: u8) -> bool {
        self.y = self.mem_read_u8(self.operand_address(AddressMode::ZeroPageX(address)));
        self.set_zero_and_neg(self.y);
        false
    }
    fn ldy_zero_page_y(&mut self, address: u8) -> bool {
        self.y = self.mem_read_u8(self.operand_address(AddressMode::ZeroPageY(address)));
        self.set_zero_and_neg(self.y);
        false
    }
    fn tax(&mut self) -> bool {
        self.x = self.a;
        self.set_zero_and_neg(self.x);
        false
    }
    fn adc(&mut self, address: AddressMode) -> bool {
        let to_add = self.eval_address_mode_u8(address);
        let (new_val, overflow) = self.a.overflowing_add(to_add);
        if overflow {
            self.status |= 0b0000_0001;
        }
        self.a = new_val;
        self.set_zero_and_neg(self.a);
        false
    }
}