#!/usr/bin/env python3
"""Scans the CPU instruction source files for which opcodes are actually
implemented (vs. still a `pass();` placeholder) and renders that status into
opcode-coverage.html.

Usage:
    python3 render_instructions.py
    python3 render_instructions.py --out docs/coverage.html
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent
RUST_SOURCES = [
    REPO_ROOT / "src" / "architecture" / "instructions_set.rs",
    REPO_ROOT / "src" / "architecture" / "instructions_missing.rs",
]
TEST_SOURCES = [
    REPO_ROOT / "src" / "tests" / "instructions_test.rs",
    REPO_ROOT / "src" / "tests" / "instructions_test_grids.rs",
    REPO_ROOT / "src" / "tests" / "instructions_test_other.rs",
]

TEST_FN_RE = re.compile(r"#\[test\]\s*fn\s+([a-zA-Z0-9_]+)\s*\(\s*\)\s*\{")
CALL_RE = re.compile(r"cpu\.([a-zA-Z0-9_]+)\(")
TEST_RESULT_RE = re.compile(r"^test (\S+) \.\.\. (ok|FAILED)$")

FN_RE = re.compile(
    r"pub\s+fn\s+([a-zA-Z0-9_]+)\s*\([^)]*\)(?:\s*->\s*[^\{]+)?\s*\{"
)


def extract_function_bodies(source: str) -> dict[str, list[str]]:
    """Returns {function_name: [body_text, ...]} — a name can appear more
    than once if it's (re)defined across the two files."""
    bodies: dict[str, list[str]] = {}
    for match in FN_RE.finditer(source):
        name = match.group(1)
        start = match.end()
        depth = 1
        i = start
        while depth > 0 and i < len(source):
            if source[i] == "{":
                depth += 1
            elif source[i] == "}":
                depth -= 1
            i += 1
        body = source[start : i - 1]
        bodies.setdefault(name, []).append(body)
    return bodies


def strip_line_comments(body: str) -> str:
    return re.sub(r"//.*", "", body)


def is_placeholder_body(body: str) -> bool:
    stripped = strip_line_comments(body).strip()
    if stripped == "":
        return True
    normalized = re.sub(r"\s+", "", stripped)
    return normalized in ("pass();", "pass()")


def load_function_status() -> dict[str, bool]:
    """Returns {function_name: is_implemented}."""
    all_bodies: dict[str, list[str]] = {}
    for path in RUST_SOURCES:
        source = path.read_text()
        for name, bodies in extract_function_bodies(source).items():
            all_bodies.setdefault(name, []).extend(bodies)

    implemented: dict[str, bool] = {}
    for name, bodies in all_bodies.items():
        implemented[name] = any(not is_placeholder_body(b) for b in bodies)
    return implemented


# ---------------------------------------------------------------------------
# Opcode table — mirrors src/architecture/instructions_set.rs's dispatch and
# src/architecture/instructions_missing.rs's placeholders. Only the *shape*
# (mnemonic/function-name/description) lives here; `done` vs. `todo` status
# is resolved from the Rust source above, not hardcoded.
# ---------------------------------------------------------------------------

Entry = dict
TABLE: list[Entry | None] = [None] * 256


def set_op(op: int, mnemonic: str, fn: str | None, desc: str) -> None:
    TABLE[op] = {"op": op, "mnemonic": mnemonic, "fn": fn, "desc": desc}


