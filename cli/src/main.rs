use chip_8::Chip8;

fn main() {
    let mut chip = Chip8::new();
    chip.load(include_bytes!("../../roms/ibm.ch8"));
    println!("{:?}", chip);
}
