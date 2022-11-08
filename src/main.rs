
#[derive(Debug)]
enum Instruction {
	SYS(u16),
	INVALID
}

impl Instruction {
	fn decode (bytes: u16) -> Self {
		match (bytes & 0xF000) >> 12 {
			0 => {
				Instruction::SYS(bytes & 0x0FFF)	
			},	
			_ => {
				Instruction::INVALID
			}
		}
	}
}

fn main() {
	let instr: u16 = 0x0001;
	println!("{:?}", Instruction::decode(instr));
}
