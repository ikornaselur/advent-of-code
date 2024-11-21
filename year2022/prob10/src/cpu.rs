#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
    pub reg_a: i32,
    pub cycles: usize,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            reg_a: 1,
            cycles: 0,
        }
    }

    pub fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::AddX(value) => {
                self.reg_a += value;
                self.cycles += instruction.cycles();
            }
            Instruction::Noop => {
                self.cycles += instruction.cycles();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_run_instruction() {
        let mut cpu = CPU::new();

        cpu.run_instruction(Instruction::AddX(3));
        assert_eq!(cpu.reg_a, 4);
        assert_eq!(cpu.cycles, 2);

        cpu.run_instruction(Instruction::Noop);
        assert_eq!(cpu.reg_a, 4);
        assert_eq!(cpu.cycles, 3);

        cpu.run_instruction(Instruction::AddX(-5));
        assert_eq!(cpu.reg_a, -1);
        assert_eq!(cpu.cycles, 5);
    }
}
