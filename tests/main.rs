use cucumber::World;
use mortgage_calculator_test_suite::{
    Cost, Date, Input, Month, MortgageCalculatorWorld, Results, Unit,
};
use thirtyfour::error::WebDriverResult;

#[tokio::main]
async fn main() {
    MortgageCalculatorWorld::run("tests/mortgage_calculator.feature").await;
}

#[cucumber::given("default input data")]
fn default_input_data(world: &mut MortgageCalculatorWorld) {
    world.input = Input {
        home_price: "400000".into(),
        down_payment: Cost::new("20", Unit::Percent),
        loan_term: "30".into(),
        interest_rate: "6.652".into(),
        start_date: Date::new(Month::May, "2025"),
    };
}

#[cucumber::then("results are correct")]
async fn results_are_correct(world: &mut MortgageCalculatorWorld) -> WebDriverResult<()> {
    if let Some(driver) = &world.driver {
        assert_eq!(
            Results::read(&driver).await?,
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

    Ok(())
}
