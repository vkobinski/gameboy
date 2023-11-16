use super::{super::architecture::cpu, cpu::Reg8};

impl cpu::Cpu {

   pub fn adc_a_r8(&mut self, reg: Reg8) {

       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let r_val = self.bank.get_8_bit_reg(&reg);

       let sum = a_val.wrapping_add(r_val) + self.bank.get_flag(cpu::Flag::CARRY);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_sum_flags(&reg, r_val);

       self.tick(1);
   }

   pub fn adc_a_hl(&mut self) {
       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let r_val = self.bus.mem.read_byte(self.bank.get_16_bit_reg(&cpu::Reg16::HL));

       let sum = a_val.wrapping_add(r_val) + self.bank.get_flag(cpu::Flag::CARRY);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_sum_flags(&Reg8::A, r_val);

       self.tick(2);
   }

   pub fn adc_a_n8(&mut self, val: u8) {

       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let sum = a_val.wrapping_add(val) + self.bank.get_flag(cpu::Flag::CARRY);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_sum_flags(&Reg8::A, val);

       self.tick(2);

   }
}
