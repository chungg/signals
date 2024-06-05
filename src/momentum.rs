//! Momentum Indicators
//!
//! Provides technical indicators that measures the rate of change or speed of price
//! movement of a security. In the context of this library, these indicators are typically
//! range bound and used  alongside threshold(s) such as: zero lines, signal lines, etc...

use std::iter;

use itertools::izip;

use crate::smooth;
use crate::volatility::_true_range;

/// Relative strength index
///
/// Calculated by comparing the average gain of up days to the average loss of down days
/// over a specified period. Shows the magnitude of recent price changes to determine
/// overbought or oversold conditions.
///
/// # Source
///
/// https://www.investopedia.com/terms/r/rsi.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::rsi(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn rsi(data: &[f64], window: usize) -> impl Iterator<Item = f64> + '_ {
    let (gain, loss): (Vec<f64>, Vec<f64>) = data[1..]
        .iter()
        .zip(data[..data.len() - 1].iter())
        .map(|(curr, prev)| (f64::max(0.0, curr - prev), f64::min(0.0, curr - prev).abs()))
        .unzip();
    smooth::wilder(&gain, window)
        .zip(smooth::wilder(&loss, window))
        .map(|(g, l)| 100.0 * g / (g + l))
        .collect::<Vec<f64>>()
        .into_iter()
}

/// Moving average convergence/divergence
///
/// Shows the convergence and divergence of the two moving averages, indicating changes in
/// the strength and direction of the trend. When the MACD crosses above the signal line,
/// it's a bullish signal, indicating a potential uptrend.
///
/// # Source
///
/// https://www.investopedia.com/terms/m/macd.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::macd(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     3, 6).collect::<Vec<f64>>();
///
/// ```
pub fn macd(close: &[f64], short: usize, long: usize) -> impl Iterator<Item = f64> + '_ {
    let short_ma = smooth::ewma(close, short);
    let long_ma = smooth::ewma(close, long);
    short_ma.skip(long - short).zip(long_ma).map(|(x, y)| x - y)
}

/// Chande momentum oscillator
///
/// The CMO oscillates between +100 and -100, with high values indicating strong upward
/// momentum and low values indicating strong downward momentum. The indicator is
/// calculated by summing up the positive and negative price changes over a specified
/// period, then dividing the result by the sum of the absolute values of all price
/// changes over the same period.
///
/// # Source
///
/// https://www.investopedia.com/terms/c/chandemomentumoscillator.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::cmo(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn cmo(data: &[f64], window: usize) -> impl Iterator<Item = f64> + '_ {
    smooth::_cmo(data, window).map(|x| x * 100.0)
}

/// Chande Forecast Oscillator
///
/// An indicator that attempts to forecast the future direction of the market by
/// analyzing the relationship between the current price and the price a certain number
/// of periods ago.
///
/// The resulting value is then multiplied by 100 to create an oscillator that ranges
/// from -100 to +100.
///
/// # Source
///
/// https://www.stockmaniacs.net/chande-forecast-oscillator/
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::cfo(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn cfo(data: &[f64], window: usize) -> impl Iterator<Item = f64> + '_ {
    smooth::lrf(data, window)
        .zip(data.iter().skip(window - 1))
        .map(|(tsf, x)| 100.0 * (x - tsf) / x)
}

/// Elder ray
///
/// The Elder Ray Index consists of two components:
///
/// Bull Power
///   - This measures the ability of buyers to push the price up.
///   - It's calculated by subtracting the EWMA from the high price.
///
/// Bear Power
///   - This measures the ability of sellers to push the price down.
///   -  It's calculated by subtracting the EWMA from the low price.
///
/// # Source
///
/// https://www.investopedia.com/articles/trading/03/022603.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::elder_ray(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     3).collect::<Vec<(f64,f64)>>();
///
/// ```
pub fn elder_ray<'a>(
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    window: usize,
) -> impl Iterator<Item = (f64, f64)> + 'a {
    let close_ma = smooth::ewma(close, window);
    izip!(
        high.iter().skip(window - 1),
        low.iter().skip(window - 1),
        close_ma
    )
    .map(|(h, l, c)| (h - c, l - c))
}

