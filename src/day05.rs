use std::{collections::BTreeMap, iter};

pub fn run() {
    let input = r##"
    seeds: 565778304 341771914 1736484943 907429186 3928647431 87620927 311881326 149873504 1588660730 119852039 1422681143 13548942 1095049712 216743334 3671387621 186617344 3055786218 213191880 2783359478 44001797

    seed-to-soil map:
    1136439539 28187015 34421000
    4130684560 3591141854 62928737
    2493176649 2843539493 216586902
    4035246184 3979580848 40675839
    784987951 2449883248 10512167
    1230114095 458474273 89127842
    3591141854 4278550666 16416630
    795500118 1007741104 49669915
    4075922023 4020256687 54762537
    1170860539 385724159 59253556
    1754134353 1447758461 710855281
    2464989634 0 28187015
    3811089926 3654070591 224156258
    367106182 564462768 34737691
    0 3060126395 64826339
    1438999297 87449756 298274403
    1319241937 901480302 106260802
    1425502739 444977715 13496558
    906091129 2158613742 230348410
    401843873 2460395415 383144078
    1737273700 547602115 16860653
    64826339 599200459 302279843
    2709763551 1057411019 390347442
    845170033 2388962152 60921096
    3607558484 4075019224 203531442
    3100110993 3124952734 265337006
    4193613297 3878226849 101353999
    3365447999 62608015 24841741
    
    soil-to-fertilizer map:
    2997768542 2385088490 141138894
    2483957796 2361581050 23507440
    98641524 1346083581 385280737
    3138907436 2256873732 8670947
    0 2158232208 98641524
    3147578383 2265544679 96036371
    1035235183 2879344429 108036359
    2567031012 2526227384 63435416
    740156227 2589662800 180702628
    2630466428 1790930094 367302114
    1029837856 0 5397327
    1143271542 5397327 1340686254
    483922261 2987380788 256233966
    2507465236 1731364318 59565776
    920858855 2770365428 108979001
    
    fertilizer-to-water map:
    1539871014 1431400479 38399903
    4189242304 3947275099 105724992
    2012473116 0 61612686
    3673653298 3769966020 177309079
    25380533 833117788 21807501
    143369400 1411638591 19761888
    2698209531 61612686 40666379
    401367210 2888296065 27849039
    3850962377 4057978463 170640183
    1076364770 854925289 39443942
    0 2048878915 25380533
    2682826842 1483785677 15382689
    4026580932 4228618646 66348650
    790899137 2074259448 70405647
    2738875910 2609016218 230235412
    2090748148 1854132037 12185242
    163131288 1499168366 238235922
    1115808712 3002097202 63461270
    545943998 1215727887 195910704
    4092929582 3673653298 96312722
    1000530579 3065558472 75834191
    2074085802 2916145104 16662346
    429216249 1737404288 116727749
    1578270917 2174814019 434202199
    2969111322 102279065 44844471
    1179269982 669063687 164054101
    2463111278 507301424 161762263
    741854702 2839251630 49044435
    3013955793 894369231 236513741
    861304784 2144665095 30148924
    1525885719 1469800382 13985295
    132032949 2932807450 11336451
    2102933390 147123536 360177888
    47188034 1130882972 84844915
    4021602560 4053000091 4978372
    2624873541 2944143901 57953301
    891453708 3141392663 109076871
    1343324083 1866317279 182561636
    
    water-to-light map:
    1509583382 1639808290 20361832
    3841220400 2799952377 116887408
    1472887638 3349716751 36695744
    1375316591 4197396249 97571047
    1030032900 38536653 44339012
    3776233310 1557050237 64987090
    1857053855 3386412495 71799907
    2963593546 2694182899 38493443
    3758462347 1622037327 17770963
    1018869652 82875665 11163248
    1308040556 2732676342 67276035
    1928853762 3953749938 243646311
    2488961036 3503789336 239267964
    3562290347 3458212402 6096433
    3568386780 1308040556 190075567
    2728229000 1997029610 235364546
    215668494 0 38536653
    1646361217 3743057300 109790942
    1529945214 3233300748 116416003
    1756152159 3852848242 100901696
    3958107808 1660170122 336859488
    3503356233 1498116123 58934114
    254205147 552157437 522214475
    3002086989 3464308835 39480501
    776419622 94038913 242450030
    2172500073 2916839785 316460963
    0 336488943 215668494
    3041567490 2232394156 461788743
    
    light-to-temperature map:
    3498288578 2645051323 42074132
    608593503 673232568 65024140
    0 1287033796 108723708
    3979313387 3634135302 315653909
    2652759587 3018896130 103365881
    1093544955 942695289 7961217
    2756125468 3501628238 132507064
    419683046 1625547778 126722533
    683243352 349510049 26140330
    1101506172 511314709 142580382
    1283347805 375650379 135664330
    673617643 24016268 9625709
    709383682 1238532395 48501401
    3763762608 2402322728 184668443
    3948431051 2621479653 23571670
    3660408219 2687125455 102463666
    3561232935 2586991171 34488482
    2459767392 3122262011 192992195
    1244086554 1754618583 31645052
    1073753155 1497748002 19791800
    3972002721 3949789211 7310666
    3540362710 4077337854 20870225
    1419012135 738256708 79984976
    1746461061 1457945428 39802574
    3595721417 4190079809 64686802
    1616767336 653895091 19337477
    1275731606 823578131 7616199
    108723708 818241684 5336447
    546405579 1395757504 62187924
    1498997111 950656506 117770225
    2888632532 3957099877 102103275
    1744112789 1752270311 2348272
    2233819388 2077946837 225948004
    3762871885 2401432005 890723
    2136282224 2303894841 97537164
    3311914546 3315254206 186374032
    2118147522 4059203152 18134702
    308182087 831194330 111500959
    114060155 0 24016268
    2077946837 4254766611 40200685
    971525741 33641977 102227414
    1636104813 1517539802 108007976
    138076423 1068426731 170105664
    3220042816 4098208079 91871730
    2990735807 2789589121 229307009
    757885083 135869391 213640658
    
    temperature-to-humidity map:
    1130946446 972737563 146373650
    1277320096 1760175559 41760032
    4151385320 4147641404 143581976
    2634337722 0 466605084
    1992884166 956487184 16250379
    4147641404 4291223380 3743916
    641064346 466605084 489882100
    1319080128 1801935591 673804038
    2009134545 2475739629 625203177
    0 1119111213 641064346
    
    humidity-to-location map:
    3903940466 3635148971 125939893
    1458128760 2186815403 67353660
    3125319983 1458128760 728686643
    2261072201 3994982121 66012689
    3854006626 2992363154 49933840
    1525482420 3780550419 183145699
    2233668127 3967578047 27404074
    2442260515 3138011064 466023456
    740912129 0 327948845
    2422798960 3761088864 19461555
    1708628119 3963696118 3881929
    2327084890 3042296994 95714070
    4029880359 3604034520 31114451
    1712510048 2471205075 521158079
    367508399 695457244 373403730
    2908283971 2254169063 217036012
    0 327948845 367508399
    "##;
    let lines = read_lines(input);
    // println!("{:#?}", lines);

    let seed_data = parse_data(&lines);
    // println!("{:#?}", seed_data);

    for i in 30..110 {
        println!(
            "{} -> {}",
            i,
            seed_data.get_data(SEED_SOIL).convert_number(i)
        );
    }

    let min_value = seed_data.seeds.iter().map(|seed| {
        let mut cur_value = *seed;
        for mapper in seed_data.maps.iter() {
            cur_value = mapper.convert_number(cur_value);
        }
        println!("Seed: {}, final: {}", seed, cur_value);
        cur_value
    }).min().unwrap();

    println!("Min value, part 1: {}", min_value);

}

