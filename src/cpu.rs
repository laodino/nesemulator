pub enum StatusType {
    N,
    V,
    B,
    D,
    I,
    Z,
    C,
}
#[derive(Debug)]
pub enum AddressingModes {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
    UndefindeMode,
}

pub struct CPU {
    register_a: u8, //a寄存器，累加器
    register_x: u8, //x寄存器
    register_y: u8, //y寄存器
    //(从7 [最高]到0 [最低])： NV-BDIZC
    status: u8,           //标志位
    program_counter: u16, //程序计数器
    memory: [u8; 0xFFFF], //内存
}

impl CPU {
    pub fn new(&mut self) -> CPU {
        CPU {
            register_a : 0,
            register_x : 0,
            register_y : 0,
            status:0,
            program_counter:0,
            memory:[0;0xFFFF]
        }
    }
    //NES uses little endian
    //Real Address	0x8000
    // Address packed in big-endian	80 00
    // Address packed in little-endian	00 80
    //     ADDRESSES	VECTOR
    // $FFFA and $FFFB	NMI
    // $FFFC and $FFFD	Reset
    // $FFFE and $FFFF	IRQ/BRK

    pub fn write_to_memory_u8(&mut self, add: u16, value: u8) {
        self.memory[add as usize] = value;
    }
    pub fn read_from_memory_u8(&self, add: u16) -> u8 {
        self.memory[add as usize]
    }

    pub fn write_to_memory_u16(&mut self, add: u16, value: u16) {
        let low = (value >> 8) as u8;
        let high = (value & 0x00ff) as u8;
        self.write_to_memory_u8(add, low);
        self.write_to_memory_u8(add + 1, high);
    }

    pub fn read_from_memory_u16(&self, add: u16) -> u16 {
        let low = self.read_from_memory_u8(add) as u16;
        let high = self.read_from_memory_u8(add + 1) as u16;
        (high << 8) | low
    }

    pub fn setstatus(&mut self, statype: StatusType, flag: bool) {
        match statype {
            StatusType::N => {
                if flag {
                    self.status = self.status | 0b1000_0000;
                } else {
                    self.status = self.status & 0b0111_1111;
                }
            }
            StatusType::V => {
                if flag {
                    self.status = self.status | 0b0100_0000;
                } else {
                    self.status = self.status & 0b1011_1111;
                }
            }
            StatusType::B => {
                if flag {
                    self.status = self.status | 0b0001_0000;
                } else {
                    self.status = self.status & 0b1110_1111;
                }
            }
            StatusType::D => {
                if flag {
                    self.status = self.status | 0b0000_1000;
                } else {
                    self.status = self.status & 0b1111_0111;
                }
            }
            StatusType::I => {
                if flag {
                    self.status = self.status | 0b0000_0100;
                } else {
                    self.status = self.status & 0b1111_1011;
                }
            }
            StatusType::Z => {
                if flag {
                    self.status = self.status | 0b0000_0010;
                } else {
                    self.status = self.status & 0b1111_1101;
                }
            }
            StatusType::C => {
                if flag {
                    self.status = self.status | 0b0000_0001;
                } else {
                    self.status = self.status & 0b1111_1110;
                }
            }
        }
    }

