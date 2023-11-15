#[cfg(test)]
mod tests {
    use super::super::cpu::Register;

    #[test]
    fn register_get_value_should_be_high_and_low() {
        let mut reg = Register::new();
        reg.set_low(10);
        assert_eq!(reg.get_value(), 10);
    }

    #[test]
    fn register_set_low_value() {
        let mut reg = Register::new();
        reg.set_low(10);
        assert_eq!(reg.get_low(), 10);
    }


    #[test]
    fn register_set_high_value() {
        let mut reg = Register::new();
        reg.set_high(10);
        assert_eq!(reg.get_high(), 10);
    }

    #[test]
    fn set_zero_flag() {

    }
}
