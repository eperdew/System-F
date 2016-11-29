use std::str::FromStr;
use systemf::{Type, Expr};
use std::rc::Rc;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Expression {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use systemf::{Type, Expr};
    use std::rc::Rc;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_2e_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22Lambda_22(&'input str),
        Term_22Let_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22forall_22(&'input str),
        Term_22in_22(&'input str),
        Term_22lambda_22(&'input str),
        Term_22let_22(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        NtExpression(Expr),
        NtFactor(Expr),
        NtIdentifier(Rc<String>),
        NtTFactor(Rc<Type>),
        NtTTerm(Rc<Type>),
        NtTerm(Expr),
        NtTyp(Rc<Type>),
        Nt____Expression(Expr),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        6, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        6, -5, 0, 0, 0, 0, 0, 0, 13, 0, 0, -5, 0, 0, 11, 0,
        // State 3
        -14, -14, 0, 0, 0, 0, 0, 0, -14, 0, 0, -14, 0, 0, -14, 0,
        // State 4
        -8, -8, 0, 0, 0, 0, 0, 0, -8, 0, 0, -8, 0, 0, -8, 0,
        // State 5
        6, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 10
        -9, -9, -9, -9, -9, -9, 0, 0, -9, -9, 0, -9, 0, 0, -9, 0,
        // State 11
        -6, -6, 0, 0, 0, 0, 0, 0, -6, 0, 0, -6, 0, 0, -6, 0,
        // State 12
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 11, 0,
        // State 13
        0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, -12, -12, -12, 0, -12, 0, 0, 0, -12, 0, -12, 0, 0, 0, 0,
        // State 19
        0, -17, 0, -17, 0, -17, 0, 0, 0, -17, 0, -17, 0, 0, 0, 0,
        // State 20
        0, -11, 30, -11, 0, -11, 0, 0, 0, -11, 0, -11, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0,
        // State 22
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 11, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 24
        -15, -15, 0, 0, 0, 0, 0, 0, -15, 0, 0, -15, 0, 0, -15, 0,
        // State 25
        6, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 26
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 11, 0,
        // State 27
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 11, 0,
        // State 28
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 11, 0,
        // State 29
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0,
        // State 30
        -7, -7, 0, 0, 0, 0, 0, 0, -7, 0, 0, -7, 0, 0, -7, 0,
        // State 31
        0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, -10, 0, -10, 0, -10, 0, 0, 0, -10, 0, -10, 0, 0, 0, 0,
        // State 38
        0, -13, -13, -13, 0, -13, 0, 0, 0, -13, 0, -13, 0, 0, 0, 0,
        // State 39
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 11, 0,
        // State 40
        6, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 41
        6, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 42
        6, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 43
        0, -16, 0, -16, 0, -16, 0, 0, 0, -16, 0, -16, 0, 0, 0, 0,
        // State 44
        0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 45
        0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0,
        // State 47
        6, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 9, 10, 11, 0,
        // State 48
        0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -18,
        -5,
        -14,
        -8,
        0,
        0,
        0,
        0,
        0,
        -9,
        -6,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -15,
        0,
        0,
        0,
        0,
        0,
        -7,
        0,
        0,
        -2,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -4,
        -1,
        0,
        0,
        -3,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 3, 4, 0, 0, 5, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 4, 0, 0, 12, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        14, 3, 4, 0, 0, 5, 0, 0,
        // State 6
        0, 0, 15, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 16, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 17, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 18, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 19, 20, 21, 0, 22, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 19, 20, 21, 0, 32, 0,
        // State 23
        0, 0, 33, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        34, 3, 4, 0, 0, 5, 0, 0,
        // State 26
        0, 0, 19, 20, 21, 0, 35, 0,
        // State 27
        0, 0, 19, 20, 21, 0, 36, 0,
        // State 28
        0, 0, 19, 20, 21, 0, 37, 0,
        // State 29
        0, 0, 19, 38, 21, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 19, 20, 21, 0, 44, 0,
        // State 40
        45, 3, 4, 0, 0, 5, 0, 0,
        // State 41
        46, 3, 4, 0, 0, 5, 0, 0,
        // State 42
        47, 3, 4, 0, 0, 5, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        49, 3, 4, 0, 0, 5, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    pub fn parse_Expression<
        'input,
    >(
        input: &'input str,
    ) -> Result<Expr, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 16 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_28_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_29_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_2d_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_2e_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_3d_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22Lambda_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22Let_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22forall_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22in_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22lambda_22(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22let_22(__tok0),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead.clone()),
                        expected: vec![],
                    };
                    return Err(__error);
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Expression = "lambda", Identifier, ":", Typ, ".", Expression => ActionFn(1);
                let __sym5 = __pop_NtExpression(__symbols);
                let __sym4 = __pop_Term_22_2e_22(__symbols);
                let __sym3 = __pop_NtTyp(__symbols);
                let __sym2 = __pop_Term_22_3a_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22lambda_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                0
            }
            2 => {
                // Expression = "Lambda", Identifier, ".", Expression => ActionFn(2);
                let __sym3 = __pop_NtExpression(__symbols);
                let __sym2 = __pop_Term_22_2e_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22Lambda_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                0
            }
            3 => {
                // Expression = "let", Identifier, ":", Typ, "=", Expression, "in", Expression => ActionFn(3);
                let __sym7 = __pop_NtExpression(__symbols);
                let __sym6 = __pop_Term_22in_22(__symbols);
                let __sym5 = __pop_NtExpression(__symbols);
                let __sym4 = __pop_Term_22_3d_22(__symbols);
                let __sym3 = __pop_NtTyp(__symbols);
                let __sym2 = __pop_Term_22_3a_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22let_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                0
            }
            4 => {
                // Expression = "Let", Identifier, "=", Typ, "in", Expression => ActionFn(4);
                let __sym5 = __pop_NtExpression(__symbols);
                let __sym4 = __pop_Term_22in_22(__symbols);
                let __sym3 = __pop_NtTyp(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22Let_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                0
            }
            5 => {
                // Expression = Factor => ActionFn(5);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                0
            }
            6 => {
                // Factor = Factor, Term => ActionFn(6);
                let __sym1 = __pop_NtTerm(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            7 => {
                // Factor = Factor, "[", Typ, "]" => ActionFn(7);
                let __sym3 = __pop_Term_22_5d_22(__symbols);
                let __sym2 = __pop_NtTyp(__symbols);
                let __sym1 = __pop_Term_22_5b_22(__symbols);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            8 => {
                // Factor = Term => ActionFn(8);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                1
            }
            9 => {
                // Identifier = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(17);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                2
            }
            10 => {
                // TFactor = TTerm, "->", TFactor => ActionFn(13);
                let __sym2 = __pop_NtTFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym0 = __pop_NtTTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTFactor(__nt), __end));
                3
            }
            11 => {
                // TFactor = TTerm => ActionFn(14);
                let __sym0 = __pop_NtTTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTFactor(__nt), __end));
                3
            }
            12 => {
                // TTerm = Identifier => ActionFn(15);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTTerm(__nt), __end));
                4
            }
            13 => {
                // TTerm = "(", Typ, ")" => ActionFn(16);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTyp(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTTerm(__nt), __end));
                4
            }
            14 => {
                // Term = Identifier => ActionFn(9);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                5
            }
            15 => {
                // Term = "(", Expression, ")" => ActionFn(10);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                5
            }
            16 => {
                // Typ = "forall", Identifier, ".", Typ => ActionFn(11);
                let __sym3 = __pop_NtTyp(__symbols);
                let __sym2 = __pop_Term_22_2e_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22forall_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTyp(__nt), __end));
                6
            }
            17 => {
                // Typ = TFactor => ActionFn(12);
                let __sym0 = __pop_NtTFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTyp(__nt), __end));
                6
            }
            18 => {
                // __Expression = Expression => ActionFn(0);
                let __sym0 = __pop_NtExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 8 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Lambda_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Lambda_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Let_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Let_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22forall_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22forall_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22in_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22in_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22lambda_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22lambda_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22let_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22let_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdentifier<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Rc<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdentifier(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Rc<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Rc<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTyp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Rc<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTyp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expression::parse_Expression;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        40 => /* '(' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        41 => /* ')' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        45 => /* '-' */ {
                            __current_state = 3;
                            continue;
                        }
                        46 => /* '.' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        61 => /* '=' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        65 ... 75 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        77 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        93 => /* ']' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        97 ... 101 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        102 => /* 'f' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        103 ... 104 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        106 ... 107 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        62 => /* '>' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        98 ... 100 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        98 ... 100 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 108 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        110 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 108 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        110 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        98 => /* 'b' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        99 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        98 => /* 'b' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        99 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((14, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((14, __index + __ch.len_utf8()));
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, Rc<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Rc<Type>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Lam(id, t, Rc::new(e))
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, Rc<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Expr, usize),
) -> Expr
{
    Expr::TLam(id, Rc::new(e))
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, Rc<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Rc<Type>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e1, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e2, _): (usize, Expr, usize),
) -> Expr
{
    Expr::Let(id, t, Rc::new(e1), Rc::new(e2))
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, Rc<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Rc<Type>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Expr, usize),
) -> Expr
{
    Expr::TLet(id, t, Rc::new(e))
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expr, usize),
    (_, r, _): (usize, Expr, usize),
) -> Expr
{
    Expr::App(Rc::new(l), Rc::new(r))
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Rc<Type>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    Expr::TApp(Rc::new(l), r)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> Expr
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, Rc<String>, usize),
) -> Expr
{
    Expr::Var(id)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Expr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    e
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, Rc<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Rc<Type>, usize),
) -> Rc<Type>
{
    Rc::new(Type::Forall(id, t))
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Rc<Type>, usize),
) -> Rc<Type>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, t1, _): (usize, Rc<Type>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t2, _): (usize, Rc<Type>, usize),
) -> Rc<Type>
{
    Rc::new(Type::Fun(t1, t2))
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Rc<Type>, usize),
) -> Rc<Type>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, Rc<String>, usize),
) -> Rc<Type>
{
    Rc::new(Type::Var(id))
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Rc<Type>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Rc<Type>
{
    t
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Rc<String>
{
    Rc::new(String::from(__0))
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