def build_table() -> None:
    # ---- 0x00-0x3F: singles ----
    set_op(0x00, "NOP", "nop", "Does nothing for one cycle.")
    set_op(0x01, "LD BC,d16", "ld_bc_d16", "Load the 16-bit immediate value into BC.")
    set_op(0x02, "LD (BC),A", "ld_bc_a", "Store A into the memory byte addressed by BC.")
    set_op(0x03, "INC BC", "inc_bc", "Increment BC by 1. No flags affected.")
    set_op(0x04, "INC B", "inc_b", "Increment B by 1. Sets Z,H; clears N.")
    set_op(0x05, "DEC B", "dec_b", "Decrement B by 1. Sets Z,H; sets N.")
    set_op(0x06, "LD B,d8", "ld_b_d8", "Load the 8-bit immediate into B.")
    set_op(0x07, "RLCA", "rlca", "Rotate A left circularly; bit 7 moves into Carry and into bit 0.")
    set_op(0x08, "LD (a16),SP", "ld_a16_sp", "Store SP (2 bytes, little-endian) at the given 16-bit memory address.")
    set_op(0x09, "ADD HL,BC", "add_hl_r16", "Add BC to HL.")
    set_op(0x0A, "LD A,(BC)", "ld_a_bc", "Load A from the memory byte addressed by BC.")
    set_op(0x0B, "DEC BC", "dec_bc", "Decrement BC by 1. No flags affected.")
    set_op(0x0C, "INC C", "inc_c", "Increment C by 1.")
    set_op(0x0D, "DEC C", "dec_c", "Decrement C by 1.")
    set_op(0x0E, "LD C,d8", "ld_c_d8", "Load the 8-bit immediate into C.")
    set_op(0x0F, "RRCA", "rrca", "Rotate A right circularly; bit 0 moves into Carry and into bit 7.")

    set_op(0x10, "STOP", "stop", "Halt CPU and display until a button is pressed (low-power mode).")
    set_op(0x11, "LD DE,d16", "ld_de_d16", "Load the 16-bit immediate value into DE.")
    set_op(0x12, "LD (DE),A", "ld_de_a", "Store A into the memory byte addressed by DE.")
    set_op(0x13, "INC DE", "inc_de", "Increment DE by 1.")
    set_op(0x14, "INC D", "inc_d", "Increment D by 1.")
    set_op(0x15, "DEC D", "dec_d", "Decrement D by 1.")
    set_op(0x16, "LD D,d8", "ld_d_d8", "Load the 8-bit immediate into D.")
    set_op(0x17, "RLA", "rla", "Rotate A left through Carry (9-bit rotation using the carry flag).")
    set_op(0x18, "JR r8", "jr_r8", "Unconditional relative jump by a signed 8-bit offset.")
    set_op(0x19, "ADD HL,DE", "add_hl_r16", "Add DE to HL.")
    set_op(0x1A, "LD A,(DE)", "ld_a_de", "Load A from the memory byte addressed by DE.")
    set_op(0x1B, "DEC DE", "dec_de", "Decrement DE by 1.")
    set_op(0x1C, "INC E", "inc_e", "Increment E by 1.")
    set_op(0x1D, "DEC E", "dec_e", "Decrement E by 1.")
    set_op(0x1E, "LD E,d8", "ld_e_d8", "Load the 8-bit immediate into E.")
    set_op(0x1F, "RRA", "rra", "Rotate A right through Carry.")

    set_op(0x20, "JR NZ,r8", "jr_nz_r8", "Relative jump by a signed offset if Zero is not set.")
    set_op(0x21, "LD HL,d16", "ld_hl_d16", "Load the 16-bit immediate value into HL.")
    set_op(0x22, "LD (HL+),A", "ld_hli_a", "Store A at the address in HL, then increment HL.")
    set_op(0x23, "INC HL", "inc_hl", "Increment HL by 1.")
    set_op(0x24, "INC H", "inc_h", "Increment H by 1.")
    set_op(0x25, "DEC H", "dec_h", "Decrement H by 1.")
    set_op(0x26, "LD H,d8", "ld_h_d8", "Load the 8-bit immediate into H.")
    set_op(0x27, "DAA", "daa", "Adjust A to a valid packed-BCD result after an add/subtract.")
    set_op(0x28, "JR Z,r8", "jr_z_r8", "Relative jump by a signed offset if Zero is set.")
    set_op(0x29, "ADD HL,HL", "add_hl_r16", "Add HL to itself (double HL).")
    set_op(0x2A, "LD A,(HL+)", "ld_a_hli", "Load A from the address in HL, then increment HL.")
    set_op(0x2B, "DEC HL", "dec_hl", "Decrement HL by 1.")
    set_op(0x2C, "INC L", "inc_l", "Increment L by 1.")
    set_op(0x2D, "DEC L", "dec_l", "Decrement L by 1.")
    set_op(0x2E, "LD L,d8", "ld_l_d8", "Load the 8-bit immediate into L.")
    set_op(0x2F, "CPL", "cpl", "Flip every bit of A (one's complement). Sets N,H.")

    set_op(0x30, "JR NC,r8", "jr_nc_r8", "Relative jump by a signed offset if Carry is not set.")
    set_op(0x31, "LD SP,d16", "ld_sp_d16", "Load the 16-bit immediate value into SP.")
    set_op(0x32, "LD (HL-),A", "ld_hld_a", "Store A at the address in HL, then decrement HL.")
    set_op(0x33, "INC SP", "inc_sp", "Increment SP by 1.")
    set_op(0x34, "INC (HL)", "inc_hl_ind", "Increment the byte at the memory address in HL.")
    set_op(0x35, "DEC (HL)", "dec_hl_ind", "Decrement the byte at the memory address in HL.")
    set_op(0x36, "LD (HL),d8", "ld_hl_ind_d8", "Store the 8-bit immediate at the memory address in HL.")
    set_op(0x37, "SCF", "scf", "Set the Carry flag. Clears N,H.")
    set_op(0x38, "JR C,r8", "jr_c_r8", "Relative jump by a signed offset if Carry is set.")
    set_op(0x39, "ADD HL,SP", "add_hl_sp", "Add SP to HL.")
    set_op(0x3A, "LD A,(HL-)", "ld_a_hld", "Load A from the address in HL, then decrement HL.")
    set_op(0x3B, "DEC SP", "dec_sp", "Decrement SP by 1.")
    set_op(0x3C, "INC A", "inc_a", "Increment A by 1.")
    set_op(0x3D, "DEC A", "dec_a", "Decrement A by 1.")
    set_op(0x3E, "LD A,d8", "ld_a_d8", "Load the 8-bit immediate into A.")
    set_op(0x3F, "CCF", "ccf", "Flip the Carry flag. Clears N,H.")

    # ---- 0x40-0x7F: LD r8,r8 / LD r8,(HL) / LD (HL),r8 grid ----
    r8 = ["B", "C", "D", "E", "H", "L", "(HL)", "A"]
    for d, dst in enumerate(r8):
        for s, src in enumerate(r8):
            op = 0x40 + d * 8 + s
            if dst == "(HL)" and src == "(HL)":
                set_op(op, "HALT", "halt", "Stop executing instructions until an interrupt occurs (low-power wait state).")
                continue
            if dst != "(HL)" and src != "(HL)":
                fn = f"ld_{dst.lower()}_{src.lower()}"
                desc = f"Copy {src} into {dst}."
                mnemonic = f"LD {dst},{src}"
            elif src == "(HL)":
                fn = "ld_a_hl_ind" if dst == "A" else f"ld_{dst.lower()}_hl"
                desc = f"Load {dst} from the byte at the memory address in HL."
                mnemonic = f"LD {dst},(HL)"
            else:
                fn = f"ld_hl_ind_{src.lower()}"
                desc = f"Store {src} into the byte at the memory address in HL."
                mnemonic = f"LD (HL),{src}"
            set_op(op, mnemonic, fn, desc)

    # ---- 0x80-0xBF: ALU families (+ their d8 immediates elsewhere) ----
    alu_families = [
        {"base": 0x80, "mnemonic": "ADD", "fn": "add_a", "mode": "shared",
         "verb": lambda o: f"Add {o} to A.", "imm": 0xC6, "imm_verb": "Add the 8-bit immediate to A."},
        {"base": 0x88, "mnemonic": "ADC", "fn": "adc_a", "mode": "shared",
         "verb": lambda o: f"Add {o} plus Carry to A.", "imm": 0xCE, "imm_verb": "Add the 8-bit immediate plus Carry to A."},
        {"base": 0x90, "mnemonic": "SUB", "fn": "sub_a", "mode": "distinct",
         "verb": lambda o: f"Subtract {o} from A.", "imm": 0xD6, "imm_verb": "Subtract the 8-bit immediate from A."},
        {"base": 0x98, "mnemonic": "SBC", "fn": "sbc_a", "mode": "distinct",
         "verb": lambda o: f"Subtract {o} and Carry from A.", "imm": 0xDE, "imm_verb": "Subtract the 8-bit immediate and Carry from A."},
        {"base": 0xA0, "mnemonic": "AND", "fn": "and_a", "mode": "shared",
         "verb": lambda o: f"Bitwise AND {o} into A.", "imm": 0xE6, "imm_verb": "Bitwise AND the 8-bit immediate into A."},
        {"base": 0xA8, "mnemonic": "XOR", "fn": "xor_a", "mode": "distinct",
         "verb": lambda o: f"Bitwise XOR {o} into A.", "imm": 0xEE, "imm_verb": "Bitwise XOR the 8-bit immediate into A."},
        {"base": 0xB0, "mnemonic": "OR", "fn": "or_a", "mode": "distinct",
         "verb": lambda o: f"Bitwise OR {o} into A.", "imm": 0xF6, "imm_verb": "Bitwise OR the 8-bit immediate into A."},
        {"base": 0xB8, "mnemonic": "CP", "fn": "cp_a", "mode": "distinct",
         "verb": lambda o: f"Compare A with {o}: subtract without storing the result — only flags change.",
         "imm": 0xFE, "imm_verb": "Compare A with the 8-bit immediate the same way."},
    ]

    for fam in alu_families:
        for i, reg in enumerate(r8):
            op = fam["base"] + i
            operand_text = "the byte at the memory address in HL" if reg == "(HL)" else reg
            desc = fam["verb"](operand_text)
            if fam["mode"] == "shared":
                fn = f"{fam['fn']}_hl" if reg == "(HL)" else f"{fam['fn']}_r8"
            else:
                fn = f"{fam['fn']}_hl" if reg == "(HL)" else f"{fam['fn']}_{reg.lower()}"
            set_op(op, f"{fam['mnemonic']} A,{reg}", fn, desc)
        imm_fn = f"{fam['fn']}_n8" if fam["mode"] == "shared" else f"{fam['fn']}_d8"
        set_op(fam["imm"], f"{fam['mnemonic']} A,d8", imm_fn, fam["imm_verb"])

    # ---- 0xC0-0xFF: control flow, stack, misc ----
    set_op(0xC0, "RET NZ", "ret_nz", "Pop the return address and jump to it, if Zero is not set.")
    set_op(0xC1, "POP BC", "pop_bc", "Pop 2 bytes off the stack into BC.")
    set_op(0xC2, "JP NZ,a16", "jp_nz_a16", "Jump to the 16-bit address, if Zero is not set.")
    set_op(0xC3, "JP a16", "jp_a16", "Unconditional absolute jump to the 16-bit address.")
    set_op(0xC4, "CALL NZ,a16", "call_nz_a16", "Push the return address and jump, if Zero is not set.")
    set_op(0xC5, "PUSH BC", "push_bc", "Push BC onto the stack.")
    set_op(0xC7, "RST 00H", "rst_00", "Push the return address and jump to 0x0000.")
    set_op(0xC8, "RET Z", "ret_z", "Pop the return address and jump to it, if Zero is set.")
    set_op(0xC9, "RET", "ret", "Unconditional return: pop the return address and jump to it.")
    set_op(0xCA, "JP Z,a16", "jp_z_a16", "Jump to the 16-bit address, if Zero is set.")
    set_op(0xCB, "PREFIX CB", "cb_prefixed", "Fetch a second byte and execute it as a CB-table opcode (rotates/shifts/BIT/SET/RES). Whole 256-opcode table collapsed here for now.")
    set_op(0xCC, "CALL Z,a16", "call_z_a16", "Push the return address and jump, if Zero is set.")
    set_op(0xCD, "CALL a16", "call_a16", "Unconditional call: push the return address, jump to the 16-bit address.")
    set_op(0xCF, "RST 08H", "rst_08", "Push the return address and jump to 0x0008.")

    set_op(0xD0, "RET NC", "ret_nc", "Return, if Carry is not set.")
    set_op(0xD1, "POP DE", "pop_de", "Pop 2 bytes off the stack into DE.")
    set_op(0xD2, "JP NC,a16", "jp_nc_a16", "Jump to the 16-bit address, if Carry is not set.")
    set_op(0xD4, "CALL NC,a16", "call_nc_a16", "Call, if Carry is not set.")
    set_op(0xD5, "PUSH DE", "push_de", "Push DE onto the stack.")
    set_op(0xD7, "RST 10H", "rst_10", "Push the return address and jump to 0x0010.")
    set_op(0xD8, "RET C", "ret_c", "Return, if Carry is set.")
    set_op(0xD9, "RETI", "reti", "Return from subroutine and re-enable interrupts.")
    set_op(0xDA, "JP C,a16", "jp_c_a16", "Jump to the 16-bit address, if Carry is set.")
    set_op(0xDC, "CALL C,a16", "call_c_a16", "Call, if Carry is set.")
    set_op(0xDF, "RST 18H", "rst_18", "Push the return address and jump to 0x0018.")

    set_op(0xE0, "LDH (a8),A", "ldh_a8_a", "Store A into memory address 0xFF00 + a8 (I/O / high RAM).")
    set_op(0xE1, "POP HL", "pop_hl", "Pop 2 bytes off the stack into HL.")
    set_op(0xE2, "LD (C),A", "ld_c_ind_a", "Store A into memory address 0xFF00 + C.")
    set_op(0xE5, "PUSH HL", "push_hl", "Push HL onto the stack.")
    set_op(0xE7, "RST 20H", "rst_20", "Push the return address and jump to 0x0020.")
    set_op(0xE8, "ADD SP,r8", "add_sp_e8", "Add a signed 8-bit offset to SP.")
    set_op(0xE9, "JP HL", "jp_hl", "Jump to the address currently stored in HL.")
    set_op(0xEA, "LD (a16),A", "ld_a16_a", "Store A into the given 16-bit memory address.")
    set_op(0xEF, "RST 28H", "rst_28", "Push the return address and jump to 0x0028.")

    set_op(0xF0, "LDH A,(a8)", "ldh_a_a8", "Load A from memory address 0xFF00 + a8.")
    set_op(0xF1, "POP AF", "pop_af", "Pop 2 bytes off the stack into AF (including flags).")
    set_op(0xF2, "LD A,(C)", "ld_a_c_ind", "Load A from memory address 0xFF00 + C.")
    set_op(0xF3, "DI", "di", "Disable interrupts.")
    set_op(0xF5, "PUSH AF", "push_af", "Push AF (including flags) onto the stack.")
    set_op(0xF7, "RST 30H", "rst_30", "Push the return address and jump to 0x0030.")
    set_op(0xF8, "LD HL,SP+r8", "ld_hl_sp_r8", "Load HL with SP plus a signed 8-bit offset.")
    set_op(0xF9, "LD SP,HL", "ld_sp_hl", "Load SP with the current value of HL.")
    set_op(0xFA, "LD A,(a16)", "ld_a_a16", "Load A from the given 16-bit memory address.")
    set_op(0xFB, "EI", "ei", "Enable interrupts (takes effect after the next instruction).")
    set_op(0xFF, "RST 38H", "rst_38", "Push the return address and jump to 0x0038.")

    # ---- unused opcodes: don't exist on real hardware ----
    for op in (0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD):
        set_op(op, "—", None, "Unused opcode; doesn't exist on real hardware.")

    missing = [i for i, e in enumerate(TABLE) if e is None]
    if missing:
        raise RuntimeError(f"opcode table has unfilled slots: {[hex(m) for m in missing]}")