/// williams alligator
/// https://www.investopedia.com/articles/trading/072115/exploring-williams-alligator-indicator.asp
pub fn alligator(_data: &[f64]) {}

/// Williams Percent Range
///
/// Measure the level of a security's close price in relation to its high-low range over a
/// specified period.
///
/// W%R = (Highest High - Close) / (Highest High - Lowest Low) * -100
///
/// # Source
/// https://www.investopedia.com/terms/w/williamsr.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::wpr(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn wpr<'a>(
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    window: usize,
) -> impl Iterator<Item = f64> + 'a {
    izip!(
        high.windows(window),
        low.windows(window),
        &close[(window - 1)..]
    )
    .map(|(h, l, c)| {
        let hh = h.iter().fold(f64::NAN, |state, &x| state.max(x));
        let ll = l.iter().fold(f64::NAN, |state, &x| state.min(x));
        -100.0 * ((hh - c) / (hh - ll))
    })
}

/// Percent price oscillator
///
/// Measure the difference between two moving averages as a percentage of the larger
/// moving average.
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::ppo(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     3, 6).collect::<Vec<f64>>();
///
/// ```
pub fn ppo(data: &[f64], short: usize, long: usize) -> impl Iterator<Item = f64> + '_ {
    let short_ma = smooth::ewma(data, short);
    let long_ma = smooth::ewma(data, long);
    short_ma
        .skip(long - short)
        .zip(long_ma)
        .map(|(x, y)| 100.0 * (x / y - 1.0))
}

/// Absolute Price Oscillator
///
/// Measure the difference between two moving averages.
///
/// # Source
///
/// https://www.fidelity.com/learning-center/trading-investing/technical-analysis/technical-indicator-guide/apo
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::apo(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     3, 6).collect::<Vec<f64>>();
///
/// ```
pub fn apo(data: &[f64], short: usize, long: usize) -> impl Iterator<Item = f64> + '_ {
    let short_ma = smooth::ewma(data, short);
    let long_ma = smooth::ewma(data, long);
    short_ma.skip(long - short).zip(long_ma).map(|(x, y)| x - y)
}

/// Detrended Price Oscillator
///
/// Measure the difference between a security's price and its trend.
///
/// # Examples
///
/// ```
/// use traquer::{momentum,smooth};
///
/// momentum::dpo(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6, Some(smooth::MaMode::SMA)).collect::<Vec<f64>>();
///
/// ```
pub fn dpo(
    data: &[f64],
    window: usize,
    mamode: Option<smooth::MaMode>,
) -> impl Iterator<Item = f64> + '_ {
    let ma = smooth::ma(data, window, mamode.unwrap_or(smooth::MaMode::SMA));
    let lag = window / 2 + 1;
    data[window - lag - 1..].iter().zip(ma).map(|(x, y)| x - y)
}

/// Ultimate oscillator
///
/// A technical indicator that uses the weighted average of three different time periods
/// to reduce the volatility and false transaction signals.
///
/// # Source
///
/// https://www.investopedia.com/terms/u/ultimateoscillator.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::ultimate(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     2, 4, 8).collect::<Vec<f64>>();
///
/// ```
pub fn ultimate<'a>(
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    win1: usize,
    win2: usize,
    win3: usize,
) -> impl Iterator<Item = f64> + 'a {
    let bp_tr_vals = izip!(
        &high[1..],
        &low[1..],
        &close[..close.len() - 1],
        &close[1..],
    )
    .map(|(h, l, prevc, c)| {
        (
            c - f64::min(*l, *prevc),
            f64::max(*h, *prevc) - f64::min(*l, *prevc),
        )
    })
    .collect::<Vec<(f64, f64)>>();
    bp_tr_vals
        .windows(win3)
        .map(|w| {
            let (bp_sum1, tr_sum1) = w
                .iter()
                .skip(win3 - win1)
                .fold((0.0, 0.0), |acc, (bp, tr)| (acc.0 + bp, acc.1 + tr));
            let (bp_sum2, tr_sum2) = w
                .iter()
                .skip(win3 - win2)
                .fold((0.0, 0.0), |acc, (bp, tr)| (acc.0 + bp, acc.1 + tr));
            let (bp_sum3, tr_sum3) = w
                .iter()
                .fold((0.0, 0.0), |acc, (bp, tr)| (acc.0 + bp, acc.1 + tr));
            100.0 * (bp_sum1 / tr_sum1 * 4.0 + bp_sum2 / tr_sum2 * 2.0 + bp_sum3 / tr_sum3)
                / (4.0 + 2.0 + 1.0)
        })
        .collect::<Vec<f64>>()
        .into_iter()
}

