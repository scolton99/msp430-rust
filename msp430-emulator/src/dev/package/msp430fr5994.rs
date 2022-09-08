use elf::File;
use crate::dev::package::{*};

pub struct MSP430FR5994 {

}

impl Default for MSP430FR5994 {
    fn default() -> Self {
        MSP430FR5994 {

        }
    }
}

impl Package for MSP430FR5994 {
    fn load_prog(_: File) {
        todo!()
    }

    fn get_cpu() -> Box<dyn CPU> {
        todo!()
    }
}