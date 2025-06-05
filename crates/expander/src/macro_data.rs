use std::{cell::RefCell, collections::HashMap, rc::Rc};

use data::Ident;
use util::{AsmResult, Location, TokenKind, Tokenizer};

use crate::{macro_process::{parse_args, parse_def_stream, parse_params, replaced_stream}, macro_tokenizer::MacroTokenizer};

#[derive(Debug, Clone)]
pub enum Expander {
    Replace((Rc<Vec<TokenKind>>, Rc<Vec<TokenKind>>)), // args, stream
    Definition,
    Nasm,
    For,
}

impl Expander {
    pub fn expand<'code>(
        &self,
        macro_data: MacroData,
        stream: Rc<Vec<TokenKind>>,
    ) -> AsmResult<'code, Rc<Vec<TokenKind>>> {
        match self {
            Expander::Definition => {
                // name shoulb be "macro_def"
                // stream should be following cases
                // <macro_name> '(' <args>* ')' "=>" { <newline> <stream> <newline> }
                eprintln!("read macro-def");
                let tokenizer = Rc::new(MacroTokenizer::new(Location::default(), stream));

                tokenizer.consume_token(TokenKind::OpenParenthesis)?;
                tokenizer.skip_space();

                let macro_name = match tokenizer.peek_token().kind {
                    TokenKind::Identifier(ident) => {
                        tokenizer.next_token();
                        Ident::new(ident.to_string())
                    },
                    _ => {
                        // eprintln!("e: {:?}", tokenizer.peek_token());
                        return Err(util::AsmError::ParseError(
                            tokenizer.location(),
                            "expected Ident, but found others".to_string(),
                            String::new(),
                        ));
                    }
                };
                tokenizer.skip_space();

                tokenizer.consume_token(TokenKind::OpenParenthesis)?;
                tokenizer.skip_space();

                let mut params = Vec::new();
                parse_args(tokenizer.clone(), &mut params)?;
                tokenizer.skip_space();

                tokenizer.consume_token(TokenKind::Arcane('='))?;
                tokenizer.consume_token(TokenKind::GreaterThan)?;
                tokenizer.skip_space();

                let def_stream = parse_def_stream(tokenizer)?;

                macro_data.register_macro(
                    macro_name,
                    Expander::Replace((Rc::new(params), Rc::new(def_stream))),
                );
                eprintln!("expand_defmacro_end");
                Ok(Rc::new(Vec::new()))
            }
            Expander::Replace((params, def_stream)) => {
                // 1. anlyze the stream
                // 2. connect the anlyzed stream and args
                // 3. load the def_stream and relpace them
                eprintln!("expand_replace_start");
                let stream_tokenizer = Rc::new(MacroTokenizer::new(Location::default(), stream));
                
                stream_tokenizer.consume_token(TokenKind::OpenParenthesis)?;

                let mut args = Vec::new();
                parse_params(stream_tokenizer, &mut args)?;

                let replacement_table = if params.len() == args.len() {
                    params
                        .iter()
                        .cloned()
                        .zip(args)
                        .collect::<HashMap<TokenKind, Vec<TokenKind>>>()
                } else {
                    unimplemented!("should be error")
                };

                let replaced = replaced_stream(def_stream.clone(), replacement_table)?;
                eprintln!("expand_replace_end");
                Ok(Rc::new(replaced))
            }
            Expander::Nasm => {
                eprintln!("read nasm");
                let def_stream = stream[1..stream.len() -1]. to_vec();
                
                let nasm = Rc::new(def_stream.iter().map(|s| format!("{}", s)).collect::<Vec<String>>().concat());
                eprintln!("end nasm");
                Ok(Rc::new(vec![TokenKind::Identifier(Rc::new("nasm".to_string())), TokenKind::OpenParenthesis, TokenKind::String(nasm),TokenKind::CloseParenthesis ]))
            }
            Expander::For => {
                eprintln!("read for");
                let tokenizer = Rc::new(MacroTokenizer::new(Location::default(), stream));

                tokenizer.consume_token(TokenKind::OpenParenthesis)?;
                tokenizer.skip_space();
                
                let meta = match tokenizer.peek_token().kind {
                    TokenKind::Identifier(ident) => {
                        tokenizer.next_token();
                        TokenKind::Identifier(ident.clone())
                    }
                    _ => {
                        eprintln!("e: {:?}", tokenizer.peek_token());
                        return Err(util::AsmError::ParseError(
                            tokenizer.location(),
                            "expected Ident, but found others".to_string(),
                            String::new(),
                        ));
                    }
                };
                tokenizer.skip_space();

                if matches!(tokenizer.peek_token().kind, TokenKind::Identifier(s) if s.to_string() == "in" ) {
                    tokenizer.next_token();
                } else {
                    return Err(todo!());
                }
                tokenizer.skip_space();


                tokenizer.consume_token(TokenKind::OpenParenthesis)?;
                let mut args = Vec::new();
                parse_params(tokenizer.clone(), &mut args)?;
                tokenizer.skip_space();
                
                eprintln!("args: {:?}", args);

                tokenizer.consume_token(TokenKind::Arcane('='))?;
                tokenizer.consume_token(TokenKind::GreaterThan)?;
                tokenizer.skip_space();
                
                let def_stream = Rc::new(parse_def_stream(tokenizer)?);
                
                let mut output_stream = vec![TokenKind::OpenBrace];
                
                for i in args {
                    eprintln!("{:?}", i);
                    let mut replacement_table = HashMap::new();
                    replacement_table.insert(meta.clone(), i);
                    let replaced   = replaced_stream(def_stream.clone(), replacement_table)?;
                    output_stream.append(&mut replaced[1..replaced.len() -1]. to_vec());
                }
                eprintln!("{:?}", output_stream);
                output_stream.push(TokenKind::CloseBrace);
                eprintln!("expand for");
                Ok(Rc::new(output_stream))
            }
        }
    }
}

#[derive(Clone)]
pub struct MacroData {
    definition: Rc<RefCell<HashMap<Ident, Expander>>>,
}

impl MacroData {
    pub fn new() -> Self {
        Self {
            definition: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, name: Ident) -> Option<Expander> {
        match name.get_str().as_str() {
            "def_macro" => Some(Expander::Definition),
            "nasm" => Some(Expander::Nasm),
            "for" => Some(Expander::For),
            _ => self.definition.borrow().get(&name).cloned()
        }
    }

    pub fn register_macro(&self, name: Ident, expander: Expander) {
        self.definition.borrow_mut().insert(name, expander);
    }
}

impl Default for MacroData {
    fn default() -> Self {
        Self::new()
    }
}