/// Pretty good oscillator
///
/// Combines moving averages and the Average True Range (ATR) to create an oscillator
/// that oscillates around a centerline
///
/// # Source
///
/// https://library.tradingtechnologies.com/trade/chrt-ti-pretty-good-oscillator.html
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::pgo(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn pgo<'a>(
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    window: usize,
) -> impl Iterator<Item = f64> + 'a {
    let tr = _true_range(high, low, close).collect::<Vec<f64>>();
    let atr = smooth::ewma(&tr, window);
    let sma_close = smooth::sma(close, window);
    izip!(close.iter().skip(window), sma_close.skip(1), atr)
        .map(|(c, c_ma, tr_ma)| (c - c_ma) / tr_ma)
        .collect::<Vec<f64>>()
        .into_iter()
}

pub(crate) fn _swing<'a>(
    open: &'a [f64],
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    limit: f64,
) -> impl Iterator<Item = f64> + 'a {
    izip!(
        &open[..open.len() - 1],
        &open[1..],
        &high[1..],
        &low[1..],
        &close[..close.len() - 1],
        &close[1..],
    )
    .map(move |(prevo, o, h, l, prevc, c)| {
        let r1 = (h - prevc).abs();
        let r2 = (l - prevc).abs();
        let r3 = h - l;
        let r4 = (prevc - prevo).abs() / 4.0;
        let max_r = r1.max(r2).max(r3);
        let r = if r1 == max_r {
            r1 - r2 / 2.0 + r4
        } else if r2 == max_r {
            r2 - r1 / 2.0 + r4
        } else {
            r3 + r4
        };
        // does not use formula described in investopedia as it appears to be wrong?
        // it seems to overweight previous period's values
        ((c - prevc + (c - o) / 2.0 + (prevc - prevo) / 4.0) / r) * 50.0 * r1.max(r2) / limit
    })
}

/// Swing Index
///
/// Calculates the strength of price movement and predicts potential trend reversal.
///
/// # Source
///
/// https://www.investopedia.com/terms/a/asi.asp
/// https://quantstrategy.io/blog/accumulative-swing-index-how-to-trade/
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::si(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     0.5).collect::<Vec<f64>>();
///
/// ```
pub fn si<'a>(
    open: &'a [f64],
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    limit: f64,
) -> impl Iterator<Item = f64> + 'a {
    _swing(open, high, low, close, limit)
}

/// Triple Exponential Average
///
/// Indicator to show the percentage change in a moving average that has been smoothed
/// exponentially three times.
///
/// # Source
///
/// https://www.investopedia.com/terms/t/trix.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::trix(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0,5.0,2.0],
///     3).collect::<Vec<f64>>();
///
/// ```
pub fn trix(close: &[f64], window: usize) -> impl Iterator<Item = f64> + '_ {
    let ema3 = smooth::ewma(
        &smooth::ewma(&smooth::ewma(close, window).collect::<Vec<f64>>(), window)
            .collect::<Vec<f64>>(),
        window,
    )
    .collect::<Vec<f64>>();
    ema3[..ema3.len() - 1]
        .iter()
        .zip(&ema3[1..])
        .map(|(prev, curr)| 100.0 * (curr - prev) / prev)
        .collect::<Vec<f64>>()
        .into_iter()
}

