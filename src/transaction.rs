use posting::ClearedStatus;
use posting::Posting;
use posting::Tag;
use chrono::Date;
use chrono::Local;

#[derive(Clone, PartialEq, Eq)]
pub struct GenericSourcePos(String, usize, usize);

#[derive(Clone, PartialEq, Eq)]
pub struct Transaction {
    pub index: usize,
    pub source_pos: GenericSourcePos,
    pub date: Date<Local>,
    pub date2: Option<Date<Local>>,
    pub status: ClearedStatus,
    pub code: String,
    pub description: String,
    pub comment: String,
    pub tags: Vec<Tag>,
    pub postings: Vec<Posting>,
    pub preceding_comment_lines: String
}

impl Transaction {
    pub fn all_tags(&self) -> Vec<Tag> {
        self.tags.iter().chain(self.postings.iter().flat_map(|x| x.tags.iter())).map(|x| x.clone()).collect()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ModifierTransaction {
    pub value_expression: String,
    pub postings: Vec<Posting>
}

#[derive(Clone, PartialEq, Eq)]
pub struct PeriodicTransaction {
    pub periodic_expression: String,
    pub postings: Vec<Posting>
}
