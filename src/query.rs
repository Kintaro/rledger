use std::rc::Rc;
use chrono::{ Date, Duration, Local };
use regex::Regex;

use posting::ClearedStatus;
use quantity::Quantity;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OrdPlus {
    Lt,
    LtEq,
    Gt,
    GtEq,
    Eq,
    AbsLt,
    AbsLtEq,
    AbsGt,
    AbsGtEq,
    AbsEq
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Query {
    Any,
    None,
    Not(Rc<Query>),
    Or(Vec<Query>),
    And(Vec<Query>),
    Code(Regex),
    Desc(Regex),
    Acct(Regex),
    Date(Duration),
    Date2(Duration),
    Status(ClearedStatus),
    Real(bool),
    Amount(OrdPlus, Quantity),
    Symbol(Regex),
    Empty(bool),
    Depth(usize),
    Tag(Regex, Option<Regex>)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum QueryOption {
    InAccountOnly(String),
    InAccount(String)
}

pub fn parse_query(day: Date<Local>, query: String) -> (Query, Vec<QueryOption>) {
    (Query::None, Vec::new())
}

pub fn in_account(query_opts: &Vec<QueryOption>) -> Option<(String, bool)> {
    for opt in query_opts {
        match opt {
            &QueryOption::InAccountOnly(ref a) => return Some((a.clone(), false)),
            &QueryOption::InAccount(ref a) => return Some((a.clone(), true))
        }
    }

    None
}

pub fn account_name_level(account_name: String) -> usize {
    return 0;
}

pub fn same<T: Eq>(vec: &Vec<T>) -> bool {
    vec.iter().all(|x| vec.first().unwrap() == x)
}

impl Query {
    pub fn matches_account(&self, account_name: String) -> bool {
        match *self {
            Query::None => false,
            Query::Not(ref x) => Query::matches_account(x.as_ref(), account_name),
            Query::Or(ref xs) => xs.iter().any(|x|  Query::matches_account(x, account_name.clone())),
            Query::And(ref xs) => xs.iter().all(|x| Query::matches_account(x, account_name.clone())),
            Query::Depth(d) => account_name_level(account_name) <= d,
            Query::Tag(_, _) => false,
            _ => true,
        }
    }

    fn simplify_helper_and(xs: &Vec<Query>) -> Query {
        match xs.as_slice() {
            [] => Query::Any,
            [ref q] => Query::simplify_helper(q),
            q => {
                if same(xs) {
                    xs.first().unwrap().simplify()
                } else if q.iter().any(|x| *x == Query::None) {
                    Query::None
                } else if q.iter().all(|x| true) {
                    Query::None
                } else {
                    Query::None
                }
            }
        }
    }

    fn simplify_helper_or(xs: &Vec<Query>) -> Query {
        match xs.as_slice() {
            [] => Query::Any,
            [ref q] => Query::simplify_helper(q),
            q => {
                if same(xs) {
                    xs.first().unwrap().simplify()
                } else if q.iter().any(|x| *x == Query::Any) {
                    Query::None
                } else {
                    Query::Or(q.iter().filter(|&x| *x != Query::None).map(|x| x.simplify()).collect())
                }
            }
        }
    }

    fn simplify_helper(query: &Query) -> Query {
        match query {
            &Query::And(ref xs) => Query::simplify_helper_and(xs),
            &Query::Or(ref xs) => Query::simplify_helper_or(xs),
            q => q.clone()
        }
    }

    /// Simplify the query as much as possible
    pub fn simplify(&self) -> Query {
        let q = Query::simplify_helper(self);
        if *self == q {
            self.clone()
        } else {
            q.simplify()
        }
    }

    pub fn filter<F>(&self, pred: F) -> Query where F : Fn(&Query) -> bool {
        match self {
            &Query::And(ref qs) => Query::And(qs.iter().filter(|&x| pred(x)).map(|x| x.clone()).collect()),
            &Query::Or(ref qs) => Query::Or(qs.iter().filter(|&x| pred(x)).map(|x| x.clone()).collect()),
            _ => if pred(self) { self.clone() } else { Query::Any }
        }.simplify()
    }

    pub fn is_null(&self) -> bool {
        match self {
            &Query::Any => true,
            &Query::And(ref q) => q.is_empty(),
            &Query::Not(ref r) => match r.as_ref() {
                &Query::Or(ref q) => q.is_empty(),
                _ => false
            },
            _ => false
        }
    }

    pub fn is_depth(&self) -> bool {
        match self {
            &Query::Depth(_) => true,
            _ => false
        }
    }

    pub fn depth(&self) -> usize {
        fn depth_helper(query: &Query) -> Vec<usize> {

            match query {
                &Query::Depth(d) => vec!(d),
                &Query::Or(ref q) => q.iter().flat_map(|x| depth_helper(x)).collect(),
                &Query::And(ref q) => q.iter().flat_map(|x| depth_helper(x)).collect(),
                _ => Vec::new()
            }
        }

        let q = depth_helper(self);
        let r = q.iter().min();

        match r {
            None => 99999,
            Some(x) => *x
        }
    }
}

#[test]
fn simplify_test() {
    let q = Query::And(vec!(Query::Or(vec!(Query::Or(vec!(Query::Desc(Regex::new("b b").unwrap())))))));
    let sq = q.simplify();

    assert!(sq == Query::Desc(Regex::new("b b").unwrap()))
}