def extract_test_bodies(source: str) -> dict[str, str]:
    """Returns {test_fn_name: body_text} for every `#[test] fn name() { ... }`."""
    bodies: dict[str, str] = {}
    for match in TEST_FN_RE.finditer(source):
        name = match.group(1)
        start = match.end()
        depth = 1
        i = start
        while depth > 0 and i < len(source):
            if source[i] == "{":
                depth += 1
            elif source[i] == "}":
                depth -= 1
            i += 1
        bodies[name] = source[start : i - 1]
    return bodies


def run_cargo_tests() -> dict[str, bool]:
    """Runs `cargo test`, returns {bare_test_fn_name: passed}."""
    proc = subprocess.run(
        ["cargo", "test"],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
    )
    results: dict[str, bool] = {}
    for line in proc.stdout.splitlines():
        match = TEST_RESULT_RE.match(line)
        if not match:
            continue
        full_path, outcome = match.groups()
        bare_name = full_path.rsplit("::", 1)[-1]
        results[bare_name] = outcome == "ok"
    return results


def load_test_status() -> dict[str, bool]:
    """Returns {opcode_fn_name: all_referencing_tests_passed}, derived by
    matching each `#[test]` body's `cpu.<fn>(...)` calls against the actual
    `cargo test` pass/fail outcome for that test."""
    test_bodies: dict[str, str] = {}
    for path in TEST_SOURCES:
        test_bodies.update(extract_test_bodies(path.read_text()))

    test_outcomes = run_cargo_tests()

    fn_outcomes: dict[str, list[bool]] = {}
    for test_name, body in test_bodies.items():
        outcome = test_outcomes.get(test_name)
        if outcome is None:
            continue  # test wasn't found in the run (shouldn't normally happen)
        for fn_name in set(CALL_RE.findall(body)):
            fn_outcomes.setdefault(fn_name, []).append(outcome)

    return {fn: all(outcomes) for fn, outcomes in fn_outcomes.items()}


