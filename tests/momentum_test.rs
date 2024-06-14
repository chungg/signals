use traquer::momentum;

mod common;

#[test]
fn test_rsi() {
    let stats = common::test_data();
    let result = momentum::rsi(&stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            46.128106720103226,
            44.63367886613521,
            47.00425158432756,
            46.30758159352003,
            47.45475914349044,
            48.24782813294231,
            48.597941941188665,
            47.8652658468592,
            51.205748434634394,
            52.13878031503008,
            49.934061780601795,
            51.64109828171606,
            53.76628012209769,
            52.33740047051155,
            54.32827223456175,
            55.69669831790614,
            58.06858163571402,
            54.77626809114967
        ],
        &result
    ));
}

#[test]
fn test_macd() {
    let stats = common::test_data();
    let result = momentum::macd(&stats.close, 12, 26).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -2.217893848328025,
            -2.012936386965073,
            -1.7145457437113834,
            -1.316039757031355,
            -1.0733808769373852,
            -0.7410026847983033,
            -0.3820054099132051,
            0.06315775884193187,
            0.24329207900761673,
        ],
        &result
    ));
}

#[test]
fn test_cmo() {
    let stats = common::test_data();
    let result = momentum::cmo(&stats.close, 16);
    assert_eq!(
        vec![
            -7.743786559793555,
            -40.5065764761353,
            -48.55718616179731,
            -40.50419392251485,
            -21.175095245365554,
            -9.902549550882728,
            -26.391891770751375,
            -18.232928699810707,
            -1.3123299513601174,
            -4.972814726212496,
            -6.534210047973057,
            3.3257784266638946,
            22.112731584577258,
            8.795331918339102,
            25.684922447035984,
            42.304220440657446,
            44.454063824420864,
            43.61402085815939,
        ],
        result.collect::<Vec<f64>>()
    );
}

#[test]
fn test_cfo() {
    let stats = common::test_data();
    let result = momentum::cfo(&stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -1.0827212473925485,
            6.489853839461397,
            3.26724075507739,
            6.494631425837021,
            3.2897604385439467,
            5.702554284788103,
            7.356890145930599,
            6.369370755372636,
            4.116781344839304,
            8.361441355822752,
            7.121133194554285,
            1.5079786242835373,
            2.4028493038253407,
            3.947620573327679,
            -0.33454143227586053,
            0.9372834260723698,
            1.456301120200947,
            2.8509113301892537,
            -1.9095010899369487
        ],
        &result
    ));
}

#[test]
fn test_elder_ray() {
    let stats = common::test_data();
    let results: (Vec<f64>, Vec<f64>) =
        momentum::elder_ray(&stats.high, &stats.low, &stats.close, 16).unzip();
    assert_eq!(stats.close.len(), results.1.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -6.323126792907715,
            -5.426287538865033,
            -5.97348870868089,
            -3.8769004733650263,
            -1.5737349488532146,
            -2.4380034539800874,
            -0.690589604456811,
            -0.5922851156483517,
            -1.6737827615957386,
            1.753899378196202,
            4.2190289169688455,
            2.1200258021967784,
            2.9185518732331417,
            4.514192005348761,
            3.396935010966615,
            4.217882038054185,
            4.2369573334537165,
            8.442607464131385,
            4.079358351350422,
        ],
        &results.0
    ));
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -8.743124961853027,
            -8.246287233689252,
            -7.878487487977765,
            -8.656903067359167,
            -5.23373861096259,
            -4.498004827271103,
            -2.9285923815564203,
            -2.1122855734120236,
            -3.4637836771230823,
            -1.7951004081807511,
            1.189026322974705,
            -0.48497374003954974,
            -0.18145041558521768,
            0.5951932870870351,
            0.936935926493959,
            1.0978831061694194,
            1.3069570282779353,
            2.9826083796587284,
            1.229359877229328,
        ],
        &results.1
    ));
}

