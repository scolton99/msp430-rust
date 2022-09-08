use crate::util::BitFieldError::ValueTooLarge;

pub enum BitFieldError {
    ValueTooLarge,
    Overflow
}

pub struct BitField {
    size: u8,
    value: u128,
}

impl BitField {
    fn validate_value(size: u8, value: u128) -> Result<(), BitFieldError> {
        let mask = if size > 128 {
            return Err(BitFieldError::Overflow);
        } else if size == 128 {
            return Ok(())
        } else {
            2u128.pow(size as u32) - 1
        };

        match mask & value {
            0u128 => Ok(()),
            _     => Err(ValueTooLarge)
        }
    }

    pub fn empty(size: u8) -> Self {
        BitField { size, value: 0u128 }
    }

    pub fn new(size: u8, value: u128) -> Self {
        BitField { size, value }
    }

    pub fn set_value(&mut self, value: u128) {
        self.value = value;
    }

    pub fn get_u8(&self) -> Result<u8, BitFieldError> {
        match Self::validate_value(8, self.value) {
            Ok(_) => Ok(self.value as u8),
            err => err
        }
    }

    pub fn get_u16(&self) -> Result<u16, BitFieldError> {
        match Self::validate_value(16, self.value) {
            Ok(_) => Ok(self.value as u16),
            err => err
        }
    }

    pub fn get_u32(&self) -> Result<u32, BitFieldError> {
        match Self::validate_value(32, self.value) {
            Ok(_) => Ok(self.value as u32),
            err => err
        }
    }

    pub fn get_u64(&self) -> Result<u64, BitFieldError> {
        match Self::validate_value(64, self.value) {
            Ok(_) => Ok(self.value as u64),
            err => err
        }
    }

    pub fn get_u128(&self) -> Result<u128, BitFieldError> {
        Ok(self.value)
    }
}

impl Default for BitField {
    fn default() -> Self {
        BitField { size: 8u8, value: 0 }
    }
}
