#[cfg(test)]
mod tests {
    use crate::architecture::cpu::Flag;

    use super::super::cpu::{Register, RegBank, RegPos};

    #[test]
    fn register_get_value_should_be_high_and_low() {
        let mut reg = Register::new();
        reg.set_part(RegPos::LOW, 10);
        assert_eq!(reg.get_value(), 10);
    }

    #[test]
    fn register_set_low_value() {
        let mut reg = Register::new();
        reg.set_part(RegPos::LOW, 10);
        assert_eq!(reg.get_part(RegPos::LOW), 10);
    }


    #[test]
    fn register_set_high_value() {
        let mut reg = Register::new();
        reg.set_part(RegPos::HIGH, 10);
        assert_eq!(reg.get_part(RegPos::HIGH), 10);
    }

    #[test]
    fn set_zero_flag() {
        let mut bank = RegBank::new();
        bank.set_flag(Flag::ZERO);
        assert_eq!(bank.get_flag(Flag::ZERO), 1)
    }
}
