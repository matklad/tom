// auto-generated: "lalrpop 0.15.0"
use events::Events;
use symbols;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__TomlFile {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use events::Events;
    use symbols;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(()),
        Variant2(::std::vec::Vec<()>),
        Variant3(((), ())),
        Variant4(::std::vec::Vec<((), ())>),
        Variant5(usize),
        Variant6(::std::option::Option<()>),
        Variant7(((), (), ())),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 15, 0, 0, 0, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 1
        0, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, -44, 0, 0, 0, -44, -44, -44, -44, 0, 0, -44, 0, 0, 0,
        // State 5
        0, 0, 0, 15, 0, 0, 0, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 6
        -41, 0, 0, -41, 0, 0, -41, -41, -41, -41, -41, 0, 0, -41, 0, 0, 0,
        // State 7
        0, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 9
        0, -51, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, -68, 0, 0, 0, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, -52, -52, -52, 0, -52, -52, -52, -52, -52, -52, -52, -52, -52, -52,
        // State 15
        0, -19, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, -21, -21, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, -20, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        -22, -22, -22, -22, -22, 0, -22, -22, -22, -22, -22, 0, 0, -22, 0, 0, 0,
        // State 19
        -61, -61, -61, -61, -61, 0, -61, -61, -61, -61, -61, 0, 0, -61, 0, 0, 0,
        // State 20
        0, 0, 0, 15, 0, 42, 0, 0, 43, 44, 19, 45, 46, 20, 47, 48, 49,
        // State 21
        0, 0, 0, -37, 0, -37, 0, 0, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 22
        0, 0, 0, -45, 0, 0, 0, -45, -45, -45, -45, 0, 0, -45, 0, 0, 0,
        // State 23
        0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 53, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, -69, 0, 0, 0, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 27
        -84, 0, 0, -84, -84, 0, -84, -84, -84, -84, -84, 0, 0, -84, 0, 0, 0,
        // State 28
        -57, 0, 0, -57, -57, 0, -57, -57, -57, -57, -57, 0, 0, -57, 0, 0, 0,
        // State 29
        -55, 0, 0, -55, -55, 0, -55, -55, -55, -55, -55, 0, 0, -55, 0, 0, 0,
        // State 30
        -56, 0, 0, -56, -56, 0, -56, -56, -56, -56, -56, 0, 0, -56, 0, 0, 0,
        // State 31
        -85, 0, 0, -85, -85, 0, -85, -85, -85, -85, -85, 0, 0, -85, 0, 0, 0,
        // State 32
        0, 0, 0, 15, -30, 42, 0, 0, 43, 44, 19, 45, 46, 20, 47, 48, 49,
        // State 33
        0, 0, 0, 0, 0, 0, -26, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 34
        -83, 0, 0, -83, -83, 0, -83, -83, -83, -83, -83, 0, 0, -83, 0, 0, 0,
        // State 35
        -59, 0, 0, -59, -59, 0, -59, -59, -59, -59, -59, 0, 0, -59, 0, 0, 0,
        // State 36
        -58, 0, 0, -58, -58, 0, -58, -58, -58, -58, -58, 0, 0, -58, 0, 0, 0,
        // State 37
        -60, 0, 0, -60, -60, 0, -60, -60, -60, -60, -60, 0, 0, -60, 0, 0, 0,
        // State 38
        -54, 0, 0, -54, -54, 0, -54, -54, -54, -54, -54, 0, 0, -54, 0, 0, 0,
        // State 39
        -48, 0, 0, -48, 0, 0, -48, -48, -48, -48, -48, 0, 0, -48, 0, 0, 0,
        // State 40
        -80, 0, 0, -80, -80, 0, -80, -80, -80, -80, -80, 0, 0, -80, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, -53, -53, -53, -53, -53, 0, 0, -53, 0, 0, 0,
        // State 42
        -34, 0, 0, -34, -34, 0, -34, -34, -34, -34, -34, 0, 0, -34, 0, 0, 0,
        // State 43
        -65, 0, 0, -65, -65, 0, -65, -65, -65, -65, -65, 0, 0, -65, 0, 0, 0,
        // State 44
        -23, 0, 0, -23, -23, 0, -23, -23, -23, -23, -23, 0, 0, -23, 0, 0, 0,
        // State 45
        -33, 0, 0, -33, -33, 0, -33, -33, -33, -33, -33, 0, 0, -33, 0, 0, 0,
        // State 46
        -62, 0, 0, -62, -62, 0, -62, -62, -62, -62, -62, 0, 0, -62, 0, 0, 0,
        // State 47
        -63, 0, 0, -63, -63, 0, -63, -63, -63, -63, -63, 0, 0, -63, 0, 0, 0,
        // State 48
        -64, 0, 0, -64, -64, 0, -64, -64, -64, -64, -64, 0, 0, -64, 0, 0, 0,
        // State 49
        0, 53, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 51
        0, 0, 0, -74, 0, 0, 0, -74, -74, -74, -74, 0, 0, -74, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, -36, -36, -36, -36, 0, 0, -36, 0, 0, 0,
        // State 53
        -66, 0, 0, -66, -66, 0, -66, -66, -66, -66, -66, 0, 0, -66, 0, 0, 0,
        // State 54
        0, 0, 0, 15, -32, 42, 0, 0, 43, 44, 19, 45, 46, 20, 47, 48, 49,
        // State 55
        0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        67, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, -28, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        67, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 19, 0, 0, 20, 0, 0, 0,
        // State 61
        0, 0, 0, -75, 0, 0, 0, -75, -75, -75, -75, 0, 0, -75, 0, 0, 0,
        // State 62
        0, -14, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        67, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        -18, 0, 0, -18, -18, 0, -18, -18, -18, -18, -18, 0, 0, -18, 0, 0, 0,
        // State 65
        0, 0, 0, -9, -9, -9, 0, 0, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 66
        0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 67
        67, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        -35, 0, 0, -35, -35, 0, -35, -35, -35, -35, -35, 0, 0, -35, 0, 0, 0,
        // State 69
        -67, 0, 0, -67, -67, 0, -67, -67, -67, -67, -67, 0, 0, -67, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, 0, 0, -4, 0, 0, 0,
        // State 71
        0, -15, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, -10, -10, -10, 0, 0, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 73
        0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, 0, 0, -5, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -76,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        -44,
        // State 5
        -78,
        // State 6
        -41,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        -72,
        // State 11
        -77,
        // State 12
        -68,
        // State 13
        -86,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        -22,
        // State 19
        -61,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -45,
        // State 23
        -79,
        // State 24
        0,
        // State 25
        -73,
        // State 26
        -69,
        // State 27
        -84,
        // State 28
        -57,
        // State 29
        -55,
        // State 30
        -56,
        // State 31
        -85,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -83,
        // State 35
        -59,
        // State 36
        -58,
        // State 37
        -60,
        // State 38
        -54,
        // State 39
        -48,
        // State 40
        -80,
        // State 41
        0,
        // State 42
        -34,
        // State 43
        -65,
        // State 44
        -23,
        // State 45
        -33,
        // State 46
        -62,
        // State 47
        -63,
        // State 48
        -64,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -74,
        // State 52
        0,
        // State 53
        -66,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        -75,
        // State 62
        0,
        // State 63
        0,
        // State 64
        -18,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        -35,
        // State 69
        -67,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 6, 0, 7, 8, 9, 0, 0, 10, 0, 0, 0, 0, 0, 11, 0, 12, 13, 14, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 23, 0, 0, 0, 7, 8, 9, 0, 0, 10, 0, 0, 0, 0, 0, 11, 0, 24, 13, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 8, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 13, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 27, 0, 7, 8, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 29, 30, 0, 0, 0, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 34, 35, 36, 37, 38, 39, 0, 0, 0, 0, 0, 0, 0, 40, 0, 41, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 13, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 23, 0, 0, 0, 7, 8, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 28, 0, 29, 30, 0, 0, 56, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 34, 35, 36, 37, 38, 39, 0, 0, 0, 0, 0, 0, 0, 57, 0, 41, 0,
        // State 33
        0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 59, 0, 0, 0, 0, 0, 0, 4, 60, 0, 0, 0, 7, 8, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 8, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 29, 30, 0, 0, 0, 31, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 34, 35, 36, 37, 38, 39, 0, 0, 0, 0, 0, 0, 0, 64, 0, 41, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 68, 0, 0, 0, 7, 8, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 8, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"",""###,
            r###"".""###,
            r###""=""###,
            r###""[""###,
            r###""]""###,
            r###""{""###,
            r###""}""###,
            r###"bare_key"###,
            r###"bare_key_or_date"###,
            r###"bare_key_or_number"###,
            r###"basic_string"###,
            r###"bool"###,
            r###"date_time"###,
            r###"literal_string"###,
            r###"multiline_basic_string"###,
            r###"multiline_literal_string"###,
            r###"number"###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct TomlFileParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl TomlFileParser {
        pub fn new() -> TomlFileParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            TomlFileParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            E,
        >(
            &self,
            events: &mut E,
            input: &'input str,
        ) -> Result<(), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> where
          E:  Events,
        {
            let mut __tokens = self.builder.matcher(input);
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(e),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token(0, _) if true => 0,
                    Token(1, _) if true => 1,
                    Token(2, _) if true => 2,
                    Token(3, _) if true => 3,
                    Token(4, _) if true => 4,
                    Token(5, _) if true => 5,
                    Token(6, _) if true => 6,
                    Token(8, _) if true => 7,
                    Token(15, _) if true => 8,
                    Token(14, _) if true => 9,
                    Token(10, _) if true => 10,
                    Token(16, _) if true => 11,
                    Token(7, _) if true => 12,
                    Token(12, _) if true => 13,
                    Token(11, _) if true => 14,
                    Token(13, _) if true => 15,
                    Token(9, _) if true => 16,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(15, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(16, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                Token(13, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(events, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<(E)>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(events, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<(E)>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> Option<Result<(),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>> where
      E:  Events,
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                (|| {
                    // (<KeyVal> Comma) = KeyVal, Comma => ActionFn(68);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action68::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (2, __symbol, 0)
                })()
            }
            2 => {
                (|| {
                    // (<KeyVal> Comma)* =  => ActionFn(66);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action66::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (0, __symbol, 1)
                })()
            }
            3 => {
                (|| {
                    // (<KeyVal> Comma)* = (<KeyVal> Comma)+ => ActionFn(67);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action67::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 1)
                })()
            }
            4 => {
                (|| {
                    // (<KeyVal> Comma)+ = KeyVal, Comma => ActionFn(75);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action75::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (2, __symbol, 2)
                })()
            }
            5 => {
                (|| {
                    // (<KeyVal> Comma)+ = (<KeyVal> Comma)+, KeyVal, Comma => ActionFn(76);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action76::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (3, __symbol, 2)
                })()
            }
            6 => {
                (|| {
                    // (<Val> Comma) = Val, Comma => ActionFn(63);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action63::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (2, __symbol, 3)
                })()
            }
            7 => {
                (|| {
                    // (<Val> Comma)* =  => ActionFn(61);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action61::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (0, __symbol, 4)
                })()
            }
            8 => {
                (|| {
                    // (<Val> Comma)* = (<Val> Comma)+ => ActionFn(62);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action62::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 4)
                })()
            }
            9 => {
                (|| {
                    // (<Val> Comma)+ = Val, Comma => ActionFn(79);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action79::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (2, __symbol, 5)
                })()
            }
            10 => {
                (|| {
                    // (<Val> Comma)+ = (<Val> Comma)+, Val, Comma => ActionFn(80);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action80::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (3, __symbol, 5)
                })()
            }
            11 => {
                (|| {
                    // (Dot Key) = Dot, Key => ActionFn(48);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action48::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (2, __symbol, 6)
                })()
            }
            12 => {
                (|| {
                    // (Dot Key)* =  => ActionFn(46);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action46::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                    (0, __symbol, 7)
                })()
            }
            13 => {
                (|| {
                    // (Dot Key)* = (Dot Key)+ => ActionFn(47);
                    let __sym0 = __pop_Variant4(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action47::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                    (1, __symbol, 7)
                })()
            }
            14 => {
                (|| {
                    // (Dot Key)+ = Dot, Key => ActionFn(83);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action83::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                    (2, __symbol, 8)
                })()
            }
            15 => {
                (|| {
                    // (Dot Key)+ = (Dot Key)+, Dot, Key => ActionFn(84);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant4(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action84::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                    (3, __symbol, 8)
                })()
            }
            16 => {
                (|| {
                    // @L =  => ActionFn(45);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action45::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (0, __symbol, 9)
                })()
            }
            17 => {
                (|| {
                    // @R =  => ActionFn(44);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action44::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (0, __symbol, 10)
                })()
            }
            18 => {
                (|| {
                    // Array = L_BRACK, CommaList<Val>, R_BRACK => ActionFn(21);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant5(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action21::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (3, __symbol, 11)
                })()
            }
            19 => {
                (|| {
                    // BareKey = bare_key => ActionFn(106);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action106::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 12)
                })()
            }
            20 => {
                (|| {
                    // BareKey = bare_key_or_number => ActionFn(107);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action107::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 12)
                })()
            }
            21 => {
                (|| {
                    // BareKey = bare_key_or_date => ActionFn(108);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action108::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 12)
                })()
            }
            22 => {
                (|| {
                    // BasicString = basic_string => ActionFn(109);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action109::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 13)
                })()
            }
            23 => {
                (|| {
                    // Bool = bool => ActionFn(110);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action110::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 14)
                })()
            }
            24 => {
                (|| {
                    // Comma = "," => ActionFn(111);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action111::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 15)
                })()
            }
            25 => {
                (|| {
                    // CommaList<KeyVal> = KeyVal => ActionFn(129);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action129::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 16)
                })()
            }
            26 => {
                (|| {
                    // CommaList<KeyVal> =  => ActionFn(130);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action130::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (0, __symbol, 16)
                })()
            }
            27 => {
                (|| {
                    // CommaList<KeyVal> = (<KeyVal> Comma)+, KeyVal => ActionFn(131);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action131::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (2, __symbol, 16)
                })()
            }
            28 => {
                (|| {
                    // CommaList<KeyVal> = (<KeyVal> Comma)+ => ActionFn(132);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action132::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 16)
                })()
            }
            29 => {
                (|| {
                    // CommaList<Val> = Val => ActionFn(137);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action137::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 17)
                })()
            }
            30 => {
                (|| {
                    // CommaList<Val> =  => ActionFn(138);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action138::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (0, __symbol, 17)
                })()
            }
            31 => {
                (|| {
                    // CommaList<Val> = (<Val> Comma)+, Val => ActionFn(139);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action139::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (2, __symbol, 17)
                })()
            }
            32 => {
                (|| {
                    // CommaList<Val> = (<Val> Comma)+ => ActionFn(140);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action140::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 17)
                })()
            }
            33 => {
                (|| {
                    // DateTime = date_time => ActionFn(112);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action112::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 18)
                })()
            }
            34 => {
                (|| {
                    // DateTime = bare_key_or_date => ActionFn(113);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action113::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 18)
                })()
            }
            35 => {
                (|| {
                    // Dict = L_CURLY, CommaList<KeyVal>, R_CURLY => ActionFn(22);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant5(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action22::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (3, __symbol, 19)
                })()
            }
            36 => {
                (|| {
                    // Dot = "." => ActionFn(114);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action114::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 20)
                })()
            }
            37 => {
                (|| {
                    // Eq = "=" => ActionFn(115);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action115::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 21)
                })()
            }
            38 => {
                (|| {
                    // FileForm = KeyVal => ActionFn(2);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action2::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 22)
                })()
            }
            39 => {
                (|| {
                    // FileForm = Table => ActionFn(3);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action3::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 22)
                })()
            }
            40 => {
                (|| {
                    // Key = Key_ => ActionFn(6);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action6::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 23)
                })()
            }
            41 => {
                (|| {
                    // KeyVal = KeyVal_ => ActionFn(4);
                    let __sym0 = __pop_Variant7(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action4::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 24)
                })()
            }
            42 => {
                (|| {
                    // KeyVal* =  => ActionFn(53);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action53::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (0, __symbol, 25)
                })()
            }
            43 => {
                (|| {
                    // KeyVal* = KeyVal+ => ActionFn(54);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action54::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 25)
                })()
            }
            44 => {
                (|| {
                    // KeyVal+ = KeyVal => ActionFn(55);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action55::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 26)
                })()
            }
            45 => {
                (|| {
                    // KeyVal+ = KeyVal+, KeyVal => ActionFn(56);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action56::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (2, __symbol, 26)
                })()
            }
            46 => {
                (|| {
                    // KeyVal? = KeyVal => ActionFn(64);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action64::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                    (1, __symbol, 27)
                })()
            }
            47 => {
                (|| {
                    // KeyVal? =  => ActionFn(65);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action65::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                    (0, __symbol, 27)
                })()
            }
            48 => {
                (|| {
                    // KeyVal_ = Key, Eq, Val => ActionFn(5);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action5::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                    (3, __symbol, 28)
                })()
            }
            49 => {
                (|| {
                    // Key_ = BareKey => ActionFn(7);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action7::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 29)
                })()
            }
            50 => {
                (|| {
                    // Key_ = BasicString => ActionFn(8);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action8::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 29)
                })()
            }
            51 => {
                (|| {
                    // Key_ = LiteralString => ActionFn(9);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action9::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 29)
                })()
            }
            52 => {
                (|| {
                    // L_BRACK = "[" => ActionFn(116);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action116::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 30)
                })()
            }
            53 => {
                (|| {
                    // L_CURLY = "{" => ActionFn(117);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action117::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 31)
                })()
            }
            54 => {
                (|| {
                    // Literal = Number => ActionFn(14);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action14::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 32)
                })()
            }
            55 => {
                (|| {
                    // Literal = Bool => ActionFn(15);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action15::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 32)
                })()
            }
            56 => {
                (|| {
                    // Literal = DateTime => ActionFn(16);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action16::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 32)
                })()
            }
            57 => {
                (|| {
                    // Literal = BasicString => ActionFn(17);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action17::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 32)
                })()
            }
            58 => {
                (|| {
                    // Literal = MultilineBasicString => ActionFn(18);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action18::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 32)
                })()
            }
            59 => {
                (|| {
                    // Literal = LiteralString => ActionFn(19);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action19::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 32)
                })()
            }
            60 => {
                (|| {
                    // Literal = MultilineLiteralString => ActionFn(20);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action20::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 32)
                })()
            }
            61 => {
                (|| {
                    // LiteralString = literal_string => ActionFn(118);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action118::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 33)
                })()
            }
            62 => {
                (|| {
                    // MultilineBasicString = multiline_basic_string => ActionFn(119);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action119::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 34)
                })()
            }
            63 => {
                (|| {
                    // MultilineLiteralString = multiline_literal_string => ActionFn(120);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action120::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 35)
                })()
            }
            64 => {
                (|| {
                    // Number = number => ActionFn(121);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action121::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 36)
                })()
            }
            65 => {
                (|| {
                    // Number = bare_key_or_number => ActionFn(122);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action122::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 36)
                })()
            }
            66 => {
                (|| {
                    // R_BRACK = "]" => ActionFn(123);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action123::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 37)
                })()
            }
            67 => {
                (|| {
                    // R_CURLY = "}" => ActionFn(124);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action124::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 38)
                })()
            }
            68 => {
                (|| {
                    // Table = TableHeader => ActionFn(125);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action125::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 39)
                })()
            }
            69 => {
                (|| {
                    // Table = TableHeader, KeyVal+ => ActionFn(126);
                    let __sym1 = __pop_Variant2(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action126::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (2, __symbol, 39)
                })()
            }
            70 => {
                (|| {
                    // Table* =  => ActionFn(51);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action51::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (0, __symbol, 40)
                })()
            }
            71 => {
                (|| {
                    // Table* = Table+ => ActionFn(52);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action52::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 40)
                })()
            }
            72 => {
                (|| {
                    // Table+ = Table => ActionFn(57);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action57::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 41)
                })()
            }
            73 => {
                (|| {
                    // Table+ = Table+, Table => ActionFn(58);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action58::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (2, __symbol, 41)
                })()
            }
            74 => {
                (|| {
                    // TableHeader = L_BRACK, Key, R_BRACK => ActionFn(85);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action85::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (3, __symbol, 42)
                })()
            }
            75 => {
                (|| {
                    // TableHeader = L_BRACK, Key, (Dot Key)+, R_BRACK => ActionFn(86);
                    let __sym3 = __pop_Variant1(__symbols);
                    let __sym2 = __pop_Variant4(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym3.2.clone();
                    let __nt = super::__action86::<E>(events, input, __sym0, __sym1, __sym2, __sym3);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (4, __symbol, 42)
                })()
            }
            76 => {
                (|| {
                    // TomlFile =  => ActionFn(133);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action133::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (0, __symbol, 43)
                })()
            }
            77 => {
                (|| {
                    // TomlFile = Table+ => ActionFn(134);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action134::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 43)
                })()
            }
            78 => {
                (|| {
                    // TomlFile = KeyVal+ => ActionFn(135);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action135::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 43)
                })()
            }
            79 => {
                (|| {
                    // TomlFile = KeyVal+, Table+ => ActionFn(136);
                    let __sym1 = __pop_Variant2(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action136::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (2, __symbol, 43)
                })()
            }
            80 => {
                (|| {
                    // Val = Val_ => ActionFn(10);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action10::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 44)
                })()
            }
            81 => {
                (|| {
                    // Val? = Val => ActionFn(59);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action59::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                    (1, __symbol, 45)
                })()
            }
            82 => {
                (|| {
                    // Val? =  => ActionFn(60);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action60::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                    (0, __symbol, 45)
                })()
            }
            83 => {
                (|| {
                    // Val_ = Literal => ActionFn(11);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action11::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 46)
                })()
            }
            84 => {
                (|| {
                    // Val_ = Array => ActionFn(12);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action12::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 46)
                })()
            }
            85 => {
                (|| {
                    // Val_ = Dict => ActionFn(13);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action13::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 46)
                })()
            }
            86 => {
                // __TomlFile = TomlFile => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<E>(events, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 48 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ((), ()), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ((), (), ()), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<()>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<()>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<((), ())>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__TomlFile::TomlFileParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use events::Events;
    use symbols;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^((?u:,))",
                "^((?u:\\.))",
                "^((?u:=))",
                "^((?u:\\[))",
                "^((?u:\\]))",
                "^((?u:\\{))",
                "^((?u:\\}))",
                "^(((?u:[0-9]){4}(?u:\\-)(?u:[0-9]){2}(?u:\\-)(?u:[0-9]){2}((?u:[T-Tt-t])((?u:[0-9]){2}(?u::)(?u:[0-9]){2}(?u::)(?u:[0-9]){2}((?u:\\.)(?u:[0-9])+)?))?|((?u:[0-9]){2}(?u::)(?u:[0-9]){2}(?u::)(?u:[0-9]){2}((?u:\\.)(?u:[0-9])+)?))((?u:[Z-Zz-z])|(?u:[-\\+-\\+])(?u:[0-9]){2}(?u::)(?u:[0-9]){2})?)",
                "^((?u:[-0-9A-Z_-_a-z])+)",
                "^((?u:[-\\+-\\+])?((?u:0)|(?u:[1-9])((?u:_)?(?u:[0-9]))*)((?u:\\.)(?u:[0-9])((?u:_)?(?u:[0-9]))*)?((?u:[E-Ee-e])(?u:[-\\+-\\+])?(?u:[1-9])((?u:_)?(?u:[0-9]))*)?)",
                "^((?u:\")((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\"))",
                "^((?u:\"\"\")((?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\"\")(?u:[\u{0}-!\\#-\u{10ffff}]))*(?u:\"\"\"))",
                "^((?u:\')((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\'))",
                "^((?u:\'\'\')((?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\')(?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\'\')(?u:[\u{0}-\\&\\(-\u{10ffff}]))*(?u:\'\'\'))",
                "^((?u:[0-9])+)",
                "^((?u:[0-9]){4}(?u:\\-)(?u:[0-9]){2}(?u:\\-)(?u:[0-9]){2}(?u:[Z-Zz-z])?)",
                "^((?u:false)|(?u:true))",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:,))").unwrap(),
                __regex::Regex::new("^((?u:\\.))").unwrap(),
                __regex::Regex::new("^((?u:=))").unwrap(),
                __regex::Regex::new("^((?u:\\[))").unwrap(),
                __regex::Regex::new("^((?u:\\]))").unwrap(),
                __regex::Regex::new("^((?u:\\{))").unwrap(),
                __regex::Regex::new("^((?u:\\}))").unwrap(),
                __regex::Regex::new("^(((?u:[0-9]){4}(?u:\\-)(?u:[0-9]){2}(?u:\\-)(?u:[0-9]){2}((?u:[T-Tt-t])((?u:[0-9]){2}(?u::)(?u:[0-9]){2}(?u::)(?u:[0-9]){2}((?u:\\.)(?u:[0-9])+)?))?|((?u:[0-9]){2}(?u::)(?u:[0-9]){2}(?u::)(?u:[0-9]){2}((?u:\\.)(?u:[0-9])+)?))((?u:[Z-Zz-z])|(?u:[-\\+-\\+])(?u:[0-9]){2}(?u::)(?u:[0-9]){2})?)").unwrap(),
                __regex::Regex::new("^((?u:[-0-9A-Z_-_a-z])+)").unwrap(),
                __regex::Regex::new("^((?u:[-\\+-\\+])?((?u:0)|(?u:[1-9])((?u:_)?(?u:[0-9]))*)((?u:\\.)(?u:[0-9])((?u:_)?(?u:[0-9]))*)?((?u:[E-Ee-e])(?u:[-\\+-\\+])?(?u:[1-9])((?u:_)?(?u:[0-9]))*)?)").unwrap(),
                __regex::Regex::new("^((?u:\")((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\"))").unwrap(),
                __regex::Regex::new("^((?u:\"\"\")((?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\"\")(?u:[\u{0}-!\\#-\u{10ffff}]))*(?u:\"\"\"))").unwrap(),
                __regex::Regex::new("^((?u:\')((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\'))").unwrap(),
                __regex::Regex::new("^((?u:\'\'\')((?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\')(?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\'\')(?u:[\u{0}-\\&\\(-\u{10ffff}]))*(?u:\'\'\'))").unwrap(),
                __regex::Regex::new("^((?u:[0-9])+)").unwrap(),
                __regex::Regex::new("^((?u:[0-9]){4}(?u:\\-)(?u:[0-9]){2}(?u:\\-)(?u:[0-9]){2}(?u:[Z-Zz-z])?)").unwrap(),
                __regex::Regex::new("^((?u:false)|(?u:true))").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 17 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action1<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, kvs, _): (usize, ::std::vec::Vec<()>, usize),
    (_, ts, _): (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::FILE, kvs.len() + ts.len())
}

#[allow(unused_variables)]
fn __action2<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action3<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action4<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, ((), (), ()), usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::KEY_VAL, 3)
}

#[allow(unused_variables)]
fn __action5<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, (), usize),
    (_, __2, _): (usize, (), usize),
) -> ((), (), ()) where
  E:  Events,
{
    (__0, __1, __2)
}

#[allow(unused_variables)]
fn __action6<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::KEY, 1)
}

#[allow(unused_variables)]
fn __action7<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action8<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action9<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action10<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::VAL, 1)
}

