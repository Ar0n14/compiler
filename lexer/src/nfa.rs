mod state;

use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

use crate::nfa::state::{State, StatePtr};
use crate::regex::Regex;

#[derive(Debug)]
pub struct NFA<T> {
    pub start: StatePtr<T>,
    pub accept: StatePtr<T>,
}

impl<T: Clone + std::fmt::Debug> NFA<T> {
    pub fn build_nfa(regex: Regex, accept_label: Option<T>) -> NFA<T> {
        match regex {
            Regex::Literal(c) => NFA::literal(c, accept_label),
            Regex::Concat(a, b) => {
                NFA::concat(NFA::build_nfa(*a, None), NFA::build_nfa(*b, accept_label))
            }
            Regex::Union(a, b) => {
                NFA::union(NFA::build_nfa(*a, None), NFA::build_nfa(*b, accept_label))
            }
            Regex::Star(a) => NFA::star(NFA::build_nfa(*a, accept_label)),
            Regex::Empty => {
                panic!("Empty Regex found. This shouldn't ever happen!");
            }
        }
    }
    pub fn literal(c: Option<char>, accept_label: Option<T>) -> NFA<T> {
        let s = State::<T>::new();
        let t = State::<T>::new();

        s.borrow().add_transition(c, Rc::clone(&t).into());
        t.borrow_mut().token = accept_label;
        NFA {
            start: s,
            accept: t,
        }
    }
    pub fn concat(a: NFA<T>, b: NFA<T>) -> NFA<T> {
        a.accept
            .borrow()
            .add_transition(None, Rc::clone(&b.start).into());

        NFA {
            start: a.start,
            accept: b.accept,
        }
    }
    pub fn union(a: NFA<T>, b: NFA<T>) -> NFA<T> {
        let new_start = State::<T>::new();
        let new_accept = State::<T>::new();

        new_start
            .borrow()
            .add_transition(None, Rc::clone(&a.start).into());
        new_start
            .borrow()
            .add_transition(None, Rc::clone(&b.start).into());

        a.accept
            .borrow()
            .add_transition(None, Rc::clone(&new_accept).into());
        b.accept
            .borrow()
            .add_transition(None, Rc::clone(&new_accept).into());

        new_accept.borrow_mut().token = b.accept.borrow().token.clone();
        NFA {
            start: new_start,
            accept: new_accept,
        }
    }
    pub fn star(a: NFA<T>) -> NFA<T> {
        let new_start = State::<T>::new();
        let new_accept = State::<T>::new();

        new_start
            .borrow()
            .add_transition(None, Rc::clone(&a.start).into());
        new_start
            .borrow()
            .add_transition(None, Rc::clone(&new_accept).into());
        a.accept
            .borrow()
            .add_transition(None, Rc::clone(&a.start).into());
        a.accept
            .borrow()
            .add_transition(None, Rc::clone(&new_accept).into());

        new_accept.borrow_mut().token = a.accept.borrow().token.clone();

        NFA {
            start: new_start,
            accept: new_accept,
        }
    }
}

impl<T: Debug> Display for NFA<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: \n{}accept: \n{}", self.start, self.accept)
    }
}
