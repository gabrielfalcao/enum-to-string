use std::collections::{BTreeMap, VecDeque};
use proc_macro2::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Copy)]
pub(crate) enum Kind {
    Ident,
    Punct,
    Group,
    Literal,
}
impl Into<Kind> for &Kind {
    fn into(self) -> Kind {
        self.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct Token {
    kind: Kind,
    content: String,
    or: Option<Box<Token>>,
    and: Option<Box<Token>>,
}
impl Token {
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }
    pub fn content(&self) -> String {
        self.content.clone()
    }
    pub fn of(kind: Kind, content: impl Into<String>) -> Token {
        let content: String = content.into();
        Token {
            kind,
            content,
            and: None,
            or: None,
        }
    }
    pub fn or(&mut self, other: Token) -> Token {
        let mut expect = {
            let mut expect = Box::new(self.clone());
            while let Some(other) = expect.or {
                expect = other
            }
            expect
        };
        expect.or = Some(Box::new(other));
        self.clone()
    }
    pub fn and(&mut self, other: Token) -> Token {
        let mut expect = {
            let mut expect = Box::new(self.clone());
            while let Some(other) = expect.and {
                expect = other
            }
            expect
        };
        expect.and = Some(Box::new(other));
        self.clone()
    }
}
impl Into<Kind> for Token {
    fn into(self) -> Kind {
        self.kind()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct OptionalToken {
    token: Token,
}

impl OptionalToken  {
    pub fn of(kind: Kind, content: impl Into<String>) -> OptionalToken {
        OptionalToken::from_token(Token::of(kind, content))
    }
    pub fn from_token(token: Token) -> OptionalToken {
        OptionalToken {
            token,
        }
    }
}


#[derive(Debug, Clone)]
pub(crate) struct Match {
    kind: Kind,
    content: String,
    span: Span,
}

impl Match {
    pub fn new(expected: Token, span: Span) -> Match {
        Match {
            kind: expected.kind(),
            content: expected.content(),
            span,
        }
    }
}

impl Default for TokenExpectations {
    fn default() -> TokenExpectations {
        TokenExpectations {
            tokens: BTreeMap::new(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TokenExpectations {
    tokens: BTreeMap<Kind, VecDeque<Token>>,
}

impl TokenExpectations {
    pub fn expect(&mut self, kind: Kind, content: impl Into<String>) {
        let ness = content.into();
        let expectation = Token::of(kind, ness);
        if let Some(queue) = self.tokens.get_mut(&kind) {
            queue.push_front(expectation);
        } else {
            let queue = VecDeque::from(vec![expectation]);
            self.tokens.insert(kind, queue);
        }
    }
    // pub fn parse_tree_iter(&mut self, stream: impl Iterator<Item = TokenTree>, on_success: impl Fn(TokenTree, Token), on_finished: impl Fn(Vec<Token>)) -> Result<(), TokenStream> {
    //     let mut matches: Vec<Token> = Vec::new();
    //     for tok in stream {

    //     }
    //     Ok(())
    // }
}
