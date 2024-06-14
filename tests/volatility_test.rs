use traquer::volatility;

mod common;

#[test]
fn test_mass() {
    let stats = common::test_data();
    let result = volatility::mass(&stats.high, &stats.low, 9, 6).collect::<Vec<_>>();
    assert_eq!(stats.high.len(), result.len());
    assert_eq!(
        vec![
            4.520707218752633,
            4.57207750331251,
            4.6318245383826735,
            4.680054893601096,
            4.7327145765178855,
            4.827573448286292,
            4.970401688240842,
            5.218891609600159,
            5.444846247296847,
            5.597926479420781,
            5.709803845941307,
            5.942103734664013,
            6.0808751147235265,
        ],
        result[16 + 5..]
    );
}

#[test]
fn test_keltner() {
    let stats = common::test_data();
    let result = volatility::keltner(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            //(48.42312526702881, f64::NAN, f64::NAN,),
            (47.56628692851347, 59.468162424424115, 35.664411432602826,),
            (46.57848825091722, 58.04149635667207, 35.115480145162365,),
            (46.00690154148026, 57.3509719648747, 34.66283111808582,),
            (45.4037367799079, 56.49630325960386, 34.31117030021194,),
            (45.008003148804306, 55.66478439518065, 34.351221902427966,),
            (44.75059097774783, 55.0210737433631, 34.480108212132556,),
            (44.56228633635148, 54.38086398633625, 34.7437086863667,),
            (44.313782151244176, 53.806199079574625, 34.82136522291373,),
            (44.44510193405966, 53.81286787303401, 35.077335995085306,),
            (44.66097215114639, 53.874502814302275, 35.4474414879905,),
            (44.634975265918456, 53.59828520540664, 35.67166532643027,),
            (44.78144888970631, 53.59205222500529, 35.97084555440733,),
            (45.12480793361609, 53.8746234002416, 36.37499246699058,),
            (45.30306575197284, 53.813517637493334, 36.79261386645234,),
            (45.65211689383058, 54.034415479285585, 37.26981830837558,),
            (46.09304449760097, 54.317699459612015, 37.86838953558993,),
            (46.717392383280725, 55.14800620035773, 38.28677856620372,),
            (47.02064012277067, 55.31809076701522, 38.72318947852612,),
        ],
        result[16..]
    );
}

#[test]
fn test_gri() {
    let stats = common::test_data();
    let result = volatility::gri(&stats.high, &stats.low, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            1.2845807635148472,
            1.2882486699344737,
            1.2944794556504038,
            1.1807530970142432,
            1.041478000063852,
            0.9518387305144009,
            0.9518387305144009,
            0.9489937567570867,
            0.8910945979665684,
            0.8910945979665684,
            0.8910945979665684,
            0.8818302331128451,
            0.8818302331128451,
            0.9048239475184171,
            0.9048239475184171,
            0.9115406774766412,
            0.9245547128348174,
            1.0386539306425049,
            1.0386539306425049,
        ],
        result[15..]
    );
}

#[test]
fn test_tr() {
    let stats = common::test_data();
    let result = volatility::tr(&stats.high, &stats.low, &stats.close).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            15.939998626708984,
            15.10000228881836,
            9.490001678466797,
            8.650001525878906,
            4.924999237060547,
            7.349998474121094,
            4.69000244140625,
            3.3300018310546875,
            3.6100006103515625,
            4.1399993896484375,
            2.5900001525878906,
            3.279998779296875,
            4.340000152587891,
            2.3699989318847656,
            2.5900001525878906,
            2.8199996948242188,
            2.4399986267089844,
            4.780002593994141,
            3.660003662109375,
            2.0600013732910156,
            2.2380027770996094,
            1.5200004577636719,
            2.3000030517578125,
            3.7490005493164063,
            3.450000762939453,
            2.604999542236328,
            3.2600021362304688,
            3.918998718261726,
            2.4599990844726563,
            3.229999542236328,
            2.9300003051757813,
            5.759998321533203,
            3.1500015258789063,
        ],
        result[1..]
    );
}