#[test]
fn test_wpr() {
    let stats = common::test_data();
    let result = momentum::wpr(&stats.high, &stats.low, &stats.close, 16);
    assert_eq!(
        vec![
            -99.09142622449394,
            -94.88476784384058,
            -98.7016646516565,
            -83.45322691468988,
            -80.33424822308433,
            -66.49998256138393,
            -60.92856270926339,
            -58.24332819489061,
            -56.88925190670342,
            -31.699065254207774,
            -24.51395669497002,
            -38.50825188642196,
            -26.019074805435594,
            -15.778329765623885,
            -24.403940042613893,
            -12.779540060870623,
            -7.164869527394986,
            -21.111723928174033,
            -32.930944560522704,
        ],
        result.collect::<Vec<f64>>()
    );
}

#[test]
fn test_ppo() {
    let stats = common::test_data();
    let result = momentum::ppo(&stats.volume, 10, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -35.378723807894985,
            -37.993168058882375,
            -39.524346350340444,
            -40.41967020222986,
            -40.51097271301698,
            -42.086823387150005,
            -42.30015209438188,
            -43.383528771209576,
            -43.81409605428357,
            -40.648471039972534,
            -37.7876496415686,
            -37.39351505516741,
            -37.136103488993875,
            -34.28067604316157,
            -35.12026222042619,
            -32.44673522414948,
            -18.294669010949182,
            2.308566455542005,
            -0.92080395315155,
        ],
        &result
    ));
}

#[test]
fn test_apo() {
    let stats = common::test_data();
    let result = momentum::apo(&stats.close, 10, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -3.066861454520314,
            -2.976616647434078,
            -2.974212899316754,
            -2.745221486406585,
            -2.5750892679175763,
            -2.3227459270799784,
            -2.040835124550796,
            -1.7724858153941057,
            -1.5857634044718836,
            -1.2258138130321825,
            -0.885191183160849,
            -0.7384274508922886,
            -0.5242732104821357,
            -0.24166405735277152,
            -0.1004936005485888,
            0.10816949510991947,
            0.3290082798737828,
            0.6097419848129704,
            0.6642876636596782,
        ],
        &result
    ));
}

#[test]
fn test_pmo() {
    let stats = common::test_data();
    let result = momentum::pmo(&stats.close, 10, 6).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -3.8697582054697164,
            -5.025517442390475,
            -7.927443373077454,
            -5.8296927191470305,
            -5.346827455789476,
            -3.3128123363211874,
            -0.9881779822960846,
            0.8220576750326956,
            0.994556201816032,
            4.690466630412156,
            7.578705354256442,
            6.807587490832155,
            7.686581402943451,
            9.861687558383029,
            9.466155404271682,
            10.558761806589173,
            11.864895189635732,
            14.115021635278136,
            12.569334850393318,
        ],
        &result
    ));
}

#[test]
fn test_ultimate() {
    let stats = common::test_data();
    let result = momentum::ultimate(&stats.high, &stats.low, &stats.close, 6, 12, 24);
    assert_eq!(
        vec![
            52.64489292919164,
            51.59059656807282,
            46.03014177584667,
            46.83402417416914,
            47.63501864800235,
            43.80674742529631,
            38.16680505680669,
            44.10353752395525,
            44.154676988833835,
            42.65072465563253,
        ],
        result.collect::<Vec<f64>>()
    );
}

#[test]
fn test_pgo() {
    let stats = common::test_data();
    let result = momentum::pgo(&stats.high, &stats.low, &stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -1.1728195008646218,
            -1.3831996312942632,
            -0.669537387649706,
            -0.6555101010723227,
            -0.37389208904655474,
            -0.18480884776195328,
            -0.01324917609586888,
            -0.11871191726864071,
            0.6407704612678665,
            0.8895192850294852,
            0.4619045740578769,
            0.8450928215439151,
            1.234792786595683,
            0.9512531985339747,
            1.3179726777354788,
            1.5016968220594469,
            1.7570721475194715,
            1.0410621667725752,
        ],
        &result
    ));
}

