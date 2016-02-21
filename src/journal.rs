use transaction::{ Transaction, ModifierTransaction, PeriodicTransaction };
use amount::Amount;
use commodity::Commodity;
use chrono::{ Date, Local };

#[derive(Clone, PartialEq, Eq)]
pub struct MarketPrice {
    pub date: Date<Local>,
    pub commodity: Commodity,
    pub amount: Amount
}

#[derive(Clone, PartialEq, Eq)]
pub struct Year(usize);

#[derive(Clone, PartialEq, Eq)]
pub struct JournalContext {
    year: Option<Year>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Journal {
    pub modifier_transactions: Vec<ModifierTransaction>,
    pub periodic_transactions: Vec<PeriodicTransaction>,
    pub transactions: Vec<Transaction>,
    //pub open_timelog_entries: Vec<TimeLogEntry>,
    pub market_prices: Vec<MarketPrice>,
    pub final_comment_lines: String,
    pub context: JournalContext,
}
