fn main() {
    let lang = tree_sitter_scala::language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(lang).unwrap();
    let _ = parser.parse(SRC, None);
}

const SRC: &str = include_str!("bad.scala");
