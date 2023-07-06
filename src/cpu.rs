use crate::cpuoperand::AddressingModes;
use crate::cpuoperand::CPU_OPRAND_HASHMAP;

const PROGRAMSTARTADDRESS: u16 = 0x8000;
const MEMORYSIZE: usize = 0xFFFF;
const RESETADDRESS: u16 = 0xFFFC;
//const STACKPOINTERSTART: u16 = 0x0100;
const STACKRESET: u8 = 0xFD;

pub enum StatusType {
    NegativeFlag,
    OverflowFlag,
    Break,
    Break2,
    DecimalModeFlag,
    InterruptDisable,
    ZeroFlag,
    CarryFlag,
}

pub struct CPU {
    register_a: u8, //a寄存器，累加器
    register_x: u8, //x寄存器
    register_y: u8, //y寄存器
    //(从7 [最高]到0 [最低])： NV-BDIZC
    status: u8,               //标志位
    program_counter: u16,     //程序计数器
    memory: [u8; MEMORYSIZE], //内存
    stack_pointer: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; MEMORYSIZE],
            stack_pointer: STACKRESET,
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
        let low = (value & 0x00ff) as u8;
        let high = (value >> 8) as u8;
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
            StatusType::NegativeFlag => {
                if flag {
                    self.status = self.status | 0b1000_0000;
                } else {
                    self.status = self.status & 0b0111_1111;
                }
            }
            StatusType::OverflowFlag => {
                if flag {
                    self.status = self.status | 0b0100_0000;
                } else {
                    self.status = self.status & 0b1011_1111;
                }
            }
            StatusType::Break2 => {
                if flag {
                    self.status = self.status | 0b0010_0000;
                } else {
                    self.status = self.status & 0b1101_1111;
                }
            }
            StatusType::Break => {
                if flag {
                    self.status = self.status | 0b0001_0000;
                } else {
                    self.status = self.status & 0b1110_1111;
                }
            }
            StatusType::DecimalModeFlag => {
                if flag {
                    self.status = self.status | 0b0000_1000;
                } else {
                    self.status = self.status & 0b1111_0111;
                }
            }
            StatusType::InterruptDisable => {
                if flag {
                    self.status = self.status | 0b0000_0100;
                } else {
                    self.status = self.status & 0b1111_1011;
                }
            }
            StatusType::ZeroFlag => {
                if flag {
                    self.status = self.status | 0b0000_0010;
                } else {
                    self.status = self.status & 0b1111_1101;
                }
            }
            StatusType::CarryFlag => {
                if flag {
                    self.status = self.status | 0b0000_0001;
                } else {
                    self.status = self.status & 0b1111_1110;
                }
            }
        }
    }

    pub fn getstatus(&mut self, statype: StatusType) -> bool {
        match statype {
            StatusType::NegativeFlag => {
                let tempstatus = self.status;
                (tempstatus & 0b1000_0000) >> 7 == 1
            }
            StatusType::OverflowFlag => {
                let tempstatus = self.status;
                (tempstatus & 0b0100_0000) >> 6 == 1
            }
            StatusType::Break2 => {
                let tempstatus = self.status;
                (tempstatus & 0b0010_0000) >> 5 == 1
            }
            StatusType::Break => {
                let tempstatus = self.status;
                (tempstatus & 0b0001_0000) >> 4 == 1
            }
            StatusType::DecimalModeFlag => {
                let tempstatus = self.status;
                (tempstatus & 0b0000_1000) >> 3 == 1
            }
            StatusType::InterruptDisable => {
                let tempstatus = self.status;
                (tempstatus & 0b0000_0100) >> 2 == 1
            }
            StatusType::ZeroFlag => {
                let tempstatus = self.status;
                (tempstatus & 0b0000_0010) >> 1 == 1
            }
            StatusType::CarryFlag => {
                let tempstatus = self.status;
                (tempstatus & 0b0000_0001) == 1
            }
        }
    }

    pub fn get_operand_address(&mut self, mode: &AddressingModes) -> u16 {
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
            //indirect x,先把操作数加上 x获得地址，再去读地址、地址+1
            AddressingModes::IndexedIndirect => {
                let para = self.read_from_memory_u16(self.program_counter);
                let base = para + (self.register_x as u16);
                let low = self.read_from_memory_u8(base) as u16;
                let high = self.read_from_memory_u8(base.wrapping_add(1)) as u16;
                (high << 8) | (low & 0x00ff)
            }
            //indirect y，先读操作数即地址、地址+1,再将读出来的加上y
            AddressingModes::IndirectIndexed => {
                let para = self.read_from_memory_u16(self.program_counter);
                let low = self.read_from_memory_u8(para) as u16;
                let high = self.read_from_memory_u8(para.wrapping_add(1)) as u16;
                let addr = (high << 8) | (low & 0x00ff);
                addr.wrapping_add(self.register_y as u16)
            }
            AddressingModes::NoAddressingMode => {
                panic!("undefined mode :{:?}", mode);
            }
        }
    }

    pub fn setvaluetoregistera(&mut self, para: u8) {
        //标志位影响;Z和N
        //读取第二个参数
        self.register_a = para;
        //判断是否为0
        self.setstatus(StatusType::ZeroFlag, self.register_a == 0);

        //判断是否为负
        self.setstatus(StatusType::NegativeFlag, self.register_a & 0b1000_0000 != 0);
    }

    pub fn setvaluetoregisterx(&mut self, para: u8) {
        //标志位影响;Z和N
        //读取第二个参数
        self.register_x = para;
        //判断是否为0
        self.setstatus(StatusType::ZeroFlag, self.register_x == 0);

        //判断是否为负
        self.setstatus(StatusType::NegativeFlag, self.register_x & 0b1000_0000 != 0);
    }
    pub fn setvaluetoregistery(&mut self, para: u8) {
        //标志位影响;Z和N
        //读取第二个参数
        self.register_y = para;
        //判断是否为0
        self.setstatus(StatusType::ZeroFlag, self.register_y == 0);

        //判断是否为负
        self.setstatus(StatusType::NegativeFlag, self.register_y & 0b1000_0000 != 0);
    }

    pub fn adc(&mut self,mode:&AddressingModes){
        let addr = self.get_operand_address(mode);
        let mut para = self.read_from_memory_u8(addr);
        let carrybit = self.getstatus(StatusType::CarryFlag);
        let signbit = self.getstatus(StatusType::NegativeFlag);
        if carrybit{
            para = para.wrapping_add(1);
        }
        self.register_a = para;
        self.setstatus(StatusType::CarryFlag,false);

        self.setstatus(StatusType::ZeroFlag, self.register_a == 0);

        self.setstatus(StatusType::NegativeFlag, self.register_y & 0b1000_0000 != 0);

        if self.getstatus(StatusType::NegativeFlag)!=signbit{
            self.setstatus(StatusType::OverflowFlag,true);
            self.setstatus(StatusType::CarryFlag,true);
        }
    }

    pub fn lda(&mut self, mode: &AddressingModes) {
        let addr = self.get_operand_address(mode);
        let para = self.read_from_memory_u8(addr);
        self.setvaluetoregistera(para)
    }

    pub fn ldx(&mut self, mode: &AddressingModes) {
        let addr = self.get_operand_address(mode);
        let para = self.read_from_memory_u8(addr);
        self.setvaluetoregisterx(para)
    }

    pub fn ldy(&mut self, mode: &AddressingModes) {
        let addr = self.get_operand_address(mode);
        let para = self.read_from_memory_u8(addr);
        self.setvaluetoregistery(para)
    }

    pub fn sta(&mut self, mode: &AddressingModes) {
        let addr = self.get_operand_address(mode);
        self.write_to_memory_u8(addr, self.register_a);
    }

    pub fn stx(&mut self, mode: &AddressingModes) {
        let addr = self.get_operand_address(mode);
        self.write_to_memory_u8(addr, self.register_x);
    }

    pub fn sty(&mut self, mode: &AddressingModes) {
        let addr = self.get_operand_address(mode);
        self.write_to_memory_u8(addr, self.register_y);
    }

    pub fn tax(&mut self) {
        self.register_x = self.register_a;
         //判断是否为0
        self.setstatus(StatusType::ZeroFlag, self.register_x == 0);
         //判断是否为负
        self.setstatus(StatusType::NegativeFlag, self.register_x & 0b1000_0000 != 0);
    }

    pub fn tay(&mut self) {
        self.register_y = self.register_a;
         //判断是否为0
        self.setstatus(StatusType::ZeroFlag, self.register_y == 0);
         //判断是否为负
        self.setstatus(StatusType::NegativeFlag, self.register_y & 0b1000_0000 != 0);
    }

    pub fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
         //判断是否为0
        self.setstatus(StatusType::ZeroFlag, self.register_x == 0);
         //判断是否为负
        self.setstatus(StatusType::NegativeFlag, self.register_x & 0b1000_0000 != 0);
    }

    pub fn txa(&mut self) {
        self.register_a = self.register_x;
         //判断是否为0
        self.setstatus(StatusType::ZeroFlag, self.register_a == 0);
         //判断是否为负
        self.setstatus(StatusType::NegativeFlag, self.register_a & 0b1000_0000 != 0);
    }

    pub fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    pub fn tya(&mut self) {
        self.register_a = self.register_y;
         //判断是否为0
        self.setstatus(StatusType::ZeroFlag, self.register_a == 0);
         //判断是否为负
        self.setstatus(StatusType::NegativeFlag, self.register_a & 0b1000_0000 != 0);
    }

    //初始化cpu
    pub fn load(&mut self, program: Vec<u8>) {
        //将程序从
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.write_to_memory_u16(RESETADDRESS, PROGRAMSTARTADDRESS);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;
        self.program_counter = self.read_from_memory_u16(RESETADDRESS);
        self.stack_pointer = STACKRESET;
    }

    //解析程序指令
    pub fn run(&mut self) {
        let ref opcodes = *CPU_OPRAND_HASHMAP;
        loop {
            //读取第一个参数
            let operatecode = self.read_from_memory_u8(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;
            let opcode = opcodes
                .get(&operatecode)
                .expect(&format!("OpCode {:x} is not recognized", operatecode));

            match opcode.opname {
                "ADC"=>{
                    self.adc(&opcode.addressmode);
                }
                "LDA" => {
                    //LDA
                    self.lda(&opcode.addressmode);
                }
                "LDX" => {
                    //LDX
                    self.ldx(&opcode.addressmode);
                }
                "LDY" => {
                    //LDY
                    self.ldy(&opcode.addressmode);
                }
                "STA" => {
                    self.sta(&opcode.addressmode);
                }
                "STX" => {
                    self.stx(&opcode.addressmode);
                }
                "STY" => {
                    self.sty(&opcode.addressmode);
                }
                "TAX" => {
                    //TAX
                    self.tax();
                }
                "TAY" => {
                    //TAY
                    self.tay();
                }
                "TSX" => {
                    //TAY
                    self.tsx();
                }
                "TXA" => {
                    //TXA
                    self.txa();
                }
                "TXS" => {
                    //TAY
                    self.txs();
                }
                "TYA" => {
                    //TYA
                    self.tya();
                }

                
                "BRK" => {
                    //BRK
                    return;
                }
                _ => print!("undefined operator!"),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.bytes - 1) as u16;
            }
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn setstatusworks() {
        let value = u16::from_le_bytes([0x34, 0x12]);
        assert_eq!(value, 0x1234);
    }

    #[test]
    fn write_u8_should_work() {
        let mut ncpu = CPU::new();
        ncpu.write_to_memory_u8(0x0001, 0x01);
        assert_eq!(ncpu.read_from_memory_u8(0x0001), 0x01);
    }

    #[test]
    fn write_u16_should_work() {
        let mut ncpu = CPU::new();
        ncpu.write_to_memory_u16(0x0021, 0x0001);
        assert_eq!(ncpu.read_from_memory_u16(0x0021), 0x0001);
    }

    #[test]
    fn setstatus_c_should_work() {
        let mut ncpu = CPU::new();
        ncpu.setstatus(StatusType::CarryFlag, true);
        assert_eq!(ncpu.status, 0b0000_0001);
        ncpu.setstatus(StatusType::CarryFlag, false);
        assert_eq!(ncpu.status, 0b0000_0000);
        ncpu.setstatus(StatusType::CarryFlag, true);
        assert_eq!(ncpu.getstatus(StatusType::CarryFlag), true);
    }

    #[test]
    fn setstatus_z_should_work() {
        let mut ncpu = CPU::new();
        ncpu.setstatus(StatusType::ZeroFlag, true);
        assert_eq!(ncpu.status, 0b0000_0010);
        ncpu.setstatus(StatusType::ZeroFlag, false);
        assert_eq!(ncpu.status, 0b0000_0000);
    }

    #[test]
    fn setstatus_i_should_work() {
        let mut ncpu = CPU::new();
        ncpu.setstatus(StatusType::InterruptDisable, true);
        assert_eq!(ncpu.status, 0b0000_0100);
        ncpu.setstatus(StatusType::InterruptDisable, false);
        assert_eq!(ncpu.status, 0b0000_0000);
    }

    #[test]
    fn setstatus_d_should_work() {
        let mut ncpu = CPU::new();
        ncpu.setstatus(StatusType::DecimalModeFlag, true);
        assert_eq!(ncpu.status, 0b0000_1000);
        ncpu.setstatus(StatusType::DecimalModeFlag, false);
        assert_eq!(ncpu.status, 0b0000_0000);
    }

    #[test]
    fn setstatus_b_should_work() {
        let mut ncpu = CPU::new();
        ncpu.setstatus(StatusType::Break, true);
        assert_eq!(ncpu.status, 0b0001_0000);
        ncpu.setstatus(StatusType::Break, false);
        assert_eq!(ncpu.status, 0b0000_0000);
    }

    #[test]
    fn setstatus_v_should_work() {
        let mut ncpu = CPU::new();
        ncpu.setstatus(StatusType::OverflowFlag, true);
        assert_eq!(ncpu.status, 0b0100_0000);
        ncpu.setstatus(StatusType::OverflowFlag, false);
        assert_eq!(ncpu.status, 0b0000_0000);
    }

    #[test]
    fn setstatus_n_should_work() {
        let mut ncpu = CPU::new();
        ncpu.setstatus(StatusType::NegativeFlag, true);
        assert_eq!(ncpu.status, 0b1000_0000);
        ncpu.setstatus(StatusType::NegativeFlag, false);
        assert_eq!(ncpu.status, 0b0000_0000);
    }
    #[test]
    fn set_data_to_rigestera_should_work() {
        let mut ncpu = CPU::new();
        ncpu.setvaluetoregistera(0x55);
        assert_eq!(ncpu.register_a, 0x55);
    }

    #[test]
    fn reset_should_work() {
        let mut ncpu = CPU::new();
        ncpu.load(vec![0xa5, 0x10, 0x00]);
        ncpu.reset();
        assert_eq!(ncpu.program_counter, 0x8000);
        let operatecode = ncpu.read_from_memory_u8(ncpu.program_counter);
        assert_eq!(operatecode, 0xa5);
    }

    #[test]
    fn lda_from_memory_should_work() {
        let mut ncpu = CPU::new();
        ncpu.write_to_memory_u8(0x10, 0x55);

        ncpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(ncpu.register_a, 0x55);
    }

    #[test]
    fn sta_should_work() {
        let mut ncpu = CPU::new();
        // ncpu.write_to_memory_u8(0x11, 0x10);
        ncpu.load(vec![0x85, 0x10, 0x00]);
        ncpu.reset();
        ncpu.register_a = 0x55;
        ncpu.run();
        assert_eq!(ncpu.read_from_memory_u8(0x10), 0x55);
    }
}
