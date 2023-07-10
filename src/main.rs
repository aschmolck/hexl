use std::io::{self, BufReader, Write};

const FW_OFFSET: u32 = 'Ａ' as u32 - 'A' as u32;
const SUP: [char; 10] = [' ', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
fn column_ruler(mut col: usize) -> String {
    col %= 100;
    if 0 == col % 10 {
        return format!("{:2}", col);
    }
    return format!(" {}", SUP[(col % 10) as usize]);
}
fn main() {
    let mut stdout = io::stdout().lock();
    let mut octets: usize = 0;
    for (i, line) in io::stdin().lines().enumerate() {
        let ln = i + 1;
        let line = line.expect("Error reading STDIN");
        let line = line.as_bytes();
        write!(stdout, "\x1b[32m{ln:6}\x1b[39m\x1b[1m\x1b[91m");
        for c in line {
            match c {
                33..=126 => {
                    write!(stdout, "{}", char::from_u32(*c as u32 + FW_OFFSET).unwrap());
                }
                32 => {
                    write!(stdout, "  ");
                }
                127 => {
                    write!(stdout, "^?");
                }
                0..=31 => {
                    write!(stdout, "\x1b[7m^{}\x1b[27m", ((*c as u8) + 64) as char);
                }
                _ => {
                    write!(stdout, "\x1b[7m{c:2x}\x1b[27m");

                    // panic!("I'm gonna deal with that later");
                }
            }
        }
        write!(stdout, "\x1b[m\n");
        write!(stdout, "\x1b[m{octets:06x}\x1b[m");
        for (j, c) in line.iter().enumerate() {
            write!(
                stdout,
                "{}{c:02x}\x1b[m",
                ["\x1b[22m", "\x1b[2m"][(j + octets) % 2]
            );
        }
        write!(stdout, "\x1b[m\n");
        write!(stdout, "\x1b[31m{octets:6}\x1b[m");
        for (j, _) in line.iter().enumerate() {
            let divisible_by_10 = (j % 10 == 0) as usize;
            write!(
                stdout,
                "{}{}",
                ["\x1b[2m", "\x1b[22m"][divisible_by_10],
                column_ruler(j),
            );
        }
        write!(stdout, "\x1b[m\n");
        octets += line.len();
    }
}