#[test]
fn test_atr() {
    let stats = common::test_data();
    let result = volatility::atr(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            5.950937747955322,
            5.731504052877426,
            5.672035211697221,
            5.54628323984798,
            5.32839062318817,
            5.135241382807635,
            4.909288824992387,
            4.7462084641652265,
            4.6838829694871755,
            4.606765331577943,
            4.481654969744092,
            4.405301667649491,
            4.374907733312755,
            4.255225942760249,
            4.191149292727504,
            4.112327481005521,
            4.215306908538501,
            4.148725322122276,
        ],
        result[16..]
    );
}

#[test]
fn test_typical() {
    let stats = common::test_data();
    let result = volatility::typical(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            48.82131250699361,
            48.41006247202555,
            47.38204161326091,
            45.67329160372416,
            44.58475001653036,
            43.98891671498617,
            43.760937531789146,
            43.422812620798744,
            43.03031253814698,
            42.92550015449524,
            42.935500065485634,
            42.83081253369649,
            42.84727080663045,
            43.15249999364217,
            43.33354179064433,
            43.67937509218853,
            44.2075002193451,
            44.90875029563905,
            45.53729192415874,
        ],
        result[15..]
    );
}

#[test]
fn test_std_dev() {
    let stats = common::test_data();
    let result = volatility::std_dev(&stats.close, 16, None).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            13.25162572657857,
            13.675877473717561,
            12.89547461554456,
            8.98517502012738,
            6.557030792233012,
            6.098639502630048,
            6.016125848996605,
            4.889473982961166,
            4.488570530474252,
            4.4357419509169445,
            4.132481276694623,
            3.8620698826696045,
            4.012893026824987,
            4.645152841708539,
            4.91017049156476,
            5.44108447177409,
            5.768391548667223,
            6.494713715395781,
            6.118108066427051,
        ],
        result[15..]
    );
}

#[test]
fn test_bbands() {
    let stats = common::test_data();
    let result = volatility::bbands(&stats.close, 16, None, None).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            (61.674750993607375, 48.42312526702881, 35.17149954045024),
            (61.24216440223103, 47.56628692851347, 33.89040945479591),
            (59.473962866461775, 46.57848825091722, 33.68301363537266),
            (54.99207656160764, 46.00690154148026, 37.02172652135288),
            (51.96076757214092, 45.4037367799079, 38.84670598767489),
            (51.106642651434356, 45.008003148804306, 38.909363646174256),
            (50.76671682674443, 44.75059097774783, 38.734465128751225),
            (49.45176031931264, 44.56228633635148, 39.67281235339031),
            (48.80235268171843, 44.313782151244176, 39.82521162076992),
            (48.8808438849766, 44.44510193405966, 40.009359983142716),
            (48.793453427841015, 44.66097215114639, 40.52849087445176),
            (48.49704514858806, 44.634975265918456, 40.772905383248855),
            (48.794341916531295, 44.78144888970631, 40.76855586288133),
            (49.76996077532463, 45.12480793361609, 40.47965509190755),
            (50.2132362435376, 45.30306575197284, 40.392895260408075),
            (51.09320136560467, 45.65211689383058, 40.21103242205649),
            (51.861436046268196, 46.09304449760097, 40.324652948933746),
            (53.21210609867651, 46.717392383280725, 40.22267866788494),
            (53.13874818919772, 47.02064012277067, 40.902532056343624)
        ],
        result[15..]
    );
}

