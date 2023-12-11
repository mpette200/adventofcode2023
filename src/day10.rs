use std::fmt::Display;

pub fn run() {
    let input = r##"
    .-FL-L777F-F.7-LJF-7.F|7F7.FF---7-F-L|-F-JJ-LL.J-F7.L-LJFF-LFFJ7.FF-F|FFFJ77-F-LF|--7-FL.JJ.F-F7|--F77|JJFLF-|-F-F-7-7-77F-7FF.FF7F7F--|.7.F
    --|J.F-7-|7F|7-|FF-J-7L|JL--.LF|.7J||F77|7.FL-7-77|--L|.F7FLJ--F--7.LL-7|LFL7-FJL|7|J.F|7LJ7..LJJ..LLJJ..F.L7J77LL7L-F-LL|J-J.FFLL7|JJLL7|-|
    |.F-7-L..J-JJL7.||..L|--JF7...-FLJ7|L|J|777.|--J|LJF7--LLJ77.|7L77|F||7|-|L|J.|L|LJJJ7LJ-..FL|FJF7F|.F|L7|FLJ7FLJL|J.--|.|.7FJ-L7.FF.|F|7.FJ
    7J.-J7.FFLF--.-FJ7|--J7--F|77L-|J7|..F.L7-7.J.|J|.F-F7JL-7.F7FLL|-|-JL-JFFF7J-|7F7L7.7L|LLF7||L7||L7.-J|FL..|-||J.LF7|F.F--J|J77|-JLFL|LJF-7
    JJ.FL7|77.L7|-.FJ-|-F.|7J|.FJLFJ7-|.7LJF|.7JJ.L.LFJFL|LLFJ7|.L7||FJ7|7|7L7J|..LFJ|F7-F.L7LL|L7L7L|L|FFL7|L--JLL7.FFFJ-L-.---JLL---7FJ7L|FJ.|
    L7-|JFF77J--|.L|--JFF7LJF--7|LL-|.LF|LF-7.LL77|FLJ-JFF7.-|JF7JL7-7.LJF|7J.L.FL7JFJFL7LL.-7J|-J||-JJLL-.|LF||.FF--FJ|7.|F-.F7J.|L7||7F-7|FJ-7
    F|J|F|LFJ..F|JJ.7FJ-F7FJ7JLJJ-|-7-L|LL||F77.J.FF7..L|JLF-7-7JJ-7-F7|FFJ77FF-JF|.|||FFJ|7L|-JJ.FJ-L-7J|-F-7F--7JJ||FF-77|LJ|L.|J7L-FJF|FJJF7J
    7-FJF7---77F7.F|J|JF||LJL-.||.FFF7.F7F-7||J7L7JLL-.L|7FF7F7J|77.FJ77-JLL-777---FJJ|FJ-L77.J|LF7L||--F-JL7|L7FJ|.F--JFJ-7JLJ77|-JJFLF7.F--J-.
    |JJL-|-JF|FL|-FJF-77||7-JL|J.FFJ|L-JLJFJ||.|7|J7JJ.F---J|||7-|7FJL||7|L|-|L7..L||FLJ7F|FJ--FFF7.F7J|J|F-JL-J|7F7L-7FJJFJ7FJJFJFL-7.LL7|77|FJ
    L|-7FJJ.7JFJJ|L.JL|-L-L7.F77LF7LL----7L-J|F7F77L7F-L---7|||J..L-F77F77FF.|-7.---L7L|L7-7J.FF7||F|F77LF|F---7L7||F7||7J||LJFL-.|7JFF7-LJLLL7|
    L7LL-.FJJ-|7FL-77F77.||7F|L7|||F7F---JF--J||||7FF7J.|-FJ|||F-7J.F--J|F7F7.|.|.LL.-.L7|FJJ7FJLJ|F-J|F-7LJF7L|FJ|LJ||L---7||LJLFL7JL7JL|F--LJ-
    LL7FJJ|J|-FLJJ..|-JFF-LL-L7|FJ|||L---7|F7||||L7FJ|JFF-JFJ|LJFJ7FL--7||LJ|7-FJ..LF-J|7-FJ.LL--7|L-7||FJF-J|FJL7L-7LJF7F-JJJJ--7|FJ|J|.FLJJF7J
    F--7JF-FF-FJL-F.|.F|JJ7.LF|LJFJ||F--7|||L7||L7||FJ7LL-7|FJF-J.7FF--J|L7FJJ-F7F7LF7L77-JJ.|F77|L-7|LJL7L-7|L7FJF-JF-JLJ.|7|-7JJL7-L7L7F7.LLL-
    JFL--J-LL7L77JF-|F7J-FJ-F-JF7L7||L-7||||FJ|L7||||F7F-7|LJFJF-7--L--7L7||J.FJLJ|.|F7F7.|JF-JL7L7FJ|F--JF-JL-JL7|F-JF-7F7LJ77|7FL-7FJJ-L-7L|.|
    |7.FJJ7.|.FL|-F--JL7-L7LL--JL7LJ|F7||||||7|FJ||||||L7LJF-J.|FJ7JJF-JFJ|L-7L-7FJFFJ|||.F7L-7FJFJL7||F7JL-----7LJL--JFJ||7LF||-|77L-J|J.|JLLF|
    7.-JFF|-JF7J|LL--7FJ-FF7L.F-7L-7||LJLJLJ|FJ|.|||LJL7L-7L-7FJL7|-FL-7L7|F-JF-JL-7|FJ||FJL7FJL7L-7LJ|||F7JF--7|F-----JF|L7.|LL.L-7JJ.77-F-F77|
    L7..J--JF77-J.L|FJL7F7F7-FL7L7FJ||F7FS-7LJFJFJ|L--7|-FJF-JL7FJF-7F7|FJ||F7L7F--J|L-J|L7FJL-7|F-JF7||LJL7|F7LJL-----7FJFJ-FJ|F7|.F--LL-|F-7|7
    FJ7.JJ|.FL--.7LFJF7||LJL7F7L7||FJLJLJF7L7FJFJFJ|F-JL7L7L-7FJL7L7||LJ|J|||L7|L7F7L--7L-J|F-7||L7FJLJL7F-JLJL7F------J|FJF7J7FLL77|F7JF-F|JLJ7
    |L777L-L777F-J-L7|LJL--7|||F|LJ|F7F7FJL-JL7L7|F7L7F7L7L7FJ|F-JFJ|L-7|FJLJFJ|FJ|||F7L--7|L7|||FJL-7F-J|F7-F7|L7F7F7F7|L-JL7-LJ7F-L-7-J.-|7FJ7
    L7|.7J7L|F-L|FF7|L-7F7L|LJL7L-7|||||L--7F7L7|||L7LJL7L7||FJ|F7|FJF7|LJF-7L7||FJL7||LF7||FJLJ|L7F-JL-7LJ|FJ||FJ|LJLJ||F---J.FLJ|7F|.F|FF-7-L7
    F|J7.F7-L7.FFFJLJF-J|L7L7F-JF7||||||F-7LJ|FJ||L7L7F7|FJ|||FJ|LJ|FJLJF-J-L7|||L-7||L7||||L--7|7|||F-7L-7|L7|||FJF7F-J||F7LF777LL-77--JJ--|7.J
    FJJ.|.J7FL|7JL7F7L7LL7|.|L-7|LJLJ||||FJ|FJL7|L7|FJ|LJ|FJLJL7L-7|L--7L-7F7LJ||.FJ|L7LJ|||F7-||FJL7|FJF7|L7|LJ||FJLJF7|||L-JL-7.|.|JL-F|J7F|7J
    |FL7F77LF-F77.LJL7|F7|L7|F7|L-7F-J|LJ|F7L7FJ|FJ|L7L-7|L---7|F7||F-7|F-J|L7FJL7|FJFL-7LJLJL7||L-7|||7|LJFJL7FJ||F7J|LJ||F-7F7|-L-L7.FJ|FFJ7-7
    |LF|--J..||L7-F--JLJ|L7|LJ||LFJ|F7L-7|||FJL7|L-JFJ|FJ|F7F7|||||||FJ||F7L7||F-J||-F7|L----7||L77|LJL7L-7|7FJL7|||L-JF7|||JLJLJ|.||JFLL7F-J|JF
    |..L|JJ7.-L7|.L----7|.||F-JL7L7|||7FJLJ|L-7||F--JF7L7||||||||LJ|||FJ||L-JLJ|F7|L7|L7F7F7FJ||FJFJF--JF7|L7L-7LJLJF--JLJ|L-7F7|F|-JF-|||J|.--J
    |7FFJLFJ-LFJ|F7FF--JL-J|L--7|FJ||L7L--7L--JLJL-7FJL7|LJ||LJ||F-J|||FJL---7FJ||L7LJFJ||||L7||L-JFJ.F7||L7L7-|F--7L77F7-|F-J|L7J|--L7FJJLF-L7|
    .-F7-FJ|.LL7LJL-JF7F7F7|-F-J|L7|L7|F7JL---7F---JL7FJ|F-JL-7|||F7||||F7F7FJL7||FJF-J|||||FJ|L-7FJ|FJ||L7L7|FJL-7L7L-JL-J|F-JFJ7...|7F-7-|.FF7
    7.L77F7-L-J|F7F--JLJLJ|L7|F7L7||FJ|||F-7-FJL-7F7FJL7|L-7F-JLJ||LJ||||LJ|L7FJ|||FJF7FJ||||FJF-JL-7L7|L7|-|||F--J7L-----7||F-JJF7JJ.FJ-JL|.FL-
    L7FJ-7F-J--LJLJF7F--7FJFJLJL7LJ|L7|||L7|FJF--J|||F-J|F-JL---7||F-J|||F-JFJL7|||L7||L7||||L7L7F--JFJ|FJL7|LJL---7F7F7F7LJ||F77||F|-J|F7FLFJJ.
    ||JF7J|JL7.FF--J|L-7|L7L7F-7L-7L-J||L7|LJFJLF7|LJL-7|L-7JF77|LJ|F7||||F7L7FJ|||FJ||FJLJ||FJFJL7F7|FJL-7|L7F----J|||LJL7FJLJL7||7JJLFLJ7-FJF|
    7|-L-7|.J-LLL7F7L--JL-JFJ|FJF7L--7LJFJL-7L-7||L-7F-J|F-JFJL7L-7||LJLJ|||FJL7|LJ|FJ|L--7LJL-JF-J||||F7FJL7||F7LF7|||F--JL7F--J|L7|77|-L77|FF|
    .|7JFL77L|LL.LJL--7F7F7L7||FJ|-F7L-7L7F-JF-J|L7||||FJL7FJF-J7FJLJF7F-J||L-7||F-J|FJF7FJF----JFFJ|||||L-7LJ|||FJLJLJL---7|L--7|FJFF|LF-7FL7.L
    -LJ--J||F-77FF---7||LJL7LJ|L7L-JL--JFJL-7|F7L7L-JL7L7FJ|FJFF7L7F-JLJF-JL7F||||F7||J||L7L-7F7F7L7||||L7L|F-J|LJF7F------J|F--J||F7F--JFJJL777
    .FL-L7LL|-|-FJF-7|LJF7FJF7L7L-7F7F-7|F7FJ||L7L--7FJ-||FJ|F7||FJL-7F7L--7|FJLJ||||L7||FJF-J|LJ|FJ|||L7L7|L-7|F-JLJFF-7F--JL---JLJLJF--JJ.7|FL
    7--|-L7|JL|.L7|FJL--JLJFJL7L--J|||FJ|||L7||FJF7|||F7||L7|||||L7F-J||F7-||L-7FJ||L7|||L7|F7|F-JL7||||L7||F-J||F7|F7L7||F----------7|-JJJ7|L-J
    FJF7F|-7.L|FLLJL-------JLFJF-7FJLJL7LJL7||||FJL7|||||L-J||||L7||F7|||L7||F7|L7||FJ||L7||||||F7FJLJL7FJ||L--JLJL7|L-J||L-------7F7LJ.||-FL7|7
    L77L-J7F7FFFF7F--7F-7F--7L-JFJ|F---JLF-J|LJ||7FJ|LJ|L-7FJ|||FJ|||||LJFJ||||L7|||L7||FJ|||||LJ||F---J|-||F--7F-7LJF--J|F----7F-J||F7-|J-7LJL|
    LF|-LL7LJFF7||L-7|L7|L7FJF--JFJL-7F-7L7FJF-JL7|FJF-JF7|L7||||FJ||||F7L7|LJ|FJLJ|FJ|||FJ|||L7|LJL7F-7|FJ|L7FJL7|F7|F--JL7JF7LJF7|LJL7LF-7L|.|
    JJ..7LJ7F7|||L-7|L-JL7||LL7F-JF7-LJFJ7||FJF7FJ|L7L7FJLJFJ|||||FJ||||L-JL7F|L-7FJ|FJ|||7||L7L--7-||FJ|L7|F|L-7||||LJF-7FJFJL--JLJF-7L7|FJF7-7
    ||L||LLFJLJ|L-7|L---7LJL-7||F7||F7-L-7||L7||L7L7L7||F7FJFJLJ|||FJLJL-7F7|FJF7|L7|L7|||FJL7L7F7L7LJL7|7||FJF7LJLJL7FJ|LJ7L7F-7F7FJJL7LJ|.7L7-
    F7-|FF7L-7FJF7|L7F7LL--7FJLJ||||||F--J||FJ||FJFJFJ|LJ|L7|F--J|||FF---J|LJ|FJLJJLJFJ|LJL7FJFJ|L7L7F7LJFJ|L-J|F7F-7|L-7LF7FJL7LJLJ7J7L-7L7.|J7
    -7FF7J7FFJL-J|L7LJL----JL7F-JLJLJ|L--7|||FJ|L7|FJ|L7FJFJ|L7F7||L7L-7F7L-7LJLF--7FL7|F--JL7L-J|L7LJ|F-JFJ-F7LJLJFJL--JFJLJF7L7F77F7F-7|FJFJ-7
    L||||-FFL---7|FJF7F-----7|L-7F7F7L---JLJ|L7L7||L--7|L7|FJFJ|LJL7|.FJ||F7|F7FJF-JF7LJL7F-7L-7F--JF7|L-7|F-JL----JF7F--JF-7|L7LJL7|||FJLJ7-FL7
    FF7||F7JF---JLJFJLJF7F7L||F7LJLJL7F---77L-JFJ||F--J|FJ|||L7L7J-LJFJFJLJ|LJLJFJF-JL--7|L7|F7||F7FJLJ|FJ|L-----7F-JLJF7FJFJL7|F-7LJLJ|F|.|FJ.|
    F|LJLJL-JF7F7F7L-7-|LJL-JLJ|F7F77LJF--JF7F7|FJ|L--7||L||F7L7L7F7FJFJF--JF7F7L7|F7F-7LJFJ||LJ||||F7F7L-JF----7LJF---JLJ-L7FJLJLL-7F7L7J-7-7FF
    FL-7F---7|LJLJ|F7|FJF---7F7LJLJL7F-JF7F||||LJ||F7FJLJFJ||L-JFJ|LJFJ7L-7FJ||L-J||LJFJF7|FJL-7|||||||||F7L---7L--JF--7F--7|L-77F--J||FJ|J.L--|
    |LLLJ-F-J|F---J|||L-JF--J|L----7LJF7|L7|LJL--7||LJF-7L7|L-7FJJ|F7L-7|FJ|FJL-7.LJF-JFJLJ|F7FJLJLJ|||L-JL-7F7L----JF7LJF7||F7L7L7F7||L-7.JJ|FL
    J-7.F7L--JL7F7FJLJF-7L7F-JFF7F7L--J||FJ|F7F-7||L--JFJFJL7FJL7FJ||F-JFJFJ|F-7L--7L7FJF7LLJ||F----J|L----7LJ|F7F7F7|L7FJLJLJ|FJFLJLJ|F-J7|7L7J
    |FJ-|L-----J|LJ-F7L7|7LJF--JLJL----J|L-J|||7LJ|F--7L7L7FJ|F-J|FJ|L-7|FJJLJF|F7FJFJL-JL7F-J|L-7F-7L7F7F-JF7LJLJLJLJFJ|F--7FJL7F--7|LJJLJ77JF7
    L7|||F---7F7L7F-JL-JL7FFJF7F7F7F7F--JF7FJLJF7FJL-7L-J-|L7LJF-J|FJF7|||F----J|LJFL7F--7|L7FJF-J|FL7LJ|L--JL------7|L7LJF-J|F7LJF7L-7-F-L7-.7J
    .J-FLJFLLLJ|FJL--7F-7L-JFJLJLJLJ||F--JLJF7FJ|L7F7L---7|FJF7L7FJ|FJ||||L----7|F7F-JL-7||FJ|FJF7|F7L-7L-----------JF-JF-JF7LJL7FJL-7|||F.|.|J.
    FL7LL.|FF--JL7F--J|-L7F7|JF--7F-J|L---7FJ|L7L7|||F7F-JLJ|||7|L7|L7LJLJF7F7FJLJ||F7F7|LJL-JL7|LJ|L--JF7F7F-------7|F7L--JL-7JLJJ.LLJFL--J7.-7
    FFJJ|.F7L---7|L-7FJF7LJLJFJF7LJF7|LF7FJL7L-JFJ||LJ|L-7F--JL-JFJL7L-7F-JLJLJF7FJ||LJ|L--7F--J||FJF7F7|LJLJF------JLJL-7F7F7L-7F7.7L|77|J.|.LL
    .FJ-F7||F--7LJF-J|7||JF7JL7|L7FJLJFJLJ7FJF7FJFJL-7L--JL-7F-7FJF7|F7|L-----7||L7|L-7|F-7||F-7|FJFJ||||F---JF7F77F7F-7FJ|LJL-7LJL-77L7-|.7.77L
    FJ|||||||F-JF-JF7|FJL-JL-7LJ7LJFF7L----JFJ|L7L---JJF7F7FJL7|L-J|LJ||F-----J||FJL--J||FJ|||FJ|L7|FJ|LJL7F--JLJL-JLJFJL7L--77L7F7FJJ7|L|JL7J|.
    FF--|LJ|||-FJF7|LJL7F---7|F7F7F7||FF----JFL-JF77F7FJLJ||F7||F7FJF-J||F---7FJ|L----7LJL-JLJL-J.LJL7|F7FLJF---------JF-JF-7L-7||LJ-|JF-J|F7|LJ
    LJ7LL-7LJL-JFJ||F--J|F--JLJLJLJLJL7L--7F-7F7FJL7|||F--J|||||||L7L--JLJ-F-JL7L7F7F7L7|LF7F7F-----7LJ||F77L---------7L-7|FJF-JLJ|J.|7F7F7|F|-J
    .J7JL7L7F7F7L7LJL--7|L--7F7F7F7F-7|F--JL7||||F-J|LJL--7||||||L7|F-7F7|FJF7FJ7||LJL-J7FJLJ|L---7FJF-JLJL--------7F-JF7LJL7L-7.L|JF7-LJL|-|||J
    7F77.-FJ|LJ|FJ-F7F-JL7F-J|||||||FJLJF7F7|LJLJ|F-JF7F7FJ||||LJFLJL7LJL-JFJ|L7FJL7F7F-7L--7L----JL-JF-----------7|L--JL7F7L7FJ--JFLJ-F-7JJ|7F-
    L7|77.L7L7FJL-7||L7F-JL--JLJLJLJ|F--JLJLJF7F7LJF-JLJLJJLJLJ.L|7F-JF7F-7L7L7||F7LJLJFJF--JF7F--7F--JF7F--------JL--7F7|||LLJLF|JLLF7-7.|J|.7.
    L-|--F-JFJL7F7||L-J|F7F7F7FF7F--J|F------JLJL--JF7F--7F7-F7F7.LL7FJLJJ|FJ|LJ||L7F7FJJL--7|LJF-J|.F-JLJF---------7LLJLJ||F----7LFF7|JFFL.-7|7
    |.||||F7|JLLJLJL---J|LJLJL-J|L---JL-----------7.|||F-J|L7||7||JLLJF---JL---7||7LJ||J.F7FJ|F-JF7|FJF---JLF------7L7F7F7|LJF-7FJ.|L7J7LL|.F-JJ
    F.|--LJLJ7-L7|FLLF--JF-7F7F7|F7F----7F7F-7F7F7L-JLJL--JFJ||F777..LL7F7F7F7FJ|L--7LJF7|LJFJL-7|LJL-JFF7F7L---7F7L7LJLJLJF-JL||---.|.JJF||JLFJ
    |7LLFJLLJ..||-7.LL---J|LJLJ||||L---7LJ|L7LJ|||F7F7F---7|FJLJL7F7FF-J||||||L7L7F7L--J|L--J.F7LJLF-7F7|LJL---7LJL7L-----7L7F7LJF|-J.||-F|J.||J
    LJ7L||J-LF-LJ-J-F----------JLJL----JF7L-JF7LJLJ|||L--7LJL7F7FJ|L-JF-J|||||FJF|||F---J|F---JL7F7L7LJ|L--7F-7L---JF7F--7L7LJL-7F--.7L|7|LF-J|J
    F7LF-77-7J.LFJFFL-7F7F7F---7F-7F----JL---JL--7FJ|L7F-JF7FJ|LJLL7F7|F-J|LJLJLFLJLJF7F--JF7F-7|||FL-7L--7LJFJF7F7||||F-JJL-7F-J7FLFJFL7-.J7-L7
    FJF7LJ7F|77F77LFF-J|||LJF--JL7LJF------7F--7FJL-JJLJF7|||FJF7JL||LJ|F7L----7J.F|FJLJF7FJ||FJLJ|F7|L7F7L7JL-JLJL-JLJL----7|L--77-LLJ7|J.LJ7F-
    LLFJ||F-7F7||F7JL--J||F7L-7F7|F-JF----7LJF7LJF7F7F7FJ||LJL-JL7.LJJL|||F-7F-JL--FL--7|||.LJL--7LJL--J|L7L----------7F7F--JL---J7-.L7F7-J7FJ7J
    |.J-77L7LJ||LJ|LF--7LJ|L--J|LJL--JF---JF-JL--JLJLJ||FJ|F-----JF--7-LJ|L7||-|.L-LLJFLJLJF--7JFJF7F---J.L7F--7F7F7F7LJLJF7F7JF7F7J7FL.J.L|-LF7
    7FL7|F7L-7LJF-JFL-7|F7L----JF-7F77L7F--JF-----7F--J||FJL---7F7|F-JJL-L-JLJ.F7JL|FL-J|LLL-7|FJFJ||F-----J|F7||LJLJL7JF-JLJ|FJLJL7LJF-F---J.LL
    F-F7-F7F7|F-JF7F--J||L7F-7F7L7|||F7LJF-7L----7LJ-F7|||F----J|||L-77||FJ.||FL7LFFF7LJLFF7FJLJFJ-||L----7FJ||LJF-7F7L-JF---J|F---J-|7|L.7JFJLJ
    LF-7L|||LJ|F7||L--7|L7|L7||L-J||LJL--J7L----7L-7FJLJLJL7F7F7||L7FJF|J7.F||F----F7JJ-FF|LJF-7|F7LJF7F-7LJFJL--J-LJL7F7L77F-J|F7J7.FLJJ7F7-JFJ
    F|FJF||L-7LJLJL7F-JL7||FJ||F-7LJF----------7L--JL-7F---J||||||FJL-7J|L7FLJ||F|J|||-F--JF-JFJ||L--J|L7|F7L--7F----7LJL-JFJF-J|L7F--7J.LFJJ.J7
    L|L--J|F7L--7F-JL--7|||L7|LJ-L7FJF---------J|F-7F7||-F7FJ|||||L7F7L77.F7-LFF77FJL7JL---JF7L-J|F--7|FJLJL---J|F---JF-7F7|FJF7|FJL7FJ||F|7|7L7
    L|F7F7LJL-7FJL---7J|||L-JL77F7LJ-L--------7FFJFJ|||L-JLJFJ|LJL-J|L7|J.||FLF7F7L7FJ7JF---JL7F7LJF-JLJF7F7F7.FJ|F---JFJ|LJL-JLJL-7||F7.-7F-F7J
    FLJLJ|F7F7LJF--7FJFJ|L---7L-J|F-7F7F-----7L7L7|FJLJF7F-7L-JF7F7FJLLJJLF7F-J|||FJL-7.L---7FJ|L-7|F7F7|LJLJL7L7LJF---JFJF----7F7FJ|||L7LLJFL||
    |JLL-||LJL7FJF-JL7L7|F7F7L--7|L7LJLJF-7F7L-JFJLJF--JLJJL--7||||L-7-LJF||L-7|||L7F7|F-7F-JL7L7FJLJLJLJ-F---JF|F-JF7|FJFJLF-7LJ||J|||FJF7-JFJJ
    .--LFJL77.LJLL--7L-JLJLJ|F7J||.L---7|FJ||F-7|F--JF7F7F---7||LJ|F-J-|FF|L7-|LJL7||LJ|FJL--7L-JL-77.F7F7L-----J|F-JL7L7|F7|FJF7LJFJ||L-JL7F|7|
    -7FLL--J-F----7|L7F-7F-7||L7||F77F7LJL-JLJFJ||F--JLJLJF--J|L7||L7|.FF7L7|.L7F-J|L-7|L7F7FJF7F-7L7FJLJL------7LJF--JFJLJLJL-JL-7L7LJF7F-J-FF7
    FL7JJ.L||L---7L-7LJLLJL||L7LJLJ|FJL-7F-7F7L-JLJF7F7F--JFF7L-JFJFJ77F||FJ|F7|L7FJF-JL7LJLJFJLJFL7LJF--------7|F7|-F-JF-7F------JFJF7|||JFL|L-
    -7.F-FJF-----JF7L----7FJL-JF--7LJF-7LJ.|||F7F--JLJ|L---7|L--7L7|-F--J|L7|||L7|L7L-7FJF---J7F--7|F-JLF7F----JLJ|L-JF-J.|L-----7F|FJ||LJ-7JLFJ
    |-77.|.L7F----JL7F--7LJF7F-JF7L--JFJLF7LJ|||L---7||F7F7||F--J-|L7L7F7L-JLJL-J|FJF-JL-JF-7F7L-7|LJF--J|L--7F7JFJF7FJF-7L7F-7F7L-JL7LJ.J.|.L|7
    L7J.F|7JLJF-----J|F-JF7||||FJL-7F7L--JL7-LJL7F-7L7|||||LJL7F-7L7|-LJL7F-----7||FJF7F7.L7LJ|JFJL7FL7F7|F77LJL7|FJLJF|FJFJL7LJ|F---J||.|.7..|7
    ||7.F||F7FJF----7||F7|||||FJF-7LJL----7L---7||FJFJ||LJ|F-7||FJLLJ7F77LJLF77FJ||L7|LJ|F7L-7L7L-7|F7LJ|LJL---7|||F---JL7|F7L-7|L--7LJ--JFLLLJJ
    L7--F77|LJFJF---J|LJLJ||LJL-J|L7F----7L7F--JLJL7L7||F-J|FJ||||F7F7||F7F7||FJFJ|FJL7FJ||F7L7L--J|||F-JF7F--7LJLJ|F----J|||F-JL---J.J||F-7J-||
    ||..||FJF7|FL----JF---J|F--7F--J|F--7L7|L-----7L-JLJL--JL7LJL7||||||||||||L7L-JL--JL-J||L7L---7LJLJF7|||F-JFF7L|L----7|||L--7F7|F7.FF|-.L.F-
    FF-F|LJFJ||F------JF---JL-7LJF7FJL-7L-JL7F-7F7L--7F--7F7-L---J|LJLJ||||||L-JF---------JL7L--77L----JLJLJL---JL7|F----JLJL7F7LJL7|L-7.7J7FF77
    F--7L7FJ7||L7F7F7F7|F--7F7L--JLJJF-JF7F7LJ7LJL7F-JL-7LJL-7.F7LL--7FJ||||L--7L---77F7F7F-JF-7L-----------7F7F--J|L---7F-7FJ|L7F7LJF-J.F77FJFJ
    L-7|F||LFJ|FJ|LJLJLJL-7||L7F-----JF-JLJL-7F7F7LJLF7FJF---JFJL-7LFJL7||||.F7|F---JFJ|||L-7|FJF7F--7F-7F-7LJLJ|F-JF--7LJFJL7|7|||F-JJ.F|LJ.F|J
    FFJL-J|.L7|L7|F------7||L7LJF--7F-JF7F7F-J|LJL7F7||L7|LF7|L7F-JFJF7||LJL-JLJ|F7F7L7|||F7LJL7|LJF7LJ|LJFJF7F--JF7L-7|F7L-7|L7|||L--7-FL7|7F.|
    LL7F7FJ7LLJ-LJ|F----7LJL-JF-JF7LJF7|LJLJF7|F--J|||L7||FJL7L||F7|FJ||L7F7F7F7LJ|||FJLJ||||F7||F-JL----7L-JLJF-7|L--JLJL--JL-JLJL---J777LJ-|.F
    LFLJ||JL|JLF.LLJF---JF7F--JF-JL-7|LJ-F7FJ|||F-7||L7|||L-7L7|||LJL7LJ7||||LJ|F-J||L--7|||FJ|||L------7L-----JF|L--7F7F-7F7F7LF7JF7J7||L7FLL.|
    F7FFJ|JFF7LJ-|LFL----J||7F7L---7|L--7|LJFJ|||FJ|L-JLJ|F7|FJ||L-7FJJF7LJ||F-J|F7||F7FJLJ||FJLJF7F--7FJF------7|F-7LJLJFJ|LJL-JL-JL--7-7FJF|F7
    ||FL-JF-J7J.FF7F7.F7F7LJFJ|JF7FJL---J|F-JFJLJL7L--7F-J|LJL7|L7FJ|F-J|F7LJL-7||LJ|||L--7||L77FJLJF7LJFJF-----JLJJL7F--JL|F----------J|L-.FFFJ
    JJ..J7|J.--7F|LJL-JLJL--JFJFJLJF7F---JL--JF---JF7FJL7|L7F-J|FJL7LJF-J||7F7FJ||F-J|L7F7||L7L7L--7|L--JFJF-----7F7-|L----JL----7F---7-J|-F-LF7
    |F-7JFLF.|-F-JF-----7F7F7L7L--7|||F----7F7L--7FJLJF7L7FJL-7||F-JF-JF-JL7|LJFJ|L7-L7||LJ|JL7L--7||F---JFJF----J|L7|F-7F7F-----J|F--JLLL7L|.|.
    FLF7-7F|7JJL7FJF7|F7LJLJL7L--7LJLJL7LF7LJL---JL---J|FJL7F-J||L7FJF7L--7|L-7|-|FJF-J|L-7|F7|F--JLJL--7FJ-L---7FJFJ|L7LJ|L---7LFJL-7|FLLF7J7||
    7-LF-LJJ...LLJFJL-JL-----JF-7L--7F7L-JL---7.F-7F7F-JL-7||F7||FJ|FJL7F-J|.FJL7|L7L7FJF-JLJ|||F7F7F-7-||F----7||FJ7L-JF7L7F--JFJF7FJ-L-JL---J7
    JJ||7.J7FL-F7LL7F7F7F7F---JFL--7LJL---7F-7|FJFJ||L---7|||||||L7||F-JL-7|FJF-J|FJFJL7L-7F-J||||||L7L7LJL---7LJ|L--7F-J|LLJF7FJFJLJJJFF-7.FLLJ
    |.7LJ-FJ-FF|L--J|||||||F---77F7L-----7|L7LJ|FJ-|L7F-7|||LJ|||FJLJL--7FJ|L7L-7||FJF-JF-JL--JLJ||L7L7L7|F---JF7|F--J|F-JF7F|LJFJLF|-FFL7|.|.LF
    FF77J-|J-FFL-7F7|||LJLJL--7|FJL------JL7L7FJ|F7L7|L7LJLJF-J||L-7F---JL7|FJF-J||L7|F7L-7F-----J|FJ|L7L7L----JLJ|F7F||F-JL-JF7L-7-F7J|-|J|-7.7
    L||.LJ.|7FJF7LJLJLJ|F-7F-7|LJF-------7|L-J|FJ||FJL7L--7FJJFJ||FJL-7F7FJ|L7L-7|L7|LJ|F7|L-7FF7|||F7FL7|F7F-----J|L-JLJF-7F-J|F7|7||FF.|..JLF-
    LLL7FF|FF7L|L-77F7-FJFJL7LJF-JF77F--7L7.F-J|7||L-7|F7FJ|F7L7L7L7F-J|||FJ7|F-JL7|L-7||||F-JFJL7||||F7||||L------JF----J.|L-7LJLJFJ|7|77.FJ7.L
    .|L--LFL|L-JF7L-JL-JFJF7L-7|F-JL-JF7L-JFJF7|FJ|F7|||||FJ|L7L7|FJ|F7|||L7FJ|F7FJ|F7|||||L7FJF-J|||||||||L---7F7F7L-7F--7L-7L7F7FJFJF--7-J.FJJ
    F|.L7FJLL---J|F7F--7L-JL-7LJL-7F7FJ|LF7L7|LJ|FJ||||||||7L7|J|||FJ|||||FJL7|||L7||||||||FJL7|F7||||||||L---7|||||F-J|F-JF7L-J||L7|||F7|.L-FJJ
    FJ-.FL777F---J|LJF7L----7|F7F7||||JL7|L-JL7FJL7|||LJ||L7FJ|FJ||L7||||||F7|LJ|FJ||||||||L7FJ||LJ||||LJ|F7F7|LJLJ|L-7||F-J|.F-JL-JL-J|LJ-F7LF7
    |LLF77JF-L7F7FJF-J|LF7F-JLJLJLJ|||F-JL-7F-JL-7||LJF-JL7|L7||7||||||||||||L-7|L7||||||||F|L7|L7F||||F-J||||L7-F7L-7LJLJF7L-JF7F-7F-7L-7FJ7.L.
    J-.L|JFLJ|LJ|||L-7L-JLJF7F7F-7FJ|||F7F7||F7F-J|L7FJF7FJL-J||FJL7||||LJLJ|F-J|FJ||||LJ||FJFJL7L7||||L-7|LJL7L-JL-7|F7F7|L---JLJ.|L7L7FJJ.J7L7
    LL-..-JL-FLLLJF77L--7F-J||LJFJL7||||||||||||F-JFJL7|||F7F-J||F7|||||F---J|F7LJJLJ||F-J|L-J7-|FJ||LJF-JL-7FJF7F7FJLJ||||F--7F7F7L7L7LJJ.----L
    |J|F|JF|7F--7FJL----JL7FJL7FJF7|LJ|||||||||||F7L-7||LJ|LJF7|||LJ|||||F--7LJL----7||L-7L---7FJL7|L-7L-7F7|L7|LJ|L7F7||LJ|F-J||||-L-J77FLJFJFL
    7.|FJ-|7-L-7LJF--7F-7FJL7FJL7|||F-J|||||||LJ|||F-J||F-JF7|||||F7||LJ||F-JF-7F---JLJF-JF7F7||F-J|F-JF-J||L7|L-7|FJ|LJL7FJL7.||||F7F-777|FL7L|
    |F-JLFJ-F--JF7L7J||FJ|F-JL-7||||L7FJ||LJLJF7|||L7JLJ|F7|||LJ|||LJL7FLJL-7L7|L--7F--JF7|||LJLJF-JL-7|F7|L7|L7FJLJLL7F7LJF7L7|||LJLJFJLJ77LF7|
    JLJLLLJJL--7||FJFJ|L-J|F7F7|||LJFLJ.|L7F7FJLJ|L7L-7JLJ|||L-7||L7F7L7.F--JFJ|F7FJL-7FJ|||L7F7FJF-7FJ|||L7||FJL--7F-J|L7FJ|FJ|||F--7L77FF-JL|7
    L7F77J.7FFFJ|||J|FJF--J|||||LJF-----JFJ|LJF7FJFJF7L7F7|||F-J||FJ|L-JFJF-7|LLJ|L-7FJL7||L7LJ|L7|FJL7|||FJLJL7F7FJL7FJFJL7|L-JLJ|F7L7|L|.|-FF7
    F-LJ|7|.|-L-JLJ7LJ|L7F7|LJ|L-7L-7F7F7L7L-7|LJ7L7|L7||LJ||L-7LJL7|F--JFJFJ|F-7|F-JL7FJLJFJF7|-LJ|F7||||L--7FJ||L7F|L7L7FJL7F7F7LJL7LJ7|.JF--J
    -.LFJLJJ..LJ||.F---7||LJF-JF7|F-J||||FJ|FJL7F7FJL7LJL7FJL--JF7FJ|L7F7L7L7LJFJ|L-7FJL--7L7|LJF-7||||||L7F-JL-JL7L7|FJFJ|F7LJ||L7F7L7JJ7F7L7|.
    |FFJ7JLL7||--L7L7F7LJL7FL7FJ|||F7|LJ||F-JF7LJ|L7FJF7FJL--7F7|LJFJFJ||FJ|L7FJL|F-JL7F7FJF|L--JFJ||||||FJL----7-|FJ|L7L7LJ|F-JL7|||FJFLLF7-|.J
    |7L||L|J|-J7F|FLLJL7F7|F-JL7||||||F-J|L7FJ|F-J7LJFJLJF7F7LJ||F7L7L7|LJ|F7||F7LJFF-J|LJF-JF7F7L7||||LJL-7F7F-JFJL7L-JFJF7|L7F-JLJLJL-JFJLFLLJ
    L7JJ|7JF-7FF--J|.LFJ|LJL-7FJLJ||LJL-7|FJ|FJL--7F-JF-7||||F7|||L7L7LJF7FJLJLJL7F-JF7L-7L-7|||L-JLJ||F--7|||L7FJF7|F7F|FJLJL|L----7JL|F||J|L|J
    |J|-||L|.-JJJJJFF-L-JF---JL-7FJL7F--J|L7|L7F--JL-7L7||LJ||LJLJFJFJF-JLJF7F-7FJL7FJ|F7|7FJ||L----7||L-7LJ||FJL7|LJ|L-JL---7L7F---JL-F--7FF7L|
    F7F7-J---L-L.LFF7LL-LL7F-7F7|L7FJL7F7|FJ|.|L--7F-JFJLJJFJL7F--JFJFJF7F7||L7|L7FJ|FJ||L7L7|L7F--7|||7FJF7||L-7|L7LL-7F-7F7|FJL---7--JLL.-J-FJ
    7-|7.|||7|7F7.FLJ7|F|JLJJ||LJLLJF-J|LJL7L7|F7FJL7FJF---JF-JL-7FJFJFJLJLJ|FJ|FJ|FJL7||FJ.|L7|L-7||||FJFJ|||F7|L7L7-FJ|FJ||||F-7F-J-7-7-LJ.||J
    |L||FF7LFF-FLFJ7F7-7JFJ-FLJ.|FJJL-7|F-7L7|||LJ7FJL7|F7F7|F---J|FL7|F----JL7|L7|L77LJ|L7FJFJ|F-JLJLJL7|FJ||||L7L7L7L7|L7|LJ||J||JJ7LF777|FL-F
    ..FJFJJ7LJF---7F|J||JL|L|-|-L7|.|FJ|L7L-J||||F7L7FJLJLJLJ|F7F7L-7||L---7F-J|FJ|FJF-7L7|L7|||L-----7FJ|L7||||FJFJFJ-||7|L-7||FLJJ7J-|JL77|J7L
    |F|J.|FJ7F-J7L--||JJF7L7J||J7F7-FJFJFJF7FJ|L-JL7||F------J|LJL7FJLJ|F--JL7LLJFJL-JFJFJ|FJL7|F7F7F7|L7|7||LJ||-L7|F-JL7|F-J||-JJ..|-JLL|JJJL|
    LFJ|FLJFJJLL7-|LLJ.F|-F|J-|.-||FJFJJL7||L7|F7F-J||L--7F7F7|F--J|F7F-JF-7FJF-7|F-7FJ-L7|L-7|LJ||||LJ-||FJL7FLJJFJ|L-7FJ|L-7LJJ|.F-..|.LJ|7|7|
    LL--J7LJ|..FLF|.FF-.|-F|.7-L.|LJFJL77LJL7||||L7FJL7LFLJ|||||F--J||L-7|7|L-JFJ||FJL-7FJ|LFJL-7LJ||J|.|||F7L--7.L7|7FJ|FJF-JFL7LLJJ-L|7-7LJ7JJ
    F7FFLFJFL77L-J|F-L--7.||-7.FF|F7L77|F---J||||FJL7FJ-F--J|LJLJF--JL--JL7L-7FJ-|||F-7|L-J7L-7FJ-L|L7|FJ|||L7F7L-7LJ-L7|L-J.|7.J|.|L.LJ.7|7.-.L
    7L7JLLFF--L.|.L-77L.|-7LJ.LLL||L7L7-L7F7FJ|||L77LJ|LL7F7L7-LFJF-----7FJF-J|F7|||||LJJJLF--J|L|LL-JFJFJ||L||L--J|.7JLJJ-LLJ.F-777.7.F|.|-|J7.
    J-.|7.FJ-|.-77-J--JFLFF7|77-|||7L7|JJLJLJ|||L7L-7.||LLJL7L7JL7|F----J|-L-7LJ||||L-7J7L-L7F7|-JJ.|.L7|JLJ-LJ-|7.7F--LJLFJ|.F-7|FL77F--7JL|LL|
    |JLLL7|7J.F|JF7.|L-77|J|7-J--||JJLJ7J..F|F||FJF-J-|-F|.FJFJJFJ||F7F-7L--7L7FJLJL7FJ7FL-J||LJF|.FL7LLJ77|FLJ-|7-L7J7FL7-JFF77LF-.LL-JF||L-7L|
    -7..F-J|--FJ||.FJLFJF|.LF-|7FLJF||LL---FL-LJL7|7J||FF-.L-JJF|FJLJ|||L7F7|-|L-7.LLJL777F-LJJ-7.FL7LJ||LL--7L7JJ--JFF---J7LJJF--JFF-L7||L-LL-J
    F77F7L-JL|.FL-J7LF--F-J-L-JFJJ-|-LJF77|L|LL|||L7|F.F-7-|J|||||J.L||-FJ||L7|F-J-|LJ7L-J-L|L|-J--7F-7-F-.FLJ.|LL7J-JLJ.|-L7L-J-L7LL7FJLLL7.|7.
    ||.-|--J-J-77|.F7|JJ.|J77.FLL7.L7F--FJJ|L|J.LL7|JLFJFJL|7FJ-LJ7.FLJ|L7||FJ|L-77--.FJJ7LFJJF..FL7|.JJL.-7.JJ.||LJ.LLJ.77FJ7|J|F||LFJ.J7.|7L-7
    LL7JJJ7|J.L|F--F|777L-F-|.|JLLJ--F7L||L-.L7F|LLJ..LL|.F-JJJFLL7-FJFF7|||L7L--JJLL7LJ.FFLJ..|.JF|--|LF--F.|F|JF-|-.|J.|LJLFJ-L7FJ-777.F|FJJ-|
    L||L|----F.7J..LLJ-FJ-JL|-7.|-|7.L-F7-|FJ-|7F-JJFL..J7|||-F|JLLJF-|7-LJL-JLJ7LJL|-L-FF-J-|7JFLJ|L.|F--F7-F7JLF-J-FL..FFJFJ|FL|JJ|L77.F|J..FJ
    7L|-7F|-|LL77...|J.-7L7-L-7-L.L-7JF||7--|7|||-LJ|JFJ||-|7FFFJ.|F-JLJ||.||..FFFL-|.|--JL77L--|.FJ.J-|JF||.|J.FF-|7F7.-7|F|--F7|..JJ|L7.L7--F7
    --|7.L--||J|J-L-J.7L|L-.FLJ7F7|F|FFJLF7J|JL.L7|7JF-L7||JLF-J|LJ.|JFL-|FJ-L-JLF7FFLF.J.FLJ7J..F|7F|-LF7L--7.FL7FL7777LFJ7|F-LJ7F|7LL.L7F||F|7
    .L77FJ|.|L7||.|.F-F-|7F7-7FL|LJ-J-JJ-|F-|L|FLLJLF|7-FL---7-|JFL-J.LJ-JJ-||.L7----.L7|.----.LFJJL-J.L-L.|LF7LJFJFLJJFL7-F|--7J--.|LJF|L||FJ-.
    7LLL-.7--LJJJ7-7J-JJLJ-JL|--|JL-JJ|.F.FJ.-J.L.|.L.|L|-JJLJ-L--J-J-|LLLL7--7J.|.LJ.|.LJJ--J.LLJLJJ--.LLJ7LJJ.LLJFL--FJL-JJ-7LJLL-|--FLFJ|-LL.
    "##;
    let lines = read_lines(input);
    println!("{:#?}", lines);

    let grid = Grid::read_grid(&lines[..]);
    // println!("grid: {:#?}", grid);

    let moves: Vec<_> = grid.iter_valid_moves(grid.start).collect();
    println!("start moves: {:#?}", moves);

    let moves2: Vec<_> = grid.iter_valid_moves(Point::new(3, 3)).collect();
    println!("moves (3,3): {:#?}", moves2);

    let route1 = grid.get_loop();
    // println!("Route1: ");
    // route1.iter().for_each(|p| println!("{}", p));

    let total1 = route1.len() / 2;
    println!("total 1: {}", total1);

    let size = (grid.v.len(), grid.v[0].len());
    let mut count = 0;
    let mut g = vec![vec![Tile::Dot; size.1]; size.0];
    for pt in route1.iter() {
        // keep only the boundaries
        g[pt.i][pt.j] = grid.get_tile(*pt);
    }

    // replace the start position with direction
    let start = route1[0];
    let d1 = start.direction(route1[1]);
    let d2 = start.direction(*route1.last().unwrap());
    println!("Start pipe: {:#?}, {:#?}", d1, d2);
    g[start.i][start.j] = Tile::Pipe(d1, d2);

    let mut tmp = vec![vec!['.'; size.1]; size.0];
    for i in 0..size.0 {
        // everytime you cross a boundary alternate
        // between inside or outside
        use Move::*;
        use Tile::*;
        let mut is_inside = false;
        let mut is_on_line = false;
        let mut entered_north = false;
        for j in 0..size.1 {
            match g[i][j] {
                Pipe(d1, d2) => {
                    if [d1, d2] == [N, S] || [d1, d2] == [S, N] {
                        is_inside = !is_inside;
                    } else if !is_on_line {
                        is_on_line = true;
                        if [d1, d2].contains(&N) {
                            entered_north = true;
                        } else if [d1, d2].contains(&S) {
                            entered_north = false;
                        }
                    } else {
                        if [d1, d2].contains(&N) {
                            is_on_line = false;
                            if !entered_north {
                                is_inside = !is_inside;
                            }
                        } else if [d1, d2].contains(&S) {
                            is_on_line = false;
                            if entered_north {
                                is_inside = !is_inside;
                            }
                        }
                    }
                }
                _ => {
                    if is_inside {
                        count += 1;
                        tmp[i][j] = 'I';
                    }
                }
            };
        }
    }
    for row in tmp {
        println!("{}", row.iter().collect::<String>());
    }
    println!("Total 2: {}", count);

    println!();
}