/// Trend intensity index
///
/// Uses RSI principles but applies them to closing price deviations instead of the closing prices
///
/// # Source
///
/// https://www.marketvolume.com/technicalanalysis/trendintensityindex.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::tii(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn tii(data: &[f64], window: usize) -> impl Iterator<Item = f64> + '_ {
    smooth::sma(data, window)
        .zip(&data[(window - 1)..])
        .map(|(avg, actual)| {
            let dev: f64 = actual - avg;
            (dev.max(0.0), dev.min(0.0).abs())
        })
        .collect::<Vec<(f64, f64)>>()
        .windows(window.div_ceil(2))
        .map(|w| {
            let mut sd_pos = 0.0;
            let mut sd_neg = 0.0;
            for (pos_dev, neg_dev) in w {
                sd_pos += pos_dev;
                sd_neg += neg_dev;
            }
            100.0 * sd_pos / (sd_pos + sd_neg)
        })
        .collect::<Vec<f64>>()
        .into_iter()
}

/// Stochastic Oscillator
///
/// Compares a security’s closing price to a range of its highest highs and lowest lows
/// over a specific time period.
///
/// # Source
///
/// https://www.investopedia.com/articles/technical/073001.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::stochastic(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     3).collect::<Vec<(f64, f64)>>();
///
/// ```
pub fn stochastic<'a>(
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    window: usize,
) -> impl Iterator<Item = (f64, f64)> + 'a {
    let fast_k = smooth::sma(
        &izip!(
            high.windows(window),
            low.windows(window),
            &close[(window - 1)..]
        )
        .map(|(h, l, c)| {
            let hh = h.iter().fold(f64::NAN, |state, &x| state.max(x));
            let ll = l.iter().fold(f64::NAN, |state, &x| state.min(x));
            100.0 * (c - ll) / (hh - ll)
        })
        .collect::<Vec<f64>>(),
        3,
    )
    .collect::<Vec<f64>>();
    let k = smooth::sma(&fast_k, 3).collect::<Vec<f64>>();
    izip!(fast_k, iter::repeat(f64::NAN).take(3 - 1).chain(k))
}

fn _stc(series: &[f64], window: usize) -> impl Iterator<Item = f64> + '_ {
    smooth::wilder(
        &series
            .windows(window)
            .map(|w| {
                let mut hh = f64::NAN;
                let mut ll = f64::NAN;
                for x in w {
                    hh = hh.max(*x);
                    ll = ll.min(*x);
                }
                100.0 * (w.last().unwrap() - ll) / (hh - ll)
            })
            .collect::<Vec<f64>>(),
        2,
    )
    .collect::<Vec<f64>>()
    .into_iter()
}

/// Schaff Trend Cycle
///
/// A modified version of the Moving Average Convergence Divergence. It aims to improve
/// upon traditional moving averages (MAs) by incorporating cycle analysis.
///
/// # Source
///
/// https://www.investopedia.com/articles/forex/10/schaff-trend-cycle-indicator.asp
/// https://www.stockmaniacs.net/schaff-trend-cycle-indicator/
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::stc(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0,6.0,1.0],
///     2, 3, 5).collect::<Vec<f64>>();
///
/// ```
pub fn stc(
    close: &[f64],
    window: usize,
    short: usize,
    long: usize,
) -> impl Iterator<Item = f64> + '_ {
    let series = macd(close, short, long);
    _stc(
        &_stc(&series.collect::<Vec<f64>>(), window).collect::<Vec<f64>>(),
        window,
    )
    .collect::<Vec<f64>>()
    .into_iter()
}

