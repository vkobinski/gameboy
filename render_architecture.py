#!/usr/bin/env python3
"""Scans the repo for how much of the wider emulator (bus, cartridge/MBC,
interrupts, timer, PPU, joypad, APU, boot ROM, main loop) exists beyond raw
CPU opcode decode, and renders architecture-coverage.html.

Status here is necessarily heuristic (keyword/marker search over source
files), unlike render_instructions.py's precise per-opcode function-body
check — there's no one-function-per-feature scaffold for these subsystems.
Each check's evidence is shown on the card so you can judge for yourself.

Usage:
    python3 render_architecture.py
    python3 render_architecture.py --out docs/architecture.html
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent


def read(*parts: str) -> str:
    path = REPO_ROOT.joinpath(*parts)
    return path.read_text() if path.exists() else ""


def any_marker(text: str, markers: list[str]) -> list[str]:
    hits = []
    for m in markers:
        pattern = r"\b" + re.escape(m) + r"\b"
        if re.search(pattern, text, re.IGNORECASE):
            hits.append(m)
    return hits


def load_cpu_opcode_stats() -> dict:
    spec = importlib.util.spec_from_file_location(
        "render_instructions", REPO_ROOT / "render_instructions.py"
    )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)

    module.build_table()
    implemented = module.load_function_status()
    module.resolve_statuses(implemented)

    done = sum(1 for e in module.TABLE if e["status"] == "done")
    todo = sum(1 for e in module.TABLE if e["status"] == "todo")
    return {"done": done, "todo": todo, "total": done + todo, "implemented_fns": implemented}


def check_cpu_opcodes(cpu_stats: dict) -> dict:
    done, total = cpu_stats["done"], cpu_stats["total"]
    if done == 0:
        status = "not_started"
    elif done == total:
        status = "done"
    else:
        status = "partial"
    return {
        "status": status,
        "evidence": f"{done}/{total} unprefixed opcodes implemented (see LR35902 Coverage). CB table not broken out.",
    }


def check_cartridge_bus_wiring() -> dict:
    main_src = read("src", "main.rs")
    bus_src = read("src", "architecture", "bus.rs")
    wired_in_bus = "cartridge" in bus_src.lower() and "rom" in bus_src.lower()
    stepped_in_main = ".step()" in main_src

    if wired_in_bus:
        return {"status": "partial" if not stepped_in_main else "done",
                 "evidence": "bus.rs references the cartridge's ROM."}
    return {
        "status": "not_started",
        "evidence": "cartridge.rom (src/cartridge/cartridge.rs) is never referenced from bus.rs or main.rs — "
                    "the CPU currently reads an all-zero array, not the loaded ROM.",
    }


def check_address_mapped_bus() -> dict:
    bus_src = read("src", "architecture", "bus.rs")
    mem_src = read("src", "architecture", "mem.rs")
    combined = bus_src + "\n" + mem_src
    markers = ["0x8000", "0x9FFF", "0xA000", "0xC000", "0xFE00", "VRAM", "OAM", "WRAM", "match addr"]
    hits = any_marker(combined, markers)
    if len(hits) >= 3:
        status = "done"
    elif hits:
        status = "partial"
    else:
        status = "not_started"
    evidence = (
        f"Address-range markers found: {', '.join(hits)}."
        if hits else
        "mem.rs is one flat [u8; 0xFFFF] array; bus.rs has no per-region dispatch — "
        "every address behaves identically."
    )
    return {"status": status, "evidence": evidence}


def check_mbc_banking() -> dict:
    combined = (
        read("src", "cartridge", "cartridge.rs")
        + read("src", "architecture", "mem.rs")
        + read("src", "architecture", "bus.rs")
    )
    markers = ["rom_bank", "ram_bank", "bank_select", "current_bank", "bank_switch"]
    hits = any_marker(combined, markers)
    header_decodes_mbc = "Mbc1" in read("src", "cartridge", "c_type.rs")
    if hits:
        return {"status": "partial", "evidence": f"Bank-switching markers found: {', '.join(hits)}."}
    if header_decodes_mbc:
        return {
            "status": "not_started",
            "evidence": "CartridgeType (c_type.rs) already decodes MBC1/MBC2/MBC3/MBC5 from the header, "
                        "but no code switches banks yet — every cartridge is treated as ROM-only.",
        }
    return {"status": "not_started", "evidence": "No bank-switching code and no MBC type decoding found."}


def check_interrupts(cpu_stats: dict) -> dict:
    cpu_src = read("src", "architecture", "cpu.rs")
    has_ime = re.search(r"\bime\b", cpu_src, re.IGNORECASE) is not None
    impl = cpu_stats["implemented_fns"]
    handler_done = [n for n in ("ei", "di", "reti") if impl.get(n)]
    if has_ime or handler_done:
        return {
            "status": "partial",
            "evidence": f"{'IME found in cpu.rs. ' if has_ime else ''}"
                        f"{'Implemented: ' + ', '.join(handler_done) + '.' if handler_done else ''}".strip(),
        }
    return {
        "status": "not_started",
        "evidence": "No IME/IE/IF handling in cpu.rs; EI (0xFB), DI (0xF3), RETI (0xD9) are still "
                    "pass() placeholders.",
    }


def check_timer() -> dict:
    all_src = "".join(p.read_text() for p in (REPO_ROOT / "src").rglob("*.rs"))
    markers = ["TIMA", "TMA", "TAC"]
    hits = any_marker(all_src, markers)
    timer_file = any((REPO_ROOT / "src").rglob("*timer*"))
    if timer_file or hits:
        return {"status": "partial", "evidence": f"Markers found: {', '.join(hits) or 'timer file present'}."}
    return {"status": "not_started", "evidence": "No timer file, no DIV/TIMA/TMA/TAC anywhere in src/."}


def check_ppu() -> dict:
    ppu_file = list((REPO_ROOT / "src").rglob("*ppu*")) + list((REPO_ROOT / "src").rglob("*gpu*"))
    all_src = "".join(p.read_text() for p in (REPO_ROOT / "src").rglob("*.rs"))
    hits = any_marker(all_src, ["LCDC", "scanline", "framebuffer"])
    if ppu_file or hits:
        return {"status": "partial", "evidence": f"Found: {', '.join(hits) or ppu_file[0].name}."}
    return {"status": "not_started", "evidence": "No PPU/GPU file, no LCDC/STAT/scanline/framebuffer anywhere."}


def check_joypad(cpu_stats: dict) -> dict:
    joypad_file = list((REPO_ROOT / "src").rglob("*joypad*")) + list((REPO_ROOT / "src").rglob("*input*"))
    impl = cpu_stats["implemented_fns"]
    io_fns = [n for n in ("ld_c_ind_a", "ld_a_c_ind", "ldh_a8_a", "ldh_a_a8") if impl.get(n)]
    if joypad_file or io_fns:
        return {"status": "partial", "evidence": f"Found: {', '.join(io_fns) or joypad_file[0].name}."}
    return {"status": "not_started", "evidence": "No joypad file; the I/O-register opcodes that would touch 0xFF00 are still placeholders."}


def check_apu() -> dict:
    apu_file = list((REPO_ROOT / "src").rglob("*apu*")) + list((REPO_ROOT / "src").rglob("*sound*")) + list((REPO_ROOT / "src").rglob("*audio*"))
    if apu_file:
        return {"status": "partial", "evidence": f"Found: {apu_file[0].name}."}
    return {"status": "not_started", "evidence": "No APU/sound/audio file anywhere in src/."}


def check_boot_rom() -> dict:
    main_src = read("src", "main.rs")
    if re.search(r"\bboot\b", main_src, re.IGNORECASE):
        return {"status": "partial", "evidence": "main.rs references a boot ROM."}
    return {"status": "not_started", "evidence": "main.rs loads the cartridge directly; no boot ROM step. (Optional — low priority.)"}


def check_main_loop() -> dict:
    main_src = read("src", "main.rs")
    if ".step()" in main_src:
        return {"status": "done" if "loop" in main_src else "partial",
                 "evidence": "main.rs calls cpu.step()."}
    return {
        "status": "not_started",
        "evidence": "main.rs parses and prints the cartridge header, then exits — cpu.step() is never called.",
    }


SUBSYSTEMS = [
    ("CPU opcode decode", "Fetch/decode/execute for all 245 real opcodes + the CB-prefixed table.", check_cpu_opcodes, True),
    ("Cartridge → Bus wiring", "Getting the parsed ROM bytes actually readable by the CPU.", check_cartridge_bus_wiring, False),
    ("Address-mapped bus", "Per-region behavior for ROM, VRAM, WRAM, OAM, I/O registers, HRAM.", check_address_mapped_bus, False),
    ("MBC bank switching", "ROM/RAM bank switching for MBC1/2/3/5 cartridges over 32KB.", check_mbc_banking, False),
    ("Interrupts", "IME/IE/IF and dispatch for VBlank, LCD STAT, Timer, Serial, Joypad.", check_interrupts, True),
    ("Timer", "DIV/TIMA/TMA/TAC — free-running divider and configurable timer.", check_timer, False),
    ("PPU (LCD/graphics)", "Turns VRAM + OAM + registers into a 160x144 image, scanline by scanline.", check_ppu, False),
    ("Joypad input", "Reading button state through the 0xFF00 register.", check_joypad, True),
    ("APU (sound)", "Four channels mixed to an audio buffer.", check_apu, False),
    ("Boot ROM", "The 256-byte startup ROM that runs before the cartridge maps in. Optional.", check_boot_rom, False),
    ("Main execution loop", "Actually running the emulator: step CPU, feed cycles to timer/PPU, service interrupts, poll input.", check_main_loop, False),
]


def build_cards(cpu_stats: dict) -> list[dict]:
    cards = []
    for name, desc, check_fn, needs_cpu_stats in SUBSYSTEMS:
        result = check_fn(cpu_stats) if needs_cpu_stats else check_fn()
        cards.append({"name": name, "desc": desc, **result})
    return cards


HTML_TEMPLATE = r"""<title>Emulator Architecture</title>
<style>
  :root {
    --bg: #f5f6ef;
    --surface: #ffffff;
    --surface-2: #eceee1;
    --border: #dbdfcb;
    --text: #1b2013;
    --text-muted: #5c6350;
    --accent: #3f7a72;
    --done: #2f7d3a;
    --done-bg: #e3f0e0;
    --partial: #ad6a17;
    --partial-bg: #f6ead2;
    --not-started: #8b9279;
    --not-started-bg: #ececdf;
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
      --done: #7cc576;
      --done-bg: #1e3320;
      --partial: #e0973c;
      --partial-bg: #3a2c14;
      --not-started: #838a70;
      --not-started-bg: #21261a;
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
    --done: #7cc576;
    --done-bg: #1e3320;
    --partial: #e0973c;
    --partial-bg: #3a2c14;
    --not-started: #838a70;
    --not-started-bg: #21261a;
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
    line-height: 1.55;
  }

  .page { max-width: 900px; margin: 0 auto; padding: 28px 20px 48px; }

  .chip-eyebrow { text-transform: uppercase; letter-spacing: 0.08em; font-size: 11px; color: var(--text-muted); }

  h1 { margin: 4px 0 0; font-size: 22px; font-weight: 700; letter-spacing: -0.01em; text-wrap: balance; }

  .subtitle { margin: 6px 0 24px; color: var(--text-muted); font-size: 13px; max-width: 66ch; }

  .summary-row {
    display: flex;
    gap: 10px;
    margin-bottom: 22px;
    flex-wrap: wrap;
  }

  .summary-pill {
    border: 1px solid var(--border);
    background: var(--surface);
    border-radius: 4px;
    padding: 8px 12px;
    font-size: 12px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .summary-pill strong { color: var(--text); font-size: 14px; }

  .order-note {
    border: 1px dashed var(--border);
    border-radius: 4px;
    padding: 10px 14px;
    margin-bottom: 24px;
    font-size: 12px;
    color: var(--text-muted);
  }

  .order-note a { color: var(--accent); }

  .cards { display: flex; flex-direction: column; gap: 10px; }

  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 5px;
    padding: 14px 16px;
    box-shadow: 0 1px 3px var(--shadow);
  }

  .card-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 10px;
    flex-wrap: wrap;
  }

  .card-name { font-weight: 700; font-size: 14px; }

  .pill {
    display: inline-block;
    padding: 2px 9px;
    border-radius: 3px;
    font-size: 11px;
    letter-spacing: 0.03em;
    white-space: nowrap;
  }

  .pill.done { color: var(--done); background: var(--done-bg); }
  .pill.partial { color: var(--partial); background: var(--partial-bg); }
  .pill.not_started { color: var(--not-started); background: var(--not-started-bg); }

  .card-desc { margin: 6px 0 8px; color: var(--text-muted); font-size: 13px; }

  .card-evidence {
    font-size: 12px;
    color: var(--text-muted);
    border-left: 2px solid var(--border);
    padding-left: 10px;
  }

  footer { margin-top: 26px; color: var(--text-muted); font-size: 12px; }
  footer a { color: var(--accent); text-decoration: underline; }