    pub fn get_operand_address(&mut self, mode: AddressingModes) -> u16 {
        match mode {
            //立即数，本质是一个数
            AddressingModes::Immediate => self.program_counter,
            //八位地址
            AddressingModes::ZeroPage => self.read_from_memory_u8(self.program_counter) as u16,
            AddressingModes::ZeroPageX => {
                //读取参数
                let para = self.read_from_memory_u8(self.program_counter) as u16;
                let addr = para.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingModes::ZeroPageY => {
                let para = self.read_from_memory_u8(self.program_counter) as u16;
                let addr = para.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingModes::Relative => {
                let para = self.read_from_memory_u8(self.program_counter) as u16;
                let addr = self.program_counter + para + 1;
                addr
            }
            //16位地址
            AddressingModes::Absolute => self.read_from_memory_u16(self.program_counter),
            AddressingModes::AbsoluteX => {
                let para = self.read_from_memory_u16(self.program_counter);
                let addr = para.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingModes::AbsoluteY => {
                let para = self.read_from_memory_u16(self.program_counter);
                let addr = para.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingModes::Indirect => {
                let para = self.read_from_memory_u16(self.program_counter);
                let low = self.read_from_memory_u8(para) as u16;
                let high = self.read_from_memory_u8(para.wrapping_add(1)) as u16;
                (high << 8) | (low & 0x00ff)
            }
            //indirect x,先把操作数加上 x获得地址，再去读地址，地址+1
            AddressingModes::IndexedIndirect => {
                let para = self.read_from_memory_u16(self.program_counter);
                let base = para + (self.register_x as u16);
                let low = self.read_from_memory_u8(base) as u16;
                let high = self.read_from_memory_u8(base.wrapping_add(1)) as u16;
                (high << 8) | (low & 0x00ff)
            }
            //indirect y，先读操作数即地址，地址+1,再将读出来的加上y
            AddressingModes::IndirectIndexed => {
                let para = self.read_from_memory_u16(self.program_counter);
                let low = self.read_from_memory_u8(para) as u16;
                let high = self.read_from_memory_u8(para.wrapping_add(1)) as u16;
                let addr = (high << 8) | (low & 0x00ff);
                addr.wrapping_add(self.register_y as u16)
            }
            AddressingModes::UndefindeMode => {
                panic!("undefined mode :{:?}", mode);
            }
        }
    }

    pub fn setvaluetoregistera(&mut self, para: u8) {
        //标志位影响;Z和N
        //读取第二个参数
        self.register_a = para;
        //判断是否为0
        self.setstatus(StatusType::Z, self.register_a == 0);

        //判断是否为负
        self.setstatus(StatusType::N, self.register_a & 0b1000_0000 != 0);
    }

    pub fn setvaluetoregisterx(&mut self, para: u8) {
        //标志位影响;Z和N
        //读取第二个参数
        self.register_x = para;
        //判断是否为0
        self.setstatus(StatusType::Z, self.register_x == 0);

        //判断是否为负
        self.setstatus(StatusType::N, self.register_x & 0b1000_0000 != 0);
    }
    pub fn setvaluetoregistery(&mut self, para: u8) {
        //标志位影响;Z和N
        //读取第二个参数
        self.register_y = para;
        //判断是否为0
        self.setstatus(StatusType::Z, self.register_y == 0);

        //判断是否为负
        self.setstatus(StatusType::N, self.register_y & 0b1000_0000 != 0);
    }

    pub fn lda(&mut self, mode: AddressingModes) {
        let addr = self.get_operand_address(mode);
        let para = self.read_from_memory_u8(addr);
        self.setvaluetoregistera(para)
    }

    //初始化cpu
    pub fn load(&mut self, program: Vec<u8>) {
        //将程序从
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.write_to_memory_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.program_counter = self.read_from_memory_u16(0xFFFC);
    }
    //解析程序指令
    pub fn run(&mut self) {
        self.program_counter = 0;

        loop {
            //读取第一个参数
            let operatecode = self.read_from_memory_u8(self.program_counter);
            self.program_counter += 1;
            match operatecode {
                0xA9 => {
                    //LDA
                    self.lda(AddressingModes::Immediate);
                    self.program_counter += 1;
                }
                0xA5 => {
                    self.lda(AddressingModes::ZeroPage);
                    self.program_counter += 1;
                }
                0xB5 => {
                    self.lda(AddressingModes::ZeroPageX);
                    self.program_counter += 1;
                }
                0xAD => {
                    self.lda(sAddressingModes::Absolute);
                    self.program_counter += 1;
                }
                0xBD => {
                    self.lda(AddressingModes::ZeroPageX);
                    self.program_counter += 1;
                }
                0xB9 => {
                    self.lda(AddressingModes::ZeroPageY);
                    self.program_counter += 1;
                }
                0xA1 => {
                    self.lda(AddressingModes::IndexedIndirect);
                    self.program_counter += 1;
                }
                0xB1 => {
                    self.lda(AddressingModes::IndirectIndexed);
                    self.program_counter += 1;
                }
                0xA2 => {
                    //LDX
                    let para = self.read_from_memory_u8(self.program_counter);
                    self.setvaluetoregistera(para)
                }
                0xA0 => {
                    //LDY
                    let para = self.read_from_memory_u8(self.program_counter);
                    self.setvaluetoregistera(para)
                }
                0xAA => {
                    //TAX
                    self.setvaluetoregisterx(self.register_a);
                }
                0x8A => {
                    //TXA
                    self.setvaluetoregistera(self.register_x);
                }
                0xA8 => {
                    //TAY
                    self.setvaluetoregistery(self.register_a);
                }
                0x98 => {
                    //TYA
                    self.setvaluetoregistera(self.register_y)
                }
                0x00 => {
                    //BRK
                    return;
                }
                _ => print!("undefined operator!"),
            }
        }
    }
}
#[cfg(test)]

mod tests {
    // use super::*;

    // #[test]
    // fn setstatusworks() {
    //     let value = u16::from_le_bytes([0x34, 0x12]);
    //     assert_eq!(value, 0x1234);
    // }
}
