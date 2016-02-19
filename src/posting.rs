use amount::MixedAmount;
use transaction::Transaction;

#[derive(Clone, PartialEq, Eq)]
pub enum ClearedStatus {
    Uncleared,
    Pending,
    Cleared
}

#[derive(Clone, PartialEq, Eq)]
pub enum PostingType {
    Regular,
    Virtual,
    BalancedVirtual
}

#[derive(Clone, PartialEq, Eq)]
pub struct Posting {
    status: ClearedStatus,
    amount: MixedAmount,
    posting_type: PostingType,
    balance_assertion: Option<MixedAmount>,
    transaction: Option<Transaction>
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
}