def resolve_statuses(implemented: dict[str, bool], tests_passed: dict[str, bool]) -> None:
    for entry in TABLE:
        if entry["fn"] is None:
            entry["status"] = "unused"
            entry["tests_passed"] = None
        else:
            entry["status"] = "done" if implemented.get(entry["fn"]) else "todo"
            entry["tests_passed"] = tests_passed.get(entry["fn"])


# ---------------------------------------------------------------------------
# Flags affected — Z N H C, in that column order. Each slot is one of:
#   "Z"/"1" = set based on the result / always set
#   "0"     = always cleared
#   "-"     = left untouched
#   "N"/"H"/"C" (in the value position) = restored verbatim, e.g. POP AF
# This is the LR35902 spec, independent of whether the current Rust body
# actually does this yet — it's what "Passed Tests?" is checking against.
# ---------------------------------------------------------------------------

FLAG_OVERRIDES = {
    "add_hl_r16": "- 0 H C",
    "add_hl_sp": "- 0 H C",
    "add_sp_e8": "0 0 H C",
    "ld_hl_sp_r8": "0 0 H C",
    "rlca": "0 0 0 C",
    "rrca": "0 0 0 C",
    "rla": "0 0 0 C",
    "rra": "0 0 0 C",
    "daa": "Z - 0 C",
    "cpl": "- 1 1 -",
    "scf": "- 0 0 1",
    "ccf": "- 0 0 C",
    "pop_af": "Z N H C",
}