</style>

<div class="page">
  <div class="chip-eyebrow">Beyond the opcode table</div>
  <h1>Emulator Architecture</h1>
  <p class="subtitle">
    What exists today outside of raw CPU opcode decode: bus wiring, cartridge
    banking, interrupts, timer, PPU, joypad, APU, boot ROM, and the loop that
    actually runs the thing. Status is read from the repo each time this
    renders — see the evidence line on each card.
  </p>

  <div class="summary-row" id="summary"></div>

  <div class="order-note">
    Suggested build order: Cartridge→Bus wiring, then the address-mapped bus,
    then more CPU opcodes + interrupts, then a bare main loop, then Timer,
    then the PPU (first real payoff), then MBC banking, then Joypad, then
    APU, with Boot ROM optional at any point. See <code>ARCHITECTURE.md</code>
    for the reasoning.
  </div>

  <div class="cards" id="cards"></div>

  <footer>
    Generated by <code>render_architecture.py</code>. CPU opcode detail lives in
    <code>INSTRUCTIONS.md</code> / <a href="./opcode-coverage.html">opcode-coverage.html</a>.
  </footer>
</div>

<script>
  (function () {
    "use strict";
    var cards = __CARDS_JSON__;

    function label(status) {
      return status === "done" ? "Done" : status === "partial" ? "Partial" : "Not started";
    }

    var counts = { done: 0, partial: 0, not_started: 0 };
    cards.forEach(function (c) { counts[c.status]++; });

    var summary = document.getElementById("summary");
    [["done", "Done"], ["partial", "Partial"], ["not_started", "Not started"]].forEach(function (pair) {
      var el = document.createElement("div");
      el.className = "summary-pill";
      el.innerHTML = "<strong>" + counts[pair[0]] + "</strong> " + pair[1];
      summary.appendChild(el);
    });

    var container = document.getElementById("cards");
    cards.forEach(function (c) {
      var card = document.createElement("div");
      card.className = "card";

      var head = document.createElement("div");
      head.className = "card-head";
      var name = document.createElement("div");
      name.className = "card-name";
      name.textContent = c.name;
      var pill = document.createElement("span");
      pill.className = "pill " + c.status;
      pill.textContent = label(c.status);
      head.appendChild(name);
      head.appendChild(pill);

      var desc = document.createElement("div");
      desc.className = "card-desc";
      desc.textContent = c.desc;

      var evidence = document.createElement("div");
      evidence.className = "card-evidence";
      evidence.textContent = c.evidence;

      card.appendChild(head);
      card.appendChild(desc);
      card.appendChild(evidence);
      container.appendChild(card);
    });
  })();
</script>
"""


def render_html(cards: list[dict]) -> str:
    return HTML_TEMPLATE.replace("__CARDS_JSON__", json.dumps(cards, separators=(",", ":")))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--out",
        type=Path,
        default=REPO_ROOT / "architecture-coverage.html",
        help="where to write the rendered HTML (default: architecture-coverage.html)",
    )
    args = parser.parse_args()

    cpu_stats = load_cpu_opcode_stats()
    cards = build_cards(cpu_stats)

    args.out.write_text(render_html(cards))

    counts = {"done": 0, "partial": 0, "not_started": 0}
    for c in cards:
        counts[c["status"]] += 1
    print(f"{counts['done']} done, {counts['partial']} partial, {counts['not_started']} not started "
          f"(of {len(cards)} subsystems)")
    print(f"wrote {args.out.relative_to(REPO_ROOT) if args.out.is_relative_to(REPO_ROOT) else args.out}")


if __name__ == "__main__":
    main()
