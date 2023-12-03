pub fn run() {
    let input = r##"
    Hello, World!!!
    "##;
    let lines = read_lines(input);
    println!("{:#?}", lines);
}

fn read_lines(txt: &str) -> Vec<String> {
    txt.lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}
