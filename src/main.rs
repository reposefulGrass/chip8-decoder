
use std::convert::TryFrom;


struct Register(u8);

type Immediate = u16;

#[derive(Debug)]
enum Instruction {
	SYS(Immediate),
	CLS,
	RET,
	JP(Immediate),
	CALL(Immediate),
	SE_imm(Register, Immediate),
	SNE(Register, Immediate),
	SE_reg(Register, Register),
	LD_imm(Register, Immediate),
	ADD_imm(Register, Immediate),
	LD_reg(Register, Register),
	OR(Register, Register),
	AND(Register, Register),
	XOR(Register, Register),
	ADD_reg(Register, Register),
	SUB(Register, Register),
	SHR(Register, Register),
	SUBN(Register, Register),
	SHL(Register, Register),
	INVALID
}

impl TryFrom<u8> for Register {
	type Error = &'static str;	

	fn try_from (value: u8) -> Result<Register, Self::Error> {
		if value > 15 {
			return Err(format!("Invalid register `{}`.", value));	
		}	

		Ok(Register(value))
	}
}

impl Instruction {
	fn decode (bytes: u16) -> Self {
		let nibble1 = (bytes & 0xF000) >> 12;
		let nibble2 = (bytes & 0x0F00) >> 8;
		let nibble3 = (bytes & 0x00F0) >> 4;
		let nibble4 = (bytes & 0x000F) >> 0;

		match (nibble1) >> 12 {
			0 => {
				match bytes & 0x0FFF {
					0x0E0 => Instruction::CLS,
					0x0EE => Instruction::RET,
					_ 	  => Instruction::SYS(bytes & 0x0FFF),
				}
			},	
			1 => Instruction::JP(bytes & 0x0FFF),
			2 => Instruction::CALL(bytes & 0x0FFF),
			3 => {
				if let Ok(r1) = Register::TryFrom(nibble2) {
					Instruction::SE_imm(r1, bytes & 0x00FF)
				} else {
					Instruction::INVALID
				}
			},
			4 => Instruction::SNE(r1, bytes & 0x00FF),
			5 => {
				if nibble4 == 0 {
					Instruction::SE_reg(r1, r2)
				} else {
					Instruction::INVALID
				}
			},
			6 => Instruction::LD_imm(r1, bytes & 0x00FF),
			7 => Instruction::ADD_imm(r1, bytes & 0x00FF),
			8 => {
				match nibble4 {
					0 => Instruction::LD_reg(r1, r2),
					1 => Instruction::OR(r1, r2),
					2 => Instruction::AND(r1, r2),
					3 => Instruction::XOR(r1, r2),
					4 => Instruction::ADD_reg(r1, r2),
					5 => Instruction::SUB(r1, r2),
					6 => Instruction::SHR(r1, r2),
					7 => Instruction::SUBN(r1, r2),
					0xE => Instruction::SHL(r1, r2),
					_ => Instruction::INVALID,
				}
			},
			_ => {
				Instruction::INVALID
			}
		}
	}
}

fn main() {
	let instructions: Vec<u16> = vec![
		0x0001,
		0x00E0,
		0x00EE,
		0x1001,
		0x2001,
		0x3102,
	];

	for &instr in instructions.iter() {
		println!("{:?}", Instruction::decode(instr));
	}
}
