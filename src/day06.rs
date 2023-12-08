use std::ops::Range;

pub fn run() {
    let input = r##"
    Time:        49     87     78     95
    Distance:   356   1378   1502   1882
    "##;
    let lines = read_lines(input);
    println!("{:#?}", lines);

    let times: Vec<i64> = read_nums(&lines[0]);
    let distances: Vec<i64> = read_nums(&lines[1]);

    println!("{:#?}", times);
    println!("{:#?}", distances);

    let races = read_races(&lines);

    println!("{:#?}", races);

    let total = compute_total(&races);
    println!("Total 1: {}", total);

    let joined_lines: Vec<String> = lines.iter().map(|x| x.replace(" ", "")).collect();
    println!("{:#?}", joined_lines);

    let race_2 = read_races(&joined_lines);
    let total_2 = compute_total(&race_2);
    println!("Total 2: {}", total_2);
}

fn read_lines(txt: &str) -> Vec<String> {
    txt.lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct RaceInfo {
    duration: i64,
    distance: i64,
}

fn read_races(lines: &Vec<String>) -> Vec<RaceInfo> {
    let times: Vec<i64> = read_nums(&lines[0]);
    let distances: Vec<i64> = read_nums(&lines[1]);

    println!("{:#?}", times);
    println!("{:#?}", distances);

    times
        .into_iter()
        .zip(distances)
        .map(|x| RaceInfo {
            duration: x.0,
            distance: x.1,
        })
        .collect()
}

fn read_nums(txt: &str) -> Vec<i64> {
    let mut items = Vec::new();
    let mut char_iter = txt.chars();
    loop {
        let digits: String = char_iter
            .by_ref()
            .skip_while(|x| !x.is_ascii_digit())
            .take_while(|x| x.is_ascii_digit())
            .collect();
        // println!("{}", digits);
        if digits.is_empty() {
            break;
        } else {
            items.push(digits.parse().unwrap());
        }
    }
    items
}

fn compute_total(races: &Vec<RaceInfo>) -> i64 {
    races
        .iter()
        .map(|x| {
            let valid_range = compute_hold_times(x.duration, x.distance);
            valid_range.end - valid_range.start
        })
        .product()
}

/// t = duration of race
/// d = distance of race
/// h = button held time
fn compute_hold_times(duration: i64, distance: i64) -> Range<i64> {
    // h (t - h) = d
    // ht - h^2 = d
    // -d = h^2 - ht
    // -d + t^2/4 = h^2 - ht + t^2/4
    // -d + t^2/4 = (h - t/2) ^ 2
    // sqrt(t^2/4 - d) = h - t/2
    // t/2 + sqrt(t^2/4 - d) = h
    let t: f64 = duration.to_string().parse().unwrap();
    let d: f64 = distance.to_string().parse().unwrap();

    let h_af = t / 2.0 - (t.powi(2) / 4.0 - d).sqrt();
    let h_bf = t / 2.0 + (t.powi(2) / 4.0 - d).sqrt();

    // Might be approximate due to rounding so check again
    let mut h_a: i64 = unsafe { h_af.to_int_unchecked() };
    let mut h_b: i64 = unsafe { h_bf.to_int_unchecked() };

    println!("compute: {} - {}", h_a, h_b);

    let lower = loop {
        let dist_computed = h_a * (duration - h_a);
        if dist_computed > distance {
            break h_a;
        }
        h_a += 1;
    };

    let upper = loop {
        let dist_computed = h_b * (duration - h_b);
        if dist_computed <= distance {
            break h_b;
        }
        h_b += 1;
    };

    let valid_range = lower..upper;
    println!(
        "{:#?} --> length({})",
        valid_range,
        valid_range.end - valid_range.start
    );

    valid_range
}
