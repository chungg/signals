use traquer::trend;

mod common;

#[test]
fn test_adx() {
    let stats = common::test_data();
    let result = trend::adx(&stats.high, &stats.low, &stats.close, 14, 14);
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
    let result = trend::qstick(&stats.open, &stats.close, 8);
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
fn test_cog() {
    let stats = common::test_data();
    let result = trend::cog(&stats.close, 16);
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
    let results = trend::shinohara(&stats.high, &stats.low, &stats.close, 26);
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
fn test_vortex() {
    let stats = common::test_data();
    let (vi_pos, vi_neg) = trend::vortex(&stats.high, &stats.low, &stats.close, 16);
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
fn test_vhf() {
    let stats = common::test_data();
    let result = trend::vhf(&stats.high, &stats.low, &stats.close, 16);
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
fn test_asi() {
    let stats = common::test_data();
    let result = trend::asi(&stats.open, &stats.high, &stats.low, &stats.close, 0.5);
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
    let result = trend::ulcer(&stats.close, 8);
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
fn test_supertrend() {
    let stats = common::test_data();
    let result = trend::supertrend(&stats.high, &stats.low, &stats.close, 16, 3.0);
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
fn test_rwi() {
    let stats = common::test_data();
    let result = trend::rwi(&stats.high, &stats.low, &stats.close, 16);
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
fn test_psy() {
    let stats = common::test_data();
    let result = trend::psy(&stats.close, 16);
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
    let result = trend::mass(&stats.high, &stats.low, 9, 6);
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

#[test]
fn test_keltner() {
    let stats = common::test_data();
    let result = trend::keltner(&stats.high, &stats.low, &stats.close, 16);
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
        result[1..]
    );
}

#[test]
fn test_gri() {
    let stats = common::test_data();
    let result = trend::gri(&stats.high, &stats.low, 16);
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
        result
    );
}

#[test]
fn test_tr() {
    let stats = common::test_data();
    let result = trend::tr(&stats.high, &stats.low, &stats.close);
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
    let result = trend::atr(&stats.high, &stats.low, &stats.close, 16);
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
fn test_typical() {
    let stats = common::test_data();
    let result = trend::typical(&stats.high, &stats.low, &stats.close, 16);
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
