/* 
    0x73EE (opcode)
    73: high byte
        7 high nibble, 3 low nibble
    EE: low byte
        E high nibble, E low nibble

    Representation:
        c: high byte, high nibble
        x: high byte, low nibble
        y: low byte, high nibble
        d: low byte, low nibble

    To extract nibbles, we perform the & operation with F and then right shift the remaining bits.

    for example: opcode = 0x8014 
        c = (opcode & 0xF000) >> 12 = 8 (means arithmetic/logical operations between registers)
        x = (opcode & 0x0F00) >> 8 = 0 (register index 0, V0)
        y = (opcode & 0x00F0) >> 4 = 1 (register index 1, V1)
        d = (opcode & 0x000F) >> 0 = 4 (add register 1 to 0)
*/
struct CPU {
    memory: [u8; 0x1000],
    program_counter: usize,
    registers: [u8; 16],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;

        op_byte1 << 8 | op_byte2
    }
    
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
        
            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; },
                (0x8, _, _, 0x4) => self.add(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1; // in case of overflow we flip the last register vF to 1 
        } else {
            self.registers[0xF] = 0;
        }
    }
}


fn main(){
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 26;
    cpu.registers[3] = 20;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14;
    mem[2] = 0x80; mem[3] = 0x24;
    mem[4] = 0x80; mem[5] = 0x34;
    
    cpu.run();
    assert_eq!(cpu.registers[0], 61);
    println!("5 + 10 + 26 + 20 = {}", cpu.registers[0]);
}