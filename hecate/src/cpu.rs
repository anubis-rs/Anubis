use registers::*;
use memory::*;

pub struct Cpu {
    pub registers: CpuRegisters,
    pub memory: Memory,
}