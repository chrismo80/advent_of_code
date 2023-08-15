use serde::Deserialize;

pub fn main()
{
    let tickers = vec!["UPRO", "TMF", "18MF.DE", "IS04.DE"];

    for ticker in tickers {
        println!("{}: {:0.2}", ticker, get_price(ticker));
    }
}

fn get_price(stock: &str) -> f64
{
    let stock_info: Stock = reqwest::blocking::get(format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/?symbol={stock}&range=1d&interval=1d"
    ))
    .unwrap()
    .json()
    .unwrap();

    stock_info.chart.result[0].indicators.quote[0].close[0]
}

#[derive(Deserialize, Debug)]
struct Stock
{
    chart: Chart,
}

#[derive(Deserialize, Debug)]
struct Chart
{
    result: Vec<Result>,
}

#[derive(Deserialize, Debug)]
struct Result
{
    meta: Meta,
    timestamp: Vec<i64>,
    indicators: Indicators,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Meta
{
    currency: String,
    symbol: String,
    exchangeName: String,
    instrumentType: String,
    firstTradeDate: i64,
    regularMarketTime: i64,
    gmtoffset: i64,
    timezone: String,
    exchangeTimezoneName: String,
    regularMarketPrice: f64,
    chartPreviousClose: f64,
    priceHint: i64,
    currentTradingPeriod: CurrentTradingPeriod,
    dataGranularity: String,
    range: String,
    validRanges: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CurrentTradingPeriod
{
    pre: TradingPeriod,
    regular: TradingPeriod,
    post: TradingPeriod,
}

#[derive(Deserialize, Debug)]
struct TradingPeriod
{
    timezone: String,
    start: i64,
    end: i64,
    gmtoffset: i64,
}

#[derive(Deserialize, Debug)]
struct Indicators
{
    quote: Vec<Quote>,
    adjclose: Vec<AdjClose>,
}

#[derive(Deserialize, Debug)]
struct Quote
{
    open: Vec<f64>,
    volume: Vec<i64>,
    low: Vec<f64>,
    close: Vec<f64>,
    high: Vec<f64>,
}

#[derive(Deserialize, Debug)]
struct AdjClose
{
    adjclose: Vec<f64>,
}
