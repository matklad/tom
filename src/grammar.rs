// auto-generated: "lalrpop 0.15.0"
#![cfg_attr(rustfmt, rustfmt_skip)]
use events::Events;
use symbols;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

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
        Variant1(usize),
        Variant2(()),
        Variant3(::std::vec::Vec<()>),
        Variant4(((), (), ())),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 19, 0, 20, 0, 0, 0, 0, 0, 0, 21,
        // State 12
        0, 0, 0, 0, 0, -5, 0, -5, 0, 0, 0, 0, 0, 0, -5,
        // State 13
        0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -19,
        // State 1
        0,
        // State 2
        -9,
        // State 3
        -20,
        // State 4
        0,
        // State 5
        -6,
        // State 6
        -12,
        // State 7
        0,
        // State 8
        -23,
        // State 9
        0,
        // State 10
        -10,
        // State 11
        0,
        // State 12
        0,
        // State 13
        -16,
        // State 14
        -22,
        // State 15
        -15,
        // State 16
        -13,
        // State 17
        -21,
        // State 18
        -18,
        // State 19
        -4,
        // State 20
        -17,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 2, 0, 0, 3, 0, 4, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 2, 0, 0, 11, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 15, 16, 0, 17, 18, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"L_BRACK"###,
            r###"L_CURLY"###,
            r###"R_BRACK"###,
            r###"R_CURLY"###,
            r###"bare_key"###,
            r###"bare_key_or_number"###,
            r###"basic_string"###,
            r###"bool"###,
            r###"comma"###,
            r###"dot"###,
            r###"eq"###,
            r###"literal_string"###,
            r###"mutiline_basic_string"###,
            r###"mutiline_literal_string"###,
            r###"number"###,
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
                    Token(11, _) if true => 0,
                    Token(13, _) if true => 1,
                    Token(12, _) if true => 2,
                    Token(14, _) if true => 3,
                    Token(0, _) if true => 4,
                    Token(6, _) if true => 5,
                    Token(2, _) if true => 6,
                    Token(7, _) if true => 7,
                    Token(8, _) if true => 8,
                    Token(9, _) if true => 9,
                    Token(10, _) if true => 10,
                    Token(4, _) if true => 11,
                    Token(3, _) if true => 12,
                    Token(5, _) if true => 13,
                    Token(1, _) if true => 14,
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
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(13, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
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
                // @L =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<E>(events, input, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                (0, __symbol, 0)
            }
            2 => {
                // @R =  => ActionFn(16);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action16::<E>(events, input, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                (0, __symbol, 1)
            }
            3 => {
                // BareKey = bare_key => ActionFn(27);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 2)
            }
            4 => {
                // Bool = bool => ActionFn(28);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 3)
            }
            5 => {
                // Eq = eq => ActionFn(29);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 4)
            }
            6 => {
                // FileForm = KeyVal => ActionFn(2);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 5)
            }
            7 => {
                // FileForm* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<E>(events, input, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (0, __symbol, 6)
            }
            8 => {
                // FileForm* = FileForm+ => ActionFn(19);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (1, __symbol, 6)
            }
            9 => {
                // FileForm+ = FileForm => ActionFn(20);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (1, __symbol, 7)
            }
            10 => {
                // FileForm+ = FileForm+, FileForm => ActionFn(21);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<E>(events, input, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 7)
            }
            11 => {
                // Key = Key_ => ActionFn(5);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 8)
            }
            12 => {
                // KeyVal = KeyVal_ => ActionFn(3);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 9)
            }
            13 => {
                // KeyVal_ = Key, Eq, Val => ActionFn(4);
                let __sym2 = __pop_Variant2(__symbols);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<E>(events, input, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (3, __symbol, 10)
            }
            14 => {
                // Key_ = BareKey => ActionFn(6);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 11)
            }
            15 => {
                // Literal = Number => ActionFn(9);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 12)
            }
            16 => {
                // Literal = Bool => ActionFn(10);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 12)
            }
            17 => {
                // Number = number => ActionFn(30);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 13)
            }
            18 => {
                // Number = bare_key_or_number => ActionFn(31);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 13)
            }
            19 => {
                // TomlFile =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<E>(events, input, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (0, __symbol, 14)
            }
            20 => {
                // TomlFile = FileForm+ => ActionFn(33);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 14)
            }
            21 => {
                // Val = Val_ => ActionFn(7);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 15)
            }
            22 => {
                // Val_ = Literal => ActionFn(8);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<E>(events, input, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 16)
            }
            23 => {
                // __TomlFile = TomlFile => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
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
        let __next_state = __GOTO[__state * 18 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
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
    ) -> (usize, ((), (), ()), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
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
    ) -> (usize, ::std::vec::Vec<()>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
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
                "^(?u:[-0-9A-Z_-_a-z])+",
                "^(?u:[-\\+-\\+])?((?u:0)|(?u:[1-9])((?u:_)?(?u:[0-9]))*)((?u:\\.)(?u:[0-9])((?u:_)?(?u:[0-9]))*)?((?u:[E-Ee-e])(?u:[-\\+-\\+])?(?u:[1-9])((?u:_)?(?u:[0-9]))*)?",
                "^(?u:\")((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-!\\#-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\")",
                "^(?u:\"\"\")((?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\"\")(?u:[\u{0}-!\\#-\u{10ffff}]))*(?u:\"\"\")",
                "^(?u:\')((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\')",
                "^(?u:\'\'\')((?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\')(?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\'\')(?u:[\u{0}-\\&\\(-\u{10ffff}]))*(?u:\'\'\')",
                "^(?u:[0-9])+",
                "^(?u:false)|(?u:true)",
                "^(?u:,)",
                "^(?u:\\.)",
                "^(?u:=)",
                "^(?u:\\[)",
                "^(?u:\\])",
                "^(?u:\\{)",
                "^(?u:\\})",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[-0-9A-Z_-_a-z])+").unwrap(),
                __regex::Regex::new("^(?u:[-\\+-\\+])?((?u:0)|(?u:[1-9])((?u:_)?(?u:[0-9]))*)((?u:\\.)(?u:[0-9])((?u:_)?(?u:[0-9]))*)?((?u:[E-Ee-e])(?u:[-\\+-\\+])?(?u:[1-9])((?u:_)?(?u:[0-9]))*)?").unwrap(),
                __regex::Regex::new("^(?u:\")((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-!\\#-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\")").unwrap(),
                __regex::Regex::new("^(?u:\"\"\")((?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])|(?u:\"\")(?u:[\u{0}-!\\#-\u{10ffff}]))*(?u:\"\"\")").unwrap(),
                __regex::Regex::new("^(?u:\')((?u:[\u{0}-\t\u{b}-\u{c}\u{e}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.))*(?u:\')").unwrap(),
                __regex::Regex::new("^(?u:\'\'\')((?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\\\\)(?u:.)|(?u:\')(?u:[\u{0}-\\&\\(-\u{10ffff}])|(?u:\'\')(?u:[\u{0}-\\&\\(-\u{10ffff}]))*(?u:\'\'\')").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:false)|(?u:true)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\.)").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:\\[)").unwrap(),
                __regex::Regex::new("^(?u:\\])").unwrap(),
                __regex::Regex::new("^(?u:\\{)").unwrap(),
                __regex::Regex::new("^(?u:\\})").unwrap(),
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
    (_, l, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, usize, usize),
) -> () where
  E:  Events,
{
    events.shift(symbols::EQ, l, r)
}

#[allow(unused_variables)]
fn __action12<
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
fn __action13<
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
fn __action14<
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
fn __action15<
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
fn __action16<
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
fn __action17<
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
fn __action18<
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
fn __action19<
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
fn __action20<
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
fn __action21<
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
fn __action22<
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
    let __temp0 = __action17(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action23<
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
    let __temp0 = __action17(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action24<
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
    let __temp0 = __action17(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action25<
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
    let __temp0 = __action17(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action26<
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
    let __temp0 = __action17(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        events,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action27<
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
    let __temp0 = __action16(
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
fn __action28<
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
    let __temp0 = __action16(
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
fn __action29<
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
    let __temp0 = __action16(
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
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action30<
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
    let __temp0 = __action16(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action31<
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
    let __temp0 = __action16(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action32<
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
    let __temp0 = __action18(
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
fn __action33<
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
    let __temp0 = __action19(
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
