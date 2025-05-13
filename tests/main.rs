use cucumber::World;
use mortgage_calculator_test_suite::{
    Input, Month, MortgageCalculatorError, MortgageCalculatorWorld, Results, Unit,
};
use thirtyfour::error::WebDriverResult;

#[tokio::main]
async fn main() {
    MortgageCalculatorWorld::run("tests/mortgage_calculator.feature").await;
}

#[cucumber::given("default input data")]
async fn default_input_data(world: &mut MortgageCalculatorWorld) -> WebDriverResult<()> {
    world.input = Input::default()
        .home_price("400000")
        .down_payment("20", Unit::Percent)
        .loan_term("30")
        .interest_rate("6.652")
        .start_date(Month::May, "2025");

    Ok(())
}

#[cucumber::then("results are correct")]
async fn results_are_correct(world: &mut MortgageCalculatorWorld) -> WebDriverResult<()> {
    match &world.driver {
        Some(driver) => Ok(assert_eq!(
            Results::read(&driver).await?,
            Results::default()
                .house_price("$400,000.00")
                .loan_amount("$320,000.00")
                .down_payment("$80,000.00")
                .total_payments("$739,696.04")
                .total_interest("$419,696.04")
                .payoff_date("May. 2055")
        )),
        None => Err(MortgageCalculatorError::uninitialized_driver()),
    }
}
