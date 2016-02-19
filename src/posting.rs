use amount::MixedAmount;
use transaction::Transaction;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::result::Result;

#[derive(Clone, PartialEq, Eq)]
pub enum ClearedStatus {
    Uncleared,
    Pending,
    Cleared
}

impl Display for ClearedStatus {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &ClearedStatus::Uncleared => write!(f, ""),
            &ClearedStatus::Pending => write!(f, "!"),
            &ClearedStatus::Cleared => write!(f, "*"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Tag(String, String);

#[derive(Clone, PartialEq, Eq)]
pub enum PostingType {
    Regular,
    Virtual,
    BalancedVirtual
}

#[derive(Clone, PartialEq, Eq)]
pub struct Posting {
    pub status: ClearedStatus,
    pub amount: MixedAmount,
    pub posting_type: PostingType,
    pub tags: Vec<Tag>,
    pub balance_assertion: Option<MixedAmount>,
    pub transaction: Option<Transaction>
}

impl Posting {
    pub fn is_real(&self) -> bool {
        self.posting_type == PostingType::Regular
    }

    pub fn is_virtual(&self) -> bool {
        self.posting_type == PostingType::Virtual
    }

    pub fn related_postings(&self) -> Vec<Posting> {
        match self.transaction.clone() {
            Some(t) => t.postings.iter().filter(|&x| x != self).map(|x| x.clone()).collect(),
            _ => vec!()
        }
    }

    pub fn sum_postings(postings: Vec<Posting>) -> MixedAmount {
        postings.iter().map(|x| x.clone().amount).sum()
    }

    pub fn status(&self) -> ClearedStatus {
        match self.status.clone() {
            ClearedStatus::Uncleared => match self.transaction.clone() {
                Some(t) => t.status,
                _ => ClearedStatus::Uncleared
            },
            s => s
        }
    }

    pub fn all_tags(&self) -> Vec<Tag> {
        self.tags.iter().chain(match self.transaction.clone() {
            Some(t) => t.tags,
            _ => vec!()
        }.iter()).map(|x| x.clone()).collect()
    }
}
