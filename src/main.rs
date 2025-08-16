struct CPU {
    memory: [u8; 0x1000],
    program_counter: usize,
    registers: [u8; 16],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {    
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.program_counter] as u16;
            let op_byte2 = self.memory[self.program_counter + 1] as u16;
            let opcode = (op_byte1 << 8) | op_byte2;

            self.program_counter += 2;
            
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let kk = (opcode & 0x00FF) as u8;
            let op_minor = (opcode & 0x000F) as u8;
        
            let addr = opcode & 0x0FFF; 

            match opcode {
                0x0000 => { return; },
                0x00E0 => { /* Clear Screen */},
                0x00EE => { self.rtn(); },
                0x1000..=0x1FFF => { self.jmp(addr); },
                0x2000..=0x2FFF => { self.call(addr); },
                0x3000..=0x3FFF => { self.se(x, kk); },
                0x4000..=0x4FFF => { self.sne(x, kk); },
                0x5000..=0x5FFF => { self.se(x,y); },
                0x6000..=0x6FFF => { self.ld(x, kk); },
                0x7000..=0x7FFF => { self.add(x, kk); },
                0x8000..=0x8FFF => {
                    match op_minor {
                        0 => { self.ld(x, self.registers[y as usize]); },
                        1 => { self.or_xy(x,y); },
                        2 => { self.and_xy(x,y); },
                        3 => { self.xor_xy(x,y); },
                        4 => { self.add_xy(x,y); },
                        _ => { todo!("opcode: {:04x}", opcode); },
                    }
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    // (6xkk) LD sets the register vx with kk 
    fn ld(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] = kk;
    }
    
    // (7xkk) Adds the value kk into reg vx
    fn add(&mut self, vx: u8, kk: u8) {
        self.registers[vx as usize] += kk;
    }

    fn se(&mut self, vx: u8, kk: u8){
        if vx == kk {
            self.program_counter += 2;
        }
    }

    fn sne(&mut self, vx: u8, kk: u8){
        if vx != kk {
            self.program_counter += 2;
        }
    }

    // (1nnn) JUMP to nnn
    fn jmp(&mut self, addr: u16){
        self.program_counter = addr as usize;
    }

    // (2nnn) CALL sub-routine at nnn
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack Overflow")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    // (00EE) RTN return from the current call
    fn rtn(&mut self) {
        if self.stack_pointer == 0{
            panic!("underflow")
        }
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    // (7xkk)
    fn add_xy(&mut self, x: u8, y: u8) {
        let sum = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
        
        self.registers[0xF] = if sum > 0xFF {1} else {0};

        self.registers[x as usize] = (sum & 0xFF) as u8;
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];
        
        self.registers[x as usize] = x_ & y_;
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];
        
        self.registers[x as usize] = x_ | y_;
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];
        
        self.registers[x as usize] = x_ ^ y_;
    }
}


fn main(){
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0x000] = 0x21; cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21; cpu.memory[0x003] = 0x00;

    cpu.memory[0x100] = 0x80; cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80; cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00; cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10*2) + (10*2) = {}", cpu.registers[0]);
}

// loading a function into memory: use slice or mannual mapping
// fn main() {
//     let mut memory: [u8; 4096] = [0; 4096];
//     let mem = &mut memory;

//     mem[0x100] = 0x80; mem[0x101] = 0x14;
//     mem[0x102] = 0x80; mem[0x103] = 0x14; 
//     mem[0x104] = 0x00; mem[0x105] = 0xEE; 

//     println!("{:?}", &mem[0x100..0x106]);
// }