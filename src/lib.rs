pub mod streams;

pub fn format_offset(offset: usize, is_hex: bool) -> String {
    if is_hex {
        format!("{:06X}", offset)
    } else {
        format!("{}", offset)
    }
}
