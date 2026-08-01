use crate::cpu::*;
use crate::utils::*;

#[rustfmt::skip]
const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
//  0x00,   0x01,  0x02,   0x03,  0x04,   0x05,   0x06,  0x07,  0x08,  0x09,  0x0A,  0x0B,   0x0C,   0x0D,   0x0E,  0x0F
    nop_00, ld_01, ld_02, inc_03, inc_04, dec_05, ld_06, todo,  ld_08, todo,  ld_0a, dec_0b, inc_0c, dec_0d, ld_0e, todo,  // 0x00
    todo,   ld_11, ld_12, inc_13, inc_14, dec_15, ld_16, todo,  todo,  todo,  ld_1a, dec_1b, inc_1c, dec_1d, ld_1e, todo,  // 0x10
    todo,   ld_21, ld_22, inc_23, inc_24, dec_25, ld_26, todo,  todo,  todo,  ld_2a, dec_2b, inc_2c, dec_2d, ld_2e, todo,  // 0x20
    todo,   ld_31, ld_32, inc_33, inc_34, dec_35, ld_36, todo,  todo,  todo,  ld_3a, dec_3b, inc_3c, dec_3d, ld_3e, todo,  // 0x30
    ld_40,  ld_41, ld_42, ld_43,  ld_44,  ld_45,  ld_46, ld_47, ld_48, ld_49, ld_4a, ld_4b,  ld_4c,  ld_4d,  ld_4e, ld_4f, // 0x40
    ld_50,  ld_51, ld_52, ld_53,  ld_54,  ld_55,  ld_56, ld_57, ld_58, ld_59, ld_5a, ld_5b,  ld_5c,  ld_5d,  ld_5e, ld_5f, // 0x50
    ld_60,  ld_61, ld_62, ld_63,  ld_64,  ld_65,  ld_66, ld_67, ld_68, ld_69, ld_6a, ld_6b,  ld_6c,  ld_6d,  ld_6e, ld_6f, // 0x60
    ld_70,  ld_71, ld_72, ld_73,  ld_74,  ld_75,  todo,  ld_77, ld_78, ld_79, ld_7a, ld_7b,  ld_7c,  ld_7d,  ld_7e, ld_7f, // 0x70
    todo,   todo,  todo,  todo,   todo,   todo,   todo,  todo,  todo,  todo,  todo,  todo,   todo,   todo,   todo,  todo,  // 0x80
    todo,   todo,  todo,  todo,   todo,   todo,   todo,  todo,  todo,  todo,  todo,  todo,   todo,   todo,   todo,  todo,  // 0x90
    todo,   todo,  todo,  todo,   todo,   todo,   todo,  todo,  todo,  todo,  todo,  todo,   todo,   todo,   todo,  todo,  // 0xA0
    todo,   todo,  todo,  todo,   todo,   todo,   todo,  todo,  todo,  todo,  todo,  todo,   todo,   todo,   todo,  todo,  // 0xB0
    todo,   todo,  todo,  todo,   todo,   todo,   todo,  todo,  todo,  todo,  todo,  todo,   todo,   todo,   todo,  todo,  // 0xC0
    todo,   todo,  todo,  todo,   todo,   todo,   todo,  todo,  todo,  todo,  todo,  todo,   todo,   todo,   todo,  todo,  // 0xD0
    ld_e0,  todo,  ld_e2, todo,   todo,   todo,   todo,  todo,  todo,  todo,  ld_ea, todo,   todo,   todo,   todo,  todo,  // 0xE0
    ld_f0,  todo,  ld_f2, todo,   todo,   todo,   todo,  todo,  ld_f8, ld_f9, ld_fa, todo,   todo,   todo,   todo,  todo,  // 0xF0
];

fn todo(_cpu: &mut Cpu) -> u8 {
    todo!();
}

// LD BC, A
fn ld_02(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::BC);
    let val = cpu.get_r8(Regs::A);
    cpu.write_ram(addr, val);
    2
}

// LD B, u8
fn ld_06(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::B, val);
    2
}

// LD A, [BC]
fn ld_0a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::BC);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    2
}

// LD C, u8
fn ld_0e(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::C, val);
    2
}

// LD DE, u16
fn ld_11(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::DE, val);
    3
}

// LD [DE], A
fn ld_12(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::DE);
    let val = cpu.get_r8(Regs::A);
    cpu.write_ram(addr, val);
    2
}

// LD D, u8
fn ld_16(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::D, val);
    2
}

// NOP - No Operation
fn nop_00(_cpu: &mut Cpu) -> u8 {
    // Nop does absolutely nothing
    1
}

// INC C
fn inc_0c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::C);
    1
}
// DEC C
fn dec_0d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::C);
    1
}
// DEC BC
fn dec_0b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::BC);
    2
}
// INC DE
fn inc_13(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::DE);
    2
}
// INC D
fn inc_14(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::D);
    1
}
// DEC D
fn dec_15(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::D);
    1
}
// DEC DE
fn dec_1b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::DE);
    2
}
// INC E
fn inc_1c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::E);
    1
}
// DEC E
fn dec_1d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::E);
    1
}
// INC HL
fn inc_23(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::HL);
    2
}
// INC H
fn inc_24(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::H);
    1
}
// DEC H
fn dec_25(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::H);
    1
}
// DEC HL
fn dec_2b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::HL);
    2
}
// INC L
fn inc_2c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::L);
    1
}
// DEC L
fn dec_2d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::L);
    1
}
// INC SP
fn inc_33(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::SP);
    2
}
// INC A
fn inc_3c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::A);
    1
}
// DEC A
fn dec_3d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::A);
    1
}

// INC BC
fn inc_03(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::BC);
    2
}
// INC B
fn inc_04(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::B);
    1
}

// DEC B
fn dec_05(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::B);
    1
}

// DEC SP
fn dec_3b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::SP);
    1
}

// DEC HL
fn dec_35(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs::HL);
    3
}

// INC HL
fn inc_34(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs::HL);
    3
}

// LD B, C
fn ld_41(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs::C);
    cpu.set_r8(Regs::B, val);
    1
}

// LD B, B
fn ld_40(cpu: &mut Cpu) -> u8 {
    // do nothing
    1
}

// LD E, u8
fn ld_1e(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs::E, val);
    2
}

// LD BC, u16
fn ld_01(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::BC, val);
    3
}

// LD u16, SP
fn ld_08(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let val = cpu.get_r16(Regs16::SP);
    cpu.write_ram(addr, val.low_byte());
    cpu.write_ram(addr + 1, val.high_byte());
    5
}

// LD A, (HL+)
fn ld_2a(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs::A, val);
    cpu.set_r16(Regs16::HL, addr.wrapping_add(1));
    2
}

// LD (FF00+u8), A
fn ld_e0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let addr = 0xFF00 + offset;
    let val = cpu.get_r8(Regs::A);
    cpu.write_ram(addr, val);
    2
}

// LD HL, SP+i8
fn ld_f8(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as i8 as i16 as u16;
    let sp = cpu.get_r16(Regs16::SP);
    let set_c = check_c_carry_u8(sp.low_byte(), offset.low_byte());
    let set_h = check_h_carry_u8(sp.low_byte(), offset.low_byte());

    cpu.set_r16(Regs16::HL, offset.wrapping_add(sp));
    cpu.set_flag(Flags::Z, false);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::C, set_c);
    cpu.set_flag(Flags::H, set_h);
    3
}

pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_index = cpu.fetch();
    OPCODES[op_index as usize](cpu)
}
