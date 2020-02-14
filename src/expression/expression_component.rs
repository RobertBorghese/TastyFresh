/**********************************************************
 * --- Expression Components ---
 *
 * The components that make up Tasty Fresh expressions
 * are represented using enums constructed in this file.
 **********************************************************/

use crate::expression::Expression;
use crate::expression::value_type::ValueType;

pub enum ExpressionComponent {
	Test(String),
	Test2(String, Vec<Expression>),
	Expression(Expression),
	Value(String, ValueType),
	PrefixOperator(String, Expression, i32),
	SuffixOperator(String, Expression, i32),
	InfixOperator(String, Expression, Expression, i32)
}


