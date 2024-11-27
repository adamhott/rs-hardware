use crate::register_bank::{RegisterBank, Registers, BANK0};

pub struct ICM42688<BUS> {
    pub(crate) bus: BUS,
    current_bank: RegisterBank,
}

#[derive(Debug, defmt::Format)]
pub struct BankSelectionError;

impl core::fmt::Display for BankSelectionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bank selection error")
    }
}

// Only available in nightly
// impl core::error::Error for BankSelectionError {}

pub trait BankSelectable {
    fn set_bank(&mut self, bank: RegisterBank) -> Result<(), BankSelectionError>;
}

impl<BUS> ICM42688<BUS> {
    pub fn new(bus: BUS) -> Self {
        ICM42688 {
            bus,
            current_bank: BANK0,
        }
    }

    pub fn set_bank(&mut self, bank: RegisterBank) {
        self.current_bank = bank;
    }

    pub fn get_bank(&self) -> RegisterBank {
        self.current_bank
    }

    pub fn bank<const BANK: RegisterBank>(&mut self) -> Registers<BUS, BANK> {
        if self.current_bank != BANK {
            panic!("Bank mismatch")
        }
        Registers::new(&mut self.bus)
    }

    /// Get a reference to the bus
    pub fn bus(&mut self) -> &mut BUS {
        &mut self.bus
    }

    /// Release the bus from the ICM42688 instance
    pub fn release(self) -> BUS {
        self.bus
    }
}
