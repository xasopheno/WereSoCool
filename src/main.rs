use std::collections::HashMap;
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub socool); // synthesized by LALRPOP
pub mod ast;
use crate::ast::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut table = make_table();

    let filename = "working.socool";

    let mut f = File::open(filename).expect("file not found");

    let mut composition = String::new();
    f.read_to_string(&mut composition)
        .expect("something went wrong reading the file");

    println!(
        "\n Settings: {:?}",
        socool::SoCoolParser::new()
            .parse(&mut table, &composition)
            .unwrap()
    );

    for (key, val) in table.iter() {
        println!("\n name: {:?} op: {:?}", key, val);
    }

    println!("\n Main: {:?}", table.get("main").unwrap());
}

fn make_table() -> HashMap<String, Op> {
    let table: HashMap<String, Op> = HashMap::new();
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Init, Op};
    lalrpop_mod!(pub socool);

    fn mock_init() -> (String) {
        "{ f: 200, l: 1.0, g: 1.0, p: 0.0 }
        let main = {"
            .to_string()
    }

    fn test_parsed_operation(mut parse_str: String, expected: Op) {
        let mut table = make_table();

        parse_str.push_str("}");

        let _result = socool::SoCoolParser::new().parse(&mut table, &parse_str);

        let main = table.get(&"main".to_string()).unwrap();
        assert_eq!(*main, expected);
    }

    #[test]
    fn tm_test() {
        let mut parse_str = mock_init();
        parse_str.push_str("Tm 3/2");
        test_parsed_operation(parse_str, Op::TransposeM { m: 1.5 });
    }

    #[test]
    fn ta_test() {
        let mut parse_str = mock_init();
        parse_str.push_str("Ta 2.0");
        test_parsed_operation(parse_str, Op::TransposeA { a: 2.0 });
    }

    #[test]
    fn pan_a_test() {
        let mut parse_str = mock_init();
        parse_str.push_str("PanA 2.0");
        test_parsed_operation(parse_str, Op::PanA { a: 2.0 });
    }

    #[test]
    fn pan_m_test() {
        let mut parse_str = mock_init();
        parse_str.push_str("PanM 3.0/2.0");
        test_parsed_operation(parse_str, Op::PanM { m: 1.5 });
    }

    #[test]
    fn gain_test() {
        let mut parse_str = mock_init();
        parse_str.push_str("Gain 0.25");
        test_parsed_operation(parse_str, Op::Gain { m: 0.25 });
    }

    #[test]
    fn length_test() {
        let mut parse_str = mock_init();
        parse_str.push_str("Length 0.5");
        test_parsed_operation(parse_str, Op::Length { m: 0.5 });
    }

    #[test]
    fn reverse_test() {
        let mut parse_str = mock_init();
        parse_str.push_str("Reverse");
        test_parsed_operation(parse_str, Op::Reverse);
    }

    #[test]
    fn asis_test() {
        let mut parse_str = mock_init();
        parse_str.push_str("AsIs");
        test_parsed_operation(parse_str, Op::AsIs);
    }

    #[test]
    fn sequence_test() {
        let mut parse_str = mock_init();
        parse_str.push_str(
            "
            Sequence [
                AsIs,
                Tm 3/2,
            ]
        ",
        );
        test_parsed_operation(
            parse_str,
            Op::Sequence {
                operations: vec![Op::AsIs, Op::TransposeM { m: 3.0 / 2.0 }],
            },
        );
    }

    #[test]
    fn overlay_test() {
        let mut parse_str = mock_init();
        parse_str.push_str(
            "
            Overlay [
                AsIs,
                Tm 3/2,
            ]
        ",
        );
        test_parsed_operation(
            parse_str,
            Op::Overlay {
                operations: vec![Op::AsIs, Op::TransposeM { m: 3.0 / 2.0 }],
            },
        );
    }

    #[test]
    fn o_test() {
        let mut parse_str = mock_init();
        parse_str.push_str(
            "
            o[(3/2, 3.0, 1.0, 0.3),
              (1, 0.0, 0.5, 0.0)]
        ",
        );
        test_parsed_operation(
            parse_str,
            Op::Overlay {
                operations: vec![
                    Op::Compose {
                        operations: vec![
                            Op::TransposeM { m: 1.5 },
                            Op::TransposeA { a: 3.0 },
                            Op::Gain { m: 1.0 },
                            Op::PanA { a: 0.3 },
                        ],
                    },
                    Op::Compose {
                        operations: vec![
                            Op::TransposeM { m: 1.0 },
                            Op::TransposeA { a: 0.0 },
                            Op::Gain { m: 0.5 },
                            Op::PanA { a: 0.0 },
                        ],
                    },
                ],
            },
        );
    }

    #[test]
    fn let_insert() {
        let mut table = make_table();
        socool::SoCoolParser::new()
            .parse(
                &mut table,
                "
                { f: 200, l: 1.0, g: 1.0, p: 0.0 }

                let thing = {
                    Tm 3/2
                    | Gain 0.3
                }
                ",
            )
            .unwrap();
        let thing = table.get(&"thing".to_string()).unwrap();
        assert_eq!(
            *thing,
            Op::Compose {
                operations: vec![Op::TransposeM { m: 1.5 }, Op::Gain { m: 0.3 }]
            }
        )
    }

    #[test]
    fn let_get() {
        let mut table = make_table();
        socool::SoCoolParser::new()
            .parse(
                &mut table,
                "
                { f: 200, l: 1.0, g: 1.0, p: 0.0 }

                let thing = {
                    Tm 3/2
                    | Gain 0.3
                }

                let main = { thing }
                ",
            )
            .unwrap();
    }

    #[test]
    fn fit_length_test() {
        let mut table = make_table();

        let result = socool::SoCoolParser::new().parse(
            &mut table,
            "
                { f: 200, l: 1.0, g: 1.0, p: 0.0 }

                let thing = {
                    Sequence [
                     AsIs,
                     Tm 3/2
                     | Length 2.0
                    ]
                }

                let thing2 = {
                    Sequence [
                        Tm 5/4,
                        Tm 3/2
                    ]
                    | Repeat 2
                    > fitLength thing
                }

                let main = {
                    thing2
                }
            ",
        );
        let thing = table.get(&"main".to_string()).unwrap();
        assert_eq!(
            *thing,
            Op::Compose { operations: vec![
                Op::Compose { operations: vec![
                    Op::Sequence { operations: vec![
                        Op::TransposeM { m: 1.25 }, Op::TransposeM { m: 1.5 }] },
                    Op::Sequence { operations: vec![Op::AsIs, Op::AsIs] }] },
                Op::WithLengthRatioOf {
                    length_of: Box::new(Op::Sequence { operations: vec![
                        Op::AsIs,
                        Op::Compose { operations: vec![
                            Op::TransposeM { m: 1.5 }, Op::Length { m: 2.0 }] }] }),
                    main: Box::new(Op::Compose { operations: vec![
                        Op::Sequence { operations: vec![
                            Op::TransposeM { m: 1.25 }, Op::TransposeM { m: 1.5 }] },
                        Op::Sequence { operations: vec![
                            Op::AsIs, Op::AsIs] }] })
                }]
            }
        )
    }
}


