use mltt_span::FileSpan;

/// Metavariable index.
///
/// These are used as placeholders for undetermined terms that we will need to
/// eventually fill in during elaboration. They can also be used to stand for
/// 'holes' in the concrete syntax, to support type-directed editing.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index(pub u32);

impl From<u32> for Index {
    fn from(src: u32) -> Index {
        Index(src)
    }
}

/// An entry in the metavariable environment.
#[derive(Debug, Clone, PartialEq)]
pub enum Solution<Solved> {
    Unsolved,
    Solved(Solved),
}

/// An environment of solved and unsolved metavariables.
#[derive(Debug, Clone, PartialEq)]
pub struct Env<Solved> {
    /// The solutions.
    solutions: Vec<(FileSpan, Solution<Solved>)>,
}

impl<Solved> Env<Solved> {
    /// Create a new, empty environment.
    pub fn new() -> Env<Solved> {
        Env {
            solutions: Vec::new(),
        }
    }

    /// Lookup a the solution for a metavariable in the environment.
    pub fn lookup_solution(&self, index: Index) -> Option<&(FileSpan, Solution<Solved>)> {
        self.solutions.get(index.0 as usize)
    }

    /// Add a solution to the given metavariable index.
    pub fn add_solved(&mut self, index: Index, solved: Solved) {
        match self.solutions.get_mut(index.0 as usize) {
            Some((_, solution @ Solution::Unsolved)) => *solution = Solution::Solved(solved),
            Some((_, Solution::Solved(_))) => unimplemented!("updating solved solution"),
            None => unimplemented!("no corresponding solution"),
        }
    }

    /// Create a fresh metavariable index.
    pub fn add_unsolved(&mut self, span: FileSpan) -> Index {
        let index = Index(self.solutions.len() as u32);
        self.solutions.push((span, Solution::Unsolved));
        index
    }
}
