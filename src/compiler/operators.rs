use crate::opcode::OpCode;
use crate::parser::{BinaryOperator, UnaryOperator};
use crate::types::{ConstantIndex8, RegisterIndex};
use crate::value::Value;

// Binary operators which map directly to a single opcode
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SimpleBinOp {
    Add,
    Sub,
    Mul,
    Mod,
    Pow,
    Div,
    IDiv,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

// Binary operators which map to Eq / LessThan / LessEqual operations combined with Jump and
// LoadBool
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ComparisonBinOp {
    NotEqual,
    Equal,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
}

// 'and' and 'or', which short circuit their right hand side
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShortCircuitBinOp {
    And,
    Or,
}

// Categorized BinaryOperator
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum BinOpCategory {
    Simple(SimpleBinOp),
    Comparison(ComparisonBinOp),
    ShortCircuit(ShortCircuitBinOp),
    Concat,
}

pub fn categorize_binop(binop: BinaryOperator) -> BinOpCategory {
    match binop {
        BinaryOperator::Add => BinOpCategory::Simple(SimpleBinOp::Add),
        BinaryOperator::Sub => BinOpCategory::Simple(SimpleBinOp::Sub),
        BinaryOperator::Mul => BinOpCategory::Simple(SimpleBinOp::Mul),
        BinaryOperator::Mod => BinOpCategory::Simple(SimpleBinOp::Mod),
        BinaryOperator::Pow => BinOpCategory::Simple(SimpleBinOp::Pow),
        BinaryOperator::Div => BinOpCategory::Simple(SimpleBinOp::Div),
        BinaryOperator::IDiv => BinOpCategory::Simple(SimpleBinOp::IDiv),
        BinaryOperator::BitAnd => BinOpCategory::Simple(SimpleBinOp::BitAnd),
        BinaryOperator::BitOr => BinOpCategory::Simple(SimpleBinOp::BitOr),
        BinaryOperator::BitXor => BinOpCategory::Simple(SimpleBinOp::BitXor),
        BinaryOperator::ShiftLeft => BinOpCategory::Simple(SimpleBinOp::ShiftLeft),
        BinaryOperator::ShiftRight => BinOpCategory::Simple(SimpleBinOp::ShiftRight),
        BinaryOperator::Concat => BinOpCategory::Concat,
        BinaryOperator::NotEqual => BinOpCategory::Comparison(ComparisonBinOp::NotEqual),
        BinaryOperator::Equal => BinOpCategory::Comparison(ComparisonBinOp::Equal),
        BinaryOperator::LessThan => BinOpCategory::Comparison(ComparisonBinOp::LessThan),
        BinaryOperator::LessEqual => BinOpCategory::Comparison(ComparisonBinOp::LessEqual),
        BinaryOperator::GreaterThan => BinOpCategory::Comparison(ComparisonBinOp::GreaterThan),
        BinaryOperator::GreaterEqual => BinOpCategory::Comparison(ComparisonBinOp::GreaterEqual),
        BinaryOperator::And => BinOpCategory::ShortCircuit(ShortCircuitBinOp::And),
        BinaryOperator::Or => BinOpCategory::ShortCircuit(ShortCircuitBinOp::Or),
    }
}

pub enum RegisterOrConstant {
    Register(RegisterIndex),
    Constant(ConstantIndex8),
}

pub fn simple_binop_opcode(
    simple_binop: SimpleBinOp,
    dest: RegisterIndex,
    left: RegisterOrConstant,
    right: RegisterOrConstant,
) -> OpCode {
    match simple_binop {
        SimpleBinOp::Add => match (left, right) {
            (RegisterOrConstant::Register(left), RegisterOrConstant::Register(right)) => {
                OpCode::AddRR { dest, left, right }
            }
            (RegisterOrConstant::Register(left), RegisterOrConstant::Constant(right)) => {
                OpCode::AddRC { dest, left, right }
            }
            (RegisterOrConstant::Constant(left), RegisterOrConstant::Register(right)) => {
                OpCode::AddCR { dest, left, right }
            }
            (RegisterOrConstant::Constant(left), RegisterOrConstant::Constant(right)) => {
                OpCode::AddCC { dest, left, right }
            }
        },
        _ => panic!("unsupported binary operator {:?}", simple_binop),
    }
}

pub fn simple_binop_const_fold<'gc>(
    simple_binop: SimpleBinOp,
    left: Value<'gc>,
    right: Value<'gc>,
) -> Option<Value<'gc>> {
    match simple_binop {
        SimpleBinOp::Add => left.add(right),
        _ => None,
    }
}

pub fn comparison_binop_opcode(
    comparison_binop: ComparisonBinOp,
    left: RegisterOrConstant,
    right: RegisterOrConstant,
    skip_if: bool,
) -> OpCode {
    match comparison_binop {
        ComparisonBinOp::Equal => match (left, right) {
            (RegisterOrConstant::Register(left), RegisterOrConstant::Register(right)) => {
                OpCode::EqRR {
                    skip_if,
                    left,
                    right,
                }
            }
            (RegisterOrConstant::Register(left), RegisterOrConstant::Constant(right)) => {
                OpCode::EqRC {
                    skip_if,
                    left,
                    right,
                }
            }
            (RegisterOrConstant::Constant(left), RegisterOrConstant::Register(right)) => {
                OpCode::EqCR {
                    skip_if,
                    left,
                    right,
                }
            }
            (RegisterOrConstant::Constant(left), RegisterOrConstant::Constant(right)) => {
                OpCode::EqCC {
                    skip_if,
                    left,
                    right,
                }
            }
        },
        ComparisonBinOp::NotEqual => match (left, right) {
            (RegisterOrConstant::Register(left), RegisterOrConstant::Register(right)) => {
                OpCode::EqRR {
                    skip_if: !skip_if,
                    left,
                    right,
                }
            }
            (RegisterOrConstant::Register(left), RegisterOrConstant::Constant(right)) => {
                OpCode::EqRC {
                    skip_if: !skip_if,
                    left,
                    right,
                }
            }
            (RegisterOrConstant::Constant(left), RegisterOrConstant::Register(right)) => {
                OpCode::EqCR {
                    skip_if: !skip_if,
                    left,
                    right,
                }
            }
            (RegisterOrConstant::Constant(left), RegisterOrConstant::Constant(right)) => {
                OpCode::EqCC {
                    skip_if: !skip_if,
                    left,
                    right,
                }
            }
        },
        _ => panic!("unsupported binary operator {:?}", comparison_binop),
    }
}

pub fn comparison_binop_const_fold<'gc>(
    comparison_binop: ComparisonBinOp,
    left: Value<'gc>,
    right: Value<'gc>,
) -> Option<Value<'gc>> {
    match comparison_binop {
        ComparisonBinOp::Equal => Some(Value::Boolean(left == right)),
        _ => None,
    }
}

pub fn unop_opcode(unop: UnaryOperator, dest: RegisterIndex, source: RegisterIndex) -> OpCode {
    match unop {
        UnaryOperator::Not => OpCode::Not { dest, source },
        _ => panic!("unimplemented unary operator {:?}", unop),
    }
}

pub fn unop_const_fold<'gc>(unop: UnaryOperator, value: Value<'gc>) -> Option<Value<'gc>> {
    match unop {
        UnaryOperator::Not => Some(Value::Boolean(!value.as_bool())),
        _ => None,
    }
}