#[test]
fn test_si() {
    let stats = common::test_data();
    let result = momentum::si(&stats.open, &stats.high, &stats.low, &stats.close, 0.5);
    assert_eq!(
        vec![
            1863.9824746176116,
            654.8878036623194,
            -1104.4095193965052,
            -1214.2944568105527,
            -487.3407354098372,
            427.5088169169998,
            -223.860381128408,
            -110.7121065452446,
            162.75807618131765,
            -98.12145222473158,
            -101.13846512002326,
            -403.1845902647818,
            283.1891569876117,
            -199.95462669308907,
            -326.86640146468073,
            63.131576090683794,
            -253.67664708736052,
            215.41843387212208,
            2.1629182406584904,
            143.69919207899204,
            115.4626169352983,
            51.354553395932655,
            -18.304465830729768,
            429.0203486740835,
            141.26847935246593,
            -183.79992761728568,
            160.03660663696868,
            235.8667734660081,
            -90.16470772181825,
            162.16835332629032,
            144.83405393167723,
            59.009650555611834,
            -321.26018448483165,
        ],
        result.collect::<Vec<f64>>()
    );
}

#[test]
fn test_trix() {
    let stats = common::test_data();
    let result = momentum::trix(&stats.close, 7).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -1.7609812348121436,
            -1.58700358125189,
            -1.3595873824994853,
            -1.1099628496419054,
            -0.8955729429209658,
            -0.6091528694171965,
            -0.2949587326281726,
            -0.07920030554104754,
            0.1135712345224751,
            0.32952700679764474,
            0.4769166955410566,
            0.6219613185170387,
            0.7718438325710452,
            0.9560958810856369,
            1.0335513961870586,
        ],
        &result
    ));
}

#[test]
fn test_tii_even() {
    let stats = common::test_data();
    let result = momentum::tii(&stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            12.365592243625398,
            36.838871014658466,
            53.81832516603384,
            77.25510120423154,
            91.83894521268712,
            97.2706485470137,
            98.02691957272636,
            100.0,
            100.0,
            100.0,
        ],
        &result
    ));
}

#[test]
fn test_tii_odd() {
    let stats = common::test_data();
    let result = momentum::tii(&stats.close, 15).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.6686354433953655,
            0.9316341618307428,
            17.111743788068527,
            44.82345282880815,
            61.21209452789354,
            83.24057181663895,
            96.05752982533481,
            98.58952713255982,
            98.83515680783523,
            100.0,
            100.0,
            100.0,
        ],
        &result
    ));
}

#[test]
fn test_stochastic() {
    let stats = common::test_data();
    let results: (Vec<f64>, Vec<f64>) =
        momentum::stochastic(&stats.high, &stats.low, &stats.close, 16).unzip();
    assert_eq!(stats.close.len(), results.0.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            2.4407137600029882,
            7.653446863271012,
            12.503620070189761,
            23.237514100280617,
            30.745735502089445,
            38.10937551148735,
            41.31295239638086,
            51.05611821473273,
            62.299242048039595,
            68.42624205480008,
            70.31957220439081,
            73.23144784750619,
            77.93288512877554,
            82.34606337696387,
            85.21721678970684,
            86.31462216118678,
            79.59748732796943,
        ],
        &results.0
    ));
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            7.532593564487921,
            14.464860344580464,
            22.162289890853273,
            30.69754170461914,
            36.72268780331922,
            43.49281537420031,
            51.556104219717724,
            60.5938674391908,
            67.01501876907683,
            70.65908736889902,
            73.82796839355751,
            77.8367987844152,
            81.83205509848209,
            84.62596744261917,
            83.70977542628769,
        ],
        &results.1
    ));
}

