/**********************************************************
 * --- Source Files ---
 *
 * Stores the source files read by the compiler.
 **********************************************************/

use std::cell::RefCell;

struct SourceFiles {
	files: BTreeMap<String,SourceFile>;
}

struct SourceFile {
	content: String,
	
}
