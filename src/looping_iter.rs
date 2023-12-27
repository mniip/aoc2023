use std::{
    cell::RefCell, collections::HashMap, hash::Hash, iter::FusedIterator, mem, ops::Deref, rc::Rc,
};

enum LoopingState<I: Iterator> {
    Searching {
        trace: Vec<I::Item>,
        locations: HashMap<I::Item, usize>,
        iter: I,
    },
    Exhausted {
        trace: Vec<I::Item>,
    },
    Looped {
        init: Vec<I::Item>,
        cycle: Vec<I::Item>,
    },
}

impl<I: Iterator> LoopingState<I>
where
    I::Item: Eq + Hash + Clone,
{
    fn get_cycle<'a>(init: &Vec<I::Item>, cycle: &'a Vec<I::Item>, n: usize) -> &'a I::Item {
        &cycle[(n - init.len()) % cycle.len()]
    }

    fn get(&mut self, n: usize) -> Option<&I::Item> {
        match self {
            Self::Exhausted { trace } => trace.get(n),
            Self::Looped { init, cycle } => Some(match init.get(n) {
                Some(value) => value,
                None => Self::get_cycle(init, cycle, n),
            }),
            Self::Searching {
                trace,
                locations,
                iter,
            } => {
                while trace.len() <= n {
                    if let Some(value) = iter.next() {
                        if let Some(&pos) = locations.get(&value) {
                            let mut init = mem::take(trace);
                            let cycle = init.split_off(pos);
                            *self = Self::Looped { init, cycle };
                            return match self {
                                Self::Looped { init, cycle } => {
                                    Some(Self::get_cycle(init, cycle, n))
                                }
                                _ => unreachable!(),
                            };
                        } else {
                            locations.insert(value.clone(), trace.len());
                            trace.push(value)
                        }
                    } else {
                        let trace = mem::take(trace);
                        *self = Self::Exhausted { trace };
                        return None;
                    }
                }
                match self {
                    Self::Searching { trace, .. } => Some(&trace[n]),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn force_loop(&mut self) -> (&[I::Item], &[I::Item]) {
        match self {
            Self::Exhausted { trace } => (trace, &[]),
            Self::Looped { init, cycle } => (init, cycle),
            Self::Searching {
                trace,
                locations,
                iter,
            } => loop {
                if let Some(value) = iter.next() {
                    if let Some(&pos) = locations.get(&value) {
                        let mut init = mem::take(trace);
                        let cycle = init.split_off(pos);
                        *self = Self::Looped { init, cycle };
                        break match self {
                            Self::Looped { init, cycle } => (init, cycle),
                            _ => unreachable!(),
                        };
                    } else {
                        locations.insert(value.clone(), trace.len());
                        trace.push(value)
                    }
                } else {
                    let trace = mem::take(trace);
                    *self = Self::Exhausted { trace };
                    break match self {
                        Self::Exhausted { trace } => (trace, &[]),
                        _ => unreachable!(),
                    };
                }
            },
        }
    }
}

#[derive(Clone)]
pub struct LoopingIter<I: Iterator> {
    pos: usize,
    state: Rc<RefCell<LoopingState<I>>>,
}

impl<I: Iterator> LoopingIter<I> {
    pub fn new(iter: I) -> LoopingIter<I> {
        LoopingIter {
            pos: 0,
            state: Rc::new(RefCell::new(LoopingState::Searching {
                trace: Vec::new(),
                locations: HashMap::new(),
                iter,
            })),
        }
    }

    pub fn loop_structure(&self) -> (Vec<I::Item>, Vec<I::Item>)
    where
        I::Item: Eq + Hash + Clone,
    {
        let mut state = self.state.borrow_mut();
        let (init, cycle) = state.force_loop();
        (init.to_vec(), cycle.to_vec())
    }
}

impl<I: Iterator> Iterator for LoopingIter<I>
where
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.state.borrow_mut().get(self.pos).cloned();
        self.pos += 1;
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.state.borrow().deref() {
            LoopingState::Searching { trace, .. } => (trace.len() - self.pos, None),
            LoopingState::Exhausted { trace } => {
                (trace.len() - self.pos, Some(trace.len() - self.pos))
            }
            LoopingState::Looped { .. } => (usize::MAX, None),
        }
    }

    fn count(self) -> usize {
        match self.state.borrow_mut().force_loop() {
            (init, []) => init.len(),
            _ => panic!("count of a looping iterator"),
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let result = self.state.borrow_mut().get(self.pos + n).cloned();
        self.pos += n;
        self.pos += 1;
        result
    }
}

impl<I: Iterator> FusedIterator for LoopingIter<I> where I::Item: Eq + Hash + Clone {}

pub trait Delooping: Iterator {
    fn delooping(self) -> LoopingIter<Self>
    where
        Self: Sized,
    {
        LoopingIter::new(self)
    }
}

impl<I: Iterator> Delooping for I {}
