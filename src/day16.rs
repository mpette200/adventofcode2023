use std::{collections::VecDeque, fmt::Display};

const LINE_SEP: &str = "\n";

pub fn run() {
    let input = r##"
    \.........|......./...|........................................................|............|-...\...-/.......
    .....................\.......\........./...|...............|...\................................-...|.......\.
    .......................................-....-./............/.......-.............\..|.......|......//.........
    ...../..../.....\.........-.........-./..-.......\./..\.............-.....\..........|-..................|....
    ...........\..\...............-.............|.......-.\......\...\..........\.......-.....|..................\
    ................|.........|........|............--........./\....|-.....................................-.....
    ........./...........\................../...................\././........./...-...-.\.-.........-.............
    ......|.....|............-............./...|...................-...|........./............/......./...........
    ......./........../..|.....-...........\|.................................................\.........|..../....
    ........................|..................|.......|.................-..\...........-/.\.................\.|..
    .......|...|./....................................-...\........\.......|............................|...|.....
    \..-................................\.......\........../....|.....................|........\../.........../..\
    ....-|...............................\....|..............\.....................|.....\......\............../..
    .....\......-.........-.\.|..........|...............-.................|..........-............../............
    .........-...................-.............................................-.........../...................|.-
    ....................\..\........|-.\...............-............/....|..................../...................
    .\.........-........................|..-.......|.......-...........-.-...................\/...\....|....\.....
    .-........-./.....-.....\.......-..........\..-........\..-.....................................\..../...|....
    ....-.........................|..\.....................................\./...............-............/.......
    ..........|..\....-........./..............................|........................-.......|./|........\.....
    ..\.........|..|..-|........\...-................|/..\......../.......-|..................||.-./..............
    .--..-.......-........./.................\....../.................-............................--.........-...
    .......|.........../......./........../..\.........\...\../..\..././.......-..................................
    .........-/...............|..\|.................\...........|............../............\.....................
    ..........-.........\.-................|-..................../......../...|/...\..-....|-........-.......-....
    .--|.\|.......|.....-............................|.-....-...|...................|........../..................
    .....-......./..../.|...................|\..-.........../...|.............................\/../........../....
    ......./........|.\......-../...........\........................./..........\.........-...\................|.
    ./.......................\........-......|...|............................/............\....-.....|..../...\..
    ....................-..............-..-.............../..\...../.\.........\................../.....\.....-...
    ...................../......-....-.......|....|......../..|-...............\..\../.-.....././..........|......
    ..\.....|...\............................./.../...../..../......../...........................................
    ./.../................................../.....-.........|.....|..........-...............................-....
    ...............|..\...-................................\.........................|....\...............-.-.....
    /........|.......|.....-.......-..-..-.\.................................-.\...-.......\.............-..\.....
    ./......|../......|................................/.........||.-....................|..\.........-....|../...
    |..................................................\......|...-............................../.......-........
    ....../...............-....-..................\....-../................................./.\.................-.
    ......-...........-...../.....-......|..........\.\...-......../......\................--..............-......
    .............\/........|...../...........-.......\...|........................................................
    ...\\........\..........|...\............/........-..../....-.....................-......................../..
    .......-......../......./..........-....|....-..\.........................\.............../..........-.....-..
    .....|\/..../.........|.........../|.....\.......-........-............../..././.............|................
    .........|....|....\..-....../............/.....\/....|.............\........................|.........-......
    .\\..........................//......................\....\...........................-.|/....................
    ...................|...................-.|........................................../..........\..............
    ......\..|....../...........\.-.........-.......-..|../........../.............................../.......-...|
    ..........|................/|.....-.....\................||........./.............|.........../..-..|./.......
    ............../../|.......\...............-..../|..../-..|................................../......-....|..|..
    ............\...............\.........................\....-.............-..|/.....--...................-...-.
    .....|\......\...........................-.................-.......\...............|............|........../..
    .....\...-.............|............\........\...../.-...........|.-.................\.........|..............
    ......./......\..-............|..-...\......................\..........|....../..........|........./.......\.-
    ..-.....|........./|............./.....................|...|.\....-.......-..........|...\.\|-........\....|..
    ......................../............-...........|...................../............-.......\.........-.......
    ......../..............-./...................................--........................\....-.......-..//.....
    ........|.....-....................|...../..............-..........-......|..\................................
    .......-..//.............../.....-......................./..|........./...........-..../..|.........-.........
    .-...............-...........................\.......\...........-..................-.|...\..\.....\..\.....\.
    .........-../............|......|.|.........................\.......-......\...\..............-../.../-\......
    .-...........-..|.....\/........-.|...................../.|........-...........\..........\.....\/|..../...\.\
    .-..........\.\-..\....-\/................./.\.......|.........................-............./....\........./.
    .............\.........|............/...........\\................./..-..........................|//.......\..
    ..../-........-.........................-\...........|................./..........|.............|.\..|........
    ...-..|.....\..-..\.................\-.................|...../.-..........|........|/...-.-.\.................
    .-.-............................|.....\..-..........|/|.........|....\............\.....\..-...........|/...|.
    ......................../...............|..-............................-.........|........../.-....../.......
    .........|-...................|.....................-...........|.../..............................|....-...-.
    ..............|....................................................|.....|.......--.....\...\.............../.
    ..-...............|........|..................-............|.........../.....\................................
    ........................|.........|............-....-..\../..................\..|......\.........\../.../.....
    .|............................./..............-....-.......--....-.../..........|.....\.........-/.....|-./...
    ....-..............|...-.....\.....\/.-........................................-.\.............|...../|/......
    ........./..-................/........./..............-............................................-....-.....
    ...........-........./...................../.........-........-................./.|...........................
    ............................................................-.............-..-......|..........-.\............
    ....\/........................-../.../.................................\............/..........|./-......-...\
    ....-............../..-.\.......|......-...|...|.........../......|../...........|..-.\.................../...
    ...-......../....-........................|...-............-.........................|/.....|.........|......-
    ...|........-.......-.......|...|\.......\............-...-...................................-|........-.....
    ....\/..-.....-.....-..--./-..-............../................|............\.............../.......|..........
    .........-.\.......-......|........-.............|..../..............\-.....\..........-..../.................
    .................-......\|.--..../../.-......-.......-........\....-...../....................../.............
    ...../..../....../.............-......\..........|..........\..\........-.-...................................
    ..............................|........./........-..................|.............-.../.................\..-..
    .....|./......-........\...\.................|....-........|..............|...................................
    \...|............................-..|.............../..............|.../......-......-.........../....../.....
    ..../.............................-..........\.....\-..................................-........../...........
    ..........................\....................-...-/.|....................../................................
    \.......|.........-............\\...............-...-...................../...................................
    ..\...................................-................................................../.......\.|..........
    ...............................-................/........\....\.......|...................-.../.....|.........
    ...-..-............-.||..../././..........-\\............||...............-.\.........../...\........|...\...|
    ....\./.|./...................\..-.-./..-.....||............../............|.......\.|...../..................
    ......-.......\.\.........\......|..\...\.-|.\..............|...|......\......./...\........................-.
    ........./.-......../....|..............................|...|...\........//./.................................
    .........................|...........\...........|.....................\........//..-.................\-/.....
    .\....................\..-.-..-..........|/.\..........\././.....................|............\....|...|/.....
    ......./.................../...-.................\.........../.........\................|....................|
    ......-..\....|..................\............\........|......|....../........................................
    .........\/..............|.............\.........-............-...........\.\..\./....../.....//..............
    .........../...\.......-.../.-.............\../....\|./............/...-..\........|..|/.....\......\\........
    ..|............/..............................................-./...-.|..........-......../...................
    ...............-..|.......\.............\..../.....................|..........\...................\.....\./\..
    .|..........\..\...|......................\...........-.......--........................-.....................
    ...................../................\.|..........-..|................-................................-.....
    ...../.........|-............|.\.|.../\.....|../........\../..................................................
    ................/........\../.....................\./..../...........-............\-......../../..\.|.........
    ..............|.........-......|/......././.....|..../.|...........-.....................|./.-.....\...|......
    .......|..-.......\..........|................................................/..\.....-..........-...........
    "##;
    let lines = read_lines(input);
    println!("{:#?}", lines);

    let devices = parse_lines(&lines);
    println!("devices: {}", devices);

    let mut out1 = empty_output(devices.elems.len(), devices.elems[0].len());
    // println!("out1_empty: {}", out1);

    shine_light(&devices, &mut out1);
    println!("out1: {}", out1);

    let total1 = out1
        .elems
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| x.is_any_alight())
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