#[test]
fn test_donchian() {
    let stats = common::test_data();
    let result = volatility::donchian(&stats.high, &stats.low, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            (74.9000015258789, 57.290000915527344, 39.68000030517578),
            (74.9000015258789, 57.11000061035156, 39.31999969482422),
            (74.9000015258789, 56.80000114440918, 38.70000076293945),
            (63.7599983215332, 50.55499839782715, 37.349998474121094),
            (55.29999923706055, 46.32499885559082, 37.349998474121094),
            (51.349998474121094, 44.349998474121094, 37.349998474121094),
            (51.349998474121094, 44.349998474121094, 37.349998474121094),
            (51.2400016784668, 44.295000076293945, 37.349998474121094),
            (49.18000030517578, 43.26499938964844, 37.349998474121094),
            (49.18000030517578, 43.26499938964844, 37.349998474121094),
            (49.18000030517578, 43.26499938964844, 37.349998474121094),
            (48.880001068115234, 43.114999771118164, 37.349998474121094),
            (48.880001068115234, 43.114999771118164, 37.349998474121094),
            (49.63899993896485, 43.49449920654297, 37.349998474121094),
            (49.63899993896485, 43.49449920654297, 37.349998474121094),
            (49.869998931884766, 43.60999870300293, 37.349998474121094),
            (50.33000183105469, 43.84000015258789, 37.349998474121094),
            (55.15999984741211, 46.2549991607666, 37.349998474121094),
            (55.15999984741211, 46.2549991607666, 37.349998474121094)
        ],
        result[15..]
    );
}

#[test]
fn test_fbands() {
    let stats = common::test_data();
    let result = volatility::fbands(&stats.high, &stats.low).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            (74.9000015258789, 0.0,),
            (74.9000015258789, 0.0,),
            (74.9000015258789, 0.0,),
            (74.9000015258789, 0.0,),
            (74.9000015258789, 44.0,),
            (74.9000015258789, 44.0,),
            (74.9000015258789, 44.0,),
            (74.9000015258789, 44.68000030517578,),
            (49.18000030517578, 44.68000030517578,),
            (49.18000030517578, 44.68000030517578,),
            (49.18000030517578, 41.720001220703125,),
            (49.18000030517578, 41.720001220703125,),
            (49.18000030517578, 41.720001220703125,),
            (49.18000030517578, 41.720001220703125,),
            (49.18000030517578, 41.720001220703125,),
            (49.18000030517578, 41.720001220703125,),
            (49.18000030517578, 37.349998474121094,),
            (49.18000030517578, 37.349998474121094,),
            (49.18000030517578, 37.349998474121094,),
            (44.060001373291016, 37.349998474121094,),
            (44.060001373291016, 37.349998474121094,),
            (44.060001373291016, 40.849998474121094,),
            (44.060001373291016, 40.849998474121094,),
            (48.880001068115234, 40.849998474121094,),
            (48.880001068115234, 40.849998474121094,),
            (48.880001068115234, 40.849998474121094,),
            (48.880001068115234, 40.849998474121094,),
            (48.880001068115234, 40.849998474121094,),
            (48.880001068115234, 40.849998474121094,),
            (48.880001068115234, 40.849998474121094,),
        ],
        result[4..]
    );
}

#[test]
fn test_hv() {
    let stats = common::test_data();
    let result = volatility::hv(&stats.close, 16, None).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            1.5426523650567059,
            1.0814850225220476,
            1.0485549292793799,
            0.9782010217271724,
            0.8181908667315462,
            0.7854803744652038,
            0.6801589758647922,
            0.6334127247577533,
            0.6878195293244158,
            0.6744749472856001,
            0.6825716435084193,
            0.6876958317470336,
            0.6413496021691397,
            0.618520803889366,
            0.5858264543082782,
            0.5264279426082943,
            0.5334323440731333,
            0.5205135758451463,
        ],
        result[16..]
    );
}

#[test]
fn test_starc() {
    let stats = common::test_data();
    let result =
        volatility::starc(&stats.high, &stats.low, &stats.close, 16, 5, None).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            (49.147125053405766, 34.86487445831299,),
            (48.29980422258377, 34.54419449567795,),
            (47.66644210144878, 34.053557593375444,),
            (47.2375398573, 33.92646008166485,),
            (47.384068900413695, 34.595931404762084,),
            (47.48828987299221, 35.16371055425388,),
            (48.013147475000636, 36.2308542950189,),
            (47.963450950455304, 36.572550636458764,),
            (48.79866020425376, 37.55734107748453,),
            (49.55411861151658, 38.49788181572952,),
            (49.72798596369291, 38.972014036307094,),
            (50.18236190962666, 39.609637907267874,),
            (51.19588918842257, 40.696110628471956,),
            (51.294270856654094, 41.0817285940295,),
            (51.61537921230816, 41.55662090976215,),
            (52.51279361807577, 42.643207663662515,),
            (53.74036902266808, 43.623632442175676,),
            (53.979470600169776, 44.02252982707631,),
        ],
        result[16..]
    );
}

