use tree_sitter::{Language, Parser, Tree};

pub enum Lang {
    Rust,
    Toml,
    TypeScript,
    Tsx,
}

pub struct Ccc {
    parser: Parser,
}

impl From<Lang> for Language {
    fn from(lang: Lang) -> Self {
        match lang {
            Lang::Rust => tree_sitter_rust::language(),
            Lang::Toml => tree_sitter_toml::language(),
            Lang::TypeScript => tree_sitter_typescript::language_typescript(),
            Lang::Tsx => tree_sitter_typescript::language_tsx(),
        }
    }
}

impl TryFrom<String> for Lang {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "rs" => Ok(Lang::Rust),
            "toml" => Ok(Lang::Toml),
            "ts" => Ok(Lang::TypeScript),
            "tsx" => Ok(Lang::Tsx),
            _ => Err("invalid extension".into()),
        }
    }
}

impl Ccc {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    pub fn parse<S>(&mut self, text: S, lang: Lang) -> Option<Tree>
    where
        S: AsRef<[u8]>,
    {
        self.parser.set_language(lang.into()).unwrap();

        self.parser.parse(text, None)
    }
}
