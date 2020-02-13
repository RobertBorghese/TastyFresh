/**********************************************************
 * --- Expression Part ---
 *
 * Used to store and represent an expression component
 * within a linear list prior to being parsed into an AST.
 **********************************************************/

use crate::expression::Expression;

use crate::context_management::position::Position;

use std::rc::Rc;

pub enum ExpressionPiece {
	Expression(Rc<Expression>),
	Prefix(usize, Position),
	Value(String, Position),
	Suffix(usize, Position),
	Infix(usize, Position),
	EncapsulatedValues(Rc<Vec<Expression>>, Position),
	FunctionParameters(Rc<Vec<Expression>>, Position),
	ArrayAccessParameters(Rc<Vec<Expression>>, Position),
	TernaryCondition(Rc<Vec<Expression>>, Position)
}