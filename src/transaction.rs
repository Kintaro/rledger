use posting::ClearedStatus;
use posting::Posting;

#[derive(Clone, PartialEq, Eq)]
struct GenericSourcePos(String, usize, usize);

#[derive(Clone, PartialEq, Eq)]
pub struct Transaction {
    index: usize,
    source_pos: GenericSourcePos,
    //date: Day,
    //date2: Option<Day>,
    status: ClearedStatus,
    code: String,
    description: String,
    comment: String,
    //tags: Vec<Tag>,
    pub postings: Vec<Posting>,
    preceding_comment_lines: String
}
