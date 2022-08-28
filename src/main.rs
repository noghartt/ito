pub mod stack;

use stack::{ VM, Opcode };

fn main() {
  let mut vm = VM::new();

  vm.add_instruction(Opcode::IPSH(5)).unwrap();
  vm.add_instruction(Opcode::IPSH(10)).unwrap();
  vm.add_instruction(Opcode::IPSH(15)).unwrap();
  vm.add_instruction(Opcode::ISUB).unwrap();
  vm.add_instruction(Opcode::IMUL).unwrap();
  vm.add_instruction(Opcode::IPSH(1)).unwrap();
  vm.add_instruction(Opcode::IPSH(2)).unwrap();
  vm.add_instruction(Opcode::IADD).unwrap();
  vm.add_instruction(Opcode::IADD).unwrap();

  vm.run();
}
