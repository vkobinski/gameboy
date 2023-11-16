use super::{super::architecture::cpu, cpu::{Reg8, Reg16}};

impl cpu::Cpu {

   pub fn adc_a_r8(&mut self, reg: Reg8) {

       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let r_val = self.bank.get_8_bit_reg(&reg);

       let sum = a_val.wrapping_add(r_val) + self.bank.get_flag(cpu::Flag::CARRY);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_8_sum_flags(&reg, a_val, r_val);

       self.tick(1);
   }

   pub fn adc_a_hl(&mut self) {
       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let r_val = self.bus.mem.read_byte(self.bank.get_16_bit_reg(&cpu::Reg16::HL));

       let sum = a_val.wrapping_add(r_val) + self.bank.get_flag(cpu::Flag::CARRY);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_8_sum_flags(&Reg8::A, a_val, r_val);

       self.tick(2);
   }

   pub fn adc_a_n8(&mut self, val: u8) {

       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let sum = a_val.wrapping_add(val) + self.bank.get_flag(cpu::Flag::CARRY);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_8_sum_flags(&Reg8::A, a_val, val);

       self.tick(2);

   }

   pub fn add_a_r8(&mut self, reg: Reg8) {

       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let r_val = self.bank.get_8_bit_reg(&reg);

       let sum = a_val.wrapping_add(r_val);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_8_sum_flags(&reg, a_val,r_val);

       self.tick(1);
   }

    pub fn add_a_hl(&mut self) {
       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let r_val = self.bus.mem.read_byte(self.bank.get_16_bit_reg(&cpu::Reg16::HL));

       let sum = a_val.wrapping_add(r_val);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_8_sum_flags(&Reg8::A, a_val,r_val);

       self.tick(2);
   }

   pub fn add_a_n8(&mut self, val: u8) {

       let a_val = self.bank.get_8_bit_reg(&Reg8::A);
       let sum = a_val.wrapping_add(val);

       self.bank.set_8_bit_reg(&Reg8::A, sum);
       self.bank.check_all_8_sum_flags(&Reg8::A, a_val,val);

       self.tick(2);
   }

   pub fn add_hl_r16(&mut self, val: &Reg16) {
       let a_val = self.bank.get_16_bit_reg(&Reg16::HL);
       let r_val = self.bank.get_16_bit_reg(val);
       let sum = a_val.wrapping_add(r_val);

       self.bank.set_16_bit_reg(&Reg16::HL, sum);
       self.bank.check_all_16_sum_flags(&Reg16::HL, a_val,r_val);

       self.tick(2);
   }
}