/// Relative Vigor
///
/// Measures the strength of a trend by comparing a security’s closing price to its trading range
/// while smoothing the results using a simple moving average.
///
/// # Source
///
/// https://www.investopedia.com/terms/r/relative_vigor_index.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::relative_vigor(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn relative_vigor<'a>(
    open: &'a [f64],
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    window: usize,
) -> impl Iterator<Item = f64> + 'a {
    let close_open = open
        .iter()
        .zip(close)
        .map(|(o, c)| c - o)
        .collect::<Vec<f64>>();
    let high_low = high
        .iter()
        .zip(low)
        .map(|(h, l)| h - l)
        .collect::<Vec<f64>>();

    let numerator = close_open
        .windows(4)
        .map(|w| (w[3] + 2.0 * w[2] + 2.0 * w[1] + w[0]) / 6.0)
        .collect::<Vec<f64>>();
    let denominator = high_low
        .windows(4)
        .map(|w| (w[3] + 2.0 * w[2] + 2.0 * w[1] + w[0]) / 6.0)
        .collect::<Vec<f64>>();
    smooth::sma(&numerator, window)
        .zip(smooth::sma(&denominator, window))
        .map(|(n, d)| n / d)
        .collect::<Vec<f64>>()
        .into_iter()
}

/// Elhers Fisher Transform
///
/// Converts price data into a Gaussian normal distribution.
/// Extreme readings (above +1 or below -1) may signal potential price reversals.
///
/// # Source
///
/// https://www.investopedia.com/terms/f/fisher-transform.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::fisher(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn fisher<'a>(
    high: &'a [f64],
    low: &'a [f64],
    window: usize,
) -> impl Iterator<Item = f64> + 'a {
    let hl2 = high
        .iter()
        .zip(low)
        .map(|(h, l)| (h + l) / 2.0)
        .collect::<Vec<f64>>();
    hl2.windows(window)
        .scan((0.0, 0.0), |state, w| {
            let mut hl_max: f64 = 0.0;
            let mut hl_min: f64 = f64::MAX;
            for &e in w {
                hl_max = hl_max.max(e);
                hl_min = hl_min.min(e);
            }
            let transform = (0.66
                * ((w[window - 1] - hl_min) / (hl_max - hl_min).max(0.000001) - 0.5)
                + 0.67 * state.0)
                .min(0.999999)
                .max(-0.999999);
            let result = 0.5 * ((1.0 + transform) / (1.0 - transform)).ln() + 0.5 * state.1;
            *state = (transform, result);
            Some(state.1)
        })
        .collect::<Vec<f64>>()
        .into_iter()
}

/// Rainbow Oscillator
///
/// Based on multiple simple moving averages (SMAs). The highest high and lowest low
/// of these SMAs create high and low oscillator curves.
///
/// # Source
///
/// https://www.tradingview.com/script/gWYg0ti0-Indicators-Rainbow-Charts-Oscillator-Binary-Wave-and-MAs/
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::rainbow(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0,1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     2, 3).collect::<Vec<(f64, f64)>>();
///
/// ```
pub fn rainbow(
    data: &[f64],
    window: usize,
    lookback: usize,
) -> impl Iterator<Item = (f64, f64)> + '_ {
    let mut smas = Vec::with_capacity(10);
    smas.push(smooth::sma(data, window).collect::<Vec<f64>>());
    for _ in 1..10 {
        smas.push(
            iter::repeat(f64::NAN)
                .take(window - 1)
                .chain(smooth::sma(&smas[smas.len() - 1], window))
                .collect::<Vec<f64>>(),
        );
    }
    ((window - 1) * 10..data.len()).map(move |i| {
        let mut total: f64 = 0.0;
        let mut hsma = f64::MIN;
        let mut lsma = f64::MAX;
        for sma in smas.iter() {
            let val = sma[i - (window - 1)];
            total += val;
            hsma = hsma.max(val);
            lsma = lsma.min(val);
        }
        let mut hlookback = f64::MIN;
        let mut llookback = f64::MAX;
        ((i - (lookback - 1)).max(0)..=i).for_each(|x| {
            let val = data[x];
            hlookback = hlookback.max(val);
            llookback = llookback.min(val);
        });
        let osc = 100.0 * (data[i] - total / 10.0) / (hlookback - llookback).max(0.000001);
        let band = 100.0 * (hsma - lsma) / (hlookback - llookback).max(0.000001);
        (osc, band)
    })
}