fn read_lines(txt: &str) -> Vec<String> {
    txt.lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    N,
    S,
    E,
    W,
}

impl Move {
    fn opposite(&self) -> Self {
        use Move::*;
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Pipe(Move, Move),
    Start,
    Dot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    i: usize,
    j: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Point({}, {})", self.i, self.j))
    }
}

impl Point {
    fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    fn is_within(&self, other: Self) -> bool {
        self.i < other.i && self.j < other.j
    }

    /// overflowing next_pt so always check bounds
    fn next_pt(&self, direction: Move) -> Self {
        use Move::*;
        let p = self;
        match direction {
            N => Self {
                i: p.i.wrapping_sub(1),
                j: p.j,
            },
            S => Self { i: p.i + 1, j: p.j },
            E => Self { i: p.i, j: p.j + 1 },
            W => Self {
                i: p.i,
                j: p.j.wrapping_sub(1),
            },
        }
    }

    fn direction(&self, other: Self) -> Move {
        use Move::*;
        if other.i > self.i {
            S
        } else if other.i < self.i {
            N
        } else if other.j > self.j {
            E
        } else if other.j < self.j {
            W
        } else {
            panic!("bad direction");
        }
    }
}

#[derive(Debug)]
struct Grid {
    v: Vec<Vec<Tile>>,
    start: Point,
}

