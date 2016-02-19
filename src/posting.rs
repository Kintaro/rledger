use amount::MixedAmount;

#[derive(Clone, PartialEq, Eq)]
enum ClearedStatus {
    Uncleared,
    Pending,
    Cleared
}

#[derive(Clone, PartialEq, Eq)]
enum PostingType {
    Regular,
    Virtual,
    BalancedVirtual
}

#[derive(Clone, PartialEq, Eq)]
struct Posting {
    status: ClearedStatus,
    amount: MixedAmount,
    posting_type: PostingType,
    balance_assertion: Option<MixedAmount>
}

impl Posting {
    pub fn is_real(&self) -> bool {
        self.posting_type == PostingType::Regular
    }

    pub fn is_virtual(&self) -> bool {
        self.posting_type == PostingType::Virtual
    }

    pub fn related_postings(&self) -> Vec<Posting> {
        vec!()
    }

    pub fn sum_postings(postings: Vec<Posting>) -> MixedAmount {
        postings.iter().map(|x| x.amount).sum()
    }
}
