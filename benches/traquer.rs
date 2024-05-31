use criterion::{black_box, criterion_group, criterion_main, Criterion};

use serde::{Deserialize, Serialize};
use std::fs;

use traquer::*;

#[derive(Deserialize, Serialize, Debug)]
struct SecStats {
    high: Vec<f64>,
    low: Vec<f64>,
    open: Vec<f64>,
    close: Vec<f64>,
    volume: Vec<f64>,
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = fs::read_to_string("./benches/aapl.input").expect("Unable to read file");
    let stats: SecStats = serde_json::from_str(&data).expect("JSON does not have correct format.");
    c.bench_function("sig-trend-adx", |b| {
        b.iter(|| {
            black_box(
                trend::adx(&stats.high, &stats.low, &stats.close, 14, 14)
                    .collect::<Vec<(f64, f64, f64)>>(),
            )
        })
    });
    c.bench_function("sig-trend-qstick", |b| {
        b.iter(|| black_box(trend::qstick(&stats.open, &stats.close, 8).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-volume-twiggs", |b| {
        b.iter(|| {
            black_box(
                volume::twiggs(&stats.high, &stats.low, &stats.close, &stats.volume, 16)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-momentum-rsi", |b| {
        b.iter(|| black_box(momentum::rsi(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-volume-kvo", |b| {
        b.iter(|| {
            black_box(
                volume::kvo(&stats.high, &stats.low, &stats.close, &stats.volume, 10, 16)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-momentum-macd", |b| {
        b.iter(|| black_box(momentum::macd(&stats.close, 12, 26).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-cmo", |b| {
        b.iter(|| black_box(momentum::cmo(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-trend-cog", |b| {
        b.iter(|| black_box(trend::cog(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-trend-shinohara", |b| {
        b.iter(|| {
            black_box(
                trend::shinohara(&stats.high, &stats.low, &stats.close, 26)
                    .collect::<Vec<(f64, f64)>>(),
            )
        })
    });
    c.bench_function("sig-momentum-elder_ray", |b| {
        b.iter(|| {
            black_box(
                momentum::elder_ray(&stats.high, &stats.low, &stats.close, 16)
                    .collect::<Vec<(f64, f64)>>(),
            )
        })
    });
    c.bench_function("sig-volume-elder_force", |b| {
        b.iter(|| {
            black_box(volume::elder_force(&stats.close, &stats.volume, 16).collect::<Vec<f64>>())
        })
    });
    c.bench_function("sig-volume-mfi", |b| {
        b.iter(|| {
            black_box(
                volume::mfi(&stats.high, &stats.low, &stats.close, &stats.volume, 16)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-volume-ad", |b| {
        b.iter(|| {
            black_box(
                volume::ad(&stats.high, &stats.low, &stats.close, &stats.volume, None)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-volume-ad_yahoo", |b| {
        b.iter(|| {
            black_box(
                volume::ad(
                    &stats.high,
                    &stats.low,
                    &stats.close,
                    &stats.volume,
                    Some(true),
                )
                .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-volume-cmf", |b| {
        b.iter(|| {
            black_box(
                volume::cmf(&stats.high, &stats.low, &stats.close, &stats.volume, 16)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-momentum-cvi", |b| {
        b.iter(|| black_box(momentum::cvi(&stats.high, &stats.low, 16, 2).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-wpr", |b| {
        b.iter(|| {
            black_box(
                momentum::wpr(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-trend-vortex", |b| {
        b.iter(|| {
            black_box(
                trend::vortex(&stats.high, &stats.low, &stats.close, 16)
                    .collect::<Vec<(f64, f64)>>(),
            )
        })
    });
    c.bench_function("sig-momentum-ppo", |b| {
        b.iter(|| black_box(momentum::ppo(&stats.volume, 10, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-apo", |b| {
        b.iter(|| black_box(momentum::apo(&stats.close, 10, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-dpo", |b| {
        b.iter(|| black_box(momentum::dpo(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-trend-vhf", |b| {
        b.iter(|| {
            black_box(trend::vhf(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<f64>>())
        })
    });
    c.bench_function("sig-momentum-ultimate", |b| {
        b.iter(|| {
            black_box(
                momentum::ultimate(&stats.high, &stats.low, &stats.close, 6, 12, 24)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-momentum-pgo", |b| {
        b.iter(|| {
            black_box(
                momentum::pgo(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-momentum-si", |b| {
        b.iter(|| {
            black_box(
                momentum::si(&stats.open, &stats.high, &stats.low, &stats.close, 0.5)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-trend-asi", |b| {
        b.iter(|| {
            black_box(
                trend::asi(&stats.open, &stats.high, &stats.low, &stats.close, 0.5)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-trend-ulcer", |b| {
        b.iter(|| black_box(trend::ulcer(&stats.close, 8).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-trend-tr", |b| {
        b.iter(|| black_box(trend::tr(&stats.high, &stats.low, &stats.close).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-trend-atr", |b| {
        b.iter(|| {
            black_box(trend::atr(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<f64>>())
        })
    });
    c.bench_function("sig-trend-typical", |b| {
        b.iter(|| {
            black_box(
                trend::typical(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-momentum-trix", |b| {
        b.iter(|| black_box(momentum::trix(&stats.close, 7).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-tii", |b| {
        b.iter(|| black_box(momentum::tii(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-volume-tvi", |b| {
        b.iter(|| black_box(volume::tvi(&stats.close, &stats.volume, 0.5).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-trend-supertrend", |b| {
        b.iter(|| {
            black_box(
                trend::supertrend(&stats.high, &stats.low, &stats.close, 16, 3.0)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-momentum-stochastic", |b| {
        b.iter(|| {
            black_box(
                momentum::stochastic(&stats.high, &stats.low, &stats.close, 16)
                    .collect::<Vec<(f64, f64)>>(),
            )
        })
    });
    c.bench_function("sig-momentum-stc", |b| {
        b.iter(|| black_box(momentum::stc(&stats.close, 3, 6, 12).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-relative_vol", |b| {
        b.iter(|| black_box(momentum::relative_vol(&stats.close, 6, 10).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-relative_vigor", |b| {
        b.iter(|| {
            black_box(
                momentum::relative_vigor(&stats.open, &stats.high, &stats.low, &stats.close, 16)
                    .collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-trend-rwi", |b| {
        b.iter(|| {
            black_box(
                trend::rwi(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<(f64, f64)>>(),
            )
        })
    });
    c.bench_function("sig-momentum-fisher", |b| {
        b.iter(|| black_box(momentum::fisher(&stats.high, &stats.low, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-rainbow", |b| {
        b.iter(|| black_box(momentum::rainbow(&stats.close, 3, 16).collect::<Vec<(f64, f64)>>()))
    });
    c.bench_function("sig-momentum-coppock", |b| {
        b.iter(|| black_box(momentum::coppock(&stats.close, 10, 11, 14).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-trend-psych", |b| {
        b.iter(|| black_box(trend::psych(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-trend-mass", |b| {
        b.iter(|| black_box(trend::mass(&stats.high, &stats.low, 9, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-roc", |b| {
        b.iter(|| black_box(momentum::roc(&stats.low, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-momentum-chop", |b| {
        b.iter(|| {
            black_box(
                momentum::chop(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-trend-keltner", |b| {
        b.iter(|| {
            black_box(
                trend::keltner(&stats.high, &stats.low, &stats.close, 16)
                    .collect::<Vec<(f64, f64, f64)>>(),
            )
        })
    });
    c.bench_function("sig-trend-gri", |b| {
        b.iter(|| black_box(trend::gri(&stats.high, &stats.low, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("sig-volume-bw_mfi", |b| {
        b.iter(|| {
            black_box(volume::bw_mfi(&stats.high, &stats.low, &stats.volume).collect::<Vec<f64>>())
        })
    });
    c.bench_function("sig-volume-ease", |b| {
        b.iter(|| {
            black_box(
                volume::ease(&stats.high, &stats.low, &stats.volume, 16).collect::<Vec<f64>>(),
            )
        })
    });
    c.bench_function("sig-volume-obv", |b| {
        b.iter(|| black_box(volume::obv(&stats.close, &stats.volume).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-ewma", |b| {
        b.iter(|| black_box(smooth::ewma(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-sma", |b| {
        b.iter(|| black_box(smooth::sma(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-dema", |b| {
        b.iter(|| black_box(smooth::dema(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-tema", |b| {
        b.iter(|| black_box(smooth::tema(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-wma", |b| {
        b.iter(|| black_box(smooth::wma(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-wilder", |b| {
        b.iter(|| black_box(smooth::wilder(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-hull", |b| {
        b.iter(|| black_box(smooth::hull(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-vidya", |b| {
        b.iter(|| black_box(smooth::vidya(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-vma", |b| {
        b.iter(|| black_box(smooth::vma(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-lrf", |b| {
        b.iter(|| black_box(smooth::lrf(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-trima", |b| {
        b.iter(|| black_box(smooth::trima(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-zlma", |b| {
        b.iter(|| black_box(smooth::zlma(&stats.close, 16).collect::<Vec<f64>>()))
    });
    c.bench_function("ma-pwma", |b| {
        b.iter(|| black_box(smooth::pwma(&stats.close, 16).collect::<Vec<f64>>()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

//#[divan::bench]
//fn run() {
//    divan::black_box(traquer::main());
//}
//
//fn main() {
//    divan::main();
//}
