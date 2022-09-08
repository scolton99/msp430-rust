use elf as libelf;
use crate::dev::cpu::CPU;
use crate::dev::package::msp430fr5994::MSP430FR5994;

mod msp430fr5994;

pub trait Package {
    fn load_prog(_: libelf::File);
    fn get_cpu() -> Box<dyn CPU>;
}

pub fn get_instance(package_name: &str) -> Box<dyn Package> {
    match package_name {
        "msp430fr4994" => MSP430FR5994::default(),
        _ => panic!(format!("Package {} does not exist", package_name))
    }
}