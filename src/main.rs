fn main() -> std::io::Result<()> {
    let cur_dir = std::env::current_dir()?;
    cargo_c1::cargo_clean1(cur_dir)
}
