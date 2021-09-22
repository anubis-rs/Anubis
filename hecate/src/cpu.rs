use registers::CpuRegisters;
use memory::Memory;

pub struct Cpu {
    pub registers: CpuRegisters,
    pub memory: Memory,
}