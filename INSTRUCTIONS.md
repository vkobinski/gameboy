# Instruction status

Every opcode is now wired into `parse_instruction` (`src/architecture/instructions_set.rs`).
Implemented ones call their real function (in `instructions_set.rs`). Everything else calls
a placeholder in `src/architecture/instructions_missing.rs` — each just calls `pass()` and
does nothing. Implement a placeholder's body, then delete it from that file and move the
real function into `instructions_set.rs` (or wherever fits your pattern) when you're done.

The CB-prefixed table (`0xCB` + a second byte, 256 more opcodes — rotates/shifts/`BIT`/`SET`/`RES`)
is collapsed into a single `cb_prefixed()` placeholder for now; it isn't broken out opcode-by-opcode
yet. `0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD` don't exist on real
hardware and are left panicking on purpose.

Legend: ✅ implemented · ⬜ placeholder only

## 0x0_

| Op | Mnemonic | Status | Function | Description |
|----|----------|--------|----------|-------------|
| 00 | NOP | ✅ | `nop` | Does nothing for one cycle. |
| 01 | LD BC,d16 | ⬜ | `ld_bc_d16` | Load the 16-bit immediate value into `BC`. |
| 02 | LD (BC),A | ⬜ | `ld_bc_a` | Store `A` into the memory byte addressed by `BC`. |
| 03 | INC BC | ⬜ | `inc_bc` | Increment `BC` by 1. No flags affected. |
| 04 | INC B | ⬜ | `inc_b` | Increment `B` by 1. Sets `Z`,`H`; clears `N`. |
| 05 | DEC B | ⬜ | `dec_b` | Decrement `B` by 1. Sets `Z`,`H`; sets `N`. |
| 06 | LD B,d8 | ⬜ | `ld_b_d8` | Load the 8-bit immediate into `B`. |
| 07 | RLCA | ⬜ | `rlca` | Rotate `A` left circularly; bit 7 moves into `Carry` and into bit 0. |
| 08 | LD (a16),SP | ⬜ | `ld_a16_sp` | Store `SP` (2 bytes, little-endian) at the given 16-bit memory address. |
| 09 | ADD HL,BC | ✅ | `add_hl_r16` | Add `BC` to `HL`. |
| 0A | LD A,(BC) | ⬜ | `ld_a_bc` | Load `A` from the memory byte addressed by `BC`. |
| 0B | DEC BC | ⬜ | `dec_bc` | Decrement `BC` by 1. No flags affected. |
| 0C | INC C | ⬜ | `inc_c` | Increment `C` by 1. |
| 0D | DEC C | ⬜ | `dec_c` | Decrement `C` by 1. |
| 0E | LD C,d8 | ⬜ | `ld_c_d8` | Load the 8-bit immediate into `C`. |
| 0F | RRCA | ⬜ | `rrca` | Rotate `A` right circularly; bit 0 moves into `Carry` and into bit 7. |

## 0x1_

| Op | Mnemonic | Status | Function | Description |
|----|----------|--------|----------|-------------|
| 10 | STOP | ⬜ | `stop` | Halt CPU and display until a button is pressed (low-power mode). |
| 11 | LD DE,d16 | ⬜ | `ld_de_d16` | Load the 16-bit immediate value into `DE`. |
| 12 | LD (DE),A | ⬜ | `ld_de_a` | Store `A` into the memory byte addressed by `DE`. |
| 13 | INC DE | ⬜ | `inc_de` | Increment `DE` by 1. |
| 14 | INC D | ⬜ | `inc_d` | Increment `D` by 1. |
| 15 | DEC D | ⬜ | `dec_d` | Decrement `D` by 1. |
| 16 | LD D,d8 | ⬜ | `ld_d_d8` | Load the 8-bit immediate into `D`. |
| 17 | RLA | ⬜ | `rla` | Rotate `A` left through `Carry` (9-bit rotation using the carry flag). |
| 18 | JR r8 | ⬜ | `jr_r8` | Unconditional relative jump by a signed 8-bit offset. |
| 19 | ADD HL,DE | ✅ | `add_hl_r16` | Add `DE` to `HL`. |
| 1A | LD A,(DE) | ⬜ | `ld_a_de` | Load `A` from the memory byte addressed by `DE`. |
| 1B | DEC DE | ⬜ | `dec_de` | Decrement `DE` by 1. |
| 1C | INC E | ⬜ | `inc_e` | Increment `E` by 1. |
| 1D | DEC E | ⬜ | `dec_e` | Decrement `E` by 1. |
| 1E | LD E,d8 | ⬜ | `ld_e_d8` | Load the 8-bit immediate into `E`. |
| 1F | RRA | ⬜ | `rra` | Rotate `A` right through `Carry`. |

