use super::*;
use std::mem::size_of;

#[test]
fn test_instruction_is_32bits() {
    // Each instruction should be 4 bytes long
    assert_eq!(size_of::<Instruction>(), 4);
}
