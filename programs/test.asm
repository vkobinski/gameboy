; Exercises every opcode currently wired into parse_instruction.
; There's no LD yet, so B, C, D, BC, DE, HL, SP and [HL] all start at 0 —
; this only proves decode/dispatch works, arithmetic is limited without LD.

ADD A, 0x0F   ; A = 0x0F
ADC A, 0x01   ; A = 0x10 (carry in is 0)
AND A, 0x3C   ; A = 0x10 & 0x3C = 0x10

ADD A, B      ; A = A + B
ADC A, C      ; A = A + C + carry
AND A, D      ; A = A & D
ADD A, (HL)   ; A = A + [HL]

ADD HL, BC    ; HL = HL + BC
ADD HL, DE    ; HL = HL + DE
ADD SP, -2    ; SP = SP - 2

NOP
