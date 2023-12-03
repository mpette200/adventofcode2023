use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Peekable;
use std::ops::Range;
use std::str::Chars;

pub fn run() {
    let input = r##"
    .........798...145.........629....579.....455.....................130.............243.................154........167........................
    ............*.....*...........*...&...179.*........737...194.........*854........./...........52..560*............................699...&...
    ........459..489.817........880.........*..996........*....*........................................................................*.36....
    ...........@.........................813............234.552..307....184............370..................736.....960..............631........
    ...100...................*...............131..................*........=......435..*......34...........*....................................
    ......+....#126.......214..........$......*.....+.............939................/..729..............861.....243..........438...............
    ...................................854....979....177.......%.........280..138................158*241............*..........*....*904...#....
    .......427............................................683&.726...303........*.......905.......................&..115.....412.479.....491....
    .....=.*...........989......888.....................$............../.226....922.172....*..702*693...........543.............................
    ..924..219...........*.....*........979............191.%.........+..............*.....323....................................777............
    ....................109........................559.....835.....708........989..685................................#.............=.915.......
    ........................................*........=.........751........886................243.......154+..922...347................/.........
    ......923...............571.....@.672....963.471.......154...*.........*.....................914...........*.............891.........73*212.
    .........*......654+....&....309...=.........*....543.........583....871.....*........................551...370...$......*...492............
    ..618*....60.........................426...&.645....*....................203..310....$...........@166...........477...@.......%......150*...
    ......353......381..........416.....*....19......536......#..312..../790.*............779..$.........../............985..................761
    ...........#............636..*....238...................576.....*........899................460.841..829.415...271*.......168.77....=.......
    ..........427............$.......................................75..............789...942........-......*.........795......*.......73......
    ....863................%.......920..........*152.....852*387.........990.....563..*.....*...629........772....912............283............
    837*.......897.......674...966....&.%621..77..................................&..571...272.........566.....16*.....936.....&......980.......
    ..............*............*....-.........................*..............424#...................53.*......................203...............
    ...826.781.....105......&...771.982............967......40.223.....................71............*..123.........-...@....................998
    ...$................64.562.............../382....*.................866-....276.....*........750.975...........878..156..........188.........
    ...........38+..............937...................142.........*712........$.....619..*734............424..................=.................
    ....746...................@....+..868..-......881...........................+.........................*.......832.../....649...-.......181..
    ...*.......+637...478....628......%.....651....................../.....399%..396.............&..293.472..........*.960..........988....*....
    ....693..........$..................-.......711.99.489.878....541...........................565..#.........32..710......499............888..
    ....................681.59.442.....368.%.........*.......&..........................974............*........%...............................
    677....924......797.......*............964......666.................89......309.......*.........324.179..........437.......235.321...678....
    ............+..........$......5...............$......./......23..............$.........328..564.........392......*............*........*....
    ...298......811.........976..*.................400...140....*.......893................................*....%.682.....345.............726...
    ..............................570.839......................828.....*......-......59....373.......%...558..271...........*......*751.........
    .....570...........353*674........#....636........659..............647.....226..*.......+......684...............139...290..401........290..
    ........&..................231...............80..........@112....................892......605...............247...+................$...*....
    165........*329..94*254..........115...............938.................192...............*......$58.............@......786.....328..59..285.
    .........98........................$.....895......*...............459..*...............399..641..................285...$..........*.........
    .............../..455*429............794*.....................512.*....770.....448...........@...................................336.175....
    ......#......277................................763............*..254.....................*....143...495..$.......676...................#...
    .......272.........-..................711..911...*.....614..516............747.389.....803........%.....*.832.......*....905....576..#......
    625.................595......989...............291.....*................*.....*............$..201.............528.722....%.....*......896...
    ....342............................................569..486.......@..278.184.........259..107................%.................749..........
    ...#..................851.......16..436*910.......@..............338...........989...*..........+.367.867............=..918.................
    .................106..#.........@...........................@.....................-...99......38....%....*.18&....550..*..........250.......
    ..................+.........................&732....53....258....850..................................306.............539....@..............
    658*869..............684*636....422.686............*...............*...955..............*......................../...........313.....$......
    .............604.............................954....357.........458..................168.310.......698............435......%......394.......
    .....109....*.........163...........962$.....@..........=..................876*........................230..125...........24..972.....641...
    ......*...312..........#........................*914....740....466*............374.............+......%....*...........46....*.....60.......
    ...422............804......$.@......%........589...................500..958.............21..301...........889............*..717.............
    ..........474..=.....*...302.475.155.............665..800..46..............=...................................19......902........936...887.
    ............*.754...831...............8@.@............................110.....554..........732...=913.........*.....$............*..........
    456......211.................562.........183........................@......+..*.....839...............@...500.739..962.....105...528..-.....
    ...............794.882.......+.....@..........833............595....704.693......=...*..............81...*.............571*............727..
    ......590.30......*.................872...@...................................773.....156......&346......807.813.285+.........380...........
    ........$..*...........*621.....528.....835.....556.......*85...516-.......................28.........*......-..........791$.....%..........
    ....................965.....507.*.............*....*...377.............957................@....835.707.409.....987*...........=.......729...
    .....503......471..........*....267..169....200.369..........499..#...*....................................161.....464.....794.....*...#....
    ........$.......*..........808......*....&..........643.....*....39....354......159........117@...308....-...*.........144.........452......
    ..........$......910...........195..952..320.........*......832.................*...296.............*..860..401.........%....157/...........
    .......438..721................@..............89..144..332........987.........979..*.....%.........824............709................710....
    ...209.......-........../............=952..............*.............*292.........582.661.....$........815.............857....254....*......
    ....&..218%..........100................................74....................*................692.-.......545...................*..........
    .............297..........101....276.756..............#....462......492*578..636......=............860..........................399.........
    ........171..............=........#..*...............407...%.......................331..915..............................919................
    ....454*...................*83......100..702..............................271...........@...........59....582.../........*...........+......
    ..............185*......687...........................186............739-...=......*.................+....*.....380..539..192.80......455...
    .......174........686.........562...958......192.......*..117...108.............567.298.................545...........*...........$.........
    .....$.....887........131..........-....955...*.....504...........%.243..281........................877.............610......216..130...-...
    .....646....*......76....................%..50.......................*.......541.......782.....752+....*......296*.......523...@......881...
    ............802....*........976.................223*478........96...785.142.*......530....*721.........577........317.......*.....295.......
    ....412.114........561...#....+............................52....*......*....999......*./....................................221.....*40....
    ......*./......*.........435..........168..779....-..........*.272.....372.........652...489........*988.......810..........................
    80.110.......137....938...........924....#.+...896........305....................................885...........................831..........
    ........&...............792.........................................773*100.958..&78.....*..............902....286................+.........
    ........873..............&...........585..................127*899..............*.......60.982...........$......@....268..370.............567
    ............781...*...........*174...*...............................26........203.652................................*............538......
    .............$.....534.....848......170......418........494/....298...*....792......./.......%......+687..............179............@.160..
    ..474...711....................417.................................@.458....$..........*338..719............509..488......781.982...........
    .....*..........235.................622............$...804+..............*......577.870.............&..43.....&..-.............*.....278....
    ....696.222.604.*.......*29...........$.....&....755...........55.....631.202.....................443....*....................692...*.......
    ...........*....55...456.....782..422.......133......844........*..........................286-...........865.89..27...718........317.......
    .......611.....................-...*..995........712..*..........89...............@166..............771............*.....*..............589.
    ...........417.....150..678...........*.............$.417..298.............611&..........811.327......*..$...610%.614.....443......-...*....
    .......658*..........-...*.......622..940.............................&....................*../..773.194.479..........996.........263..872..
    ....90......%550.587......215.......*.......322.....229......841*....996...........826....305......@.........#405.....$.....................
    ......*...........*..................671....*......*.............695.......694....+..............................................223.462.826
    ..419.832.......271...828................228.................380........21*..........644............725....703......255..330........*.......
    .....................*.............................357.................................*........360...+.....#..%173...*..../../.............
    ..................182...........88.664*......865..*................................428.................................143...829.552.....829
    ...&..................425..894...*.....792...*.....321..219................=...564....*..166....%....+.....764...................*......=...
    ..619.....529..384.....*......*...693.........188......-............281.238......=.339....*...124...278.....+....490.....481....416.........
    .................*.442..76....801.....376.................+511.......*......*.............967.......................*.68*............919....
    ...731..484....875..#....................*243...681.................135..191.162.688%.........29.....*............688..........437......*...
    .....%.=........................329*87.............*138.-949..186.................................595.554...............755....*.....454....
    ............341..302.........&...........330.....................@.......................779..............353......@.......*..535...........
    .....766.....*..*..........315......856.*................540..............136............*.......914..162*.......7..336..383............642.
    ......../..253...502................*...990.....299.......+...413............*..........338.......%.............................294*........
    .....&..................636.986.431.345...............693.......*.......*...............................276..........836@.278.......716..469
    .....247........*.905*....$..*...*.......................$.....502...177.........72=......$...232......*................................*...
    .........674.905......664...124............*127.............................564........924.....*.....&.616...661..403....10.........429.724.
    ........../..............................78..............#5..................*................498..650..........*.......*............*......
    .....348....*.........488...........414..........................962...552...107...@...533......................109....432....*......423....
    ....*.......536................283....&......800.............&.......+..../........677.$..................32...............410.392..........
    ....826.............755........*........396..*......../.......771.....837........=.............577.................53...............314.....
    .........@....109$...*..........988........$..241..750.............@............169...162......*...%....575.743......*..................278.
    ..........862........227....585........158.............262.....51..151..................*..509...31............*..........+....144.481.&....
    ........*........467........*.........#..../..........*......................%....973..218..*..........@.......543......678.......*.........
    ......32.106...$...*......260...........425.........912.................383..392.*...........503....546..212........%......./442.....221....
    ..............668...593............422.......966*37................503..*.........776.....5$..............*......833.................&......
    ............-...................21....*.-497............................133...........862......241.............-..........824...............
    .........215.......................851.........-........932........@./.......-.......-....%..................597..........*........@........
    ................*323....557....782..........964...14..$....$.....631.144.....588.........316......................252..396........257...957.
    .............367........*......*.....64.97........=..976.....245................../982..........135...169........*.....................=....
    ....513..........96.101..359.212.585..............................965*287.803.501.........460.........*.........618....98.208...............
    .............802*...+............#..............804..123...579*..............*.............*.......282.....263...........*..................
    .707....522....................................*........@......106..143.........716......770...........576*......................497........
    ......#.../...=.........835............545*610.462..188............*.............*.....$...............................*115......*....*.....
    928%.606.......207.......*......61..........................797.....228..357....484..492..........247...............574..........880.994....
    ...........541...........444......@....122......................505......+...14..........534........*.....210...........150.948..........210
    ..............*..676*765...............*.....295.156...601#....*......-.......*.....709..........901.....*........714......*............&...
    .......723...299................325...965...........*.........656......156....824...../..............840.383...........249..........=.......
    ..................................#........26*781.456................#.....................745......*.............165.*..........278..452...
    .746..............953.943.............................877.....59......128............#....+......585......641....%.....229...*2........*....
    ...*.......614=......*.......903.............433.370...=...54....+..............&.....336.....$.............*..........................856..
    698...272.....................%......545.....-....$.............223.183..........339........189............994.......531.815*206............
    ........./................657...........#...............571............=..-832.+....................876..............................754....
    .....&.....*960....628.............597.............747$.*......776..+..........709..................*..........266..........225.....*.......
    ...541..511.............353.......*......................649.....*..753...719......212..526=...578..634..........*..........*......113......
    ................996=...*...........18........#628..............321........*.......*...........*...................194....334.............126
    ......................627..693.........248..........290.................@..204..=..247...574..311......791....................679*..........
    .......@....................=......283*............*.............38..214........90.........*.......447...+.............77...%.....265.......
    ......566......&.................*..........965...929...........*......................869.908.753..$.......437....848*.....943.............
    ...........403.950..............439............*............340.419...........868.....*...........*...128.....&.....................869.....
    ............*..........637...............256.793........436*....................*.....85.361.....280..................349...................
    .........654...........*..............=.-........949............=....224..506.631....................*21.@...........%......................
    .............#..44.......105.108...190..........*....$173......750....*...../.........37.......-..461.....727...75.......................893
    ......@...583.....*........*................358...........750........................*......532..................*...22...../....512...#....
    863...112................178...+...........*...........5./............98......584..222..........862...235.....448...*.....737.....*.....516.
    ....#.......425..............923.84*......947......999*..............*....280*...........732...&.....*.............14.............683.......
    .....353............................914........105.................829...........112...............75.......................................
    "##;
    let lines = read_lines(input);
    // println!("{:#?}", lines);

    let digit_coords: Vec<Digit> = get_digit_coords(&lines);
    // println!("{:#?}", digit_coords);

    let symbol_coords: HashSet<Coord> = get_symbol_coords(&lines);
    // println!("{:#?}", symbol_coords);

    let valid_nums: Vec<i32> = digit_coords.iter()
        .filter(|d| is_adjacent_to_symbol(&symbol_coords, d))
        .map(|d| d.val)
        .collect();
    println!("{:#?}", valid_nums);

    let total: i32 = valid_nums.iter().sum();
    println!("total: {}", total);
}

