pub mod scanner{
    pub struct Token {
        lexeme_type: LexemeType,
        str: Option<String>
    }

    impl Token {
        pub fn new(lexeme_type: LexemeType, str: Option<String>) -> Self {
            Self {lexeme_type: lexeme_type, str:str}
        }
    }

    pub enum LexemeType {
        Keyword(KeywordName),
        Operator(OperatorName),
        Punctuator(PunctuatorName),
        Literal,
        Identifier
    }

    pub enum KeywordName {
        C_FOR,
        C_WHILE, //etc (???)
        // then match the token to this using the dfa recogniser
        // also match this to priority when deciding on the token from the regex
    }

    pub enum OperatorName {

    }

    pub enum PunctuatorName {

    }

    fn remove_whitespace(){}


    fn scan(source_code: String){

    }


}


mod scanner_generator{
    use std::collections::HashMap;
    use std::collections::VecDeque;
    pub struct ReversePolishRE{
        stack: Vec<char>,
        output_queue: VecDeque<char>

    }

    impl ReversePolishRE{
        //pub fn new(input_RE: &'_ str) -> Self {
        //    
        //}
        //use shunting yard algorithm to eliminate parentheses and put into reverse polish notation
    }
    

    pub struct Automata<'a>{
        states: Vec<State<'a>>, //vector of states
        alphabet: &'a str, //literally just ascii lol but maybe ill need less in some cases?
        trans_table: HashMap<State<'a>, State<'a>>, //some kind of hashmap that points to the transition states for each state
        start: Vec<State<'a>>, //vector of states
        accept_states: Vec<State<'a>> //vector of states
    }
    //accept states is a subset of states, alphabet is just ascii, we want each state to have information about 
    //which states it can transition to. error state will be default(?). 
    pub struct State<'a>{
        id: &'a str,
    }
    
    pub trait DFA{
        //each of its transitions is uniquely determined by its source state and input symbol
        //reading an input symbol is required for each state transition
    }
    pub trait NFA{}

    fn thompson_construction(regular_expression: &str){}
    //thompson construction should be done on an re representing things in the language 
    //to create dfa depending on state of environment variable.
    //how am i going to get the re?
    //do it yourself unless you want to use lex or bison

    // thompson construction: build trivial NFAs for each character in the regex
    // apply alternation concatenation and closure to the collection of trivial NFAs
    // do this according to precedence
    // parentheses -> closure -> concatenation -> alternation
    // remove parentheses using shunting yard algorithm, then i can use thompson's
    // construction to construct the nfa 
    fn subset_construction(nfa: &str){}

    fn hopcroft_algo(dfa: &str){}

    fn dfa_recognizer(dfa: &str){}
}

use scanner::*;
use scanner::{KeywordName::*, OperatorName::*, PunctuatorName::*};
use scanner_generator::*;
fn main()
{
    let _x = scanner::Token::new(
        LexemeType::Keyword(C_FOR), None);
    
    //let _test_regex = scanner_generator::ReversePolishRE::new(
    //    "a?(a+b)*?b"
    //);
}