#[allow(unused_variables)]
fn __action11<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action12<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action13<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action14<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action15<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action16<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action17<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action18<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action19<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action20<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    ()
}

#[allow(unused_variables)]
fn __action21<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, xs, _): (usize, usize, usize),
    (_, _, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::ARRAY, xs + 2)
}

#[allow(unused_variables)]
fn __action22<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, xs, _): (usize, usize, usize),
    (_, _, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::DICT, xs + 2)
}

#[allow(unused_variables)]
fn __action23<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, kvs, _): (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::TABLE, 1 + kvs.len())
}

#[allow(unused_variables)]
fn __action24<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, (), usize),
    (_, keys, _): (usize, ::std::vec::Vec<((), ())>, usize),
    (_, _, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::TABLE_HEADER, 3 + keys.len() * 2)
}

#[allow(unused_variables)]
fn __action25<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::EQ, l, r)
}

#[allow(unused_variables)]
fn __action26<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::DOT, l, r)
}

#[allow(unused_variables)]
fn __action27<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::COMMA, l, r)
}

#[allow(unused_variables)]
fn __action28<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::L_BRACK, l, r)
}

#[allow(unused_variables)]
fn __action29<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::R_BRACK, l, r)
}

#[allow(unused_variables)]
fn __action30<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::L_CURLY, l, r)
}