impl Grid {
    fn read_grid(lines: &[String]) -> Self {
        let v: Vec<_> = lines.iter().map(|x| Self::read_row(x)).collect();
        let start = Self::find_start(&v);
        Grid { v, start }
    }

    fn read_row(txt: &str) -> Vec<Tile> {
        use Move::*;
        use Tile::*;
        txt.chars()
            .map(|x| match x {
                '|' => Pipe(N, S),
                '-' => Pipe(E, W),
                'L' => Pipe(N, E),
                'J' => Pipe(N, W),
                '7' => Pipe(S, W),
                'F' => Pipe(S, E),
                'S' => Start,
                '.' => Dot,
                x => panic!("Unrecognised char: {}", x),
            })
            .collect()
    }

    fn find_start(v: &Vec<Vec<Tile>>) -> Point {
        v.iter()
            .enumerate()
            .flat_map(|(i, vals)| {
                vals.iter()
                    .enumerate()
                    .filter(|(_, val)| **val == Tile::Start)
                    .map(move |(j, _)| Point { i, j })
            })
            .next()
            .unwrap_or_else(|| panic!("start coord not found"))
    }

    fn get_loop(&self) -> Vec<Point> {
        let mut out = Vec::new();
        let mut p: Point = self.start;
        let mut prev_p = p;

        loop {
            out.push(p);
            let moves: Vec<_> = self.iter_valid_moves(p).collect();
            if moves.len() == 2 {
                let next_p1 = p.next_pt(moves[0]);
                let next_p2 = p.next_pt(moves[1]);

                if ![next_p1, next_p2].contains(&prev_p) {
                    if prev_p != self.start {
                        panic!("expected one {:#?} within {:#?}", p, moves);
                    }
                }

                if next_p1 != prev_p {
                    prev_p = p;
                    p = next_p1;
                } else if next_p2 != prev_p {
                    prev_p = p;
                    p = next_p2;
                } else {
                    panic!("duplicate points {:#?} within {:#?}", p, moves);
                }
            } else if moves.len() == 1 {
                panic!("only one connect {:#?} within {:#?}", p, moves);
            } else {
                panic!("not connected at point: {:#?}", p);
            }
            // println!("Moved from {:#?} to {:#?}", prev_p, p);
            if p == self.start {
                break;
            }
        }
        out
    }

    fn iter_valid_moves(&self, p: Point) -> impl Iterator<Item = Move> + '_ {
        use Move::*;
        use Tile::*;

        let moves = match self.get_tile(p) {
            Pipe(d1, d2) => vec![d1, d2],
            Start => vec![N, S, E, W],
            Dot => vec![],
        };

        moves.into_iter().filter(move |d| self.can_move_from(p, *d))
    }

    fn can_move_from(&self, p: Point, direction: Move) -> bool {
        use Tile::*;
        let next_p = p.next_pt(direction);
        next_p.is_within(self.grid_bounds())
            && match self.get_tile(next_p) {
                Pipe(d1, d2) => {
                    // println!("can move from: {:#?} to: {:#?}", p, next_p);
                    // println!("checking: {:#?} in [{:#?}, {:#?}]", direction.opposite(), d1, d2);
                    [d1, d2].contains(&direction.opposite())
                }
                Start => true,
                Dot => false,
            }
    }

    fn grid_bounds(&self) -> Point {
        Point {
            i: self.v.len(),
            j: self.v[0].len(),
        }
    }

    fn get_tile(&self, p: Point) -> Tile {
        self.v[p.i][p.j]
    }
}
