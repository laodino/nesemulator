use lazy_static::lazy_static;
use std::collections::HashMap;

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
    NoAddressingMode,
}

pub struct OpCode {
    pub addressmode: AddressingModes,
    pub opcode: u8,
    pub opname: &'static str,
    pub bytes: u8,
    pub cycles: u8,
}

impl OpCode {
    fn new(
        opcode: u8,
        opname: &'static str,
        addressmode: AddressingModes,
        bytes: u8,
        cycles: u8,
    ) -> Self {
        OpCode {
            addressmode: addressmode,
            opcode: opcode,
            opname: opname,
            bytes: bytes,
            cycles: cycles,
        }
    }
}

lazy_static! {
   pub static ref CPU_OPRAND:Vec<OpCode> = vec![
       //ADC
        OpCode::new(0x69,"ADC",AddressingModes::Immediate,2,2),
        OpCode::new(0x65,"ADC",AddressingModes::ZeroPage,2,3),
        OpCode::new(0x75,"ADC",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0x6D,"ADC",AddressingModes::Absolute,3,4),
        OpCode::new(0x7D,"ADC",AddressingModes::AbsoluteX,3,4),
        OpCode::new(0x79,"ADC",AddressingModes::AbsoluteY,3,4),
        OpCode::new(0x61,"ADC",AddressingModes::IndexedIndirect,2,6),
        OpCode::new(0x71,"ADC",AddressingModes::IndirectIndexed,2,5),
       //AND
        OpCode::new(0x29,"AND",AddressingModes::Immediate,2,2),
        OpCode::new(0x25,"AND",AddressingModes::ZeroPage,2,3),
        OpCode::new(0x35,"AND",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0x2D,"AND",AddressingModes::Absolute,3,4),
        OpCode::new(0x3D,"AND",AddressingModes::AbsoluteX,3,4),
        OpCode::new(0x39,"AND",AddressingModes::AbsoluteY,3,4),
        OpCode::new(0x21,"AND",AddressingModes::IndexedIndirect,2,6),
        OpCode::new(0x31,"AND",AddressingModes::IndirectIndexed,2,5),
        //ASL
        OpCode::new(0x0A,"ASL",AddressingModes::NoAddressingMode,1,2),
        OpCode::new(0x06,"ASL",AddressingModes::ZeroPage,2,5),
        OpCode::new(0x16,"ASL",AddressingModes::ZeroPageX,2,6),
        OpCode::new(0x0E,"ASL",AddressingModes::Absolute,3,6),
        OpCode::new(0x1E,"ASL",AddressingModes::AbsoluteX,3,7),
        //BCC
        OpCode::new(0x90,"BCC",AddressingModes::Relative,2,2),
        //BCS
        OpCode::new(0xB0,"BCS",AddressingModes::Relative,2,2),
        //BEQ
        OpCode::new(0xF0,"BEQ",AddressingModes::Immediate,2,2),
        //BIT
        OpCode::new(0x24,"BIT",AddressingModes::ZeroPage,2,3),
        OpCode::new(0x2C,"BIT",AddressingModes::Absolute,3,4),
        //BMI
        OpCode::new(0x30,"BMI",AddressingModes::Relative,2,2),
        //BNE
        OpCode::new(0xD0,"BNE",AddressingModes::Relative,2,2),
        //BPL
        OpCode::new(0x10,"BPL",AddressingModes::Relative,2,2),
        //BRK
        OpCode::new(0x00,"BRK",AddressingModes::NoAddressingMode,1,7),
        //BVC
        OpCode::new(0x50,"BVC",AddressingModes::Relative,2,2),
        //BVS
        OpCode::new(0x70,"BVS",AddressingModes::Relative,2,2),
        //CLC
        OpCode::new(0x18,"CLC",AddressingModes::NoAddressingMode,1,2),
        //CLD
        OpCode::new(0xD8,"CLD",AddressingModes::NoAddressingMode,1,2),
        //CLI
        OpCode::new(0x58,"CLI",AddressingModes::NoAddressingMode,1,2),
        //CLV
        OpCode::new(0xB8,"CLV",AddressingModes::NoAddressingMode,1,2),
        //CMP
        OpCode::new(0xC9,"CMP",AddressingModes::Immediate,2,2),
        OpCode::new(0xC5,"CMP",AddressingModes::ZeroPage,2,3),
        OpCode::new(0xD5,"CMP",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0xCD,"CMP",AddressingModes::Absolute,3,4),
        OpCode::new(0xDD,"CMP",AddressingModes::AbsoluteX,3,4),
        OpCode::new(0xD9,"CMP",AddressingModes::AbsoluteY,3,4),
        OpCode::new(0xC1,"CMP",AddressingModes::IndexedIndirect,2,6),
        OpCode::new(0xD1,"CMP",AddressingModes::IndirectIndexed,2,5),
        //CPX
        OpCode::new(0xE0,"CPX",AddressingModes::Immediate,2,2),
        OpCode::new(0xE4,"CPX",AddressingModes::ZeroPage,2,3),
        OpCode::new(0xEC,"CPX",AddressingModes::Absolute,3,4),
        //CPY
        OpCode::new(0xC0,"CPY",AddressingModes::Immediate,2,2),
        OpCode::new(0xC4,"CPY",AddressingModes::ZeroPage,2,3),
        OpCode::new(0xCC,"CPY",AddressingModes::Absolute,3,4),
        //DEC
        OpCode::new(0xC6,"DEC",AddressingModes::ZeroPage,2,5),
        OpCode::new(0xD6,"DEC",AddressingModes::ZeroPageX,2,6),
        OpCode::new(0xCE,"DEC",AddressingModes::Absolute,3,6),
        OpCode::new(0xDE,"DEC",AddressingModes::AbsoluteX,3,7),
        //DEX
        OpCode::new(0xCA,"DEX",AddressingModes::NoAddressingMode,1,2),
        //DEY
        OpCode::new(0x88,"DEY",AddressingModes::NoAddressingMode,1,2),
        //EOR
        OpCode::new(0x49,"EOR",AddressingModes::Immediate,2,2),
        OpCode::new(0x45,"EOR",AddressingModes::ZeroPage,2,3),
        OpCode::new(0x55,"EOR",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0x4D,"EOR",AddressingModes::Absolute,3,4),
        OpCode::new(0x5D,"EOR",AddressingModes::AbsoluteX,3,4),
        OpCode::new(0x59,"EOR",AddressingModes::AbsoluteY,3,4),
        OpCode::new(0x41,"EOR",AddressingModes::IndexedIndirect,2,6),
        OpCode::new(0x51,"EOR",AddressingModes::IndirectIndexed,2,5),
        //INC
        OpCode::new(0xE6,"INC",AddressingModes::ZeroPage,2,5),
        OpCode::new(0xF6,"INC",AddressingModes::ZeroPageX,2,6),
        OpCode::new(0xEE,"INC",AddressingModes::Absolute,3,6),
        OpCode::new(0xFE,"INC",AddressingModes::AbsoluteX,3,7),
        //INX
        OpCode::new(0xE8,"INX",AddressingModes::NoAddressingMode,1,2),
        //INY
        OpCode::new(0xC8,"INY",AddressingModes::NoAddressingMode,1,2),
        //JMP
        OpCode::new(0x4C,"JMP",AddressingModes::Absolute,3,3),
        OpCode::new(0x6C,"JMP",AddressingModes::Indirect,3,5),
        //JSR
        OpCode::new(0x20,"JSR",AddressingModes::Absolute,3,6),
       //LDA
        OpCode::new(0xa9,"LDA",AddressingModes::Immediate,2,2),
        OpCode::new(0xa5,"LDA",AddressingModes::ZeroPage,2,3),
        OpCode::new(0xb5,"LDA",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0xad,"LDA",AddressingModes::Absolute,3,4),
        OpCode::new(0xbd,"LDA",AddressingModes::AbsoluteX,3,4),//+1 if page crossed
        OpCode::new(0xb9,"LDA",AddressingModes::AbsoluteY,3,4),//+1 if page crossed
        OpCode::new(0xa1,"LDA",AddressingModes::IndexedIndirect,2,6),
        OpCode::new(0xba,"LDA",AddressingModes::IndirectIndexed,2,5),//+1 if page crossed
        //LDX
        OpCode::new(0xa2,"LDX",AddressingModes::Immediate,2,2),
        OpCode::new(0xa6,"LDX",AddressingModes::ZeroPage,2,3),
        OpCode::new(0xb6,"LDX",AddressingModes::ZeroPageY,2,4),
        OpCode::new(0xae,"LDX",AddressingModes::Absolute,3,4),
        OpCode::new(0xbe,"LDX",AddressingModes::AbsoluteY,3,4),
        //LDY
        OpCode::new(0xa0,"LDY",AddressingModes::Immediate,2,2),
        OpCode::new(0xa4,"LDY",AddressingModes::ZeroPage,2,3),
        OpCode::new(0xb4,"LDY",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0xaC,"LDY",AddressingModes::Absolute,3,4),
        OpCode::new(0xbC,"LDY",AddressingModes::AbsoluteX,3,4),
        //LSR
        OpCode::new(0x4A,"LSR",AddressingModes::NoAddressingMode,1,2),
        OpCode::new(0x46,"LSR",AddressingModes::ZeroPage,2,5),
        OpCode::new(0x56,"LSR",AddressingModes::ZeroPageX,2,6),
        OpCode::new(0x4E,"LSR",AddressingModes::Absolute,3,6),
        OpCode::new(0x5E,"LSR",AddressingModes::AbsoluteX,3,7),
        //NOP
        OpCode::new(0xEA,"NOP",AddressingModes::NoAddressingMode,1,2),
        //ORA
        OpCode::new(0x09,"ORA",AddressingModes::Immediate,2,2),
        OpCode::new(0x05,"ORA",AddressingModes::ZeroPage,2,3),
        OpCode::new(0x15,"ORA",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0x0D,"ORA",AddressingModes::Absolute,3,4),
        OpCode::new(0x1D,"ORA",AddressingModes::AbsoluteX,3,4),
        OpCode::new(0x19,"ORA",AddressingModes::AbsoluteY,3,4),
        OpCode::new(0x01,"ORA",AddressingModes::IndexedIndirect,2,6),
        OpCode::new(0x11,"ORA",AddressingModes::IndirectIndexed,2,5),
        //PHA
        OpCode::new(0x48,"PHA",AddressingModes::NoAddressingMode,1,3),
        //PHP
        OpCode::new(0x08,"PHP",AddressingModes::NoAddressingMode,1,3),
        //PLA
        OpCode::new(0x68,"PLA",AddressingModes::NoAddressingMode,1,4),
        //PLP
        OpCode::new(0x28,"PLP",AddressingModes::NoAddressingMode,1,4),
        //ROL
        OpCode::new(0x2A,"ROL",AddressingModes::NoAddressingMode,1,2),
        OpCode::new(0x26,"ROL",AddressingModes::ZeroPage,2,5),
        OpCode::new(0x36,"ROL",AddressingModes::ZeroPageX,2,6),
        OpCode::new(0x2E,"ROL",AddressingModes::Absolute,3,6),
        OpCode::new(0x3E,"ROL",AddressingModes::AbsoluteX,3,7),
        //ROR
        OpCode::new(0x6A,"ROR",AddressingModes::NoAddressingMode,1,2),
        OpCode::new(0x66,"ROR",AddressingModes::ZeroPage,2,5),
        OpCode::new(0x76,"ROR",AddressingModes::ZeroPageX,2,6),
        OpCode::new(0x6E,"ROR",AddressingModes::Absolute,3,6),
        OpCode::new(0x7E,"ROT",AddressingModes::AbsoluteX,3,7),
        //RTI
        OpCode::new(0x40,"RTI",AddressingModes::NoAddressingMode,1,6),
        //RTS
        OpCode::new(0x60,"RTS",AddressingModes::NoAddressingMode,1,6),
        //SBC
        OpCode::new(0xE9,"SBC",AddressingModes::Immediate,2,2),
        OpCode::new(0xE5,"SBC",AddressingModes::ZeroPage,2,3),
        OpCode::new(0xF5,"SBC",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0xED,"SBC",AddressingModes::Absolute,3,4),
        OpCode::new(0xFD,"SBC",AddressingModes::AbsoluteX,3,4),
        OpCode::new(0xF9,"SBC",AddressingModes::AbsoluteY,3,4),
        OpCode::new(0xE1,"SBC",AddressingModes::IndexedIndirect,2,6),
        OpCode::new(0xF1,"SBC",AddressingModes::IndirectIndexed,2,5),
        //SEC
        OpCode::new(0x38,"SEC",AddressingModes::NoAddressingMode,1,2),
        //SED
        OpCode::new(0xF8,"SED",AddressingModes::NoAddressingMode,1,2),
        //SEI
        OpCode::new(0x78,"SEI",AddressingModes::NoAddressingMode,1,2),
        //STA
        OpCode::new(0x85,"STA",AddressingModes::ZeroPage,2,3),
        OpCode::new(0x95,"STA",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0x8d,"STA",AddressingModes::Absolute,3,4),
        OpCode::new(0x9d,"STA",AddressingModes::AbsoluteX,3,5),//+1 if page crossed
        OpCode::new(0x99,"STA",AddressingModes::AbsoluteY,3,5),//+1 if page crossed
        OpCode::new(0x81,"STA",AddressingModes::IndexedIndirect,2,6),
        OpCode::new(0x91,"STA",AddressingModes::IndirectIndexed,2,6),//+1 if page crossed
        //STX
        OpCode::new(0x86,"STX",AddressingModes::ZeroPage,2,3),
        OpCode::new(0x96,"STX",AddressingModes::ZeroPageY,2,4),
        OpCode::new(0x8e,"STX",AddressingModes::Absolute,3,4),
        //STY
        OpCode::new(0x84,"STY",AddressingModes::ZeroPage,2,3),
        OpCode::new(0x94,"STY",AddressingModes::ZeroPageX,2,4),
        OpCode::new(0x8c,"STY",AddressingModes::Absolute,3,4),
        //TAX,隐寻址
        OpCode::new(0xAA,"TAX",AddressingModes::NoAddressingMode,1,2),
        //TAY
        OpCode::new(0xA8,"TAY",AddressingModes::NoAddressingMode,1,2),
        //TSX
        OpCode::new(0xBA,"TSX",AddressingModes::NoAddressingMode,1,2),
        //TXA
        OpCode::new(0x8A,"TXA",AddressingModes::NoAddressingMode,1,2),
        //TXS
        OpCode::new(0x9A,"TXS",AddressingModes::NoAddressingMode,1,2),
        //TYA
        OpCode::new(0x98 ,"TYA",AddressingModes::NoAddressingMode,1,2),

        //BRK
        OpCode::new(0x00,"BRK",AddressingModes::NoAddressingMode,1,7),
    ];

   pub static ref  CPU_OPRAND_HASHMAP:HashMap<u8,&'static OpCode> ={
            let mut m  = HashMap::new();
             for opcode in &*CPU_OPRAND{
               m.insert(opcode.opcode,opcode);
             }
             m
    };
}
