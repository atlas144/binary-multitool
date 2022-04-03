pub fn draw_separator(length: u8) {
    let mut counter = 0;

    while counter < length {
        print!("-");
        counter += 1;
    }

    println!();
}

pub fn draw_underscore(line: &str) {
    draw_separator(line.len() as u8);
}
