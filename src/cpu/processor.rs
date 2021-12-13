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
    pub fn interpret(&mut self, instr_stream: Vec<u8>) {
        loop {
            // TODO : Add fetch cycles and implement bus
            // Fetch opcode
            let opcode = instr_stream[self.pc as usize];
            // Increment PC
            self.pc += 1;
            match opcode {
                0xa9 => self.lda_immediate(instr_stream[self.pc as usize]),
                0xa2 => self.ldx_immediate(instr_stream[self.pc as usize]),
                0xa0 => self.ldy_immediate(instr_stream[self.pc as usize]),
                0x00 => { return; },
                0xaa => self.tax(),
                opcode => {
                    log::error!("Reached unmatched opcode : {:x}", opcode);
                    false
                }
            };
        }
    }
}