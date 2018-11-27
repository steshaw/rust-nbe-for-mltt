//! The semantic domain

use im;
use std::rc::Rc;

use syntax::core::RcTerm;
use syntax::{DbLevel, IdentHint, UniverseLevel};

pub type Env = im::Vector<RcValue>;

/// A closure that binds a variable
#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub term: RcTerm,
    pub env: Env,
}

impl Closure {
    pub fn new(term: RcTerm, env: Env) -> Closure {
        Closure { term, env }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RcValue {
    pub inner: Rc<Value>,
}

impl From<Value> for RcValue {
    fn from(src: Value) -> RcValue {
        RcValue {
            inner: Rc::new(src),
        }
    }
}

impl RcValue {
    /// Construct a variable
    pub fn var(level: impl Into<DbLevel>, ann: impl Into<RcValue>) -> RcValue {
        RcValue::from(Value::var(level, ann))
    }
}

/// Terms that are in _weak head normal form_
///
/// These can either be _neutral values_ (values that are stuck on a variable),
/// or _canonical values_.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Neutral values, annotated with a type
    Neutral(RcNeutral, RcType),

    /// Dependent function types
    FunType(IdentHint, RcType, Closure),
    /// Introduce a function
    FunIntro(IdentHint, Closure),

    /// Dependent pair types
    PairType(IdentHint, RcType, Closure),
    /// Introduce a pair
    PairIntro(RcValue, RcValue),

    /// Universe of types
    Universe(UniverseLevel),
}

impl Value {
    /// Construct a variable
    pub fn var(level: impl Into<DbLevel>, ann: impl Into<RcValue>) -> Value {
        Value::Neutral(RcNeutral::from(Neutral::Var(level.into())), ann.into())
    }
}

/// Alias for types - we are using describing a dependently typed language
/// types, so this is just an alias
pub type Type = Value;

/// Alias for reference counted types - we are using describing a dependently
/// typed language types, so this is just an alias
pub type RcType = RcValue;

#[derive(Debug, Clone, PartialEq)]
pub struct RcNeutral {
    pub inner: Rc<Neutral>,
}

impl From<Neutral> for RcNeutral {
    fn from(src: Neutral) -> RcNeutral {
        RcNeutral {
            inner: Rc::new(src),
        }
    }
}

/// Terms for which computation has stopped because of an attempt to evaluate a
/// variable
///
/// These are known as _neutral values_ or _accumulators_.
#[derive(Debug, Clone, PartialEq)]
pub enum Neutral {
    /// Variables
    Var(DbLevel),

    /// Apply a function to an argument
    ///
    /// We annotate the argument with a type with a so that we can eta-expand
    /// it appropriately during readback
    FunApp(RcNeutral, RcValue, RcType),

    /// Project the first element of a pair
    PairFst(RcNeutral),
    /// Project the second element of a pair
    PairSnd(RcNeutral),
}
