use adventofcode2023 as lib;

fn main() {
    let val = lib::run_day13();
    if let Err(_) = val {
        println!("{:#?}", val);
    }
}