/// Coppock Curve
///
/// Calculated as a weighted moving average of the sum of two rate of change periods
///
/// # Source
///
/// https://www.investopedia.com/terms/c/coppockcurve.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::coppock(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0,1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     2, 3, 6).collect::<Vec<f64>>();
///
/// ```
pub fn coppock(
    data: &[f64],
    window: usize,
    short: usize,
    long: usize,
) -> impl Iterator<Item = f64> + '_ {
    smooth::wma(
        &(long..data.len())
            .map(|x| 100.0 * (data[x] / data[x - short] + data[x] / data[x - long] - 2.0))
            .collect::<Vec<f64>>(),
        window,
    )
    .collect::<Vec<f64>>()
    .into_iter()
}

/// Rate of Change
///
/// Measures the percentage change in price between the current price
/// and the price a certain number of periods prior.
///
/// # Source
///
/// https://www.investopedia.com/terms/p/pricerateofchange.asp.
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::roc(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn roc(data: &[f64], window: usize) -> impl Iterator<Item = f64> + '_ {
    data.windows(window)
        .map(|w| 100.0 * (w[w.len() - 1] / w[0] - 1.0))
}

/// Balance of Power
///
/// An oscillator that measures the strength of buying and selling pressure
///
/// # Source
///
/// https://www.tradingview.com/support/solutions/43000589100-balance-of-power-bop/
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::bal_power(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     3).collect::<Vec<f64>>();
///
/// ```
pub fn bal_power<'a>(
    open: &'a [f64],
    high: &'a [f64],
    low: &'a [f64],
    close: &'a [f64],
    window: usize,
) -> impl Iterator<Item = f64> + 'a {
    smooth::ewma(
        &izip!(open, high, low, close)
            .map(|(o, h, l, c)| (c - o) / (h - l))
            .collect::<Vec<f64>>(),
        window,
    )
    .collect::<Vec<f64>>()
    .into_iter()
}

/// Disparity Index
///
/// Measures the relative position of the most recent closing price to a selected
/// moving average as a percentage.
///
/// # Source
///
/// https://www.investopedia.com/terms/d/disparityindex.asp
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::disparity(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     6).collect::<Vec<f64>>();
///
/// ```
pub fn disparity(data: &[f64], window: usize) -> impl Iterator<Item = f64> + '_ {
    data.iter()
        .skip(window - 1)
        .zip(smooth::ewma(data, window))
        .map(|(x, ma)| 100.0 * (x - ma) / ma)
}

/// Aroon
///
/// Measures the trend strength and direction of an asset based on the time between highs and
/// lows. Consists of two lines that show the time since a recent high or low.
///
/// # Source
///
/// https://www.tradingview.com/support/solutions/43000501801-aroon/
///
/// # Examples
///
/// ```
/// use traquer::momentum;
///
/// momentum::aroon(
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     &vec![1.0,2.0,3.0,4.0,5.0,6.0,4.0,5.0],
///     3).collect::<Vec<(f64, f64)>>();
///
/// ```
pub fn aroon<'a>(
    high: &'a [f64],
    low: &'a [f64],
    window: usize,
) -> impl Iterator<Item = (f64, f64)> + 'a {
    high.windows(window + 1)
        .zip(low.windows(window + 1))
        .map(move |(h, l)| {
            let (hh, _) =
                h.iter().enumerate().fold(
                    (0, h[0]),
                    |state, (idx, x)| {
                        if x >= &state.1 {
                            (idx, *x)
                        } else {
                            state
                        }
                    },
                );
            let (ll, _) =
                l.iter().enumerate().fold(
                    (0, l[0]),
                    |state, (idx, x)| {
                        if x <= &state.1 {
                            (idx, *x)
                        } else {
                            state
                        }
                    },
                );
            (
                hh as f64 / window as f64 * 100.0,
                ll as f64 / window as f64 * 100.0,
            )
        })
}