#[allow(unused_variables)]
fn __action31<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::R_CURLY, l, r)
}

#[allow(unused_variables)]
fn __action32<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::NUMBER, l, r)
}

#[allow(unused_variables)]
fn __action33<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::NUMBER, l, r)
}

#[allow(unused_variables)]
fn __action34<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::DATE_TIME, l, r)
}

#[allow(unused_variables)]
fn __action35<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::DATE_TIME, l, r)
}

#[allow(unused_variables)]
fn __action36<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::BOOL, l, r)
}

#[allow(unused_variables)]
fn __action37<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::BARE_KEY, l, r)
}

#[allow(unused_variables)]
fn __action38<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::BARE_KEY, l, r)
}

#[allow(unused_variables)]
fn __action39<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::BARE_KEY, l, r)
}

#[allow(unused_variables)]
fn __action40<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::BASIC_STRING, l, r)
}

#[allow(unused_variables)]
fn __action41<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::MULTILINE_BASIC_STRING, l, r)
}

#[allow(unused_variables)]
fn __action42<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::LITERAL_STRING, l, r)
}

#[allow(unused_variables)]
fn __action43<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::MULTILINE_LITERAL_STRING, l, r)
}

#[allow(unused_variables)]
fn __action44<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  Events,
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action45<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  Events,
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action46<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<((), ())> where
  E:  Events,
{
    vec![]
}

