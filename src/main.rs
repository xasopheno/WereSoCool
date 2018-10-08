#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub socool); // synthesized by LALRPOP
pub mod ast;

fn main() {
    println!("{:?}", socool::OperationParser::new().parse("Tm 3/2"));
    println!("{:?}", socool::OperationParser::new().parse("Ta 3.0"));
    println!("{:?}", socool::OperationParser::new().parse("PanM 3"));
    println!("{:?}", socool::OperationParser::new().parse("PanA 3"));
    println!("{:?}", socool::OperationParser::new().parse("Length 3"));
    println!("{:?}", socool::OperationParser::new().parse("Gain 3"));
    println!("{:?}", socool::OperationParser::new().parse("Reverse"));
    println!("{:?}", socool::OperationParser::new().parse("AsIs"));
}

#[cfg(test)]
mod tests {
    use crate::ast::Op;
    lalrpop_mod!(pub socool); // synthesized by LALRPOP
    #[test]
    fn ops() {
        assert_eq!(socool::OperationParser::new().parse("Tm 3/2").unwrap(), Op::TransposeM {m: 1.5});
        assert_eq!(socool::OperationParser::new().parse("Ta 3.0").unwrap(), Op::TransposeA {a: 3.0});
        assert_eq!(socool::OperationParser::new().parse("PanM 3.0").unwrap(), Op::PanM {m: 3.0});
        assert_eq!(socool::OperationParser::new().parse("PanA 3.0").unwrap(), Op::PanA {a: 3.0});
        assert_eq!(socool::OperationParser::new().parse("Gain 3.0").unwrap(), Op::Gain {m: 3.0});
        assert_eq!(socool::OperationParser::new().parse("Length 3.0").unwrap(), Op::Length {m: 3.0});
        assert_eq!(socool::OperationParser::new().parse("Reverse").unwrap(), Op::Reverse);
        assert_eq!(socool::OperationParser::new().parse("AsIs").unwrap(), Op::AsIs);
    }
}