fn read_lines(txt: &str) -> Vec<String> {
    txt.lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Digit {
    xpos: usize,
    ypos: usize,
    val: i32,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

fn is_adjacent_to_symbol(symbol_coords: &HashSet<Coord>, digit: &Digit) -> bool {
    let xr = Range { start: min_limit(digit.xpos), end: digit.xpos + digit.len + 1 };
    let yr = Range { start: min_limit(digit.ypos), end: digit.ypos + 2 };
    xr.flat_map(|x| yr.clone()
            .map(move |y| Coord { x, y })
        ).any(|coord| symbol_coords.contains(&coord))
}

fn min_limit(val: usize) -> usize {
    if val == 0 {
        val
    } else {
        val - 1
    }
}

fn get_digit_coords(lines: &Vec<String>) -> Vec<Digit> {
    Range {
        start: 0,
        end: lines.len(),
    }
    .into_iter()
    .flat_map(|i| parse_digits(&lines[i], i))
    .collect()
}

/// Parse the digits into numbers whilst tracking the
/// coordinates (xpos, ypos) of the first digit
fn parse_digits(txt: &str, ypos: usize) -> Vec<Digit> {
    let mut out: Vec<Digit> = Vec::new();

    let mut c_iter = txt.chars();
    let mut items = c_iter.by_ref().peekable();
    let mut i = 0;
    loop {
        i += skip_items(&mut items, |x| !x.is_ascii_digit());
        let idx = i;

        let digits: String = items
            .borrow_mut()
            .inspect(|_| i += 1)
            .take_while(|x| x.is_ascii_digit())
            .collect();

        if digits.is_empty() {
            break;
        }

        // println!("Digits: {}, [{}]", digits, idx);
        out.push(Digit {
            xpos: idx,
            ypos,
            val: digits.parse().unwrap(),
            len: digits.len(),
        });
    }
    out
}

/// similar to skip_while(|x| some_bool)
/// but useful when necessary to track the number of items skipped
fn skip_items<F>(items: &mut Peekable<&mut Chars<'_>>, mut predicate: F) -> usize
where
    F: FnMut(&char) -> bool,
{
    let mut i: usize = 0;
    loop {
        match items.peek() {
            Some(item) => {
                if predicate(item) {
                    i += 1;
                    items.next();
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    i
}

fn get_symbol_coords(lines: &Vec<String>) -> HashSet<Coord> {
    Range {
        start: 0,
        end: lines.len(),
    }
    .into_iter()
    .flat_map(|i| parse_symbols(&lines[i], i))
    .collect()
}

/// Parse the coordinate positions of the symbols
fn parse_symbols(txt: &str, ypos: usize) -> Vec<Coord> {
    let mut out: Vec<Coord> = Vec::new();

    let mut c_iter = txt.chars();
    let mut items = c_iter.by_ref().peekable();
    let mut i = 0;
    loop {
        i += skip_items(&mut items, |x| !is_symbol(x));

        if let None = items.peek() {
            break;
        }

        out.push(Coord { x: i, y: ypos });
        i += skip_items(&mut items, |x| is_symbol(x));
    }
    out
}

fn is_symbol(x: &char) -> bool {
    !x.is_ascii_digit() && *x != '.'
}
