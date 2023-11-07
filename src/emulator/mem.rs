use crate::consts;

#[derive(Clone)]
pub struct Mem {
    ax: i32,
    bx: i32,
    cx: i32,
    dx: i32,
    ex: i32,
    fx: i32,
}

impl std::fmt::Display for Mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[AX: {}, BX: {}, CX: {}, DX: {}, EX: {}, FX: {}]",
            self.ax, self.bx, self.cx, self.dx, self.ex, self.fx
        )
    }
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,
            ex: 0,
            fx: 0,
        }
    }

    pub fn set_reg(&mut self, reg: u8, val: i32) -> Result<(), ()> {
        match reg {
            consts::AX => {
                self.ax = val;
            }
            consts::BX => {
                self.bx = val;
            }
            consts::CX => {
                self.cx = val;
            }
            consts::DX => {
                self.dx = val;
            }
            consts::EX => {
                self.ex = val;
            }
            consts::FX => {
                self.fx = val;
            }
            _ => {
                return Err(());
            }
        }

        Ok(())
    }

    pub fn get_reg(&self, reg: u8) -> Result<i32, ()> {
        match reg {
            consts::AX => Ok(self.ax),
            consts::BX => Ok(self.bx),
            consts::CX => Ok(self.cx),
            consts::DX => Ok(self.dx),
            consts::EX => Ok(self.ex),
            consts::FX => Ok(self.fx),
            _ => Err(()),
        }
    }
}