fn parse_lines(lines: &[String]) -> Grid<Device> {
    let mut out = Grid::new();
    for line in lines {
        out.push_iter(line.chars().map(|x| parse_device(x)));
    }
    out
}

fn parse_device(c: char) -> Device {
    use Device::*;
    match c {
        '.' => Empty,
        '/' => MirrorUL,
        '\\' => MirrorUR,
        '-' => SplitterLR,
        '|' => SplitterUD,
        x => panic!("Bad input: {}", x),
    }
}

fn empty_output(rows: usize, cols: usize) -> Grid<Energized> {
    let mut v = Grid::new();
    v.fill(rows, cols, Energized::new);
    v
}

fn shine_light(devices: &Grid<Device>, result: &mut Grid<Energized>) {
    let mut queue = VecDeque::<CurrentPos>::new();
    queue.push_back(CurrentPos::new(Coord(0, 0), Direction::Right));
    while !queue.is_empty() {
        queue
            .pop_front()
            .unwrap()
            .shine(devices, result, &mut queue);
    }
}

#[derive(Debug, Clone, Copy)]
struct CurrentPos {
    coord: Coord,
    going_to: Direction,
}

impl CurrentPos {
    fn new(coord: Coord, going_to: Direction) -> Self {
        Self { coord, going_to }
    }