fn read_lines(txt: &str) -> Vec<String> {
    txt.lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}

const SEED_SOIL: MapType = MapType(0);
const SOIL_FERT: MapType = MapType(1);
const FERT_WATER: MapType = MapType(2);
const WATER_LIGHT: MapType = MapType(3);
const LIGHT_TEMP: MapType = MapType(4);
const TEMP_HUM: MapType = MapType(5);
const HUM_LOC: MapType = MapType(6);

const NUMBER_OF_MAPS: usize = 7;

#[derive(Debug, Clone, Copy)]
struct MapType(usize);

/// Usage example:
///
/// let x = ParseStates::GetNumbers(SOIL_FERT);
#[derive(Debug)]
enum ParseStates {
    FindSection,
    GetNumbers(MapType),
}

fn parse_data(lines: &Vec<String>) -> SeedData {
    let mut seed_data = SeedData::new();
    let mut parse_state = ParseStates::FindSection;
    for line in lines {
        let nums = read_nums(line);
        if line.starts_with("seeds:") {
            seed_data.seeds.extend(nums.iter());
        } else if nums.is_empty() {
            parse_state = section_heading_state(line);
        } else {
            if let ParseStates::GetNumbers(map_type) = parse_state {
                seed_data.get_data_mut(map_type).insert(nums);
            }
        }
    }
    seed_data
}

