use traquer::indicator;

mod common;

#[test]
fn test_adx() {
    let stats = common::test_data();
    let result = indicator::adx(&stats.high, &stats.low, &stats.close, 14, 14);
    assert_eq!(
        vec![
            31.860139412194698,
            30.900411343329193,
            29.846209753924423,
            28.926644263612655,
            29.10834871990969,
            29.939197830151386,
            29.09419668802638,
            30.29237400866025,
            29.60030469615574,
            28.537858280042748,
            32.473148424041746,
            34.982949940305744,
            33.47283006287883,
            33.22624512306407,
            34.306046041056824,
            32.83133865336625,
            33.02567539621716,
            32.10723238925509,
            37.32862815236502,
            35.23477297887128,
        ],
        result.0
    );
    assert_eq!(
        vec![
            26.730133235533003,
            28.809316234232448,
            28.26198015278067,
            28.174104325301958,
            26.454545579221207,
            25.18695046089198,
            24.47607630765512,
            23.69367885417301,
            23.152365451974088,
            24.818266407121218,
            23.34724730276131,
            22.051913999559698,
            23.91704820756414,
            22.602128450065592,
            21.10029388200336,
            20.193259616585753,
            19.03617008379953,
            18.027067079868925,
            16.20809033889095,
            17.880973133699126,
        ],
        result.1
    );
    assert_eq!(
        vec![
            10.317020581019671,
            11.282545552195204,
            12.179104454001054,
            13.228521011257213,
            14.289690573973955,
            16.086895874937387,
            17.27152240413718,
        ],
        result.2
    );
}

