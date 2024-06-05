use std::io;
use std::path::Path;

use tree_sitter::{Language, Node, Query, QueryCursor};

use super::common::path_spills_up;

/// Pick a [tree_sitter] parser that would likely
/// handle a file with the given filename extension.
fn language_for_filename_extension(ext: Option<&str>) -> Option<Language> {
    let ext = ext?;
    match ext {
        "py" => Some(tree_sitter_python::language()),
        "rs" => Some(tree_sitter_rust::language()),
        "ts" | "js" => Some(tree_sitter_typescript::language_typescript()),
        "tsx" | "jsx" => Some(tree_sitter_typescript::language_tsx()),
        _ => None,
    }
}

/// Given a filename extension, pick a pair of Tree Sitter Queries
/// that will match definitions and references for the language.
fn queries_for_filename_extension(ext: Option<&str>) -> Option<(Query, Query)> {
    use query_expressions::*;

    let ext = ext?;
    match ext {
        "py" => Some((
            Query::new(&tree_sitter_python::language(), python::DEFS).unwrap(),
            Query::new(&tree_sitter_python::language(), python::REFS).unwrap(),
        )),
        "rs" => Some((
            Query::new(&tree_sitter_rust::language(), rust::DEFS).unwrap(),
            Query::new(&tree_sitter_rust::language(), rust::REFS).unwrap(),
        )),
        "ts" | "js" => Some((
            Query::new(
                &tree_sitter_typescript::language_typescript(),
                typescript::DEFS,
            )
            .unwrap(),
            Query::new(
                &tree_sitter_typescript::language_typescript(),
                typescript::REFS,
            )
            .unwrap(),
        )),
        "tsx" | "jsx" => Some((
            Query::new(&tree_sitter_typescript::language_tsx(), typescript::DEFS).unwrap(),
            Query::new(&tree_sitter_typescript::language_tsx(), typescript::REFS).unwrap(),
        )),
        _ => None,
    }
}

/// Outline the definitions across all the files
/// contained in the given directory.
fn query_ast_of_directory(path: &Path) -> io::Result<String> {
    use std::fs;

    let mut result = String::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?.path();
        let basename = entry.strip_prefix(path).unwrap_or(&entry);
        let basename: String = basename
            .to_string_lossy()
            .chars()
            .flat_map(char::escape_default)
            .collect();
        if entry.is_dir() {
            result.push_str(&format!("{}/ (directory)", basename));
        } else {
            result.push_str(&format!("{} {}", basename, query_ast_of_file(&entry)?));
        }
        result.push('\n');
    }
    Ok(result)
}

/// Run the given query over the text, and return all the matches.
fn all_matches(query: &Query, node: Node, text_provider: &str) -> Vec<String> {
    use std::collections::HashSet;

    let mut cursor = QueryCursor::new();
    cursor
        .matches(query, node, text_provider.as_bytes())
        .flat_map(|m| m.captures)
        .map(|c| {
            c.node
                .utf8_text(text_provider.as_bytes())
                .expect("Node::utf8_text")
                .to_string()
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

/// Produce an S-expression representing the exports and imports of this module.
///
/// This is necessary for making a dependency graph of symbols within the codebase.
/// The format is as follows:
/// `(module name:'...' defs:[...] refs:[...])`
///
/// If parsing fails, return the unit value `()`.
fn query_ast_of_file(path: &Path) -> io::Result<String> {
    let filename_extension = Path::new(&path).extension().and_then(|ext| ext.to_str());
    let Some(language) = language_for_filename_extension(filename_extension) else {
        return Ok("()".into());
    };
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language)
        .expect("the parser should accept all languages");

    let source_code = std::fs::read_to_string(path)?;
    let Some(tree) = parser.parse(&source_code, None) else {
        return Err(io::Error::new(io::ErrorKind::Other, "could not parse"));
    };
    let root_node = tree.root_node();

    let Some((defs, refs)) = queries_for_filename_extension(filename_extension) else {
        return Ok("()".into());
    };

    let defs = all_matches(&defs, root_node, &source_code);
    let refs = all_matches(&refs, root_node, &source_code);

    let filename_without_extension = path.file_stem().unwrap().to_string_lossy();
    Ok(format!(
        "(module name:\"{}\" defs:{:?} refs:{:?})",
        filename_without_extension, defs, refs
    ))
}

pub mod rpc {
    use super::*;

    /// `tree-sitter dump ...`
    pub fn query_ast(arguments: &str) -> Result<String, String> {
        #[derive(serde::Deserialize)]
        struct Arguments {
            path: String,
        }
        let Arguments { path } = serde_json::from_str(arguments).map_err(|err| err.to_string())?;

        let path = Path::new(&path);
        if path_spills_up(path) {
            return Err("cannot read files outside the current directory".into());
        }
        if path.is_dir() {
            query_ast_of_directory(path).map_err(|err| err.to_string())
        } else {
            query_ast_of_file(path).map_err(|err| err.to_string())
        }
    }
}

mod query_expressions {
    // https://github.com/tree-sitter/tree-sitter-python/blob/master/queries/tags.scm
    pub mod python {
        pub const DEFS: &str = "
            (class_definition
                name: (identifier) @name)

            (function_definition
                name: (identifier) @name)
        ";

        pub const REFS: &str = "
            (identifier) @name
        ";
    }

    // https://github.com/tree-sitter/tree-sitter-typescript/blob/master/queries/tags.scm
    pub mod typescript {
        pub const DEFS: &str = "
            (function_signature
                name: (identifier) @name)

            (method_signature
                name: (property_identifier) @name)

            (abstract_method_signature
                name: (property_identifier) @name)

            (abstract_class_declaration
                name: (type_identifier) @name)

            (module
                name: (identifier) @name)

            (interface_declaration
                name: (type_identifier) @name)
        ";

        pub const REFS: &str = "
            (type_annotation
                (type_identifier) @name)

            (new_expression
                constructor: (identifier) @name)
        ";
    }

    // https://github.com/tree-sitter/tree-sitter-rust/blob/master/queries/tags.scm
    pub mod rust {
        pub const DEFS: &str = "
            (struct_item
                name: (type_identifier) @name)

            (enum_item
                name: (type_identifier) @name)

            (union_item
                name: (type_identifier) @name)

            (type_item
                name: (type_identifier) @name)

            (declaration_list
                (function_item
                    name: (identifier) @name))

            (function_item
                name: (identifier) @name)

            (trait_item
                name: (type_identifier) @name)

            (mod_item
                name: (identifier) @name)

            (macro_definition
                name: (identifier) @name)
        ";

        pub const REFS: &str = "
            (call_expression
                function: (identifier) @name)

            (call_expression
                function: (field_expression
                    field: (field_identifier) @name))

            (macro_invocation
                macro: (identifier) @name)
        ";
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "run manually to see output"]
    fn abstract_syntax_tree() {
        let tree = query_ast_of_file(Path::new("src/main.rs")).unwrap();
        println!("{}", tree);
        assert!(false);
    }

    #[test]
    #[ignore = "run manually to see output"]
    fn many_abstract_syntax_trees() {
        let tree = query_ast_of_directory(Path::new(".")).unwrap();
        println!("{}", tree);
        assert!(false);
    }
}
