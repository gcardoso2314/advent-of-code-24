use std::ops::BitXor;

#[derive(Debug)]
struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    program: Vec<u32>,
    instruction_pointer: usize,
    output: Vec<String>,
}

impl Computer {
    fn combo_operand(&self, operand: u32) -> u32 {
        match operand {
            0..4 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Unexpected operand"),
        }
    }

    fn adv(&mut self, operand: u32) {
        let op = self.combo_operand(operand);
        self.reg_a = self.reg_a / 2_u32.pow(op as u32);
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, operand: u32) {
        self.reg_b = self.reg_b.bitxor(operand);
        self.instruction_pointer += 2;
    }

    fn bst(&mut self, operand: u32) {
        let op = self.combo_operand(operand);
        self.reg_b = op.rem_euclid(8);
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self, operand: u32) {
        if self.reg_a == 0 {
            self.instruction_pointer += 2;
        } else {
            self.instruction_pointer = operand as usize;
        }
    }

    fn bxc(&mut self, _operand: u32) {
        self.reg_b = self.reg_b.bitxor(self.reg_c);
        self.instruction_pointer += 2;
    }

    fn out(&mut self, operand: u32) {
        let op = self.combo_operand(operand);
        self.output.push((op.rem_euclid(8)).to_string());
        self.instruction_pointer += 2;
    }

    fn bdv(&mut self, operand: u32) {
        let op = self.combo_operand(operand);
        self.reg_b = self.reg_a / 2_u32.pow(op as u32);
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, operand: u32) {
        let op = self.combo_operand(operand);
        self.reg_c = self.reg_a / 2_u32.pow(op as u32);
        self.instruction_pointer += 2;
    }

    fn execute_program(&mut self) -> String {
        while self.instruction_pointer < self.program.len() - 1 {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("invalid opcode"),
            }
        }

        self.output.join(",")
    }

    fn from_input(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let reg_a: u32 = lines[0].split_once(": ").unwrap().1.parse().unwrap();
        let reg_b: u32 = lines[1].split_once(": ").unwrap().1.parse().unwrap();
        let reg_c: u32 = lines[2].split_once(": ").unwrap().1.parse().unwrap();
        let program: Vec<u32> = lines[4]
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect();

        Computer {
            reg_a,
            reg_b,
            reg_c,
            program,
            instruction_pointer: 0,
            output: vec![],
        }
    }
}

pub fn process_part_one(input: &str) -> String {
    let mut computer = Computer::from_input(input);
    computer.execute_program()
}

pub fn process_part_two(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST_INPUT_2: &str = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(TEST_INPUT), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(process_part_one(TEST_INPUT_2), "0,3,5,4,3,0");
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        assert_eq!(process_part_two(TEST_INPUT), 9021)
    }
}
