use crate::cpu::processor::Processor;

#[test]
fn lda_immediate() {
    let mut processor = Processor::new();
    processor.interpret(vec![0xa9, 0xc0, 0x00]);
    assert_eq!(processor.a, 0xc0);
    let mut processor = Processor::new();
    processor.interpret(vec![0xa9, 0x00, 0x00]);
    assert_eq!(processor.status & 0b0000_0010, 0b10);
    let mut processor = Processor::new();
    processor.interpret(vec![0xa9, 0xff, 0x00]);
    assert_eq!(processor.status & 0b1000_0000, 0x80);
}

#[test]
fn ldx_immediate() {
    let mut processor = Processor::new();
    processor.interpret(vec![0xa2, 0xc0, 0x00]);
    assert_eq!(processor.x, 0xc0);
    let mut processor = Processor::new();
    processor.interpret(vec![0xa2, 0x00, 0x00]);
    assert_eq!(processor.status & 0b0000_0010, 0b10);
    let mut processor = Processor::new();
    processor.interpret(vec![0xa2, 0xff, 0x00]);
    assert_eq!(processor.status & 0b1000_0000, 0x80);
}

#[test]
fn ldy_immediate() {
    let mut processor = Processor::new();
    processor.interpret(vec![0xa0, 0xc0, 0x00]);
    assert_eq!(processor.y, 0xc0);
    let mut processor = Processor::new();
    processor.interpret(vec![0xa0, 0x00, 0x00]);
    assert_eq!(processor.status & 0b0000_0010, 0b10);
    let mut processor = Processor::new();
    processor.interpret(vec![0xa0, 0xff, 0x00]);
    assert_eq!(processor.status & 0b1000_0000, 0x80);
}

#[test]
fn inx() {
    let mut processor = Processor::new();
    processor.interpret(vec![0xa2, 0xfe, 0xe8, 0xe8, 0x00]);
    assert_eq!(processor.x, 0x00);
    assert_eq!(processor.status & 0b0000_0010, 0b10);
}

#[test]
fn iny() {
    let mut processor = Processor::new();
    processor.interpret(vec![0xa0, 0xfe, 0xc8, 0xc8, 0x69, 0x00]);
    assert_eq!(processor.y, 0x00);
    assert_eq!(processor.status & 0b0000_0010, 0b10);
}