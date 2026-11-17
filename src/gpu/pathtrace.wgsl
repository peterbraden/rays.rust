struct type_10 {
    member: vec4<u32>,
    member_1: vec4<u32>,
    member_2: vec4<u32>,
    member_3: vec4<u32>,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<f32>,
    member_7: vec4<u32>,
    member_8: vec4<f32>,
    member_9: vec4<f32>,
}

struct type_12 {
    member: type_10,
}

struct type_14 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<u32>,
}

struct type_17 {
    member: array<type_14>,
}

struct type_19 {
    member: vec3<f32>,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: u32,
}

struct type_22 {
    member: array<type_19>,
}

struct type_24 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<f32>,
}

struct type_27 {
    member: array<type_24>,
}

struct type_29 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_32 {
    member: array<type_29>,
}

struct type_36 {
    member: array<f32>,
}

struct type_38 {
    member: vec4<f32>,
    member_1: vec4<f32>,
}

struct type_40 {
    member: array<type_38>,
}

struct type_43 {
    member: array<vec4<f32>>,
}

struct type_49 {
    member: f32,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
}

struct type_52 {
    member: f32,
    member_1: f32,
}

struct type_53 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_59 {
    member: vec3<f32>,
    member_1: f32,
}

struct type_60 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<uniform> global_1: type_12;
@group(0) @binding(1) 
var<storage> global_2: type_17;
@group(0) @binding(2) 
var<storage> global_3: type_22;
@group(0) @binding(3) 
var<storage> global_4: type_27;
@group(0) @binding(4) 
var<storage> global_5: type_32;
@group(0) @binding(5) 
var<storage> global_6: type_36;
@group(0) @binding(6) 
var<storage> global_7: type_40;
@group(0) @binding(7) 
var<storage, read_write> global_8: type_43;

