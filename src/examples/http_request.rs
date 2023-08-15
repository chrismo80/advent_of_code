use serde::Deserialize;

pub fn main()
{
    let exchange_rate = get_price("EUR=X");

    let tickers = vec!["UPRO", "TMF", "18MF.DE", "IS04.DE", "VGVF.DE", "XMLC.DE", "WRLD.DE"];

    for ticker in &tickers {
        let price = get_price_euro(ticker, exchange_rate.0);

        println!("{:>7}: {:>8.2} €", ticker, price);
    }
}

fn get_price_euro(stock: &str, exchange_rate: f64) -> f64
{
    let stock = get_price(stock);

    match stock.1.as_str() {
        "USD" => stock.0 * exchange_rate,
        _ => stock.0,
    }
}

fn get_price(stock: &str) -> (f64, String)
{
    let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{stock}?range=1d&interval=1d");

    let stock_info: Stock = reqwest::blocking::get(url).unwrap().json().unwrap();

    let currency = stock_info.chart.result[0].meta.currency.parse().unwrap();
    let price = stock_info.chart.result[0].indicators.adjclose[0].adjclose[0];

    (price, currency)
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
