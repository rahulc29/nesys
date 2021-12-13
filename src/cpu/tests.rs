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
}