## 0x2_

| Op | Mnemonic | Status | Function | Description |
|----|----------|--------|----------|-------------|
| 20 | JR NZ,r8 | ⬜ | `jr_nz_r8` | Relative jump by a signed offset if `Zero` is not set. |
| 21 | LD HL,d16 | ⬜ | `ld_hl_d16` | Load the 16-bit immediate value into `HL`. |
| 22 | LD (HL+),A | ⬜ | `ld_hli_a` | Store `A` at the address in `HL`, then increment `HL`. |
| 23 | INC HL | ⬜ | `inc_hl` | Increment `HL` by 1. |
| 24 | INC H | ⬜ | `inc_h` | Increment `H` by 1. |
| 25 | DEC H | ⬜ | `dec_h` | Decrement `H` by 1. |
| 26 | LD H,d8 | ⬜ | `ld_h_d8` | Load the 8-bit immediate into `H`. |
| 27 | DAA | ⬜ | `daa` | Adjust `A` to a valid packed-BCD result after an add/subtract. |
| 28 | JR Z,r8 | ⬜ | `jr_z_r8` | Relative jump by a signed offset if `Zero` is set. |
| 29 | ADD HL,HL | ✅ | `add_hl_r16` | Add `HL` to itself (i.e. double `HL`). |
| 2A | LD A,(HL+) | ⬜ | `ld_a_hli` | Load `A` from the address in `HL`, then increment `HL`. |
| 2B | DEC HL | ⬜ | `dec_hl` | Decrement `HL` by 1. |
| 2C | INC L | ⬜ | `inc_l` | Increment `L` by 1. |
| 2D | DEC L | ⬜ | `dec_l` | Decrement `L` by 1. |
| 2E | LD L,d8 | ⬜ | `ld_l_d8` | Load the 8-bit immediate into `L`. |
| 2F | CPL | ⬜ | `cpl` | Flip every bit of `A` (one's complement). Sets `N`,`H`. |

## 0x3_

| Op | Mnemonic | Status | Function | Description |
|----|----------|--------|----------|-------------|
| 30 | JR NC,r8 | ⬜ | `jr_nc_r8` | Relative jump by a signed offset if `Carry` is not set. |
| 31 | LD SP,d16 | ⬜ | `ld_sp_d16` | Load the 16-bit immediate value into `SP`. |
| 32 | LD (HL-),A | ⬜ | `ld_hld_a` | Store `A` at the address in `HL`, then decrement `HL`. |
| 33 | INC SP | ⬜ | `inc_sp` | Increment `SP` by 1. |
| 34 | INC (HL) | ⬜ | `inc_hl_ind` | Increment the byte at the memory address in `HL`. |
| 35 | DEC (HL) | ⬜ | `dec_hl_ind` | Decrement the byte at the memory address in `HL`. |
| 36 | LD (HL),d8 | ⬜ | `ld_hl_ind_d8` | Store the 8-bit immediate at the memory address in `HL`. |
| 37 | SCF | ⬜ | `scf` | Set the `Carry` flag. Clears `N`,`H`. |
| 38 | JR C,r8 | ⬜ | `jr_c_r8` | Relative jump by a signed offset if `Carry` is set. |
| 39 | ADD HL,SP | ✅ | `add_hl_sp` | Add `SP` to `HL`. |
| 3A | LD A,(HL-) | ⬜ | `ld_a_hld` | Load `A` from the address in `HL`, then decrement `HL`. |
| 3B | DEC SP | ⬜ | `dec_sp` | Decrement `SP` by 1. |
| 3C | INC A | ⬜ | `inc_a` | Increment `A` by 1. |
| 3D | DEC A | ⬜ | `dec_a` | Decrement `A` by 1. |
| 3E | LD A,d8 | ⬜ | `ld_a_d8` | Load the 8-bit immediate into `A`. |
| 3F | CCF | ⬜ | `ccf` | Flip the `Carry` flag. Clears `N`,`H`. |

## 0x4_ – 0x7_ (LD r8,r8 / LD r8,(HL) / LD (HL),r8 grid)

All ⬜ except `76` (`HALT`, also ⬜). Every register-to-register combination copies
the source register's value into the destination register and changes no flags — e.g.
`ld_b_c` is "copy `C` into `B`". Reads from `(HL)` (`ld_<dst>_hl` / `ld_a_hl_ind`) load
the destination from the memory byte addressed by `HL`; writes to `(HL)`
(`ld_hl_ind_<src>`) store the source register into that memory byte. `0x76` (`HALT`)
stops executing instructions until an interrupt occurs, instead of being a load. 63
loads + `HALT` = 64 opcodes, none implemented yet.

## 0x8_ (ADD A,* / ADC A,*)

All ✅.

| Op | Mnemonic | Description |
|----|----------|-------------|
| 80-87 | ADD A,{B,C,D,E,H,L,(HL),A} | Add the register (or the byte at `HL`) to `A`. |
| C6 | ADD A,d8 | Add the 8-bit immediate to `A`. |
| 88-8F | ADC A,{B,C,D,E,H,L,(HL),A} | Add the register (or byte at `HL`) plus `Carry` to `A`. |
| CE | ADC A,d8 | Add the 8-bit immediate plus `Carry` to `A`. |

## 0x9_

| Op | Mnemonic | Status | Description |
|----|----------|--------|-------------|
| 90-97 | SUB A,{B,C,D,E,H,L,(HL),A} | ⬜ | Subtract the register (or byte at `HL`) from `A`. |
| D6 | SUB A,d8 | ⬜ | Subtract the 8-bit immediate from `A`. |
| 98-9F | SBC A,{B,C,D,E,H,L,(HL),A} | ⬜ | Subtract the register (or byte at `HL`) and `Carry` from `A`. |
| DE | SBC A,d8 | ⬜ | Subtract the 8-bit immediate and `Carry` from `A`. |

Functions: `sub_a_b` … `sub_a_hl`, `sub_a_a`, `sub_a_d8`; `sbc_a_b` … `sbc_a_hl`, `sbc_a_a`, `sbc_a_d8`.

## 0xA_

| Op | Mnemonic | Status | Description |
|----|----------|--------|-------------|
| A0-A7 | AND A,{B,C,D,E,H,L,(HL),A} | ✅ | Bitwise AND the register (or byte at `HL`) into `A`. |
| E6 | AND A,d8 | ✅ | Bitwise AND the 8-bit immediate into `A`. |
| A8-AF | XOR A,{B,C,D,E,H,L,(HL),A} | ⬜ | Bitwise XOR the register (or byte at `HL`) into `A`. |
| EE | XOR A,d8 | ⬜ | Bitwise XOR the 8-bit immediate into `A`. |

Functions: `and_a_r8`, `and_a_hl`, `and_a_n8`; `xor_a_b` … `xor_a_hl`, `xor_a_a`, `xor_a_d8`.

## 0xB_

| Op | Mnemonic | Status | Description |
|----|----------|--------|-------------|
| B0-B7 | OR A,{B,C,D,E,H,L,(HL),A} | ⬜ | Bitwise OR the register (or byte at `HL`) into `A`. |
| F6 | OR A,d8 | ⬜ | Bitwise OR the 8-bit immediate into `A`. |
| B8-BF | CP A,{B,C,D,E,H,L,(HL),A} | ⬜ | Compare `A` with the register (or byte at `HL`): subtracts without storing the result, only flags change. |
| FE | CP A,d8 | ⬜ | Compare `A` with the 8-bit immediate the same way. |

Functions: `or_a_b` … `or_a_hl`, `or_a_a`, `or_a_d8`; `cp_a_b` … `cp_a_hl`, `cp_a_a`, `cp_a_d8`.

## 0xC_ – 0xF_ (control flow, stack, misc)

| Op | Mnemonic | Status | Function | Description |
|----|----------|--------|----------|-------------|
| C0 | RET NZ | ⬜ | `ret_nz` | Pop the return address and jump to it, if `Zero` is not set. |
| C1 | POP BC | ⬜ | `pop_bc` | Pop 2 bytes off the stack into `BC`. |
| C2 | JP NZ,a16 | ⬜ | `jp_nz_a16` | Jump to the 16-bit address, if `Zero` is not set. |
| C3 | JP a16 | ⬜ | `jp_a16` | Unconditional absolute jump to the 16-bit address. |
| C4 | CALL NZ,a16 | ⬜ | `call_nz_a16` | Push the return address and jump to the 16-bit address, if `Zero` is not set. |
| C5 | PUSH BC | ⬜ | `push_bc` | Push `BC` onto the stack. |
| C6 | ADD A,d8 | ✅ | `add_a_n8` | See 0x8_ table above. |
| C7 | RST 00H | ⬜ | `rst_00` | Push the return address and jump to `0x0000`. |
| C8 | RET Z | ⬜ | `ret_z` | Pop the return address and jump to it, if `Zero` is set. |
| C9 | RET | ⬜ | `ret` | Unconditional return: pop the return address and jump to it. |
| CA | JP Z,a16 | ⬜ | `jp_z_a16` | Jump to the 16-bit address, if `Zero` is set. |
| CB | PREFIX CB | ⬜ | `cb_prefixed` | Fetch a second byte and execute it as a CB-table opcode (rotates/shifts/`BIT`/`SET`/`RES`). Whole 256-opcode table collapsed here for now. |
| CC | CALL Z,a16 | ⬜ | `call_z_a16` | Push the return address and jump, if `Zero` is set. |
| CD | CALL a16 | ⬜ | `call_a16` | Unconditional call: push the return address, jump to the 16-bit address. |
| CE | ADC A,d8 | ✅ | `adc_a_n8` | See 0x8_ table above. |
| CF | RST 08H | ⬜ | `rst_08` | Push the return address and jump to `0x0008`. |
| D0 | RET NC | ⬜ | `ret_nc` | Return, if `Carry` is not set. |
| D1 | POP DE | ⬜ | `pop_de` | Pop 2 bytes off the stack into `DE`. |
| D2 | JP NC,a16 | ⬜ | `jp_nc_a16` | Jump to the 16-bit address, if `Carry` is not set. |
| D3 | — | — | — | Unused opcode; doesn't exist on real hardware. |
| D4 | CALL NC,a16 | ⬜ | `call_nc_a16` | Call, if `Carry` is not set. |
| D5 | PUSH DE | ⬜ | `push_de` | Push `DE` onto the stack. |
| D6 | SUB A,d8 | ⬜ | `sub_a_d8` | See 0x9_ table above. |
| D7 | RST 10H | ⬜ | `rst_10` | Push the return address and jump to `0x0010`. |
| D8 | RET C | ⬜ | `ret_c` | Return, if `Carry` is set. |
| D9 | RETI | ⬜ | `reti` | Return from subroutine and re-enable interrupts. |
| DA | JP C,a16 | ⬜ | `jp_c_a16` | Jump to the 16-bit address, if `Carry` is set. |
| DB | — | — | — | Unused opcode; doesn't exist on real hardware. |
| DC | CALL C,a16 | ⬜ | `call_c_a16` | Call, if `Carry` is set. |
| DD | — | — | — | Unused opcode; doesn't exist on real hardware. |
| DE | SBC A,d8 | ⬜ | `sbc_a_d8` | See 0x9_ table above. |
| DF | RST 18H | ⬜ | `rst_18` | Push the return address and jump to `0x0018`. |
| E0 | LDH (a8),A | ⬜ | `ldh_a8_a` | Store `A` into memory address `0xFF00 + a8` (I/O/high RAM). |
| E1 | POP HL | ⬜ | `pop_hl` | Pop 2 bytes off the stack into `HL`. |
| E2 | LD (C),A | ⬜ | `ld_c_ind_a` | Store `A` into memory address `0xFF00 + C`. |
| E3 | — | — | — | Unused opcode; doesn't exist on real hardware. |
| E4 | — | — | — | Unused opcode; doesn't exist on real hardware. |
| E5 | PUSH HL | ⬜ | `push_hl` | Push `HL` onto the stack. |
| E6 | AND A,d8 | ✅ | `and_a_n8` | See 0xA_ table above. |
| E7 | RST 20H | ⬜ | `rst_20` | Push the return address and jump to `0x0020`. |
| E8 | ADD SP,r8 | ✅ | `add_sp_e8` | Add a signed 8-bit offset to `SP`. |
| E9 | JP HL | ⬜ | `jp_hl` | Jump to the address currently stored in `HL`. |
| EA | LD (a16),A | ⬜ | `ld_a16_a` | Store `A` into the given 16-bit memory address. |
| EB | — | — | — | Unused opcode; doesn't exist on real hardware. |
| EC | — | — | — | Unused opcode; doesn't exist on real hardware. |
| ED | — | — | — | Unused opcode; doesn't exist on real hardware. |
| EE | XOR A,d8 | ⬜ | `xor_a_d8` | See 0xA_ table above. |
| EF | RST 28H | ⬜ | `rst_28` | Push the return address and jump to `0x0028`. |
| F0 | LDH A,(a8) | ⬜ | `ldh_a_a8` | Load `A` from memory address `0xFF00 + a8`. |
| F1 | POP AF | ⬜ | `pop_af` | Pop 2 bytes off the stack into `AF` (including flags). |
| F2 | LD A,(C) | ⬜ | `ld_a_c_ind` | Load `A` from memory address `0xFF00 + C`. |
| F3 | DI | ⬜ | `di` | Disable interrupts. |
| F4 | — | — | — | Unused opcode; doesn't exist on real hardware. |
| F5 | PUSH AF | ⬜ | `push_af` | Push `AF` (including flags) onto the stack. |
| F6 | OR A,d8 | ⬜ | `or_a_d8` | See 0xB_ table above. |
| F7 | RST 30H | ⬜ | `rst_30` | Push the return address and jump to `0x0030`. |
| F8 | LD HL,SP+r8 | ⬜ | `ld_hl_sp_r8` | Load `HL` with `SP` plus a signed 8-bit offset. |
| F9 | LD SP,HL | ⬜ | `ld_sp_hl` | Load `SP` with the current value of `HL`. |
| FA | LD A,(a16) | ⬜ | `ld_a_a16` | Load `A` from the given 16-bit memory address. |
| FB | EI | ⬜ | `ei` | Enable interrupts (takes effect after the next instruction). |
| FC | — | — | — | Unused opcode; doesn't exist on real hardware. |
| FD | — | — | — | Unused opcode; doesn't exist on real hardware. |
| FE | CP A,d8 | ⬜ | `cp_a_d8` | See 0xB_ table above. |
| FF | RST 38H | ⬜ | `rst_38` | Push the return address and jump to `0x0038`. |

## Totals

- 245 real opcodes in the unprefixed table (256 minus 11 unused).
- 33 implemented (✅), 212 placeholders (⬜).
- Plus the entire CB-prefixed table (256 opcodes) still collapsed into one `cb_prefixed` stub.
