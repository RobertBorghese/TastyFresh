/**********************************************************
 * --- C++ Transpiler ---
 *
 * A trait to be implemented by declarations in order to
 * transpile them to C++.
 **********************************************************/

pub trait CPPTranspiler {
	fn to_cpp(&self) -> String;
}