#[allow(unused_variables)]
fn __action47<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<((), ())>, usize),
) -> ::std::vec::Vec<((), ())> where
  E:  Events,
{
    v
}

#[allow(unused_variables)]
fn __action48<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, (), usize),
) -> ((), ()) where
  E:  Events,
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action49<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
    (_, e, _): (usize, ::std::option::Option<()>, usize),
) -> usize where
  E:  Events,
{
    v.len() * 2 + e.into_iter().count()
}

#[allow(unused_variables)]
fn __action50<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
    (_, e, _): (usize, ::std::option::Option<()>, usize),
) -> usize where
  E:  Events,
{
    v.len() * 2 + e.into_iter().count()
}

#[allow(unused_variables)]
fn __action51<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![]
}

#[allow(unused_variables)]
fn __action52<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    v
}

#[allow(unused_variables)]
fn __action53<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![]
}

#[allow(unused_variables)]
fn __action54<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    v
}

#[allow(unused_variables)]
fn __action55<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action56<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
    (_, e, _): (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action57<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action58<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
    (_, e, _): (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action59<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ::std::option::Option<()> where
  E:  Events,
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action60<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<()> where
  E:  Events,
{
    None
}

#[allow(unused_variables)]
fn __action61<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![]
}

#[allow(unused_variables)]
fn __action62<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    v
}

