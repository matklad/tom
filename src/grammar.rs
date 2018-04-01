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
        Variant3(usize),
        Variant4(((), (), ())),
        Variant5(::std::option::Option<()>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 26, 27, 28, 0, 0, 0, 29, 0, 30, 31, 32, 33, 0, 0,
        // State 13
        0, -18, -18, -18, 0, 0, 0, -18, 0, -18, -18, -18, -18, 0, 0,
        // State 14
        -47, -47, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0,
        // State 15
        -31, -31, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0,
        // State 16
        -30, -30, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0,
        // State 17
        0, 26, 27, 28, 0, 0, 0, 29, 0, 30, 31, 32, 33, -15, 0,
        // State 18
        -46, -46, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0,
        // State 19
        -32, -32, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0,
        // State 20
        -33, -33, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0,
        // State 21
        -34, -34, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0,
        // State 22
        -29, -29, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0,
        // State 23
        -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        -43, -43, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0,
        // State 25
        -39, -39, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0,
        // State 26
        -11, -11, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0,
        // State 27
        -12, -12, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0,
        // State 28
        0, -28, -28, -28, 0, 0, 0, -28, 0, -28, -28, -28, -28, -28, 0,
        // State 29
        -35, -35, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0,
        // State 30
        -36, -36, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0,
        // State 31
        -37, -37, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0,
        // State 32
        -38, -38, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0,
        // State 33
        0, 26, 27, 28, 0, 0, 0, 29, 0, 30, 31, 32, 33, -17, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0,
        // State 35
        0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0,
        // State 36
        0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0,
        // State 37
        -8, -8, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0,
        // State 38
        -40, -40, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0,
        // State 39
        0, -4, -4, -4, 0, 0, 0, -4, 0, -4, -4, -4, -4, -4, 0,
        // State 40
        0, -13, -13, -13, 0, 0, 0, -13, 0, -13, -13, -13, -13, -13, 0,
        // State 41
        0, -5, -5, -5, 0, 0, 0, -5, 0, -5, -5, -5, -5, -5, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -41,
        // State 1
        0,
        // State 2
        -22,
        // State 3
        -42,
        // State 4
        0,
        // State 5
        -19,
        // State 6
        -25,
        // State 7
        0,
        // State 8
        -48,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -23,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -47,
        // State 15
        -31,
        // State 16
        -30,
        // State 17
        0,
        // State 18
        -46,
        // State 19
        -32,
        // State 20
        -33,
        // State 21
        -34,
        // State 22
        -29,
        // State 23
        -26,
        // State 24
        -43,
        // State 25
        -39,
        // State 26
        -11,
        // State 27
        -12,
        // State 28
        0,
        // State 29
        -35,
        // State 30
        -36,
        // State 31
        -37,
        // State 32
        -38,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -8,
        // State 38
        -40,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 12, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 15, 0, 16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 19, 20, 21, 22, 23, 0, 0, 24, 0, 25, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 34, 0, 0, 15, 0, 16, 17, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 18, 19, 20, 21, 22, 23, 0, 0, 36, 0, 25, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 15, 0, 16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 19, 20, 21, 22, 23, 0, 0, 37, 0, 25, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"bare_key"###,
            r###"bare_key_or_number"###,
            r###"basic_string"###,
            r###"bool"###,
            r###"comma"###,
            r###"dot"###,
            r###"eq"###,
            r###"l_brack"###,
            r###"l_curly"###,
            r###"literal_string"###,
            r###"multiline_basic_string"###,
            r###"multiline_literal_string"###,
            r###"number"###,
            r###"r_brack"###,
            r###"r_curly"###,
        ];
        __ACTION[(__state * 15)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                    Token(6, _) if true => 1,
                    Token(2, _) if true => 2,
                    Token(7, _) if true => 3,
                    Token(8, _) if true => 4,
                    Token(9, _) if true => 5,
                    Token(10, _) if true => 6,
                    Token(11, _) if true => 7,
                    Token(13, _) if true => 8,
                    Token(4, _) if true => 9,
                    Token(3, _) if true => 10,
                    Token(5, _) if true => 11,
                    Token(1, _) if true => 12,
                    Token(12, _) if true => 13,
                    Token(14, _) if true => 14,
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
                    let __action = __ACTION[__state * 15 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(13, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
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
                    // (<Val> Comma) = Val, Comma => ActionFn(41);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action41::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (2, __symbol, 0)
                })()
            }
            2 => {
                (|| {
                    // (<Val> Comma)* =  => ActionFn(39);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action39::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (0, __symbol, 1)
                })()
            }
            3 => {
                (|| {
                    // (<Val> Comma)* = (<Val> Comma)+ => ActionFn(40);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action40::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 1)
                })()
            }
            4 => {
                (|| {
                    // (<Val> Comma)+ = Val, Comma => ActionFn(44);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action44::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (2, __symbol, 2)
                })()
            }
            5 => {
                (|| {
                    // (<Val> Comma)+ = (<Val> Comma)+, Val, Comma => ActionFn(45);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action45::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (3, __symbol, 2)
                })()
            }
            6 => {
                (|| {
                    // @L =  => ActionFn(31);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action31::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (0, __symbol, 3)
                })()
            }
            7 => {
                (|| {
                    // @R =  => ActionFn(30);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action30::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (0, __symbol, 4)
                })()
            }
            8 => {
                (|| {
                    // Array = L_BRACK, CommaList<Val>, R_BRACK => ActionFn(16);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant3(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action16::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (3, __symbol, 5)
                })()
            }
            9 => {
                (|| {
                    // BareKey = bare_key => ActionFn(61);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action61::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 6)
                })()
            }
            10 => {
                (|| {
                    // BareKey = bare_key_or_number => ActionFn(62);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action62::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 6)
                })()
            }
            11 => {
                (|| {
                    // BasicString = basic_string => ActionFn(63);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action63::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 7)
                })()
            }
            12 => {
                (|| {
                    // Bool = bool => ActionFn(64);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action64::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            13 => {
                (|| {
                    // Comma = comma => ActionFn(65);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action65::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 9)
                })()
            }
            14 => {
                (|| {
                    // CommaList<Val> = Val => ActionFn(76);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action76::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (1, __symbol, 10)
                })()
            }
            15 => {
                (|| {
                    // CommaList<Val> =  => ActionFn(77);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action77::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (0, __symbol, 10)
                })()
            }
            16 => {
                (|| {
                    // CommaList<Val> = (<Val> Comma)+, Val => ActionFn(78);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action78::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (2, __symbol, 10)
                })()
            }
            17 => {
                (|| {
                    // CommaList<Val> = (<Val> Comma)+ => ActionFn(79);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action79::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (1, __symbol, 10)
                })()
            }
            18 => {
                (|| {
                    // Eq = eq => ActionFn(66);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action66::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 11)
                })()
            }
            19 => {
                (|| {
                    // FileForm = KeyVal => ActionFn(2);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action2::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 12)
                })()
            }
            20 => {
                (|| {
                    // FileForm* =  => ActionFn(33);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action33::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (0, __symbol, 13)
                })()
            }
            21 => {
                (|| {
                    // FileForm* = FileForm+ => ActionFn(34);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action34::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 13)
                })()
            }
            22 => {
                (|| {
                    // FileForm+ = FileForm => ActionFn(35);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action35::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 14)
                })()
            }
            23 => {
                (|| {
                    // FileForm+ = FileForm+, FileForm => ActionFn(36);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action36::<E>(events, input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (2, __symbol, 14)
                })()
            }
            24 => {
                (|| {
                    // Key = Key_ => ActionFn(5);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action5::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 15)
                })()
            }
            25 => {
                (|| {
                    // KeyVal = KeyVal_ => ActionFn(3);
                    let __sym0 = __pop_Variant4(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action3::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 16)
                })()
            }
            26 => {
                (|| {
                    // KeyVal_ = Key, Eq, Val => ActionFn(4);
                    let __sym2 = __pop_Variant1(__symbols);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action4::<E>(events, input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                    (3, __symbol, 17)
                })()
            }
            27 => {
                (|| {
                    // Key_ = BareKey => ActionFn(6);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action6::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 18)
                })()
            }
            28 => {
                (|| {
                    // L_BRACK = l_brack => ActionFn(67);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action67::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 19)
                })()
            }
            29 => {
                (|| {
                    // Literal = Number => ActionFn(10);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action10::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 20)
                })()
            }
            30 => {
                (|| {
                    // Literal = Bool => ActionFn(11);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action11::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 20)
                })()
            }
            31 => {
                (|| {
                    // Literal = BasicString => ActionFn(12);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action12::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 20)
                })()
            }
            32 => {
                (|| {
                    // Literal = LiteralString => ActionFn(13);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action13::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 20)
                })()
            }
            33 => {
                (|| {
                    // Literal = MultilineBasicString => ActionFn(14);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action14::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 20)
                })()
            }
            34 => {
                (|| {
                    // Literal = MultilineLiteralString => ActionFn(15);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action15::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 20)
                })()
            }
            35 => {
                (|| {
                    // LiteralString = literal_string => ActionFn(68);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action68::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 21)
                })()
            }
            36 => {
                (|| {
                    // MultilineBasicString = multiline_basic_string => ActionFn(69);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action69::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 22)
                })()
            }
            37 => {
                (|| {
                    // MultilineLiteralString = multiline_literal_string => ActionFn(70);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action70::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 23)
                })()
            }
            38 => {
                (|| {
                    // Number = number => ActionFn(71);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action71::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 24)
                })()
            }
            39 => {
                (|| {
                    // Number = bare_key_or_number => ActionFn(72);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action72::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 24)
                })()
            }
            40 => {
                (|| {
                    // R_BRACK = r_brack => ActionFn(73);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action73::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 25)
                })()
            }
            41 => {
                (|| {
                    // TomlFile =  => ActionFn(74);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action74::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (0, __symbol, 26)
                })()
            }
            42 => {
                (|| {
                    // TomlFile = FileForm+ => ActionFn(75);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action75::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 26)
                })()
            }
            43 => {
                (|| {
                    // Val = Val_ => ActionFn(7);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action7::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 27)
                })()
            }
            44 => {
                (|| {
                    // Val? = Val => ActionFn(37);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action37::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 28)
                })()
            }
            45 => {
                (|| {
                    // Val? =  => ActionFn(38);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action38::<E>(events, input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (0, __symbol, 28)
                })()
            }
            46 => {
                (|| {
                    // Val_ = Literal => ActionFn(8);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action8::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 29)
                })()
            }
            47 => {
                (|| {
                    // Val_ = Array => ActionFn(9);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action9::<E>(events, input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 29)
                })()
            }
            48 => {
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
        let __next_state = __GOTO[__state * 31 + __nonterminal] - 1;
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
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ((), (), ()), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
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
    ) -> (usize, ::std::option::Option<()>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
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
                "^((?u:[-0-9A-Z_-_a-z])+)",
                "^((?u:[-\\+-\\+])?((?u:0)|(?u:[1-9])((?u:_)?(?u:[0-9]))*)((?u:\\.)(?u:[0-9])((?u:_)?(?u:[0-9]))*)?((?u:[E-Ee-e])(?u:[-\\+-\\+])?(?u:[1-9])((?u:_)?(?u:[0-9]))*)?)",
                "^((?u:\")((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\"))",
                "^((?u:\"\"\")((?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\"\")(?u:[\u{0}-!\\#-\u{10ffff}]))*(?u:\"\"\"))",
                "^((?u:\')((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\'))",
                "^((?u:\'\'\')((?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\')(?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\'\')(?u:[\u{0}-\\&\\(-\u{10ffff}]))*(?u:\'\'\'))",
                "^((?u:[0-9])+)",
                "^((?u:false)|(?u:true))",
                "^((?u:,))",
                "^((?u:\\.))",
                "^((?u:=))",
                "^((?u:\\[))",
                "^((?u:\\]))",
                "^((?u:\\{))",
                "^((?u:\\}))",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:[-0-9A-Z_-_a-z])+)").unwrap(),
                __regex::Regex::new("^((?u:[-\\+-\\+])?((?u:0)|(?u:[1-9])((?u:_)?(?u:[0-9]))*)((?u:\\.)(?u:[0-9])((?u:_)?(?u:[0-9]))*)?((?u:[E-Ee-e])(?u:[-\\+-\\+])?(?u:[1-9])((?u:_)?(?u:[0-9]))*)?)").unwrap(),
                __regex::Regex::new("^((?u:\")((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-!\\#-\\[\\]-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\"))").unwrap(),
                __regex::Regex::new("^((?u:\"\"\")((?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\"\")(?u:[\u{0}-!\\#-\u{10ffff}]))*(?u:\"\"\"))").unwrap(),
                __regex::Regex::new("^((?u:\')((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\'))").unwrap(),
                __regex::Regex::new("^((?u:\'\'\')((?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\')(?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\'\')(?u:[\u{0}-\\&\\(-\u{10ffff}]))*(?u:\'\'\'))").unwrap(),
                __regex::Regex::new("^((?u:[0-9])+)").unwrap(),
                __regex::Regex::new("^((?u:false)|(?u:true))").unwrap(),
                __regex::Regex::new("^((?u:,))").unwrap(),
                __regex::Regex::new("^((?u:\\.))").unwrap(),
                __regex::Regex::new("^((?u:=))").unwrap(),
                __regex::Regex::new("^((?u:\\[))").unwrap(),
                __regex::Regex::new("^((?u:\\]))").unwrap(),
                __regex::Regex::new("^((?u:\\{))").unwrap(),
                __regex::Regex::new("^((?u:\\}))").unwrap(),
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
                    for __i in 0 .. 15 {
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
    (_, xs, _): (usize, ::std::vec::Vec<()>, usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::FILE, xs.len())
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
    (_, __0, _): (usize, ((), (), ()), usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::KEY_VAL, 3)
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
) -> ((), (), ()) where
  E:  Events,
{
    (__0, __1, __2)
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
    events.reduce(symbols::KEY, 1)
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
    events.reduce(symbols::VAL, 1)
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
    (_, _, _): (usize, (), usize),
    (_, xs, _): (usize, usize, usize),
    (_, _, _): (usize, (), usize),
) -> () where
  E:  Events,
{
    events.reduce(symbols::ARRAY, xs + 2)
}

#[allow(unused_variables)]
fn __action17<
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
fn __action18<
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
fn __action19<
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
fn __action20<
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
fn __action21<
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
fn __action22<
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
fn __action23<
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
    events.shift(symbols::BARE_KEY, l, r)
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
    events.shift(symbols::BARE_KEY, l, r)
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
    events.shift(symbols::BASIC_STRING, l, r)
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
    events.shift(symbols::MULTILINE_BASIC_STRING, l, r)
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
    events.shift(symbols::LITERAL_STRING, l, r)
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
    events.shift(symbols::MULTILINE_LITERAL_STRING, l, r)
}

#[allow(unused_variables)]
fn __action30<
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
fn __action31<
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
fn __action32<
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
fn __action33<
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
fn __action34<
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
fn __action35<
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
fn __action36<
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
fn __action37<
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
fn __action38<
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
fn __action39<
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
fn __action40<
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
fn __action41<
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
fn __action42<
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
fn __action43<
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
fn __action44<
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
    let __temp0 = __action41(
        events,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action45<
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
    let __temp0 = __action41(
        events,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action46<
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
    let __temp0 = __action39(
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
    )
}

#[allow(unused_variables)]
fn __action47<
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
    let __temp0 = __action40(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        events,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action48<
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
    let __temp0 = __action31(
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
fn __action49<
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
    let __temp0 = __action31(
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
fn __action50<
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
    let __temp0 = __action31(
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
fn __action51<
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
    let __temp0 = __action31(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action52<
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
    let __temp0 = __action31(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action53<
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
    let __temp0 = __action31(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action54<
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
    let __temp0 = __action31(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action55<
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
    let __temp0 = __action31(
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
fn __action56<
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
    let __temp0 = __action31(
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
fn __action57<
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
    let __temp0 = __action31(
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
fn __action58<
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
    let __temp0 = __action31(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action59<
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
    let __temp0 = __action31(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action60<
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
    let __temp0 = __action31(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action61<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action66<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action67<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action68<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action69<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action70<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action71<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action72<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action73<
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
    let __temp0 = __action30(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action74<
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
    let __temp0 = __action33(
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
) -> () where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
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
    __0: (usize, (), usize),
) -> usize where
  E:  Events,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        events,
        input,
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  Events,
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action38(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        events,
        input,
        __temp0,
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
    __1: (usize, (), usize),
) -> usize where
  E:  Events,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action37(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        events,
        input,
        __0,
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
) -> usize where
  E:  Events,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action38(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
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
