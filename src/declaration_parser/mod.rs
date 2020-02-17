/**********************************************************
 * --- Declaration Parser ---
 *
 * Parses out all of the "declared" information from the
 * source files. This information is necessary prior
 * to parsing and type-checking the expression content
 * contained within the program.
 **********************************************************/

pub mod parser;
pub mod declaration;
pub mod module_declaration;
pub mod variable_declaration;
pub mod function_declaration;
pub mod attribute_declaration;
pub mod include_declaration;
pub mod import_declaration;