SIXTEEN_BIT_INC_DEC = {
    "inc_bc", "dec_bc", "inc_de", "dec_de", "inc_hl", "dec_hl", "inc_sp", "dec_sp",
}

ADD_ADC_RE = re.compile(r"^(add_a|adc_a)(_|$)")
SUB_SBC_CP_RE = re.compile(r"^(sub_a|sbc_a|cp_a)(_|$)")


def flags_for(fn: str | None) -> str | None:
    if fn is None or fn == "cb_prefixed":
        return None  # unused slot, or varies per CB sub-opcode (not broken out)
    if fn in FLAG_OVERRIDES:
        return FLAG_OVERRIDES[fn]
    if fn in SIXTEEN_BIT_INC_DEC:
        return "- - - -"
    if fn.startswith("inc_"):
        return "Z 0 H -"
    if fn.startswith("dec_"):
        return "Z 1 H -"
    if ADD_ADC_RE.match(fn):
        return "Z 0 H C"
    if SUB_SBC_CP_RE.match(fn):
        return "Z 1 H C"
    if fn.startswith("and_a"):
        return "Z 0 1 0"
    if fn.startswith("xor_a") or fn.startswith("or_a"):
        return "Z 0 0 0"
    return "- - - -"  # every LD/PUSH/POP(bc,de,hl)/JP/JR/CALL/RET/RST/NOP/HALT/STOP/DI/EI


def resolve_flags() -> None:
    for entry in TABLE:
        entry["flags"] = flags_for(entry["fn"])