#[test]
fn test_stc() {
    let stats = common::test_data();
    let result = momentum::stc(&stats.close, 3, 6, 12).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            75.0,
            87.5,
            93.75,
            96.875,
            98.4375,
            99.21875,
            99.609375,
            99.8046875,
            99.90234375,
            99.951171875,
            99.9755859375,
            99.98779296875,
            99.993896484375,
            99.9969482421875,
            99.99847412109375,
            99.99923706054688,
            49.99961853027344,
        ],
        &result
    ));
}

#[test]
fn test_relative_vigor() {
    let stats = common::test_data();
    let result = momentum::relative_vigor(&stats.open, &stats.high, &stats.low, &stats.close, 16)
        .collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -0.153337143981914,
            -0.18751486156482663,
            -0.1655477409598533,
            -0.08107480302944187,
            -0.022096084872983195,
            0.0005334258559703508,
            0.015530276750573162,
            0.042209478128338834,
            0.07104433745828966,
            0.08624376262637506,
            0.11491458879556057,
            0.14208003040068837,
            0.15066113435945197,
            0.16153832477598623,
            0.17222662984562823,
            0.1687163540078826,
        ],
        &result
    ));
}

#[test]
fn test_fisher() {
    let stats = common::test_data();
    let result = momentum::fisher(&stats.high, &stats.low, 16);
    assert_eq!(
        vec![
            -0.34282825441539394,
            -0.7913738721291064,
            -1.2614929493509068,
            -1.7172725714330925,
            -1.8081828617352156,
            -1.753446111207961,
            -1.5065909817285177,
            -1.23773948174167,
            -1.0987825209904734,
            -0.7590158390343599,
            -0.18553810038469534,
            0.21092693964146053,
            0.5573750790188244,
            0.9935618421240107,
            1.412718060950448,
            1.8482793337505146,
            2.282834484617007,
            2.7111315248922243,
            2.457215146525715,
        ],
        result.collect::<Vec<f64>>()
    );
}

#[test]
fn test_rainbow() {
    let stats = common::test_data();
    let result = momentum::rainbow(&stats.close, 3, 6).collect::<Vec<(f64, f64)>>();
    assert_eq!(stats.close.len(), result.len());
    assert_eq!(
        vec![
            (-15.581553207931039, 167.79131131051022),
            (18.51712436379442, 107.11290269481944),
            (28.579723816754267, 74.98003039933955),
            (23.56911341477455, 92.71147191044699),
            (74.39265282061145, 56.69518580646786),
            (93.39951200265162, 83.40508885098447),
            (45.35273805467191, 107.52161550781406),
            (72.38472533173207, 109.41901142040155),
            (78.21191198865775, 87.31589625324459),
            (76.72283481160511, 154.7976271899086),
            (92.28370944292567, 141.68164799833704),
            (82.03271695479356, 110.4243596611003),
            (96.19784933644493, 117.51887397606791),
            (53.817510802467865, 130.69353809849056),
        ],
        result[(3 - 1) * 10..]
    );
}

#[test]
fn test_coppock() {
    let stats = common::test_data();
    let result = momentum::coppock(&stats.close, 10, 11, 14).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -23.219023231862437,
            -17.657274623448355,
            -10.580507093014912,
            -3.8822210243130435,
            1.2650769588645043,
            8.805290715833202,
            14.099190228788462,
            19.756470919186093,
            25.749612602464033,
            30.587155565412456,
            32.92949971385037,
        ],
        &result
    ));
}

#[test]
fn test_roc() {
    let stats = common::test_data();
    let result = momentum::roc(&stats.close, 16);
    assert_eq!(
        vec![
            -13.043478260869568,
            -31.2040135208693,
            -39.84027368797499,
            -27.75757364380411,
            -17.112730492564665,
            -8.549054167537983,
            -15.123886923149922,
            -8.17195151186294,
            -7.273914602774845,
            -4.478555522732619,
            0.28168418010792173,
            -1.2444474962022611,
            9.004516278896514,
            7.118800966858307,
            10.338298756942965,
            20.675001144409187,
            20.07778866984824,
            31.222884678686967,
            18.156751501922706,
        ],
        result.collect::<Vec<f64>>()
    );
}

