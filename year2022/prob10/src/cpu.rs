use std::ops::RangeInclusive;

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

    pub fn sprite_range(&self) -> RangeInclusive<i32> {
        let start = self.reg_a - 1;
        let end = self.reg_a + 1;
        start..=end
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

    #[test]
    fn test_cpu_sprite_range() {
        let cpu = CPU {
            reg_a: 5,
            cycles: 0,
        };

        assert_eq!(cpu.sprite_range(), 4..=6);
    }
}
