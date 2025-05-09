use cucumber::World;
use mortgage_calculator_test_suite::{
    Cost, Date, Input, Month, MortgageCalculatorWorld, Results, Unit,
};

#[tokio::main]
async fn main() {
    MortgageCalculatorWorld::run("tests/mortgage_calculator.feature").await;
}

#[cucumber::given("default input data")]
fn default_input_data(world: &mut MortgageCalculatorWorld) {
    world.input = Input {
        home_price: "400000".into(),
        down_payment: Cost {
            amount: "20".into(),
            unit: Unit::Percent,
        },
        loan_term: "30".into(),
        interest_rate: "6.652".into(),
        start_date: Date {
            month: Month::May,
            year: "2025".into(),
        },
    };
}

#[cucumber::then("results are correct")]
fn results_are_correct(world: &mut MortgageCalculatorWorld) {
    world.results.read();
    assert_eq!(
        world.results,
        Results {
            house_price: "$400,000.00".into(),
            loan_amount: "$320,000.00".into(),
            down_payment: "$80,000.00".into(),
            total_payments: "$739,696.04".into(),
            total_interest: "$419,696.04".into(),
            payoff_date: "May. 2055".into(),
        }
    );
}