    fn shine(
        &mut self,
        devices: &Grid<Device>,
        result: &mut Grid<Energized>,
        queue: &mut VecDeque<Self>,
    ) {
        use NextDirection::*;
        let bounds = devices.bounds();
        while !result.get(self.coord).is_alight(self.going_to) {
            let c = self.coord;
            result.get_mut(c).set_alight(self.going_to);
            let (coord, going_to) = match devices.get(c).next_dir(self.going_to) {
                Single(d) => match c.next_coord(d, bounds) {
                    Some(coord) => (coord, d),
                    None => break,
                },
                Pair(d, d2) => {
                    match c.next_coord(d2, bounds) {
                        Some(nc2) => queue.push_back(Self::new(nc2, d2)),
                        None => (),
                    };
                    match c.next_coord(d, bounds) {
                        Some(coord) => (coord, d),
                        None => break,
                    }
                }
            };
            self.coord = coord;
            self.going_to = going_to;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum NextDirection {
    Single(Direction),
    Pair(Direction, Direction),
}

#[derive(Debug, Clone, Copy)]
enum Device {
    Empty,
    MirrorUL,
    MirrorUR,
    SplitterLR,
    SplitterUD,
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Device::*;
        let val = match self {
            Empty => '.',
            MirrorUL => '/',
            MirrorUR => '\\',
            SplitterLR => '-',
            SplitterUD => '|',
        };
        write!(f, "{}", val)
    }
}

impl Device {
    fn lookup_dir(map: &[(Direction, Direction)], going_to: Direction) -> Direction {
        map.iter()
            .find_map(|x| if x.0 == going_to { Some(x.1) } else { None })
            .unwrap()
    }

    fn next_dir(&self, going_to: Direction) -> NextDirection {
        use Device::*;
        use Direction::*;
        use NextDirection::*;

        const MIRR_UL_MAP: &'static [(Direction, Direction)] =
            &[(Down, Left), (Up, Right), (Left, Down), (Right, Up)];

        const MIRR_UR_MAP: &'static [(Direction, Direction)] =
            &[(Down, Right), (Up, Left), (Left, Up), (Right, Down)];

        match self {
            Empty => Single(going_to),
            MirrorUL => Single(Self::lookup_dir(MIRR_UL_MAP, going_to)),
            MirrorUR => Single(Self::lookup_dir(MIRR_UR_MAP, going_to)),
            SplitterLR => {
                if going_to == Up || going_to == Down {
                    Pair(Left, Right)
                } else {
                    Single(going_to)
                }
            }
            SplitterUD => {
                if going_to == Left || going_to == Right {
                    Pair(Up, Down)
                } else {
                    Single(going_to)
                }
            }
        }
    }
}

/// (+ve down, +ve right)
#[derive(Debug, Clone, Copy)]
struct Coord(usize, usize);

impl Coord {
    fn next_coord(&self, going_to: Direction, bounds: Self) -> Option<Coord> {
        use Direction::*;
        let c = match going_to {
            Up => Self(self.0.wrapping_sub(1), self.1),
            Down => Self(self.0 + 1, self.1),
            Left => Self(self.0, self.1.wrapping_sub(1)),
            Right => Self(self.0, self.1 + 1),
        };
        if c.0 < bounds.0 && c.1 < bounds.1 {
            Some(c)
        } else {
            None
        }
    }
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

    fn push_iter(&mut self, items: impl Iterator<Item = T>) {
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

    fn get(&self, coord: Coord) -> &T {
        &self.elems[coord.0][coord.1]
    }

    fn get_mut(&mut self, coord: Coord) -> &mut T {
        &mut self.elems[coord.0][coord.1]
    }

    fn bounds(&self) -> Coord {
        if self.elems.is_empty() {
            Coord(0, 0)
        } else {
            Coord(self.elems.len(), self.elems[0].len())
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Energized {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Display for Energized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = if self.up || self.down || self.left || self.right {
            "#"
        } else {
            "."
        };
        write!(f, "{}", val)
    }
}

impl Energized {
    fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    fn set_alight(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Up => self.up = true,
            Down => self.down = true,
            Left => self.left = true,
            Right => self.right = true,
        }
    }

    fn is_alight(&self, direction: Direction) -> bool {
        use Direction::*;
        match direction {
            Up => self.up,
            Down => self.down,
            Left => self.left,
            Right => self.right,
        }
    }

    fn is_any_alight(&self) -> bool {
        self.up || self.down || self.left || self.right
    }
}