#[test]
fn test_qstick() {
    let stats = common::test_data();
    let result = indicator::qstick(&stats.open, &stats.close, 8);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_rsi() {
    let stats = common::test_data();
    let result = indicator::rsi(&stats.close, 16);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_macd() {
    let stats = common::test_data();
    let result = indicator::macd(&stats.close, 12, 26);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_cmo() {
    let stats = common::test_data();
    let result = indicator::cmo(&stats.close, 16);
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
        result
    );
}

#[test]
fn test_cog() {
    let stats = common::test_data();
    let result = indicator::cog(&stats.close, 16);
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
        result
    );
}

#[test]
fn test_shinohara() {
    let stats = common::test_data();
    let results = indicator::shinohara(&stats.high, &stats.low, &stats.close, 26);
    assert_eq!(
        vec![
            119.72458721822203,
            95.38515682324336,
            68.70621869411681,
            85.35865719588207,
            119.54886405652606,
            139.72926160563833,
            150.77427373025645,
            150.9041867287,
        ],
        results.0
    );
    assert_eq!(
        vec![
            130.9311144714237,
            126.54955949420238,
            167.3688494557726,
            146.27994868199906,
            142.35182619352173,
            125.70722906999697,
            118.00938106154118,
            143.57856018147575,
            133.74958218587676,
        ],
        results.1
    );
}

#[test]
fn test_elder_ray() {
    let stats = common::test_data();
    let results = indicator::elder_ray(&stats.high, &stats.low, &stats.close, 16);
    assert_eq!(
        vec![
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
        results.0
    );
    assert_eq!(
        vec![
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
        results.1
    );
}

#[test]
fn test_cvi() {
    let stats = common::test_data();
    let result = indicator::cvi(&stats.high, &stats.low, 16, 2);
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
        result
    );
}

#[test]
fn test_wpr() {
    let stats = common::test_data();
    let result = indicator::wpr(&stats.high, &stats.low, &stats.close, 16);
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
        result
    );
}

#[test]
fn test_vortex() {
    let stats = common::test_data();
    let (vi_pos, vi_neg) = indicator::vortex(&stats.high, &stats.low, &stats.close, 16);
    assert_eq!(
        vec![
            0.8610723090930696,
            0.8159089697456218,
            0.5782198030623583,
            0.7200793857247078,
            0.8358118890986928,
            0.9355813847576413,
            0.9484126915125689,
            0.8489016637989781,
            0.9131108818882286,
            0.9790387062979787,
            0.9343265196480259,
            0.9443134085341751,
            1.0302488811514465,
            1.0364553935724832,
            1.0553301509762798,
            1.1219923299032897,
            1.161732264987062,
            1.1478332770638693,
        ],
        vi_pos
    );
    assert_eq!(
        vec![
            1.029701151945177,
            1.1817046446451998,
            1.3803206728528217,
            1.238892593460365,
            1.1850444607043855,
            1.061167502645104,
            1.111042759028416,
            1.1313347365841422,
            0.9951327158267141,
            0.9280527122256887,
            0.9901265627745084,
            0.9313767804581332,
            0.8402113281909229,
            0.891932290028499,
            0.8280623772231498,
            0.7860652938018367,
            0.6974842970716879,
            0.7557323147066469,
        ],
        vi_neg
    );
}

#[test]
fn test_ppo() {
    let stats = common::test_data();
    let result = indicator::ppo(&stats.volume, 10, 16);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_apo() {
    let stats = common::test_data();
    let result = indicator::apo(&stats.close, 10, 16);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_dpo() {
    let stats = common::test_data();
    let result = indicator::dpo(&stats.close, 16);
    assert_eq!(
        vec![
            2.0268754959106445,
            -1.129373550415039,
            -1.0500013828277588,
            2.191876173019409,
            1.8362512588500977,
            1.141249656677246,
            -1.5718750953674316,
            1.324373483657837,
            -0.6518747806549072,
            -2.9000003337860107,
            -1.6800007820129395,
            -3.5431268215179443,
            -1.048123836517334,
            -2.2387490272521973,
            -1.2106242179870605,
            -0.8056254386901855,
            -1.0631237030029297,
            -2.404374599456787,
            -0.057187557220458984,
        ],
        result
    );
}

#[test]
fn test_vhf() {
    let stats = common::test_data();
    let result = indicator::vhf(&stats.high, &stats.low, &stats.close, 16);
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
        result
    );
}

#[test]
fn test_ultimate() {
    let stats = common::test_data();
    let result = indicator::ultimate(&stats.high, &stats.low, &stats.close, 6, 12, 24);
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
        result
    );
}

#[test]
fn test_pgo() {
    let stats = common::test_data();
    let result = indicator::pgo(&stats.high, &stats.low, &stats.close, 16);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_si() {
    let stats = common::test_data();
    let result = indicator::si(&stats.open, &stats.high, &stats.low, &stats.close, 0.5);
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
        result
    );
}

#[test]
fn test_asi() {
    let stats = common::test_data();
    let result = indicator::asi(&stats.open, &stats.high, &stats.low, &stats.close, 0.5);
    assert_eq!(
        vec![
            1863.9824746176116,
            2518.870278279931,
            1414.4607588834258,
            200.16630207287312,
            -287.17443333696406,
            140.33438358003576,
            -83.52599754837223,
            -194.2381040936168,
            -31.480027912299164,
            -129.60148013703076,
            -230.73994525705402,
            -633.9245355218358,
            -350.7353785342241,
            -550.6900052273131,
            -877.5564066919939,
            -814.4248306013101,
            -1068.1014776886707,
            -852.6830438165487,
            -850.5201255758901,
            -706.8209334968981,
            -591.3583165615997,
            -540.0037631656671,
            -558.3082289963969,
            -129.2878803223134,
            11.980599030152518,
            -171.81932858713316,
            -11.782721950164472,
            224.08405151584364,
            133.9193437940254,
            296.08769712031574,
            440.92175105199294,
            499.93140160760476,
            178.6712171227731,
        ],
        result
    );
}

#[test]
fn test_ulcer() {
    let stats = common::test_data();
    let result = indicator::ulcer(&stats.close, 8);
    assert_eq!(
        vec![
            20.73223668909602,
            19.094435269396094,
            16.649343658592862,
            14.662110635179326,
            13.083755073024895,
            12.845054285539643,
            11.60084753454864,
            10.833635700777663,
            10.094706264836718,
            8.405227208060785,
            6.91887557627602,
            4.390517821635424,
            3.822504042165251,
            2.511303218451697,
            1.5486160074003668,
            1.7365186635053362,
            1.7365186635053362,
            1.6390653783421978,
            1.6390653783421978,
            2.187009055695484,
        ],
        result
    );
}

#[test]
fn test_tr() {
    let stats = common::test_data();
    let result = indicator::tr(&stats.high, &stats.low, &stats.close);
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
        result
    );
}

#[test]
fn test_atr() {
    let stats = common::test_data();
    let result = indicator::atr(&stats.high, &stats.low, &stats.close, 16);
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
        result
    );
}