#[test]
fn test_bal_power() {
    let stats = common::test_data();
    let result = momentum::bal_power(&stats.open, &stats.high, &stats.low, &stats.close, 16)
        .collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -0.26684921210684515,
            -0.20875516800799515,
            -0.2601570855656015,
            -0.15768228202067733,
            -0.12916674954536495,
            -0.04772285587508085,
            -0.0026824931149390316,
            0.03555890939531569,
            0.08461250855298905,
            0.15587404428157808,
            0.14646620080108347,
            0.08271787843106115,
            0.11131658687058162,
            0.13094196564772775,
            0.029453819189000877,
            0.07425409451434198,
            0.1325731996247852,
            0.03940688314737367,
            -0.04551836629903973
        ],
        &result
    ));
}

#[test]
fn test_disparity() {
    let stats = common::test_data();
    let result = momentum::disparity(&stats.close, 16).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -17.394840214421464,
            -13.51017275854007,
            -15.905389719955151,
            -9.317950518602139,
            -9.963355513492703,
            -6.594387721366419,
            -4.314113491559456,
            -3.1692377716277673,
            -4.205872976365648,
            2.215988552748409,
            3.625148648066157,
            -0.43682479501304494,
            2.45314121281467,
            5.706822803792927,
            2.9510886636129685,
            5.734418778479189,
            7.174525059740964,
            10.023267360859805,
            4.836935524987116
        ],
        &result
    ));
}

#[test]
fn test_qstick() {
    let stats = common::test_data();
    let result = momentum::qstick(&stats.open, &stats.close, 8).collect::<Vec<_>>();
    assert_eq!(stats.close.len(), result.len());
    assert!(common::vec_eq(
        &vec![
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -0.5343747138977051,
            -0.6200696627298992,
            -0.06449838920875839,
            -0.4301652103785134,
            -0.6456843916008229,
            -0.9266433817811088,
            -0.16961162088834514,
            -0.39858698578859136,
            -0.7411229059931231,
            -0.43420684029499856,
            -0.6110505106591309,
            0.17362782140791555,
            0.20393305515971555,
            0.4163923423269142,
            0.49052737736537766,
            0.4904105553879152,
            0.5614307371441598,
            0.9811129650986694,
            0.8141988711292603,
            0.40437632332406004,
            0.5389598373032619,
            0.6614132406998461,
            0.11443269008642548,
            0.3734473765776365,
            0.6615706225920679,
            -0.2854447322330791,
            -0.6542355716305024,
        ],
        &result
    ));
}

#[test]
fn test_cog() {
    let stats = common::test_data();
    let result = momentum::cog(&stats.close, 16);
    assert_eq!(
        vec![
            -8.967512936942779,
            -9.068163820345244,
            -9.040879286087515,
            -8.897050544657091,
            -8.80553442078012,
            -8.772365854633845,
            -8.759057548354765,
            -8.683880858831056,
            -8.646508895454593,
            -8.583784959158981,
            -8.489125991456865,
            -8.429902985473813,
            -8.366876603883997,
            -8.322698920592734,
            -8.267741798560767,
            -8.227747449061596,
            -8.213709147564295,
            -8.179093436115151,
            -8.204185273664265,
        ],
        result.collect::<Vec<f64>>()
    );
}
#[test]
fn test_psych() {
    let stats = common::test_data();
    let result = momentum::psych(&stats.close, 16);
    assert_eq!(
        vec![
            37.5, 31.25, 31.25, 31.25, 37.5, 43.75, 43.75, 43.75, 50.0, 50.0, 50.0, 56.25, 62.5,
            56.25, 62.5, 68.75, 68.75, 68.75,
        ],
        result.collect::<Vec<f64>>()
    );
}
