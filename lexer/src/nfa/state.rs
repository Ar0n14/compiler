use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    io::empty,
    ops::Deref,
    rc::Rc,
};
#[derive(Debug, Clone)]
pub struct StatePtr<T>(Rc<RefCell<State<T>>>);

impl<T> PartialEq for StatePtr<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl<T> Eq for StatePtr<T> {}
impl<T> Hash for StatePtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr: *const _ = &*self.0;
        ptr.hash(state);
    }
}

impl<T> Deref for StatePtr<T> {
    type Target = Rc<RefCell<State<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<Rc<RefCell<State<T>>>> for StatePtr<T> {
    fn from(value: Rc<RefCell<State<T>>>) -> Self {
        Self(value)
    }
}

impl<T: Debug> Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_ind(f, 1)
    }
}
impl<T: Debug> State<T> {
    fn fmt_with_ind(&self, f: &mut std::fmt::Formatter<'_>, ind: usize) -> std::fmt::Result {
        if ind > 10 {
            writeln!(
                f,
                "TOO MUCH TO DISPLAY. THE DISPLAY TRAIT IS IMPLEMENTED POORLY, BUT I WON'T FIX IT!"
            )?;
            return Ok(());
        }
        let pad = "  ".repeat(ind);
        writeln!(f, "{}State {:p} {{", pad, self)?;
        if !self.transitions.borrow().is_empty() {
            writeln!(
                f,
                "{}  return token: {:?}\n{0}  transitions:",
                pad, self.token
            )?;
            for (c, t) in self.transitions.borrow().iter() {
                writeln!(f, "    {}'{}':", pad, c.unwrap_or('ε'))?;
                for p in t.iter() {
                    p.borrow().fmt_with_ind(f, ind + 3)?;
                }
            }
        } else {
            writeln!(f, "{}  return token: {:?}", pad, self.token)?;
        }

        writeln!(f, "{}}}", pad)?;

        Ok(())
    }
}

impl<T: Debug> Display for StatePtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.borrow())
    }
}

#[derive(Debug, Clone)]
pub struct State<T> {
    pub transitions: RefCell<HashMap<Option<char>, HashSet<StatePtr<T>>>>,
    pub token: Option<T>,
}
impl<T> State<T> {
    pub fn new() -> StatePtr<T> {
        StatePtr(Rc::new(RefCell::new(State {
            transitions: RefCell::new(HashMap::new()),
            token: None,
        })))
    }

    pub fn add_transition(&self, symbol: Option<char>, target: StatePtr<T>) {
        self.transitions
            .borrow_mut()
            .entry(symbol)
            .or_insert_with(HashSet::new)
            .insert(target);
    }
}