#[test]
fn test_hlc3() {
    let stats = common::test_data();
    let result = indicator::hlc3(&stats.high, &stats.low, &stats.close, 16);
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
        result
    );
}

#[test]
fn test_trix() {
    let stats = common::test_data();
    let result = indicator::trix(&stats.close, 7);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_tii_even() {
    let stats = common::test_data();
    let result = indicator::tii(&stats.close, 16);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_tii_odd() {
    let stats = common::test_data();
    let result = indicator::tii(&stats.close, 15);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_supertrend() {
    let stats = common::test_data();
    let result = indicator::supertrend(&stats.high, &stats.low, &stats.close, 16, 3.0);
    assert_eq!(
        vec![
            22.87718629837036,
            22.87718629837036,
            22.87718629837036,
            25.36115028045606,
            25.5548271386142,
            27.535275836318306,
            28.482134516844127,
            28.482134516844127,
            30.372852510605856,
            33.54470377638434,
            33.54470377638434,
            33.54470377638434,
            34.55477737989572,
            34.70432339242238,
            35.73655158775987,
            36.52801923545023,
            39.78407957956028,
            39.78407957956028,
        ],
        result
    );
}

#[test]
fn test_stochastic() {
    let stats = common::test_data();
    let results = indicator::stochastic(&stats.high, &stats.low, &stats.close, 16);
    assert_eq!(
        vec![
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
        results.0
    );
    assert_eq!(
        vec![
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
        results.1
    );
}

#[test]
fn test_stc() {
    let stats = common::test_data();
    let result = indicator::stc(&stats.close, 3, 6, 12);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_relative_vol() {
    let stats = common::test_data();
    let result = indicator::relative_vol(&stats.close, 6, 10);
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
        result
    );
}

#[test]
fn test_relative_vigor() {
    let stats = common::test_data();
    let result = indicator::relative_vigor(&stats.open, &stats.high, &stats.low, &stats.close, 16);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_rwi() {
    let stats = common::test_data();
    let result = indicator::rwi(&stats.high, &stats.low, &stats.close, 16);
    assert_eq!(
        vec![
            (0.9498065403643551, 1.7412617466727387,),
            (0.4556737540683345, 1.7685523890567207,),
            (1.4057386211737706, 1.535472296106406,),
            (1.3556485021734983, 0.9465833084480927,),
            (0.8746671860087694, 0.9071038763955941,),
            (1.7233013034774831, 0.754581037874933,),
            (1.1384779666654379, 0.7193916946064202,),
            (0.8296249568590155, 2.0526327677377094,),
            (2.325650322092707, 0.5940724651955457,),
            (1.8773568877196494, 0.28738638191882215,),
            (1.1197267076507562, 1.3710140568798879,),
            (1.3627638621436993, 0.9996428423372017,),
            (1.5457049579330462, 0.6073614247767936,),
            (1.1223183991371906, 0.8673129298714549,),
            (1.4756091887717602, 0.7926835319767893,),
            (1.1929354504792102, 0.7647051876347102,),
            (2.6484633151147925, 0.21501740699560723,),
            (1.111277033009784, 1.1996531008663207,),
        ],
        result
    );
}

#[test]
fn test_fisher() {
    let stats = common::test_data();
    let result = indicator::fisher(&stats.high, &stats.low, 16);
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
        result
    );
}

#[test]
fn test_rainbow() {
    let stats = common::test_data();
    let result = indicator::rainbow(&stats.close, 3, 6);
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
        result
    );
}

#[test]
fn test_coppock() {
    let stats = common::test_data();
    let result = indicator::coppock(&stats.close, 10, 11, 14);
    assert_eq!(
        vec![
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
        result
    );
}

#[test]
fn test_psy() {
    let stats = common::test_data();
    let result = indicator::psy(&stats.close, 16);
    assert_eq!(
        vec![
            37.5, 31.25, 31.25, 31.25, 37.5, 43.75, 43.75, 43.75, 50.0, 50.0, 50.0, 56.25, 62.5,
            56.25, 62.5, 68.75, 68.75, 68.75,
        ],
        result
    );
}

#[test]
fn test_mass() {
    let stats = common::test_data();
    let result = indicator::mass(&stats.high, &stats.low, 9, 6);
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
        result
    );
}
