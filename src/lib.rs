use cucumber::World;
use thirtyfour::prelude::{By, WebDriver, WebDriverResult};

#[derive(Debug, Default)]
pub struct Cost {
    pub amount: String,
    pub unit: Unit,
}

impl Cost {
    pub fn new(amount: &str, unit: Unit) -> Self {
        Self {
            amount: amount.into(),
            unit,
        }
    }
}

#[derive(Debug, Default)]
pub struct Date {
    pub month: Month,
    pub year: String,
}

impl Date {
    pub fn new(month: Month, year: &str) -> Self {
        Self {
            month,
            year: year.into(),
        }
    }
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
#[repr(u8)]
pub enum Month {
    #[default]
    Jan = 1,
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
    pub driver: Option<WebDriver>,
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
    pub async fn read(driver: &WebDriver) -> WebDriverResult<Results> {
        Ok(Results {
            house_price: driver
                .find(By::XPath(
                    r#"//*[@id="content"]/div[5]/table/tbody/tr/td/table/tbody/tr[2]/td[2]/b"#,
                ))
                .await?
                .value()
                .await?
                .unwrap_or_default(),
            loan_amount: driver
                .find(By::XPath(
                    r#"//*[@id="content"]/div[4]/table/tbody/tr/td/table/tbody/tr[3]/td[2]"#,
                ))
                .await?
                .value()
                .await?
                .unwrap_or_default(),
            down_payment: driver
                .find(By::XPath(
                    r#"//*[@id="content"]/div[4]/table/tbody/tr/td/table/tbody/tr[4]/td[2]"#,
                ))
                .await?
                .value()
                .await?
                .unwrap_or_default(),
            total_payments: driver
                .find(By::XPath(
                    r#"//*[@id="content"]/div[4]/table/tbody/tr/td/table/tbody/tr[5]/td[2]"#,
                ))
                .await?
                .value()
                .await?
                .unwrap_or_default(),
            total_interest: driver
                .find(By::XPath(
                    r#"//*[@id="content"]/div[4]/table/tbody/tr/td/table/tbody/tr[6]/td[2]"#,
                ))
                .await?
                .value()
                .await?
                .unwrap_or_default(),
            payoff_date: driver
                .find(By::XPath(
                    r#"//*[@id="content"]/div[4]/table/tbody/tr/td/table/tbody/tr[7]/td[2]"#,
                ))
                .await?
                .value()
                .await?
                .unwrap_or_default(),
        })
    }
}

#[derive(Debug, Default)]
pub enum Unit {
    #[default]
    Dollars,
    Percent,
}

#[cucumber::when("input is submitted")]
async fn submit(world: &mut MortgageCalculatorWorld) -> WebDriverResult<()> {
    if let Some(driver) = &world.driver {
        let start_month = &raw const world.input.start_date.month;

        driver
            .goto("https://www.calculator.net/mortgage-calculator.html")
            .await?;
        driver
            .find(By::Name("chouseprice"))
            .await?
            .send_keys(&world.input.home_price)
            .await?;
        driver
            .find(By::Name("cdownpayment"))
            .await?
            .send_keys(&world.input.down_payment.amount)
            .await?;
        driver
            .find(By::Name("cdownpaymentunit"))
            .await?
            .send_keys(match world.input.down_payment.unit {
                Unit::Dollars => "d",
                Unit::Percent => "p",
            })
            .await?;
        driver
            .find(By::Name("cloanterm"))
            .await?
            .send_keys(&world.input.loan_term)
            .await?;
        driver
            .find(By::Name("cinterestrate"))
            .await?
            .send_keys(&world.input.interest_rate)
            .await?;
        driver
            .find(By::Name("cstartmonth"))
            .await?
            .send_keys(format!("{}", start_month as u8))
            .await?;
        driver
            .find(By::Name("cstartyear"))
            .await?
            .send_keys(&world.input.start_date.year)
            .await?;
        driver.find(By::Name("caddoptional")).await?.click().await?;
        driver.find(By::Name("x")).await?.click().await?;
    }

    Ok(())
}