#[allow(unused_variables)]
fn __action63<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, _, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    (__0)
}

#[allow(unused_variables)]
fn __action64<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ::std::option::Option<()> where
  E:  Events,
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action65<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<()> where
  E:  Events,
{
    None
}

#[allow(unused_variables)]
fn __action66<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![]
}

#[allow(unused_variables)]
fn __action67<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    v
}

#[allow(unused_variables)]
fn __action68<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, _, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    (__0)
}

#[allow(unused_variables)]
fn __action69<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, ((), ()), usize),
) -> ::std::vec::Vec<((), ())> where
  E:  Events,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action70<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<((), ())>, usize),
    (_, e, _): (usize, ((), ()), usize),
) -> ::std::vec::Vec<((), ())> where
  E:  Events,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action71<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action72<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
    (_, e, _): (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action73<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action74<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<()>, usize),
    (_, e, _): (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action75<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
    __1: (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action68(
        events,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action76<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, (), usize),
    __2: (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action68(
        events,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action77<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::option::Option<()>, usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action66(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action78<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, ::std::option::Option<()>, usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action67(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        events,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action79<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
    __1: (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action63(
        events,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action80<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, (), usize),
    __2: (usize, (), usize),
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action63(
        events,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action81<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::option::Option<()>, usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action61(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action82<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, ::std::option::Option<()>, usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action62(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        events,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action83<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
    __1: (usize, (), usize),
) -> ::std::vec::Vec<((), ())> where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        events,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action84<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<((), ())>, usize),
    __1: (usize, (), usize),
    __2: (usize, (), usize),
) -> ::std::vec::Vec<((), ())> where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        events,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action85<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
    __1: (usize, (), usize),
    __2: (usize, (), usize),
) -> () where
  E:  Events,
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action46(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        events,
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action86<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
    __1: (usize, (), usize),
    __2: (usize, ::std::vec::Vec<((), ())>, usize),
    __3: (usize, (), usize),
) -> () where
  E:  Events,
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action47(
        events,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        events,
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action87<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action88<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action89<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action90<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action91<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action92<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action93<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action94<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action95<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action96<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action97<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action98<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action99<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action100<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action101<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action102<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action103<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action104<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action105<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action106<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action107<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action88(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action108<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action89(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action109<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action90(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action110<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action91(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action111<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action92(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action112<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action93(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action113<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action94(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action114<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action95(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action115<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action96(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action116<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action97(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action117<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action98(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action118<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action99(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action119<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action100(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action120<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action101(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action121<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action102(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action122<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action103(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action123<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action104(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action124<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action105(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action125<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action53(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action126<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
    __1: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action54(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action127<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action53(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action128<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action54(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        events,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action129<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action64(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action130<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  Events,
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action65(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action131<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, (), usize),
) -> usize where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action64(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action132<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action65(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action78(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action133<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> () where
  E:  Events,
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action51(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action127(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action134<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action52(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action127(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action135<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action51(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action128(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action136<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action52(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action128(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action137<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, (), usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action59(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action138<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  Events,
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action60(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action139<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, (), usize),
) -> usize where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action59(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action140<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<()>, usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action60(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action82(
        events,
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, E, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, E, > __ToTriple<'input, E, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, E, > __ToTriple<'input, E, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
