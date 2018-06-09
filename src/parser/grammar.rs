// auto-generated: "lalrpop 0.15.2"
// sha256: b27172711e4041ccb6d740e6e8ae3c4fbc675af7271ff28bb77671a77cd464b7
use super::Events;
use *;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__TomlFile {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use super::super::Events;
    use *;
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
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 1
        0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, -25, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 5
        0, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, -54, 0, 0, 0, -54, -54, -54, -54, 0, 0, -54, 0, 0, 0,
        // State 9
        0, 0, 0, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 10
        0, 0, 0, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 11
        0, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, -74, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, -58, -58, -58, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58,
        // State 16
        0, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        -32, -32, -32, -32, -32, 0, -32, -32, -32, -32, -32, 0, 0, -32, 0, 0, 0,
        // State 20
        -67, -67, -67, -67, -67, 0, -67, -67, -67, -67, -67, 0, 0, -67, 0, 0, 0,
        // State 21
        0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, -26, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 23
        0, 0, 0, 16, 0, 44, 0, 0, 45, 46, 20, 47, 48, 21, 49, 50, 51,
        // State 24
        0, 0, 0, -47, 0, -47, 0, 0, -47, -47, -47, -47, -47, -47, -47, -47, -47,
        // State 25
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, -55, 0, 0, 0, -55, -55, -55, -55, 0, 0, -55, 0, 0, 0,
        // State 27
        0, 55, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 29
        0, 0, 0, -75, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 30
        -83, 0, 0, -83, -83, 0, -83, -83, -83, -83, -83, 0, 0, -83, 0, 0, 0,
        // State 31
        -63, 0, 0, -63, -63, 0, -63, -63, -63, -63, -63, 0, 0, -63, 0, 0, 0,
        // State 32
        -61, 0, 0, -61, -61, 0, -61, -61, -61, -61, -61, 0, 0, -61, 0, 0, 0,
        // State 33
        -62, 0, 0, -62, -62, 0, -62, -62, -62, -62, -62, 0, 0, -62, 0, 0, 0,
        // State 34
        -84, 0, 0, -84, -84, 0, -84, -84, -84, -84, -84, 0, 0, -84, 0, 0, 0,
        // State 35
        0, 0, 0, 16, -40, 44, 0, 0, 45, 46, 20, 47, 48, 21, 49, 50, 51,
        // State 36
        0, 0, 0, 0, 0, 0, -36, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 37
        -82, 0, 0, -82, -82, 0, -82, -82, -82, -82, -82, 0, 0, -82, 0, 0, 0,
        // State 38
        -65, 0, 0, -65, -65, 0, -65, -65, -65, -65, -65, 0, 0, -65, 0, 0, 0,
        // State 39
        -64, 0, 0, -64, -64, 0, -64, -64, -64, -64, -64, 0, 0, -64, 0, 0, 0,
        // State 40
        -66, 0, 0, -66, -66, 0, -66, -66, -66, -66, -66, 0, 0, -66, 0, 0, 0,
        // State 41
        -60, 0, 0, -60, -60, 0, -60, -60, -60, -60, -60, 0, 0, -60, 0, 0, 0,
        // State 42
        -51, 0, 0, -51, 0, 0, -51, -51, -51, -51, -51, 0, 0, -51, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, -59, -59, -59, -59, -59, 0, 0, -59, 0, 0, 0,
        // State 44
        -44, 0, 0, -44, -44, 0, -44, -44, -44, -44, -44, 0, 0, -44, 0, 0, 0,
        // State 45
        -71, 0, 0, -71, -71, 0, -71, -71, -71, -71, -71, 0, 0, -71, 0, 0, 0,
        // State 46
        -33, 0, 0, -33, -33, 0, -33, -33, -33, -33, -33, 0, 0, -33, 0, 0, 0,
        // State 47
        -43, 0, 0, -43, -43, 0, -43, -43, -43, -43, -43, 0, 0, -43, 0, 0, 0,
        // State 48
        -68, 0, 0, -68, -68, 0, -68, -68, -68, -68, -68, 0, 0, -68, 0, 0, 0,
        // State 49
        -69, 0, 0, -69, -69, 0, -69, -69, -69, -69, -69, 0, 0, -69, 0, 0, 0,
        // State 50
        -70, 0, 0, -70, -70, 0, -70, -70, -70, -70, -70, 0, 0, -70, 0, 0, 0,
        // State 51
        0, 55, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 53
        0, 0, 0, -76, 0, 0, 0, -76, -76, -76, -76, 0, 0, -76, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, -46, -46, -46, -46, 0, 0, -46, 0, 0, 0,
        // State 55
        -72, 0, 0, -72, -72, 0, -72, -72, -72, -72, -72, 0, 0, -72, 0, 0, 0,
        // State 56
        0, 55, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 16, -42, 44, 0, 0, 45, 46, 20, 47, 48, 21, 49, 50, 51,
        // State 58
        0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        72, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, -38, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        72, 0, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 17, 18, 19, 20, 0, 0, 21, 0, 0, 0,
        // State 64
        0, 0, 0, -77, 0, 0, 0, -77, -77, -77, -77, 0, 0, -77, 0, 0, 0,
        // State 65
        0, -14, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 55, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        72, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        -24, 0, 0, -24, -24, 0, -24, -24, -24, -24, -24, 0, 0, -24, 0, 0, 0,
        // State 70
        0, 0, 0, -9, -9, -9, 0, 0, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 71
        0, 0, 0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 72
        72, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        -45, 0, 0, -45, -45, 0, -45, -45, -45, -45, -45, 0, 0, -45, 0, 0, 0,
        // State 74
        -73, 0, 0, -73, -73, 0, -73, -73, -73, -73, -73, 0, 0, -73, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, 0, 0, -4, 0, 0, 0,
        // State 76
        0, -15, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, -27, 0, 0, 0, -27, -27, -27, -27, 0, 0, -27, 0, 0, 0,
        // State 79
        0, 0, 0, -10, -10, -10, 0, 0, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 80
        0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, 0, 0, -5, 0, 0, 0,
        // State 81
        0, 0, 0, -28, 0, 0, 0, -28, -28, -28, -28, 0, 0, -28, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -78,
        // State 1
        -22,
        // State 2
        -80,
        // State 3
        -19,
        // State 4
        -25,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        -54,
        // State 9
        -79,
        // State 10
        0,
        // State 11
        0,
        // State 12
        -18,
        // State 13
        -74,
        // State 14
        -87,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -32,
        // State 20
        -67,
        // State 21
        -23,
        // State 22
        -26,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -81,
        // State 26
        -55,
        // State 27
        0,
        // State 28
        0,
        // State 29
        -75,
        // State 30
        -83,
        // State 31
        -63,
        // State 32
        -61,
        // State 33
        -62,
        // State 34
        -84,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -82,
        // State 38
        -65,
        // State 39
        -64,
        // State 40
        -66,
        // State 41
        -60,
        // State 42
        -51,
        // State 43
        0,
        // State 44
        -44,
        // State 45
        -71,
        // State 46
        -33,
        // State 47
        -43,
        // State 48
        -68,
        // State 49
        -69,
        // State 50
        -70,
        // State 51
        0,
        // State 52
        0,
        // State 53
        -76,
        // State 54
        0,
        // State 55
        -72,
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
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        -77,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        -24,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        -45,
        // State 74
        -73,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        -27,
        // State 79
        0,
        // State 80
        0,
        // State 81
        -28,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 0, 10, 0, 11, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 15, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 0, 23, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 26, 0, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 8, 27, 0, 0, 0, 11, 0, 0, 12, 0, 0, 0, 0, 0, 13, 14, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 29, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 0, 30, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 8, 27, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 32, 33, 0, 0, 0, 34, 35, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 39, 40, 41, 42, 0, 0, 0, 0, 0, 43, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 8, 27, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 32, 33, 0, 0, 59, 34, 35, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 39, 40, 41, 42, 0, 0, 0, 0, 0, 60, 0, 0,
        // State 36
        0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 62, 0, 0, 0, 0, 0, 8, 63, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 32, 33, 0, 0, 0, 34, 35, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 39, 40, 41, 42, 0, 0, 0, 0, 0, 69, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 8, 73, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                    Token(16, _) if true => 8,
                    Token(15, _) if true => 9,
                    Token(10, _) if true => 10,
                    Token(14, _) if true => 11,
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
                                Token(16, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(15, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
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
                __reduce1(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            2 => {
                __reduce2(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            3 => {
                __reduce3(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            4 => {
                __reduce4(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            5 => {
                __reduce5(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            6 => {
                __reduce6(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            7 => {
                __reduce7(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            8 => {
                __reduce8(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            9 => {
                __reduce9(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            10 => {
                __reduce10(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            11 => {
                __reduce11(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            12 => {
                __reduce12(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            13 => {
                __reduce13(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            14 => {
                __reduce14(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            15 => {
                __reduce15(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            16 => {
                __reduce16(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            17 => {
                __reduce17(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            18 => {
                __reduce18(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            19 => {
                __reduce19(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            20 => {
                __reduce20(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            21 => {
                __reduce21(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            22 => {
                __reduce22(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            23 => {
                __reduce23(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            24 => {
                __reduce24(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            25 => {
                __reduce25(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            26 => {
                __reduce26(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            27 => {
                __reduce27(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            28 => {
                __reduce28(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            29 => {
                __reduce29(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            30 => {
                __reduce30(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            31 => {
                __reduce31(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            32 => {
                __reduce32(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            33 => {
                __reduce33(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            34 => {
                __reduce34(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            35 => {
                __reduce35(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            36 => {
                __reduce36(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            37 => {
                __reduce37(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            38 => {
                __reduce38(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            39 => {
                __reduce39(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            40 => {
                __reduce40(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            41 => {
                __reduce41(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            42 => {
                __reduce42(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            43 => {
                __reduce43(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            44 => {
                __reduce44(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            45 => {
                __reduce45(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            46 => {
                __reduce46(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            47 => {
                __reduce47(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            48 => {
                __reduce48(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            49 => {
                __reduce49(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            50 => {
                __reduce50(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            51 => {
                __reduce51(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            52 => {
                __reduce52(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            53 => {
                __reduce53(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            54 => {
                __reduce54(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            55 => {
                __reduce55(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            56 => {
                __reduce56(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            57 => {
                __reduce57(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            58 => {
                __reduce58(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            59 => {
                __reduce59(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            60 => {
                __reduce60(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            61 => {
                __reduce61(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            62 => {
                __reduce62(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            63 => {
                __reduce63(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            64 => {
                __reduce64(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            65 => {
                __reduce65(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            66 => {
                __reduce66(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            67 => {
                __reduce67(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            68 => {
                __reduce68(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            69 => {
                __reduce69(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            70 => {
                __reduce70(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            71 => {
                __reduce71(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            72 => {
                __reduce72(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            73 => {
                __reduce73(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            74 => {
                __reduce74(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            75 => {
                __reduce75(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            76 => {
                __reduce76(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            77 => {
                __reduce77(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            78 => {
                __reduce78(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            79 => {
                __reduce79(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            80 => {
                __reduce80(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            81 => {
                __reduce81(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            82 => {
                __reduce82(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            83 => {
                __reduce83(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            84 => {
                __reduce84(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            85 => {
                __reduce85(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            86 => {
                __reduce86(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            87 => {
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
        let __next_state = __GOTO[__state * 47 + __nonterminal] - 1;
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
    pub(crate) fn __reduce1<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<KeyVal> Comma) = KeyVal, Comma => ActionFn(67);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action67::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<KeyVal> Comma)* =  => ActionFn(65);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action65::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<KeyVal> Comma)* = (<KeyVal> Comma)+ => ActionFn(66);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<KeyVal> Comma)+ = KeyVal, Comma => ActionFn(74);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action74::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<KeyVal> Comma)+ = (<KeyVal> Comma)+, KeyVal, Comma => ActionFn(75);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action75::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<Val> Comma) = Val, Comma => ActionFn(62);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<Val> Comma)* =  => ActionFn(60);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action60::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<Val> Comma)* = (<Val> Comma)+ => ActionFn(61);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<Val> Comma)+ = Val, Comma => ActionFn(78);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action78::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce10<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (<Val> Comma)+ = (<Val> Comma)+, Val, Comma => ActionFn(79);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action79::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 5)
    }
    pub(crate) fn __reduce11<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (Dot Key) = Dot, Key => ActionFn(47);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action47::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 6)
    }
    pub(crate) fn __reduce12<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (Dot Key)* =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (Dot Key)* = (Dot Key)+ => ActionFn(46);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (Dot Key)+ = Dot, Key => ActionFn(82);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce15<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // (Dot Key)+ = (Dot Key)+, Dot, Key => ActionFn(83);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action83::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 8)
    }
    pub(crate) fn __reduce16<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // @L =  => ActionFn(44);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action44::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 9)
    }
    pub(crate) fn __reduce17<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // @R =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce18<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // AnyTable = Table => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce19<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // AnyTable = ArrayTable => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce20<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // AnyTable* =  => ActionFn(50);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action50::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (0, __symbol, 12)
    }
    pub(crate) fn __reduce21<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // AnyTable* = AnyTable+ => ActionFn(51);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action51::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce22<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // AnyTable+ = AnyTable => ActionFn(56);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce23<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // AnyTable+ = AnyTable+, AnyTable => ActionFn(57);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action57::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 13)
    }
    pub(crate) fn __reduce24<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Array = L_BRACK, CommaList<Val>, R_BRACK => ActionFn(18);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce25<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // ArrayTable = ArrayTableHeader => ActionFn(128);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action128::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce26<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // ArrayTable = ArrayTableHeader, KeyVal+ => ActionFn(129);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action129::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce27<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // ArrayTableHeader = L_BRACK, L_BRACK, Key, R_BRACK, R_BRACK => ActionFn(84);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action84::<E>(events, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (5, __symbol, 16)
    }
    pub(crate) fn __reduce28<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // ArrayTableHeader = L_BRACK, L_BRACK, Key, (Dot Key)+, R_BRACK, R_BRACK => ActionFn(85);
        let __sym5 = __pop_Variant1(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action85::<E>(events, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (6, __symbol, 16)
    }
    pub(crate) fn __reduce29<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // BareKey = bare_key => ActionFn(107);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action107::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce30<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // BareKey = bare_key_or_number => ActionFn(108);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action108::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce31<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // BareKey = bare_key_or_date => ActionFn(109);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action109::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce32<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // BasicString = basic_string => ActionFn(110);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action110::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce33<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Bool = bool => ActionFn(111);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action111::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce34<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Comma = "," => ActionFn(112);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action112::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce35<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // CommaList<KeyVal> = KeyVal => ActionFn(136);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action136::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce36<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // CommaList<KeyVal> =  => ActionFn(137);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action137::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 21)
    }
    pub(crate) fn __reduce37<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // CommaList<KeyVal> = (<KeyVal> Comma)+, KeyVal => ActionFn(138);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action138::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce38<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // CommaList<KeyVal> = (<KeyVal> Comma)+ => ActionFn(139);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action139::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce39<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // CommaList<Val> = Val => ActionFn(140);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action140::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce40<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // CommaList<Val> =  => ActionFn(141);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action141::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 22)
    }
    pub(crate) fn __reduce41<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // CommaList<Val> = (<Val> Comma)+, Val => ActionFn(142);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action142::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 22)
    }
    pub(crate) fn __reduce42<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // CommaList<Val> = (<Val> Comma)+ => ActionFn(143);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action143::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce43<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // DateTime = date_time => ActionFn(113);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action113::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce44<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // DateTime = bare_key_or_date => ActionFn(114);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action114::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce45<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Dict = L_CURLY, CommaList<KeyVal>, R_CURLY => ActionFn(19);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce46<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Dot = "." => ActionFn(115);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action115::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce47<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Eq = "=" => ActionFn(116);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action116::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce48<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Key = BareKey => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce49<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Key = BasicString => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce50<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Key = LiteralString => ActionFn(7);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce51<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // KeyVal = Key, Eq, Val => ActionFn(4);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 28)
    }
    pub(crate) fn __reduce52<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // KeyVal* =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (0, __symbol, 29)
    }
    pub(crate) fn __reduce53<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // KeyVal* = KeyVal+ => ActionFn(53);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce54<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // KeyVal+ = KeyVal => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce55<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // KeyVal+ = KeyVal+, KeyVal => ActionFn(55);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action55::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 30)
    }
    pub(crate) fn __reduce56<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // KeyVal? = KeyVal => ActionFn(63);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action63::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 31)
    }
    pub(crate) fn __reduce57<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // KeyVal? =  => ActionFn(64);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action64::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 31)
    }
    pub(crate) fn __reduce58<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // L_BRACK = "[" => ActionFn(117);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action117::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 32)
    }
    pub(crate) fn __reduce59<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // L_CURLY = "{" => ActionFn(118);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action118::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 33)
    }
    pub(crate) fn __reduce60<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Literal = Number => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 34)
    }
    pub(crate) fn __reduce61<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Literal = Bool => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 34)
    }
    pub(crate) fn __reduce62<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Literal = DateTime => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 34)
    }
    pub(crate) fn __reduce63<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Literal = BasicString => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 34)
    }
    pub(crate) fn __reduce64<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Literal = MultilineBasicString => ActionFn(15);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 34)
    }
    pub(crate) fn __reduce65<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Literal = LiteralString => ActionFn(16);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 34)
    }
    pub(crate) fn __reduce66<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Literal = MultilineLiteralString => ActionFn(17);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 34)
    }
    pub(crate) fn __reduce67<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // LiteralString = literal_string => ActionFn(119);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action119::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 35)
    }
    pub(crate) fn __reduce68<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // MultilineBasicString = multiline_basic_string => ActionFn(120);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action120::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 36)
    }
    pub(crate) fn __reduce69<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // MultilineLiteralString = multiline_literal_string => ActionFn(121);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action121::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 37)
    }
    pub(crate) fn __reduce70<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Number = number => ActionFn(122);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action122::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 38)
    }
    pub(crate) fn __reduce71<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Number = bare_key_or_number => ActionFn(123);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action123::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 38)
    }
    pub(crate) fn __reduce72<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // R_BRACK = "]" => ActionFn(124);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action124::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 39)
    }
    pub(crate) fn __reduce73<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // R_CURLY = "}" => ActionFn(125);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action125::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 40)
    }
    pub(crate) fn __reduce74<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Table = TableHeader => ActionFn(130);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action130::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 41)
    }
    pub(crate) fn __reduce75<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Table = TableHeader, KeyVal+ => ActionFn(131);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action131::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 41)
    }
    pub(crate) fn __reduce76<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // TableHeader = L_BRACK, Key, R_BRACK => ActionFn(86);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action86::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 42)
    }
    pub(crate) fn __reduce77<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // TableHeader = L_BRACK, Key, (Dot Key)+, R_BRACK => ActionFn(87);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action87::<E>(events, input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 42)
    }
    pub(crate) fn __reduce78<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // TomlFile =  => ActionFn(132);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action132::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (0, __symbol, 43)
    }
    pub(crate) fn __reduce79<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // TomlFile = KeyVal+ => ActionFn(133);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action133::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 43)
    }
    pub(crate) fn __reduce80<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // TomlFile = AnyTable+ => ActionFn(134);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action134::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 43)
    }
    pub(crate) fn __reduce81<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // TomlFile = KeyVal+, AnyTable+ => ActionFn(135);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action135::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 43)
    }
    pub(crate) fn __reduce82<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Val = Literal => ActionFn(8);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 44)
    }
    pub(crate) fn __reduce83<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Val = Array => ActionFn(9);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 44)
    }
    pub(crate) fn __reduce84<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Val = Dict => ActionFn(10);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 44)
    }
    pub(crate) fn __reduce85<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Val? = Val => ActionFn(58);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 45)
    }
    pub(crate) fn __reduce86<
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
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  Events,
    {
        // Val? =  => ActionFn(59);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action59::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 45)
    }
}
pub use self::__parse__TomlFile::TomlFileParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use super::Events;
    use *;
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
                "^(((?u:false)|(?u:true)))",
                "^((?u:[0-9])+)",
                "^((?u:[0-9]){4}(?u:\\-)(?u:[0-9]){2}(?u:\\-)(?u:[0-9]){2}(?u:[Z-Zz-z])?)",
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
                __regex::Regex::new("^(((?u:false)|(?u:true)))").unwrap(),
                __regex::Regex::new("^((?u:[0-9])+)").unwrap(),
                __regex::Regex::new("^((?u:[0-9]){4}(?u:\\-)(?u:[0-9]){2}(?u:\\-)(?u:[0-9]){2}(?u:[Z-Zz-z])?)").unwrap(),
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
    events.reduce(FILE, kvs.len() + ts.len())
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
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, (), usize),
    (_, __2, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(KEY_VAL, 3)
}

#[allow(unused_variables)]
fn __action5<
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
    ()
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
    ()
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
    (_, _, _): (usize, (), usize),
    (_, xs, _): (usize, usize, usize),
    (_, _, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(ARRAY, xs + 2)
}

#[allow(unused_variables)]
fn __action19<
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
    events.reduce(DICT, xs + 2)
}

#[allow(unused_variables)]
fn __action20<
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
    events.reduce(TABLE, 1 + kvs.len())
}

#[allow(unused_variables)]
fn __action21<
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
    events.reduce(TABLE_HEADER, 3 + keys.len() * 2)
}

#[allow(unused_variables)]
fn __action22<
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
    events.reduce(ARRAY_TABLE, 1 + kvs.len())
}

#[allow(unused_variables)]
fn __action23<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, (), usize),
    (_, keys, _): (usize, ::std::vec::Vec<((), ())>, usize),
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(TABLE_HEADER, 5 + keys.len() * 2)
}

#[allow(unused_variables)]
fn __action24<
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
    events.shift(EQ, l, r)
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
    events.shift(DOT, l, r)
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
    events.shift(COMMA, l, r)
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
    events.shift(L_BRACK, l, r)
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
    events.shift(R_BRACK, l, r)
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
    events.shift(L_CURLY, l, r)
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
    events.shift(R_CURLY, l, r)
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
    events.shift(NUMBER, l, r)
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
    events.shift(NUMBER, l, r)
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
    events.shift(DATE_TIME, l, r)
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
    events.shift(DATE_TIME, l, r)
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
    events.shift(BOOL, l, r)
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
    events.shift(BARE_KEY, l, r)
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
    events.shift(BARE_KEY, l, r)
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
    events.shift(BARE_KEY, l, r)
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
    events.shift(BASIC_STRING, l, r)
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
    events.shift(MULTILINE_BASIC_STRING, l, r)
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
    events.shift(LITERAL_STRING, l, r)
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
    events.shift(MULTILINE_LITERAL_STRING, l, r)
}

#[allow(unused_variables)]
fn __action43<
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
    __lookahead.clone()
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
) -> ::std::vec::Vec<((), ())> where
  E:  Events,
{
    vec![]
}

#[allow(unused_variables)]
fn __action46<
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
fn __action47<
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
fn __action48<
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<()> where
  E:  Events,
{
    vec![]
}

#[allow(unused_variables)]
fn __action51<
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
fn __action52<
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
fn __action53<
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
fn __action54<
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
fn __action55<
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
fn __action56<
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
fn __action57<
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
fn __action58<
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
fn __action59<
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
fn __action60<
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
fn __action61<
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
fn __action62<
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
fn __action63<
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
fn __action64<
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
fn __action65<
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
fn __action66<
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
fn __action67<
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
fn __action68<
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
fn __action69<
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
fn __action70<
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
fn __action71<
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
fn __action72<
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
fn __action73<
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
fn __action74<
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
    let __temp0 = __action67(
        events,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action75<
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
    let __temp0 = __action67(
        events,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action71(
        events,
        input,
        __0,
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
    __0: (usize, ::std::option::Option<()>, usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action65(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action77<
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
    let __temp0 = __action66(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        events,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action78<
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
    let __temp0 = __action62(
        events,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action72(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action79<
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
    let __temp0 = __action62(
        events,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        events,
        input,
        __0,
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
    __0: (usize, ::std::option::Option<()>, usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action60(
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
fn __action81<
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
    let __temp0 = __action61(
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
fn __action82<
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
    let __temp0 = __action47(
        events,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action83<
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
    let __temp0 = __action47(
        events,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        events,
        input,
        __0,
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
    __0: (usize, (), usize),
    __1: (usize, (), usize),
    __2: (usize, (), usize),
    __3: (usize, (), usize),
    __4: (usize, (), usize),
) -> () where
  E:  Events,
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action45(
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
        __1,
        __2,
        __temp0,
        __3,
        __4,
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
    __3: (usize, ::std::vec::Vec<((), ())>, usize),
    __4: (usize, (), usize),
    __5: (usize, (), usize),
) -> () where
  E:  Events,
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action46(
        events,
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        events,
        input,
        __0,
        __1,
        __2,
        __temp0,
        __4,
        __5,
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
    __2: (usize, (), usize),
) -> () where
  E:  Events,
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action45(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        events,
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action87<
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
    let __temp0 = __action46(
        events,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        events,
        input,
        __0,
        __1,
        __temp0,
        __3,
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
    let __temp0 = __action44(
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
fn __action106<
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
    let __temp0 = __action44(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    let __temp0 = __action43(
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
    __0: (usize, &'input str, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action43(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action106(
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
    __0: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action50(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
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
    __1: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action51(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action128<
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
    let __temp0 = __action52(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        events,
        input,
        __0,
        __temp0,
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
    __1: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action53(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        events,
        input,
        __0,
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
    __0: (usize, (), usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action52(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        events,
        input,
        __0,
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
    __0: (usize, (), usize),
    __1: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action53(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> () where
  E:  Events,
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action52(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action126(
        events,
        input,
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
    __0: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action53(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action126(
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
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
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
        __0,
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
    __1: (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action53(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action127(
        events,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action136<
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
    let __temp0 = __action63(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        events,
        input,
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  Events,
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action64(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
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
    __0: (usize, ::std::vec::Vec<()>, usize),
    __1: (usize, (), usize),
) -> usize where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action63(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        events,
        input,
        __0,
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
) -> usize where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action64(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
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
    __0: (usize, (), usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action141<
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
    let __temp0 = __action59(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action80(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action142<
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
    let __temp0 = __action58(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action143<
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
    let __temp0 = __action59(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action81(
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
