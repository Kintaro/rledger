use std::rc::Rc;
use chrono::{ Date, Duration, Local };
use regex::Regex;

use posting::ClearedStatus;
use quantity::Quantity;

#[derive(Clone, PartialEq, Eq)]
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
    Amount(Quantity),
    Symbol(Regex),
    Empty(bool),
    Depth(usize),
    Tag(Regex, Option<Regex>)
}

#[derive(Clone, PartialEq, Eq)]
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

pub fn matches_account(query: &Query, account_name: String) -> bool {
    match query {
        &Query::None => false,
        &Query::Not(ref x) => matches_account(x.as_ref(), account_name),
        &Query::Or(ref xs) => xs.iter().any(|x| matches_account(x, account_name.clone())),
        &Query::And(ref xs) => xs.iter().all(|x| matches_account(x, account_name.clone())),
        &Query::Depth(d) => account_name_level(account_name) <= d,
        &Query::Tag(_, _) => false,
        _ => true,
    }
}
