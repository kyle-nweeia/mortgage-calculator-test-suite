use cucumber::World;

#[derive(Debug, Default)]
pub struct Cost {
    pub amount: String,
    pub unit: Unit,
}

#[derive(Debug, Default)]
pub struct Date {
    pub month: Month,
    pub year: String,
}

#[derive(Debug, Default)]
pub struct Input {
    pub home_price: String,
    pub down_payment: Cost,
    pub loan_term: String,
    pub interest_rate: String,
    pub start_date: Date,
}

#[derive(Debug, Default)]
pub enum Month {
    #[default]
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

#[derive(Debug, Default, World)]
pub struct MortgageCalculatorWorld {
    pub input: Input,
    pub results: Results,
}

#[derive(Debug, Default, PartialEq)]
pub struct Results {
    pub house_price: String,
    pub loan_amount: String,
    pub down_payment: String,
    pub total_payments: String,
    pub total_interest: String,
    pub payoff_date: String,
}

impl Results {
    pub fn read(&self) {}
}

#[derive(Debug, Default)]
pub enum Unit {
    #[default]
    Dollars,
    Percent,
}

#[cucumber::when("input is submitted")]
fn submit(world: &mut MortgageCalculatorWorld) {}
