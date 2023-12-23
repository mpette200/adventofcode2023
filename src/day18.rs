use std::fmt::Display;
use std::ops::{Index, IndexMut};

const LINE_SEP: &str = "\n";

pub fn run() {
    let input = r##"
    R 2 (#3e6b80)
    U 6 (#2b1de3)
    R 4 (#1465b0)
    U 2 (#549d23)
    R 6 (#aa9992)
    U 6 (#27e7f3)
    R 7 (#aa9990)
    U 6 (#5a3933)
    R 5 (#1465b2)
    U 5 (#23ade3)
    R 4 (#49b5b2)
    U 6 (#39ad53)
    R 7 (#54c4f2)
    U 5 (#491513)
    R 9 (#4f5960)
    U 5 (#357be3)
    R 3 (#4f2140)
    U 6 (#258b43)
    R 4 (#4f9570)
    U 3 (#1ea371)
    R 8 (#7be6c0)
    U 5 (#67bd61)
    R 5 (#a31240)
    U 6 (#6bf7d1)
    R 5 (#a31242)
    U 4 (#3464a1)
    R 3 (#7be6c2)
    U 4 (#33f091)
    R 5 (#5c0530)
    U 6 (#4c4e11)
    R 2 (#611e90)
    U 3 (#128231)
    R 6 (#5ecef2)
    D 9 (#6eb251)
    R 5 (#5e54d2)
    U 9 (#1e8081)
    R 3 (#73a8b0)
    U 10 (#6dc731)
    R 3 (#84d470)
    U 2 (#6b91a1)
    R 7 (#64edf0)
    U 5 (#4da881)
    R 10 (#710072)
    D 5 (#4aa4f1)
    R 7 (#4dd192)
    D 6 (#4aa4f3)
    R 3 (#2af062)
    D 6 (#6fabe1)
    R 5 (#177cc0)
    U 5 (#26ba61)
    R 5 (#5a3aa0)
    U 9 (#3b5401)
    R 5 (#012590)
    U 2 (#747f11)
    R 4 (#753b32)
    U 7 (#965a31)
    R 4 (#753b30)
    U 3 (#20c281)
    R 5 (#225830)
    U 4 (#7a4da1)
    R 4 (#716560)
    U 6 (#036ca1)
    R 5 (#55fd00)
    D 4 (#064ad1)
    R 4 (#93a480)
    D 6 (#411d21)
    R 3 (#990d00)
    D 6 (#4af4a1)
    R 3 (#0f8880)
    D 4 (#0f56c3)
    R 3 (#7f2590)
    U 7 (#0f56c1)
    R 7 (#374e40)
    U 6 (#4af4a3)
    R 2 (#161a20)
    U 3 (#5277d1)
    R 5 (#7c6760)
    U 4 (#528313)
    R 4 (#525840)
    U 4 (#267643)
    L 10 (#525842)
    U 5 (#501273)
    L 4 (#447950)
    D 10 (#69cdc3)
    L 3 (#530a00)
    U 10 (#590643)
    L 5 (#243450)
    D 5 (#74a853)
    L 6 (#7c2a42)
    U 5 (#1d9673)
    L 3 (#3f8d62)
    U 8 (#606b43)
    L 5 (#576480)
    D 8 (#63d243)
    L 5 (#86ad60)
    U 4 (#63d241)
    L 7 (#0be9f0)
    U 8 (#4b0801)
    L 2 (#64f9c0)
    U 4 (#478081)
    R 9 (#20b940)
    U 4 (#a60cd1)
    L 9 (#02c6a0)
    U 5 (#0d8be1)
    L 4 (#8226e0)
    D 6 (#013a81)
    L 3 (#5cf860)
    D 4 (#985951)
    L 5 (#162d72)
    D 2 (#1c2c31)
    L 2 (#654f82)
    D 4 (#51b4e1)
    R 10 (#401a32)
    D 5 (#32c291)
    L 7 (#84ff82)
    D 2 (#25cf41)
    L 3 (#43b090)
    D 4 (#6208d1)
    L 3 (#a428a0)
    U 9 (#6208d3)
    L 3 (#07d9c0)
    U 9 (#6e8a01)
    L 2 (#3ab640)
    U 3 (#187711)
    L 4 (#162d70)
    U 6 (#8e5a81)
    L 6 (#049580)
    D 6 (#a81333)
    L 2 (#342030)
    D 3 (#233073)
    L 7 (#510730)
    D 4 (#15cbf3)
    R 3 (#507520)
    D 5 (#3be881)
    R 6 (#52f2a0)
    D 3 (#a52711)
    L 3 (#0d7680)
    D 5 (#1db6c3)
    L 5 (#5f0450)
    U 5 (#6c6d73)
    L 2 (#47abc0)
    D 5 (#707733)
    L 5 (#771be2)
    D 2 (#151c53)
    L 3 (#413da2)
    D 3 (#768703)
    R 11 (#640b02)
    D 2 (#37b343)
    R 4 (#4ac8d0)
    D 5 (#016421)
    L 2 (#907990)
    D 5 (#016423)
    L 3 (#412220)
    U 5 (#0f5603)
    L 9 (#5e11d0)
    D 5 (#24abb1)
    L 7 (#1ab552)
    U 2 (#06e641)
    L 2 (#9e07e2)
    U 4 (#06e643)
    L 6 (#51f692)
    U 4 (#763581)
    L 8 (#3716a0)
    U 4 (#1360d1)
    R 5 (#567bc0)
    U 2 (#222831)
    R 9 (#7d2160)
    U 6 (#72b991)
    L 6 (#4a6840)
    U 3 (#3ef773)
    L 4 (#626a10)
    U 4 (#4c3d93)
    L 4 (#6fa5e2)
    D 4 (#6e0b33)
    L 4 (#6fa5e0)
    U 3 (#38aaf3)
    L 5 (#223552)
    U 5 (#025c33)
    L 4 (#4205c2)
    D 4 (#49ddd3)
    L 3 (#3aa782)
    D 4 (#028fd3)
    L 5 (#76e842)
    D 3 (#028fd1)
    L 3 (#9bc962)
    U 4 (#6bde73)
    L 9 (#9d6be0)
    U 6 (#2db831)
    L 5 (#1192c0)
    U 3 (#6b5c33)
    L 4 (#152940)
    U 4 (#746923)
    L 5 (#8648a0)
    U 3 (#4cb8d1)
    L 3 (#877170)
    U 5 (#24ca21)
    L 4 (#877172)
    D 3 (#6e4261)
    L 2 (#691310)
    D 6 (#655001)
    R 6 (#7e7740)
    D 3 (#1562b1)
    L 6 (#261a00)
    D 4 (#700e91)
    L 4 (#6ade10)
    U 9 (#4a1251)
    L 3 (#5a7c00)
    U 7 (#5a30e1)
    L 6 (#15a832)
    U 3 (#152e01)
    L 3 (#643622)
    U 5 (#977db1)
    L 7 (#4b7bc2)
    U 2 (#546691)
    L 7 (#8cd5a2)
    U 7 (#5b7a31)
    R 3 (#6c85f2)
    U 6 (#6996a1)
    R 8 (#458692)
    U 3 (#370e51)
    R 6 (#7bc6d2)
    U 4 (#07aa01)
    R 4 (#389dc2)
    U 6 (#3de6d3)
    R 6 (#54ece2)
    U 5 (#625703)
    R 10 (#0e9512)
    D 5 (#1e8793)
    R 3 (#873d92)
    U 4 (#2eb4b3)
    R 5 (#2d3432)
    U 5 (#26b2f1)
    R 2 (#2c8c52)
    U 3 (#2ff4c1)
    R 7 (#0e68f2)
    U 3 (#32b371)
    R 8 (#0e68f0)
    U 4 (#5d6fb1)
    R 3 (#128622)
    U 5 (#06af41)
    R 6 (#a95192)
    U 4 (#274af1)
    R 3 (#3af302)
    U 6 (#50fff3)
    R 6 (#3f9cf2)
    U 2 (#50fff1)
    R 3 (#4734a2)
    U 3 (#1a5051)
    L 12 (#986ce0)
    U 3 (#224b61)
    R 9 (#2d66c0)
    U 4 (#6f2a31)
    R 6 (#0aba80)
    U 7 (#0ecb81)
    R 4 (#606870)
    U 3 (#0ecb83)
    L 3 (#4e78e0)
    U 3 (#4563f1)
    L 11 (#8bed00)
    U 3 (#3298c1)
    L 5 (#5a39d0)
    U 2 (#51d641)
    L 8 (#8e6ee0)
    U 4 (#233851)
    R 4 (#31c7a0)
    U 9 (#728831)
    R 3 (#670cd0)
    U 4 (#1f3621)
    L 7 (#8df230)
    U 4 (#44a361)
    R 7 (#2aed30)
    U 7 (#2a0161)
    R 4 (#1db2c0)
    U 3 (#6387c1)
    R 3 (#1886b0)
    D 3 (#128c21)
    R 3 (#acb330)
    D 3 (#3a9d01)
    L 4 (#3658a0)
    D 6 (#00ec51)
    R 4 (#682712)
    D 6 (#2440e1)
    R 5 (#45f812)
    D 9 (#6298d1)
    R 3 (#778f22)
    U 4 (#6298d3)
    R 2 (#271462)
    U 10 (#0d5ff1)
    L 4 (#6a69e2)
    U 5 (#0d5ff3)
    R 6 (#24b5e2)
    U 3 (#2440e3)
    R 5 (#5f3192)
    U 4 (#41c313)
    R 3 (#236852)
    U 7 (#6e15c1)
    R 8 (#5187b2)
    D 5 (#6e15c3)
    R 2 (#65a3d2)
    D 4 (#41c311)
    R 11 (#3dfa02)
    D 4 (#26ca51)
    R 5 (#4e3912)
    D 6 (#797803)
    R 5 (#2f8ee2)
    U 5 (#2a9b83)
    R 8 (#3d6702)
    D 5 (#4646e3)
    R 3 (#3d6700)
    D 2 (#43a1c3)
    R 4 (#592f22)
    D 3 (#9d61c1)
    R 4 (#1f90f2)
    D 5 (#172261)
    R 7 (#5a7ba2)
    D 8 (#797801)
    R 6 (#4e4ad2)
    D 8 (#28fda1)
    R 5 (#55a862)
    D 4 (#28fda3)
    R 5 (#1665a2)
    D 6 (#3fca41)
    R 9 (#850700)
    D 3 (#43f8f1)
    R 3 (#3cbd90)
    D 6 (#620311)
    R 8 (#70b2b2)
    D 6 (#25cb11)
    R 3 (#336c52)
    D 4 (#8da331)
    R 4 (#336c50)
    D 5 (#0594c1)
    R 3 (#4a2b92)
    U 8 (#31cac1)
    R 5 (#392742)
    D 8 (#233743)
    R 6 (#798b32)
    D 4 (#233741)
    R 6 (#4fcde2)
    D 4 (#1ffd91)
    L 9 (#207ed2)
    U 5 (#9314d1)
    L 9 (#056d22)
    D 5 (#56c131)
    L 9 (#431340)
    D 6 (#94bbb1)
    R 8 (#357120)
    D 2 (#19ca71)
    R 8 (#8bebb0)
    D 6 (#4257c1)
    R 8 (#0c4fc0)
    D 3 (#380131)
    R 3 (#07a260)
    D 3 (#02a841)
    L 11 (#100a10)
    D 2 (#07ad11)
    L 4 (#4a2b90)
    D 3 (#10d091)
    R 6 (#609d22)
    D 2 (#69a281)
    R 9 (#320ed2)
    D 5 (#0782c1)
    R 2 (#67c382)
    D 2 (#19a311)
    R 7 (#14e152)
    D 3 (#4ee351)
    R 5 (#14e150)
    D 8 (#54dae1)
    R 3 (#428ba2)
    D 5 (#0a5431)
    R 3 (#1880e2)
    D 2 (#2fc091)
    R 9 (#440bc0)
    D 6 (#758b11)
    R 6 (#6a54b0)
    D 3 (#31f8d1)
    L 6 (#37e190)
    D 5 (#0c4af1)
    L 4 (#6acc62)
    D 2 (#161df1)
    L 4 (#72f272)
    D 4 (#161df3)
    L 7 (#088332)
    D 4 (#182831)
    L 8 (#82d720)
    D 3 (#371df1)
    L 7 (#7207b0)
    D 6 (#206d61)
    R 4 (#0c1082)
    D 2 (#796721)
    R 6 (#220252)
    D 7 (#2948a1)
    R 7 (#5f2c02)
    U 7 (#137d21)
    R 3 (#0fe252)
    D 2 (#0164d1)
    R 6 (#1ce912)
    D 6 (#4e0983)
    R 2 (#7d7c72)
    D 11 (#598c93)
    R 4 (#581202)
    D 8 (#0ffba3)
    L 4 (#500e32)
    D 2 (#42bcf3)
    R 4 (#31e852)
    D 8 (#2d8913)
    R 4 (#1d67d2)
    D 5 (#5a03c3)
    R 9 (#45c0b0)
    D 4 (#3a1e53)
    L 9 (#687430)
    D 4 (#3a1e51)
    L 4 (#25b1b0)
    D 8 (#613e03)
    L 7 (#4ea210)
    D 8 (#366193)
    L 3 (#4cb7c2)
    D 8 (#452333)
    L 6 (#675322)
    D 6 (#6ae223)
    L 3 (#0d88b2)
    D 2 (#23a4a3)
    L 8 (#60f512)
    D 5 (#568a53)
    L 3 (#0cbd02)
    D 3 (#4bad03)
    L 4 (#588672)
    D 4 (#72dfa3)
    L 2 (#960472)
    D 9 (#394403)
    L 5 (#28bcc2)
    D 6 (#4eb073)
    L 6 (#04e992)
    D 3 (#22c803)
    L 3 (#a76142)
    D 5 (#22c801)
    R 10 (#171512)
    D 3 (#25f2f3)
    R 6 (#581600)
    D 7 (#3d0f33)
    L 8 (#142a40)
    D 4 (#6eb893)
    L 5 (#142a42)
    U 4 (#02a243)
    L 9 (#522a40)
    U 2 (#4d2a01)
    L 4 (#081760)
    D 7 (#17c601)
    R 4 (#081762)
    D 6 (#497a01)
    R 5 (#2fbb50)
    D 3 (#4be663)
    R 4 (#3b3140)
    D 2 (#48f803)
    R 3 (#69ec82)
    U 3 (#80c993)
    R 5 (#3f05f2)
    U 8 (#5d5463)
    R 4 (#50d1c2)
    D 3 (#472973)
    R 3 (#1b68a2)
    D 5 (#5ebab3)
    R 7 (#464fd2)
    D 3 (#64c0a1)
    L 7 (#4062d2)
    D 6 (#8e33b3)
    L 5 (#1c3382)
    D 6 (#8e33b1)
    L 4 (#50b882)
    D 4 (#64c0a3)
    L 2 (#2bcfb2)
    D 7 (#84fc61)
    L 3 (#413452)
    U 10 (#84fc63)
    L 3 (#47ad42)
    U 2 (#03ba03)
    L 3 (#7197b2)
    U 5 (#81aa33)
    L 8 (#5c0552)
    D 5 (#235433)
    L 5 (#4236f2)
    D 5 (#4e9bc3)
    L 5 (#3740a0)
    D 2 (#1e7f23)
    L 10 (#3740a2)
    U 3 (#5f66a3)
    R 4 (#4236f0)
    U 7 (#164f83)
    R 4 (#5d1a30)
    U 3 (#1192e1)
    R 4 (#4c67a0)
    U 5 (#954a91)
    L 8 (#3348c0)
    U 3 (#306c01)
    L 4 (#694200)
    U 8 (#52c163)
    L 3 (#6abee0)
    U 3 (#26c123)
    L 4 (#6abee2)
    U 4 (#4c3413)
    L 9 (#4e8800)
    U 5 (#1192e3)
    L 3 (#2bb700)
    U 4 (#21ceb1)
    L 12 (#4ec360)
    U 3 (#21ceb3)
    L 4 (#32f940)
    U 8 (#5d5f73)
    L 4 (#1dcf10)
    U 9 (#3cf413)
    L 3 (#7cc870)
    U 8 (#5f0953)
    L 4 (#376c70)
    U 6 (#01d623)
    L 5 (#697e50)
    U 5 (#99e693)
    L 5 (#6f63b0)
    U 3 (#576c53)
    R 10 (#7fa030)
    U 3 (#1fd933)
    L 5 (#4eb2d2)
    U 6 (#39a1f3)
    L 4 (#3f0452)
    D 4 (#42ffd3)
    L 3 (#5b9d52)
    U 7 (#25c2d3)
    L 8 (#5ba250)
    D 7 (#40d763)
    L 5 (#5ba252)
    D 3 (#5f2a73)
    L 7 (#477ce2)
    D 3 (#2652d1)
    L 5 (#453770)
    D 5 (#758781)
    L 11 (#453772)
    D 3 (#937381)
    L 2 (#119e62)
    D 5 (#131891)
    R 5 (#3285e2)
    U 3 (#211fd3)
    R 11 (#8349d2)
    U 4 (#211fd1)
    R 4 (#3246c2)
    U 3 (#2515f3)
    R 9 (#5a5bd0)
    D 7 (#6fbf83)
    R 4 (#6a5680)
    D 3 (#6fbf81)
    R 9 (#329240)
    D 7 (#750063)
    R 5 (#5e71f2)
    D 9 (#3358d3)
    L 6 (#1d2652)
    D 4 (#6a8ee3)
    L 3 (#4e9cd2)
    U 6 (#3e8623)
    L 2 (#3607d2)
    U 5 (#379d43)
    L 6 (#3cf342)
    U 3 (#853b63)
    L 3 (#737102)
    D 3 (#1de1b3)
    L 2 (#66ff82)
    D 10 (#634163)
    L 3 (#2eff02)
    U 6 (#2ac343)
    L 2 (#528a42)
    U 7 (#4d81d3)
    L 3 (#2ea1e2)
    D 5 (#19e063)
    L 6 (#8b5a82)
    D 7 (#4b8e33)
    L 4 (#5e41d0)
    D 2 (#159903)
    L 5 (#31f3c0)
    D 9 (#537013)
    L 2 (#31f3c2)
    D 7 (#4b7b43)
    L 4 (#5bd890)
    D 10 (#1be163)
    L 5 (#0c3340)
    D 3 (#540d83)
    L 9 (#2449f0)
    U 6 (#9411c1)
    L 7 (#766db0)
    U 7 (#9411c3)
    L 4 (#37ee30)
    D 3 (#516ad3)
    L 3 (#0991b0)
    D 2 (#98a733)
    L 5 (#0ea042)
    D 3 (#513f83)
    L 2 (#a0c682)
    D 8 (#616323)
    L 4 (#a0c680)
    D 3 (#4555b3)
    L 3 (#79e032)
    U 9 (#673de3)
    L 4 (#5904b0)
    U 7 (#495bd3)
    L 3 (#5904b2)
    U 2 (#49a633)
    L 5 (#76a9c2)
    U 10 (#10de23)
    "##;
    let lines = read_lines(input);
    println!("{:#?}", lines);

    let moves = parse_lines(&lines);
    // println!("{:#?}", moves);

    let mut grid = make_boundary(&moves);
    println!("Boundaries: {}", grid);

    let (i, j) = find_start(&grid);
    println!("Starting at [{} {}]", i, j);

    recursive_fill(&mut grid, i, j);
    println!("Filled: {}", grid);

    let total1 = grid
        .elems
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x == '#')
        .count();
    println!("Total 1: {}", total1);
}

fn read_lines(txt: &str) -> Vec<String> {
    txt.lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}

fn parse_lines(lines: &[String]) -> Vec<(char, usize)> {
    lines
        .iter()
        .map(|x| {
            let d = x.chars().next().unwrap();
            let mut items = x.split_ascii_whitespace();
            let amount = items.nth(1).unwrap();
            (d, amount.parse().unwrap())
        })
        .collect()
}

fn make_boundary(moves: &[(char, usize)]) -> Grid<char> {
    let mut i_min = 0_i64;
    let mut i_max = 0_i64;
    let mut j_min = 0_i64;
    let mut j_max = 0_i64;
    let mut i = 0_i64;
    let mut j = 0_i64;
    for (d, amt) in moves {
        let amount = i64::try_from(*amt).unwrap();
        if *d == 'R' {
            j += amount;
        } else if *d == 'L' {
            j -= amount;
        } else if *d == 'D' {
            i += amount;
        } else if *d == 'U' {
            i -= amount;
        } else {
            panic!("Unrecognised char {}", *d);
        }

        if i_min > i {
            i_min = i;
        } else if i_max < i {
            i_max = i;
        }

        if j_min > j {
            j_min = j;
        } else if j_max < j {
            j_max = j;
        }
    }

    println!("min: [{} {}] max: [{} {}]", i_min, j_min, i_max, j_max);

    // the minimums could be negative
    let i_bound: usize = (i_max + i_min.abs() + 1).try_into().unwrap();
    let j_bound: usize = (j_max + j_min.abs() + 1).try_into().unwrap();

    let mut grid = Grid::<char>::new();
    grid.fill(i_bound, j_bound, || '.');

    let i_offset: usize = i_min.abs().try_into().unwrap();
    let j_offset: usize = j_min.abs().try_into().unwrap();

    let mut i = i_offset;
    let mut j = j_offset;
    for (d, amount) in moves {
        if *d == 'R' {
            for idx in j..j + amount + 1 {
                grid[i][idx] = '#';
            }
            j += *amount;
        } else if *d == 'L' {
            for idx in j - amount..j + 1 {
                grid[i][idx] = '#';
            }
            j -= *amount;
        } else if *d == 'D' {
            for idx in i..i + amount + 1 {
                grid[idx][j] = '#';
            }
            i += *amount;
        } else if *d == 'U' {
            for idx in i - amount..i + 1 {
                grid[idx][j] = '#';
            }
            i -= *amount;
        } else {
            panic!("Unrecognised char {}", *d);
        }
    }

    grid
}

/// Start needs to be inside outline.
/// This won't really cover all cases.
fn find_start(grid: &Grid<char>) -> (usize, usize) {
    let (j, _) = grid[1]
        .iter()
        .enumerate()
        .skip_while(|(_, x)| **x != '#')
        .nth(1)
        .unwrap();
    (1, j)
}

fn recursive_fill(grid: &mut Grid<char>, i: usize, j: usize) {
    if grid[i][j] == '#' {
        return;
    }
    grid[i][j] = '#';
    recursive_fill(grid, i + 1, j);
    recursive_fill(grid, i - 1, j);
    recursive_fill(grid, i, j + 1);
    recursive_fill(grid, i, j - 1);
}

#[derive(Debug)]
struct Grid<T> {
    elems: Vec<Vec<T>>,
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str(LINE_SEP);
        for row in self.elems.iter() {
            let val: String = row.iter().map(|x| format!("{}", x)).collect();
            out.push_str(&val);
            out.push_str(LINE_SEP);
        }
        write!(f, "{}", out)
    }
}

impl<T> Grid<T> {
    fn new() -> Self {
        Grid { elems: Vec::new() }
    }

    fn _push_iter(&mut self, items: impl Iterator<Item = T>) {
        self.elems.push(Vec::from_iter(items));
    }

    fn fill<F>(&mut self, rows: usize, cols: usize, f: F)
    where
        F: FnMut() -> T,
    {
        let mut func = f;
        self.elems.resize_with(rows, || {
            let mut v = Vec::with_capacity(cols);
            v.resize_with(cols, || func());
            v
        });
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}