HTML_TEMPLATE = r"""<title>LR35902 Coverage</title>
<style>
  :root {
    --bg: #f5f6ef;
    --surface: #ffffff;
    --surface-2: #eceee1;
    --border: #dbdfcb;
    --text: #1b2013;
    --text-muted: #5c6350;
    --accent: #3f7a72;
    --accent-contrast: #ffffff;
    --done: #2f7d3a;
    --done-bg: #e3f0e0;
    --todo: #ad6a17;
    --todo-bg: #f6ead2;
    --unused: #8b9279;
    --fail: #b3403a;
    --fail-bg: #f7dfdb;
    --shadow: rgba(27, 32, 19, 0.08);
  }

  @media (prefers-color-scheme: dark) {
    :root:not([data-theme="light"]) {
      --bg: #11140d;
      --surface: #171b12;
      --surface-2: #1e2317;
      --border: #2b3120;
      --text: #e7ebdc;
      --text-muted: #8b9279;
      --accent: #6bbdb2;
      --accent-contrast: #0b1310;
      --done: #7cc576;
      --done-bg: #1e3320;
      --todo: #e0973c;
      --todo-bg: #3a2c14;
      --unused: #626b52;
      --fail: #e0736c;
      --fail-bg: #3a1e1c;
      --shadow: rgba(0, 0, 0, 0.4);
    }
  }

  :root[data-theme="dark"] {
    --bg: #11140d;
    --surface: #171b12;
    --surface-2: #1e2317;
    --border: #2b3120;
    --text: #e7ebdc;
    --text-muted: #8b9279;
    --accent: #6bbdb2;
    --accent-contrast: #0b1310;
    --done: #7cc576;
    --done-bg: #1e3320;
    --todo: #e0973c;
    --todo-bg: #3a2c14;
    --unused: #626b52;
    --fail: #e0736c;
    --fail-bg: #3a1e1c;
    --shadow: rgba(0, 0, 0, 0.4);
  }

  * { box-sizing: border-box; }
  html, body { margin: 0; padding: 0; }

  body {
    background: var(--bg);
    color: var(--text);
    font-family: ui-monospace, "SF Mono", "Cascadia Code", "JetBrains Mono",
      "Roboto Mono", Menlo, Consolas, monospace;
    font-size: 14px;
    line-height: 1.5;
  }

  a { color: var(--accent); }

  .page { max-width: 1080px; margin: 0 auto; padding: 0 20px 48px; }

  header.panel {
    position: sticky;
    top: 0;
    z-index: 5;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    box-shadow: 0 2px 10px var(--shadow);
    padding: 20px 20px 16px;
    margin: 0 -20px 20px;
  }

  .title-row { display: flex; align-items: baseline; justify-content: space-between; gap: 16px; flex-wrap: wrap; }

  h1 { margin: 0; font-size: 22px; font-weight: 700; letter-spacing: -0.01em; text-wrap: balance; }

  .chip-eyebrow { text-transform: uppercase; letter-spacing: 0.08em; font-size: 11px; color: var(--text-muted); }

  .subtitle { margin: 4px 0 0; color: var(--text-muted); font-size: 13px; max-width: 62ch; }

  .progress-block { display: flex; flex-direction: column; gap: 6px; min-width: 220px; }

  .progress-numbers { display: flex; justify-content: space-between; font-size: 12px; color: var(--text-muted); font-variant-numeric: tabular-nums; }

  .progress-numbers strong { color: var(--text); }

  .progress-track { height: 8px; border-radius: 4px; background: var(--surface-2); border: 1px solid var(--border); overflow: hidden; display: flex; }

  .progress-fill { height: 100%; background: var(--done); }

  .controls { display: flex; align-items: center; gap: 10px; margin-top: 16px; flex-wrap: wrap; }

  #search {
    flex: 1 1 220px;
    min-width: 180px;
    background: var(--bg);
    border: 1px solid var(--border);
    color: var(--text);
    padding: 8px 10px;
    border-radius: 3px;
    font: inherit;
  }

  #search:focus-visible, .chip:focus-visible, .cb-toggle input:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .chip-group { display: flex; gap: 6px; }

  .chip {
    border: 1px solid var(--border);
    background: var(--surface-2);
    color: var(--text-muted);
    padding: 7px 12px;
    border-radius: 3px;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
  }

  .chip[aria-pressed="true"] { background: var(--accent); color: var(--accent-contrast); border-color: var(--accent); }

  .cb-toggle { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--text-muted); margin-left: auto; white-space: nowrap; }

  .table-wrap { overflow-x: auto; border: 1px solid var(--border); border-radius: 4px; }

  table { width: 100%; border-collapse: collapse; min-width: 720px; }

  thead th {
    position: sticky;
    top: 0;
    background: var(--surface-2);
    text-align: left;
    padding: 9px 12px;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
    white-space: nowrap;
  }

  tbody td { padding: 7px 12px; border-bottom: 1px solid var(--border); vertical-align: top; }

  tbody tr.data-row:hover { background: var(--surface-2); }

  tbody tr.group-row td {
    background: var(--bg);
    color: var(--text-muted);
    font-size: 11px;
    letter-spacing: 0.06em;
    padding: 5px 12px;
    border-bottom: 1px solid var(--border);
  }

  .col-op { font-variant-numeric: tabular-nums; white-space: nowrap; color: var(--text-muted); }
  .col-mnemonic { white-space: nowrap; font-weight: 600; }
  .col-fn { color: var(--accent); white-space: nowrap; }
  .col-desc { color: var(--text-muted); min-width: 260px; }
  .col-flags { font-variant-numeric: tabular-nums; white-space: nowrap; color: var(--text-muted); letter-spacing: 0.08em; }

  .pill { display: inline-block; padding: 2px 8px; border-radius: 3px; font-size: 11px; letter-spacing: 0.03em; white-space: nowrap; }
  .pill.done { color: var(--done); background: var(--done-bg); }
  .pill.todo { color: var(--todo); background: var(--todo-bg); }
  .pill.unused { color: var(--unused); }
  .pill.tests-yes { color: var(--done); background: var(--done-bg); }
  .pill.tests-no { color: var(--fail); background: var(--fail-bg); }
  .col-tests { color: var(--text-muted); }

  #empty-state { display: none; padding: 24px; text-align: center; color: var(--text-muted); }

  footer { margin-top: 18px; color: var(--text-muted); font-size: 12px; }
  footer a { color: inherit; text-decoration: underline; }

  @media (prefers-reduced-motion: no-preference) {
    .progress-fill { transition: width 0.4s ease; }
  }
</style>

<div class="page">
  <header class="panel">
    <div class="title-row">
      <div>
        <div class="chip-eyebrow">Sharp LR35902 &middot; unprefixed + CB opcode table</div>
        <h1>LR35902 Coverage</h1>
        <p class="subtitle">
          Every opcode this CPU can decode, whether it's implemented, still a
          placeholder, or doesn't exist on real hardware. Status below is
          read straight from the Rust source, not hand-maintained.
        </p>
      </div>
      <div class="progress-block">
        <div class="progress-numbers">
          <span><strong id="done-count">0</strong> implemented</span>
          <span><strong id="todo-count">0</strong> placeholder</span>
        </div>
        <div class="progress-track">
          <div class="progress-fill" id="progress-fill"></div>
        </div>
      </div>
    </div>

    <div class="controls">
      <input id="search" type="text" placeholder="Search opcode, mnemonic, function, or description&hellip;" aria-label="Search opcodes" />
      <div class="chip-group" role="group" aria-label="Filter by status">
        <button class="chip" data-filter="all" aria-pressed="true">All</button>
        <button class="chip" data-filter="done" aria-pressed="false">Implemented</button>
        <button class="chip" data-filter="todo" aria-pressed="false">Placeholder</button>
        <button class="chip" data-filter="unused" aria-pressed="false">Unused</button>
      </div>
      <label class="cb-toggle">
        <input type="checkbox" id="cb-toggle" />
        Show CB-prefixed table (256 more, not decoded yet)
      </label>
    </div>
  </header>

  <div class="table-wrap">
    <table>
      <thead>
        <tr><th>Op</th><th>Mnemonic</th><th>Status</th><th>Passed Tests?</th><th>Function</th><th>Flags (Z N H C)</th><th>Description</th></tr>
      </thead>
      <tbody id="rows"></tbody>
    </table>
    <div id="empty-state">No opcodes match that search.</div>
  </div>

  <footer>
    Generated by <code>render_instructions.py</code> from
    <code>src/architecture/instructions_set.rs</code> and
    <code>src/architecture/instructions_missing.rs</code> — re-run it after
    implementing an opcode to refresh this page.
  </footer>
</div>

<script>
  (function () {
    "use strict";

    var table = __TABLE_JSON__;

    function hex(n) { return "0x" + n.toString(16).toUpperCase().padStart(2, "0"); }

    function statusLabel(status) {
      return status === "done" ? "Implemented" : status === "todo" ? "Placeholder" : "Unused";
    }

    function buildCbRows() {
      var rows = [];
      for (var i = 0; i < 256; i++) {
        rows.push({
          op: "CB " + hex(i),
          mnemonic: "(not decoded)",
          status: "todo",
          tests_passed: null,
          fn: "cb_prefixed",
          flags: null,
          desc: "Part of the CB-prefixed table (rotates/shifts/BIT/SET/RES); not broken out individually yet."
        });
      }
      return rows;
    }

    var tbody = document.getElementById("rows");
    var searchInput = document.getElementById("search");
    var chips = Array.prototype.slice.call(document.querySelectorAll(".chip"));
    var cbToggle = document.getElementById("cb-toggle");
    var emptyState = document.getElementById("empty-state");

    var state = { filter: "all", query: "", showCb: false };

    function rowMatchesQuery(entry, opLabel, q) {
      if (!q) return true;
      var haystack = (opLabel + " " + entry.mnemonic + " " + (entry.fn || "") + " " + (entry.flags || "") + " " + entry.desc).toLowerCase();
      return haystack.indexOf(q) !== -1;
    }

    function appendDataRow(frag, opLabel, entry) {
      var tr = document.createElement("tr");
      tr.className = "data-row";

      var tdOp = document.createElement("td");
      tdOp.className = "col-op";
      tdOp.textContent = opLabel;

      var tdMnemonic = document.createElement("td");
      tdMnemonic.className = "col-mnemonic";
      tdMnemonic.textContent = entry.mnemonic;

      var tdStatus = document.createElement("td");
      var pill = document.createElement("span");
      pill.className = "pill " + entry.status;
      pill.textContent = statusLabel(entry.status);
      tdStatus.appendChild(pill);

      var tdTests = document.createElement("td");
      tdTests.className = "col-tests";
      if (entry.tests_passed === true) {
        var testsPill = document.createElement("span");
        testsPill.className = "pill tests-yes";
        testsPill.textContent = "Yes";
        tdTests.appendChild(testsPill);
      } else if (entry.tests_passed === false) {
        var testsPillFail = document.createElement("span");
        testsPillFail.className = "pill tests-no";
        testsPillFail.textContent = "No";
        tdTests.appendChild(testsPillFail);
      } else {
        tdTests.textContent = "—";
      }

      var tdFn = document.createElement("td");
      tdFn.className = "col-fn";
      tdFn.textContent = entry.fn || "—";

      var tdFlags = document.createElement("td");
      tdFlags.className = "col-flags";
      tdFlags.textContent = entry.flags || "—";

      var tdDesc = document.createElement("td");
      tdDesc.className = "col-desc";
      tdDesc.textContent = entry.desc;

      tr.appendChild(tdOp);
      tr.appendChild(tdMnemonic);
      tr.appendChild(tdStatus);
      tr.appendChild(tdTests);
      tr.appendChild(tdFn);
      tr.appendChild(tdFlags);
      tr.appendChild(tdDesc);
      frag.appendChild(tr);
    }

    function render() {
      var q = state.query.trim().toLowerCase();
      var frag = document.createDocumentFragment();
      var visibleCount = 0;

      for (var i = 0; i < 256; i++) {
        var entry = table[i];
        var opLabel = hex(i);
        var isGroupStart = i % 16 === 0;

        var matchesFilter = state.filter === "all" || entry.status === state.filter;
        var matchesQuery = rowMatchesQuery(entry, opLabel, q);
        var visible = matchesFilter && matchesQuery;

        if (isGroupStart) {
          var groupRow = document.createElement("tr");
          groupRow.className = "group-row";
          var groupTd = document.createElement("td");
          groupTd.colSpan = 7;
          groupTd.textContent = opLabel.slice(0, 3) + "_";
          groupRow.appendChild(groupTd);
          groupRow.style.display = "none";
          frag.appendChild(groupRow);
          groupRow._isGroup = true;
        }

        if (visible) {
          visibleCount++;
          appendDataRow(frag, opLabel, entry);
        }
      }

      var children = Array.prototype.slice.call(frag.children);
      for (var g = 0; g < children.length; g++) {
        if (children[g]._isGroup) {
          var next = children[g + 1];
          if (next && !next._isGroup) children[g].style.display = "";
        }
      }

      if (state.showCb) {
        var cbHeader = document.createElement("tr");
        cbHeader.className = "group-row";
        var cbTd = document.createElement("td");
        cbTd.colSpan = 7;
        cbTd.textContent = "0xCB __  (256 opcodes, not decoded individually)";
        cbHeader.appendChild(cbTd);
        frag.appendChild(cbHeader);

        buildCbRows().forEach(function (entry) {
          if (state.filter !== "all" && state.filter !== "todo") return;
          if (!rowMatchesQuery(entry, entry.op, q)) return;
          visibleCount++;
          appendDataRow(frag, entry.op, entry);
        });
      }

      tbody.innerHTML = "";
      tbody.appendChild(frag);
      emptyState.style.display = visibleCount === 0 ? "block" : "none";
    }

    function updateProgress() {
      var done = 0, todo = 0, total = 0;
      table.forEach(function (entry) {
        if (entry.status === "unused") return;
        total++;
        if (entry.status === "done") done++;
        else todo++;
      });
      document.getElementById("done-count").textContent = done;
      document.getElementById("todo-count").textContent = todo;
      document.getElementById("progress-fill").style.width = (done / total * 100).toFixed(1) + "%";
    }

    searchInput.addEventListener("input", function (e) { state.query = e.target.value; render(); });

    chips.forEach(function (chip) {
      chip.addEventListener("click", function () {
        chips.forEach(function (c) { c.setAttribute("aria-pressed", "false"); });
        chip.setAttribute("aria-pressed", "true");
        state.filter = chip.getAttribute("data-filter");
        render();
      });
    });

    cbToggle.addEventListener("change", function (e) { state.showCb = e.target.checked; render(); });

    updateProgress();
    render();
  })();
</script>
"""


