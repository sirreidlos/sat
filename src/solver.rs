use rustc_hash::FxHashMap;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use crate::dimacs::InputClause;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// enum VariableState {
//     Undecided(bool),
//     Decided(bool),
//     Empty,
//     Discarded,
// }

// impl From<u8> for VariableState {
//     fn from(value: u8) -> Self {
//         match value {
//             0 => Self::Undecided(false),
//             1 => Self::Undecided(true),
//             3 => Self::Empty,
//             _ => panic!(),
//         }
//     }
// }

// impl VariableState {
//     const fn is_empty(&self) -> bool {
//         matches!(self, Self::Empty)
//     }

//     const fn is_discarded(&self) -> bool {
//         matches!(self, Self::Discarded)
//     }

//     const fn is_decided(&self) -> bool {
//         matches!(self, Self::Decided(_))
//     }

//     const fn is_undecided(&self) -> bool {
//         matches!(self, Self::Undecided(_))
//     }

//     fn set_decided(&self) -> Self {
//         if let Self::Undecided(b) = self {
//             Self::Decided(*b)
//         } else {
//             *self
//         }
//     }

//     fn conflicts_with(&self, other: &Self) -> bool {
//         if let Self::Undecided(l) = self {
//             if let Self::Undecided(r) = other {
//                 return l != r;
//             }
//         }

//         false
//     }

//     fn evaluate(&self, other: &Self) -> Self {
//         if self.is_decided() {
//             return *self;
//         }

//         if self.is_empty() {
//             return *other;
//         }

//         if self.is_discarded() {
//             return VariableState::Discarded;
//         }

//         if self.conflicts_with(other) {
//             return VariableState::Discarded;
//         }

//         if self == other {
//             return *self;
//         }

//         if self.is_undecided() {
//             return *self;
//         }

//         panic!("Should've exhausted all. {:?} {:?}", self, other);
//     }
// }

// // pub struct Formula {}

// // impl Formula {
// //     pub fn solve(&self) -> bool {
// //         let mut clauses = self.clauses.clone();
// //         let mut evaluation_state = clauses.first().unwrap().0.clone();

// //         // this should actually start from idx 1, but clauses[1..] seems to break it so i cant be bothered now, its just a redundant check it shouldn't affect actual result`
// //         for clause in clauses {
// //             let clause = clause.0;
// //             let curr_eval_state = evaluation_state.clone();
// //             let temp_state: Vec<VariableState> = evaluation_state
// //                 .iter()
// //                 .zip(clause.iter())
// //                 .map(|(state, clause)| state.evaluate(clause))
// //                 .collect();

// //             println!("Current Evaluation State: {:?}", evaluation_state);
// //             println!("          Current Clause: {:?}", clause);
// //             println!("  Temp Evaluation Result: {:?}", temp_state);

// //             if temp_state.iter().all(|c| c.is_discarded()) {
// //                 evaluation_state
// //                     .iter_mut()
// //                     .for_each(|c| *c = c.set_decided());

// //                 let replace_idx = evaluation_state
// //                     .iter()
// //                     .zip(clause.iter())
// //                     .enumerate()
// //                     .find(|(_, (eval, curr_clause))| {
// //                         eval.is_discarded() && curr_clause.is_undecided()
// //                     })
// //                     .map(|(idx, _)| idx);

// //                 if replace_idx.is_none() {
// //                     println!("False. Final Evaluation: {:?}", evaluation_state);
// //                     return false;
// //                 }

// //                 let replace_idx = replace_idx.unwrap();

// //                 evaluation_state[replace_idx] = clause[replace_idx];
// //             } else {
// //                 evaluation_state = temp_state;
// //             }
// //         }

// //         println!("True. Final Evaluation: {:?}", evaluation_state);

// //         true
// //     }
// // }

#[derive(Debug, PartialEq, Eq)]
pub enum SolverState {
    Running,
    Halted,
}

// Solver assumes the input is sorted by length
pub struct Solver {
    // current_state: Vec<VariableState>,
    influence: FxHashMap<isize, usize>,
    used: HashSet<isize>,
}

impl Solver {
    pub fn new(influence: FxHashMap<isize, usize>) -> Self {
        Self {
            influence,
            used: HashSet::default(),
        }
    }

    pub fn add_clause(&mut self, clause: InputClause) -> SolverState {
        println!("Clause: {:?}", clause);
        println!("Influence: {:?}", self.influence);
        println!("Used: {:?}", self.used);

        // let mut candidate_heap: BinaryHeap<(usize, Reverse<usize>, isize)> = BinaryHeap::new();
        let mut candidate_heap: BinaryHeap<(Reverse<usize>, usize, isize)> = BinaryHeap::new();
        let mut clause_handled = false;
        clause.iter().for_each(|var| {
            if !self.used.contains(&-var) {
                // candidate_heap.push((
                //     *self.influence.get(var).unwrap(),
                //     Reverse(*self.influence.get(&-var).unwrap_or(&usize::MAX)),
                //     *var,
                // ));
                candidate_heap.push((
                    Reverse(*self.influence.get(&-var).unwrap_or(&usize::MAX)),
                    *self.influence.get(var).unwrap(),
                    *var,
                ));
            }
            self.influence.entry(*var).and_modify(|v| *v -= 1);
            if self.used.contains(var) {
                clause_handled = true;
            }
        });

        println!("Candidates: {:?}", candidate_heap);
        println!("Is Handled: {}", clause_handled);

        if clause_handled {
            println!();
            return SolverState::Running;
        }

        let s = candidate_heap.pop();
        println!("Selected: {:?}", s);
        println!();
        let selected = if let Some(selected) = s {
            selected
        } else {
            // this means that there are no good candidates left, as their counterpart has all been used up
            return SolverState::Halted;
        };

        let (_, _, variable) = selected;
        self.used.insert(variable);

        // if candidate_heap.is_empty() {
        //     return SolverState::Halted;
        // }

        SolverState::Running

        // println!("CURRENT STATE: {:?}", self.current_state);
        // println!("INPUT: {:?}", clause);
        // clause
        //     .iter()
        //     .for_each(|v| println!("VAR IDX {} {:?}", v.idx, self.current_state[v.idx]));

        // let mut temp_state = self.current_state.clone();

        // clause.iter().for_each(|v| {
        //     if temp_state[v.idx].is_empty() {
        //         temp_state[v.idx] = VariableState::Undecided(v.nonnegated);
        //     }

        //     if let VariableState::Undecided(l) = temp_state[v.idx] {
        //         if l != v.nonnegated {
        //             temp_state[v.idx] = VariableState::Discarded;
        //         }
        //     }
        // });

        // if temp_state.iter().any(|v| v.is_undecided()) {
        //     self.current_state = temp_state;
        //     SolverState::Running
        // } else {
        //     // find a variable that is VariableState::Discarded, but also in
        //     // clause. this can be optimized by using some heuristics but for
        //     // now this implementation wont use any
        //     self.current_state
        //         .iter_mut()
        //         .for_each(|v| *v = v.set_decided());

        //     let var_revive = clause
        //         .iter()
        //         .find(|v| self.current_state[v.idx].is_discarded());

        //     match var_revive {
        //         Some(v) => {
        //             self.current_state[v.idx] = VariableState::Undecided(v.nonnegated);
        //             SolverState::Running
        //         }
        //         None => SolverState::Halted,
        //     }
        // }
    }
}
