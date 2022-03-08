use rome_js_syntax::JsSyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenSet([u128; 2]);

impl TokenSet {
    pub const EMPTY: TokenSet = TokenSet([0; 2]);

    pub const fn singleton(kind: JsSyntaxKind) -> TokenSet {
        TokenSet(mask(kind))
    }

    pub const fn union(self, other: TokenSet) -> TokenSet {
        TokenSet([self.0[0] | other.0[0], self.0[1] | other.0[1]])
    }

    pub fn contains(&self, kind: JsSyntaxKind) -> bool {
        let num = kind as usize;
        match num {
            0..=127 => self.0[0] & mask(kind)[0] != 0,
            _ => self.0[1] & mask(kind)[1] != 0,
        }
    }
}

const fn mask(kind: JsSyntaxKind) -> [u128; 2] {
    let num = kind as usize;
    match num {
        0..=127 => [1u128 << num, 0],
        _ => [0, 1u128 << (num - 127)],
    }
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
