#[derive(Debug)]
pub enum Opcode {
  IPSH(i64),
  IADD,
  ISUB,
  IMUL,
}

#[derive(Debug)]
pub enum StackValue {
  INumber(i64),
}

impl From<StackValue> for i64 {
  fn from(item: StackValue) -> i64 {
    match item {
      StackValue::INumber(i) => i,
    }
  }
}

#[derive(Debug)]
pub struct VM {
  ip: usize,
  stack: Vec<StackValue>,
  instructions: Vec<Opcode>,
}

impl VM {
  pub fn new() -> VM {
    VM {
      ip: 0,
      stack: Vec::new(),
      instructions: Vec::new(),
    }
  }

  pub fn run(&mut self) {
    for opcode in self.instructions.iter() {
      match opcode {
        Opcode::IPSH(i) => {
          self.stack.push(StackValue::INumber(*i));
        }
        Opcode::IADD => {
          let x: i64 = self.stack.pop().unwrap().into();
          let y: i64 = self.stack.pop().unwrap().into();

          self.stack.push(StackValue::INumber(x + y));
        }
        Opcode::ISUB => {
          let x: i64 = self.stack.pop().unwrap().into();
          let y: i64 = self.stack.pop().unwrap().into();

          self.stack.push(StackValue::INumber(y - x));
        }
        Opcode::IMUL => {
          let x: i64 = self.stack.pop().unwrap().into();
          let y: i64 = self.stack.pop().unwrap().into();

          self.stack.push(StackValue::INumber(x * y));
        }
      }

      self.ip += 1;
    }

    println!("{:#?}", self);
  }

  pub fn add_instruction(&mut self, opcode: Opcode) -> Result<(), &'static str> {
    self.instructions.push(opcode);
    Ok(())
  }
}
