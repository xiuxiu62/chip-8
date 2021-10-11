pub fn hex_to_col(hexcol: &str) -> Result<(u8, u8, u8), String> {
    let r = hex_to_u8(&hexcol[..2]);
    let g = hex_to_u8(&hexcol[2..4]);
    let b = hex_to_u8(&hexcol[4..]);

    if r.is_err() || g.is_err() || b.is_err() {
        return Err(format!(
            "Error converting hex '{}' to decimal, using default",
            hexcol
        ));
    }

    Ok((r.unwrap(), g.unwrap(), b.unwrap()))
}

pub fn hex_to_u8(hexbyte: &str) -> Result<u8, String> {
    match hex::decode(hexbyte) {
        Ok(val) => Ok(*val.get(0).unwrap()),
        Err(e) => Err(format!("{:?}", e)),
    }
}