fn function_() {
    var local: array<u32, 100>;
    var local_1: array<u32, 100>;
    var phi_13272_: u32;
    var phi_13192_: bool;
    var phi_334_: u32;
    var phi_337_: vec3<f32>;
    var phi_339_: vec3<f32>;
    var phi_341_: f32;
    var phi_343_: vec3<f32>;
    var phi_345_: u32;
    var phi_347_: bool;
    var phi_349_: bool;
    var phi_351_: vec3<f32>;
    var phi_372_: u32;
    var phi_375_: u32;
    var phi_377_: u32;
    var phi_379_: vec4<f32>;
    var phi_381_: type_49;
    var phi_7908_: bool;
    var phi_7938_: bool;
    var phi_7885_: type_52;
    var phi_7888_: type_52;
    var phi_7889_: bool;
    var phi_7894_: type_52;
    var phi_7995_: bool;
    var phi_8010_: bool;
    var phi_8025_: bool;
    var phi_8040_: bool;
    var phi_7972_: type_52;
    var phi_7975_: type_52;
    var phi_7976_: bool;
    var phi_7981_: type_52;
    var phi_8082_: bool;
    var phi_8097_: bool;
    var phi_8112_: bool;
    var phi_8127_: bool;
    var phi_8059_: type_52;
    var phi_8062_: type_52;
    var phi_8063_: bool;
    var phi_8068_: type_52;
    var phi_7860_: bool;
    var phi_7863_: bool;
    var phi_7864_: bool;
    var phi_795_: u32;
    var phi_796_: u32;
    var phi_797_: u32;
    var phi_798_: u32;
    var phi_799_: u32;
    var phi_800_: u32;
    var phi_801_: u32;
    var phi_802_: u32;
    var phi_803_: u32;
    var phi_684_: vec4<f32>;
    var phi_685_: vec4<f32>;
    var phi_686_: bool;
    var phi_690_: vec4<f32>;
    var phi_691_: bool;
    var phi_595_: bool;
    var phi_632_: bool;
    var phi_638_: f32;
    var phi_648_: vec4<f32>;
    var phi_649_: vec4<f32>;
    var phi_650_: bool;
    var phi_654_: vec4<f32>;
    var phi_655_: bool;
    var phi_659_: vec4<f32>;
    var phi_660_: bool;
    var phi_692_: vec4<f32>;
    var phi_693_: bool;
    var phi_521_: vec4<f32>;
    var phi_522_: vec4<f32>;
    var phi_523_: bool;
    var phi_527_: vec4<f32>;
    var phi_528_: bool;
    var phi_529_: vec4<f32>;
    var phi_530_: bool;
    var phi_531_: bool;
    var phi_539_: vec4<f32>;
    var phi_540_: bool;
    var phi_541_: vec4<f32>;
    var phi_542_: bool;
    var phi_543_: bool;
    var phi_551_: vec4<f32>;
    var phi_552_: bool;
    var phi_702_: vec4<f32>;
    var phi_703_: bool;
    var phi_724_: type_49;
    var phi_726_: type_49;
    var phi_373_: u32;
    var phi_376_: u32;
    var phi_378_: u32;
    var phi_380_: vec4<f32>;
    var phi_382_: type_49;
    var phi_805_: type_49;
    var phi_806_: bool;
    var phi_807_: bool;
    var phi_13177_: bool;
    var phi_7745_: bool;
    var phi_7743_: type_49;
    var local_2: type_49;
    var local_3: vec4<f32>;
    var phi_975_: type_53;
    var local_4: vec4<f32>;
    var local_5: vec4<f32>;
    var phi_934_: vec3<f32>;
    var phi_935_: vec3<f32>;
    var phi_976_: type_53;
    var local_6: type_49;
    var phi_981_: type_49;
    var phi_982_: type_49;
    var phi_987_: type_49;
    var phi_1135_: vec3<f32>;
    var phi_1138_: vec3<f32>;
    var phi_1140_: u32;
    var phi_1158_: f32;
    var phi_1164_: f32;
    var phi_1171_: f32;
    var phi_1136_: vec3<f32>;
    var phi_1139_: vec3<f32>;
    var phi_1141_: u32;
    var local_7: vec3<f32>;
    var local_8: vec3<f32>;
    var local_9: vec3<f32>;
    var local_10: vec3<f32>;
    var phi_1317_: vec3<f32>;
    var phi_1318_: vec3<f32>;
    var phi_1319_: vec3<f32>;
    var phi_1320_: vec3<f32>;
    var phi_1351_: i32;
    var phi_1354_: u32;
    var phi_1408_: bool;
    var phi_1414_: bool;
    var phi_1421_: u32;
    var phi_1423_: i32;
    var phi_1424_: u32;
    var phi_1352_: i32;
    var phi_1355_: u32;
    var phi_1426_: bool;
    var local_11: u32;
    var phi_1471_: vec3<f32>;
    var phi_1488_: vec3<f32>;
    var phi_8156_: bool;
    var phi_1519_: f32;
    var phi_1631_: vec3<f32>;
    var phi_1637_: f32;
    var phi_8229_: bool;
    var phi_8218_: f32;
    var phi_1727_: vec3<f32>;
    var phi_1728_: vec3<f32>;
    var phi_1729_: vec3<f32>;
    var phi_13398_: u32;
    var phi_1731_: vec3<f32>;
    var phi_1732_: vec3<f32>;
    var phi_1733_: vec3<f32>;
    var phi_1734_: bool;
    var phi_1749_: vec3<f32>;
    var phi_1751_: vec3<f32>;
    var phi_13396_: u32;
    var phi_1752_: vec3<f32>;
    var phi_1753_: vec3<f32>;
    var phi_1754_: vec3<f32>;
    var phi_13395_: u32;
    var phi_1755_: vec3<f32>;
    var phi_1756_: vec3<f32>;
    var phi_1757_: vec3<f32>;
    var phi_1758_: bool;
    var phi_13394_: u32;
    var phi_1759_: vec3<f32>;
    var phi_1760_: vec3<f32>;
    var phi_1761_: vec3<f32>;
    var phi_1762_: bool;
    var phi_1763_: bool;
    var phi_8302_: bool;
    var phi_8464_: bool;
    var phi_8479_: bool;
    var phi_8494_: bool;
    var phi_8521_: bool;
    var phi_8536_: bool;
    var phi_8551_: bool;
    var phi_8578_: bool;
    var phi_8593_: bool;
    var phi_8608_: bool;
    var phi_1942_: f32;
    var phi_1950_: f32;
    var phi_1958_: f32;
    var phi_8805_: bool;
    var phi_8794_: f32;
    var phi_2065_: vec3<f32>;
    var phi_8833_: bool;
    var phi_2066_: vec3<f32>;
    var phi_2067_: vec3<f32>;
    var phi_2068_: vec3<f32>;
    var phi_13777_: vec3<f32>;
    var phi_13701_: vec3<f32>;
    var phi_13623_: u32;
    var phi_13542_: f32;
    var phi_2069_: vec3<f32>;
    var phi_2070_: vec3<f32>;
    var phi_2072_: bool;
    var phi_8848_: bool;
    var phi_8899_: bool;
    var phi_8888_: f32;
    var phi_13749_: vec3<f32>;
    var phi_1592_: vec3<f32>;
    var phi_14098_: vec3<f32>;
    var phi_13893_: f32;
    var phi_13748_: vec3<f32>;
    var phi_13440_: u32;
    var phi_1594_: vec3<f32>;
    var phi_1595_: bool;
    var phi_14044_: vec3<f32>;
    var phi_13853_: f32;
    var phi_13775_: vec3<f32>;
    var phi_13699_: vec3<f32>;
    var phi_13621_: u32;
    var phi_13540_: f32;
    var phi_13464_: u32;
    var phi_13391_: u32;
    var phi_2080_: vec3<f32>;
    var phi_2081_: vec3<f32>;
    var phi_2082_: vec3<f32>;
    var phi_2083_: bool;
    var phi_8965_: bool;
    var phi_2152_: vec3<f32>;
    var phi_13453_: u32;
    var phi_2154_: vec3<f32>;
    var phi_2155_: bool;
    var phi_13452_: u32;
    var phi_2156_: vec3<f32>;
    var phi_2157_: bool;
    var phi_13450_: u32;
    var phi_2163_: vec3<f32>;
    var phi_2164_: bool;
    var phi_2194_: vec3<f32>;
    var phi_2195_: vec3<f32>;
    var phi_2196_: u32;
    var phi_13448_: u32;
    var phi_2198_: vec3<f32>;
    var phi_2199_: vec3<f32>;
    var phi_2200_: u32;
    var phi_2201_: bool;
    var phi_2202_: bool;
    var phi_13447_: u32;
    var phi_2203_: vec3<f32>;
    var phi_2204_: vec3<f32>;
    var phi_2205_: u32;
    var phi_2207_: bool;
    var phi_2208_: bool;
    var phi_2209_: bool;
    var phi_14041_: vec3<f32>;
    var phi_13850_: f32;
    var phi_13772_: vec3<f32>;
    var phi_13696_: vec3<f32>;
    var phi_13618_: u32;
    var phi_13537_: f32;
    var phi_13461_: u32;
    var phi_13446_: u32;
    var phi_2210_: vec3<f32>;
    var phi_2211_: vec3<f32>;
    var phi_2212_: u32;
    var phi_2213_: vec3<f32>;
    var phi_2214_: bool;
    var phi_2215_: bool;
    var phi_2216_: bool;
    var phi_2217_: bool;
    var phi_15202_: bool;
    var phi_13445_: u32;
    var phi_2226_: u32;
    var phi_2229_: vec3<f32>;
    var phi_2231_: u32;
    var phi_3438_: u32;
    var phi_3441_: u32;
    var phi_3439_: u32;
    var phi_3442_: u32;
    var local_12: u32;
    var phi_3479_: u32;
    var phi_3482_: u32;
    var phi_3480_: u32;
    var phi_3483_: u32;
    var local_13: u32;
    var local_14: u32;
    var phi_3559_: f32;
    var local_15: u32;
    var local_16: u32;
    var phi_3608_: f32;
    var phi_3764_: f32;
    var phi_4085_: vec3<f32>;
    var phi_4087_: vec3<f32>;
    var phi_15544_: u32;
    var phi_4088_: vec3<f32>;
    var phi_4089_: f32;
    var phi_4090_: vec3<f32>;
    var phi_2911_: u32;
    var phi_2914_: u32;
    var phi_2912_: u32;
    var phi_2915_: u32;
    var local_17: u32;
    var local_18: u32;
    var phi_3307_: f32;
    var phi_3310_: f32;
    var phi_9235_: bool;
    var phi_9216_: f32;
    var phi_3311_: f32;
    var phi_9250_: bool;
    var phi_9273_: bool;
    var phi_9288_: bool;
    var phi_3062_: f32;
    var phi_9303_: bool;
    var phi_9318_: bool;
    var phi_9333_: bool;
    var phi_9398_: bool;
    var phi_9486_: bool;
    var phi_9467_: f32;
    var phi_3127_: type_60;
    var phi_3313_: type_60;
    var phi_3391_: bool;
    var phi_3400_: vec3<f32>;
    var phi_3402_: vec3<f32>;
    var phi_3403_: bool;
    var phi_3408_: vec3<f32>;
    var phi_3409_: f32;
    var phi_15547_: u32;
    var phi_3410_: vec3<f32>;
    var phi_3411_: f32;
    var phi_3412_: vec3<f32>;
    var phi_3413_: f32;
    var phi_15546_: u32;
    var phi_3414_: vec3<f32>;
    var phi_3415_: f32;
    var phi_3416_: vec3<f32>;
    var phi_3417_: f32;
    var phi_15543_: u32;
    var phi_4091_: vec3<f32>;
    var phi_4092_: f32;
    var phi_4093_: vec3<f32>;
    var phi_4094_: f32;
    var phi_9501_: bool;
    var phi_9611_: f32;
    var phi_9639_: f32;
    var phi_9667_: f32;
    var phi_9801_: bool;
    var phi_9816_: bool;
    var phi_9831_: bool;
    var phi_9858_: bool;
    var phi_9873_: bool;
    var phi_9888_: bool;
    var phi_9915_: bool;
    var phi_9930_: bool;
    var phi_9945_: bool;
    var phi_2563_: f32;
    var phi_2571_: f32;
    var phi_2579_: f32;
    var phi_10142_: bool;
    var phi_10131_: f32;
    var phi_2686_: vec3<f32>;
    var phi_10170_: bool;
    var phi_2687_: vec3<f32>;
    var phi_2699_: type_59;
    var phi_2700_: type_59;
    var phi_2701_: bool;
    var phi_2706_: type_59;
    var phi_10207_: bool;
    var phi_10237_: bool;
    var phi_10226_: f32;
    var phi_10276_: vec3<f32>;
    var phi_2734_: type_59;
    var phi_2735_: type_59;
    var phi_10298_: vec3<f32>;
    var phi_2736_: type_59;
    var phi_2737_: type_59;
    var phi_2738_: bool;
    var phi_2742_: type_59;
    var phi_10330_: bool;
    var phi_2827_: f32;
    var phi_10401_: bool;
    var phi_10416_: bool;
    var phi_2883_: vec3<f32>;
    var phi_2884_: vec3<f32>;
    var phi_15581_: u32;
    var phi_2885_: vec3<f32>;
    var phi_2886_: f32;
    var phi_2887_: vec3<f32>;
    var phi_15580_: u32;
    var phi_2888_: vec3<f32>;
    var phi_2889_: f32;
    var phi_2890_: vec3<f32>;
    var phi_15542_: u32;
    var phi_4095_: vec3<f32>;
    var phi_4096_: f32;
    var phi_4097_: vec3<f32>;
    var phi_4098_: f32;
    var phi_2327_: f32;
    var phi_2328_: f32;
    var phi_15541_: u32;
    var phi_4099_: vec3<f32>;
    var phi_4100_: f32;
    var phi_4101_: vec3<f32>;
    var phi_4102_: f32;
    var phi_10431_: bool;
    var phi_10541_: f32;
    var phi_10569_: f32;
    var phi_10597_: f32;
    var phi_10731_: bool;
    var phi_10746_: bool;
    var phi_10761_: bool;
    var phi_10788_: bool;
    var phi_10803_: bool;
    var phi_10818_: bool;
    var phi_10845_: bool;
    var phi_10860_: bool;
    var phi_10875_: bool;
    var phi_4325_: f32;
    var phi_4333_: f32;
    var phi_4341_: f32;
    var phi_11072_: bool;
    var phi_11061_: f32;
    var phi_4448_: vec3<f32>;
    var phi_11100_: bool;
    var phi_4449_: vec3<f32>;
    var phi_4461_: type_59;
    var phi_4462_: type_59;
    var phi_4463_: bool;
    var phi_4468_: type_59;
    var phi_11137_: bool;
    var phi_11167_: bool;
    var phi_11156_: f32;
    var phi_11206_: vec3<f32>;
    var phi_4496_: type_59;
    var phi_4497_: type_59;
    var phi_11228_: vec3<f32>;
    var phi_4498_: type_59;
    var phi_4499_: type_59;
    var phi_4500_: bool;
    var phi_4504_: type_59;
    var phi_11242_: f32;
    var phi_11245_: f32;
    var phi_4513_: f32;
    var phi_4547_: u32;
    var phi_4550_: u32;
    var phi_4552_: u32;
    var phi_4556_: type_49;
    var phi_11342_: bool;
    var phi_11372_: bool;
    var phi_11319_: type_52;
    var phi_11322_: type_52;
    var phi_11323_: bool;
    var phi_11328_: type_52;
    var phi_11429_: bool;
    var phi_11444_: bool;
    var phi_11459_: bool;
    var phi_11474_: bool;
    var phi_11406_: type_52;
    var phi_11409_: type_52;
    var phi_11410_: bool;
    var phi_11415_: type_52;
    var phi_11516_: bool;
    var phi_11531_: bool;
    var phi_11546_: bool;
    var phi_11561_: bool;
    var phi_11493_: type_52;
    var phi_11496_: type_52;
    var phi_11497_: bool;
    var phi_11502_: type_52;
    var phi_11295_: bool;
    var phi_11298_: bool;
    var phi_11299_: bool;
    var phi_4963_: u32;
    var phi_4964_: u32;
    var phi_4965_: u32;
    var phi_4966_: u32;
    var phi_4967_: u32;
    var phi_4968_: u32;
    var phi_4969_: u32;
    var phi_4970_: u32;
    var phi_4971_: u32;
    var phi_4852_: vec4<f32>;
    var phi_4853_: vec4<f32>;
    var phi_4854_: bool;
    var phi_4858_: vec4<f32>;
    var phi_4859_: bool;
    var phi_4764_: bool;
    var phi_4801_: bool;
    var phi_4807_: f32;
    var phi_4817_: vec4<f32>;
    var phi_4818_: vec4<f32>;
    var phi_4819_: bool;
    var phi_4823_: vec4<f32>;
    var phi_4824_: bool;
    var phi_4828_: vec4<f32>;
    var phi_4829_: bool;
    var phi_4860_: vec4<f32>;
    var phi_4861_: bool;
    var phi_4693_: vec4<f32>;
    var phi_4694_: vec4<f32>;
    var phi_4695_: bool;
    var phi_4699_: vec4<f32>;
    var phi_4700_: bool;
    var phi_4701_: vec4<f32>;
    var phi_4702_: bool;
    var phi_4703_: bool;
    var phi_4711_: vec4<f32>;
    var phi_4712_: bool;
    var phi_4713_: vec4<f32>;
    var phi_4714_: bool;
    var phi_4715_: bool;
    var phi_4723_: vec4<f32>;
    var phi_4724_: bool;
    var phi_4870_: vec4<f32>;
    var phi_4871_: bool;
    var phi_4892_: type_49;
    var phi_4894_: type_49;
    var phi_4900_: u32;
    var phi_4548_: u32;
    var phi_4551_: u32;
    var phi_4553_: u32;
    var phi_4557_: type_49;
    var phi_4973_: type_49;
    var phi_4974_: bool;
    var phi_4975_: bool;
    var phi_15122_: bool;
    var phi_7747_: bool;
    var phi_7746_: type_49;
    var phi_5147_: type_49;
    var phi_5152_: type_49;
    var phi_5185_: vec3<f32>;
    var phi_15503_: bool;
    var phi_5186_: u32;
    var phi_5187_: vec3<f32>;
    var phi_15502_: bool;
    var phi_5188_: u32;
    var phi_5189_: vec3<f32>;
    var phi_15719_: u32;
    var phi_15501_: bool;
    var phi_2227_: u32;
    var phi_2230_: vec3<f32>;
    var phi_2232_: u32;
    var phi_15513_: u32;
    var phi_15500_: bool;
    var phi_11697_: bool;
    var phi_11727_: bool;
    var phi_11716_: f32;
    var phi_11647_: bool;
    var phi_11783_: vec3<f32>;
    var phi_11939_: bool;
    var phi_11954_: bool;
    var phi_11969_: bool;
    var phi_11878_: vec3<f32>;
    var phi_11881_: vec3<f32>;
    var phi_11882_: bool;
    var phi_11652_: vec3<f32>;
    var phi_11656_: vec3<f32>;
    var phi_11657_: bool;
    var phi_12069_: bool;
    var phi_12043_: vec3<f32>;
    var phi_11664_: vec3<f32>;
    var phi_12084_: bool;
    var phi_12194_: f32;
    var phi_12222_: f32;
    var phi_12250_: f32;
    var phi_12384_: bool;
    var phi_12399_: bool;
    var phi_12414_: bool;
    var phi_12441_: bool;
    var phi_12456_: bool;
    var phi_12471_: bool;
    var phi_12498_: bool;
    var phi_12513_: bool;
    var phi_12528_: bool;
    var phi_5406_: f32;
    var phi_5414_: f32;
    var phi_5422_: f32;
    var phi_12725_: bool;
    var phi_12714_: f32;
    var phi_5529_: vec3<f32>;
    var phi_12753_: bool;
    var phi_5530_: vec3<f32>;
    var phi_5542_: type_59;
    var phi_5543_: type_59;
    var phi_5544_: bool;
    var phi_5549_: type_59;
    var phi_12790_: bool;
    var phi_12820_: bool;
    var phi_12809_: f32;
    var phi_12859_: vec3<f32>;
    var phi_5577_: type_59;
    var phi_5578_: type_59;
    var phi_12881_: vec3<f32>;
    var phi_5579_: type_59;
    var phi_5580_: type_59;
    var phi_5581_: bool;
    var phi_5585_: type_59;
    var phi_12906_: bool;
    var phi_5661_: vec3<f32>;
    var phi_17072_: u32;
    var phi_5663_: vec3<f32>;
    var phi_5664_: bool;
    var phi_17071_: u32;
    var phi_5665_: vec3<f32>;
    var phi_5666_: bool;
    var phi_17069_: u32;
    var phi_5672_: vec3<f32>;
    var phi_5673_: bool;
    var phi_5702_: vec3<f32>;
    var phi_5714_: vec3<f32>;
    var phi_5715_: u32;
    var phi_5716_: vec3<f32>;
    var phi_17067_: u32;
    var phi_5718_: vec3<f32>;
    var phi_5719_: u32;
    var phi_5720_: vec3<f32>;
    var phi_5721_: bool;
    var phi_5722_: bool;
    var phi_17066_: u32;
    var phi_5723_: vec3<f32>;
    var phi_5724_: u32;
    var phi_5725_: vec3<f32>;
    var phi_5726_: bool;
    var phi_5727_: bool;
    var phi_5728_: bool;
    var phi_17065_: u32;
    var phi_5729_: vec3<f32>;
    var phi_5730_: f32;
    var phi_5731_: vec3<f32>;
    var phi_5732_: u32;
    var phi_5733_: vec3<f32>;
    var phi_5734_: bool;
    var phi_5735_: bool;
    var phi_5736_: bool;
    var phi_5737_: bool;
    var phi_17064_: u32;
    var phi_16970_: bool;
    var phi_5738_: u32;
    var phi_5739_: vec3<f32>;
    var phi_5744_: u32;
    var phi_5745_: vec3<f32>;
    var phi_5746_: vec3<f32>;
    var phi_5747_: f32;
    var phi_5748_: vec3<f32>;
    var phi_5749_: u32;
    var phi_5752_: vec3<f32>;
    var phi_5755_: bool;
    var phi_5756_: bool;
    var phi_5757_: bool;
    var phi_5758_: bool;
    var phi_5760_: bool;
    var phi_17063_: u32;
    var phi_16969_: bool;
    var phi_5762_: u32;
    var phi_5763_: vec3<f32>;
    var phi_5768_: u32;
    var phi_5769_: vec3<f32>;
    var phi_5770_: vec3<f32>;
    var phi_5771_: f32;
    var phi_5772_: vec3<f32>;
    var phi_5773_: u32;
    var phi_5774_: bool;
    var phi_5775_: bool;
    var phi_5776_: vec3<f32>;
    var phi_5779_: bool;
    var phi_5780_: bool;
    var phi_5781_: bool;
    var phi_5782_: bool;
    var phi_5783_: bool;
    var phi_5784_: bool;
    var phi_5785_: bool;
    var phi_17062_: u32;
    var phi_16968_: bool;
    var phi_5786_: vec3<f32>;
    var phi_5787_: vec3<f32>;
    var phi_5788_: f32;
    var phi_5789_: f32;
    var phi_5790_: f32;
    var phi_5791_: vec3<f32>;
    var phi_5792_: type_14;
    var phi_5793_: u32;
    var phi_5794_: u32;
    var phi_5795_: vec3<f32>;
    var phi_335_: u32;
    var phi_338_: vec3<f32>;
    var phi_340_: vec3<f32>;
    var phi_342_: f32;
    var phi_344_: vec3<f32>;
    var phi_346_: u32;
    var phi_348_: bool;
    var phi_350_: bool;
    var phi_352_: vec3<f32>;
    var phi_5804_: bool;
    var phi_5805_: bool;
    var phi_5806_: bool;
    var phi_5807_: bool;
    var phi_5808_: bool;
    var phi_5809_: bool;
    var phi_5810_: bool;
    var phi_5811_: bool;
    var phi_16967_: bool;
    var phi_7779_: bool;
    var phi_7778_: u32;
    var phi_7777_: type_49;
    var phi_7776_: bool;
    var phi_7775_: bool;
    var phi_7774_: bool;
    var phi_7773_: bool;
    var phi_7772_: bool;
    var phi_7771_: bool;
    var phi_7770_: bool;
    var phi_7769_: vec3<f32>;
    var phi_7767_: vec3<f32>;
    var phi_7766_: u32;
    var phi_7759_: u32;
    var phi_7757_: type_14;
    var phi_7755_: vec3<f32>;
    var phi_7754_: f32;
    var phi_7753_: f32;
    var phi_7752_: f32;
    var phi_7750_: vec3<f32>;
    var local_19: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_5967_: f32;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_12949_: f32;
    var phi_12952_: f32;
    var phi_5969_: f32;
    var phi_5970_: f32;
    var local_26: vec3<f32>;
    var local_27: vec3<f32>;
    var local_28: vec3<f32>;
    var local_29: vec3<f32>;
    var local_30: vec3<f32>;
    var local_31: vec3<f32>;
    var local_32: vec3<f32>;
    var local_33: vec3<f32>;
    var local_34: vec3<f32>;
    var local_35: bool;
    var phi_6348_: f32;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_13018_: f32;
    var phi_13021_: f32;
    var phi_6350_: f32;
    var phi_13032_: bool;
    var phi_13047_: bool;
    var phi_6405_: vec3<f32>;
    var phi_6422_: vec3<f32>;
    var phi_6423_: vec3<f32>;
    var local_39: vec3<f32>;
    var local_40: vec3<f32>;
    var local_41: vec3<f32>;
    var phi_6475_: bool;
    var phi_6484_: vec3<f32>;
    var phi_6486_: vec3<f32>;
    var phi_6487_: bool;
    var local_42: bool;
    var local_43: vec3<f32>;
    var local_44: vec3<f32>;
    var local_45: vec3<f32>;
    var phi_6651_: f32;
    var phi_6654_: f32;
    var local_46: vec3<f32>;
    var local_47: vec3<f32>;
    var local_48: vec3<f32>;
    var phi_13135_: bool;
    var phi_13116_: f32;
    var phi_6655_: f32;
    var local_49: f32;
    var local_50: f32;
    var local_51: f32;
    var phi_13153_: f32;
    var phi_13156_: f32;
    var phi_6658_: f32;
    var phi_6659_: f32;
    var phi_6660_: f32;
    var phi_6682_: vec3<f32>;
    var phi_6806_: bool;
    var phi_6807_: bool;
    var phi_6808_: bool;
    var local_52: type_49;
    var local_53: type_49;
    var local_54: u32;
    var local_55: vec3<f32>;
    var local_56: u32;
    var local_57: vec3<f32>;
    var local_58: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e115 = global;
            let _e129 = (_e115.x + (_e115.y * 4194240u));
            let _e133 = global_1.member.member[0u];
            let _e137 = global_1.member.member[1u];
            if (_e129 >= (_e133 * _e137)) {
            } else {
                if (_e129 < arrayLength((&global_7.member))) {
                } else {
                    break;
                }
                let _e144 = global_7.member[_e129].member;
                let _e148 = global_7.member[_e129].member_1;
                if (_e148.w < 0.5f) {
                } else {
                    let _e154 = global_1.member.member[2u];
                    let _e158 = global_1.member.member_7[0u];
                    let _e162 = ((((_e154 * 2654435769u) ^ _e158) * 747796405u) + 2891336453u);
                    let _e170 = (((_e162 >> bitcast<u32>((((_e162 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e162) * 277803737u);
                    let _e176 = (((_e129 ^ ((_e170 >> bitcast<u32>(22i)) ^ _e170)) * 747796405u) + 2891336453u);
                    let _e184 = (((_e176 >> bitcast<u32>((((_e176 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e176) * 277803737u);
                    let _e201 = (1f / sqrt((((_e148.x * _e148.x) + (_e148.y * _e148.y)) + (_e148.z * _e148.z))));
                    let _e208 = global_1.member.member[3u];
                    let _e211 = global_1.member.member_3[3u];
                    let _e214 = global_1.member.member_1[2u];
                    phi_13272_ = ((_e184 >> bitcast<u32>(22i)) ^ _e184);
                    phi_13192_ = false;
                    phi_334_ = 0u;
                    phi_337_ = vec3<f32>(0f, 0f, 0f);
                    phi_339_ = vec3<f32>(1f, 1f, 1f);
                    phi_341_ = 0f;
                    phi_343_ = vec3<f32>((_e148.x * _e201), (_e148.y * _e201), (_e148.z * _e201));
                    phi_345_ = 0u;
                    phi_347_ = false;
                    phi_349_ = true;
                    phi_351_ = vec3<f32>(_e144.x, _e144.y, _e144.z);
                    loop {
                        let _e216 = phi_13272_;
                        let _e218 = phi_13192_;
                        let _e220 = phi_334_;
                        let _e222 = phi_337_;
                        let _e224 = phi_339_;
                        let _e226 = phi_341_;
                        let _e228 = phi_343_;
                        let _e230 = phi_345_;
                        let _e232 = phi_347_;
                        let _e234 = phi_349_;
                        let _e236 = phi_351_;
                        let _e239 = global_1.member.member_1[0u];
                        let _e242 = global_1.member.member_1[1u];
                        let _e243 = (_e239 + _e242);
                        let _e244 = (vec3<f32>(1f, 1f, 1f) / _e228);
                        local_1 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                        let _e246 = select(0u, 1u, (_e239 > 0u));
                        phi_372_ = _e239;
                        phi_375_ = _e246;
                        phi_377_ = _e243;
                        phi_379_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_381_ = type_49(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                        local_19 = _e234;
                        local_20 = _e228;
                        local_21 = _e228;
                        local_22 = _e228;
                        local_23 = _e226;
                        local_24 = _e226;
                        local_25 = _e226;
                        local_26 = _e228;
                        local_27 = _e228;
                        local_28 = _e228;
                        local_29 = _e224;
                        local_30 = _e224;
                        local_31 = _e224;
                        local_32 = _e222;
                        local_33 = _e222;
                        local_34 = _e222;
                        local_35 = _e234;
                        local_36 = _e226;
                        local_37 = _e226;
                        local_38 = _e226;
                        local_39 = _e228;
                        local_40 = _e228;
                        local_41 = _e228;
                        local_42 = _e234;
                        local_43 = _e236;
                        local_44 = _e236;
                        local_45 = _e236;
                        local_46 = _e236;
                        local_47 = _e236;
                        local_48 = _e236;
                        local_49 = _e226;
                        local_50 = _e226;
                        local_51 = _e226;
                        loop {
                            let _e248 = phi_372_;
                            let _e250 = phi_375_;
                            let _e252 = phi_377_;
                            let _e254 = phi_379_;
                            let _e256 = phi_381_;
                            local_2 = _e256;
                            local_3 = _e254;
                            local_4 = _e254;
                            local_5 = _e254;
                            local_6 = _e256;
                            local_52 = _e256;
                            if (_e248 < _e252) {
                                let _e428 = global_2.member[_e248];
                                if (_e428.member_6.x == 0u) {
                                    let _e542 = (_e428.member_1.x - _e428.member.x);
                                    let _e543 = (_e428.member_1.y - _e428.member.y);
                                    let _e544 = (_e428.member_1.z - _e428.member.z);
                                    let _e549 = (_e428.member_2.x - _e428.member.x);
                                    let _e550 = (_e428.member_2.y - _e428.member.y);
                                    let _e551 = (_e428.member_2.z - _e428.member.z);
                                    let _e556 = ((_e228.y * _e551) - (_e550 * _e228.z));
                                    let _e560 = ((_e228.z * _e549) - (_e551 * _e228.x));
                                    let _e563 = ((_e228.x * _e550) - (_e549 * _e228.y));
                                    let _e568 = (((_e542 * _e556) + (_e543 * _e560)) + (_e544 * _e563));
                                    if (abs(_e568) < 0.000000000001f) {
                                        phi_551_ = vec4<f32>();
                                        phi_552_ = true;
                                    } else {
                                        let _e571 = (1f / _e568);
                                        let _e573 = (_e236.x - _e428.member.x);
                                        let _e575 = (_e236.y - _e428.member.y);
                                        let _e577 = (_e236.z - _e428.member.z);
                                        let _e583 = ((((_e573 * _e556) + (_e575 * _e560)) + (_e577 * _e563)) * _e571);
                                        if (_e583 < 0f) {
                                            phi_541_ = vec4<f32>();
                                            phi_542_ = true;
                                            phi_543_ = false;
                                        } else {
                                            let _e585 = (_e583 > 1f);
                                            if _e585 {
                                                phi_539_ = vec4<f32>();
                                                phi_540_ = false;
                                            } else {
                                                let _e588 = ((_e575 * _e544) - (_e543 * _e577));
                                                let _e591 = ((_e577 * _e542) - (_e544 * _e573));
                                                let _e594 = ((_e573 * _e543) - (_e542 * _e575));
                                                let _e600 = ((((_e228.x * _e588) + (_e228.y * _e591)) + (_e228.z * _e594)) * _e571);
                                                if (_e600 < 0f) {
                                                    phi_529_ = vec4<f32>();
                                                    phi_530_ = true;
                                                    phi_531_ = false;
                                                } else {
                                                    let _e603 = ((_e583 + _e600) > 1f);
                                                    if _e603 {
                                                        phi_527_ = vec4<f32>();
                                                        phi_528_ = false;
                                                    } else {
                                                        let _e609 = ((((_e549 * _e588) + (_e550 * _e591)) + (_e551 * _e594)) * _e571);
                                                        if (_e609 <= 0f) {
                                                            phi_522_ = vec4<f32>();
                                                            phi_523_ = true;
                                                        } else {
                                                            let _e611 = (_e609 >= _e256.member);
                                                            if _e611 {
                                                                phi_521_ = vec4<f32>();
                                                            } else {
                                                                phi_521_ = vec4<f32>(_e609, _e583, _e600, 1f);
                                                            }
                                                            let _e614 = phi_521_;
                                                            phi_522_ = _e614;
                                                            phi_523_ = _e611;
                                                        }
                                                        let _e616 = phi_522_;
                                                        let _e618 = phi_523_;
                                                        phi_527_ = _e616;
                                                        phi_528_ = _e618;
                                                    }
                                                    let _e620 = phi_527_;
                                                    let _e622 = phi_528_;
                                                    phi_529_ = _e620;
                                                    phi_530_ = _e603;
                                                    phi_531_ = _e622;
                                                }
                                                let _e624 = phi_529_;
                                                let _e626 = phi_530_;
                                                let _e628 = phi_531_;
                                                phi_539_ = _e624;
                                                phi_540_ = select(_e628, true, _e626);
                                            }
                                            let _e631 = phi_539_;
                                            let _e633 = phi_540_;
                                            phi_541_ = _e631;
                                            phi_542_ = _e585;
                                            phi_543_ = _e633;
                                        }
                                        let _e635 = phi_541_;
                                        let _e637 = phi_542_;
                                        let _e639 = phi_543_;
                                        phi_551_ = _e635;
                                        phi_552_ = select(_e639, true, _e637);
                                    }
                                    let _e642 = phi_551_;
                                    let _e644 = phi_552_;
                                    phi_702_ = _e642;
                                    phi_703_ = _e644;
                                } else {
                                    if (_e428.member_6.x == 1u) {
                                        let _e459 = (_e236.x - _e428.member.x);
                                        let _e461 = (_e236.y - _e428.member.y);
                                        let _e463 = (_e236.z - _e428.member.z);
                                        let _e471 = (((_e459 * _e228.x) + (_e461 * _e228.y)) + (_e463 * _e228.z));
                                        if (_e471 > 0f) {
                                            phi_595_ = select(true, false, ((((_e459 * _e459) + (_e461 * _e461)) + (_e463 * _e463)) > (_e428.member.w * _e428.member.w)));
                                        } else {
                                            phi_595_ = true;
                                        }
                                        let _e484 = phi_595_;
                                        if _e484 {
                                            let _e488 = (_e459 - (_e228.x * _e471));
                                            let _e489 = (_e461 - (_e228.y * _e471));
                                            let _e490 = (_e463 - (_e228.z * _e471));
                                            let _e499 = ((_e428.member.w * _e428.member.w) - (((_e488 * _e488) + (_e489 * _e489)) + (_e490 * _e490)));
                                            if (_e499 < 0f) {
                                                phi_654_ = vec4<f32>();
                                                phi_655_ = true;
                                            } else {
                                                let _e501 = sqrt(_e499);
                                                let _e502 = -(_e471);
                                                let _e503 = (_e502 - _e501);
                                                if (_e503 <= 0f) {
                                                    phi_632_ = true;
                                                } else {
                                                    phi_632_ = (_e503 >= _e256.member);
                                                }
                                                let _e507 = phi_632_;
                                                if _e507 {
                                                    phi_638_ = (_e502 + _e501);
                                                } else {
                                                    phi_638_ = _e503;
                                                }
                                                let _e510 = phi_638_;
                                                if (_e510 <= 0f) {
                                                    phi_649_ = vec4<f32>();
                                                    phi_650_ = true;
                                                } else {
                                                    let _e512 = (_e510 >= _e256.member);
                                                    if _e512 {
                                                        phi_648_ = vec4<f32>();
                                                    } else {
                                                        phi_648_ = vec4<f32>(_e510, 0f, 0f, 1f);
                                                    }
                                                    let _e515 = phi_648_;
                                                    phi_649_ = _e515;
                                                    phi_650_ = _e512;
                                                }
                                                let _e517 = phi_649_;
                                                let _e519 = phi_650_;
                                                phi_654_ = _e517;
                                                phi_655_ = _e519;
                                            }
                                            let _e521 = phi_654_;
                                            let _e523 = phi_655_;
                                            phi_659_ = _e521;
                                            phi_660_ = _e523;
                                        } else {
                                            phi_659_ = vec4<f32>();
                                            phi_660_ = true;
                                        }
                                        let _e525 = phi_659_;
                                        let _e527 = phi_660_;
                                        phi_692_ = _e525;
                                        phi_693_ = _e527;
                                    } else {
                                        if (abs(_e228.y) < 0.000000000001f) {
                                            phi_690_ = vec4<f32>();
                                            phi_691_ = true;
                                        } else {
                                            let _e440 = ((_e428.member.x - _e236.y) / _e228.y);
                                            if (_e440 <= 0f) {
                                                phi_685_ = vec4<f32>();
                                                phi_686_ = true;
                                            } else {
                                                let _e442 = (_e440 >= _e256.member);
                                                if _e442 {
                                                    phi_684_ = vec4<f32>();
                                                } else {
                                                    phi_684_ = vec4<f32>(_e440, 0f, 0f, 1f);
                                                }
                                                let _e445 = phi_684_;
                                                phi_685_ = _e445;
                                                phi_686_ = _e442;
                                            }
                                            let _e447 = phi_685_;
                                            let _e449 = phi_686_;
                                            phi_690_ = _e447;
                                            phi_691_ = _e449;
                                        }
                                        let _e451 = phi_690_;
                                        let _e453 = phi_691_;
                                        phi_692_ = _e451;
                                        phi_693_ = _e453;
                                    }
                                    let _e529 = phi_692_;
                                    let _e531 = phi_693_;
                                    phi_702_ = _e529;
                                    phi_703_ = select(true, false, select(true, false, _e531));
                                }
                                let _e646 = phi_702_;
                                let _e648 = phi_703_;
                                let _e651 = select(vec4<f32>(0f, 0f, 0f, 0f), _e646, vec4(select(true, false, _e648)));
                                let _e653 = (_e651.w > 0f);
                                if _e653 {
                                    let _e659 = type_49(_e651.x, _e256.member_1, _e256.member_2, _e256.member_3);
                                    let _e664 = type_49(_e659.member, _e248, _e659.member_2, _e659.member_3);
                                    phi_724_ = _e664;
                                    phi_726_ = _e664;
                                } else {
                                    phi_724_ = type_49();
                                    phi_726_ = _e256;
                                }
                                let _e666 = phi_724_;
                                let _e668 = phi_726_;
                                phi_373_ = (_e248 + 1u);
                                phi_376_ = _e250;
                                phi_378_ = _e252;
                                phi_380_ = select(_e254, _e651, vec4(_e653));
                                phi_382_ = _e668;
                                phi_805_ = _e666;
                                phi_806_ = true;
                                phi_807_ = false;
                            } else {
                                let _e258 = (_e250 == 0u);
                                if _e258 {
                                    phi_801_ = u32();
                                    phi_802_ = u32();
                                    phi_803_ = u32();
                                } else {
                                    let _e259 = (_e250 - 1u);
                                    let _e260 = (_e259 < 100u);
                                    if _e260 {
                                    } else {
                                        phi_13177_ = true;
                                        phi_7745_ = bool();
                                        phi_7743_ = type_49();
                                        break;
                                    }
                                    let _e262 = local_1[_e259];
                                    let _e264 = global_3.member[_e262];
                                    let _e278 = ((_e264.member.x - _e236.x) * _e244.x);
                                    let _e280 = ((_e264.member.y - _e236.y) * _e244.y);
                                    let _e282 = ((_e264.member.z - _e236.z) * _e244.z);
                                    let _e289 = ((_e264.member_2.x - _e236.x) * _e244.x);
                                    let _e290 = ((_e264.member_2.y - _e236.y) * _e244.y);
                                    let _e291 = ((_e264.member_2.z - _e236.z) * _e244.z);
                                    if (_e278 == _e278) {
                                        let _e293 = (_e289 == _e289);
                                        if _e293 {
                                            let _e294 = (_e278 != _e278);
                                            if _e294 {
                                                phi_7908_ = true;
                                            } else {
                                                phi_7908_ = (_e289 <= _e278);
                                            }
                                            let _e297 = phi_7908_;
                                            let _e298 = select(_e278, _e289, _e297);
                                            if _e294 {
                                                phi_7938_ = true;
                                            } else {
                                                phi_7938_ = (_e289 >= _e278);
                                            }
                                            let _e303 = phi_7938_;
                                            let _e304 = select(_e278, _e289, _e303);
                                            phi_7885_ = type_52(select(-300000000000000000000000000000000000000f, _e298, (_e298 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e304, (_e304 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_7885_ = type_52();
                                        }
                                        let _e309 = phi_7885_;
                                        phi_7888_ = _e309;
                                        phi_7889_ = select(true, false, _e293);
                                    } else {
                                        phi_7888_ = type_52();
                                        phi_7889_ = true;
                                    }
                                    let _e312 = phi_7888_;
                                    let _e314 = phi_7889_;
                                    if _e314 {
                                        phi_7894_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_7894_ = _e312;
                                    }
                                    let _e316 = phi_7894_;
                                    if (_e280 == _e280) {
                                        let _e320 = (_e290 == _e290);
                                        if _e320 {
                                            let _e321 = (_e280 != _e280);
                                            if _e321 {
                                                phi_7995_ = true;
                                            } else {
                                                phi_7995_ = (_e290 <= _e280);
                                            }
                                            let _e324 = phi_7995_;
                                            let _e325 = select(_e280, _e290, _e324);
                                            if (_e316.member != _e316.member) {
                                                phi_8010_ = true;
                                            } else {
                                                phi_8010_ = (_e325 >= _e316.member);
                                            }
                                            let _e329 = phi_8010_;
                                            if _e321 {
                                                phi_8025_ = true;
                                            } else {
                                                phi_8025_ = (_e290 >= _e280);
                                            }
                                            let _e333 = phi_8025_;
                                            let _e334 = select(_e280, _e290, _e333);
                                            if (_e316.member_1 != _e316.member_1) {
                                                phi_8040_ = true;
                                            } else {
                                                phi_8040_ = (_e334 <= _e316.member_1);
                                            }
                                            let _e338 = phi_8040_;
                                            phi_7972_ = type_52(select(_e316.member, _e325, _e329), select(_e316.member_1, _e334, _e338));
                                        } else {
                                            phi_7972_ = type_52();
                                        }
                                        let _e342 = phi_7972_;
                                        phi_7975_ = _e342;
                                        phi_7976_ = select(true, false, _e320);
                                    } else {
                                        phi_7975_ = type_52();
                                        phi_7976_ = true;
                                    }
                                    let _e345 = phi_7975_;
                                    let _e347 = phi_7976_;
                                    if _e347 {
                                        phi_7981_ = _e316;
                                    } else {
                                        phi_7981_ = _e345;
                                    }
                                    let _e349 = phi_7981_;
                                    if (_e282 == _e282) {
                                        let _e353 = (_e291 == _e291);
                                        if _e353 {
                                            let _e354 = (_e282 != _e282);
                                            if _e354 {
                                                phi_8082_ = true;
                                            } else {
                                                phi_8082_ = (_e291 <= _e282);
                                            }
                                            let _e357 = phi_8082_;
                                            let _e358 = select(_e282, _e291, _e357);
                                            if (_e349.member != _e349.member) {
                                                phi_8097_ = true;
                                            } else {
                                                phi_8097_ = (_e358 >= _e349.member);
                                            }
                                            let _e362 = phi_8097_;
                                            if _e354 {
                                                phi_8112_ = true;
                                            } else {
                                                phi_8112_ = (_e291 >= _e282);
                                            }
                                            let _e366 = phi_8112_;
                                            let _e367 = select(_e282, _e291, _e366);
                                            if (_e349.member_1 != _e349.member_1) {
                                                phi_8127_ = true;
                                            } else {
                                                phi_8127_ = (_e367 <= _e349.member_1);
                                            }
                                            let _e371 = phi_8127_;
                                            phi_8059_ = type_52(select(_e349.member, _e358, _e362), select(_e349.member_1, _e367, _e371));
                                        } else {
                                            phi_8059_ = type_52();
                                        }
                                        let _e375 = phi_8059_;
                                        phi_8062_ = _e375;
                                        phi_8063_ = select(true, false, _e353);
                                    } else {
                                        phi_8062_ = type_52();
                                        phi_8063_ = true;
                                    }
                                    let _e378 = phi_8062_;
                                    let _e380 = phi_8063_;
                                    if _e380 {
                                        phi_8068_ = _e349;
                                    } else {
                                        phi_8068_ = _e378;
                                    }
                                    let _e382 = phi_8068_;
                                    if (_e382.member_1 < 0f) {
                                        phi_7863_ = bool();
                                        phi_7864_ = true;
                                    } else {
                                        let _e386 = (_e382.member > _e382.member_1);
                                        if _e386 {
                                            phi_7860_ = bool();
                                        } else {
                                            phi_7860_ = (_e382.member > _e256.member);
                                        }
                                        let _e389 = phi_7860_;
                                        phi_7863_ = _e389;
                                        phi_7864_ = _e386;
                                    }
                                    let _e391 = phi_7863_;
                                    let _e393 = phi_7864_;
                                    if !(select(_e391, true, _e393)) {
                                        if (_e264.member_3 > 0u) {
                                            phi_795_ = _e264.member_1;
                                            phi_796_ = _e259;
                                            phi_797_ = (_e264.member_1 + _e264.member_3);
                                        } else {
                                            let _e398 = (_e250 + 1u);
                                            let _e399 = (_e398 <= 100u);
                                            if _e399 {
                                                if _e260 {
                                                } else {
                                                    phi_13177_ = true;
                                                    phi_7745_ = bool();
                                                    phi_7743_ = type_49();
                                                    break;
                                                }
                                                local_1[_e259] = _e264.member_1;
                                                if (_e250 < 100u) {
                                                } else {
                                                    phi_13177_ = true;
                                                    phi_7745_ = bool();
                                                    phi_7743_ = type_49();
                                                    break;
                                                }
                                                local_1[_e250] = (_e262 + 1u);
                                            }
                                            phi_795_ = _e248;
                                            phi_796_ = select(_e259, _e398, _e399);
                                            phi_797_ = _e252;
                                        }
                                        let _e408 = phi_795_;
                                        let _e410 = phi_796_;
                                        let _e412 = phi_797_;
                                        phi_798_ = _e408;
                                        phi_799_ = _e410;
                                        phi_800_ = _e412;
                                    } else {
                                        phi_798_ = _e248;
                                        phi_799_ = _e259;
                                        phi_800_ = _e252;
                                    }
                                    let _e414 = phi_798_;
                                    let _e416 = phi_799_;
                                    let _e418 = phi_800_;
                                    phi_801_ = _e414;
                                    phi_802_ = _e416;
                                    phi_803_ = _e418;
                                }
                                let _e420 = phi_801_;
                                let _e422 = phi_802_;
                                let _e424 = phi_803_;
                                phi_373_ = _e420;
                                phi_376_ = _e422;
                                phi_378_ = _e424;
                                phi_380_ = _e254;
                                phi_382_ = _e256;
                                phi_805_ = type_49();
                                phi_806_ = select(true, false, _e258);
                                phi_807_ = _e258;
                            }
                            let _e673 = phi_373_;
                            let _e675 = phi_376_;
                            let _e677 = phi_378_;
                            let _e679 = phi_380_;
                            let _e681 = phi_382_;
                            let _e683 = phi_805_;
                            let _e685 = phi_806_;
                            let _e687 = phi_807_;
                            phi_372_ = _e673;
                            phi_375_ = _e675;
                            phi_377_ = _e677;
                            phi_379_ = _e679;
                            phi_381_ = _e681;
                            phi_13177_ = _e218;
                            phi_7745_ = _e687;
                            phi_7743_ = _e683;
                            if !(_e685) {
                                break;
                            }
                        }
                        let _e690 = phi_13177_;
                        let _e692 = phi_7745_;
                        let _e694 = phi_7743_;
                        phi_16967_ = _e690;
                        phi_7779_ = bool();
                        phi_7778_ = u32();
                        phi_7777_ = type_49();
                        phi_7776_ = bool();
                        phi_7775_ = bool();
                        phi_7774_ = bool();
                        phi_7773_ = bool();
                        phi_7772_ = bool();
                        phi_7771_ = bool();
                        phi_7770_ = bool();
                        phi_7769_ = vec3<f32>();
                        phi_7767_ = vec3<f32>();
                        phi_7766_ = u32();
                        phi_7759_ = u32();
                        phi_7757_ = type_14();
                        phi_7755_ = vec3<f32>();
                        phi_7754_ = f32();
                        phi_7753_ = f32();
                        phi_7752_ = f32();
                        phi_7750_ = vec3<f32>();
                        if _e690 {
                            break;
                        }
                        if _e692 {
                            let _e696 = local_2;
                            if (_e696.member_1 != 4294967295u) {
                                let _e700 = global_2.member[_e696.member_1];
                                if (_e700.member_6.x == 0u) {
                                    let _e749 = (_e700.member_1.x - _e700.member.x);
                                    let _e750 = (_e700.member_1.y - _e700.member.y);
                                    let _e751 = (_e700.member_1.z - _e700.member.z);
                                    let _e756 = (_e700.member_2.x - _e700.member.x);
                                    let _e757 = (_e700.member_2.y - _e700.member.y);
                                    let _e758 = (_e700.member_2.z - _e700.member.z);
                                    let _e761 = ((_e750 * _e758) - (_e757 * _e751));
                                    let _e764 = ((_e751 * _e756) - (_e758 * _e749));
                                    let _e767 = ((_e749 * _e757) - (_e756 * _e750));
                                    let _e774 = (1f / sqrt((((_e761 * _e761) + (_e764 * _e764)) + (_e767 * _e767))));
                                    let _e778 = vec3<f32>((_e761 * _e774), (_e764 * _e774), (_e767 * _e774));
                                    if (_e700.member_6.w == 1u) {
                                        let _e787 = local_4;
                                        let _e791 = local_5;
                                        let _e793 = ((1f - _e787.y) - _e791.z);
                                        let _e814 = (((_e700.member_3.x * _e793) + (_e700.member_4.x * _e787.y)) + (_e700.member_5.x * _e791.z));
                                        let _e815 = (((_e700.member_3.y * _e793) + (_e700.member_4.y * _e787.y)) + (_e700.member_5.y * _e791.z));
                                        let _e816 = (((_e700.member_3.z * _e793) + (_e700.member_4.z * _e787.y)) + (_e700.member_5.z * _e791.z));
                                        let _e821 = (((_e814 * _e814) + (_e815 * _e815)) + (_e816 * _e816));
                                        if (_e821 > 0f) {
                                            let _e824 = (1f / sqrt(_e821));
                                            phi_934_ = vec3<f32>((_e814 * _e824), (_e815 * _e824), (_e816 * _e824));
                                        } else {
                                            phi_934_ = _e778;
                                        }
                                        let _e830 = phi_934_;
                                        phi_935_ = _e830;
                                    } else {
                                        phi_935_ = _e778;
                                    }
                                    let _e832 = phi_935_;
                                    phi_976_ = type_53(_e778, _e832);
                                } else {
                                    if (_e700.member_6.x == 1u) {
                                        let _e711 = local_3;
                                        let _e737 = vec3<f32>((((_e236.x + (_e228.x * _e711.x)) - _e700.member.x) / _e700.member.w), (((_e236.y + (_e228.y * _e711.x)) - _e700.member.y) / _e700.member.w), (((_e236.z + (_e228.z * _e711.x)) - _e700.member.z) / _e700.member.w));
                                        phi_975_ = type_53(_e737, _e737);
                                    } else {
                                        let _e708 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e228.y > 0f)));
                                        phi_975_ = type_53(_e708, _e708);
                                    }
                                    let _e740 = phi_975_;
                                    phi_976_ = _e740;
                                }
                                let _e835 = phi_976_;
                                let _e839 = local_6;
                                let _e844 = type_49(_e839.member, _e839.member_1, _e835.member, _e839.member_3);
                                phi_981_ = type_49(_e844.member, _e844.member_1, _e844.member_2, _e835.member_1);
                            } else {
                                let _e6701 = local_52;
                                phi_981_ = _e6701;
                            }
                            let _e851 = phi_981_;
                            phi_982_ = _e851;
                        } else {
                            phi_982_ = type_49();
                        }
                        let _e853 = phi_982_;
                        if select(true, false, _e692) {
                            phi_987_ = _e694;
                        } else {
                            phi_987_ = _e853;
                        }
                        let _e856 = phi_987_;
                        let _e857 = (_e220 + 1u);
                        let _e859 = (_e856.member_1 == 4294967295u);
                        if _e859 {
                            phi_17062_ = _e216;
                            phi_16968_ = _e690;
                            phi_5786_ = vec3<f32>();
                            phi_5787_ = vec3<f32>();
                            phi_5788_ = f32();
                            phi_5789_ = f32();
                            phi_5790_ = f32();
                            phi_5791_ = vec3<f32>();
                            phi_5792_ = type_14();
                            phi_5793_ = u32();
                            phi_5794_ = u32();
                            phi_5795_ = vec3<f32>();
                            phi_335_ = u32();
                            phi_338_ = vec3<f32>();
                            phi_340_ = vec3<f32>();
                            phi_342_ = f32();
                            phi_344_ = vec3<f32>();
                            phi_346_ = u32();
                            phi_348_ = bool();
                            phi_350_ = bool();
                            phi_352_ = vec3<f32>();
                            phi_5804_ = false;
                            phi_5805_ = false;
                            phi_5806_ = false;
                            phi_5807_ = false;
                            phi_5808_ = false;
                            phi_5809_ = false;
                            phi_5810_ = false;
                            phi_5811_ = false;
                        } else {
                            if ((_e211 & 8u) == 0u) {
                                phi_1319_ = _e222;
                                phi_1320_ = _e224;
                            } else {
                                if (_e856.member > 100f) {
                                    let _e865 = global_1.member.member_8;
                                    let _e878 = global_1.member.member_9;
                                    let _e889 = (abs(atan2((((_e228.x * _e878.x) + (_e228.y * _e878.y)) + (_e228.z * _e878.z)), (((_e228.x * _e865.x) + (_e228.y * _e865.y)) + (_e228.z * _e865.z)))) * 0.31830987f);
                                    let _e891 = select(_e889, 0f, (_e889 < 0f));
                                    let _e894 = (select(_e891, 1f, (_e891 > 1f)) * 31f);
                                    let _e896 = ((_e228.y + 1f) * 0.5f);
                                    let _e898 = select(_e896, 0f, (_e896 < 0f));
                                    let _e901 = (select(_e898, 1f, (_e898 > 1f)) * 63f);
                                    let _e904 = (log((_e856.member * 0.01f)) * 0.13156332f);
                                    let _e906 = select(_e904, 0f, (_e904 < 0f));
                                    let _e909 = (select(_e906, 1f, (_e906 > 1f)) * 31f);
                                    let _e914 = select(select(u32(_e894), 0u, (_e894 < 0f)), 4294967295u, (_e894 > 4294967000f));
                                    let _e916 = select(_e914, 30u, (30u < _e914));
                                    let _e921 = select(select(u32(_e901), 0u, (_e901 < 0f)), 4294967295u, (_e901 > 4294967000f));
                                    let _e923 = select(_e921, 62u, (62u < _e921));
                                    let _e928 = select(select(u32(_e909), 0u, (_e909 < 0f)), 4294967295u, (_e909 > 4294967000f));
                                    let _e930 = select(_e928, 30u, (30u < _e928));
                                    let _e932 = (_e894 - f32(_e916));
                                    let _e934 = (_e901 - f32(_e923));
                                    let _e936 = (_e909 - f32(_e930));
                                    phi_1135_ = vec3<f32>(0f, 0f, 0f);
                                    phi_1138_ = vec3<f32>(0f, 0f, 0f);
                                    phi_1140_ = 0u;
                                    loop {
                                        let _e938 = phi_1135_;
                                        let _e940 = phi_1138_;
                                        let _e942 = phi_1140_;
                                        local_7 = _e938;
                                        local_8 = _e938;
                                        local_9 = _e938;
                                        local_10 = _e940;
                                        let _e943 = (_e942 < 8u);
                                        if _e943 {
                                            let _e944 = (_e942 & 1u);
                                            let _e947 = ((_e942 >> bitcast<u32>(1i)) & 1u);
                                            let _e950 = ((_e942 >> bitcast<u32>(2i)) & 1u);
                                            if (_e944 == 1u) {
                                                phi_1158_ = _e932;
                                            } else {
                                                phi_1158_ = (1f - _e932);
                                            }
                                            let _e954 = phi_1158_;
                                            if (_e947 == 1u) {
                                                phi_1164_ = _e934;
                                            } else {
                                                phi_1164_ = (1f - _e934);
                                            }
                                            let _e958 = phi_1164_;
                                            if (_e950 == 1u) {
                                                phi_1171_ = _e936;
                                            } else {
                                                phi_1171_ = (1f - _e936);
                                            }
                                            let _e963 = phi_1171_;
                                            let _e964 = ((_e954 * _e958) * _e963);
                                            let _e970 = global_1.member.member_7[2u];
                                            let _e976 = (_e970 + (6u * (((((_e916 + _e944) * 64u) + (_e923 + _e947)) * 32u) + (_e930 + _e950))));
                                            let _e978 = global_6.member[_e976];
                                            let _e981 = global_6.member[(_e976 + 1u)];
                                            let _e984 = global_6.member[(_e976 + 2u)];
                                            let _e987 = global_6.member[(_e976 + 3u)];
                                            let _e990 = global_6.member[(_e976 + 4u)];
                                            let _e993 = global_6.member[(_e976 + 5u)];
                                            phi_1136_ = vec3<f32>((_e938.x + (_e987 * _e964)), (_e938.y + (_e990 * _e964)), (_e938.z + (_e993 * _e964)));
                                            phi_1139_ = vec3<f32>((_e940.x + (_e978 * _e964)), (_e940.y + (_e981 * _e964)), (_e940.z + (_e984 * _e964)));
                                            phi_1141_ = (_e942 + 1u);
                                        } else {
                                            phi_1136_ = vec3<f32>();
                                            phi_1139_ = vec3<f32>();
                                            phi_1141_ = u32();
                                        }
                                        let _e1016 = phi_1136_;
                                        let _e1018 = phi_1139_;
                                        let _e1020 = phi_1141_;
                                        phi_1135_ = _e1016;
                                        phi_1138_ = _e1018;
                                        phi_1140_ = _e1020;
                                        if !(_e943) {
                                            break;
                                        }
                                    }
                                    let _e1024 = local_7;
                                    let _e1029 = local_8;
                                    let _e1034 = local_9;
                                    let _e1045 = local_10;
                                    phi_1317_ = vec3<f32>((_e222.x + (_e224.x * _e1024.x)), (_e222.y + (_e224.y * _e1029.y)), (_e222.z + (_e224.z * _e1034.z)));
                                    phi_1318_ = (_e224 * _e1045);
                                } else {
                                    phi_1317_ = _e222;
                                    phi_1318_ = _e224;
                                }
                                let _e1048 = phi_1317_;
                                let _e1050 = phi_1318_;
                                phi_1319_ = _e1048;
                                phi_1320_ = _e1050;
                            }
                            let _e1052 = phi_1319_;
                            let _e1054 = phi_1320_;
                            let _e1057 = (_e228.x * _e856.member);
                            let _e1059 = (_e228.y * _e856.member);
                            let _e1061 = (_e228.z * _e856.member);
                            let _e1063 = (_e236.x + _e1057);
                            let _e1065 = (_e236.y + _e1059);
                            let _e1067 = (_e236.z + _e1061);
                            let _e1071 = global_2.member[_e856.member_1];
                            phi_1351_ = 0i;
                            phi_1354_ = _e1071.member_6.y;
                            loop {
                                let _e1075 = phi_1351_;
                                let _e1077 = phi_1354_;
                                local_11 = _e1077;
                                local_58 = _e1077;
                                if (_e1075 < 4i) {
                                    let _e1080 = global_4.member[_e1077];
                                    let _e1082 = (_e1080.member != 6u);
                                    if _e1082 {
                                        phi_1423_ = i32();
                                        phi_1424_ = u32();
                                    } else {
                                        let _e1086 = (abs(_e1063) / _e1080.member_6.z);
                                        let _e1097 = (abs(_e1067) / _e1080.member_6.w);
                                        if ((select(select(u32(_e1086), 0u, (_e1086 < 0f)), 4294967295u, (_e1086 > 4294967000f)) % 2u) == 0u) {
                                            phi_1408_ = (_e1063 > 0f);
                                        } else {
                                            phi_1408_ = (_e1063 <= 0f);
                                        }
                                        let _e1108 = phi_1408_;
                                        if ((select(select(u32(_e1097), 0u, (_e1097 < 0f)), 4294967295u, (_e1097 > 4294967000f)) % 2u) == 0u) {
                                            phi_1414_ = (_e1067 > 0f);
                                        } else {
                                            phi_1414_ = (_e1067 <= 0f);
                                        }
                                        let _e1112 = phi_1414_;
                                        if (_e1108 != _e1112) {
                                            phi_1421_ = _e1080.member_1;
                                        } else {
                                            phi_1421_ = _e1080.member_2;
                                        }
                                        let _e1117 = phi_1421_;
                                        phi_1423_ = (_e1075 + 1i);
                                        phi_1424_ = _e1117;
                                    }
                                    let _e1120 = phi_1423_;
                                    let _e1122 = phi_1424_;
                                    phi_1352_ = _e1120;
                                    phi_1355_ = _e1122;
                                    phi_1426_ = select(true, false, _e1082);
                                } else {
                                    phi_1352_ = i32();
                                    phi_1355_ = u32();
                                    phi_1426_ = false;
                                }
                                let _e1125 = phi_1352_;
                                let _e1127 = phi_1355_;
                                let _e1129 = phi_1426_;
                                phi_1351_ = _e1125;
                                phi_1354_ = _e1127;
                                if !(_e1129) {
                                    break;
                                }
                            }
                            let _e1132 = local_11;
                            let _e1134 = global_4.member[_e1132];
                            let _e1136 = (_e1134.member == 4u);
                            if _e1136 {
                                phi_17063_ = _e216;
                                phi_16969_ = _e690;
                                phi_5762_ = u32();
                                phi_5763_ = vec3<f32>();
                                phi_5768_ = u32();
                                phi_5769_ = vec3<f32>();
                                phi_5770_ = vec3<f32>();
                                phi_5771_ = f32();
                                phi_5772_ = vec3<f32>();
                                phi_5773_ = u32();
                                phi_5774_ = bool();
                                phi_5775_ = bool();
                                phi_5776_ = vec3<f32>();
                                phi_5779_ = false;
                                phi_5780_ = false;
                                phi_5781_ = false;
                                phi_5782_ = false;
                                phi_5783_ = false;
                                phi_5784_ = false;
                                phi_5785_ = false;
                            } else {
                                let _e1137 = -(_e228.x);
                                let _e1138 = -(_e228.y);
                                let _e1139 = -(_e228.z);
                                let _e1140 = -(_e228);
                                if ((((_e856.member_2.x * _e1137) + (_e856.member_2.y * _e1138)) + (_e856.member_2.z * _e1139)) < 0f) {
                                    phi_1471_ = -(_e856.member_2);
                                } else {
                                    phi_1471_ = _e856.member_2;
                                }
                                let _e1153 = phi_1471_;
                                if ((((_e856.member_3.x * _e1137) + (_e856.member_3.y * _e1138)) + (_e856.member_3.z * _e1139)) < 0f) {
                                    phi_1488_ = -(_e856.member_3);
                                } else {
                                    phi_1488_ = _e856.member_3;
                                }
                                let _e1166 = phi_1488_;
                                let _e1169 = global_1.member.member_4[2u];
                                let _e1170 = abs(_e1063);
                                let _e1171 = abs(_e1065);
                                let _e1172 = abs(_e1067);
                                let _e1174 = select(_e1171, _e1170, (_e1170 > _e1171));
                                let _e1178 = (0.00002f * (1f + select(_e1172, _e1174, (_e1174 > _e1172))));
                                if (_e1169 != _e1169) {
                                    phi_8156_ = true;
                                } else {
                                    phi_8156_ = (_e1178 >= _e1169);
                                }
                                let _e1182 = phi_8156_;
                                let _e1184 = (select(_e1169, _e1178, _e1182) * 2f);
                                let _e1191 = (_e1063 + (_e1153.x * _e1184));
                                let _e1192 = (_e1065 + (_e1153.y * _e1184));
                                let _e1193 = (_e1067 + (_e1153.z * _e1184));
                                let _e1194 = vec3<f32>(_e1191, _e1192, _e1193);
                                if _e232 {
                                    let _e1197 = global_1.member.member_4[0u];
                                    phi_1519_ = _e1197;
                                } else {
                                    phi_1519_ = 0f;
                                }
                                let _e1199 = phi_1519_;
                                let _e1202 = select(-1f, 1f, (_e1166.z >= 0f));
                                let _e1204 = (-1f / (_e1202 + _e1166.z));
                                let _e1208 = ((_e1166.x * _e1166.y) * _e1204);
                                let _e1212 = (1f + (((_e1202 * _e1166.x) * _e1166.x) * _e1204));
                                let _e1213 = (_e1202 * _e1208);
                                let _e1215 = (-(_e1202) * _e1166.x);
                                let _e1218 = (_e1202 + ((_e1166.y * _e1166.y) * _e1204));
                                let _e1219 = -(_e1166.y);
                                if (_e1134.member == 0u) {
                                    phi_14041_ = vec3<f32>(0f, 0f, 0f);
                                    phi_13850_ = 1.5f;
                                    phi_13772_ = vec3<f32>(0f, 0f, 0f);
                                    phi_13696_ = vec3<f32>(_e1134.member_4.x, _e1134.member_4.y, _e1134.member_4.z);
                                    phi_13618_ = 0u;
                                    phi_13537_ = 1f;
                                    phi_13461_ = 0u;
                                    phi_13446_ = _e216;
                                    phi_2210_ = vec3<f32>();
                                    phi_2211_ = vec3<f32>();
                                    phi_2212_ = u32();
                                    phi_2213_ = vec3<f32>();
                                    phi_2214_ = true;
                                    phi_2215_ = false;
                                    phi_2216_ = false;
                                    phi_2217_ = false;
                                } else {
                                    if (_e1134.member == 1u) {
                                        let _e1768 = (_e1134.member_6.y * _e1134.member_6.y);
                                        if (_e1768 != _e1768) {
                                            phi_8848_ = true;
                                        } else {
                                            phi_8848_ = (_e1199 >= _e1768);
                                        }
                                        let _e1772 = phi_8848_;
                                        let _e1773 = select(_e1768, _e1199, _e1772);
                                        let _e1774 = (_e1773 >= 0.001f);
                                        if _e1774 {
                                            phi_14098_ = vec3<f32>(_e1134.member_4.x, _e1134.member_4.y, _e1134.member_4.z);
                                            phi_13893_ = _e1134.member_6.x;
                                            phi_13748_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13440_ = _e216;
                                            phi_1594_ = vec3<f32>();
                                            phi_1595_ = true;
                                        } else {
                                            let _e1776 = ((_e216 * 747796405u) + 2891336453u);
                                            let _e1784 = (((_e1776 >> bitcast<u32>((((_e1776 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1776) * 277803737u);
                                            let _e1787 = ((_e1784 >> bitcast<u32>(22i)) ^ _e1784);
                                            let _e1797 = abs((((_e1137 * _e1166.x) + (_e1138 * _e1166.y)) + (_e1139 * _e1166.z)));
                                            let _e1800 = (1f / _e1134.member_6.x);
                                            let _e1804 = ((_e1800 * _e1800) * (1f - (_e1797 * _e1797)));
                                            if (_e1804 >= 1f) {
                                                phi_8888_ = 1f;
                                            } else {
                                                let _e1807 = sqrt((1f - _e1804));
                                                if (_e1797 != _e1797) {
                                                    phi_8899_ = true;
                                                } else {
                                                    phi_8899_ = (_e1807 <= _e1797);
                                                }
                                                let _e1811 = phi_8899_;
                                                let _e1816 = pow(((1f - _e1134.member_6.x) / (1f + _e1134.member_6.x)), 2f);
                                                phi_8888_ = (_e1816 + ((1f - _e1816) * pow((1f - select(_e1797, _e1807, _e1811)), 5f)));
                                            }
                                            let _e1823 = phi_8888_;
                                            let _e1824 = ((f32((_e1787 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1823);
                                            if _e1824 {
                                                let _e1837 = (((_e1166.x * _e228.x) + (_e1166.y * _e228.y)) + (_e1166.z * _e228.z));
                                                phi_13749_ = vec3<f32>(0f, 0f, 0f);
                                                phi_1592_ = (_e228 - vec3<f32>(((_e1166.x * 2f) * _e1837), ((_e1166.y * 2f) * _e1837), ((_e1166.z * 2f) * _e1837)));
                                            } else {
                                                phi_13749_ = vec3<f32>(_e1134.member_4.x, _e1134.member_4.y, _e1134.member_4.z);
                                                phi_1592_ = vec3<f32>();
                                            }
                                            let _e1844 = phi_13749_;
                                            let _e1846 = phi_1592_;
                                            phi_14098_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13893_ = 1.5f;
                                            phi_13748_ = _e1844;
                                            phi_13440_ = _e1787;
                                            phi_1594_ = _e1846;
                                            phi_1595_ = select(true, false, _e1824);
                                        }
                                        let _e1856 = phi_14098_;
                                        let _e1858 = phi_13893_;
                                        let _e1860 = phi_13748_;
                                        let _e1862 = phi_13440_;
                                        let _e1864 = phi_1594_;
                                        let _e1866 = phi_1595_;
                                        phi_14044_ = _e1856;
                                        phi_13853_ = _e1858;
                                        phi_13775_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13699_ = _e1860;
                                        phi_13621_ = select(0u, 1u, _e1774);
                                        phi_13540_ = select(1f, _e1773, _e1774);
                                        phi_13464_ = select(0u, 2u, _e1774);
                                        phi_13391_ = _e1862;
                                        phi_2080_ = _e1864;
                                        phi_2081_ = vec3<f32>(1f, 1f, 1f);
                                        phi_2082_ = _e1194;
                                        phi_2083_ = _e1866;
                                    } else {
                                        if (_e1134.member == 3u) {
                                            phi_13394_ = _e216;
                                            phi_1759_ = vec3<f32>();
                                            phi_1760_ = vec3<f32>();
                                            phi_1761_ = vec3<f32>();
                                            phi_1762_ = true;
                                            phi_1763_ = false;
                                        } else {
                                            let _e1223 = (_e1134.member == 5u);
                                            if _e1223 {
                                                phi_13395_ = _e216;
                                                phi_1755_ = vec3<f32>();
                                                phi_1756_ = vec3<f32>();
                                                phi_1757_ = vec3<f32>();
                                                phi_1758_ = false;
                                            } else {
                                                let _e1224 = (_e1134.member == 2u);
                                                if _e1224 {
                                                    let _e1229 = (((_e228.x * _e856.member_3.x) + (_e228.y * _e856.member_3.y)) + (_e228.z * _e856.member_3.z));
                                                    let _e1230 = (_e1229 > 0f);
                                                    if _e1230 {
                                                        phi_1631_ = -(_e856.member_3);
                                                    } else {
                                                        phi_1631_ = _e856.member_3;
                                                    }
                                                    let _e1233 = phi_1631_;
                                                    if _e1230 {
                                                        phi_1637_ = _e1134.member_6.x;
                                                    } else {
                                                        phi_1637_ = (1f / _e1134.member_6.x);
                                                    }
                                                    let _e1238 = phi_1637_;
                                                    let _e1239 = abs(_e1229);
                                                    let _e1240 = (_e1238 * _e1238);
                                                    let _e1243 = (_e1240 * (1f - (_e1239 * _e1239)));
                                                    if (_e1243 >= 1f) {
                                                        phi_8218_ = 1f;
                                                    } else {
                                                        let _e1246 = sqrt((1f - _e1243));
                                                        if (_e1239 != _e1239) {
                                                            phi_8229_ = true;
                                                        } else {
                                                            phi_8229_ = (_e1246 <= _e1239);
                                                        }
                                                        let _e1250 = phi_8229_;
                                                        let _e1255 = pow(((1f - _e1134.member_6.x) / (1f + _e1134.member_6.x)), 2f);
                                                        phi_8218_ = (_e1255 + ((1f - _e1255) * pow((1f - select(_e1239, _e1246, _e1250)), 5f)));
                                                    }
                                                    let _e1262 = phi_8218_;
                                                    let _e1270 = (((_e228.x * _e1233.x) + (_e228.y * _e1233.y)) + (_e228.z * _e1233.z));
                                                    let _e1274 = (1f - (_e1240 * (1f - (_e1270 * _e1270))));
                                                    if (_e1274 > 0f) {
                                                        let _e1277 = ((_e216 * 747796405u) + 2891336453u);
                                                        let _e1285 = (((_e1277 >> bitcast<u32>((((_e1277 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1277) * 277803737u);
                                                        let _e1288 = ((_e1285 >> bitcast<u32>(22i)) ^ _e1285);
                                                        let _e1293 = ((f32((_e1288 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1262);
                                                        if _e1293 {
                                                            let _e1303 = sqrt(_e1274);
                                                            let _e1307 = (((_e228.x - (_e1233.x * _e1270)) * _e1238) - (_e1233.x * _e1303));
                                                            let _e1308 = (((_e228.y - (_e1233.y * _e1270)) * _e1238) - (_e1233.y * _e1303));
                                                            let _e1309 = (((_e228.z - (_e1233.z * _e1270)) * _e1238) - (_e1233.z * _e1303));
                                                            let _e1316 = (1f / sqrt((((_e1307 * _e1307) + (_e1308 * _e1308)) + (_e1309 * _e1309))));
                                                            phi_1727_ = vec3<f32>((_e1307 * _e1316), (_e1308 * _e1316), (_e1309 * _e1316));
                                                            phi_1728_ = vec3<f32>(_e1134.member_4.x, _e1134.member_4.y, _e1134.member_4.z);
                                                            phi_1729_ = vec3<f32>((_e1063 - (_e1233.x * _e1184)), (_e1065 - (_e1233.y * _e1184)), (_e1067 - (_e1233.z * _e1184)));
                                                        } else {
                                                            phi_1727_ = vec3<f32>();
                                                            phi_1728_ = vec3<f32>();
                                                            phi_1729_ = vec3<f32>();
                                                        }
                                                        let _e1334 = phi_1727_;
                                                        let _e1336 = phi_1728_;
                                                        let _e1338 = phi_1729_;
                                                        phi_13398_ = _e1288;
                                                        phi_1731_ = _e1334;
                                                        phi_1732_ = _e1336;
                                                        phi_1733_ = _e1338;
                                                        phi_1734_ = select(true, false, _e1293);
                                                    } else {
                                                        phi_13398_ = _e216;
                                                        phi_1731_ = vec3<f32>();
                                                        phi_1732_ = vec3<f32>();
                                                        phi_1733_ = vec3<f32>();
                                                        phi_1734_ = true;
                                                    }
                                                    let _e1341 = phi_13398_;
                                                    let _e1343 = phi_1731_;
                                                    let _e1345 = phi_1732_;
                                                    let _e1347 = phi_1733_;
                                                    let _e1349 = phi_1734_;
                                                    if _e1349 {
                                                        let _e1357 = (((_e856.member_3.x * _e228.x) + (_e856.member_3.y * _e228.y)) + (_e856.member_3.z * _e228.z));
                                                        phi_1749_ = (_e228 - vec3<f32>(((_e856.member_3.x * 2f) * _e1357), ((_e856.member_3.y * 2f) * _e1357), ((_e856.member_3.z * 2f) * _e1357)));
                                                        phi_1751_ = vec3<f32>((_e1063 + (_e1233.x * _e1184)), (_e1065 + (_e1233.y * _e1184)), (_e1067 + (_e1233.z * _e1184)));
                                                    } else {
                                                        phi_1749_ = _e1343;
                                                        phi_1751_ = _e1347;
                                                    }
                                                    let _e1371 = phi_1749_;
                                                    let _e1373 = phi_1751_;
                                                    phi_13396_ = _e1341;
                                                    phi_1752_ = _e1371;
                                                    phi_1753_ = select(_e1345, vec3<f32>(1f, 1f, 1f), vec3(_e1349));
                                                    phi_1754_ = _e1373;
                                                } else {
                                                    phi_13396_ = _e216;
                                                    phi_1752_ = vec3<f32>();
                                                    phi_1753_ = vec3<f32>();
                                                    phi_1754_ = vec3<f32>();
                                                }
                                                let _e1377 = phi_13396_;
                                                let _e1379 = phi_1752_;
                                                let _e1381 = phi_1753_;
                                                let _e1383 = phi_1754_;
                                                phi_13395_ = _e1377;
                                                phi_1755_ = _e1379;
                                                phi_1756_ = _e1381;
                                                phi_1757_ = _e1383;
                                                phi_1758_ = _e1224;
                                            }
                                            let _e1385 = phi_13395_;
                                            let _e1387 = phi_1755_;
                                            let _e1389 = phi_1756_;
                                            let _e1391 = phi_1757_;
                                            let _e1393 = phi_1758_;
                                            phi_13394_ = _e1385;
                                            phi_1759_ = _e1387;
                                            phi_1760_ = _e1389;
                                            phi_1761_ = _e1391;
                                            phi_1762_ = _e1223;
                                            phi_1763_ = _e1393;
                                        }
                                        let _e1395 = phi_13394_;
                                        let _e1397 = phi_1759_;
                                        let _e1399 = phi_1760_;
                                        let _e1401 = phi_1761_;
                                        let _e1403 = phi_1762_;
                                        let _e1405 = phi_1763_;
                                        if _e1403 {
                                            let _e1408 = (_e1134.member_6.y * _e1134.member_6.y);
                                            if (_e1408 != _e1408) {
                                                phi_8302_ = true;
                                            } else {
                                                phi_8302_ = (_e1199 >= _e1408);
                                            }
                                            let _e1412 = phi_8302_;
                                            let _e1413 = select(_e1408, _e1199, _e1412);
                                            let _e1425 = select(2u, 0u, (_e1134.member == 5u));
                                            let _e1426 = (_e1413 < 0.001f);
                                            if _e1426 {
                                                let _e1434 = (((_e1166.x * _e228.x) + (_e1166.y * _e228.y)) + (_e1166.z * _e228.z));
                                                let _e1445 = abs((((_e1137 * _e1166.x) + (_e1138 * _e1166.y)) + (_e1139 * _e1166.z)));
                                                if (_e1425 == 0u) {
                                                    let _e1727 = (1f - _e1445);
                                                    if (_e1727 != _e1727) {
                                                        phi_8833_ = true;
                                                    } else {
                                                        phi_8833_ = (0f >= _e1727);
                                                    }
                                                    let _e1731 = phi_8833_;
                                                    let _e1733 = pow(select(_e1727, 0f, _e1731), 5f);
                                                    let _e1734 = (1f - _e1733);
                                                    phi_2066_ = vec3<f32>(((_e1134.member_4.x * _e1734) + _e1733), ((_e1134.member_4.y * _e1734) + _e1733), ((_e1134.member_4.z * _e1734) + _e1733));
                                                } else {
                                                    if (_e1425 == 1u) {
                                                        let _e1709 = (0.44444448f * (1f - (_e1445 * _e1445)));
                                                        if (_e1709 >= 1f) {
                                                            phi_8794_ = 1f;
                                                        } else {
                                                            let _e1712 = sqrt((1f - _e1709));
                                                            if (_e1445 != _e1445) {
                                                                phi_8805_ = true;
                                                            } else {
                                                                phi_8805_ = (_e1712 <= _e1445);
                                                            }
                                                            let _e1716 = phi_8805_;
                                                            phi_8794_ = (0.040000003f + (0.96f * pow((1f - select(_e1445, _e1712, _e1716)), 5f)));
                                                        }
                                                        let _e1723 = phi_8794_;
                                                        phi_2065_ = vec3(_e1723);
                                                    } else {
                                                        let _e1449 = select(_e1445, 0f, (_e1445 < 0f));
                                                        let _e1451 = select(_e1449, 1f, (_e1449 > 1f));
                                                        let _e1453 = (1f - (_e1451 * _e1451));
                                                        let _e1460 = ((_e1134.member_4.x * _e1134.member_4.x) - (_e1134.member_5.x * _e1134.member_5.x));
                                                        let _e1461 = ((_e1134.member_4.y * _e1134.member_4.y) - (_e1134.member_5.y * _e1134.member_5.y));
                                                        let _e1462 = ((_e1134.member_4.z * _e1134.member_4.z) - (_e1134.member_5.z * _e1134.member_5.z));
                                                        let _e1464 = (_e1134.member_4.x * (_e1134.member_5.x + _e1134.member_5.x));
                                                        let _e1466 = (_e1134.member_4.y * (_e1134.member_5.y + _e1134.member_5.y));
                                                        let _e1468 = (_e1134.member_4.z * (_e1134.member_5.z + _e1134.member_5.z));
                                                        let _e1475 = ((_e1460 * _e1460) + (_e1464 * _e1464));
                                                        let _e1476 = ((_e1461 * _e1461) + (_e1466 * _e1466));
                                                        let _e1477 = ((_e1462 * _e1462) + (_e1468 * _e1468));
                                                        let _e1482 = ((_e1453 * _e1460) / _e1475);
                                                        let _e1483 = ((_e1453 * _e1461) / _e1476);
                                                        let _e1484 = ((_e1453 * _e1462) / _e1477);
                                                        let _e1485 = (1f - _e1482);
                                                        let _e1486 = (1f - _e1483);
                                                        let _e1487 = (1f - _e1484);
                                                        let _e1489 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1482, _e1483, _e1484));
                                                        let _e1490 = (_e1453 * _e1464);
                                                        let _e1491 = (_e1453 * _e1466);
                                                        let _e1492 = (_e1453 * _e1468);
                                                        let _e1497 = (vec3<f32>(_e1490, _e1491, _e1492) / vec3<f32>(_e1475, _e1476, _e1477));
                                                        let _e1510 = ((_e1489.x * _e1489.x) + (_e1497.x * _e1497.x));
                                                        let _e1511 = ((_e1489.y * _e1489.y) + (_e1497.y * _e1497.y));
                                                        let _e1512 = ((_e1489.z * _e1489.z) + (_e1497.z * _e1497.z));
                                                        if (_e1510 != _e1510) {
                                                            phi_8464_ = true;
                                                        } else {
                                                            phi_8464_ = (0f >= _e1510);
                                                        }
                                                        let _e1516 = phi_8464_;
                                                        let _e1518 = sqrt(select(_e1510, 0f, _e1516));
                                                        if (_e1511 != _e1511) {
                                                            phi_8479_ = true;
                                                        } else {
                                                            phi_8479_ = (0f >= _e1511);
                                                        }
                                                        let _e1522 = phi_8479_;
                                                        let _e1524 = sqrt(select(_e1511, 0f, _e1522));
                                                        if (_e1512 != _e1512) {
                                                            phi_8494_ = true;
                                                        } else {
                                                            phi_8494_ = (0f >= _e1512);
                                                        }
                                                        let _e1528 = phi_8494_;
                                                        let _e1530 = sqrt(select(_e1512, 0f, _e1528));
                                                        let _e1534 = ((_e1518 + _e1485) * 0.5f);
                                                        let _e1535 = ((_e1524 + _e1486) * 0.5f);
                                                        let _e1536 = ((_e1530 + _e1487) * 0.5f);
                                                        if (_e1534 != _e1534) {
                                                            phi_8521_ = true;
                                                        } else {
                                                            phi_8521_ = (0f >= _e1534);
                                                        }
                                                        let _e1540 = phi_8521_;
                                                        let _e1542 = sqrt(select(_e1534, 0f, _e1540));
                                                        if (_e1535 != _e1535) {
                                                            phi_8536_ = true;
                                                        } else {
                                                            phi_8536_ = (0f >= _e1535);
                                                        }
                                                        let _e1546 = phi_8536_;
                                                        let _e1548 = sqrt(select(_e1535, 0f, _e1546));
                                                        if (_e1536 != _e1536) {
                                                            phi_8551_ = true;
                                                        } else {
                                                            phi_8551_ = (0f >= _e1536);
                                                        }
                                                        let _e1552 = phi_8551_;
                                                        let _e1554 = sqrt(select(_e1536, 0f, _e1552));
                                                        let _e1555 = vec3<f32>(_e1542, _e1548, _e1554);
                                                        let _e1559 = ((_e1518 - _e1485) * 0.5f);
                                                        let _e1560 = ((_e1524 - _e1486) * 0.5f);
                                                        let _e1561 = ((_e1530 - _e1487) * 0.5f);
                                                        if (_e1559 != _e1559) {
                                                            phi_8578_ = true;
                                                        } else {
                                                            phi_8578_ = (0f >= _e1559);
                                                        }
                                                        let _e1565 = phi_8578_;
                                                        let _e1567 = sqrt(select(_e1559, 0f, _e1565));
                                                        if (_e1560 != _e1560) {
                                                            phi_8593_ = true;
                                                        } else {
                                                            phi_8593_ = (0f >= _e1560);
                                                        }
                                                        let _e1571 = phi_8593_;
                                                        let _e1573 = sqrt(select(_e1560, 0f, _e1571));
                                                        if (_e1561 != _e1561) {
                                                            phi_8608_ = true;
                                                        } else {
                                                            phi_8608_ = (0f >= _e1561);
                                                        }
                                                        let _e1577 = phi_8608_;
                                                        let _e1579 = sqrt(select(_e1561, 0f, _e1577));
                                                        if ((_e1490 / _e1475) < 0f) {
                                                            phi_1942_ = -(_e1567);
                                                        } else {
                                                            phi_1942_ = _e1567;
                                                        }
                                                        let _e1583 = phi_1942_;
                                                        if ((_e1491 / _e1476) < 0f) {
                                                            phi_1950_ = -(_e1573);
                                                        } else {
                                                            phi_1950_ = _e1573;
                                                        }
                                                        let _e1587 = phi_1950_;
                                                        if ((_e1492 / _e1477) < 0f) {
                                                            phi_1958_ = -(_e1579);
                                                        } else {
                                                            phi_1958_ = _e1579;
                                                        }
                                                        let _e1591 = phi_1958_;
                                                        let _e1595 = (_e1134.member_5.x * _e1451);
                                                        let _e1596 = (_e1134.member_5.y * _e1451);
                                                        let _e1597 = (_e1134.member_5.z * _e1451);
                                                        let _e1607 = vec3<f32>(((_e1134.member_4.x * _e1542) - (_e1134.member_5.x * _e1583)), ((_e1134.member_4.y * _e1548) - (_e1134.member_5.y * _e1587)), ((_e1134.member_4.z * _e1554) - (_e1134.member_5.z * _e1591)));
                                                        let _e1614 = ((_e1134.member_4.x * _e1583) + (_e1134.member_5.x * _e1542));
                                                        let _e1615 = ((_e1134.member_4.y * _e1587) + (_e1134.member_5.y * _e1548));
                                                        let _e1616 = ((_e1134.member_4.z * _e1591) + (_e1134.member_5.z * _e1554));
                                                        let _e1618 = vec3<f32>((_e1134.member_4.x * _e1451), (_e1134.member_4.y * _e1451), (_e1134.member_4.z * _e1451));
                                                        let _e1619 = (_e1618 - _e1555);
                                                        let _e1620 = (_e1595 - _e1583);
                                                        let _e1621 = (_e1596 - _e1587);
                                                        let _e1622 = (_e1597 - _e1591);
                                                        let _e1635 = (_e1618 + _e1555);
                                                        let _e1636 = (_e1595 + _e1583);
                                                        let _e1637 = (_e1596 + _e1587);
                                                        let _e1638 = (_e1597 + _e1591);
                                                        let _e1648 = ((_e1635.x * _e1635.x) + (_e1636 * _e1636));
                                                        let _e1649 = ((_e1635.y * _e1635.y) + (_e1637 * _e1637));
                                                        let _e1650 = ((_e1635.z * _e1635.z) + (_e1638 * _e1638));
                                                        let _e1660 = vec3(_e1451);
                                                        let _e1661 = (_e1660 - _e1607);
                                                        let _e1662 = -(vec3<f32>(_e1614, _e1615, _e1616));
                                                        let _e1678 = (_e1660 + _e1607);
                                                        let _e1688 = ((_e1678.x * _e1678.x) + (_e1614 * _e1614));
                                                        let _e1689 = ((_e1678.y * _e1678.y) + (_e1615 * _e1615));
                                                        let _e1690 = ((_e1678.z * _e1678.z) + (_e1616 * _e1616));
                                                        phi_2065_ = vec3<f32>((((((_e1619.x * _e1619.x) + (_e1620 * _e1620)) / select(0.000000000000000000000000000001f, _e1648, (_e1648 > 0.000000000000000000000000000001f))) + (((_e1661.x * _e1661.x) + (_e1662.x * _e1662.x)) / select(0.000000000000000000000000000001f, _e1688, (_e1688 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1619.y * _e1619.y) + (_e1621 * _e1621)) / select(0.000000000000000000000000000001f, _e1649, (_e1649 > 0.000000000000000000000000000001f))) + (((_e1661.y * _e1661.y) + (_e1662.y * _e1662.y)) / select(0.000000000000000000000000000001f, _e1689, (_e1689 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1619.z * _e1619.z) + (_e1622 * _e1622)) / select(0.000000000000000000000000000001f, _e1650, (_e1650 > 0.000000000000000000000000000001f))) + (((_e1661.z * _e1661.z) + (_e1662.z * _e1662.z)) / select(0.000000000000000000000000000001f, _e1690, (_e1690 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e1726 = phi_2065_;
                                                    phi_2066_ = _e1726;
                                                }
                                                let _e1743 = phi_2066_;
                                                phi_2067_ = (_e228 - vec3<f32>(((_e1166.x * 2f) * _e1434), ((_e1166.y * 2f) * _e1434), ((_e1166.z * 2f) * _e1434)));
                                                phi_2068_ = _e1743;
                                            } else {
                                                phi_2067_ = vec3<f32>();
                                                phi_2068_ = vec3<f32>();
                                            }
                                            let _e1745 = phi_2067_;
                                            let _e1747 = phi_2068_;
                                            phi_13777_ = vec3<f32>(_e1134.member_5.x, _e1134.member_5.y, _e1134.member_5.z);
                                            phi_13701_ = vec3<f32>(_e1134.member_4.x, _e1134.member_4.y, _e1134.member_4.z);
                                            phi_13623_ = _e1425;
                                            phi_13542_ = _e1413;
                                            phi_2069_ = _e1745;
                                            phi_2070_ = _e1747;
                                            phi_2072_ = _e1426;
                                        } else {
                                            phi_13777_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13701_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13623_ = 0u;
                                            phi_13542_ = 1f;
                                            phi_2069_ = _e1397;
                                            phi_2070_ = _e1399;
                                            phi_2072_ = _e1405;
                                        }
                                        let _e1749 = phi_13777_;
                                        let _e1751 = phi_13701_;
                                        let _e1753 = phi_13623_;
                                        let _e1755 = phi_13542_;
                                        let _e1757 = phi_2069_;
                                        let _e1759 = phi_2070_;
                                        let _e1761 = phi_2072_;
                                        phi_14044_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13853_ = 1.5f;
                                        phi_13775_ = _e1749;
                                        phi_13699_ = _e1751;
                                        phi_13621_ = _e1753;
                                        phi_13540_ = _e1755;
                                        phi_13464_ = select(0u, 1u, _e1403);
                                        phi_13391_ = _e1395;
                                        phi_2080_ = _e1757;
                                        phi_2081_ = _e1759;
                                        phi_2082_ = select(_e1401, _e1194, vec3(_e1403));
                                        phi_2083_ = select(true, false, _e1761);
                                    }
                                    let _e1871 = phi_14044_;
                                    let _e1873 = phi_13853_;
                                    let _e1875 = phi_13775_;
                                    let _e1877 = phi_13699_;
                                    let _e1879 = phi_13621_;
                                    let _e1881 = phi_13540_;
                                    let _e1883 = phi_13464_;
                                    let _e1885 = phi_13391_;
                                    let _e1887 = phi_2080_;
                                    let _e1889 = phi_2081_;
                                    let _e1891 = phi_2082_;
                                    let _e1893 = phi_2083_;
                                    let _e1894 = select(true, false, _e1893);
                                    if _e1894 {
                                        let _e1895 = (_e230 >= _e208);
                                        if _e1895 {
                                            phi_13448_ = _e1885;
                                            phi_2198_ = vec3<f32>();
                                            phi_2199_ = vec3<f32>();
                                            phi_2200_ = u32();
                                            phi_2201_ = false;
                                            phi_2202_ = false;
                                        } else {
                                            let _e1907 = global_1.member.member_4[1u];
                                            if (sqrt((((_e1889.x * _e1889.x) + (_e1889.y * _e1889.y)) + (_e1889.z * _e1889.z))) <= _e1907) {
                                                phi_13450_ = _e1885;
                                                phi_2163_ = _e1054;
                                                phi_2164_ = false;
                                            } else {
                                                let _e1911 = global_1.member.member_7[1u];
                                                if (_e1911 == 1u) {
                                                    let _e1916 = global_1.member.member_4[3u];
                                                    if (f32(_e230) >= _e1916) {
                                                        let _e1919 = select(_e1889.y, _e1889.x, (_e1889.x > _e1889.y));
                                                        let _e1921 = select(_e1889.z, _e1919, (_e1919 > _e1889.z));
                                                        if (_e1921 != _e1921) {
                                                            phi_8965_ = true;
                                                        } else {
                                                            phi_8965_ = (1f <= _e1921);
                                                        }
                                                        let _e1925 = phi_8965_;
                                                        let _e1926 = select(_e1921, 1f, _e1925);
                                                        let _e1928 = ((_e1885 * 747796405u) + 2891336453u);
                                                        let _e1936 = (((_e1928 >> bitcast<u32>((((_e1928 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1928) * 277803737u);
                                                        let _e1939 = ((_e1936 >> bitcast<u32>(22i)) ^ _e1936);
                                                        let _e1944 = ((f32((_e1939 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1926);
                                                        if _e1944 {
                                                            phi_2152_ = vec3<f32>();
                                                        } else {
                                                            phi_2152_ = vec3<f32>((_e1054.x / _e1926), (_e1054.y / _e1926), (_e1054.z / _e1926));
                                                        }
                                                        let _e1953 = phi_2152_;
                                                        phi_13453_ = _e1939;
                                                        phi_2154_ = _e1953;
                                                        phi_2155_ = select(true, false, _e1944);
                                                    } else {
                                                        phi_13453_ = _e1885;
                                                        phi_2154_ = _e1054;
                                                        phi_2155_ = true;
                                                    }
                                                    let _e1956 = phi_13453_;
                                                    let _e1958 = phi_2154_;
                                                    let _e1960 = phi_2155_;
                                                    phi_13452_ = _e1956;
                                                    phi_2156_ = _e1958;
                                                    phi_2157_ = _e1960;
                                                } else {
                                                    phi_13452_ = _e1885;
                                                    phi_2156_ = _e1054;
                                                    phi_2157_ = true;
                                                }
                                                let _e1962 = phi_13452_;
                                                let _e1964 = phi_2156_;
                                                let _e1966 = phi_2157_;
                                                phi_13450_ = _e1962;
                                                phi_2163_ = select(_e1054, _e1964, vec3(_e1966));
                                                phi_2164_ = select(false, true, _e1966);
                                            }
                                            let _e1971 = phi_13450_;
                                            let _e1973 = phi_2163_;
                                            let _e1975 = phi_2164_;
                                            if _e1975 {
                                                phi_2194_ = (_e1973 * _e1889);
                                                phi_2195_ = (_e1887 * (1f / sqrt((((_e1887.x * _e1887.x) + (_e1887.y * _e1887.y)) + (_e1887.z * _e1887.z)))));
                                                phi_2196_ = (_e230 + 1u);
                                            } else {
                                                phi_2194_ = vec3<f32>();
                                                phi_2195_ = vec3<f32>();
                                                phi_2196_ = u32();
                                            }
                                            let _e1990 = phi_2194_;
                                            let _e1992 = phi_2195_;
                                            let _e1994 = phi_2196_;
                                            phi_13448_ = _e1971;
                                            phi_2198_ = _e1990;
                                            phi_2199_ = _e1992;
                                            phi_2200_ = _e1994;
                                            phi_2201_ = _e1975;
                                            phi_2202_ = select(true, false, _e1975);
                                        }
                                        let _e1997 = phi_13448_;
                                        let _e1999 = phi_2198_;
                                        let _e2001 = phi_2199_;
                                        let _e2003 = phi_2200_;
                                        let _e2005 = phi_2201_;
                                        let _e2007 = phi_2202_;
                                        phi_13447_ = _e1997;
                                        phi_2203_ = _e1999;
                                        phi_2204_ = _e2001;
                                        phi_2205_ = _e2003;
                                        phi_2207_ = _e2005;
                                        phi_2208_ = _e2007;
                                        phi_2209_ = _e1895;
                                    } else {
                                        phi_13447_ = _e1885;
                                        phi_2203_ = vec3<f32>();
                                        phi_2204_ = vec3<f32>();
                                        phi_2205_ = u32();
                                        phi_2207_ = false;
                                        phi_2208_ = false;
                                        phi_2209_ = false;
                                    }
                                    let _e2009 = phi_13447_;
                                    let _e2011 = phi_2203_;
                                    let _e2013 = phi_2204_;
                                    let _e2015 = phi_2205_;
                                    let _e2017 = phi_2207_;
                                    let _e2019 = phi_2208_;
                                    let _e2021 = phi_2209_;
                                    phi_14041_ = _e1871;
                                    phi_13850_ = _e1873;
                                    phi_13772_ = _e1875;
                                    phi_13696_ = _e1877;
                                    phi_13618_ = _e1879;
                                    phi_13537_ = _e1881;
                                    phi_13461_ = _e1883;
                                    phi_13446_ = _e2009;
                                    phi_2210_ = _e2011;
                                    phi_2211_ = _e2013;
                                    phi_2212_ = _e2015;
                                    phi_2213_ = _e1891;
                                    phi_2214_ = select(true, false, _e1894);
                                    phi_2215_ = _e2017;
                                    phi_2216_ = _e2019;
                                    phi_2217_ = _e2021;
                                }
                                let _e2029 = phi_14041_;
                                let _e2031 = phi_13850_;
                                let _e2033 = phi_13772_;
                                let _e2035 = phi_13696_;
                                let _e2037 = phi_13618_;
                                let _e2039 = phi_13537_;
                                let _e2041 = phi_13461_;
                                let _e2043 = phi_13446_;
                                let _e2045 = phi_2210_;
                                let _e2047 = phi_2211_;
                                let _e2049 = phi_2212_;
                                let _e2051 = phi_2213_;
                                let _e2053 = phi_2214_;
                                let _e2055 = phi_2215_;
                                let _e2057 = phi_2216_;
                                let _e2059 = phi_2217_;
                                if _e2053 {
                                    let _e2062 = global_1.member.member_1[3u];
                                    phi_15202_ = _e690;
                                    phi_13445_ = _e2043;
                                    phi_2226_ = _e857;
                                    phi_2229_ = _e1052;
                                    phi_2231_ = 0u;
                                    loop {
                                        let _e2064 = phi_15202_;
                                        let _e2066 = phi_13445_;
                                        let _e2068 = phi_2226_;
                                        let _e2070 = phi_2229_;
                                        let _e2072 = phi_2231_;
                                        local_54 = _e2068;
                                        local_55 = _e2070;
                                        local_56 = _e2068;
                                        local_57 = _e2070;
                                        let _e2074 = (_e2072 < (_e2062 + 3u));
                                        if _e2074 {
                                            if (_e2072 < _e2062) {
                                                let _e3599 = global_5.member[_e2072];
                                                let _e3604 = (_e3599.member.x - _e1191);
                                                let _e3605 = (_e3599.member.y - _e1192);
                                                let _e3606 = (_e3599.member.z - _e1193);
                                                let _e3611 = (((_e3604 * _e3604) + (_e3605 * _e3605)) + (_e3606 * _e3606));
                                                let _e3612 = sqrt(_e3611);
                                                let _e3613 = (_e3604 / _e3612);
                                                let _e3614 = (_e3605 / _e3612);
                                                let _e3615 = (_e3606 / _e3612);
                                                if (_e3599.member.w < 0.5f) {
                                                    phi_2328_ = 1f;
                                                } else {
                                                    let _e3639 = (((_e3599.member_2.x * -(_e3613)) + (_e3599.member_2.y * -(_e3614))) + (_e3599.member_2.z * -(_e3615)));
                                                    if (_e3599.member_3.x == _e3599.member_2.w) {
                                                        phi_2327_ = select(1f, 0f, (_e3639 < _e3599.member_3.x));
                                                    } else {
                                                        let _e3643 = ((_e3639 - _e3599.member_3.x) / (_e3599.member_2.w - _e3599.member_3.x));
                                                        let _e3645 = select(_e3643, 0f, (_e3643 < 0f));
                                                        let _e3647 = select(_e3645, 1f, (_e3645 > 1f));
                                                        phi_2327_ = ((_e3647 * _e3647) * (3f - (2f * _e3647)));
                                                    }
                                                    let _e3655 = phi_2327_;
                                                    phi_2328_ = _e3655;
                                                }
                                                let _e3657 = phi_2328_;
                                                phi_15541_ = _e2066;
                                                phi_4099_ = vec3<f32>(((_e3599.member_1.x * _e3657) / _e3611), ((_e3599.member_1.y * _e3657) / _e3611), ((_e3599.member_1.z * _e3657) / _e3611));
                                                phi_4100_ = 0f;
                                                phi_4101_ = vec3<f32>(_e3613, _e3614, _e3615);
                                                phi_4102_ = _e3612;
                                            } else {
                                                if (_e2072 == _e2062) {
                                                    if ((_e211 & 4u) == 0u) {
                                                        phi_15580_ = _e2066;
                                                        phi_2888_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_2889_ = 0f;
                                                        phi_2890_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2927 = global_1.member.member_5;
                                                        let _e2934 = global_1.member.member_5[3u];
                                                        let _e2935 = (1f - _e2934);
                                                        let _e2936 = (6.2831855f * _e2935);
                                                        let _e2941 = (((_e2927.x * _e1166.x) + (_e2927.y * _e1166.y)) + (_e2927.z * _e1166.z));
                                                        if (_e2941 != _e2941) {
                                                            phi_9501_ = true;
                                                        } else {
                                                            phi_9501_ = (0f >= _e2941);
                                                        }
                                                        let _e2945 = phi_9501_;
                                                        let _e2947 = (select(_e2941, 0f, _e2945) * 0.31830987f);
                                                        if (_e2041 == 0u) {
                                                            if (_e2941 <= 0f) {
                                                                phi_10298_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_10298_ = (_e2035 * (_e2941 * 0.31830987f));
                                                            }
                                                            let _e3426 = phi_10298_;
                                                            phi_2736_ = type_59(_e3426, _e2947);
                                                            phi_2737_ = type_59();
                                                            phi_2738_ = true;
                                                        } else {
                                                            let _e2956 = (((_e1140.x * _e1212) + (_e1140.y * _e1213)) + (_e1140.z * _e1215));
                                                            let _e2961 = (((_e1140.x * _e1208) + (_e1140.y * _e1218)) + (_e1140.z * _e1219));
                                                            let _e2966 = (((_e1140.x * _e1166.x) + (_e1140.y * _e1166.y)) + (_e1140.z * _e1166.z));
                                                            let _e2971 = (((_e2927.x * _e1212) + (_e2927.y * _e1213)) + (_e2927.z * _e1215));
                                                            let _e2976 = (((_e2927.x * _e1208) + (_e2927.y * _e1218)) + (_e2927.z * _e1219));
                                                            if (_e2966 <= 0f) {
                                                                phi_2700_ = type_59();
                                                                phi_2701_ = true;
                                                            } else {
                                                                let _e2978 = (_e2941 <= 0f);
                                                                if _e2978 {
                                                                    phi_2699_ = type_59();
                                                                } else {
                                                                    let _e2979 = (_e2956 + _e2971);
                                                                    let _e2980 = (_e2961 + _e2976);
                                                                    let _e2981 = (_e2966 + _e2941);
                                                                    let _e2988 = (1f / sqrt((((_e2979 * _e2979) + (_e2980 * _e2980)) + (_e2981 * _e2981))));
                                                                    let _e2991 = (_e2981 * _e2988);
                                                                    if (_e2991 <= 0f) {
                                                                        phi_9611_ = 0f;
                                                                    } else {
                                                                        let _e2993 = (_e2039 * _e2039);
                                                                        let _e2997 = ((((_e2993 - 1f) * _e2991) * _e2991) + 1f);
                                                                        phi_9611_ = (_e2993 / ((3.1415927f * _e2997) * _e2997));
                                                                    }
                                                                    let _e3002 = phi_9611_;
                                                                    let _e3003 = (_e2966 * _e2966);
                                                                    if (_e3003 >= 1f) {
                                                                        phi_9639_ = 0f;
                                                                    } else {
                                                                        phi_9639_ = ((sqrt((1f + (((_e2039 * _e2039) * ((_e2956 * _e2956) + (_e2961 * _e2961))) / _e3003))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3016 = phi_9639_;
                                                                    let _e3017 = (1f + _e3016);
                                                                    let _e3018 = (_e2941 * _e2941);
                                                                    if (_e3018 >= 1f) {
                                                                        phi_9667_ = 0f;
                                                                    } else {
                                                                        phi_9667_ = ((sqrt((1f + (((_e2039 * _e2039) * ((_e2971 * _e2971) + (_e2976 * _e2976))) / _e3018))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3031 = phi_9667_;
                                                                    let _e3038 = (((_e2956 * (_e2979 * _e2988)) + (_e2961 * (_e2980 * _e2988))) + (_e2966 * _e2991));
                                                                    if (_e2037 == 0u) {
                                                                        let _e3333 = (1f - _e3038);
                                                                        if (_e3333 != _e3333) {
                                                                            phi_10170_ = true;
                                                                        } else {
                                                                            phi_10170_ = (0f >= _e3333);
                                                                        }
                                                                        let _e3337 = phi_10170_;
                                                                        let _e3339 = pow(select(_e3333, 0f, _e3337), 5f);
                                                                        let _e3340 = (1f - _e3339);
                                                                        phi_2687_ = vec3<f32>(((_e2035.x * _e3340) + _e3339), ((_e2035.y * _e3340) + _e3339), ((_e2035.z * _e3340) + _e3339));
                                                                    } else {
                                                                        if (_e2037 == 1u) {
                                                                            let _e3306 = (1f / _e2031);
                                                                            let _e3310 = ((_e3306 * _e3306) * (1f - (_e3038 * _e3038)));
                                                                            if (_e3310 >= 1f) {
                                                                                phi_10131_ = 1f;
                                                                            } else {
                                                                                let _e3313 = sqrt((1f - _e3310));
                                                                                if (_e3038 != _e3038) {
                                                                                    phi_10142_ = true;
                                                                                } else {
                                                                                    phi_10142_ = (_e3313 <= _e3038);
                                                                                }
                                                                                let _e3317 = phi_10142_;
                                                                                let _e3322 = pow(((1f - _e2031) / (1f + _e2031)), 2f);
                                                                                phi_10131_ = (_e3322 + ((1f - _e3322) * pow((1f - select(_e3038, _e3313, _e3317)), 5f)));
                                                                            }
                                                                            let _e3329 = phi_10131_;
                                                                            phi_2686_ = vec3(_e3329);
                                                                        } else {
                                                                            let _e3042 = select(_e3038, 0f, (_e3038 < 0f));
                                                                            let _e3044 = select(_e3042, 1f, (_e3042 > 1f));
                                                                            let _e3046 = (1f - (_e3044 * _e3044));
                                                                            let _e3059 = ((_e2035.x * _e2035.x) - (_e2033.x * _e2033.x));
                                                                            let _e3060 = ((_e2035.y * _e2035.y) - (_e2033.y * _e2033.y));
                                                                            let _e3061 = ((_e2035.z * _e2035.z) - (_e2033.z * _e2033.z));
                                                                            let _e3063 = (_e2035.x * (_e2033.x + _e2033.x));
                                                                            let _e3065 = (_e2035.y * (_e2033.y + _e2033.y));
                                                                            let _e3067 = (_e2035.z * (_e2033.z + _e2033.z));
                                                                            let _e3074 = ((_e3059 * _e3059) + (_e3063 * _e3063));
                                                                            let _e3075 = ((_e3060 * _e3060) + (_e3065 * _e3065));
                                                                            let _e3076 = ((_e3061 * _e3061) + (_e3067 * _e3067));
                                                                            let _e3081 = ((_e3046 * _e3059) / _e3074);
                                                                            let _e3082 = ((_e3046 * _e3060) / _e3075);
                                                                            let _e3083 = ((_e3046 * _e3061) / _e3076);
                                                                            let _e3084 = (1f - _e3081);
                                                                            let _e3085 = (1f - _e3082);
                                                                            let _e3086 = (1f - _e3083);
                                                                            let _e3088 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3081, _e3082, _e3083));
                                                                            let _e3089 = (_e3046 * _e3063);
                                                                            let _e3090 = (_e3046 * _e3065);
                                                                            let _e3091 = (_e3046 * _e3067);
                                                                            let _e3096 = (vec3<f32>(_e3089, _e3090, _e3091) / vec3<f32>(_e3074, _e3075, _e3076));
                                                                            let _e3109 = ((_e3088.x * _e3088.x) + (_e3096.x * _e3096.x));
                                                                            let _e3110 = ((_e3088.y * _e3088.y) + (_e3096.y * _e3096.y));
                                                                            let _e3111 = ((_e3088.z * _e3088.z) + (_e3096.z * _e3096.z));
                                                                            if (_e3109 != _e3109) {
                                                                                phi_9801_ = true;
                                                                            } else {
                                                                                phi_9801_ = (0f >= _e3109);
                                                                            }
                                                                            let _e3115 = phi_9801_;
                                                                            let _e3117 = sqrt(select(_e3109, 0f, _e3115));
                                                                            if (_e3110 != _e3110) {
                                                                                phi_9816_ = true;
                                                                            } else {
                                                                                phi_9816_ = (0f >= _e3110);
                                                                            }
                                                                            let _e3121 = phi_9816_;
                                                                            let _e3123 = sqrt(select(_e3110, 0f, _e3121));
                                                                            if (_e3111 != _e3111) {
                                                                                phi_9831_ = true;
                                                                            } else {
                                                                                phi_9831_ = (0f >= _e3111);
                                                                            }
                                                                            let _e3127 = phi_9831_;
                                                                            let _e3129 = sqrt(select(_e3111, 0f, _e3127));
                                                                            let _e3133 = ((_e3117 + _e3084) * 0.5f);
                                                                            let _e3134 = ((_e3123 + _e3085) * 0.5f);
                                                                            let _e3135 = ((_e3129 + _e3086) * 0.5f);
                                                                            if (_e3133 != _e3133) {
                                                                                phi_9858_ = true;
                                                                            } else {
                                                                                phi_9858_ = (0f >= _e3133);
                                                                            }
                                                                            let _e3139 = phi_9858_;
                                                                            let _e3141 = sqrt(select(_e3133, 0f, _e3139));
                                                                            if (_e3134 != _e3134) {
                                                                                phi_9873_ = true;
                                                                            } else {
                                                                                phi_9873_ = (0f >= _e3134);
                                                                            }
                                                                            let _e3145 = phi_9873_;
                                                                            let _e3147 = sqrt(select(_e3134, 0f, _e3145));
                                                                            if (_e3135 != _e3135) {
                                                                                phi_9888_ = true;
                                                                            } else {
                                                                                phi_9888_ = (0f >= _e3135);
                                                                            }
                                                                            let _e3151 = phi_9888_;
                                                                            let _e3153 = sqrt(select(_e3135, 0f, _e3151));
                                                                            let _e3154 = vec3<f32>(_e3141, _e3147, _e3153);
                                                                            let _e3158 = ((_e3117 - _e3084) * 0.5f);
                                                                            let _e3159 = ((_e3123 - _e3085) * 0.5f);
                                                                            let _e3160 = ((_e3129 - _e3086) * 0.5f);
                                                                            if (_e3158 != _e3158) {
                                                                                phi_9915_ = true;
                                                                            } else {
                                                                                phi_9915_ = (0f >= _e3158);
                                                                            }
                                                                            let _e3164 = phi_9915_;
                                                                            let _e3166 = sqrt(select(_e3158, 0f, _e3164));
                                                                            if (_e3159 != _e3159) {
                                                                                phi_9930_ = true;
                                                                            } else {
                                                                                phi_9930_ = (0f >= _e3159);
                                                                            }
                                                                            let _e3170 = phi_9930_;
                                                                            let _e3172 = sqrt(select(_e3159, 0f, _e3170));
                                                                            if (_e3160 != _e3160) {
                                                                                phi_9945_ = true;
                                                                            } else {
                                                                                phi_9945_ = (0f >= _e3160);
                                                                            }
                                                                            let _e3176 = phi_9945_;
                                                                            let _e3178 = sqrt(select(_e3160, 0f, _e3176));
                                                                            if ((_e3089 / _e3074) < 0f) {
                                                                                phi_2563_ = -(_e3166);
                                                                            } else {
                                                                                phi_2563_ = _e3166;
                                                                            }
                                                                            let _e3182 = phi_2563_;
                                                                            if ((_e3090 / _e3075) < 0f) {
                                                                                phi_2571_ = -(_e3172);
                                                                            } else {
                                                                                phi_2571_ = _e3172;
                                                                            }
                                                                            let _e3186 = phi_2571_;
                                                                            if ((_e3091 / _e3076) < 0f) {
                                                                                phi_2579_ = -(_e3178);
                                                                            } else {
                                                                                phi_2579_ = _e3178;
                                                                            }
                                                                            let _e3190 = phi_2579_;
                                                                            let _e3194 = (_e2033.x * _e3044);
                                                                            let _e3195 = (_e2033.y * _e3044);
                                                                            let _e3196 = (_e2033.z * _e3044);
                                                                            let _e3206 = vec3<f32>(((_e2035.x * _e3141) - (_e2033.x * _e3182)), ((_e2035.y * _e3147) - (_e2033.y * _e3186)), ((_e2035.z * _e3153) - (_e2033.z * _e3190)));
                                                                            let _e3213 = ((_e2035.x * _e3182) + (_e2033.x * _e3141));
                                                                            let _e3214 = ((_e2035.y * _e3186) + (_e2033.y * _e3147));
                                                                            let _e3215 = ((_e2035.z * _e3190) + (_e2033.z * _e3153));
                                                                            let _e3217 = vec3<f32>((_e2035.x * _e3044), (_e2035.y * _e3044), (_e2035.z * _e3044));
                                                                            let _e3218 = (_e3217 - _e3154);
                                                                            let _e3219 = (_e3194 - _e3182);
                                                                            let _e3220 = (_e3195 - _e3186);
                                                                            let _e3221 = (_e3196 - _e3190);
                                                                            let _e3234 = (_e3217 + _e3154);
                                                                            let _e3235 = (_e3194 + _e3182);
                                                                            let _e3236 = (_e3195 + _e3186);
                                                                            let _e3237 = (_e3196 + _e3190);
                                                                            let _e3247 = ((_e3234.x * _e3234.x) + (_e3235 * _e3235));
                                                                            let _e3248 = ((_e3234.y * _e3234.y) + (_e3236 * _e3236));
                                                                            let _e3249 = ((_e3234.z * _e3234.z) + (_e3237 * _e3237));
                                                                            let _e3259 = vec3(_e3044);
                                                                            let _e3260 = (_e3259 - _e3206);
                                                                            let _e3261 = -(vec3<f32>(_e3213, _e3214, _e3215));
                                                                            let _e3277 = (_e3259 + _e3206);
                                                                            let _e3287 = ((_e3277.x * _e3277.x) + (_e3213 * _e3213));
                                                                            let _e3288 = ((_e3277.y * _e3277.y) + (_e3214 * _e3214));
                                                                            let _e3289 = ((_e3277.z * _e3277.z) + (_e3215 * _e3215));
                                                                            phi_2686_ = vec3<f32>((((((_e3218.x * _e3218.x) + (_e3219 * _e3219)) / select(0.000000000000000000000000000001f, _e3247, (_e3247 > 0.000000000000000000000000000001f))) + (((_e3260.x * _e3260.x) + (_e3261.x * _e3261.x)) / select(0.000000000000000000000000000001f, _e3287, (_e3287 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3218.y * _e3218.y) + (_e3220 * _e3220)) / select(0.000000000000000000000000000001f, _e3248, (_e3248 > 0.000000000000000000000000000001f))) + (((_e3260.y * _e3260.y) + (_e3261.y * _e3261.y)) / select(0.000000000000000000000000000001f, _e3288, (_e3288 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3218.z * _e3218.z) + (_e3221 * _e3221)) / select(0.000000000000000000000000000001f, _e3249, (_e3249 > 0.000000000000000000000000000001f))) + (((_e3260.z * _e3260.z) + (_e3261.z * _e3261.z)) / select(0.000000000000000000000000000001f, _e3289, (_e3289 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                        }
                                                                        let _e3332 = phi_2686_;
                                                                        phi_2687_ = _e3332;
                                                                    }
                                                                    let _e3352 = phi_2687_;
                                                                    let _e3354 = (4f * _e2966);
                                                                    phi_2699_ = type_59((_e3352 * (((1f / (_e3017 + _e3031)) * _e3002) / _e3354)), ((_e3002 / _e3017) / _e3354));
                                                                }
                                                                let _e3361 = phi_2699_;
                                                                phi_2700_ = _e3361;
                                                                phi_2701_ = _e2978;
                                                            }
                                                            let _e3363 = phi_2700_;
                                                            let _e3365 = phi_2701_;
                                                            if _e3365 {
                                                                phi_2706_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                            } else {
                                                                phi_2706_ = _e3363;
                                                            }
                                                            let _e3367 = phi_2706_;
                                                            let _e3370 = (_e2041 == 1u);
                                                            if _e3370 {
                                                                phi_2734_ = _e3367;
                                                                phi_2735_ = type_59();
                                                            } else {
                                                                if (_e2966 != _e2966) {
                                                                    phi_10207_ = true;
                                                                } else {
                                                                    phi_10207_ = (0f >= _e2966);
                                                                }
                                                                let _e3374 = phi_10207_;
                                                                let _e3375 = select(_e2966, 0f, _e3374);
                                                                let _e3376 = (1f / _e2031);
                                                                let _e3380 = ((_e3376 * _e3376) * (1f - (_e3375 * _e3375)));
                                                                if (_e3380 >= 1f) {
                                                                    phi_10226_ = 1f;
                                                                } else {
                                                                    let _e3383 = sqrt((1f - _e3380));
                                                                    if (_e3375 != _e3375) {
                                                                        phi_10237_ = true;
                                                                    } else {
                                                                        phi_10237_ = (_e3383 <= _e3375);
                                                                    }
                                                                    let _e3387 = phi_10237_;
                                                                    let _e3392 = pow(((1f - _e2031) / (1f + _e2031)), 2f);
                                                                    phi_10226_ = (_e3392 + ((1f - _e3392) * pow((1f - select(_e3375, _e3383, _e3387)), 5f)));
                                                                }
                                                                let _e3399 = phi_10226_;
                                                                if (_e2941 <= 0f) {
                                                                    phi_10276_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    phi_10276_ = (_e2029 * (_e2941 * 0.31830987f));
                                                                }
                                                                let _e3404 = phi_10276_;
                                                                let _e3405 = (1f - _e3399);
                                                                phi_2734_ = type_59();
                                                                phi_2735_ = type_59((_e3367.member + vec3<f32>((_e3404.x * _e3405), (_e3404.y * _e3405), (_e3404.z * _e3405))), ((_e3399 * _e3367.member_1) + (_e3405 * _e2947)));
                                                            }
                                                            let _e3419 = phi_2734_;
                                                            let _e3421 = phi_2735_;
                                                            phi_2736_ = _e3419;
                                                            phi_2737_ = _e3421;
                                                            phi_2738_ = _e3370;
                                                        }
                                                        let _e3429 = phi_2736_;
                                                        let _e3431 = phi_2737_;
                                                        let _e3433 = phi_2738_;
                                                        if _e3433 {
                                                            phi_2742_ = _e3429;
                                                        } else {
                                                            phi_2742_ = _e3431;
                                                        }
                                                        let _e3435 = phi_2742_;
                                                        if ((_e3435.member_1 * _e2936) < 0.001f) {
                                                            let _e3567 = global_1.member.member_6;
                                                            phi_15581_ = _e2066;
                                                            phi_2885_ = vec3<f32>(_e3567.x, _e3567.y, _e3567.z);
                                                            phi_2886_ = 0f;
                                                            phi_2887_ = vec3<f32>(_e2927.x, _e2927.y, _e2927.z);
                                                        } else {
                                                            let _e3440 = ((_e2066 * 747796405u) + 2891336453u);
                                                            let _e3448 = (((_e3440 >> bitcast<u32>((((_e3440 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3440) * 277803737u);
                                                            let _e3451 = ((_e3448 >> bitcast<u32>(22i)) ^ _e3448);
                                                            let _e3457 = (1f - ((f32((_e3451 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2935));
                                                            let _e3459 = (1f - (_e3457 * _e3457));
                                                            if (_e3459 != _e3459) {
                                                                phi_10330_ = true;
                                                            } else {
                                                                phi_10330_ = (0f >= _e3459);
                                                            }
                                                            let _e3463 = phi_10330_;
                                                            let _e3465 = sqrt(select(_e3459, 0f, _e3463));
                                                            let _e3467 = ((_e3451 * 747796405u) + 2891336453u);
                                                            let _e3475 = (((_e3467 >> bitcast<u32>((((_e3467 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3467) * 277803737u);
                                                            let _e3478 = ((_e3475 >> bitcast<u32>(22i)) ^ _e3475);
                                                            let _e3482 = (f32((_e3478 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                            let _e3484 = select(-1f, 1f, (_e2927.z >= 0f));
                                                            let _e3486 = (-1f / (_e3484 + _e2927.z));
                                                            let _e3488 = ((_e2927.x * _e2927.y) * _e3486);
                                                            let _e3501 = (_e3465 * cos(_e3482));
                                                            let _e3506 = (_e3465 * sin(_e3482));
                                                            let _e3516 = ((((1f + (((_e3484 * _e2927.x) * _e2927.x) * _e3486)) * _e3501) + (_e3488 * _e3506)) + (_e2927.x * _e3457));
                                                            let _e3517 = ((((_e3484 * _e3488) * _e3501) + ((_e3484 + ((_e2927.y * _e2927.y) * _e3486)) * _e3506)) + (_e2927.y * _e3457));
                                                            let _e3518 = ((((-(_e3484) * _e2927.x) * _e3501) + (-(_e2927.y) * _e3506)) + (_e2927.z * _e3457));
                                                            let _e3524 = (((_e3516 * _e2927.x) + (_e3517 * _e2927.y)) + (_e3518 * _e2927.z));
                                                            let _e3525 = (_e3524 <= _e2934);
                                                            if _e3525 {
                                                                phi_2827_ = 0f;
                                                            } else {
                                                                phi_2827_ = (0.15915494f / _e2935);
                                                            }
                                                            let _e3528 = phi_2827_;
                                                            if (_e3528 > 0f) {
                                                                if _e3525 {
                                                                    phi_2883_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    if (_e3524 != _e3524) {
                                                                        phi_10401_ = true;
                                                                    } else {
                                                                        phi_10401_ = (1f <= _e3524);
                                                                    }
                                                                    let _e3533 = phi_10401_;
                                                                    let _e3539 = global_1.member.member_6[3u];
                                                                    let _e3542 = (1f - pow((acos(select(_e3524, 1f, _e3533)) / _e3539), 2f));
                                                                    if (_e3542 != _e3542) {
                                                                        phi_10416_ = true;
                                                                    } else {
                                                                        phi_10416_ = (0f >= _e3542);
                                                                    }
                                                                    let _e3546 = phi_10416_;
                                                                    let _e3549 = global_1.member.member_6;
                                                                    let _e3557 = (((1f - (0.6f * (1f - sqrt(select(_e3542, 0f, _e3546))))) * 1.25f) / _e2936);
                                                                    phi_2883_ = vec3<f32>((_e3549.x * _e3557), (_e3549.y * _e3557), (_e3549.z * _e3557));
                                                                }
                                                                let _e3563 = phi_2883_;
                                                                phi_2884_ = _e3563;
                                                            } else {
                                                                phi_2884_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3565 = phi_2884_;
                                                            phi_15581_ = _e3478;
                                                            phi_2885_ = _e3565;
                                                            phi_2886_ = _e3528;
                                                            phi_2887_ = vec3<f32>(_e3516, _e3517, _e3518);
                                                        }
                                                        let _e3573 = phi_15581_;
                                                        let _e3575 = phi_2885_;
                                                        let _e3577 = phi_2886_;
                                                        let _e3579 = phi_2887_;
                                                        phi_15580_ = _e3573;
                                                        phi_2888_ = _e3575;
                                                        phi_2889_ = _e3577;
                                                        phi_2890_ = _e3579;
                                                    }
                                                    let _e3581 = phi_15580_;
                                                    let _e3583 = phi_2888_;
                                                    let _e3585 = phi_2889_;
                                                    let _e3587 = phi_2890_;
                                                    phi_15542_ = _e3581;
                                                    phi_4095_ = _e3583;
                                                    phi_4096_ = _e3585;
                                                    phi_4097_ = _e3587;
                                                    phi_4098_ = 1000000000000000000000000000000f;
                                                } else {
                                                    if (_e2072 == (_e2062 + 1u)) {
                                                        if ((_e211 & 1u) == 0u) {
                                                            phi_15546_ = _e2066;
                                                            phi_3414_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_3415_ = 0f;
                                                            phi_3416_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_3417_ = 1000000000000000000000000000000f;
                                                        } else {
                                                            if (_e214 > 0u) {
                                                                let _e2417 = global_1.member.member_2[1u];
                                                                let _e2419 = ((_e2066 * 747796405u) + 2891336453u);
                                                                let _e2427 = (((_e2419 >> bitcast<u32>((((_e2419 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2419) * 277803737u);
                                                                let _e2430 = ((_e2427 >> bitcast<u32>(22i)) ^ _e2427);
                                                                phi_2911_ = 0u;
                                                                phi_2914_ = _e214;
                                                                loop {
                                                                    let _e2436 = phi_2911_;
                                                                    let _e2438 = phi_2914_;
                                                                    local_17 = _e2436;
                                                                    local_18 = _e2436;
                                                                    let _e2440 = ((_e2436 + 1u) < _e2438);
                                                                    if _e2440 {
                                                                        let _e2442 = ((_e2436 + _e2438) / 2u);
                                                                        let _e2445 = global_6.member[(_e2417 + _e2442)];
                                                                        let _e2446 = (_e2445 <= (f32((_e2430 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                        phi_2912_ = select(_e2436, _e2442, _e2446);
                                                                        phi_2915_ = select(_e2442, _e2438, _e2446);
                                                                    } else {
                                                                        phi_2912_ = u32();
                                                                        phi_2915_ = u32();
                                                                    }
                                                                    let _e2450 = phi_2912_;
                                                                    let _e2452 = phi_2915_;
                                                                    phi_2911_ = _e2450;
                                                                    phi_2914_ = _e2452;
                                                                    if !(_e2440) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e2455 = local_17;
                                                                let _e2456 = (_e2417 + _e2455);
                                                                let _e2459 = global_6.member[(_e2456 + 1u)];
                                                                let _e2461 = global_6.member[_e2456];
                                                                let _e2465 = global_1.member.member_2[0u];
                                                                let _e2467 = local_18;
                                                                let _e2470 = global_6.member[(_e2465 + _e2467)];
                                                                let _e2473 = ((_e2430 * 747796405u) + 2891336453u);
                                                                let _e2481 = (((_e2473 >> bitcast<u32>((((_e2473 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2473) * 277803737u);
                                                                let _e2484 = ((_e2481 >> bitcast<u32>(22i)) ^ _e2481);
                                                                let _e2487 = f32((_e2484 >> bitcast<u32>(8i)));
                                                                let _e2488 = (_e2487 * 0.000000059604645f);
                                                                let _e2490 = ((_e2484 * 747796405u) + 2891336453u);
                                                                let _e2498 = (((_e2490 >> bitcast<u32>((((_e2490 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2490) * 277803737u);
                                                                let _e2501 = ((_e2498 >> bitcast<u32>(22i)) ^ _e2498);
                                                                let _e2504 = f32((_e2501 >> bitcast<u32>(8i)));
                                                                let _e2507 = global_2.member[bitcast<u32>(_e2470)];
                                                                let _e2510 = (_e2507.member_6.x == 1u);
                                                                if _e2510 {
                                                                    let _e2652 = (_e2507.member.x - _e1191);
                                                                    let _e2653 = (_e2507.member.y - _e1192);
                                                                    let _e2654 = (_e2507.member.z - _e1193);
                                                                    let _e2659 = (((_e2652 * _e2652) + (_e2653 * _e2653)) + (_e2654 * _e2654));
                                                                    let _e2660 = (_e2507.member.w * _e2507.member.w);
                                                                    let _e2662 = (_e2659 <= (_e2660 * 1.000001f));
                                                                    if _e2662 {
                                                                        let _e2771 = (_e2487 * 0.00000037450704f);
                                                                        let _e2773 = ((_e2504 * 0.00000011920929f) - 1f);
                                                                        let _e2775 = (1f - (_e2773 * _e2773));
                                                                        if (_e2775 != _e2775) {
                                                                            phi_9398_ = true;
                                                                        } else {
                                                                            phi_9398_ = (0f >= _e2775);
                                                                        }
                                                                        let _e2779 = phi_9398_;
                                                                        let _e2781 = sqrt(select(_e2775, 0f, _e2779));
                                                                        let _e2783 = (_e2781 * cos(_e2771));
                                                                        let _e2785 = (_e2781 * sin(_e2771));
                                                                        let _e2790 = (_e2507.member.x + (_e2783 * _e2507.member.w));
                                                                        let _e2791 = (_e2507.member.y + (_e2785 * _e2507.member.w));
                                                                        let _e2792 = (_e2507.member.z + (_e2773 * _e2507.member.w));
                                                                        if _e2662 {
                                                                            let _e2805 = (_e2790 - _e1191);
                                                                            let _e2806 = (_e2791 - _e1192);
                                                                            let _e2807 = (_e2792 - _e1193);
                                                                            let _e2812 = (((_e2805 * _e2805) + (_e2806 * _e2806)) + (_e2807 * _e2807));
                                                                            phi_9467_ = (_e2812 / (((abs((((_e2783 * _e2805) + (_e2785 * _e2806)) + (_e2773 * _e2807))) / sqrt(_e2812)) * 12.566371f) * _e2660));
                                                                        } else {
                                                                            let _e2794 = (_e2660 / _e2659);
                                                                            if (_e2794 != _e2794) {
                                                                                phi_9486_ = true;
                                                                            } else {
                                                                                phi_9486_ = (1f <= _e2794);
                                                                            }
                                                                            let _e2798 = phi_9486_;
                                                                            let _e2799 = select(_e2794, 1f, _e2798);
                                                                            phi_9467_ = (0.15915494f / (_e2799 / (1f + sqrt((1f - _e2799)))));
                                                                        }
                                                                        let _e2825 = phi_9467_;
                                                                        phi_3127_ = type_60(vec3<f32>(_e2790, _e2791, _e2792), vec3<f32>(_e2783, _e2785, _e2773), _e2825);
                                                                    } else {
                                                                        let _e2663 = (_e2660 / _e2659);
                                                                        let _e2664 = (_e2663 != _e2663);
                                                                        if _e2664 {
                                                                            phi_9250_ = true;
                                                                        } else {
                                                                            phi_9250_ = (1f <= _e2663);
                                                                        }
                                                                        let _e2667 = phi_9250_;
                                                                        let _e2668 = select(_e2663, 1f, _e2667);
                                                                        if _e2664 {
                                                                            phi_9273_ = true;
                                                                        } else {
                                                                            phi_9273_ = (1f <= _e2663);
                                                                        }
                                                                        let _e2672 = phi_9273_;
                                                                        let _e2673 = select(_e2663, 1f, _e2672);
                                                                        let _e2677 = (_e2673 / (1f + sqrt((1f - _e2673))));
                                                                        if (_e2668 < 0.00068523f) {
                                                                            phi_3062_ = (_e2668 * _e2488);
                                                                        } else {
                                                                            let _e2680 = (1f - (_e2488 * _e2677));
                                                                            let _e2682 = (1f - (_e2680 * _e2680));
                                                                            if (_e2682 != _e2682) {
                                                                                phi_9288_ = true;
                                                                            } else {
                                                                                phi_9288_ = (0f >= _e2682);
                                                                            }
                                                                            let _e2686 = phi_9288_;
                                                                            phi_3062_ = select(_e2682, 0f, _e2686);
                                                                        }
                                                                        let _e2690 = phi_3062_;
                                                                        let _e2691 = (1f - _e2690);
                                                                        if (_e2691 != _e2691) {
                                                                            phi_9303_ = true;
                                                                        } else {
                                                                            phi_9303_ = (0f >= _e2691);
                                                                        }
                                                                        let _e2695 = phi_9303_;
                                                                        let _e2700 = (1f - (_e2690 / _e2668));
                                                                        if (_e2700 != _e2700) {
                                                                            phi_9318_ = true;
                                                                        } else {
                                                                            phi_9318_ = (0f >= _e2700);
                                                                        }
                                                                        let _e2704 = phi_9318_;
                                                                        let _e2708 = ((_e2690 / sqrt(_e2668)) + (sqrt(select(_e2691, 0f, _e2695)) * sqrt(select(_e2700, 0f, _e2704))));
                                                                        let _e2710 = (1f - (_e2708 * _e2708));
                                                                        if (_e2710 != _e2710) {
                                                                            phi_9333_ = true;
                                                                        } else {
                                                                            phi_9333_ = (0f >= _e2710);
                                                                        }
                                                                        let _e2714 = phi_9333_;
                                                                        let _e2716 = sqrt(select(_e2710, 0f, _e2714));
                                                                        let _e2717 = (_e2504 * 0.00000037450704f);
                                                                        let _e2718 = sqrt(_e2659);
                                                                        let _e2719 = (_e2652 / _e2718);
                                                                        let _e2720 = (_e2653 / _e2718);
                                                                        let _e2721 = (_e2654 / _e2718);
                                                                        let _e2723 = select(-1f, 1f, (_e2721 >= 0f));
                                                                        let _e2725 = (-1f / (_e2723 + _e2721));
                                                                        let _e2727 = ((_e2719 * _e2720) * _e2725);
                                                                        let _e2740 = (_e2716 * cos(_e2717));
                                                                        let _e2745 = (_e2716 * sin(_e2717));
                                                                        let _e2758 = -(((((1f + (((_e2723 * _e2719) * _e2719) * _e2725)) * _e2740) + (_e2727 * _e2745)) + (_e2719 * _e2708)));
                                                                        let _e2759 = -(((((_e2723 * _e2727) * _e2740) + ((_e2723 + ((_e2720 * _e2720) * _e2725)) * _e2745)) + (_e2720 * _e2708)));
                                                                        let _e2760 = -(((((-(_e2723) * _e2719) * _e2740) + (-(_e2720) * _e2745)) + (_e2721 * _e2708)));
                                                                        phi_3127_ = type_60(vec3<f32>((_e2507.member.x + (_e2758 * _e2507.member.w)), (_e2507.member.y + (_e2759 * _e2507.member.w)), (_e2507.member.z + (_e2760 * _e2507.member.w))), vec3<f32>(_e2758, _e2759, _e2760), (0.15915494f / _e2677));
                                                                    }
                                                                    let _e2828 = phi_3127_;
                                                                    phi_3313_ = _e2828;
                                                                } else {
                                                                    let _e2511 = sqrt(_e2488);
                                                                    let _e2512 = (1f - _e2511);
                                                                    let _e2513 = ((_e2504 * 0.000000059604645f) * _e2511);
                                                                    let _e2535 = (_e2511 - _e2513);
                                                                    let _e2539 = (((_e2507.member.x * _e2512) + (_e2507.member_1.x * _e2513)) + (_e2507.member_2.x * _e2535));
                                                                    let _e2540 = (((_e2507.member.y * _e2512) + (_e2507.member_1.y * _e2513)) + (_e2507.member_2.y * _e2535));
                                                                    let _e2541 = (((_e2507.member.z * _e2512) + (_e2507.member_1.z * _e2513)) + (_e2507.member_2.z * _e2535));
                                                                    let _e2543 = (_e2507.member_1.x - _e2507.member.x);
                                                                    let _e2544 = (_e2507.member_1.y - _e2507.member.y);
                                                                    let _e2545 = (_e2507.member_1.z - _e2507.member.z);
                                                                    let _e2546 = (_e2507.member_2.x - _e2507.member.x);
                                                                    let _e2547 = (_e2507.member_2.y - _e2507.member.y);
                                                                    let _e2548 = (_e2507.member_2.z - _e2507.member.z);
                                                                    let _e2551 = ((_e2544 * _e2548) - (_e2547 * _e2545));
                                                                    let _e2554 = ((_e2545 * _e2546) - (_e2548 * _e2543));
                                                                    let _e2557 = ((_e2543 * _e2547) - (_e2546 * _e2544));
                                                                    let _e2563 = sqrt((((_e2551 * _e2551) + (_e2554 * _e2554)) + (_e2557 * _e2557)));
                                                                    let _e2564 = (1f / _e2563);
                                                                    let _e2565 = (_e2551 * _e2564);
                                                                    let _e2566 = (_e2554 * _e2564);
                                                                    let _e2567 = (_e2557 * _e2564);
                                                                    if _e2510 {
                                                                        let _e2600 = (_e2507.member.x - _e1191);
                                                                        let _e2601 = (_e2507.member.y - _e1192);
                                                                        let _e2602 = (_e2507.member.z - _e1193);
                                                                        let _e2607 = (((_e2600 * _e2600) + (_e2601 * _e2601)) + (_e2602 * _e2602));
                                                                        let _e2608 = (_e2507.member.w * _e2507.member.w);
                                                                        if (_e2607 <= (_e2608 * 1.000001f)) {
                                                                            let _e2622 = (_e2539 - _e1191);
                                                                            let _e2623 = (_e2540 - _e1192);
                                                                            let _e2624 = (_e2541 - _e1193);
                                                                            let _e2629 = (((_e2622 * _e2622) + (_e2623 * _e2623)) + (_e2624 * _e2624));
                                                                            phi_9216_ = (_e2629 / (((abs((((_e2565 * _e2622) + (_e2566 * _e2623)) + (_e2567 * _e2624))) / sqrt(_e2629)) * 12.566371f) * _e2608));
                                                                        } else {
                                                                            let _e2611 = (_e2608 / _e2607);
                                                                            if (_e2611 != _e2611) {
                                                                                phi_9235_ = true;
                                                                            } else {
                                                                                phi_9235_ = (1f <= _e2611);
                                                                            }
                                                                            let _e2615 = phi_9235_;
                                                                            let _e2616 = select(_e2611, 1f, _e2615);
                                                                            phi_9216_ = (0.15915494f / (_e2616 / (1f + sqrt((1f - _e2616)))));
                                                                        }
                                                                        let _e2642 = phi_9216_;
                                                                        phi_3311_ = _e2642;
                                                                    } else {
                                                                        let _e2569 = (_e2539 - _e1191);
                                                                        let _e2570 = (_e2540 - _e1192);
                                                                        let _e2571 = (_e2541 - _e1193);
                                                                        let _e2576 = (((_e2569 * _e2569) + (_e2570 * _e2570)) + (_e2571 * _e2571));
                                                                        let _e2584 = (abs((((_e2565 * _e2569) + (_e2566 * _e2570)) + (_e2567 * _e2571))) / sqrt(_e2576));
                                                                        if (_e2584 <= 0f) {
                                                                            phi_3310_ = 0f;
                                                                        } else {
                                                                            if (_e2507.member_6.x == 0u) {
                                                                                phi_3307_ = (_e2563 * 0.5f);
                                                                            } else {
                                                                                phi_3307_ = ((12.566371f * _e2507.member.w) * _e2507.member.w);
                                                                            }
                                                                            let _e2593 = phi_3307_;
                                                                            phi_3310_ = (_e2576 / (_e2584 * _e2593));
                                                                        }
                                                                        let _e2597 = phi_3310_;
                                                                        phi_3311_ = _e2597;
                                                                    }
                                                                    let _e2644 = phi_3311_;
                                                                    phi_3313_ = type_60(vec3<f32>(_e2539, _e2540, _e2541), vec3<f32>(_e2565, _e2566, _e2567), _e2644);
                                                                }
                                                                let _e2830 = phi_3313_;
                                                                let _e2835 = (_e2830.member.x - _e1191);
                                                                let _e2837 = (_e2830.member.y - _e1192);
                                                                let _e2839 = (_e2830.member.z - _e1193);
                                                                let _e2845 = sqrt((((_e2835 * _e2835) + (_e2837 * _e2837)) + (_e2839 * _e2839)));
                                                                let _e2846 = (_e2835 / _e2845);
                                                                let _e2847 = (_e2837 / _e2845);
                                                                let _e2848 = (_e2839 / _e2845);
                                                                if (_e2830.member_2 > 0f) {
                                                                    let _e2860 = global_4.member[_e2507.member_6.y];
                                                                    if (_e2860.member == 4u) {
                                                                        if (_e2860.member_1 == 1u) {
                                                                            phi_3391_ = true;
                                                                        } else {
                                                                            phi_3391_ = ((((-(_e2846) * _e2830.member_1.x) + (-(_e2847) * _e2830.member_1.y)) + (-(_e2848) * _e2830.member_1.z)) > 0f);
                                                                        }
                                                                        let _e2875 = phi_3391_;
                                                                        if _e2875 {
                                                                            phi_3400_ = vec3<f32>(_e2860.member_4.x, _e2860.member_4.y, _e2860.member_4.z);
                                                                        } else {
                                                                            phi_3400_ = vec3<f32>();
                                                                        }
                                                                        let _e2882 = phi_3400_;
                                                                        phi_3402_ = _e2882;
                                                                        phi_3403_ = select(true, false, _e2875);
                                                                    } else {
                                                                        phi_3402_ = vec3<f32>();
                                                                        phi_3403_ = true;
                                                                    }
                                                                    let _e2885 = phi_3402_;
                                                                    let _e2887 = phi_3403_;
                                                                    phi_3408_ = select(_e2885, vec3<f32>(0f, 0f, 0f), vec3(_e2887));
                                                                    phi_3409_ = ((_e2459 - _e2461) * _e2830.member_2);
                                                                } else {
                                                                    phi_3408_ = vec3<f32>(0f, 0f, 0f);
                                                                    phi_3409_ = 0f;
                                                                }
                                                                let _e2891 = phi_3408_;
                                                                let _e2893 = phi_3409_;
                                                                phi_15547_ = _e2501;
                                                                phi_3410_ = _e2891;
                                                                phi_3411_ = _e2893;
                                                                phi_3412_ = vec3<f32>(_e2846, _e2847, _e2848);
                                                                phi_3413_ = ((_e2845 * 0.9999f) - _e1184);
                                                            } else {
                                                                phi_15547_ = _e2066;
                                                                phi_3410_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3411_ = 0f;
                                                                phi_3412_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3413_ = 1000000000000000000000000000000f;
                                                            }
                                                            let _e2895 = phi_15547_;
                                                            let _e2897 = phi_3410_;
                                                            let _e2899 = phi_3411_;
                                                            let _e2901 = phi_3412_;
                                                            let _e2903 = phi_3413_;
                                                            phi_15546_ = _e2895;
                                                            phi_3414_ = _e2897;
                                                            phi_3415_ = _e2899;
                                                            phi_3416_ = _e2901;
                                                            phi_3417_ = _e2903;
                                                        }
                                                        let _e2905 = phi_15546_;
                                                        let _e2907 = phi_3414_;
                                                        let _e2909 = phi_3415_;
                                                        let _e2911 = phi_3416_;
                                                        let _e2913 = phi_3417_;
                                                        phi_15543_ = _e2905;
                                                        phi_4091_ = _e2907;
                                                        phi_4092_ = _e2909;
                                                        phi_4093_ = _e2911;
                                                        phi_4094_ = _e2913;
                                                    } else {
                                                        if ((_e211 & 2u) == 0u) {
                                                            phi_15544_ = _e2066;
                                                            phi_4088_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_4089_ = 0f;
                                                            phi_4090_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e2082 = ((_e2066 * 747796405u) + 2891336453u);
                                                            let _e2090 = (((_e2082 >> bitcast<u32>((((_e2082 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2082) * 277803737u);
                                                            let _e2093 = ((_e2090 >> bitcast<u32>(22i)) ^ _e2090);
                                                            let _e2097 = (f32((_e2093 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2099 = ((_e2093 * 747796405u) + 2891336453u);
                                                            let _e2107 = (((_e2099 >> bitcast<u32>((((_e2099 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2099) * 277803737u);
                                                            let _e2110 = ((_e2107 >> bitcast<u32>(22i)) ^ _e2107);
                                                            let _e2114 = (f32((_e2110 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2117 = global_1.member.member_3[1u];
                                                            let _e2120 = global_1.member.member_3[2u];
                                                            let _e2123 = global_1.member.member_2[3u];
                                                            phi_3438_ = 0u;
                                                            phi_3441_ = _e2120;
                                                            loop {
                                                                let _e2125 = phi_3438_;
                                                                let _e2127 = phi_3441_;
                                                                local_12 = _e2125;
                                                                local_15 = _e2125;
                                                                local_16 = _e2125;
                                                                let _e2129 = ((_e2125 + 1u) < _e2127);
                                                                if _e2129 {
                                                                    let _e2131 = ((_e2125 + _e2127) / 2u);
                                                                    let _e2134 = global_6.member[(_e2123 + _e2131)];
                                                                    let _e2135 = (_e2134 <= _e2097);
                                                                    phi_3439_ = select(_e2125, _e2131, _e2135);
                                                                    phi_3442_ = select(_e2131, _e2127, _e2135);
                                                                } else {
                                                                    phi_3439_ = u32();
                                                                    phi_3442_ = u32();
                                                                }
                                                                let _e2139 = phi_3439_;
                                                                let _e2141 = phi_3442_;
                                                                phi_3438_ = _e2139;
                                                                phi_3441_ = _e2141;
                                                                if !(_e2129) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2145 = global_1.member.member_3[0u];
                                                            let _e2146 = (_e2117 + 1u);
                                                            let _e2148 = local_12;
                                                            let _e2150 = (_e2145 + (_e2148 * _e2146));
                                                            phi_3479_ = 0u;
                                                            phi_3482_ = _e2117;
                                                            loop {
                                                                let _e2152 = phi_3479_;
                                                                let _e2154 = phi_3482_;
                                                                local_13 = _e2152;
                                                                local_14 = _e2152;
                                                                let _e2156 = ((_e2152 + 1u) < _e2154);
                                                                if _e2156 {
                                                                    let _e2158 = ((_e2152 + _e2154) / 2u);
                                                                    let _e2161 = global_6.member[(_e2150 + _e2158)];
                                                                    let _e2162 = (_e2161 <= _e2114);
                                                                    phi_3480_ = select(_e2152, _e2158, _e2162);
                                                                    phi_3483_ = select(_e2158, _e2154, _e2162);
                                                                } else {
                                                                    phi_3480_ = u32();
                                                                    phi_3483_ = u32();
                                                                }
                                                                let _e2166 = phi_3480_;
                                                                let _e2168 = phi_3483_;
                                                                phi_3479_ = _e2166;
                                                                phi_3482_ = _e2168;
                                                                if !(_e2156) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2171 = local_13;
                                                            let _e2174 = local_14;
                                                            let _e2175 = (_e2150 + _e2174);
                                                            let _e2177 = global_6.member[_e2175];
                                                            let _e2180 = global_6.member[(_e2175 + 1u)];
                                                            if (_e2180 > _e2177) {
                                                                let _e2184 = ((_e2114 - _e2177) / (_e2180 - _e2177));
                                                                let _e2186 = select(_e2184, 0f, (_e2184 < 0f));
                                                                phi_3559_ = select(_e2186, 0.99999f, (_e2186 > 0.99999f));
                                                            } else {
                                                                phi_3559_ = 0.5f;
                                                            }
                                                            let _e2190 = phi_3559_;
                                                            let _e2192 = f32(_e2117);
                                                            let _e2195 = local_15;
                                                            let _e2198 = local_16;
                                                            let _e2199 = (_e2123 + _e2198);
                                                            let _e2201 = global_6.member[_e2199];
                                                            let _e2204 = global_6.member[(_e2199 + 1u)];
                                                            if (_e2204 > _e2201) {
                                                                let _e2208 = ((_e2097 - _e2201) / (_e2204 - _e2201));
                                                                let _e2210 = select(_e2208, 0f, (_e2208 < 0f));
                                                                phi_3608_ = select(_e2210, 0.99999f, (_e2210 > 0.99999f));
                                                            } else {
                                                                phi_3608_ = 0.5f;
                                                            }
                                                            let _e2214 = phi_3608_;
                                                            let _e2216 = f32(_e2120);
                                                            let _e2219 = ((((f32(_e2171) + _e2190) / _e2192) - 0.5f) * 6.2831855f);
                                                            let _e2220 = (((f32(_e2195) + _e2214) / _e2216) * 3.1415927f);
                                                            let _e2221 = sin(_e2220);
                                                            let _e2223 = (_e2221 * cos(_e2219));
                                                            let _e2224 = cos(_e2220);
                                                            let _e2226 = (_e2221 * sin(_e2219));
                                                            let _e2229 = ((atan2(_e2226, _e2223) * 0.15915494f) + 0.5f);
                                                            let _e2233 = select(_e2224, -1f, (_e2224 < -1f));
                                                            let _e2236 = acos(select(_e2233, 1f, (_e2233 > 1f)));
                                                            let _e2238 = ((_e2229 - floor(_e2229)) * _e2192);
                                                            let _e2243 = select(select(u32(_e2238), 0u, (_e2238 < 0f)), 4294967295u, (_e2238 > 4294967000f));
                                                            let _e2244 = (_e2117 - 1u);
                                                            let _e2247 = ((_e2236 * 0.31830987f) * _e2216);
                                                            let _e2252 = select(select(u32(_e2247), 0u, (_e2247 < 0f)), 4294967295u, (_e2247 > 4294967000f));
                                                            let _e2253 = (_e2120 - 1u);
                                                            let _e2255 = select(_e2252, _e2253, (_e2253 < _e2252));
                                                            let _e2256 = sin(_e2236);
                                                            if (_e2256 <= 0f) {
                                                                phi_3764_ = 0f;
                                                            } else {
                                                                let _e2260 = (_e2123 + _e2255);
                                                                let _e2263 = global_6.member[(_e2260 + 1u)];
                                                                let _e2265 = global_6.member[_e2260];
                                                                let _e2268 = ((_e2145 + (_e2255 * _e2146)) + select(_e2243, _e2244, (_e2244 < _e2243)));
                                                                let _e2271 = global_6.member[(_e2268 + 1u)];
                                                                let _e2273 = global_6.member[_e2268];
                                                                phi_3764_ = ((((_e2263 - _e2265) * _e2216) * ((_e2271 - _e2273) * _e2192)) / (19.73921f * _e2256));
                                                            }
                                                            let _e2280 = phi_3764_;
                                                            let _e2281 = (_e2280 > 0f);
                                                            if _e2281 {
                                                                let _e2283 = (_e2238 - 0.5f);
                                                                let _e2284 = (_e2247 - 0.5f);
                                                                let _e2285 = f32(_e2253);
                                                                if (0f <= _e2285) {
                                                                } else {
                                                                    phi_15513_ = _e2110;
                                                                    phi_15500_ = true;
                                                                    break;
                                                                }
                                                                let _e2288 = select(_e2284, 0f, (_e2284 < 0f));
                                                                let _e2290 = select(_e2288, _e2285, (_e2288 > _e2285));
                                                                let _e2291 = floor(_e2283);
                                                                let _e2292 = floor(_e2290);
                                                                let _e2293 = (_e2283 - _e2291);
                                                                let _e2294 = (_e2290 - _e2292);
                                                                let _e2305 = (_e2117 == 0u);
                                                                if _e2305 {
                                                                    phi_15513_ = _e2110;
                                                                    phi_15500_ = true;
                                                                    break;
                                                                }
                                                                let _e2306 = (bitcast<u32>((select(0i, select(select(i32(_e2291), i32(-2147483648), (_e2291 < -2147483600f)), 2147483647i, (_e2291 > 2147483500f)), (_e2291 == _e2291)) + bitcast<i32>(_e2117))) % _e2117);
                                                                let _e2311 = select(select(u32(_e2292), 0u, (_e2292 < 0f)), 4294967295u, (_e2292 > 4294967000f));
                                                                let _e2312 = (_e2311 + 1u);
                                                                let _e2317 = global_1.member.member_2[2u];
                                                                let _e2318 = (_e2311 * _e2117);
                                                                if _e2305 {
                                                                    phi_15513_ = _e2110;
                                                                    phi_15500_ = true;
                                                                    break;
                                                                }
                                                                let _e2319 = (_e2306 % _e2117);
                                                                let _e2322 = (_e2317 + (3u * (_e2318 + _e2319)));
                                                                let _e2324 = global_6.member[_e2322];
                                                                let _e2327 = global_6.member[(_e2322 + 1u)];
                                                                let _e2330 = global_6.member[(_e2322 + 2u)];
                                                                if _e2305 {
                                                                    phi_15513_ = _e2110;
                                                                    phi_15500_ = true;
                                                                    break;
                                                                }
                                                                let _e2333 = ((_e2306 + 1u) % _e2117);
                                                                let _e2336 = (_e2317 + (3u * (_e2318 + _e2333)));
                                                                let _e2338 = global_6.member[_e2336];
                                                                let _e2341 = global_6.member[(_e2336 + 1u)];
                                                                let _e2344 = global_6.member[(_e2336 + 2u)];
                                                                let _e2352 = (vec3<f32>(_e2324, _e2327, _e2330) + vec3<f32>(((_e2338 - _e2324) * _e2293), ((_e2341 - _e2327) * _e2293), ((_e2344 - _e2330) * _e2293)));
                                                                let _e2353 = (select(_e2312, _e2253, (_e2253 < _e2312)) * _e2117);
                                                                if _e2305 {
                                                                    phi_15513_ = _e2110;
                                                                    phi_15500_ = true;
                                                                    break;
                                                                }
                                                                let _e2356 = (_e2317 + (3u * (_e2353 + _e2319)));
                                                                let _e2358 = global_6.member[_e2356];
                                                                let _e2361 = global_6.member[(_e2356 + 1u)];
                                                                let _e2364 = global_6.member[(_e2356 + 2u)];
                                                                if _e2305 {
                                                                    phi_15513_ = _e2110;
                                                                    phi_15500_ = true;
                                                                    break;
                                                                }
                                                                let _e2368 = (_e2317 + (3u * (_e2353 + _e2333)));
                                                                let _e2370 = global_6.member[_e2368];
                                                                let _e2373 = global_6.member[(_e2368 + 1u)];
                                                                let _e2376 = global_6.member[(_e2368 + 2u)];
                                                                let _e2384 = (vec3<f32>(_e2358, _e2361, _e2364) + vec3<f32>(((_e2370 - _e2358) * _e2293), ((_e2373 - _e2361) * _e2293), ((_e2376 - _e2364) * _e2293)));
                                                                phi_4085_ = (_e2352 + vec3<f32>(((_e2384.x - _e2352.x) * _e2294), ((_e2384.y - _e2352.y) * _e2294), ((_e2384.z - _e2352.z) * _e2294)));
                                                                phi_4087_ = vec3<f32>(_e2223, _e2224, _e2226);
                                                            } else {
                                                                phi_4085_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_4087_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2400 = phi_4085_;
                                                            let _e2402 = phi_4087_;
                                                            phi_15544_ = _e2110;
                                                            phi_4088_ = _e2400;
                                                            phi_4089_ = select(0f, _e2280, _e2281);
                                                            phi_4090_ = _e2402;
                                                        }
                                                        let _e2405 = phi_15544_;
                                                        let _e2407 = phi_4088_;
                                                        let _e2409 = phi_4089_;
                                                        let _e2411 = phi_4090_;
                                                        phi_15543_ = _e2405;
                                                        phi_4091_ = _e2407;
                                                        phi_4092_ = _e2409;
                                                        phi_4093_ = _e2411;
                                                        phi_4094_ = 1000000000000000000000000000000f;
                                                    }
                                                    let _e2915 = phi_15543_;
                                                    let _e2917 = phi_4091_;
                                                    let _e2919 = phi_4092_;
                                                    let _e2921 = phi_4093_;
                                                    let _e2923 = phi_4094_;
                                                    phi_15542_ = _e2915;
                                                    phi_4095_ = _e2917;
                                                    phi_4096_ = _e2919;
                                                    phi_4097_ = _e2921;
                                                    phi_4098_ = _e2923;
                                                }
                                                let _e3589 = phi_15542_;
                                                let _e3591 = phi_4095_;
                                                let _e3593 = phi_4096_;
                                                let _e3595 = phi_4097_;
                                                let _e3597 = phi_4098_;
                                                phi_15541_ = _e3589;
                                                phi_4099_ = _e3591;
                                                phi_4100_ = _e3593;
                                                phi_4101_ = _e3595;
                                                phi_4102_ = _e3597;
                                            }
                                            let _e3666 = phi_15541_;
                                            let _e3668 = phi_4099_;
                                            let _e3670 = phi_4100_;
                                            let _e3672 = phi_4101_;
                                            let _e3674 = phi_4102_;
                                            if ((((_e3668.x * _e3668.x) + (_e3668.y * _e3668.y)) + (_e3668.z * _e3668.z)) > 0f) {
                                                let _e3691 = (((_e3672.x * _e1166.x) + (_e3672.y * _e1166.y)) + (_e3672.z * _e1166.z));
                                                if (_e3691 != _e3691) {
                                                    phi_10431_ = true;
                                                } else {
                                                    phi_10431_ = (0f >= _e3691);
                                                }
                                                let _e3695 = phi_10431_;
                                                let _e3697 = (select(_e3691, 0f, _e3695) * 0.31830987f);
                                                if (_e2041 == 0u) {
                                                    if (_e3691 <= 0f) {
                                                        phi_11228_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_11228_ = (_e2035 * (_e3691 * 0.31830987f));
                                                    }
                                                    let _e4176 = phi_11228_;
                                                    phi_4498_ = type_59(_e4176, _e3697);
                                                    phi_4499_ = type_59();
                                                    phi_4500_ = true;
                                                } else {
                                                    let _e3706 = (((_e1140.x * _e1212) + (_e1140.y * _e1213)) + (_e1140.z * _e1215));
                                                    let _e3711 = (((_e1140.x * _e1208) + (_e1140.y * _e1218)) + (_e1140.z * _e1219));
                                                    let _e3716 = (((_e1140.x * _e1166.x) + (_e1140.y * _e1166.y)) + (_e1140.z * _e1166.z));
                                                    let _e3721 = (((_e3672.x * _e1212) + (_e3672.y * _e1213)) + (_e3672.z * _e1215));
                                                    let _e3726 = (((_e3672.x * _e1208) + (_e3672.y * _e1218)) + (_e3672.z * _e1219));
                                                    if (_e3716 <= 0f) {
                                                        phi_4462_ = type_59();
                                                        phi_4463_ = true;
                                                    } else {
                                                        let _e3728 = (_e3691 <= 0f);
                                                        if _e3728 {
                                                            phi_4461_ = type_59();
                                                        } else {
                                                            let _e3729 = (_e3706 + _e3721);
                                                            let _e3730 = (_e3711 + _e3726);
                                                            let _e3731 = (_e3716 + _e3691);
                                                            let _e3738 = (1f / sqrt((((_e3729 * _e3729) + (_e3730 * _e3730)) + (_e3731 * _e3731))));
                                                            let _e3741 = (_e3731 * _e3738);
                                                            if (_e3741 <= 0f) {
                                                                phi_10541_ = 0f;
                                                            } else {
                                                                let _e3743 = (_e2039 * _e2039);
                                                                let _e3747 = ((((_e3743 - 1f) * _e3741) * _e3741) + 1f);
                                                                phi_10541_ = (_e3743 / ((3.1415927f * _e3747) * _e3747));
                                                            }
                                                            let _e3752 = phi_10541_;
                                                            let _e3753 = (_e3716 * _e3716);
                                                            if (_e3753 >= 1f) {
                                                                phi_10569_ = 0f;
                                                            } else {
                                                                phi_10569_ = ((sqrt((1f + (((_e2039 * _e2039) * ((_e3706 * _e3706) + (_e3711 * _e3711))) / _e3753))) - 1f) * 0.5f);
                                                            }
                                                            let _e3766 = phi_10569_;
                                                            let _e3767 = (1f + _e3766);
                                                            let _e3768 = (_e3691 * _e3691);
                                                            if (_e3768 >= 1f) {
                                                                phi_10597_ = 0f;
                                                            } else {
                                                                phi_10597_ = ((sqrt((1f + (((_e2039 * _e2039) * ((_e3721 * _e3721) + (_e3726 * _e3726))) / _e3768))) - 1f) * 0.5f);
                                                            }
                                                            let _e3781 = phi_10597_;
                                                            let _e3788 = (((_e3706 * (_e3729 * _e3738)) + (_e3711 * (_e3730 * _e3738))) + (_e3716 * _e3741));
                                                            if (_e2037 == 0u) {
                                                                let _e4083 = (1f - _e3788);
                                                                if (_e4083 != _e4083) {
                                                                    phi_11100_ = true;
                                                                } else {
                                                                    phi_11100_ = (0f >= _e4083);
                                                                }
                                                                let _e4087 = phi_11100_;
                                                                let _e4089 = pow(select(_e4083, 0f, _e4087), 5f);
                                                                let _e4090 = (1f - _e4089);
                                                                phi_4449_ = vec3<f32>(((_e2035.x * _e4090) + _e4089), ((_e2035.y * _e4090) + _e4089), ((_e2035.z * _e4090) + _e4089));
                                                            } else {
                                                                if (_e2037 == 1u) {
                                                                    let _e4056 = (1f / _e2031);
                                                                    let _e4060 = ((_e4056 * _e4056) * (1f - (_e3788 * _e3788)));
                                                                    if (_e4060 >= 1f) {
                                                                        phi_11061_ = 1f;
                                                                    } else {
                                                                        let _e4063 = sqrt((1f - _e4060));
                                                                        if (_e3788 != _e3788) {
                                                                            phi_11072_ = true;
                                                                        } else {
                                                                            phi_11072_ = (_e4063 <= _e3788);
                                                                        }
                                                                        let _e4067 = phi_11072_;
                                                                        let _e4072 = pow(((1f - _e2031) / (1f + _e2031)), 2f);
                                                                        phi_11061_ = (_e4072 + ((1f - _e4072) * pow((1f - select(_e3788, _e4063, _e4067)), 5f)));
                                                                    }
                                                                    let _e4079 = phi_11061_;
                                                                    phi_4448_ = vec3(_e4079);
                                                                } else {
                                                                    let _e3792 = select(_e3788, 0f, (_e3788 < 0f));
                                                                    let _e3794 = select(_e3792, 1f, (_e3792 > 1f));
                                                                    let _e3796 = (1f - (_e3794 * _e3794));
                                                                    let _e3809 = ((_e2035.x * _e2035.x) - (_e2033.x * _e2033.x));
                                                                    let _e3810 = ((_e2035.y * _e2035.y) - (_e2033.y * _e2033.y));
                                                                    let _e3811 = ((_e2035.z * _e2035.z) - (_e2033.z * _e2033.z));
                                                                    let _e3813 = (_e2035.x * (_e2033.x + _e2033.x));
                                                                    let _e3815 = (_e2035.y * (_e2033.y + _e2033.y));
                                                                    let _e3817 = (_e2035.z * (_e2033.z + _e2033.z));
                                                                    let _e3824 = ((_e3809 * _e3809) + (_e3813 * _e3813));
                                                                    let _e3825 = ((_e3810 * _e3810) + (_e3815 * _e3815));
                                                                    let _e3826 = ((_e3811 * _e3811) + (_e3817 * _e3817));
                                                                    let _e3831 = ((_e3796 * _e3809) / _e3824);
                                                                    let _e3832 = ((_e3796 * _e3810) / _e3825);
                                                                    let _e3833 = ((_e3796 * _e3811) / _e3826);
                                                                    let _e3834 = (1f - _e3831);
                                                                    let _e3835 = (1f - _e3832);
                                                                    let _e3836 = (1f - _e3833);
                                                                    let _e3838 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3831, _e3832, _e3833));
                                                                    let _e3839 = (_e3796 * _e3813);
                                                                    let _e3840 = (_e3796 * _e3815);
                                                                    let _e3841 = (_e3796 * _e3817);
                                                                    let _e3846 = (vec3<f32>(_e3839, _e3840, _e3841) / vec3<f32>(_e3824, _e3825, _e3826));
                                                                    let _e3859 = ((_e3838.x * _e3838.x) + (_e3846.x * _e3846.x));
                                                                    let _e3860 = ((_e3838.y * _e3838.y) + (_e3846.y * _e3846.y));
                                                                    let _e3861 = ((_e3838.z * _e3838.z) + (_e3846.z * _e3846.z));
                                                                    if (_e3859 != _e3859) {
                                                                        phi_10731_ = true;
                                                                    } else {
                                                                        phi_10731_ = (0f >= _e3859);
                                                                    }
                                                                    let _e3865 = phi_10731_;
                                                                    let _e3867 = sqrt(select(_e3859, 0f, _e3865));
                                                                    if (_e3860 != _e3860) {
                                                                        phi_10746_ = true;
                                                                    } else {
                                                                        phi_10746_ = (0f >= _e3860);
                                                                    }
                                                                    let _e3871 = phi_10746_;
                                                                    let _e3873 = sqrt(select(_e3860, 0f, _e3871));
                                                                    if (_e3861 != _e3861) {
                                                                        phi_10761_ = true;
                                                                    } else {
                                                                        phi_10761_ = (0f >= _e3861);
                                                                    }
                                                                    let _e3877 = phi_10761_;
                                                                    let _e3879 = sqrt(select(_e3861, 0f, _e3877));
                                                                    let _e3883 = ((_e3867 + _e3834) * 0.5f);
                                                                    let _e3884 = ((_e3873 + _e3835) * 0.5f);
                                                                    let _e3885 = ((_e3879 + _e3836) * 0.5f);
                                                                    if (_e3883 != _e3883) {
                                                                        phi_10788_ = true;
                                                                    } else {
                                                                        phi_10788_ = (0f >= _e3883);
                                                                    }
                                                                    let _e3889 = phi_10788_;
                                                                    let _e3891 = sqrt(select(_e3883, 0f, _e3889));
                                                                    if (_e3884 != _e3884) {
                                                                        phi_10803_ = true;
                                                                    } else {
                                                                        phi_10803_ = (0f >= _e3884);
                                                                    }
                                                                    let _e3895 = phi_10803_;
                                                                    let _e3897 = sqrt(select(_e3884, 0f, _e3895));
                                                                    if (_e3885 != _e3885) {
                                                                        phi_10818_ = true;
                                                                    } else {
                                                                        phi_10818_ = (0f >= _e3885);
                                                                    }
                                                                    let _e3901 = phi_10818_;
                                                                    let _e3903 = sqrt(select(_e3885, 0f, _e3901));
                                                                    let _e3904 = vec3<f32>(_e3891, _e3897, _e3903);
                                                                    let _e3908 = ((_e3867 - _e3834) * 0.5f);
                                                                    let _e3909 = ((_e3873 - _e3835) * 0.5f);
                                                                    let _e3910 = ((_e3879 - _e3836) * 0.5f);
                                                                    if (_e3908 != _e3908) {
                                                                        phi_10845_ = true;
                                                                    } else {
                                                                        phi_10845_ = (0f >= _e3908);
                                                                    }
                                                                    let _e3914 = phi_10845_;
                                                                    let _e3916 = sqrt(select(_e3908, 0f, _e3914));
                                                                    if (_e3909 != _e3909) {
                                                                        phi_10860_ = true;
                                                                    } else {
                                                                        phi_10860_ = (0f >= _e3909);
                                                                    }
                                                                    let _e3920 = phi_10860_;
                                                                    let _e3922 = sqrt(select(_e3909, 0f, _e3920));
                                                                    if (_e3910 != _e3910) {
                                                                        phi_10875_ = true;
                                                                    } else {
                                                                        phi_10875_ = (0f >= _e3910);
                                                                    }
                                                                    let _e3926 = phi_10875_;
                                                                    let _e3928 = sqrt(select(_e3910, 0f, _e3926));
                                                                    if ((_e3839 / _e3824) < 0f) {
                                                                        phi_4325_ = -(_e3916);
                                                                    } else {
                                                                        phi_4325_ = _e3916;
                                                                    }
                                                                    let _e3932 = phi_4325_;
                                                                    if ((_e3840 / _e3825) < 0f) {
                                                                        phi_4333_ = -(_e3922);
                                                                    } else {
                                                                        phi_4333_ = _e3922;
                                                                    }
                                                                    let _e3936 = phi_4333_;
                                                                    if ((_e3841 / _e3826) < 0f) {
                                                                        phi_4341_ = -(_e3928);
                                                                    } else {
                                                                        phi_4341_ = _e3928;
                                                                    }
                                                                    let _e3940 = phi_4341_;
                                                                    let _e3944 = (_e2033.x * _e3794);
                                                                    let _e3945 = (_e2033.y * _e3794);
                                                                    let _e3946 = (_e2033.z * _e3794);
                                                                    let _e3956 = vec3<f32>(((_e2035.x * _e3891) - (_e2033.x * _e3932)), ((_e2035.y * _e3897) - (_e2033.y * _e3936)), ((_e2035.z * _e3903) - (_e2033.z * _e3940)));
                                                                    let _e3963 = ((_e2035.x * _e3932) + (_e2033.x * _e3891));
                                                                    let _e3964 = ((_e2035.y * _e3936) + (_e2033.y * _e3897));
                                                                    let _e3965 = ((_e2035.z * _e3940) + (_e2033.z * _e3903));
                                                                    let _e3967 = vec3<f32>((_e2035.x * _e3794), (_e2035.y * _e3794), (_e2035.z * _e3794));
                                                                    let _e3968 = (_e3967 - _e3904);
                                                                    let _e3969 = (_e3944 - _e3932);
                                                                    let _e3970 = (_e3945 - _e3936);
                                                                    let _e3971 = (_e3946 - _e3940);
                                                                    let _e3984 = (_e3967 + _e3904);
                                                                    let _e3985 = (_e3944 + _e3932);
                                                                    let _e3986 = (_e3945 + _e3936);
                                                                    let _e3987 = (_e3946 + _e3940);
                                                                    let _e3997 = ((_e3984.x * _e3984.x) + (_e3985 * _e3985));
                                                                    let _e3998 = ((_e3984.y * _e3984.y) + (_e3986 * _e3986));
                                                                    let _e3999 = ((_e3984.z * _e3984.z) + (_e3987 * _e3987));
                                                                    let _e4009 = vec3(_e3794);
                                                                    let _e4010 = (_e4009 - _e3956);
                                                                    let _e4011 = -(vec3<f32>(_e3963, _e3964, _e3965));
                                                                    let _e4027 = (_e4009 + _e3956);
                                                                    let _e4037 = ((_e4027.x * _e4027.x) + (_e3963 * _e3963));
                                                                    let _e4038 = ((_e4027.y * _e4027.y) + (_e3964 * _e3964));
                                                                    let _e4039 = ((_e4027.z * _e4027.z) + (_e3965 * _e3965));
                                                                    phi_4448_ = vec3<f32>((((((_e3968.x * _e3968.x) + (_e3969 * _e3969)) / select(0.000000000000000000000000000001f, _e3997, (_e3997 > 0.000000000000000000000000000001f))) + (((_e4010.x * _e4010.x) + (_e4011.x * _e4011.x)) / select(0.000000000000000000000000000001f, _e4037, (_e4037 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3968.y * _e3968.y) + (_e3970 * _e3970)) / select(0.000000000000000000000000000001f, _e3998, (_e3998 > 0.000000000000000000000000000001f))) + (((_e4010.y * _e4010.y) + (_e4011.y * _e4011.y)) / select(0.000000000000000000000000000001f, _e4038, (_e4038 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3968.z * _e3968.z) + (_e3971 * _e3971)) / select(0.000000000000000000000000000001f, _e3999, (_e3999 > 0.000000000000000000000000000001f))) + (((_e4010.z * _e4010.z) + (_e4011.z * _e4011.z)) / select(0.000000000000000000000000000001f, _e4039, (_e4039 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                }
                                                                let _e4082 = phi_4448_;
                                                                phi_4449_ = _e4082;
                                                            }
                                                            let _e4102 = phi_4449_;
                                                            let _e4104 = (4f * _e3716);
                                                            phi_4461_ = type_59((_e4102 * (((1f / (_e3767 + _e3781)) * _e3752) / _e4104)), ((_e3752 / _e3767) / _e4104));
                                                        }
                                                        let _e4111 = phi_4461_;
                                                        phi_4462_ = _e4111;
                                                        phi_4463_ = _e3728;
                                                    }
                                                    let _e4113 = phi_4462_;
                                                    let _e4115 = phi_4463_;
                                                    if _e4115 {
                                                        phi_4468_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        phi_4468_ = _e4113;
                                                    }
                                                    let _e4117 = phi_4468_;
                                                    let _e4120 = (_e2041 == 1u);
                                                    if _e4120 {
                                                        phi_4496_ = _e4117;
                                                        phi_4497_ = type_59();
                                                    } else {
                                                        if (_e3716 != _e3716) {
                                                            phi_11137_ = true;
                                                        } else {
                                                            phi_11137_ = (0f >= _e3716);
                                                        }
                                                        let _e4124 = phi_11137_;
                                                        let _e4125 = select(_e3716, 0f, _e4124);
                                                        let _e4126 = (1f / _e2031);
                                                        let _e4130 = ((_e4126 * _e4126) * (1f - (_e4125 * _e4125)));
                                                        if (_e4130 >= 1f) {
                                                            phi_11156_ = 1f;
                                                        } else {
                                                            let _e4133 = sqrt((1f - _e4130));
                                                            if (_e4125 != _e4125) {
                                                                phi_11167_ = true;
                                                            } else {
                                                                phi_11167_ = (_e4133 <= _e4125);
                                                            }
                                                            let _e4137 = phi_11167_;
                                                            let _e4142 = pow(((1f - _e2031) / (1f + _e2031)), 2f);
                                                            phi_11156_ = (_e4142 + ((1f - _e4142) * pow((1f - select(_e4125, _e4133, _e4137)), 5f)));
                                                        }
                                                        let _e4149 = phi_11156_;
                                                        if (_e3691 <= 0f) {
                                                            phi_11206_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_11206_ = (_e2029 * (_e3691 * 0.31830987f));
                                                        }
                                                        let _e4154 = phi_11206_;
                                                        let _e4155 = (1f - _e4149);
                                                        phi_4496_ = type_59();
                                                        phi_4497_ = type_59((_e4117.member + vec3<f32>((_e4154.x * _e4155), (_e4154.y * _e4155), (_e4154.z * _e4155))), ((_e4149 * _e4117.member_1) + (_e4155 * _e3697)));
                                                    }
                                                    let _e4169 = phi_4496_;
                                                    let _e4171 = phi_4497_;
                                                    phi_4498_ = _e4169;
                                                    phi_4499_ = _e4171;
                                                    phi_4500_ = _e4120;
                                                }
                                                let _e4179 = phi_4498_;
                                                let _e4181 = phi_4499_;
                                                let _e4183 = phi_4500_;
                                                if _e4183 {
                                                    phi_4504_ = _e4179;
                                                } else {
                                                    phi_4504_ = _e4181;
                                                }
                                                let _e4185 = phi_4504_;
                                                if (_e3670 > 0f) {
                                                    if (_e3670 >= 300000000000000000000000000000000000000f) {
                                                        phi_11245_ = 1f;
                                                    } else {
                                                        let _e4190 = (_e3670 * _e3670);
                                                        let _e4192 = (_e4190 + (_e4185.member_1 * _e4185.member_1));
                                                        if (_e4192 == 0f) {
                                                            phi_11242_ = 0f;
                                                        } else {
                                                            phi_11242_ = (_e4190 / _e4192);
                                                        }
                                                        let _e4196 = phi_11242_;
                                                        phi_11245_ = _e4196;
                                                    }
                                                    let _e4198 = phi_11245_;
                                                    phi_4513_ = (_e4198 / _e3670);
                                                } else {
                                                    phi_4513_ = 1f;
                                                }
                                                let _e4201 = phi_4513_;
                                                if ((((_e4185.member.x * _e4185.member.x) + (_e4185.member.y * _e4185.member.y)) + (_e4185.member.z * _e4185.member.z)) > 0f) {
                                                    let _e4213 = (vec3<f32>(1f, 1f, 1f) / _e3672);
                                                    local = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                    phi_4547_ = _e239;
                                                    phi_4550_ = _e246;
                                                    phi_4552_ = _e243;
                                                    phi_4556_ = type_49(_e3674, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                    loop {
                                                        let _e4215 = phi_4547_;
                                                        let _e4217 = phi_4550_;
                                                        let _e4219 = phi_4552_;
                                                        let _e4221 = phi_4556_;
                                                        local_53 = _e4221;
                                                        if (_e4215 < _e4219) {
                                                            let _e4390 = global_2.member[_e4215];
                                                            if (_e4390.member_6.x == 0u) {
                                                                let _e4496 = (_e4390.member_1.x - _e4390.member.x);
                                                                let _e4497 = (_e4390.member_1.y - _e4390.member.y);
                                                                let _e4498 = (_e4390.member_1.z - _e4390.member.z);
                                                                let _e4503 = (_e4390.member_2.x - _e4390.member.x);
                                                                let _e4504 = (_e4390.member_2.y - _e4390.member.y);
                                                                let _e4505 = (_e4390.member_2.z - _e4390.member.z);
                                                                let _e4508 = ((_e3672.y * _e4505) - (_e4504 * _e3672.z));
                                                                let _e4511 = ((_e3672.z * _e4503) - (_e4505 * _e3672.x));
                                                                let _e4514 = ((_e3672.x * _e4504) - (_e4503 * _e3672.y));
                                                                let _e4519 = (((_e4496 * _e4508) + (_e4497 * _e4511)) + (_e4498 * _e4514));
                                                                if (abs(_e4519) < 0.000000000001f) {
                                                                    phi_4723_ = vec4<f32>();
                                                                    phi_4724_ = true;
                                                                } else {
                                                                    let _e4522 = (1f / _e4519);
                                                                    let _e4523 = (_e1191 - _e4390.member.x);
                                                                    let _e4524 = (_e1192 - _e4390.member.y);
                                                                    let _e4525 = (_e1193 - _e4390.member.z);
                                                                    let _e4531 = ((((_e4523 * _e4508) + (_e4524 * _e4511)) + (_e4525 * _e4514)) * _e4522);
                                                                    if (_e4531 < 0f) {
                                                                        phi_4713_ = vec4<f32>();
                                                                        phi_4714_ = true;
                                                                        phi_4715_ = false;
                                                                    } else {
                                                                        let _e4533 = (_e4531 > 1f);
                                                                        if _e4533 {
                                                                            phi_4711_ = vec4<f32>();
                                                                            phi_4712_ = false;
                                                                        } else {
                                                                            let _e4536 = ((_e4524 * _e4498) - (_e4497 * _e4525));
                                                                            let _e4539 = ((_e4525 * _e4496) - (_e4498 * _e4523));
                                                                            let _e4542 = ((_e4523 * _e4497) - (_e4496 * _e4524));
                                                                            let _e4548 = ((((_e3672.x * _e4536) + (_e3672.y * _e4539)) + (_e3672.z * _e4542)) * _e4522);
                                                                            if (_e4548 < 0f) {
                                                                                phi_4701_ = vec4<f32>();
                                                                                phi_4702_ = true;
                                                                                phi_4703_ = false;
                                                                            } else {
                                                                                let _e4551 = ((_e4531 + _e4548) > 1f);
                                                                                if _e4551 {
                                                                                    phi_4699_ = vec4<f32>();
                                                                                    phi_4700_ = false;
                                                                                } else {
                                                                                    let _e4557 = ((((_e4503 * _e4536) + (_e4504 * _e4539)) + (_e4505 * _e4542)) * _e4522);
                                                                                    if (_e4557 <= 0f) {
                                                                                        phi_4694_ = vec4<f32>();
                                                                                        phi_4695_ = true;
                                                                                    } else {
                                                                                        let _e4559 = (_e4557 >= _e4221.member);
                                                                                        if _e4559 {
                                                                                            phi_4693_ = vec4<f32>();
                                                                                        } else {
                                                                                            phi_4693_ = vec4<f32>(_e4557, _e4531, _e4548, 1f);
                                                                                        }
                                                                                        let _e4562 = phi_4693_;
                                                                                        phi_4694_ = _e4562;
                                                                                        phi_4695_ = _e4559;
                                                                                    }
                                                                                    let _e4564 = phi_4694_;
                                                                                    let _e4566 = phi_4695_;
                                                                                    phi_4699_ = _e4564;
                                                                                    phi_4700_ = _e4566;
                                                                                }
                                                                                let _e4568 = phi_4699_;
                                                                                let _e4570 = phi_4700_;
                                                                                phi_4701_ = _e4568;
                                                                                phi_4702_ = _e4551;
                                                                                phi_4703_ = _e4570;
                                                                            }
                                                                            let _e4572 = phi_4701_;
                                                                            let _e4574 = phi_4702_;
                                                                            let _e4576 = phi_4703_;
                                                                            phi_4711_ = _e4572;
                                                                            phi_4712_ = select(_e4576, true, _e4574);
                                                                        }
                                                                        let _e4579 = phi_4711_;
                                                                        let _e4581 = phi_4712_;
                                                                        phi_4713_ = _e4579;
                                                                        phi_4714_ = _e4533;
                                                                        phi_4715_ = _e4581;
                                                                    }
                                                                    let _e4583 = phi_4713_;
                                                                    let _e4585 = phi_4714_;
                                                                    let _e4587 = phi_4715_;
                                                                    phi_4723_ = _e4583;
                                                                    phi_4724_ = select(_e4587, true, _e4585);
                                                                }
                                                                let _e4590 = phi_4723_;
                                                                let _e4592 = phi_4724_;
                                                                phi_4870_ = _e4590;
                                                                phi_4871_ = _e4592;
                                                            } else {
                                                                if (_e4390.member_6.x == 1u) {
                                                                    let _e4418 = (_e1191 - _e4390.member.x);
                                                                    let _e4419 = (_e1192 - _e4390.member.y);
                                                                    let _e4420 = (_e1193 - _e4390.member.z);
                                                                    let _e4425 = (((_e4418 * _e3672.x) + (_e4419 * _e3672.y)) + (_e4420 * _e3672.z));
                                                                    if (_e4425 > 0f) {
                                                                        phi_4764_ = select(true, false, ((((_e4418 * _e4418) + (_e4419 * _e4419)) + (_e4420 * _e4420)) > (_e4390.member.w * _e4390.member.w)));
                                                                    } else {
                                                                        phi_4764_ = true;
                                                                    }
                                                                    let _e4438 = phi_4764_;
                                                                    if _e4438 {
                                                                        let _e4442 = (_e4418 - (_e3672.x * _e4425));
                                                                        let _e4443 = (_e4419 - (_e3672.y * _e4425));
                                                                        let _e4444 = (_e4420 - (_e3672.z * _e4425));
                                                                        let _e4453 = ((_e4390.member.w * _e4390.member.w) - (((_e4442 * _e4442) + (_e4443 * _e4443)) + (_e4444 * _e4444)));
                                                                        if (_e4453 < 0f) {
                                                                            phi_4823_ = vec4<f32>();
                                                                            phi_4824_ = true;
                                                                        } else {
                                                                            let _e4455 = sqrt(_e4453);
                                                                            let _e4456 = -(_e4425);
                                                                            let _e4457 = (_e4456 - _e4455);
                                                                            if (_e4457 <= 0f) {
                                                                                phi_4801_ = true;
                                                                            } else {
                                                                                phi_4801_ = (_e4457 >= _e4221.member);
                                                                            }
                                                                            let _e4461 = phi_4801_;
                                                                            if _e4461 {
                                                                                phi_4807_ = (_e4456 + _e4455);
                                                                            } else {
                                                                                phi_4807_ = _e4457;
                                                                            }
                                                                            let _e4464 = phi_4807_;
                                                                            if (_e4464 <= 0f) {
                                                                                phi_4818_ = vec4<f32>();
                                                                                phi_4819_ = true;
                                                                            } else {
                                                                                let _e4466 = (_e4464 >= _e4221.member);
                                                                                if _e4466 {
                                                                                    phi_4817_ = vec4<f32>();
                                                                                } else {
                                                                                    phi_4817_ = vec4<f32>(_e4464, 0f, 0f, 1f);
                                                                                }
                                                                                let _e4469 = phi_4817_;
                                                                                phi_4818_ = _e4469;
                                                                                phi_4819_ = _e4466;
                                                                            }
                                                                            let _e4471 = phi_4818_;
                                                                            let _e4473 = phi_4819_;
                                                                            phi_4823_ = _e4471;
                                                                            phi_4824_ = _e4473;
                                                                        }
                                                                        let _e4475 = phi_4823_;
                                                                        let _e4477 = phi_4824_;
                                                                        phi_4828_ = _e4475;
                                                                        phi_4829_ = _e4477;
                                                                    } else {
                                                                        phi_4828_ = vec4<f32>();
                                                                        phi_4829_ = true;
                                                                    }
                                                                    let _e4479 = phi_4828_;
                                                                    let _e4481 = phi_4829_;
                                                                    phi_4860_ = _e4479;
                                                                    phi_4861_ = _e4481;
                                                                } else {
                                                                    if (abs(_e3672.y) < 0.000000000001f) {
                                                                        phi_4858_ = vec4<f32>();
                                                                        phi_4859_ = true;
                                                                    } else {
                                                                        let _e4400 = ((_e4390.member.x - _e1192) / _e3672.y);
                                                                        if (_e4400 <= 0f) {
                                                                            phi_4853_ = vec4<f32>();
                                                                            phi_4854_ = true;
                                                                        } else {
                                                                            let _e4402 = (_e4400 >= _e4221.member);
                                                                            if _e4402 {
                                                                                phi_4852_ = vec4<f32>();
                                                                            } else {
                                                                                phi_4852_ = vec4<f32>(_e4400, 0f, 0f, 1f);
                                                                            }
                                                                            let _e4405 = phi_4852_;
                                                                            phi_4853_ = _e4405;
                                                                            phi_4854_ = _e4402;
                                                                        }
                                                                        let _e4407 = phi_4853_;
                                                                        let _e4409 = phi_4854_;
                                                                        phi_4858_ = _e4407;
                                                                        phi_4859_ = _e4409;
                                                                    }
                                                                    let _e4411 = phi_4858_;
                                                                    let _e4413 = phi_4859_;
                                                                    phi_4860_ = _e4411;
                                                                    phi_4861_ = _e4413;
                                                                }
                                                                let _e4483 = phi_4860_;
                                                                let _e4485 = phi_4861_;
                                                                phi_4870_ = _e4483;
                                                                phi_4871_ = select(true, false, select(true, false, _e4485));
                                                            }
                                                            let _e4594 = phi_4870_;
                                                            let _e4596 = phi_4871_;
                                                            let _e4599 = select(vec4<f32>(0f, 0f, 0f, 0f), _e4594, vec4(select(true, false, _e4596)));
                                                            let _e4601 = (_e4599.w > 0f);
                                                            if _e4601 {
                                                                let _e4607 = type_49(_e4599.x, _e4221.member_1, _e4221.member_2, _e4221.member_3);
                                                                let _e4612 = type_49(_e4607.member, _e4215, _e4607.member_2, _e4607.member_3);
                                                                phi_4892_ = _e4612;
                                                                phi_4894_ = _e4612;
                                                            } else {
                                                                phi_4892_ = type_49();
                                                                phi_4894_ = _e4221;
                                                            }
                                                            let _e4614 = phi_4892_;
                                                            let _e4616 = phi_4894_;
                                                            let _e4617 = select(true, false, _e4601);
                                                            if _e4617 {
                                                                phi_4900_ = (_e4215 + 1u);
                                                            } else {
                                                                phi_4900_ = u32();
                                                            }
                                                            let _e4620 = phi_4900_;
                                                            phi_4548_ = _e4620;
                                                            phi_4551_ = _e4217;
                                                            phi_4553_ = _e4219;
                                                            phi_4557_ = _e4616;
                                                            phi_4973_ = _e4614;
                                                            phi_4974_ = _e4617;
                                                            phi_4975_ = false;
                                                        } else {
                                                            let _e4223 = (_e4217 == 0u);
                                                            if _e4223 {
                                                                phi_4969_ = u32();
                                                                phi_4970_ = u32();
                                                                phi_4971_ = u32();
                                                            } else {
                                                                let _e4224 = (_e4217 - 1u);
                                                                let _e4225 = (_e4224 < 100u);
                                                                if _e4225 {
                                                                } else {
                                                                    phi_15122_ = true;
                                                                    phi_7747_ = bool();
                                                                    phi_7746_ = type_49();
                                                                    break;
                                                                }
                                                                let _e4227 = local[_e4224];
                                                                let _e4229 = global_3.member[_e4227];
                                                                let _e4240 = ((_e4229.member.x - _e1191) * _e4213.x);
                                                                let _e4242 = ((_e4229.member.y - _e1192) * _e4213.y);
                                                                let _e4244 = ((_e4229.member.z - _e1193) * _e4213.z);
                                                                let _e4251 = ((_e4229.member_2.x - _e1191) * _e4213.x);
                                                                let _e4252 = ((_e4229.member_2.y - _e1192) * _e4213.y);
                                                                let _e4253 = ((_e4229.member_2.z - _e1193) * _e4213.z);
                                                                if (_e4240 == _e4240) {
                                                                    let _e4255 = (_e4251 == _e4251);
                                                                    if _e4255 {
                                                                        let _e4256 = (_e4240 != _e4240);
                                                                        if _e4256 {
                                                                            phi_11342_ = true;
                                                                        } else {
                                                                            phi_11342_ = (_e4251 <= _e4240);
                                                                        }
                                                                        let _e4259 = phi_11342_;
                                                                        let _e4260 = select(_e4240, _e4251, _e4259);
                                                                        if _e4256 {
                                                                            phi_11372_ = true;
                                                                        } else {
                                                                            phi_11372_ = (_e4251 >= _e4240);
                                                                        }
                                                                        let _e4265 = phi_11372_;
                                                                        let _e4266 = select(_e4240, _e4251, _e4265);
                                                                        phi_11319_ = type_52(select(-300000000000000000000000000000000000000f, _e4260, (_e4260 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4266, (_e4266 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_11319_ = type_52();
                                                                    }
                                                                    let _e4271 = phi_11319_;
                                                                    phi_11322_ = _e4271;
                                                                    phi_11323_ = select(true, false, _e4255);
                                                                } else {
                                                                    phi_11322_ = type_52();
                                                                    phi_11323_ = true;
                                                                }
                                                                let _e4274 = phi_11322_;
                                                                let _e4276 = phi_11323_;
                                                                if _e4276 {
                                                                    phi_11328_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_11328_ = _e4274;
                                                                }
                                                                let _e4278 = phi_11328_;
                                                                if (_e4242 == _e4242) {
                                                                    let _e4282 = (_e4252 == _e4252);
                                                                    if _e4282 {
                                                                        let _e4283 = (_e4242 != _e4242);
                                                                        if _e4283 {
                                                                            phi_11429_ = true;
                                                                        } else {
                                                                            phi_11429_ = (_e4252 <= _e4242);
                                                                        }
                                                                        let _e4286 = phi_11429_;
                                                                        let _e4287 = select(_e4242, _e4252, _e4286);
                                                                        if (_e4278.member != _e4278.member) {
                                                                            phi_11444_ = true;
                                                                        } else {
                                                                            phi_11444_ = (_e4287 >= _e4278.member);
                                                                        }
                                                                        let _e4291 = phi_11444_;
                                                                        if _e4283 {
                                                                            phi_11459_ = true;
                                                                        } else {
                                                                            phi_11459_ = (_e4252 >= _e4242);
                                                                        }
                                                                        let _e4295 = phi_11459_;
                                                                        let _e4296 = select(_e4242, _e4252, _e4295);
                                                                        if (_e4278.member_1 != _e4278.member_1) {
                                                                            phi_11474_ = true;
                                                                        } else {
                                                                            phi_11474_ = (_e4296 <= _e4278.member_1);
                                                                        }
                                                                        let _e4300 = phi_11474_;
                                                                        phi_11406_ = type_52(select(_e4278.member, _e4287, _e4291), select(_e4278.member_1, _e4296, _e4300));
                                                                    } else {
                                                                        phi_11406_ = type_52();
                                                                    }
                                                                    let _e4304 = phi_11406_;
                                                                    phi_11409_ = _e4304;
                                                                    phi_11410_ = select(true, false, _e4282);
                                                                } else {
                                                                    phi_11409_ = type_52();
                                                                    phi_11410_ = true;
                                                                }
                                                                let _e4307 = phi_11409_;
                                                                let _e4309 = phi_11410_;
                                                                if _e4309 {
                                                                    phi_11415_ = _e4278;
                                                                } else {
                                                                    phi_11415_ = _e4307;
                                                                }
                                                                let _e4311 = phi_11415_;
                                                                if (_e4244 == _e4244) {
                                                                    let _e4315 = (_e4253 == _e4253);
                                                                    if _e4315 {
                                                                        let _e4316 = (_e4244 != _e4244);
                                                                        if _e4316 {
                                                                            phi_11516_ = true;
                                                                        } else {
                                                                            phi_11516_ = (_e4253 <= _e4244);
                                                                        }
                                                                        let _e4319 = phi_11516_;
                                                                        let _e4320 = select(_e4244, _e4253, _e4319);
                                                                        if (_e4311.member != _e4311.member) {
                                                                            phi_11531_ = true;
                                                                        } else {
                                                                            phi_11531_ = (_e4320 >= _e4311.member);
                                                                        }
                                                                        let _e4324 = phi_11531_;
                                                                        if _e4316 {
                                                                            phi_11546_ = true;
                                                                        } else {
                                                                            phi_11546_ = (_e4253 >= _e4244);
                                                                        }
                                                                        let _e4328 = phi_11546_;
                                                                        let _e4329 = select(_e4244, _e4253, _e4328);
                                                                        if (_e4311.member_1 != _e4311.member_1) {
                                                                            phi_11561_ = true;
                                                                        } else {
                                                                            phi_11561_ = (_e4329 <= _e4311.member_1);
                                                                        }
                                                                        let _e4333 = phi_11561_;
                                                                        phi_11493_ = type_52(select(_e4311.member, _e4320, _e4324), select(_e4311.member_1, _e4329, _e4333));
                                                                    } else {
                                                                        phi_11493_ = type_52();
                                                                    }
                                                                    let _e4337 = phi_11493_;
                                                                    phi_11496_ = _e4337;
                                                                    phi_11497_ = select(true, false, _e4315);
                                                                } else {
                                                                    phi_11496_ = type_52();
                                                                    phi_11497_ = true;
                                                                }
                                                                let _e4340 = phi_11496_;
                                                                let _e4342 = phi_11497_;
                                                                if _e4342 {
                                                                    phi_11502_ = _e4311;
                                                                } else {
                                                                    phi_11502_ = _e4340;
                                                                }
                                                                let _e4344 = phi_11502_;
                                                                if (_e4344.member_1 < 0f) {
                                                                    phi_11298_ = bool();
                                                                    phi_11299_ = true;
                                                                } else {
                                                                    let _e4348 = (_e4344.member > _e4344.member_1);
                                                                    if _e4348 {
                                                                        phi_11295_ = bool();
                                                                    } else {
                                                                        phi_11295_ = (_e4344.member > _e4221.member);
                                                                    }
                                                                    let _e4351 = phi_11295_;
                                                                    phi_11298_ = _e4351;
                                                                    phi_11299_ = _e4348;
                                                                }
                                                                let _e4353 = phi_11298_;
                                                                let _e4355 = phi_11299_;
                                                                if !(select(_e4353, true, _e4355)) {
                                                                    if (_e4229.member_3 > 0u) {
                                                                        phi_4963_ = _e4229.member_1;
                                                                        phi_4964_ = _e4224;
                                                                        phi_4965_ = (_e4229.member_1 + _e4229.member_3);
                                                                    } else {
                                                                        let _e4360 = (_e4217 + 1u);
                                                                        let _e4361 = (_e4360 <= 100u);
                                                                        if _e4361 {
                                                                            if _e4225 {
                                                                            } else {
                                                                                phi_15122_ = true;
                                                                                phi_7747_ = bool();
                                                                                phi_7746_ = type_49();
                                                                                break;
                                                                            }
                                                                            local[_e4224] = _e4229.member_1;
                                                                            if (_e4217 < 100u) {
                                                                            } else {
                                                                                phi_15122_ = true;
                                                                                phi_7747_ = bool();
                                                                                phi_7746_ = type_49();
                                                                                break;
                                                                            }
                                                                            local[_e4217] = (_e4227 + 1u);
                                                                        }
                                                                        phi_4963_ = _e4215;
                                                                        phi_4964_ = select(_e4224, _e4360, _e4361);
                                                                        phi_4965_ = _e4219;
                                                                    }
                                                                    let _e4370 = phi_4963_;
                                                                    let _e4372 = phi_4964_;
                                                                    let _e4374 = phi_4965_;
                                                                    phi_4966_ = _e4370;
                                                                    phi_4967_ = _e4372;
                                                                    phi_4968_ = _e4374;
                                                                } else {
                                                                    phi_4966_ = _e4215;
                                                                    phi_4967_ = _e4224;
                                                                    phi_4968_ = _e4219;
                                                                }
                                                                let _e4376 = phi_4966_;
                                                                let _e4378 = phi_4967_;
                                                                let _e4380 = phi_4968_;
                                                                phi_4969_ = _e4376;
                                                                phi_4970_ = _e4378;
                                                                phi_4971_ = _e4380;
                                                            }
                                                            let _e4382 = phi_4969_;
                                                            let _e4384 = phi_4970_;
                                                            let _e4386 = phi_4971_;
                                                            phi_4548_ = _e4382;
                                                            phi_4551_ = _e4384;
                                                            phi_4553_ = _e4386;
                                                            phi_4557_ = _e4221;
                                                            phi_4973_ = type_49();
                                                            phi_4974_ = select(true, false, _e4223);
                                                            phi_4975_ = _e4223;
                                                        }
                                                        let _e4622 = phi_4548_;
                                                        let _e4624 = phi_4551_;
                                                        let _e4626 = phi_4553_;
                                                        let _e4628 = phi_4557_;
                                                        let _e4630 = phi_4973_;
                                                        let _e4632 = phi_4974_;
                                                        let _e4634 = phi_4975_;
                                                        phi_4547_ = _e4622;
                                                        phi_4550_ = _e4624;
                                                        phi_4552_ = _e4626;
                                                        phi_4556_ = _e4628;
                                                        phi_15122_ = _e2064;
                                                        phi_7747_ = _e4634;
                                                        phi_7746_ = _e4630;
                                                        if !(_e4632) {
                                                            break;
                                                        }
                                                    }
                                                    let _e4637 = phi_15122_;
                                                    let _e4639 = phi_7747_;
                                                    let _e4641 = phi_7746_;
                                                    phi_15513_ = _e3666;
                                                    phi_15500_ = _e4637;
                                                    if _e4637 {
                                                        break;
                                                    }
                                                    if _e4639 {
                                                        let _e7117 = local_53;
                                                        phi_5147_ = _e7117;
                                                    } else {
                                                        phi_5147_ = type_49();
                                                    }
                                                    let _e4643 = phi_5147_;
                                                    if select(true, false, _e4639) {
                                                        phi_5152_ = _e4641;
                                                    } else {
                                                        phi_5152_ = _e4643;
                                                    }
                                                    let _e4646 = phi_5152_;
                                                    if (_e4646.member_1 != 4294967295u) {
                                                        phi_5185_ = _e2070;
                                                    } else {
                                                        phi_5185_ = vec3<f32>((_e2070.x + (((_e1054.x * _e4185.member.x) * _e3668.x) * _e4201)), (_e2070.y + (((_e1054.y * _e4185.member.y) * _e3668.y) * _e4201)), (_e2070.z + (((_e1054.z * _e4185.member.z) * _e3668.z) * _e4201)));
                                                    }
                                                    let _e4669 = phi_5185_;
                                                    phi_15503_ = _e4637;
                                                    phi_5186_ = (_e2068 + 1u);
                                                    phi_5187_ = _e4669;
                                                } else {
                                                    phi_15503_ = _e2064;
                                                    phi_5186_ = _e2068;
                                                    phi_5187_ = _e2070;
                                                }
                                                let _e4671 = phi_15503_;
                                                let _e4673 = phi_5186_;
                                                let _e4675 = phi_5187_;
                                                phi_15502_ = _e4671;
                                                phi_5188_ = _e4673;
                                                phi_5189_ = _e4675;
                                            } else {
                                                phi_15502_ = _e2064;
                                                phi_5188_ = _e2068;
                                                phi_5189_ = _e2070;
                                            }
                                            let _e4677 = phi_15502_;
                                            let _e4679 = phi_5188_;
                                            let _e4681 = phi_5189_;
                                            phi_15719_ = _e3666;
                                            phi_15501_ = _e4677;
                                            phi_2227_ = _e4679;
                                            phi_2230_ = _e4681;
                                            phi_2232_ = (_e2072 + 1u);
                                        } else {
                                            phi_15719_ = _e2066;
                                            phi_15501_ = _e2064;
                                            phi_2227_ = u32();
                                            phi_2230_ = vec3<f32>();
                                            phi_2232_ = u32();
                                        }
                                        let _e4684 = phi_15719_;
                                        let _e4686 = phi_15501_;
                                        let _e4688 = phi_2227_;
                                        let _e4690 = phi_2230_;
                                        let _e4692 = phi_2232_;
                                        phi_15202_ = _e4686;
                                        phi_13445_ = _e4684;
                                        phi_2226_ = _e4688;
                                        phi_2229_ = _e4690;
                                        phi_2231_ = _e4692;
                                        phi_15513_ = _e4684;
                                        phi_15500_ = _e4686;
                                        if !(_e2074) {
                                            break;
                                        }
                                    }
                                    let _e4695 = phi_15513_;
                                    let _e4697 = phi_15500_;
                                    phi_16967_ = _e4697;
                                    phi_7779_ = bool();
                                    phi_7778_ = u32();
                                    phi_7777_ = type_49();
                                    phi_7776_ = bool();
                                    phi_7775_ = bool();
                                    phi_7774_ = bool();
                                    phi_7773_ = bool();
                                    phi_7772_ = bool();
                                    phi_7771_ = bool();
                                    phi_7770_ = bool();
                                    phi_7769_ = vec3<f32>();
                                    phi_7767_ = vec3<f32>();
                                    phi_7766_ = u32();
                                    phi_7759_ = u32();
                                    phi_7757_ = type_14();
                                    phi_7755_ = vec3<f32>();
                                    phi_7754_ = f32();
                                    phi_7753_ = f32();
                                    phi_7752_ = f32();
                                    phi_7750_ = vec3<f32>();
                                    if _e4697 {
                                        break;
                                    }
                                    let _e4698 = (_e230 >= _e208);
                                    if _e4698 {
                                        phi_17065_ = _e4695;
                                        phi_5729_ = vec3<f32>();
                                        phi_5730_ = f32();
                                        phi_5731_ = vec3<f32>();
                                        phi_5732_ = u32();
                                        phi_5733_ = vec3<f32>();
                                        phi_5734_ = false;
                                        phi_5735_ = false;
                                        phi_5736_ = false;
                                        phi_5737_ = false;
                                    } else {
                                        let _e4700 = ((_e4695 * 747796405u) + 2891336453u);
                                        let _e4708 = (((_e4700 >> bitcast<u32>((((_e4700 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4700) * 277803737u);
                                        let _e4711 = ((_e4708 >> bitcast<u32>(22i)) ^ _e4708);
                                        let _e4714 = f32((_e4711 >> bitcast<u32>(8i)));
                                        let _e4717 = ((_e4711 * 747796405u) + 2891336453u);
                                        let _e4725 = (((_e4717 >> bitcast<u32>((((_e4717 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4717) * 277803737u);
                                        let _e4728 = ((_e4725 >> bitcast<u32>(22i)) ^ _e4725);
                                        let _e4731 = f32((_e4728 >> bitcast<u32>(8i)));
                                        let _e4733 = ((_e4728 * 747796405u) + 2891336453u);
                                        let _e4741 = (((_e4733 >> bitcast<u32>((((_e4733 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4733) * 277803737u);
                                        let _e4744 = ((_e4741 >> bitcast<u32>(22i)) ^ _e4741);
                                        let _e4749 = (_e2041 == 0u);
                                        if _e4749 {
                                            phi_11656_ = vec3<f32>();
                                            phi_11657_ = true;
                                        } else {
                                            if (_e2041 == 2u) {
                                                let _e4758 = (((_e1140.x * _e1166.x) + (_e1140.y * _e1166.y)) + (_e1140.z * _e1166.z));
                                                if (_e4758 != _e4758) {
                                                    phi_11697_ = true;
                                                } else {
                                                    phi_11697_ = (0f >= _e4758);
                                                }
                                                let _e4762 = phi_11697_;
                                                let _e4763 = select(_e4758, 0f, _e4762);
                                                let _e4764 = (1f / _e2031);
                                                let _e4768 = ((_e4764 * _e4764) * (1f - (_e4763 * _e4763)));
                                                if (_e4768 >= 1f) {
                                                    phi_11716_ = 1f;
                                                } else {
                                                    let _e4771 = sqrt((1f - _e4768));
                                                    if (_e4763 != _e4763) {
                                                        phi_11727_ = true;
                                                    } else {
                                                        phi_11727_ = (_e4771 <= _e4763);
                                                    }
                                                    let _e4775 = phi_11727_;
                                                    let _e4780 = pow(((1f - _e2031) / (1f + _e2031)), 2f);
                                                    phi_11716_ = (_e4780 + ((1f - _e4780) * pow((1f - select(_e4763, _e4771, _e4775)), 5f)));
                                                }
                                                let _e4787 = phi_11716_;
                                                phi_11647_ = select(true, false, ((f32((_e4744 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e4787));
                                            } else {
                                                phi_11647_ = true;
                                            }
                                            let _e4791 = phi_11647_;
                                            if _e4791 {
                                                let _e4799 = (((_e1140.x * _e1212) + (_e1140.y * _e1213)) + (_e1140.z * _e1215));
                                                let _e4804 = (((_e1140.x * _e1208) + (_e1140.y * _e1218)) + (_e1140.z * _e1219));
                                                let _e4809 = (((_e1140.x * _e1166.x) + (_e1140.y * _e1166.y)) + (_e1140.z * _e1166.z));
                                                if (_e4809 <= 0f) {
                                                    phi_11881_ = vec3<f32>();
                                                    phi_11882_ = true;
                                                } else {
                                                    let _e4812 = (_e2039 * _e4799);
                                                    let _e4813 = (_e2039 * _e4804);
                                                    let _e4820 = (1f / sqrt((((_e4812 * _e4812) + (_e4813 * _e4813)) + (_e4809 * _e4809))));
                                                    let _e4821 = (_e4812 * _e4820);
                                                    let _e4822 = (_e4813 * _e4820);
                                                    let _e4823 = (_e4809 * _e4820);
                                                    let _e4826 = ((_e4821 * _e4821) + (_e4822 * _e4822));
                                                    if (_e4826 > 0f) {
                                                        let _e4829 = sqrt(_e4826);
                                                        phi_11783_ = vec3<f32>((-(_e4822) / _e4829), (_e4821 / _e4829), 0f);
                                                    } else {
                                                        phi_11783_ = vec3<f32>(1f, 0f, 0f);
                                                    }
                                                    let _e4834 = phi_11783_;
                                                    let _e4847 = sqrt((_e4714 * 0.000000059604645f));
                                                    let _e4848 = (_e4731 * 0.00000037450704f);
                                                    let _e4850 = (_e4847 * cos(_e4848));
                                                    let _e4852 = (0.5f * (1f + _e4823));
                                                    let _e4855 = (1f - (_e4850 * _e4850));
                                                    if (_e4855 != _e4855) {
                                                        phi_11939_ = true;
                                                    } else {
                                                        phi_11939_ = (0f >= _e4855);
                                                    }
                                                    let _e4859 = phi_11939_;
                                                    let _e4866 = (((1f - _e4852) * sqrt(select(_e4855, 0f, _e4859))) + ((_e4852 * _e4847) * sin(_e4848)));
                                                    let _e4877 = (_e4855 - (_e4866 * _e4866));
                                                    if (_e4877 != _e4877) {
                                                        phi_11954_ = true;
                                                    } else {
                                                        phi_11954_ = (0f >= _e4877);
                                                    }
                                                    let _e4881 = phi_11954_;
                                                    let _e4883 = sqrt(select(_e4877, 0f, _e4881));
                                                    let _e4889 = (((_e4834.z * _e4850) + (((_e4821 * _e4834.y) - (_e4834.x * _e4822)) * _e4866)) + (_e4823 * _e4883));
                                                    let _e4890 = (_e2039 * (((_e4834.x * _e4850) + (((_e4822 * _e4834.z) - (_e4834.y * _e4823)) * _e4866)) + (_e4821 * _e4883)));
                                                    let _e4891 = (_e2039 * (((_e4834.y * _e4850) + (((_e4823 * _e4834.x) - (_e4834.z * _e4821)) * _e4866)) + (_e4822 * _e4883)));
                                                    if (_e4889 != _e4889) {
                                                        phi_11969_ = true;
                                                    } else {
                                                        phi_11969_ = (0f >= _e4889);
                                                    }
                                                    let _e4895 = phi_11969_;
                                                    let _e4896 = select(_e4889, 0f, _e4895);
                                                    let _e4903 = (1f / sqrt((((_e4890 * _e4890) + (_e4891 * _e4891)) + (_e4896 * _e4896))));
                                                    let _e4904 = (_e4890 * _e4903);
                                                    let _e4905 = (_e4891 * _e4903);
                                                    let _e4906 = (_e4896 * _e4903);
                                                    let _e4912 = (2f * (((_e4799 * _e4904) + (_e4804 * _e4905)) + (_e4809 * _e4906)));
                                                    let _e4915 = (_e4906 * _e4912);
                                                    let _e4918 = (vec3<f32>((_e4904 * _e4912), (_e4905 * _e4912), _e4915) - vec3<f32>(_e4799, _e4804, _e4809));
                                                    let _e4919 = ((_e4915 - _e4809) <= 0f);
                                                    if _e4919 {
                                                        phi_11878_ = vec3<f32>();
                                                    } else {
                                                        phi_11878_ = vec3<f32>((((_e1212 * _e4918.x) + (_e1208 * _e4918.y)) + (_e1166.x * _e4918.z)), (((_e1213 * _e4918.x) + (_e1218 * _e4918.y)) + (_e1166.y * _e4918.z)), (((_e1215 * _e4918.x) + (_e1219 * _e4918.y)) + (_e1166.z * _e4918.z)));
                                                    }
                                                    let _e4940 = phi_11878_;
                                                    phi_11881_ = _e4940;
                                                    phi_11882_ = _e4919;
                                                }
                                                let _e4942 = phi_11881_;
                                                let _e4944 = phi_11882_;
                                                phi_11652_ = select(_e4942, vec3<f32>(0f, 0f, 0f), vec3(_e4944));
                                            } else {
                                                phi_11652_ = vec3<f32>();
                                            }
                                            let _e4948 = phi_11652_;
                                            phi_11656_ = _e4948;
                                            phi_11657_ = select(true, false, _e4791);
                                        }
                                        let _e4951 = phi_11656_;
                                        let _e4953 = phi_11657_;
                                        if _e4953 {
                                            let _e4954 = (_e4714 * 0.00000037450704f);
                                            let _e4956 = ((_e4731 * 0.00000011920929f) - 1f);
                                            let _e4958 = (1f - (_e4956 * _e4956));
                                            if (_e4958 != _e4958) {
                                                phi_12069_ = true;
                                            } else {
                                                phi_12069_ = (0f >= _e4958);
                                            }
                                            let _e4962 = phi_12069_;
                                            let _e4964 = sqrt(select(_e4958, 0f, _e4962));
                                            let _e4969 = (_e1166.x + (_e4964 * cos(_e4954)));
                                            let _e4970 = (_e1166.y + (_e4964 * sin(_e4954)));
                                            let _e4971 = (_e1166.z + _e4956);
                                            let _e4977 = sqrt((((_e4969 * _e4969) + (_e4970 * _e4970)) + (_e4971 * _e4971)));
                                            if (_e4977 < 0.000001f) {
                                                phi_12043_ = _e1166;
                                            } else {
                                                let _e4979 = (1f / _e4977);
                                                phi_12043_ = vec3<f32>((_e4969 * _e4979), (_e4970 * _e4979), (_e4971 * _e4979));
                                            }
                                            let _e4985 = phi_12043_;
                                            phi_11664_ = _e4985;
                                        } else {
                                            phi_11664_ = _e4951;
                                        }
                                        let _e4987 = phi_11664_;
                                        let _e4995 = (((_e4987.x * _e1166.x) + (_e4987.y * _e1166.y)) + (_e4987.z * _e1166.z));
                                        if (_e4995 != _e4995) {
                                            phi_12084_ = true;
                                        } else {
                                            phi_12084_ = (0f >= _e4995);
                                        }
                                        let _e4999 = phi_12084_;
                                        let _e5001 = (select(_e4995, 0f, _e4999) * 0.31830987f);
                                        if _e4749 {
                                            if (_e4995 <= 0f) {
                                                phi_12881_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_12881_ = (_e2035 * (_e4995 * 0.31830987f));
                                            }
                                            let _e5479 = phi_12881_;
                                            phi_5579_ = type_59(_e5479, _e5001);
                                            phi_5580_ = type_59();
                                            phi_5581_ = true;
                                        } else {
                                            let _e5009 = (((_e1140.x * _e1212) + (_e1140.y * _e1213)) + (_e1140.z * _e1215));
                                            let _e5014 = (((_e1140.x * _e1208) + (_e1140.y * _e1218)) + (_e1140.z * _e1219));
                                            let _e5019 = (((_e1140.x * _e1166.x) + (_e1140.y * _e1166.y)) + (_e1140.z * _e1166.z));
                                            let _e5024 = (((_e4987.x * _e1212) + (_e4987.y * _e1213)) + (_e4987.z * _e1215));
                                            let _e5029 = (((_e4987.x * _e1208) + (_e4987.y * _e1218)) + (_e4987.z * _e1219));
                                            if (_e5019 <= 0f) {
                                                phi_5543_ = type_59();
                                                phi_5544_ = true;
                                            } else {
                                                let _e5031 = (_e4995 <= 0f);
                                                if _e5031 {
                                                    phi_5542_ = type_59();
                                                } else {
                                                    let _e5032 = (_e5009 + _e5024);
                                                    let _e5033 = (_e5014 + _e5029);
                                                    let _e5034 = (_e5019 + _e4995);
                                                    let _e5041 = (1f / sqrt((((_e5032 * _e5032) + (_e5033 * _e5033)) + (_e5034 * _e5034))));
                                                    let _e5044 = (_e5034 * _e5041);
                                                    if (_e5044 <= 0f) {
                                                        phi_12194_ = 0f;
                                                    } else {
                                                        let _e5046 = (_e2039 * _e2039);
                                                        let _e5050 = ((((_e5046 - 1f) * _e5044) * _e5044) + 1f);
                                                        phi_12194_ = (_e5046 / ((3.1415927f * _e5050) * _e5050));
                                                    }
                                                    let _e5055 = phi_12194_;
                                                    let _e5056 = (_e5019 * _e5019);
                                                    if (_e5056 >= 1f) {
                                                        phi_12222_ = 0f;
                                                    } else {
                                                        phi_12222_ = ((sqrt((1f + (((_e2039 * _e2039) * ((_e5009 * _e5009) + (_e5014 * _e5014))) / _e5056))) - 1f) * 0.5f);
                                                    }
                                                    let _e5069 = phi_12222_;
                                                    let _e5070 = (1f + _e5069);
                                                    let _e5071 = (_e4995 * _e4995);
                                                    if (_e5071 >= 1f) {
                                                        phi_12250_ = 0f;
                                                    } else {
                                                        phi_12250_ = ((sqrt((1f + (((_e2039 * _e2039) * ((_e5024 * _e5024) + (_e5029 * _e5029))) / _e5071))) - 1f) * 0.5f);
                                                    }
                                                    let _e5084 = phi_12250_;
                                                    let _e5091 = (((_e5009 * (_e5032 * _e5041)) + (_e5014 * (_e5033 * _e5041))) + (_e5019 * _e5044));
                                                    if (_e2037 == 0u) {
                                                        let _e5386 = (1f - _e5091);
                                                        if (_e5386 != _e5386) {
                                                            phi_12753_ = true;
                                                        } else {
                                                            phi_12753_ = (0f >= _e5386);
                                                        }
                                                        let _e5390 = phi_12753_;
                                                        let _e5392 = pow(select(_e5386, 0f, _e5390), 5f);
                                                        let _e5393 = (1f - _e5392);
                                                        phi_5530_ = vec3<f32>(((_e2035.x * _e5393) + _e5392), ((_e2035.y * _e5393) + _e5392), ((_e2035.z * _e5393) + _e5392));
                                                    } else {
                                                        if (_e2037 == 1u) {
                                                            let _e5359 = (1f / _e2031);
                                                            let _e5363 = ((_e5359 * _e5359) * (1f - (_e5091 * _e5091)));
                                                            if (_e5363 >= 1f) {
                                                                phi_12714_ = 1f;
                                                            } else {
                                                                let _e5366 = sqrt((1f - _e5363));
                                                                if (_e5091 != _e5091) {
                                                                    phi_12725_ = true;
                                                                } else {
                                                                    phi_12725_ = (_e5366 <= _e5091);
                                                                }
                                                                let _e5370 = phi_12725_;
                                                                let _e5375 = pow(((1f - _e2031) / (1f + _e2031)), 2f);
                                                                phi_12714_ = (_e5375 + ((1f - _e5375) * pow((1f - select(_e5091, _e5366, _e5370)), 5f)));
                                                            }
                                                            let _e5382 = phi_12714_;
                                                            phi_5529_ = vec3(_e5382);
                                                        } else {
                                                            let _e5095 = select(_e5091, 0f, (_e5091 < 0f));
                                                            let _e5097 = select(_e5095, 1f, (_e5095 > 1f));
                                                            let _e5099 = (1f - (_e5097 * _e5097));
                                                            let _e5112 = ((_e2035.x * _e2035.x) - (_e2033.x * _e2033.x));
                                                            let _e5113 = ((_e2035.y * _e2035.y) - (_e2033.y * _e2033.y));
                                                            let _e5114 = ((_e2035.z * _e2035.z) - (_e2033.z * _e2033.z));
                                                            let _e5116 = (_e2035.x * (_e2033.x + _e2033.x));
                                                            let _e5118 = (_e2035.y * (_e2033.y + _e2033.y));
                                                            let _e5120 = (_e2035.z * (_e2033.z + _e2033.z));
                                                            let _e5127 = ((_e5112 * _e5112) + (_e5116 * _e5116));
                                                            let _e5128 = ((_e5113 * _e5113) + (_e5118 * _e5118));
                                                            let _e5129 = ((_e5114 * _e5114) + (_e5120 * _e5120));
                                                            let _e5134 = ((_e5099 * _e5112) / _e5127);
                                                            let _e5135 = ((_e5099 * _e5113) / _e5128);
                                                            let _e5136 = ((_e5099 * _e5114) / _e5129);
                                                            let _e5137 = (1f - _e5134);
                                                            let _e5138 = (1f - _e5135);
                                                            let _e5139 = (1f - _e5136);
                                                            let _e5141 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5134, _e5135, _e5136));
                                                            let _e5142 = (_e5099 * _e5116);
                                                            let _e5143 = (_e5099 * _e5118);
                                                            let _e5144 = (_e5099 * _e5120);
                                                            let _e5149 = (vec3<f32>(_e5142, _e5143, _e5144) / vec3<f32>(_e5127, _e5128, _e5129));
                                                            let _e5162 = ((_e5141.x * _e5141.x) + (_e5149.x * _e5149.x));
                                                            let _e5163 = ((_e5141.y * _e5141.y) + (_e5149.y * _e5149.y));
                                                            let _e5164 = ((_e5141.z * _e5141.z) + (_e5149.z * _e5149.z));
                                                            if (_e5162 != _e5162) {
                                                                phi_12384_ = true;
                                                            } else {
                                                                phi_12384_ = (0f >= _e5162);
                                                            }
                                                            let _e5168 = phi_12384_;
                                                            let _e5170 = sqrt(select(_e5162, 0f, _e5168));
                                                            if (_e5163 != _e5163) {
                                                                phi_12399_ = true;
                                                            } else {
                                                                phi_12399_ = (0f >= _e5163);
                                                            }
                                                            let _e5174 = phi_12399_;
                                                            let _e5176 = sqrt(select(_e5163, 0f, _e5174));
                                                            if (_e5164 != _e5164) {
                                                                phi_12414_ = true;
                                                            } else {
                                                                phi_12414_ = (0f >= _e5164);
                                                            }
                                                            let _e5180 = phi_12414_;
                                                            let _e5182 = sqrt(select(_e5164, 0f, _e5180));
                                                            let _e5186 = ((_e5170 + _e5137) * 0.5f);
                                                            let _e5187 = ((_e5176 + _e5138) * 0.5f);
                                                            let _e5188 = ((_e5182 + _e5139) * 0.5f);
                                                            if (_e5186 != _e5186) {
                                                                phi_12441_ = true;
                                                            } else {
                                                                phi_12441_ = (0f >= _e5186);
                                                            }
                                                            let _e5192 = phi_12441_;
                                                            let _e5194 = sqrt(select(_e5186, 0f, _e5192));
                                                            if (_e5187 != _e5187) {
                                                                phi_12456_ = true;
                                                            } else {
                                                                phi_12456_ = (0f >= _e5187);
                                                            }
                                                            let _e5198 = phi_12456_;
                                                            let _e5200 = sqrt(select(_e5187, 0f, _e5198));
                                                            if (_e5188 != _e5188) {
                                                                phi_12471_ = true;
                                                            } else {
                                                                phi_12471_ = (0f >= _e5188);
                                                            }
                                                            let _e5204 = phi_12471_;
                                                            let _e5206 = sqrt(select(_e5188, 0f, _e5204));
                                                            let _e5207 = vec3<f32>(_e5194, _e5200, _e5206);
                                                            let _e5211 = ((_e5170 - _e5137) * 0.5f);
                                                            let _e5212 = ((_e5176 - _e5138) * 0.5f);
                                                            let _e5213 = ((_e5182 - _e5139) * 0.5f);
                                                            if (_e5211 != _e5211) {
                                                                phi_12498_ = true;
                                                            } else {
                                                                phi_12498_ = (0f >= _e5211);
                                                            }
                                                            let _e5217 = phi_12498_;
                                                            let _e5219 = sqrt(select(_e5211, 0f, _e5217));
                                                            if (_e5212 != _e5212) {
                                                                phi_12513_ = true;
                                                            } else {
                                                                phi_12513_ = (0f >= _e5212);
                                                            }
                                                            let _e5223 = phi_12513_;
                                                            let _e5225 = sqrt(select(_e5212, 0f, _e5223));
                                                            if (_e5213 != _e5213) {
                                                                phi_12528_ = true;
                                                            } else {
                                                                phi_12528_ = (0f >= _e5213);
                                                            }
                                                            let _e5229 = phi_12528_;
                                                            let _e5231 = sqrt(select(_e5213, 0f, _e5229));
                                                            if ((_e5142 / _e5127) < 0f) {
                                                                phi_5406_ = -(_e5219);
                                                            } else {
                                                                phi_5406_ = _e5219;
                                                            }
                                                            let _e5235 = phi_5406_;
                                                            if ((_e5143 / _e5128) < 0f) {
                                                                phi_5414_ = -(_e5225);
                                                            } else {
                                                                phi_5414_ = _e5225;
                                                            }
                                                            let _e5239 = phi_5414_;
                                                            if ((_e5144 / _e5129) < 0f) {
                                                                phi_5422_ = -(_e5231);
                                                            } else {
                                                                phi_5422_ = _e5231;
                                                            }
                                                            let _e5243 = phi_5422_;
                                                            let _e5247 = (_e2033.x * _e5097);
                                                            let _e5248 = (_e2033.y * _e5097);
                                                            let _e5249 = (_e2033.z * _e5097);
                                                            let _e5259 = vec3<f32>(((_e2035.x * _e5194) - (_e2033.x * _e5235)), ((_e2035.y * _e5200) - (_e2033.y * _e5239)), ((_e2035.z * _e5206) - (_e2033.z * _e5243)));
                                                            let _e5266 = ((_e2035.x * _e5235) + (_e2033.x * _e5194));
                                                            let _e5267 = ((_e2035.y * _e5239) + (_e2033.y * _e5200));
                                                            let _e5268 = ((_e2035.z * _e5243) + (_e2033.z * _e5206));
                                                            let _e5270 = vec3<f32>((_e2035.x * _e5097), (_e2035.y * _e5097), (_e2035.z * _e5097));
                                                            let _e5271 = (_e5270 - _e5207);
                                                            let _e5272 = (_e5247 - _e5235);
                                                            let _e5273 = (_e5248 - _e5239);
                                                            let _e5274 = (_e5249 - _e5243);
                                                            let _e5287 = (_e5270 + _e5207);
                                                            let _e5288 = (_e5247 + _e5235);
                                                            let _e5289 = (_e5248 + _e5239);
                                                            let _e5290 = (_e5249 + _e5243);
                                                            let _e5300 = ((_e5287.x * _e5287.x) + (_e5288 * _e5288));
                                                            let _e5301 = ((_e5287.y * _e5287.y) + (_e5289 * _e5289));
                                                            let _e5302 = ((_e5287.z * _e5287.z) + (_e5290 * _e5290));
                                                            let _e5312 = vec3(_e5097);
                                                            let _e5313 = (_e5312 - _e5259);
                                                            let _e5314 = -(vec3<f32>(_e5266, _e5267, _e5268));
                                                            let _e5330 = (_e5312 + _e5259);
                                                            let _e5340 = ((_e5330.x * _e5330.x) + (_e5266 * _e5266));
                                                            let _e5341 = ((_e5330.y * _e5330.y) + (_e5267 * _e5267));
                                                            let _e5342 = ((_e5330.z * _e5330.z) + (_e5268 * _e5268));
                                                            phi_5529_ = vec3<f32>((((((_e5271.x * _e5271.x) + (_e5272 * _e5272)) / select(0.000000000000000000000000000001f, _e5300, (_e5300 > 0.000000000000000000000000000001f))) + (((_e5313.x * _e5313.x) + (_e5314.x * _e5314.x)) / select(0.000000000000000000000000000001f, _e5340, (_e5340 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5271.y * _e5271.y) + (_e5273 * _e5273)) / select(0.000000000000000000000000000001f, _e5301, (_e5301 > 0.000000000000000000000000000001f))) + (((_e5313.y * _e5313.y) + (_e5314.y * _e5314.y)) / select(0.000000000000000000000000000001f, _e5341, (_e5341 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5271.z * _e5271.z) + (_e5274 * _e5274)) / select(0.000000000000000000000000000001f, _e5302, (_e5302 > 0.000000000000000000000000000001f))) + (((_e5313.z * _e5313.z) + (_e5314.z * _e5314.z)) / select(0.000000000000000000000000000001f, _e5342, (_e5342 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e5385 = phi_5529_;
                                                        phi_5530_ = _e5385;
                                                    }
                                                    let _e5405 = phi_5530_;
                                                    let _e5407 = (4f * _e5019);
                                                    phi_5542_ = type_59((_e5405 * (((1f / (_e5070 + _e5084)) * _e5055) / _e5407)), ((_e5055 / _e5070) / _e5407));
                                                }
                                                let _e5414 = phi_5542_;
                                                phi_5543_ = _e5414;
                                                phi_5544_ = _e5031;
                                            }
                                            let _e5416 = phi_5543_;
                                            let _e5418 = phi_5544_;
                                            if _e5418 {
                                                phi_5549_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                            } else {
                                                phi_5549_ = _e5416;
                                            }
                                            let _e5420 = phi_5549_;
                                            let _e5423 = (_e2041 == 1u);
                                            if _e5423 {
                                                phi_5577_ = _e5420;
                                                phi_5578_ = type_59();
                                            } else {
                                                if (_e5019 != _e5019) {
                                                    phi_12790_ = true;
                                                } else {
                                                    phi_12790_ = (0f >= _e5019);
                                                }
                                                let _e5427 = phi_12790_;
                                                let _e5428 = select(_e5019, 0f, _e5427);
                                                let _e5429 = (1f / _e2031);
                                                let _e5433 = ((_e5429 * _e5429) * (1f - (_e5428 * _e5428)));
                                                if (_e5433 >= 1f) {
                                                    phi_12809_ = 1f;
                                                } else {
                                                    let _e5436 = sqrt((1f - _e5433));
                                                    if (_e5428 != _e5428) {
                                                        phi_12820_ = true;
                                                    } else {
                                                        phi_12820_ = (_e5436 <= _e5428);
                                                    }
                                                    let _e5440 = phi_12820_;
                                                    let _e5445 = pow(((1f - _e2031) / (1f + _e2031)), 2f);
                                                    phi_12809_ = (_e5445 + ((1f - _e5445) * pow((1f - select(_e5428, _e5436, _e5440)), 5f)));
                                                }
                                                let _e5452 = phi_12809_;
                                                if (_e4995 <= 0f) {
                                                    phi_12859_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_12859_ = (_e2029 * (_e4995 * 0.31830987f));
                                                }
                                                let _e5457 = phi_12859_;
                                                let _e5458 = (1f - _e5452);
                                                phi_5577_ = type_59();
                                                phi_5578_ = type_59((_e5420.member + vec3<f32>((_e5457.x * _e5458), (_e5457.y * _e5458), (_e5457.z * _e5458))), ((_e5452 * _e5420.member_1) + (_e5458 * _e5001)));
                                            }
                                            let _e5472 = phi_5577_;
                                            let _e5474 = phi_5578_;
                                            phi_5579_ = _e5472;
                                            phi_5580_ = _e5474;
                                            phi_5581_ = _e5423;
                                        }
                                        let _e5482 = phi_5579_;
                                        let _e5484 = phi_5580_;
                                        let _e5486 = phi_5581_;
                                        if _e5486 {
                                            phi_5585_ = _e5482;
                                        } else {
                                            phi_5585_ = _e5484;
                                        }
                                        let _e5488 = phi_5585_;
                                        let _e5496 = ((((_e4987.x * _e4987.x) + (_e4987.y * _e4987.y)) + (_e4987.z * _e4987.z)) == 0f);
                                        if _e5496 {
                                            phi_17066_ = _e4744;
                                            phi_5723_ = vec3<f32>();
                                            phi_5724_ = u32();
                                            phi_5725_ = vec3<f32>();
                                            phi_5726_ = false;
                                            phi_5727_ = false;
                                            phi_5728_ = false;
                                        } else {
                                            let _e5497 = (_e5488.member_1 <= 0f);
                                            if _e5497 {
                                                phi_17067_ = _e4744;
                                                phi_5718_ = vec3<f32>();
                                                phi_5719_ = u32();
                                                phi_5720_ = vec3<f32>();
                                                phi_5721_ = false;
                                                phi_5722_ = false;
                                            } else {
                                                let _e5499 = (_e5488.member.x / _e5488.member_1);
                                                let _e5501 = (_e5488.member.y / _e5488.member_1);
                                                let _e5503 = (_e5488.member.z / _e5488.member_1);
                                                let _e5512 = global_1.member.member_4[1u];
                                                if (sqrt((((_e5499 * _e5499) + (_e5501 * _e5501)) + (_e5503 * _e5503))) <= _e5512) {
                                                    phi_17069_ = _e4744;
                                                    phi_5672_ = _e1054;
                                                    phi_5673_ = false;
                                                } else {
                                                    let _e5516 = global_1.member.member_7[1u];
                                                    if (_e5516 == 1u) {
                                                        let _e5521 = global_1.member.member_4[3u];
                                                        if (f32(_e230) >= _e5521) {
                                                            let _e5524 = select(_e5501, _e5499, (_e5499 > _e5501));
                                                            let _e5526 = select(_e5503, _e5524, (_e5524 > _e5503));
                                                            if (_e5526 != _e5526) {
                                                                phi_12906_ = true;
                                                            } else {
                                                                phi_12906_ = (1f <= _e5526);
                                                            }
                                                            let _e5530 = phi_12906_;
                                                            let _e5531 = select(_e5526, 1f, _e5530);
                                                            let _e5533 = ((_e4744 * 747796405u) + 2891336453u);
                                                            let _e5541 = (((_e5533 >> bitcast<u32>((((_e5533 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5533) * 277803737u);
                                                            let _e5544 = ((_e5541 >> bitcast<u32>(22i)) ^ _e5541);
                                                            let _e5549 = ((f32((_e5544 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5531);
                                                            if _e5549 {
                                                                phi_5661_ = vec3<f32>();
                                                            } else {
                                                                phi_5661_ = vec3<f32>((_e1054.x / _e5531), (_e1054.y / _e5531), (_e1054.z / _e5531));
                                                            }
                                                            let _e5558 = phi_5661_;
                                                            phi_17072_ = _e5544;
                                                            phi_5663_ = _e5558;
                                                            phi_5664_ = select(true, false, _e5549);
                                                        } else {
                                                            phi_17072_ = _e4744;
                                                            phi_5663_ = _e1054;
                                                            phi_5664_ = true;
                                                        }
                                                        let _e5561 = phi_17072_;
                                                        let _e5563 = phi_5663_;
                                                        let _e5565 = phi_5664_;
                                                        phi_17071_ = _e5561;
                                                        phi_5665_ = _e5563;
                                                        phi_5666_ = _e5565;
                                                    } else {
                                                        phi_17071_ = _e4744;
                                                        phi_5665_ = _e1054;
                                                        phi_5666_ = true;
                                                    }
                                                    let _e5567 = phi_17071_;
                                                    let _e5569 = phi_5665_;
                                                    let _e5571 = phi_5666_;
                                                    phi_17069_ = _e5567;
                                                    phi_5672_ = select(_e1054, _e5569, vec3(_e5571));
                                                    phi_5673_ = select(false, true, _e5571);
                                                }
                                                let _e5576 = phi_17069_;
                                                let _e5578 = phi_5672_;
                                                let _e5580 = phi_5673_;
                                                if _e5580 {
                                                    if ((((_e4987.x * _e1153.x) + (_e4987.y * _e1153.y)) + (_e4987.z * _e1153.z)) < 0f) {
                                                        phi_5702_ = -(_e1153);
                                                    } else {
                                                        phi_5702_ = _e1153;
                                                    }
                                                    let _e5596 = phi_5702_;
                                                    phi_5714_ = vec3<f32>((_e5578.x * _e5499), (_e5578.y * _e5501), (_e5578.z * _e5503));
                                                    phi_5715_ = (_e230 + 1u);
                                                    phi_5716_ = vec3<f32>((_e1063 + (_e5596.x * _e1184)), (_e1065 + (_e5596.y * _e1184)), (_e1067 + (_e5596.z * _e1184)));
                                                } else {
                                                    phi_5714_ = vec3<f32>();
                                                    phi_5715_ = u32();
                                                    phi_5716_ = vec3<f32>();
                                                }
                                                let _e5609 = phi_5714_;
                                                let _e5611 = phi_5715_;
                                                let _e5613 = phi_5716_;
                                                phi_17067_ = _e5576;
                                                phi_5718_ = _e5609;
                                                phi_5719_ = _e5611;
                                                phi_5720_ = _e5613;
                                                phi_5721_ = _e5580;
                                                phi_5722_ = select(true, false, _e5580);
                                            }
                                            let _e5616 = phi_17067_;
                                            let _e5618 = phi_5718_;
                                            let _e5620 = phi_5719_;
                                            let _e5622 = phi_5720_;
                                            let _e5624 = phi_5721_;
                                            let _e5626 = phi_5722_;
                                            phi_17066_ = _e5616;
                                            phi_5723_ = _e5618;
                                            phi_5724_ = _e5620;
                                            phi_5725_ = _e5622;
                                            phi_5726_ = _e5624;
                                            phi_5727_ = _e5497;
                                            phi_5728_ = _e5626;
                                        }
                                        let _e5628 = phi_17066_;
                                        let _e5630 = phi_5723_;
                                        let _e5632 = phi_5724_;
                                        let _e5634 = phi_5725_;
                                        let _e5636 = phi_5726_;
                                        let _e5638 = phi_5727_;
                                        let _e5640 = phi_5728_;
                                        phi_17065_ = _e5628;
                                        phi_5729_ = _e5630;
                                        phi_5730_ = _e5488.member_1;
                                        phi_5731_ = _e4987;
                                        phi_5732_ = _e5632;
                                        phi_5733_ = _e5634;
                                        phi_5734_ = _e5636;
                                        phi_5735_ = _e5496;
                                        phi_5736_ = _e5638;
                                        phi_5737_ = _e5640;
                                    }
                                    let _e5642 = phi_17065_;
                                    let _e5644 = phi_5729_;
                                    let _e5646 = phi_5730_;
                                    let _e5648 = phi_5731_;
                                    let _e5650 = phi_5732_;
                                    let _e5652 = phi_5733_;
                                    let _e5654 = phi_5734_;
                                    let _e5656 = phi_5735_;
                                    let _e5658 = phi_5736_;
                                    let _e5660 = phi_5737_;
                                    phi_17064_ = _e5642;
                                    phi_16970_ = _e4697;
                                    let _e7228 = local_54;
                                    phi_5738_ = _e7228;
                                    let _e7231 = local_55;
                                    phi_5739_ = _e7231;
                                    let _e7234 = local_56;
                                    phi_5744_ = _e7234;
                                    let _e7237 = local_57;
                                    phi_5745_ = _e7237;
                                    phi_5746_ = _e5644;
                                    phi_5747_ = _e5646;
                                    phi_5748_ = _e5648;
                                    phi_5749_ = _e5650;
                                    phi_5752_ = _e5652;
                                    phi_5755_ = _e5654;
                                    phi_5756_ = _e4698;
                                    phi_5757_ = _e5656;
                                    phi_5758_ = _e5658;
                                    phi_5760_ = _e5660;
                                } else {
                                    phi_17064_ = _e2043;
                                    phi_16970_ = _e690;
                                    phi_5738_ = u32();
                                    phi_5739_ = vec3<f32>();
                                    phi_5744_ = _e857;
                                    phi_5745_ = _e1052;
                                    phi_5746_ = _e2045;
                                    phi_5747_ = _e226;
                                    phi_5748_ = _e2047;
                                    phi_5749_ = _e2049;
                                    phi_5752_ = _e2051;
                                    phi_5755_ = _e2055;
                                    phi_5756_ = false;
                                    phi_5757_ = false;
                                    phi_5758_ = false;
                                    phi_5760_ = false;
                                }
                                let _e5662 = phi_17064_;
                                let _e5664 = phi_16970_;
                                let _e5666 = phi_5738_;
                                let _e5668 = phi_5739_;
                                let _e5670 = phi_5744_;
                                let _e5672 = phi_5745_;
                                let _e5674 = phi_5746_;
                                let _e5676 = phi_5747_;
                                let _e5678 = phi_5748_;
                                let _e5680 = phi_5749_;
                                let _e5682 = phi_5752_;
                                let _e5684 = phi_5755_;
                                let _e5686 = phi_5756_;
                                let _e5688 = phi_5757_;
                                let _e5690 = phi_5758_;
                                let _e5692 = phi_5760_;
                                phi_17063_ = _e5662;
                                phi_16969_ = _e5664;
                                phi_5762_ = _e5666;
                                phi_5763_ = _e5668;
                                phi_5768_ = _e5670;
                                phi_5769_ = _e5672;
                                phi_5770_ = _e5674;
                                phi_5771_ = _e5676;
                                phi_5772_ = _e5678;
                                phi_5773_ = _e5680;
                                phi_5774_ = select(_e232, true, _e2053);
                                phi_5775_ = select(true, false, _e2053);
                                phi_5776_ = _e5682;
                                phi_5779_ = _e5684;
                                phi_5780_ = _e5686;
                                phi_5781_ = _e5688;
                                phi_5782_ = _e5690;
                                phi_5783_ = select(_e2057, false, _e2053);
                                phi_5784_ = _e5692;
                                phi_5785_ = select(_e2059, false, _e2053);
                            }
                            let _e5698 = phi_17063_;
                            let _e5700 = phi_16969_;
                            let _e5702 = phi_5762_;
                            let _e5704 = phi_5763_;
                            let _e5706 = phi_5768_;
                            let _e5708 = phi_5769_;
                            let _e5710 = phi_5770_;
                            let _e5712 = phi_5771_;
                            let _e5714 = phi_5772_;
                            let _e5716 = phi_5773_;
                            let _e5718 = phi_5774_;
                            let _e5720 = phi_5775_;
                            let _e5722 = phi_5776_;
                            let _e5724 = phi_5779_;
                            let _e5726 = phi_5780_;
                            let _e5728 = phi_5781_;
                            let _e5730 = phi_5782_;
                            let _e5732 = phi_5783_;
                            let _e5734 = phi_5784_;
                            let _e5736 = phi_5785_;
                            phi_17062_ = _e5698;
                            phi_16968_ = _e5700;
                            phi_5786_ = _e1052;
                            phi_5787_ = _e1054;
                            phi_5788_ = _e1063;
                            phi_5789_ = _e1065;
                            phi_5790_ = _e1067;
                            phi_5791_ = (_e236 + vec3<f32>(_e1057, _e1059, _e1061));
                            phi_5792_ = _e1071;
                            let _e7279 = local_58;
                            phi_5793_ = _e7279;
                            phi_5794_ = _e5702;
                            phi_5795_ = _e5704;
                            phi_335_ = _e5706;
                            phi_338_ = _e5708;
                            phi_340_ = _e5710;
                            phi_342_ = _e5712;
                            phi_344_ = _e5714;
                            phi_346_ = _e5716;
                            phi_348_ = _e5718;
                            phi_350_ = _e5720;
                            phi_352_ = _e5722;
                            phi_5804_ = _e5724;
                            phi_5805_ = _e1136;
                            phi_5806_ = _e5726;
                            phi_5807_ = _e5728;
                            phi_5808_ = _e5730;
                            phi_5809_ = _e5732;
                            phi_5810_ = _e5734;
                            phi_5811_ = _e5736;
                        }
                        let _e5738 = phi_17062_;
                        let _e5740 = phi_16968_;
                        let _e5742 = phi_5786_;
                        let _e5744 = phi_5787_;
                        let _e5746 = phi_5788_;
                        let _e5748 = phi_5789_;
                        let _e5750 = phi_5790_;
                        let _e5752 = phi_5791_;
                        let _e5754 = phi_5792_;
                        let _e5756 = phi_5793_;
                        let _e5758 = phi_5794_;
                        let _e5760 = phi_5795_;
                        let _e5762 = phi_335_;
                        let _e5764 = phi_338_;
                        let _e5766 = phi_340_;
                        let _e5768 = phi_342_;
                        let _e5770 = phi_344_;
                        let _e5772 = phi_346_;
                        let _e5774 = phi_348_;
                        let _e5776 = phi_350_;
                        let _e5778 = phi_352_;
                        let _e5780 = phi_5804_;
                        let _e5782 = phi_5805_;
                        let _e5784 = phi_5806_;
                        let _e5786 = phi_5807_;
                        let _e5788 = phi_5808_;
                        let _e5790 = phi_5809_;
                        let _e5792 = phi_5810_;
                        let _e5794 = phi_5811_;
                        phi_13272_ = _e5738;
                        phi_13192_ = _e5740;
                        phi_334_ = _e5762;
                        phi_337_ = _e5764;
                        phi_339_ = _e5766;
                        phi_341_ = _e5768;
                        phi_343_ = _e5770;
                        phi_345_ = _e5772;
                        phi_347_ = _e5774;
                        phi_349_ = _e5776;
                        phi_351_ = _e5778;
                        phi_16967_ = _e5740;
                        phi_7779_ = _e859;
                        phi_7778_ = _e857;
                        phi_7777_ = _e856;
                        phi_7776_ = _e5794;
                        phi_7775_ = _e5792;
                        phi_7774_ = _e5790;
                        phi_7773_ = _e5788;
                        phi_7772_ = _e5786;
                        phi_7771_ = _e5784;
                        phi_7770_ = _e5782;
                        phi_7769_ = _e5742;
                        phi_7767_ = _e5760;
                        phi_7766_ = _e5758;
                        phi_7759_ = _e5756;
                        phi_7757_ = _e5754;
                        phi_7755_ = _e5752;
                        phi_7754_ = _e5750;
                        phi_7753_ = _e5748;
                        phi_7752_ = _e5746;
                        phi_7750_ = _e5744;
                        if !(_e5780) {
                            break;
                        }
                    }
                    let _e5797 = phi_16967_;
                    let _e5799 = phi_7779_;
                    let _e5801 = phi_7778_;
                    let _e5803 = phi_7777_;
                    let _e5805 = phi_7776_;
                    let _e5807 = phi_7775_;
                    let _e5809 = phi_7774_;
                    let _e5811 = phi_7773_;
                    let _e5813 = phi_7772_;
                    let _e5815 = phi_7771_;
                    let _e5817 = phi_7770_;
                    let _e5819 = phi_7769_;
                    let _e5821 = phi_7767_;
                    let _e5823 = phi_7766_;
                    let _e5825 = phi_7759_;
                    let _e5827 = phi_7757_;
                    let _e5829 = phi_7755_;
                    let _e5831 = phi_7754_;
                    let _e5833 = phi_7753_;
                    let _e5835 = phi_7752_;
                    let _e5837 = phi_7750_;
                    if _e5797 {
                        break;
                    }
                    if _e5799 {
                        let _e5839 = local_19;
                        if _e5839 {
                            phi_5970_ = 1f;
                        } else {
                            if ((_e211 & 2u) == 0u) {
                                phi_5969_ = 1f;
                            } else {
                                let _e5844 = global_1.member.member_3[1u];
                                let _e5847 = global_1.member.member_3[2u];
                                let _e5849 = local_20;
                                let _e5852 = local_21;
                                let _e5856 = ((atan2(_e5849.z, _e5852.x) * 0.15915494f) + 0.5f);
                                let _e5860 = local_22;
                                let _e5863 = select(_e5860.y, -1f, (_e5860.y < -1f));
                                let _e5866 = acos(select(_e5863, 1f, (_e5863 > 1f)));
                                let _e5868 = f32(_e5844);
                                let _e5869 = ((_e5856 - floor(_e5856)) * _e5868);
                                let _e5874 = select(select(u32(_e5869), 0u, (_e5869 < 0f)), 4294967295u, (_e5869 > 4294967000f));
                                let _e5875 = (_e5844 - 1u);
                                let _e5878 = f32(_e5847);
                                let _e5879 = ((_e5866 * 0.31830987f) * _e5878);
                                let _e5884 = select(select(u32(_e5879), 0u, (_e5879 < 0f)), 4294967295u, (_e5879 > 4294967000f));
                                let _e5885 = (_e5847 - 1u);
                                let _e5887 = select(_e5884, _e5885, (_e5885 < _e5884));
                                let _e5888 = sin(_e5866);
                                if (_e5888 <= 0f) {
                                    phi_5967_ = 0f;
                                } else {
                                    let _e5892 = global_1.member.member_2[3u];
                                    let _e5895 = global_1.member.member_3[0u];
                                    let _e5899 = (_e5892 + _e5887);
                                    let _e5902 = global_6.member[(_e5899 + 1u)];
                                    let _e5904 = global_6.member[_e5899];
                                    let _e5907 = ((_e5895 + (_e5887 * (_e5844 + 1u))) + select(_e5874, _e5875, (_e5875 < _e5874)));
                                    let _e5910 = global_6.member[(_e5907 + 1u)];
                                    let _e5912 = global_6.member[_e5907];
                                    phi_5967_ = ((((_e5902 - _e5904) * _e5878) * ((_e5910 - _e5912) * _e5868)) / (19.73921f * _e5888));
                                }
                                let _e5919 = phi_5967_;
                                let _e5921 = local_23;
                                if (_e5921 >= 300000000000000000000000000000000000000f) {
                                    phi_12952_ = 1f;
                                } else {
                                    let _e5924 = local_24;
                                    let _e5926 = local_25;
                                    let _e5927 = (_e5924 * _e5926);
                                    let _e5929 = (_e5927 + (_e5919 * _e5919));
                                    if (_e5929 == 0f) {
                                        phi_12949_ = 0f;
                                    } else {
                                        phi_12949_ = (_e5927 / _e5929);
                                    }
                                    let _e5933 = phi_12949_;
                                    phi_12952_ = _e5933;
                                }
                                let _e5935 = phi_12952_;
                                phi_5969_ = _e5935;
                            }
                            let _e5937 = phi_5969_;
                            phi_5970_ = _e5937;
                        }
                        let _e5939 = phi_5970_;
                        let _e5942 = global_1.member.member_3[1u];
                        let _e5945 = global_1.member.member_3[2u];
                        let _e5947 = local_26;
                        let _e5950 = local_27;
                        let _e5954 = ((atan2(_e5947.z, _e5950.x) * 0.15915494f) + 0.5f);
                        let _e5959 = (((_e5954 - floor(_e5954)) * f32(_e5942)) - 0.5f);
                        let _e5961 = local_28;
                        let _e5964 = select(_e5961.y, -1f, (_e5961.y < -1f));
                        let _e5971 = (((acos(select(_e5964, 1f, (_e5964 > 1f))) * 0.31830987f) * f32(_e5945)) - 0.5f);
                        let _e5972 = (_e5945 - 1u);
                        let _e5973 = f32(_e5972);
                        if (0f <= _e5973) {
                        } else {
                            break;
                        }
                        let _e5976 = select(_e5971, 0f, (_e5971 < 0f));
                        let _e5978 = select(_e5976, _e5973, (_e5976 > _e5973));
                        let _e5979 = floor(_e5959);
                        let _e5980 = floor(_e5978);
                        let _e5981 = (_e5959 - _e5979);
                        let _e5982 = (_e5978 - _e5980);
                        let _e5993 = (_e5942 == 0u);
                        if _e5993 {
                            break;
                        }
                        let _e5994 = (bitcast<u32>((select(0i, select(select(i32(_e5979), i32(-2147483648), (_e5979 < -2147483600f)), 2147483647i, (_e5979 > 2147483500f)), (_e5979 == _e5979)) + bitcast<i32>(_e5942))) % _e5942);
                        let _e5999 = select(select(u32(_e5980), 0u, (_e5980 < 0f)), 4294967295u, (_e5980 > 4294967000f));
                        let _e6000 = (_e5999 + 1u);
                        let _e6005 = global_1.member.member_2[2u];
                        let _e6006 = (_e5999 * _e5942);
                        if _e5993 {
                            break;
                        }
                        let _e6007 = (_e5994 % _e5942);
                        let _e6010 = (_e6005 + (3u * (_e6006 + _e6007)));
                        let _e6012 = global_6.member[_e6010];
                        let _e6015 = global_6.member[(_e6010 + 1u)];
                        let _e6018 = global_6.member[(_e6010 + 2u)];
                        if _e5993 {
                            break;
                        }
                        let _e6021 = ((_e5994 + 1u) % _e5942);
                        let _e6024 = (_e6005 + (3u * (_e6006 + _e6021)));
                        let _e6026 = global_6.member[_e6024];
                        let _e6029 = global_6.member[(_e6024 + 1u)];
                        let _e6032 = global_6.member[(_e6024 + 2u)];
                        let _e6040 = (vec3<f32>(_e6012, _e6015, _e6018) + vec3<f32>(((_e6026 - _e6012) * _e5981), ((_e6029 - _e6015) * _e5981), ((_e6032 - _e6018) * _e5981)));
                        let _e6041 = (select(_e6000, _e5972, (_e5972 < _e6000)) * _e5942);
                        if _e5993 {
                            break;
                        }
                        let _e6044 = (_e6005 + (3u * (_e6041 + _e6007)));
                        let _e6046 = global_6.member[_e6044];
                        let _e6049 = global_6.member[(_e6044 + 1u)];
                        let _e6052 = global_6.member[(_e6044 + 2u)];
                        if _e5993 {
                            break;
                        }
                        let _e6056 = (_e6005 + (3u * (_e6041 + _e6021)));
                        let _e6058 = global_6.member[_e6056];
                        let _e6061 = global_6.member[(_e6056 + 1u)];
                        let _e6064 = global_6.member[(_e6056 + 2u)];
                        let _e6072 = (vec3<f32>(_e6046, _e6049, _e6052) + vec3<f32>(((_e6058 - _e6046) * _e5981), ((_e6061 - _e6049) * _e5981), ((_e6064 - _e6052) * _e5981)));
                        let _e6086 = (_e6040 + vec3<f32>(((_e6072.x - _e6040.x) * _e5982), ((_e6072.y - _e6040.y) * _e5982), ((_e6072.z - _e6040.z) * _e5982)));
                        let _e6088 = local_29;
                        let _e6093 = local_30;
                        let _e6098 = local_31;
                        let _e6106 = local_32;
                        let _e6108 = (_e6106.x + ((_e6088.x * _e6086.x) * _e5939));
                        let _e6110 = local_33;
                        let _e6112 = (_e6110.y + ((_e6093.y * _e6086.y) * _e5939));
                        let _e6114 = local_34;
                        let _e6116 = (_e6114.z + ((_e6098.z * _e6086.z) * _e5939));
                        if ((_e211 & 4u) == 0u) {
                            phi_6422_ = vec3<f32>(_e6108, _e6112, _e6116);
                        } else {
                            let _e6121 = local_35;
                            if _e6121 {
                                phi_6350_ = 1f;
                            } else {
                                let _e6123 = global_1.member.member_5;
                                let _e6134 = global_1.member.member_5[3u];
                                if ((((_e5950.x * _e6123.x) + (_e5961.y * _e6123.y)) + (_e5947.z * _e6123.z)) <= _e6134) {
                                    phi_6348_ = 0f;
                                } else {
                                    phi_6348_ = (0.15915494f / (1f - _e6134));
                                }
                                let _e6139 = phi_6348_;
                                let _e6141 = local_36;
                                if (_e6141 >= 300000000000000000000000000000000000000f) {
                                    phi_13021_ = 1f;
                                } else {
                                    let _e6144 = local_37;
                                    let _e6146 = local_38;
                                    let _e6147 = (_e6144 * _e6146);
                                    let _e6149 = (_e6147 + (_e6139 * _e6139));
                                    if (_e6149 == 0f) {
                                        phi_13018_ = 0f;
                                    } else {
                                        phi_13018_ = (_e6147 / _e6149);
                                    }
                                    let _e6153 = phi_13018_;
                                    phi_13021_ = _e6153;
                                }
                                let _e6155 = phi_13021_;
                                phi_6350_ = _e6155;
                            }
                            let _e6157 = phi_6350_;
                            let _e6159 = global_1.member.member_5;
                            let _e6167 = (((_e5950.x * _e6159.x) + (_e5961.y * _e6159.y)) + (_e5947.z * _e6159.z));
                            let _e6170 = global_1.member.member_5[3u];
                            if (_e6167 <= _e6170) {
                                phi_6405_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                if (_e6167 != _e6167) {
                                    phi_13032_ = true;
                                } else {
                                    phi_13032_ = (1f <= _e6167);
                                }
                                let _e6175 = phi_13032_;
                                let _e6181 = global_1.member.member_6[3u];
                                let _e6184 = (1f - pow((acos(select(_e6167, 1f, _e6175)) / _e6181), 2f));
                                if (_e6184 != _e6184) {
                                    phi_13047_ = true;
                                } else {
                                    phi_13047_ = (0f >= _e6184);
                                }
                                let _e6188 = phi_13047_;
                                let _e6193 = global_1.member.member_6;
                                let _e6201 = (((1f - (0.6f * (1f - sqrt(select(_e6184, 0f, _e6188))))) * 1.25f) / (6.2831855f * (1f - _e6170)));
                                phi_6405_ = vec3<f32>((_e6193.x * _e6201), (_e6193.y * _e6201), (_e6193.z * _e6201));
                            }
                            let _e6207 = phi_6405_;
                            phi_6422_ = vec3<f32>((_e6108 + ((_e6088.x * _e6207.x) * _e6157)), (_e6112 + ((_e6093.y * _e6207.y) * _e6157)), (_e6116 + ((_e6098.z * _e6207.z) * _e6157)));
                        }
                        let _e6222 = phi_6422_;
                        phi_6423_ = _e6222;
                    } else {
                        phi_6423_ = vec3<f32>();
                    }
                    let _e6224 = phi_6423_;
                    let _e6225 = select(_e5817, false, _e5799);
                    if _e6225 {
                        let _e6234 = local_39;
                        let _e6238 = local_40;
                        let _e6242 = local_41;
                        let _e6246 = global_4.member[_e5825];
                        if (_e6246.member == 4u) {
                            if (_e6246.member_1 == 1u) {
                                phi_6475_ = true;
                            } else {
                                phi_6475_ = ((((-(_e6234.x) * _e5803.member_3.x) + (-(_e6238.y) * _e5803.member_3.y)) + (-(_e6242.z) * _e5803.member_3.z)) > 0f);
                            }
                            let _e6261 = phi_6475_;
                            if _e6261 {
                                phi_6484_ = vec3<f32>(_e6246.member_4.x, _e6246.member_4.y, _e6246.member_4.z);
                            } else {
                                phi_6484_ = vec3<f32>();
                            }
                            let _e6268 = phi_6484_;
                            phi_6486_ = _e6268;
                            phi_6487_ = select(true, false, _e6261);
                        } else {
                            phi_6486_ = vec3<f32>();
                            phi_6487_ = true;
                        }
                        let _e6271 = phi_6486_;
                        let _e6273 = phi_6487_;
                        let _e6275 = select(_e6271, vec3<f32>(0f, 0f, 0f), vec3(_e6273));
                        let _e6277 = local_42;
                        if _e6277 {
                            phi_6660_ = 1f;
                        } else {
                            if ((_e211 & 1u) == 0u) {
                                phi_6659_ = 1f;
                            } else {
                                if (_e5827.member_6.z > 0u) {
                                    let _e6286 = global_1.member.member_2[1u];
                                    let _e6287 = (_e6286 + (_e5827.member_6.z - 1u));
                                    let _e6290 = global_6.member[(_e6287 + 1u)];
                                    let _e6292 = global_6.member[_e6287];
                                    let _e6297 = global_2.member[_e5803.member_1];
                                    if (_e6297.member_6.x == 1u) {
                                        let _e6382 = local_46;
                                        let _e6384 = (_e6297.member.x - _e6382.x);
                                        let _e6386 = local_47;
                                        let _e6388 = (_e6297.member.y - _e6386.y);
                                        let _e6390 = local_48;
                                        let _e6392 = (_e6297.member.z - _e6390.z);
                                        let _e6397 = (((_e6384 * _e6384) + (_e6388 * _e6388)) + (_e6392 * _e6392));
                                        let _e6398 = (_e6297.member.w * _e6297.member.w);
                                        if (_e6397 <= (_e6398 * 1.000001f)) {
                                            let _e6413 = (_e5829.x - _e6382.x);
                                            let _e6415 = (_e5829.y - _e6386.y);
                                            let _e6417 = (_e5829.z - _e6390.z);
                                            let _e6422 = (((_e6413 * _e6413) + (_e6415 * _e6415)) + (_e6417 * _e6417));
                                            phi_13116_ = (_e6422 / (((abs((((_e5803.member_2.x * _e6413) + (_e5803.member_2.y * _e6415)) + (_e5803.member_2.z * _e6417))) / sqrt(_e6422)) * 12.566371f) * _e6398));
                                        } else {
                                            let _e6401 = (_e6398 / _e6397);
                                            if (_e6401 != _e6401) {
                                                phi_13135_ = true;
                                            } else {
                                                phi_13135_ = (1f <= _e6401);
                                            }
                                            let _e6405 = phi_13135_;
                                            let _e6406 = select(_e6401, 1f, _e6405);
                                            phi_13116_ = (0.15915494f / (_e6406 / (1f + sqrt((1f - _e6406)))));
                                        }
                                        let _e6438 = phi_13116_;
                                        phi_6655_ = _e6438;
                                    } else {
                                        let _e6302 = local_43;
                                        let _e6304 = (_e5835 - _e6302.x);
                                        let _e6306 = local_44;
                                        let _e6308 = (_e5833 - _e6306.y);
                                        let _e6310 = local_45;
                                        let _e6312 = (_e5831 - _e6310.z);
                                        let _e6317 = (((_e6304 * _e6304) + (_e6308 * _e6308)) + (_e6312 * _e6312));
                                        let _e6328 = (abs((((_e5803.member_2.x * _e6304) + (_e5803.member_2.y * _e6308)) + (_e5803.member_2.z * _e6312))) / sqrt(_e6317));
                                        if (_e6328 <= 0f) {
                                            phi_6654_ = 0f;
                                        } else {
                                            if (_e6297.member_6.x == 0u) {
                                                let _e6343 = (_e6297.member_1.x - _e6297.member.x);
                                                let _e6344 = (_e6297.member_1.y - _e6297.member.y);
                                                let _e6345 = (_e6297.member_1.z - _e6297.member.z);
                                                let _e6350 = (_e6297.member_2.x - _e6297.member.x);
                                                let _e6351 = (_e6297.member_2.y - _e6297.member.y);
                                                let _e6352 = (_e6297.member_2.z - _e6297.member.z);
                                                let _e6355 = ((_e6344 * _e6352) - (_e6351 * _e6345));
                                                let _e6358 = ((_e6345 * _e6350) - (_e6352 * _e6343));
                                                let _e6361 = ((_e6343 * _e6351) - (_e6350 * _e6344));
                                                phi_6651_ = (sqrt((((_e6355 * _e6355) + (_e6358 * _e6358)) + (_e6361 * _e6361))) * 0.5f);
                                            } else {
                                                phi_6651_ = ((12.566371f * _e6297.member.w) * _e6297.member.w);
                                            }
                                            let _e6370 = phi_6651_;
                                            phi_6654_ = (_e6317 / (_e6328 * _e6370));
                                        }
                                        let _e6374 = phi_6654_;
                                        phi_6655_ = _e6374;
                                    }
                                    let _e6440 = phi_6655_;
                                    let _e6441 = ((_e6290 - _e6292) * _e6440);
                                    let _e6443 = local_49;
                                    if (_e6443 >= 300000000000000000000000000000000000000f) {
                                        phi_13156_ = 1f;
                                    } else {
                                        let _e6446 = local_50;
                                        let _e6448 = local_51;
                                        let _e6449 = (_e6446 * _e6448);
                                        let _e6451 = (_e6449 + (_e6441 * _e6441));
                                        if (_e6451 == 0f) {
                                            phi_13153_ = 0f;
                                        } else {
                                            phi_13153_ = (_e6449 / _e6451);
                                        }
                                        let _e6455 = phi_13153_;
                                        phi_13156_ = _e6455;
                                    }
                                    let _e6457 = phi_13156_;
                                    phi_6658_ = _e6457;
                                } else {
                                    phi_6658_ = 1f;
                                }
                                let _e6459 = phi_6658_;
                                phi_6659_ = _e6459;
                            }
                            let _e6461 = phi_6659_;
                            phi_6660_ = _e6461;
                        }
                        let _e6463 = phi_6660_;
                        phi_6682_ = vec3<f32>((_e5819.x + ((_e5837.x * _e6275.x) * _e6463)), (_e5819.y + ((_e5837.y * _e6275.y) * _e6463)), (_e5819.z + ((_e5837.z * _e6275.z) * _e6463)));
                    } else {
                        phi_6682_ = vec3<f32>();
                    }
                    let _e6484 = phi_6682_;
                    let _e6485 = select(select(_e5815, false, _e5799), false, _e6225);
                    let _e6491 = select(select(select(_e5813, false, _e5799), false, _e6225), false, _e6485);
                    let _e6497 = select(select(select(select(_e5811, false, _e5799), false, _e6225), false, _e6485), false, _e6491);
                    let _e6503 = select(_e6491, true, _e6497);
                    let _e6509 = select(select(select(select(select(select(_e5809, false, _e5799), false, _e6225), false, _e6485), false, _e6491), false, _e6497), false, _e6503);
                    let _e6514 = select(select(select(select(select(select(select(_e5807, false, _e5799), false, _e6225), false, _e6485), false, _e6491), false, _e6497), false, _e6503), false, _e6509);
                    let _e6519 = select(select(_e6503, false, _e6509), true, _e6514);
                    let _e6524 = select(select(select(select(select(select(_e6485, false, _e6491), false, _e6497), false, _e6503), false, _e6509), false, _e6514), true, _e6519);
                    let _e6528 = select(select(select(select(select(select(select(select(select(select(_e5805, false, _e5799), false, _e6225), false, _e6485), false, _e6491), false, _e6497), false, _e6503), false, _e6509), false, _e6514), false, _e6519), false, _e6524);
                    let _e6531 = select(select(select(select(_e6509, false, _e6514), false, _e6519), false, _e6524), true, _e6528);
                    let _e6537 = select(select(_e6524, false, _e6528), true, _e6531);
                    let _e6542 = select(select(select(select(select(select(select(select(select(select(select(_e6225, false, _e6485), false, _e6491), false, _e6497), false, _e6503), false, _e6509), false, _e6514), false, _e6519), false, _e6524), false, _e6528), false, _e6531), true, _e6537);
                    let _e6545 = select(_e6224, select(_e6484, select(_e5821, _e5819, vec3(_e6531)), vec3(_e6537)), vec3(_e6542));
                    let _e6548 = (_e6545.x * _e144.w);
                    let _e6550 = (_e6545.y * _e144.w);
                    let _e6552 = (_e6545.z * _e144.w);
                    if (_e6548 == _e6548) {
                        let _e6555 = (_e6550 == _e6550);
                        if _e6555 {
                            phi_6806_ = select(true, false, (_e6552 == _e6552));
                        } else {
                            phi_6806_ = false;
                        }
                        let _e6559 = phi_6806_;
                        phi_6807_ = select(true, false, _e6555);
                        phi_6808_ = _e6559;
                    } else {
                        phi_6807_ = true;
                        phi_6808_ = false;
                    }
                    let _e6562 = phi_6807_;
                    let _e6564 = phi_6808_;
                    let _e6567 = select((_e6545 * _e144.w), vec3<f32>(0f, 0f, 0f), vec3(select(_e6564, true, _e6562)));
                    if (_e129 < arrayLength((&global_8.member))) {
                    } else {
                        break;
                    }
                    let _e6576 = global_8.member[_e129][0u];
                    global_8.member[_e129][0u] = (_e6576 + _e6567.x);
                    let _e6581 = global_8.member[_e129][1u];
                    global_8.member[_e129][1u] = (_e6581 + _e6567.y);
                    let _e6586 = global_8.member[_e129][2u];
                    global_8.member[_e129][2u] = (_e6586 + _e6567.z);
                    let _e6591 = global_8.member[_e129][3u];
                    global_8.member[_e129][3u] = (_e6591 + f32(select(_e5801, select(_e5801, select(_e5823, _e5801, _e6531), _e6537), _e6542)));
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(64, 1, 1) 
fn main(@builtin(global_invocation_id) param: vec3<u32>) {
    global = param;
    function_();
}
