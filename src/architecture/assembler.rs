pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();

    for (line_no, raw_line) in source.lines().enumerate() {
        let line = strip_comment(raw_line).trim();

        if line.is_empty() {
            continue;
        }

        let (mnemonic, rest) = split_mnemonic(line);
        let operands: Vec<&str> = if rest.is_empty() {
            Vec::new()
        } else {
            rest.split(',').map(|s| s.trim()).collect()
        };

        let result = match mnemonic.to_uppercase().as_str() {
            "NOP" => {
                bytes.push(0x00);
                Ok(())
            }
            "ADD" => encode_add(&operands, &mut bytes),
            "ADC" => encode_adc(&operands, &mut bytes),
            "AND" => encode_and(&operands, &mut bytes),
            other => Err(format!(
                "unsupported instruction `{}` (not implemented in the CPU yet)",
                other
            )),
        };

        result.map_err(|e| format!("line {}: {}", line_no + 1, e))?;
    }

    Ok(bytes)
}

fn encode_add(operands: &[&str], bytes: &mut Vec<u8>) -> Result<(), String> {
    if operands.len() != 2 {
        return Err("ADD requires two operands".to_string());
    }

    match operands[0].to_uppercase().as_str() {
        "A" => encode_a_op(0x80, 0xC6, operands[1], bytes),
        "HL" => encode_add_hl(operands[1], bytes),
        "SP" => encode_add_sp(operands[1], bytes),
        other => Err(format!("ADD {},x is not implemented", other)),
    }
}

fn encode_adc(operands: &[&str], bytes: &mut Vec<u8>) -> Result<(), String> {
    if operands.len() != 2 {
        return Err("ADC requires two operands".to_string());
    }

    if operands[0].to_uppercase() != "A" {
        return Err(format!(
            "ADC {},x is not implemented, only ADC A,x is supported",
            operands[0]
        ));
    }

    encode_a_op(0x88, 0xCE, operands[1], bytes)
}

fn encode_and(operands: &[&str], bytes: &mut Vec<u8>) -> Result<(), String> {
    if operands.len() != 2 {
        return Err("AND requires two operands".to_string());
    }

    if operands[0].to_uppercase() != "A" {
        return Err(format!(
            "AND {},x is not implemented, only AND A,x is supported",
            operands[0]
        ));
    }

    encode_a_op(0xA0, 0xE6, operands[1], bytes)
}

fn encode_add_hl(operand: &str, bytes: &mut Vec<u8>) -> Result<(), String> {
    match operand.to_uppercase().as_str() {
        "BC" => bytes.push(0x09),
        "DE" => bytes.push(0x19),
        "HL" => bytes.push(0x29),
        "SP" => bytes.push(0x39),
        other => return Err(format!("ADD HL,{} is not implemented", other)),
    }

    Ok(())
}

fn encode_add_sp(operand: &str, bytes: &mut Vec<u8>) -> Result<(), String> {
    let n = parse_number(operand)?;

    if !(-128..=127).contains(&n) {
        return Err(format!(
            "`{}` does not fit in a signed 8-bit value",
            operand
        ));
    }

    bytes.push(0xE8);
    bytes.push(n as i8 as u8);

    Ok(())
}

// r8/(HL)/A share one contiguous opcode block per family (ADD A,*, ADC A,*, AND A,*),
// ordered B,C,D,E,H,L,(HL),A — matches the real Game Boy opcode table.
fn r8_index(op: &str) -> Option<u8> {
    match op.to_uppercase().as_str() {
        "B" => Some(0),
        "C" => Some(1),
        "D" => Some(2),
        "E" => Some(3),
        "H" => Some(4),
        "L" => Some(5),
        "(HL)" => Some(6),
        "A" => Some(7),
        _ => None,
    }
}

fn encode_a_op(
    base_opcode: u8,
    imm_opcode: u8,
    operand: &str,
    bytes: &mut Vec<u8>,
) -> Result<(), String> {
    if let Some(idx) = r8_index(operand) {
        bytes.push(base_opcode + idx);
        return Ok(());
    }

    let n = parse_number(operand)?;

    if !(0..=255).contains(&n) {
        return Err(format!("`{}` is not a valid 8-bit value", operand));
    }

    bytes.push(imm_opcode);
    bytes.push(n as u8);

    Ok(())
}

fn parse_number(s: &str) -> Result<i32, String> {
    let trimmed = s.trim();

    let (neg, unsigned) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed),
    };

    let value = match unsigned
        .strip_prefix("0x")
        .or_else(|| unsigned.strip_prefix("0X"))
    {
        Some(hex) => {
            i32::from_str_radix(hex, 16).map_err(|_| format!("invalid hex literal `{}`", s))?
        }
        None => unsigned
            .parse::<i32>()
            .map_err(|_| format!("invalid number `{}`", s))?,
    };

    Ok(if neg { -value } else { value })
}

fn strip_comment(line: &str) -> &str {
    match line.find(';') {
        Some(idx) => &line[..idx],
        None => line,
    }
}

fn split_mnemonic(line: &str) -> (&str, &str) {
    match line.find(char::is_whitespace) {
        Some(idx) => (&line[..idx], line[idx..].trim()),
        None => (line, ""),
    }
}