def render_html() -> str:
    return HTML_TEMPLATE.replace("__TABLE_JSON__", json.dumps(TABLE, separators=(",", ":")))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--out",
        type=Path,
        default=REPO_ROOT / "opcode-coverage.html",
        help="where to write the rendered HTML (default: opcode-coverage.html)",
    )
    args = parser.parse_args()

    build_table()
    implemented = load_function_status()
    print("running `cargo test`...")
    tests_passed = load_test_status()
    resolve_statuses(implemented, tests_passed)
    resolve_flags()

    done = sum(1 for e in TABLE if e["status"] == "done")
    todo = sum(1 for e in TABLE if e["status"] == "todo")
    unused = sum(1 for e in TABLE if e["status"] == "unused")
    total_real = done + todo

    tests_yes = sum(1 for e in TABLE if e["tests_passed"] is True)
    tests_no = sum(1 for e in TABLE if e["tests_passed"] is False)
    tests_missing = sum(1 for e in TABLE if e["status"] != "unused" and e["tests_passed"] is None)

    args.out.write_text(render_html())

    print(f"{done}/{total_real} opcodes implemented ({done / total_real:.1%}); "
          f"{todo} placeholder, {unused} unused.")
    print(f"{tests_yes} passing tests, {tests_no} failing tests, "
          f"{tests_missing} with no test coverage.")
    print(f"wrote {args.out.relative_to(REPO_ROOT) if args.out.is_relative_to(REPO_ROOT) else args.out}")


if __name__ == "__main__":
    main()
