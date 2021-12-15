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

#[test]
fn lda_zero_page() {
    // LDA $c0
    let mut processor = Processor::new();
    processor.memory[0xc0] = 0xff;
    processor.interpret(vec![0xa5, 0xc0]);
    assert_eq!(processor.a, 0xff);
}

#[test]
fn ldx_zero_page() {
    // LDX $b3
    let mut processor = Processor::new();
    processor.memory[0xb3] = 0x69;
    processor.interpret(vec![0xa6, 0xb3]);
    assert_eq!(processor.x, 0x69);
}

#[test]
fn ldy_zero_page() {
    // LDY $69
    let mut processor = Processor::new();
    processor.memory[0x69] = 0x45; // 0x45 = 69 in decimal
    processor.interpret(vec![0xa4, 0x69]);
    assert_eq!(processor.y, 0x45);
}

#[test]
fn lda_zero_page_x() {
    // LDX #$45
    // LDA $00, X
    // expect A has value stored at $45 ($69)
    let mut processor = Processor::new();
    processor.memory[0x45] = 0x69;
    processor.interpret(vec![0xa2, 0x45, 0xb5, 0x00]);
    assert_eq!(processor.a, 0x69);
}

#[test]
fn ldy_zero_page_x() {
    // LDX #$45
    // LDY $00, X
    // expect Y has value stored at $45 ($69)
    let mut processor = Processor::new();
    processor.memory[0x45] = 0x69;
    processor.interpret(vec![0xa2, 0x45, 0xb4, 0x00]);
    assert_eq!(processor.y, 0x69);
}