use serde::Deserialize;

pub fn main()
{
    let exchange_rate = get_price("EUR=X").0;

    let tickers = vec!["UPRO", "TMF", "18MF.DE", "IS04.DE", "VGVF.DE", "XMLC.DE", "WRLD.DE"];

    for ticker in &tickers {
        println!("{:>7}: {:>8.2} €", ticker, get_price_euro(ticker, exchange_rate));
    }
}

fn get_price_euro(ticker: &str, exchange_rate: f64) -> f64
{
    let price = get_price(ticker);

    match price.1.as_str() {
        "USD" => price.0 * exchange_rate,
        _ => price.0,
    }
}

fn get_price(ticker: &str) -> (f64, String)
{
    let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{ticker}?range=1d&interval=1d");

    let info: Info = reqwest::blocking::get(url).unwrap().json().unwrap();

    let currency = info.chart.result[0].meta.currency.parse().unwrap();
    let price = info.chart.result[0].indicators.adjclose[0].adjclose[0];

    (price, currency)
}

#[derive(Deserialize, Debug)]
struct Info
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
