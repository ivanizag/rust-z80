use super::opcode::*;
use super::environment::*;
use super::registers::*;

// Relative jumps
pub fn build_djnz() -> Opcode {
    Opcode {
        name: "DJNZ d".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let offset = env.advance_pc();
            let b = env.state.reg.get8(Reg8::B).wrapping_add(0xff /* -1 */);
            env.state.reg.set8(Reg8::B, b);
            if b != 0 {
                // Condition not met
                relative_jump(env, offset);
            }
        })
    }
}

pub fn build_jr_unconditional() -> Opcode {
    Opcode {
        name: "JR d".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let offset = env.advance_pc();
            relative_jump(env, offset);
        })
    }
}

pub fn build_jr_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("JR {}, d", name),
        action: Box::new(move |env: &mut Environment| {
            let offset = env.advance_pc();
            if env.state.reg.get_flag(flag) == value {
                relative_jump(env, offset);
            }
        })
    }
}


fn relative_jump(env: &mut Environment, offset: u8) {
    let mut pc = env.state.reg.pc();
    pc = pc.wrapping_add(offset as i8 as i16 as u16);
    env.state.reg.set_pc(pc);
}

// Absolute jumps
pub fn build_jp_unconditional() -> Opcode {
    Opcode {
        name: "JP nn".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_immediate16();
            env.state.reg.set_pc(address);
        })
    }
}

pub fn build_jp_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("JP {}, nn", name),
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_immediate16();
            if env.state.reg.get_flag(flag) == value {
                env.state.reg.set_pc(address);
            }
        })
    }
}

pub fn build_jp_hl() -> Opcode {
    Opcode {
        name: "JP HL".to_string(), // Note: it is usaully written as JP (HL)
        action: Box::new(move |env: &mut Environment| {
            // Note: no displacement added to the index
            let address = env.index_value();
            env.state.reg.set_pc(address);
        })
    }
}

// Calls to subroutine
pub fn build_call() -> Opcode {
    Opcode {
        name: "CALL nn".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_immediate16();
            env.subroutine_call(address);
        })
    }
}

pub fn build_call_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("CALL {}, nn", name),
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_immediate16();
            if env.state.reg.get_flag(flag) == value {
                env.subroutine_call(address);
            }
        })
    }
}

pub fn build_rst(d: u8) -> Opcode {
    Opcode {
        name: format!("RST {:02x}h", d),
        action: Box::new(move |env: &mut Environment| {
            let address = d as u16;
            env.subroutine_call(address);
        })
    }
}

// Returns

pub fn build_ret() -> Opcode {
    Opcode {
        name: "RET".to_string(),
        action: Box::new(move |env: &mut Environment| {
            env.subroutine_return();
        })
    }
}

pub fn build_reti() -> Opcode {
    Opcode {
        name: "RETI".to_string(),
        action: Box::new(move |env: &mut Environment| {
            env.subroutine_return();
        })
    }
}

pub fn build_retn() -> Opcode {
    Opcode {
        name: "RETN".to_string(),
        action: Box::new(move |env: &mut Environment| {
            env.subroutine_return();
            env.state.reg.end_nmi();
        })
    }
}

pub fn build_ret_eq((flag, value, name): (Flag, bool, &str)) -> Opcode {
    Opcode {
        name: format!("RET {}", name),
        action: Box::new(move |env: &mut Environment| {
            if env.state.reg.get_flag(flag) == value {
                env.subroutine_return();
            }
        })
    }
}
