use transaction::{ Transaction, ModifierTransaction, PeriodicTransaction };
use amount::Amount;
use commodity::Commodity;
use chrono::{ Date, Local };
use posting::Posting;

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

impl JournalContext {
    pub fn new() -> JournalContext {
        JournalContext {
            year: None
        }
    }
}

/// Represents the complete journal
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

impl Journal {
    /// Creates a new empty journal
    pub fn new() -> Journal {
        Journal {
            modifier_transactions: Vec::new(),
            periodic_transactions: Vec::new(),
            transactions: Vec::new(),
            market_prices: Vec::new(),
            final_comment_lines: String::from(""),
            context: JournalContext::new()
        }
    }

    /// Adds a new transaction to the journal
    pub fn add_transaction_mut(&mut self, transaction: Transaction) {
        self.transactions.push(transaction)
    }

    /// Adds a new transaction to the journal and returns a copy
    pub fn add_transaction(&self, transaction: Transaction) -> Journal {
        self.add_item(|s, x| s.add_transaction_mut(x), transaction)
    }

    /// Adds a new modifier transaction to the journal
    pub fn add_modifier_transaction_mut(&mut self, modifier_transaction: ModifierTransaction) {
        self.modifier_transactions.push(modifier_transaction);
    }

    /// Adds a new modifier transaction to the journal and returns a copy
    pub fn add_modifier_transaction(&mut self, modifier_transaction: ModifierTransaction) -> Journal {
        self.add_item(|s, x| s.add_modifier_transaction_mut(x), modifier_transaction)
    }

    /// Adds a new market price to the journal
    pub fn add_market_price_mut(&mut self, market_price: MarketPrice) {
        self.market_prices.push(market_price);
    }

    /// Adds a new market price to the journal and returns a copy
    pub fn add_market_price(&self, market_price: MarketPrice) -> Journal {
        self.add_item(|s, x| s.add_market_price_mut(x), market_price)
    }

    /// Applies the given function to the journal to adds new items
    pub fn add_item<T, F>(&self, f: F, item: T) -> Journal where F : FnOnce(&mut Journal, T) {
        let mut x = self.clone();
        f(&mut x, item);
        x
    }

    /// Returns the transaction at the given index, if it is available
    pub fn transaction_at(&self, pos: usize) -> Option<&Transaction> {
        self.transactions.get(pos)
    }

    ///
    pub fn next_transaction(&self, transaction: &Transaction) -> Option<&Transaction> {
        self.transaction_at(transaction.index)
    }

    /// Collects all postings from all transactions and flattens it into a list
    pub fn postings(&self) -> Vec<Posting> {
        self.transactions.iter().flat_map(|x| x.postings.iter()).map(|x| x.clone()).collect()
    }
}