#[test]
fn test_cvi() {
    let stats = common::test_data();
    let result = volatility::cvi(&stats.high, &stats.low, 16, 2).collect::<Vec<_>>();
    assert_eq!(stats.high.len(), result.len());
    assert_eq!(
        vec![
            -12.903778070881856,
            -7.712487202835517,
            -3.1188656632364697,
            -9.341946193517892,
            -12.034597148362437,
            -12.814471005826123,
            -13.271361266019388,
            -6.439924966331101,
            -2.0112332700986557,
            -4.867518373670898,
            -4.142244814924245,
            0.7501021543131925,
            -1.5573115591711928,
            -4.07753627445564,
            -2.062093533894327,
            6.497958034939444,
            5.52228653707274,
        ],
        result[15 + 2..]
    );
}

#[test]
fn test_relative_vol() {
    let stats = common::test_data();
    let result = volatility::relative_vol(&stats.close, 6, 10).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    // NOTE: this won't match yahoo as wilder ma is n+1 and we wait for full window while yahoo
    // doesn't. over larger datasets it matches.
    assert_eq!(
        vec![
            28.460365094239744,
            26.713556078390294,
            30.72456832552668,
            28.997752032109624,
            33.13828781265631,
            31.892848868843004,
            34.49622014712077,
            37.56620902000314,
            41.13195840990312,
            39.710861993052106,
            43.70526288007951,
            48.09010520437329,
            44.653045101155534,
            48.72492734457978,
            53.15308104161818,
            50.1657395923798,
            53.718765471687156,
            58.056260543528104,
            62.43245885296475,
            56.96821515395683,
        ],
        result[5 + 9..]
    );
}

#[test]
fn test_inertia() {
    let stats = common::test_data();
    let result = volatility::inertia(&stats.close, 6, 10).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            33.846695422163236,
            37.0863925832691,
            41.457100533424686,
            46.70720708567886,
            50.28458647020866,
            54.88569652125601,
            59.86174805509194,
            60.99043440008858,
            62.713158542656664,
            66.72034985001046,
            65.77887218682976,
            66.53332690661519,
            69.28895272009414,
            73.87374050904741,
            71.82880194892938,
        ],
        result[10 + 9..]
    );
}

#[test]
fn test_chop() {
    let stats = common::test_data();
    let result = volatility::chop(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            35.50305865709021,
            29.36528182694797,
            35.868470870431715,
            46.723552778406344,
            51.86692089009291,
            50.1850201980578,
            46.525910593875786,
            50.565671194602146,
            50.87870056760242,
            50.759486846471894,
            50.521735095671616,
            51.03451151332367,
            49.217489689326946,
            47.77952890236684,
            47.7727684616789,
            46.73089304040272,
            37.49049080789503,
            37.995427183993165,
        ],
        result[16..]
    );
}

#[test]
fn test_vhf() {
    let stats = common::test_data();
    let result = volatility::vhf(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            0.5669216159971233,
            0.7107794587594408,
            0.5482664867838217,
            0.4309723520119755,
            0.40721343838432617,
            0.44011310017913846,
            0.5021691775026703,
            0.4751003004570894,
            0.4435695260250405,
            0.4595960007629394,
            0.4405808834558327,
            0.4357521288992697,
            0.4843910183060343,
            0.5122550500601386,
            0.5359587346575249,
            0.5841583342518132,
            0.7716635213608084,
            0.7671760704973449,
        ],
        result[16..]
    );
}
