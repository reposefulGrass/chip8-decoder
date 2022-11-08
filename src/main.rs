
#[derive(Debug)]
enum Instruction {
	SYS(u16),
	CLS,
	RET,
	INVALID
}

impl Instruction {
	fn decode (bytes: u16) -> Self {
		match (bytes & 0xF000) >> 12 {
			0 => {
				match bytes & 0x0FFF {
					0x0E0 => Instruction::CLS,
					0x0EE => Instruction::RET,
					_ 	  => Instruction::SYS(bytes & 0x0FFF),
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
	];

	for &instr in instructions.iter() {
		println!("{:?}", Instruction::decode(instr));
	}
}
