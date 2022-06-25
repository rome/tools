use rome_json_syntax::JsonSyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSet(u64);

impl TokenSet {
    pub const EMPTY: TokenSet = TokenSet(0);

    pub const fn singleton(kind: JsonSyntaxKind) -> TokenSet {
        TokenSet(mask(kind))
    }

    pub const fn union(self, other: TokenSet) -> TokenSet {
        TokenSet(self.0 | other.0)
    }

    pub fn contains(&self, kind: JsonSyntaxKind) -> bool {
        let num = kind as usize;
        self.0 & mask(kind) != 0
    }
}

const fn mask(kind: JsonSyntaxKind) -> u64 {
    let num = kind as usize;
    1u64 << num
}

/// Utility macro for making a new token set
#[macro_export]
macro_rules! token_set {
    ($($t:expr),*) => {{
            use $crate::TokenSet;
            TokenSet::EMPTY$(.union(TokenSet::singleton($t)))*
        }};
    ($($t:expr),* ,) => { token_set!($($t),*) };
}
