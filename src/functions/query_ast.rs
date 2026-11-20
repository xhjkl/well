use std::io;
use std::path::Path;

use tree_sitter::{Language, Node, Query, QueryCursor, StreamingIterator};

use super::common::path_spills_up;

/// Select a parser and its definition and reference queries for a filename extension.
fn language_and_queries(ext: Option<&str>) -> Option<(Language, &'static str, &'static str)> {
    use query_expressions::*;

    let (language, defs, refs) = match ext? {
        "py" => (tree_sitter_python::LANGUAGE, python::DEFS, python::REFS),
        "rs" => (tree_sitter_rust::LANGUAGE, rust::DEFS, rust::REFS),
        "ts" | "js" => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
            typescript::DEFS,
            typescript::REFS,
        ),
        "tsx" | "jsx" => (
            tree_sitter_typescript::LANGUAGE_TSX,
            typescript::DEFS,
            typescript::REFS,
        ),
        _ => return None,
    };
    Some((language.into(), defs, refs))
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

    let source = text_provider.as_bytes();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, node, source);
    let mut names = HashSet::new();
    while let Some(query_match) = matches.next() {
        for capture in query_match.captures() {
            let name = capture.node.utf8_text(source).expect("Node::utf8_text");
            names.insert(name);
        }
    }
    names.into_iter().map(str::to_owned).collect()
}

/// Produce an S-expression representing the exports and imports of this module.
///
/// This is necessary for making a dependency graph of symbols within the codebase.
/// The format is as follows:
/// `(module name:'...' defs:[...] refs:[...])`
///
/// If parsing fails, return the unit value `()`.
fn query_ast_of_file(path: &Path) -> io::Result<String> {
    let filename_extension = path.extension().and_then(|ext| ext.to_str());
    let Some((language, defs, refs)) = language_and_queries(filename_extension) else {
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

    let defs = Query::new(&language, defs).expect("valid definition query");
    let refs = Query::new(&language, refs).expect("valid reference query");
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
    fn captures_definitions_and_references_with_each_grammar() {
        for (ext, source, definition, reference) in [
            (
                "py",
                "def sample():\n    return target()\n",
                "sample",
                "target",
            ),
            ("rs", "fn sample() { target(); }", "sample", "target"),
            (
                "ts",
                "interface Sample {}\nnew Target();",
                "Sample",
                "Target",
            ),
            (
                "tsx",
                "interface Sample {}\nconst element = <div />;\nnew Target();",
                "Sample",
                "Target",
            ),
        ] {
            let (language, defs, refs) = language_and_queries(Some(ext)).unwrap();
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(&language).unwrap();
            let tree = parser.parse(source, None).unwrap();
            let defs = Query::new(&language, defs).unwrap();
            let refs = Query::new(&language, refs).unwrap();
            let defs = all_matches(&defs, tree.root_node(), source);
            let refs = all_matches(&refs, tree.root_node(), source);
            assert!(
                defs.iter().any(|name| name == definition),
                "{ext}: {defs:?}"
            );
            assert!(refs.iter().any(|name| name == reference), "{ext}: {refs:?}");
        }
    }

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
