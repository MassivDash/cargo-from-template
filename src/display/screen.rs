use termion::color;

pub fn hr() {
    println!("{}", color::Fg(color::LightMagenta));
    println!(
        "{}",
        "=============================================================================================================================================="
    );
    println!("{}", color::Fg(color::Reset));
}

pub fn spacer() {
    println!("{}", color::Fg(color::Reset));
    println!("{}", color::Fg(color::Reset));
}

pub fn step(string: &str) {
    println!("{}", color::Fg(color::LightGreen));
    println!("{}", string);
    println!("{}", color::Fg(color::Reset));
}