fn section_heading_state(txt: &str) -> ParseStates {
    if txt.is_empty() {
        return ParseStates::FindSection;
    }

    if txt.starts_with("seed-to-soil map:") {
        ParseStates::GetNumbers(SEED_SOIL)
    } else if txt.starts_with("soil-to-fertilizer map:") {
        ParseStates::GetNumbers(SOIL_FERT)
    } else if txt.starts_with("fertilizer-to-water map:") {
        ParseStates::GetNumbers(FERT_WATER)
    } else if txt.starts_with("water-to-light map:") {
        ParseStates::GetNumbers(WATER_LIGHT)
    } else if txt.starts_with("light-to-temperature map:") {
        ParseStates::GetNumbers(LIGHT_TEMP)
    } else if txt.starts_with("temperature-to-humidity map:") {
        ParseStates::GetNumbers(TEMP_HUM)
    } else if txt.starts_with("humidity-to-location map:") {
        ParseStates::GetNumbers(HUM_LOC)
    } else {
        panic!("Unknown Section: {}", txt);
    }
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

#[derive(Debug)]
struct SeedData {
    seeds: Vec<i64>,
    maps: Vec<MapFinder>,
}

impl SeedData {
    fn new() -> SeedData {
        SeedData {
            seeds: Vec::new(),
            maps: iter::repeat_with(MapFinder::new)
                .take(NUMBER_OF_MAPS)
                .collect(),
        }
    }

    fn get_data(&self, map_type: MapType) -> &MapFinder {
        &self.maps[map_type.0]
    }

    fn get_data_mut(&mut self, map_type: MapType) -> &mut MapFinder {
        &mut self.maps[map_type.0]
    }

}

/// Range_len is the range length which is same for both source and dest.
///
/// Dest = Source + Offset
#[derive(Debug)]
struct MapNode {
    range_len: i64,
    offset: i64,
}

#[derive(Debug)]
struct MapFinder {
    btree: BTreeMap<i64, MapNode>,
}

impl MapFinder {
    fn new() -> MapFinder {
        MapFinder {
            btree: BTreeMap::new(),
        }
    }

    /// insert dest range, source range and length into data structure
    fn insert(&mut self, nums: Vec<i64>) {
        if nums.len() != 3 {
            panic!("Expect 3 nums: {:#?}", nums);
        }
        let source_range_start = nums[1];
        let offset = nums[0] - nums[1];
        let range_len = nums[2];
        self.btree
            .insert(source_range_start, MapNode { range_len, offset });
    }

    /// Get the closest number that is less than or equal to the given number
    fn get_nearest_below(&self, n: i64) -> Option<(&i64, &MapNode)> {
        self.btree.range(..n + 1).next_back()
    }

    fn convert_number(&self, n: i64) -> i64 {
        match self.get_nearest_below(n) {
            Some((lower_bound, map_node)) => {
                let upper_bound = lower_bound + map_node.range_len;
                if n < upper_bound {
                    n + map_node.offset
                } else {
                    n
                }
            }
            None => n,
        }
    }
}
