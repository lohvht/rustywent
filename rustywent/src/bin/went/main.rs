use wentcore::stackvm::chunk::{Chunk, Instruction};
use wentcore::stackvm::value::Value;
fn main() {
    let chunk = &mut Chunk::new();
    chunk.add_instruction(Instruction::Return, (1, 1));
    chunk.add_constant_instruction(Value::Double(1.2), (1, 15));
    let chunk = &*chunk;
    chunk.print_disassembled("test chunk")
}
