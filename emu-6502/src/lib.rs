pub mod cpu_6502 {

    use std::time::{Instant};

    pub struct CPU {
        pc: i16, sp: u8, a: u8, x: u8, y: u8, status: u8, cps: u64
    }

    impl CPU {

        pub fn reset() -> CPU {
            CPU {
                pc: 0, sp: 0, a: 0, x: 0, y: 0, status: 0, cps: 10
            }
        }
    }

    pub fn power_up() {

        let cpu : CPU = CPU::reset();
        let cpu_ref: &CPU = &cpu;

        // Later this will be forever
        for x in 0..10 {
            let now = Instant::now();
            do_tick(&cpu_ref);
        }

    }

    pub fn do_tick(cpu: &CPU) {
        println!("{}", cpu.pc);
    }
}