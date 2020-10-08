//! This file contains tests for exported nodes in the automata.

use enso_prelude::*;

use enso_automata::nfa::Nfa;
use enso_automata::dfa::Dfa;
use enso_automata::Pattern;

pub struct Rule {
    pub pattern : Pattern,
    pub rule : String
}
impl Rule {
    pub fn new(pattern_ref:&Pattern, rule_str:impl Str) -> Self {
        let rule    = rule_str.into();
        let pattern = pattern_ref.clone();
        Rule {pattern,rule}
    }
}

#[test]
fn define_exported_automaton() {
    // The Patterns
    let a_word = Pattern::char('a').many1();
    let b_word = Pattern::char('b').many1();
    let any    = Pattern::any();
    let end    = Pattern::eof();

    let patterns = vec![
        Rule::new(&a_word,"rule_1"),
        Rule::new(&b_word,"rule_2"),
        Rule::new(&end,"rule_3"),
        Rule::new(&any,"rule_4"),
    ];

    // The NFA
    let mut states:Vec<_>          = default();
    let mut callbacks:HashMap<_,_> = default();
    let mut nfa                    = Nfa::default();
    let start                      = nfa.start;
    let end                        = nfa.new_state_exported();
    states.push(start);
    for rule in patterns.into_iter() {
        let state = nfa.new_pattern(start,rule.pattern);
        states.push(state);
        callbacks.insert(state,rule.rule);
        nfa.connect(state,end);
    }
    states.push(end);

    // The DFA
    let dfa = Dfa::from(&nfa);

    println!("States: {:?}",states);
    println!("Callbacks: {:?}",callbacks);
    println!("Sources: {:?}",dfa.sources);
}