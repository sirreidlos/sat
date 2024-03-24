use core::panic;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Undecided(bool),
    Decided(bool),
    Empty,
    Discarded,
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Undecided(false),
            1 => Self::Undecided(true),
            2 => Self::Empty,
            _ => panic!(),
        }
    }
}

impl State {
    const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    const fn is_discarded(&self) -> bool {
        matches!(self, Self::Discarded)
    }

    const fn is_decided(&self) -> bool {
        matches!(self, Self::Decided(_))
    }

    const fn is_undecided(&self) -> bool {
        matches!(self, Self::Undecided(_))
    }

    fn set_decided(&self) -> Self {
        if let Self::Undecided(b) = self {
            Self::Decided(*b)
        } else {
            *self
        }
    }

    fn conflicts_with(&self, other: &Self) -> bool {
        if let Self::Undecided(l) = self {
            if let Self::Undecided(r) = other {
                return l != r;
            }
        }

        false
    }

    fn evaluate(&self, other: &Self) -> Self {
        if self.is_decided() {
            return *self;
        }

        if self.is_empty() {
            return *other;
        }

        if self.is_discarded() {
            return State::Discarded;
        }

        if self.conflicts_with(other) {
            return State::Discarded;
        }

        if self == other {
            return *self;
        }

        if self.is_undecided() {
            return *self;
        }

        panic!("Should've exhausted all. {:?} {:?}", self, other);
    }
}

#[derive(Debug, Clone)]
struct Clause(Vec<State>);

impl Clause {
    pub fn from_vec(input: Vec<u8>) -> Self {
        Self(input.into_iter().map(State::from).collect::<Vec<State>>())
    }
}

impl IntoIterator for Clause {
    type Item = State;

    type IntoIter = std::vec::IntoIter<State>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

struct Formula {
    clauses: Vec<Clause>,
}

impl Formula {
    pub fn from_2d(input: Vec<Vec<u8>>) -> Self {
        Self {
            clauses: input.into_iter().map(Clause::from_vec).collect(),
        }
    }

    pub fn solve(&self) -> bool {
        let mut clauses = self.clauses.clone();
        let mut evaluation_state = clauses.first().unwrap().0.clone();

        // this should actually start from idx 1, but clauses[1..] seems to break it so i cant be bothered now, its just a redundant check it shouldn't affect actual result`
        for clause in clauses {
            let clause = clause.0;
            let curr_eval_state = evaluation_state.clone();
            let temp_state: Vec<State> = evaluation_state
                .iter()
                .zip(clause.iter())
                .map(|(state, clause)| state.evaluate(clause))
                .collect();

            println!("Current Evaluation State: {:?}", evaluation_state);
            println!("          Current Clause: {:?}", clause);
            println!("  Temp Evaluation Result: {:?}", temp_state);

            if temp_state.iter().all(|c| c.is_discarded()) {
                evaluation_state
                    .iter_mut()
                    .for_each(|c| *c = c.set_decided());

                let replace_idx = evaluation_state
                    .iter()
                    .zip(clause.iter())
                    .enumerate()
                    .find(|(_, (eval, curr_clause))| {
                        eval.is_discarded() && curr_clause.is_undecided()
                    })
                    .map(|(idx, _)| idx);

                if replace_idx.is_none() {
                    println!("False. Final Evaluation: {:?}", evaluation_state);
                    return false;
                }

                let replace_idx = replace_idx.unwrap();

                evaluation_state[replace_idx] = clause[replace_idx];
            } else {
                evaluation_state = temp_state;
            }
        }

        println!("True. Final Evaluation: {:?}", evaluation_state);

        true
    }
}

fn main() {
    let input = vec![
        vec![0, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2],
        vec![1, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2],
        vec![0, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 1, 2, 2],
        vec![2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2],
        vec![2, 1, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2],
        vec![2, 0, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2],
        vec![2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2],
        vec![2, 2, 1, 2, 2, 2, 2, 1, 2, 2, 1, 2, 2, 2, 2, 2, 2],
        vec![2, 2, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 0, 2],
        vec![2, 2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 2],
        vec![2, 2, 0, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1],
        vec![2, 2, 2, 0, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 1, 2, 2],
        vec![2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 0, 2],
        vec![2, 2, 2, 2, 1, 0, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2],
        vec![2, 2, 2, 2, 1, 2, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2],
        vec![2, 2, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2],
        vec![2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 1, 2, 2, 2, 2, 2, 2],
        vec![2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2],
        vec![2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2],
        vec![2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 0],
        vec![2, 2, 2, 2, 2, 0, 2, 2, 2, 1, 2, 2, 2, 2, 2, 0, 2],
        vec![2, 2, 2, 2, 2, 2, 1, 2, 2, 1, 2, 2, 2, 0, 2, 2, 2],
        vec![2, 2, 2, 2, 2, 2, 2, 0, 1, 2, 1, 2, 2, 2, 2, 2, 2],
        vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 0, 0, 2, 2],
        vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 1, 2, 0, 2],
        vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 1, 2, 2, 0],
    ];

    let mut formula = Formula::from_2d(input);
    formula.solve();
}
