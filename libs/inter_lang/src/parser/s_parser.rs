use syntax::exports::sToken;

use crate::ast::eValue;
use crate::ast::sInterLangObject;
use crate::ast::sKVP;
use crate::ast::sKey;
use crate::ast::sListValue;
use crate::ast::sObjectValue;
use crate::ast::sStringValue;
use crate::lexer::eTokenIdentifier;
use crate::parser::eParserContext;

pub struct sParser {
    char_index: usize,
    line_index: usize,
    prev_token: Option<eTokenIdentifier>,
}

type aToken = sToken<eTokenIdentifier>;

const PREV_TOKEN_EXPECT_MESSAGE: &str = "We have already called self.expect at this point, so unless the logic within that method is incorrect (in which case it should reviewed), this unwrap should not fail.";
const NON_EXHAUSTIVE_PREV_TOKEN_MATCH: &str = "The last self.expect call guarantees that a legal match arm is executed.";

impl sParser {
    pub fn new() -> Self {
        Self {
            char_index: 1,
            line_index: 1,
            prev_token: None,
        }
    }

    pub fn parse(&mut self, tokens: &mut Vec<aToken>) -> sInterLangObject {
        self.char_index = 1;
        self.line_index = 1;

        let mut kvps: Vec<sKVP> = vec![];

        while tokens.len() > 0 {
            kvps.push(self.parse_kvp(tokens, 0));
        }

        sInterLangObject::new(kvps)
    }

    fn parse_kvp(&mut self, tokens: &mut Vec<aToken>, tabs: usize) -> sKVP {
        let key = self.parse_key(tokens, tabs);
        let value = self.parse_value(tokens, tabs, eParserContext::Object);

        sKVP::new(key, value)
    }

    fn parse_key(&mut self, tokens: &mut Vec<aToken>, tabs: usize) -> sKey {
        let mut key = "".to_string();

        for _ in 0..tabs {
            self.expect(vec![eTokenIdentifier::Tab], tokens);
        }

        self.expect(vec![eTokenIdentifier::LowerCaseLetter], tokens);

        loop {
            let mut prev_token = self.prev_token.expect(PREV_TOKEN_EXPECT_MESSAGE);

            let possible_key_char = match prev_token {
                eTokenIdentifier::LowerCaseLetter => self.expect(vec![eTokenIdentifier::LowerCaseLetter, eTokenIdentifier::Underscore, eTokenIdentifier::Colon], tokens),
                eTokenIdentifier::Underscore => self.expect(vec![eTokenIdentifier::LowerCaseLetter, eTokenIdentifier::Number], tokens),
                eTokenIdentifier::Number => self.expect(vec![eTokenIdentifier::Number, eTokenIdentifier::Underscore, eTokenIdentifier::Colon], tokens),
                _ => panic!("{}", NON_EXHAUSTIVE_PREV_TOKEN_MATCH),
            };

            prev_token = self.prev_token.expect(PREV_TOKEN_EXPECT_MESSAGE);

            if prev_token == eTokenIdentifier::Colon {
                break;
            }

            key += possible_key_char.as_str();
        }

        self.expect(vec![eTokenIdentifier::Whitespace], tokens);

        sKey::new(key.as_str())
    }

    fn parse_value(&mut self, tokens: &mut Vec<aToken>, tabs: usize, context: eParserContext) -> eValue {
        if context == eParserContext::List {
            for _ in 0..tabs {
                self.expect(vec![eTokenIdentifier::Tab], tokens);
            }
        }

        self.expect(vec![eTokenIdentifier::ParenOpen, eTokenIdentifier::SquareOpen, eTokenIdentifier::CurlyOpen], tokens);

        let prev_token = self.prev_token.expect(PREV_TOKEN_EXPECT_MESSAGE);

        match prev_token {
            eTokenIdentifier::ParenOpen => eValue::String(self.parse_string_value(tokens, tabs, context)),
            eTokenIdentifier::SquareOpen => eValue::List(self.parse_list_value(tokens, tabs, context)),
            eTokenIdentifier::CurlyOpen => eValue::Object(self.parse_object_value(tokens, tabs, context)),
            _ => panic!("{}", NON_EXHAUSTIVE_PREV_TOKEN_MATCH),
        }
    }

    fn parse_string_value(&mut self, tokens: &mut Vec<aToken>, tabs: usize, context: eParserContext) -> sStringValue {
        todo!()
        // let mut value = "".to_string();

        // self.expect(eTokenIdentifier::as_vec(), tokens);

        // let mut prev_token = self.prev_token.expect(PREV_TOKEN_EXPECT_MESSAGE);

        // match prev_token {
        //     eTokenIdentifier::Newline => {
        //         loop {

        //         }
        //     }
        //     eTokenIdentifier::ParenClose => (),
        //     _ => {
        //         loop {

        //         }
        //     }
        // }

        // sStringValue::new(value.as_str())
    }

    fn parse_list_value(&mut self, tokens: &mut Vec<aToken>, tabs: usize, context: eParserContext) -> sListValue {
        todo!()
    }

    fn parse_object_value(&mut self, tokens: &mut Vec<aToken>, tabs: usize, context: eParserContext) -> sObjectValue {
        todo!()
    }

    fn expect(&mut self, expected_identifiers: Vec<eTokenIdentifier>, tokens: &mut Vec<aToken>) -> String {
        let actual_token = self.take_token(tokens);

        if !expected_identifiers.contains(&actual_token.get_identifier()) {
            let mut expected_identifiers_string = "".to_string();

            for (i, expected_identifier) in expected_identifiers.iter().enumerate() {
                if i > 0 {
                    expected_identifiers_string += ", ";
                }

                expected_identifiers_string += format!("{}", expected_identifier).as_str();
            }

            panic!("Syntax Error<InterLang> @[line: {}, char: {}] :: Expected {}, got {}.", self.line_index, self.char_index, expected_identifiers_string, actual_token);
        }

        self.prev_token = Some(actual_token.get_identifier());

        actual_token.get_value().to_string()
    }

    fn take_token(&mut self, tokens: &mut Vec<aToken>) -> aToken {
        if tokens.len() == 0 {
            panic!("Programmer Logic Error<InterLang> :: take_token called after tokens Vec was emptied. This should never happen, meaning that all call sites need to be reviewed for logical errors.");
        }

        let next_token = tokens.pop().unwrap();

        if next_token.get_identifier() == eTokenIdentifier::Newline {
            self.line_index += 1;
            self.char_index = 0;
        } else {
            self.char_index += next_token.get_value().len();
        }

        tokens.pop().unwrap()
    }
}
