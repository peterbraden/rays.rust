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
    var phi_13221_: u32;
    var phi_13141_: bool;
    var phi_334_: vec3<f32>;
    var phi_337_: vec3<f32>;
    var phi_339_: f32;
    var phi_341_: vec3<f32>;
    var phi_343_: u32;
    var phi_345_: bool;
    var phi_347_: bool;
    var phi_349_: vec3<f32>;
    var phi_370_: u32;
    var phi_373_: u32;
    var phi_375_: u32;
    var phi_377_: vec4<f32>;
    var phi_379_: type_49;
    var phi_7857_: bool;
    var phi_7887_: bool;
    var phi_7834_: type_52;
    var phi_7837_: type_52;
    var phi_7838_: bool;
    var phi_7843_: type_52;
    var phi_7944_: bool;
    var phi_7959_: bool;
    var phi_7974_: bool;
    var phi_7989_: bool;
    var phi_7921_: type_52;
    var phi_7924_: type_52;
    var phi_7925_: bool;
    var phi_7930_: type_52;
    var phi_8031_: bool;
    var phi_8046_: bool;
    var phi_8061_: bool;
    var phi_8076_: bool;
    var phi_8008_: type_52;
    var phi_8011_: type_52;
    var phi_8012_: bool;
    var phi_8017_: type_52;
    var phi_7809_: bool;
    var phi_7812_: bool;
    var phi_7813_: bool;
    var phi_793_: u32;
    var phi_794_: u32;
    var phi_795_: u32;
    var phi_796_: u32;
    var phi_797_: u32;
    var phi_798_: u32;
    var phi_799_: u32;
    var phi_800_: u32;
    var phi_801_: u32;
    var phi_682_: vec4<f32>;
    var phi_683_: vec4<f32>;
    var phi_684_: bool;
    var phi_688_: vec4<f32>;
    var phi_689_: bool;
    var phi_593_: bool;
    var phi_630_: bool;
    var phi_636_: f32;
    var phi_646_: vec4<f32>;
    var phi_647_: vec4<f32>;
    var phi_648_: bool;
    var phi_652_: vec4<f32>;
    var phi_653_: bool;
    var phi_657_: vec4<f32>;
    var phi_658_: bool;
    var phi_690_: vec4<f32>;
    var phi_691_: bool;
    var phi_519_: vec4<f32>;
    var phi_520_: vec4<f32>;
    var phi_521_: bool;
    var phi_525_: vec4<f32>;
    var phi_526_: bool;
    var phi_527_: vec4<f32>;
    var phi_528_: bool;
    var phi_529_: bool;
    var phi_537_: vec4<f32>;
    var phi_538_: bool;
    var phi_539_: vec4<f32>;
    var phi_540_: bool;
    var phi_541_: bool;
    var phi_549_: vec4<f32>;
    var phi_550_: bool;
    var phi_700_: vec4<f32>;
    var phi_701_: bool;
    var phi_722_: type_49;
    var phi_724_: type_49;
    var phi_371_: u32;
    var phi_374_: u32;
    var phi_376_: u32;
    var phi_378_: vec4<f32>;
    var phi_380_: type_49;
    var phi_803_: type_49;
    var phi_804_: bool;
    var phi_805_: bool;
    var phi_13126_: bool;
    var phi_7699_: bool;
    var phi_7697_: type_49;
    var local_2: type_49;
    var local_3: vec4<f32>;
    var phi_973_: type_53;
    var local_4: vec4<f32>;
    var local_5: vec4<f32>;
    var phi_932_: vec3<f32>;
    var phi_933_: vec3<f32>;
    var phi_974_: type_53;
    var local_6: type_49;
    var phi_979_: type_49;
    var phi_980_: type_49;
    var phi_985_: type_49;
    var phi_1132_: vec3<f32>;
    var phi_1135_: vec3<f32>;
    var phi_1137_: u32;
    var phi_1155_: f32;
    var phi_1161_: f32;
    var phi_1168_: f32;
    var phi_1133_: vec3<f32>;
    var phi_1136_: vec3<f32>;
    var phi_1138_: u32;
    var local_7: vec3<f32>;
    var local_8: vec3<f32>;
    var local_9: vec3<f32>;
    var local_10: vec3<f32>;
    var phi_1314_: vec3<f32>;
    var phi_1315_: vec3<f32>;
    var phi_1316_: vec3<f32>;
    var phi_1317_: vec3<f32>;
    var phi_1348_: i32;
    var phi_1351_: u32;
    var phi_1405_: bool;
    var phi_1411_: bool;
    var phi_1418_: u32;
    var phi_1420_: i32;
    var phi_1421_: u32;
    var phi_1349_: i32;
    var phi_1352_: u32;
    var phi_1423_: bool;
    var local_11: u32;
    var phi_1468_: vec3<f32>;
    var phi_1485_: vec3<f32>;
    var phi_8105_: bool;
    var phi_1516_: f32;
    var phi_1628_: vec3<f32>;
    var phi_1634_: f32;
    var phi_8178_: bool;
    var phi_8167_: f32;
    var phi_1724_: vec3<f32>;
    var phi_1725_: vec3<f32>;
    var phi_1726_: vec3<f32>;
    var phi_13347_: u32;
    var phi_1728_: vec3<f32>;
    var phi_1729_: vec3<f32>;
    var phi_1730_: vec3<f32>;
    var phi_1731_: bool;
    var phi_1746_: vec3<f32>;
    var phi_1748_: vec3<f32>;
    var phi_13345_: u32;
    var phi_1749_: vec3<f32>;
    var phi_1750_: vec3<f32>;
    var phi_1751_: vec3<f32>;
    var phi_13344_: u32;
    var phi_1752_: vec3<f32>;
    var phi_1753_: vec3<f32>;
    var phi_1754_: vec3<f32>;
    var phi_1755_: bool;
    var phi_13343_: u32;
    var phi_1756_: vec3<f32>;
    var phi_1757_: vec3<f32>;
    var phi_1758_: vec3<f32>;
    var phi_1759_: bool;
    var phi_1760_: bool;
    var phi_8251_: bool;
    var phi_8413_: bool;
    var phi_8428_: bool;
    var phi_8443_: bool;
    var phi_8470_: bool;
    var phi_8485_: bool;
    var phi_8500_: bool;
    var phi_8527_: bool;
    var phi_8542_: bool;
    var phi_8557_: bool;
    var phi_1939_: f32;
    var phi_1947_: f32;
    var phi_1955_: f32;
    var phi_8754_: bool;
    var phi_8743_: f32;
    var phi_2062_: vec3<f32>;
    var phi_8782_: bool;
    var phi_2063_: vec3<f32>;
    var phi_2064_: vec3<f32>;
    var phi_2065_: vec3<f32>;
    var phi_13726_: vec3<f32>;
    var phi_13650_: vec3<f32>;
    var phi_13572_: u32;
    var phi_13491_: f32;
    var phi_2066_: vec3<f32>;
    var phi_2067_: vec3<f32>;
    var phi_2069_: bool;
    var phi_8797_: bool;
    var phi_8848_: bool;
    var phi_8837_: f32;
    var phi_13698_: vec3<f32>;
    var phi_1589_: vec3<f32>;
    var phi_14047_: vec3<f32>;
    var phi_13842_: f32;
    var phi_13697_: vec3<f32>;
    var phi_13389_: u32;
    var phi_1591_: vec3<f32>;
    var phi_1592_: bool;
    var phi_13993_: vec3<f32>;
    var phi_13802_: f32;
    var phi_13724_: vec3<f32>;
    var phi_13648_: vec3<f32>;
    var phi_13570_: u32;
    var phi_13489_: f32;
    var phi_13413_: u32;
    var phi_13340_: u32;
    var phi_2077_: vec3<f32>;
    var phi_2078_: vec3<f32>;
    var phi_2079_: vec3<f32>;
    var phi_2080_: bool;
    var phi_8914_: bool;
    var phi_2149_: vec3<f32>;
    var phi_13402_: u32;
    var phi_2151_: vec3<f32>;
    var phi_2152_: bool;
    var phi_13401_: u32;
    var phi_2153_: vec3<f32>;
    var phi_2154_: bool;
    var phi_13399_: u32;
    var phi_2160_: vec3<f32>;
    var phi_2161_: bool;
    var phi_2191_: vec3<f32>;
    var phi_2192_: vec3<f32>;
    var phi_2193_: u32;
    var phi_13397_: u32;
    var phi_2195_: vec3<f32>;
    var phi_2196_: vec3<f32>;
    var phi_2197_: u32;
    var phi_2198_: bool;
    var phi_2199_: bool;
    var phi_13396_: u32;
    var phi_2200_: vec3<f32>;
    var phi_2201_: vec3<f32>;
    var phi_2202_: u32;
    var phi_2204_: bool;
    var phi_2205_: bool;
    var phi_2206_: bool;
    var phi_13990_: vec3<f32>;
    var phi_13799_: f32;
    var phi_13721_: vec3<f32>;
    var phi_13645_: vec3<f32>;
    var phi_13567_: u32;
    var phi_13486_: f32;
    var phi_13410_: u32;
    var phi_13395_: u32;
    var phi_2207_: vec3<f32>;
    var phi_2208_: vec3<f32>;
    var phi_2209_: u32;
    var phi_2210_: vec3<f32>;
    var phi_2211_: bool;
    var phi_2212_: bool;
    var phi_2213_: bool;
    var phi_2214_: bool;
    var phi_15151_: bool;
    var phi_13394_: u32;
    var phi_2223_: vec3<f32>;
    var phi_2226_: u32;
    var phi_3433_: u32;
    var phi_3436_: u32;
    var phi_3434_: u32;
    var phi_3437_: u32;
    var local_12: u32;
    var phi_3474_: u32;
    var phi_3477_: u32;
    var phi_3475_: u32;
    var phi_3478_: u32;
    var local_13: u32;
    var local_14: u32;
    var phi_3554_: f32;
    var local_15: u32;
    var local_16: u32;
    var phi_3603_: f32;
    var phi_3759_: f32;
    var phi_4080_: vec3<f32>;
    var phi_4082_: vec3<f32>;
    var phi_15493_: u32;
    var phi_4083_: vec3<f32>;
    var phi_4084_: f32;
    var phi_4085_: vec3<f32>;
    var phi_2906_: u32;
    var phi_2909_: u32;
    var phi_2907_: u32;
    var phi_2910_: u32;
    var local_17: u32;
    var local_18: u32;
    var phi_3302_: f32;
    var phi_3305_: f32;
    var phi_9184_: bool;
    var phi_9165_: f32;
    var phi_3306_: f32;
    var phi_9199_: bool;
    var phi_9222_: bool;
    var phi_9237_: bool;
    var phi_3057_: f32;
    var phi_9252_: bool;
    var phi_9267_: bool;
    var phi_9282_: bool;
    var phi_9347_: bool;
    var phi_9435_: bool;
    var phi_9416_: f32;
    var phi_3122_: type_60;
    var phi_3308_: type_60;
    var phi_3386_: bool;
    var phi_3395_: vec3<f32>;
    var phi_3397_: vec3<f32>;
    var phi_3398_: bool;
    var phi_3403_: vec3<f32>;
    var phi_3404_: f32;
    var phi_15496_: u32;
    var phi_3405_: vec3<f32>;
    var phi_3406_: f32;
    var phi_3407_: vec3<f32>;
    var phi_3408_: f32;
    var phi_15495_: u32;
    var phi_3409_: vec3<f32>;
    var phi_3410_: f32;
    var phi_3411_: vec3<f32>;
    var phi_3412_: f32;
    var phi_15492_: u32;
    var phi_4086_: vec3<f32>;
    var phi_4087_: f32;
    var phi_4088_: vec3<f32>;
    var phi_4089_: f32;
    var phi_9450_: bool;
    var phi_9560_: f32;
    var phi_9588_: f32;
    var phi_9616_: f32;
    var phi_9750_: bool;
    var phi_9765_: bool;
    var phi_9780_: bool;
    var phi_9807_: bool;
    var phi_9822_: bool;
    var phi_9837_: bool;
    var phi_9864_: bool;
    var phi_9879_: bool;
    var phi_9894_: bool;
    var phi_2558_: f32;
    var phi_2566_: f32;
    var phi_2574_: f32;
    var phi_10091_: bool;
    var phi_10080_: f32;
    var phi_2681_: vec3<f32>;
    var phi_10119_: bool;
    var phi_2682_: vec3<f32>;
    var phi_2694_: type_59;
    var phi_2695_: type_59;
    var phi_2696_: bool;
    var phi_2701_: type_59;
    var phi_10156_: bool;
    var phi_10186_: bool;
    var phi_10175_: f32;
    var phi_10225_: vec3<f32>;
    var phi_2729_: type_59;
    var phi_2730_: type_59;
    var phi_10247_: vec3<f32>;
    var phi_2731_: type_59;
    var phi_2732_: type_59;
    var phi_2733_: bool;
    var phi_2737_: type_59;
    var phi_10279_: bool;
    var phi_2822_: f32;
    var phi_10350_: bool;
    var phi_10365_: bool;
    var phi_2878_: vec3<f32>;
    var phi_2879_: vec3<f32>;
    var phi_15530_: u32;
    var phi_2880_: vec3<f32>;
    var phi_2881_: f32;
    var phi_2882_: vec3<f32>;
    var phi_15529_: u32;
    var phi_2883_: vec3<f32>;
    var phi_2884_: f32;
    var phi_2885_: vec3<f32>;
    var phi_15491_: u32;
    var phi_4090_: vec3<f32>;
    var phi_4091_: f32;
    var phi_4092_: vec3<f32>;
    var phi_4093_: f32;
    var phi_2322_: f32;
    var phi_2323_: f32;
    var phi_15490_: u32;
    var phi_4094_: vec3<f32>;
    var phi_4095_: f32;
    var phi_4096_: vec3<f32>;
    var phi_4097_: f32;
    var phi_10380_: bool;
    var phi_10490_: f32;
    var phi_10518_: f32;
    var phi_10546_: f32;
    var phi_10680_: bool;
    var phi_10695_: bool;
    var phi_10710_: bool;
    var phi_10737_: bool;
    var phi_10752_: bool;
    var phi_10767_: bool;
    var phi_10794_: bool;
    var phi_10809_: bool;
    var phi_10824_: bool;
    var phi_4320_: f32;
    var phi_4328_: f32;
    var phi_4336_: f32;
    var phi_11021_: bool;
    var phi_11010_: f32;
    var phi_4443_: vec3<f32>;
    var phi_11049_: bool;
    var phi_4444_: vec3<f32>;
    var phi_4456_: type_59;
    var phi_4457_: type_59;
    var phi_4458_: bool;
    var phi_4463_: type_59;
    var phi_11086_: bool;
    var phi_11116_: bool;
    var phi_11105_: f32;
    var phi_11155_: vec3<f32>;
    var phi_4491_: type_59;
    var phi_4492_: type_59;
    var phi_11177_: vec3<f32>;
    var phi_4493_: type_59;
    var phi_4494_: type_59;
    var phi_4495_: bool;
    var phi_4499_: type_59;
    var phi_11191_: f32;
    var phi_11194_: f32;
    var phi_4508_: f32;
    var phi_4541_: u32;
    var phi_4544_: u32;
    var phi_4546_: u32;
    var phi_4550_: type_49;
    var phi_11291_: bool;
    var phi_11321_: bool;
    var phi_11268_: type_52;
    var phi_11271_: type_52;
    var phi_11272_: bool;
    var phi_11277_: type_52;
    var phi_11378_: bool;
    var phi_11393_: bool;
    var phi_11408_: bool;
    var phi_11423_: bool;
    var phi_11355_: type_52;
    var phi_11358_: type_52;
    var phi_11359_: bool;
    var phi_11364_: type_52;
    var phi_11465_: bool;
    var phi_11480_: bool;
    var phi_11495_: bool;
    var phi_11510_: bool;
    var phi_11442_: type_52;
    var phi_11445_: type_52;
    var phi_11446_: bool;
    var phi_11451_: type_52;
    var phi_11244_: bool;
    var phi_11247_: bool;
    var phi_11248_: bool;
    var phi_4957_: u32;
    var phi_4958_: u32;
    var phi_4959_: u32;
    var phi_4960_: u32;
    var phi_4961_: u32;
    var phi_4962_: u32;
    var phi_4963_: u32;
    var phi_4964_: u32;
    var phi_4965_: u32;
    var phi_4846_: vec4<f32>;
    var phi_4847_: vec4<f32>;
    var phi_4848_: bool;
    var phi_4852_: vec4<f32>;
    var phi_4853_: bool;
    var phi_4758_: bool;
    var phi_4795_: bool;
    var phi_4801_: f32;
    var phi_4811_: vec4<f32>;
    var phi_4812_: vec4<f32>;
    var phi_4813_: bool;
    var phi_4817_: vec4<f32>;
    var phi_4818_: bool;
    var phi_4822_: vec4<f32>;
    var phi_4823_: bool;
    var phi_4854_: vec4<f32>;
    var phi_4855_: bool;
    var phi_4687_: vec4<f32>;
    var phi_4688_: vec4<f32>;
    var phi_4689_: bool;
    var phi_4693_: vec4<f32>;
    var phi_4694_: bool;
    var phi_4695_: vec4<f32>;
    var phi_4696_: bool;
    var phi_4697_: bool;
    var phi_4705_: vec4<f32>;
    var phi_4706_: bool;
    var phi_4707_: vec4<f32>;
    var phi_4708_: bool;
    var phi_4709_: bool;
    var phi_4717_: vec4<f32>;
    var phi_4718_: bool;
    var phi_4864_: vec4<f32>;
    var phi_4865_: bool;
    var phi_4886_: type_49;
    var phi_4888_: type_49;
    var phi_4894_: u32;
    var phi_4542_: u32;
    var phi_4545_: u32;
    var phi_4547_: u32;
    var phi_4551_: type_49;
    var phi_4967_: type_49;
    var phi_4968_: bool;
    var phi_4969_: bool;
    var phi_15071_: bool;
    var phi_7701_: bool;
    var phi_7700_: type_49;
    var phi_5141_: type_49;
    var phi_5146_: type_49;
    var phi_5179_: vec3<f32>;
    var phi_15452_: bool;
    var phi_5180_: vec3<f32>;
    var phi_15451_: bool;
    var phi_5181_: vec3<f32>;
    var phi_15668_: u32;
    var phi_15450_: bool;
    var phi_2224_: vec3<f32>;
    var phi_2227_: u32;
    var phi_15462_: u32;
    var phi_15449_: bool;
    var phi_11646_: bool;
    var phi_11676_: bool;
    var phi_11665_: f32;
    var phi_11596_: bool;
    var phi_11732_: vec3<f32>;
    var phi_11888_: bool;
    var phi_11903_: bool;
    var phi_11918_: bool;
    var phi_11827_: vec3<f32>;
    var phi_11830_: vec3<f32>;
    var phi_11831_: bool;
    var phi_11601_: vec3<f32>;
    var phi_11605_: vec3<f32>;
    var phi_11606_: bool;
    var phi_12018_: bool;
    var phi_11992_: vec3<f32>;
    var phi_11613_: vec3<f32>;
    var phi_12033_: bool;
    var phi_12143_: f32;
    var phi_12171_: f32;
    var phi_12199_: f32;
    var phi_12333_: bool;
    var phi_12348_: bool;
    var phi_12363_: bool;
    var phi_12390_: bool;
    var phi_12405_: bool;
    var phi_12420_: bool;
    var phi_12447_: bool;
    var phi_12462_: bool;
    var phi_12477_: bool;
    var phi_5398_: f32;
    var phi_5406_: f32;
    var phi_5414_: f32;
    var phi_12674_: bool;
    var phi_12663_: f32;
    var phi_5521_: vec3<f32>;
    var phi_12702_: bool;
    var phi_5522_: vec3<f32>;
    var phi_5534_: type_59;
    var phi_5535_: type_59;
    var phi_5536_: bool;
    var phi_5541_: type_59;
    var phi_12739_: bool;
    var phi_12769_: bool;
    var phi_12758_: f32;
    var phi_12808_: vec3<f32>;
    var phi_5569_: type_59;
    var phi_5570_: type_59;
    var phi_12830_: vec3<f32>;
    var phi_5571_: type_59;
    var phi_5572_: type_59;
    var phi_5573_: bool;
    var phi_5577_: type_59;
    var phi_12855_: bool;
    var phi_5653_: vec3<f32>;
    var phi_17021_: u32;
    var phi_5655_: vec3<f32>;
    var phi_5656_: bool;
    var phi_17020_: u32;
    var phi_5657_: vec3<f32>;
    var phi_5658_: bool;
    var phi_17018_: u32;
    var phi_5664_: vec3<f32>;
    var phi_5665_: bool;
    var phi_5694_: vec3<f32>;
    var phi_5706_: vec3<f32>;
    var phi_5707_: u32;
    var phi_5708_: vec3<f32>;
    var phi_17016_: u32;
    var phi_5710_: vec3<f32>;
    var phi_5711_: u32;
    var phi_5712_: vec3<f32>;
    var phi_5713_: bool;
    var phi_5714_: bool;
    var phi_17015_: u32;
    var phi_5715_: vec3<f32>;
    var phi_5716_: u32;
    var phi_5717_: vec3<f32>;
    var phi_5718_: bool;
    var phi_5719_: bool;
    var phi_5720_: bool;
    var phi_17014_: u32;
    var phi_5721_: vec3<f32>;
    var phi_5722_: f32;
    var phi_5723_: vec3<f32>;
    var phi_5724_: u32;
    var phi_5725_: vec3<f32>;
    var phi_5726_: bool;
    var phi_5727_: bool;
    var phi_5728_: bool;
    var phi_5729_: bool;
    var phi_17013_: u32;
    var phi_16919_: bool;
    var phi_5730_: vec3<f32>;
    var phi_5733_: vec3<f32>;
    var phi_5734_: vec3<f32>;
    var phi_5735_: f32;
    var phi_5736_: vec3<f32>;
    var phi_5737_: u32;
    var phi_5740_: vec3<f32>;
    var phi_5742_: bool;
    var phi_5743_: bool;
    var phi_5744_: bool;
    var phi_5745_: bool;
    var phi_5747_: bool;
    var phi_17012_: u32;
    var phi_16918_: bool;
    var phi_5749_: vec3<f32>;
    var phi_5752_: vec3<f32>;
    var phi_5753_: vec3<f32>;
    var phi_5754_: f32;
    var phi_5755_: vec3<f32>;
    var phi_5756_: u32;
    var phi_5757_: bool;
    var phi_5758_: bool;
    var phi_5759_: vec3<f32>;
    var phi_5761_: bool;
    var phi_5762_: bool;
    var phi_5763_: bool;
    var phi_5764_: bool;
    var phi_5765_: bool;
    var phi_5766_: bool;
    var phi_5767_: bool;
    var phi_17011_: u32;
    var phi_16917_: bool;
    var phi_5768_: vec3<f32>;
    var phi_5769_: vec3<f32>;
    var phi_5770_: f32;
    var phi_5771_: f32;
    var phi_5772_: f32;
    var phi_5773_: vec3<f32>;
    var phi_5774_: type_14;
    var phi_5775_: u32;
    var phi_5776_: vec3<f32>;
    var phi_335_: vec3<f32>;
    var phi_338_: vec3<f32>;
    var phi_340_: f32;
    var phi_342_: vec3<f32>;
    var phi_344_: u32;
    var phi_346_: bool;
    var phi_348_: bool;
    var phi_350_: vec3<f32>;
    var phi_5782_: bool;
    var phi_5783_: bool;
    var phi_5784_: bool;
    var phi_5785_: bool;
    var phi_5786_: bool;
    var phi_5787_: bool;
    var phi_5788_: bool;
    var phi_5789_: bool;
    var phi_16916_: bool;
    var phi_7728_: bool;
    var phi_7727_: type_49;
    var phi_7726_: bool;
    var phi_7725_: bool;
    var phi_7724_: bool;
    var phi_7723_: bool;
    var phi_7722_: bool;
    var phi_7721_: bool;
    var phi_7720_: bool;
    var phi_7719_: vec3<f32>;
    var phi_7717_: vec3<f32>;
    var phi_7713_: u32;
    var phi_7711_: type_14;
    var phi_7709_: vec3<f32>;
    var phi_7708_: f32;
    var phi_7707_: f32;
    var phi_7706_: f32;
    var phi_7704_: vec3<f32>;
    var local_19: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_5945_: f32;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_12898_: f32;
    var phi_12901_: f32;
    var phi_5947_: f32;
    var phi_5948_: f32;
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
    var phi_6326_: f32;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_12967_: f32;
    var phi_12970_: f32;
    var phi_6328_: f32;
    var phi_12981_: bool;
    var phi_12996_: bool;
    var phi_6383_: vec3<f32>;
    var phi_6400_: vec3<f32>;
    var phi_6401_: vec3<f32>;
    var local_39: vec3<f32>;
    var local_40: vec3<f32>;
    var local_41: vec3<f32>;
    var phi_6453_: bool;
    var phi_6462_: vec3<f32>;
    var phi_6464_: vec3<f32>;
    var phi_6465_: bool;
    var local_42: bool;
    var local_43: vec3<f32>;
    var local_44: vec3<f32>;
    var local_45: vec3<f32>;
    var phi_6629_: f32;
    var phi_6632_: f32;
    var local_46: vec3<f32>;
    var local_47: vec3<f32>;
    var local_48: vec3<f32>;
    var phi_13084_: bool;
    var phi_13065_: f32;
    var phi_6633_: f32;
    var local_49: f32;
    var local_50: f32;
    var local_51: f32;
    var phi_13102_: f32;
    var phi_13105_: f32;
    var phi_6636_: f32;
    var phi_6637_: f32;
    var phi_6638_: f32;
    var phi_6660_: vec3<f32>;
    var local_52: type_49;
    var local_53: type_49;
    var local_54: vec3<f32>;
    var local_55: vec3<f32>;
    var local_56: u32;
    var break_if_0_: bool;
    var break_if_1_: bool;
    var break_if_2_: bool;
    var break_if_3_: bool;
    var break_if_4_: bool;
    var break_if_5_: bool;
    var break_if_6_: bool;
    var break_if_7_: bool;
    var break_if_8_: bool;

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
                    phi_13221_ = ((_e184 >> bitcast<u32>(22i)) ^ _e184);
                    phi_13141_ = false;
                    phi_334_ = vec3<f32>(0f, 0f, 0f);
                    phi_337_ = vec3<f32>(1f, 1f, 1f);
                    phi_339_ = 0f;
                    phi_341_ = vec3<f32>((_e148.x * _e201), (_e148.y * _e201), (_e148.z * _e201));
                    phi_343_ = 0u;
                    phi_345_ = false;
                    phi_347_ = true;
                    phi_349_ = vec3<f32>(_e144.x, _e144.y, _e144.z);
                    loop {
                        let _e216 = phi_13221_;
                        let _e218 = phi_13141_;
                        let _e220 = phi_334_;
                        let _e222 = phi_337_;
                        let _e224 = phi_339_;
                        let _e226 = phi_341_;
                        let _e228 = phi_343_;
                        let _e230 = phi_345_;
                        let _e232 = phi_347_;
                        let _e234 = phi_349_;
                        let _e237 = global_1.member.member_1[0u];
                        let _e240 = global_1.member.member_1[1u];
                        let _e241 = (_e237 + _e240);
                        let _e242 = (vec3<f32>(1f, 1f, 1f) / _e226);
                        local_1 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                        let _e244 = select(0u, 1u, (_e237 > 0u));
                        phi_370_ = _e237;
                        phi_373_ = _e244;
                        phi_375_ = _e241;
                        phi_377_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_379_ = type_49(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                        local_19 = _e232;
                        local_20 = _e226;
                        local_21 = _e226;
                        local_22 = _e226;
                        local_23 = _e224;
                        local_24 = _e224;
                        local_25 = _e224;
                        local_26 = _e226;
                        local_27 = _e226;
                        local_28 = _e226;
                        local_29 = _e222;
                        local_30 = _e222;
                        local_31 = _e222;
                        local_32 = _e220;
                        local_33 = _e220;
                        local_34 = _e220;
                        local_35 = _e232;
                        local_36 = _e224;
                        local_37 = _e224;
                        local_38 = _e224;
                        local_39 = _e226;
                        local_40 = _e226;
                        local_41 = _e226;
                        local_42 = _e232;
                        local_43 = _e234;
                        local_44 = _e234;
                        local_45 = _e234;
                        local_46 = _e234;
                        local_47 = _e234;
                        local_48 = _e234;
                        local_49 = _e224;
                        local_50 = _e224;
                        local_51 = _e224;
                        loop {
                            let _e246 = phi_370_;
                            let _e248 = phi_373_;
                            let _e250 = phi_375_;
                            let _e252 = phi_377_;
                            let _e254 = phi_379_;
                            local_2 = _e254;
                            local_3 = _e252;
                            local_4 = _e252;
                            local_5 = _e252;
                            local_6 = _e254;
                            local_52 = _e254;
                            if (_e246 < _e250) {
                                let _e426 = global_2.member[_e246];
                                if (_e426.member_6.x == 0u) {
                                    let _e540 = (_e426.member_1.x - _e426.member.x);
                                    let _e541 = (_e426.member_1.y - _e426.member.y);
                                    let _e542 = (_e426.member_1.z - _e426.member.z);
                                    let _e547 = (_e426.member_2.x - _e426.member.x);
                                    let _e548 = (_e426.member_2.y - _e426.member.y);
                                    let _e549 = (_e426.member_2.z - _e426.member.z);
                                    let _e554 = ((_e226.y * _e549) - (_e548 * _e226.z));
                                    let _e558 = ((_e226.z * _e547) - (_e549 * _e226.x));
                                    let _e561 = ((_e226.x * _e548) - (_e547 * _e226.y));
                                    let _e566 = (((_e540 * _e554) + (_e541 * _e558)) + (_e542 * _e561));
                                    if (abs(_e566) < 0.000000000001f) {
                                        phi_549_ = vec4<f32>();
                                        phi_550_ = true;
                                    } else {
                                        let _e569 = (1f / _e566);
                                        let _e571 = (_e234.x - _e426.member.x);
                                        let _e573 = (_e234.y - _e426.member.y);
                                        let _e575 = (_e234.z - _e426.member.z);
                                        let _e581 = ((((_e571 * _e554) + (_e573 * _e558)) + (_e575 * _e561)) * _e569);
                                        if (_e581 < 0f) {
                                            phi_539_ = vec4<f32>();
                                            phi_540_ = true;
                                            phi_541_ = false;
                                        } else {
                                            let _e583 = (_e581 > 1f);
                                            if _e583 {
                                                phi_537_ = vec4<f32>();
                                                phi_538_ = false;
                                            } else {
                                                let _e586 = ((_e573 * _e542) - (_e541 * _e575));
                                                let _e589 = ((_e575 * _e540) - (_e542 * _e571));
                                                let _e592 = ((_e571 * _e541) - (_e540 * _e573));
                                                let _e598 = ((((_e226.x * _e586) + (_e226.y * _e589)) + (_e226.z * _e592)) * _e569);
                                                if (_e598 < 0f) {
                                                    phi_527_ = vec4<f32>();
                                                    phi_528_ = true;
                                                    phi_529_ = false;
                                                } else {
                                                    let _e601 = ((_e581 + _e598) > 1f);
                                                    if _e601 {
                                                        phi_525_ = vec4<f32>();
                                                        phi_526_ = false;
                                                    } else {
                                                        let _e607 = ((((_e547 * _e586) + (_e548 * _e589)) + (_e549 * _e592)) * _e569);
                                                        if (_e607 <= 0f) {
                                                            phi_520_ = vec4<f32>();
                                                            phi_521_ = true;
                                                        } else {
                                                            let _e609 = (_e607 >= _e254.member);
                                                            if _e609 {
                                                                phi_519_ = vec4<f32>();
                                                            } else {
                                                                phi_519_ = vec4<f32>(_e607, _e581, _e598, 1f);
                                                            }
                                                            let _e612 = phi_519_;
                                                            phi_520_ = _e612;
                                                            phi_521_ = _e609;
                                                        }
                                                        let _e614 = phi_520_;
                                                        let _e616 = phi_521_;
                                                        phi_525_ = _e614;
                                                        phi_526_ = _e616;
                                                    }
                                                    let _e618 = phi_525_;
                                                    let _e620 = phi_526_;
                                                    phi_527_ = _e618;
                                                    phi_528_ = _e601;
                                                    phi_529_ = _e620;
                                                }
                                                let _e622 = phi_527_;
                                                let _e624 = phi_528_;
                                                let _e626 = phi_529_;
                                                phi_537_ = _e622;
                                                phi_538_ = select(_e626, true, _e624);
                                            }
                                            let _e629 = phi_537_;
                                            let _e631 = phi_538_;
                                            phi_539_ = _e629;
                                            phi_540_ = _e583;
                                            phi_541_ = _e631;
                                        }
                                        let _e633 = phi_539_;
                                        let _e635 = phi_540_;
                                        let _e637 = phi_541_;
                                        phi_549_ = _e633;
                                        phi_550_ = select(_e637, true, _e635);
                                    }
                                    let _e640 = phi_549_;
                                    let _e642 = phi_550_;
                                    phi_700_ = _e640;
                                    phi_701_ = _e642;
                                } else {
                                    if (_e426.member_6.x == 1u) {
                                        let _e457 = (_e234.x - _e426.member.x);
                                        let _e459 = (_e234.y - _e426.member.y);
                                        let _e461 = (_e234.z - _e426.member.z);
                                        let _e469 = (((_e457 * _e226.x) + (_e459 * _e226.y)) + (_e461 * _e226.z));
                                        if (_e469 > 0f) {
                                            phi_593_ = select(true, false, ((((_e457 * _e457) + (_e459 * _e459)) + (_e461 * _e461)) > (_e426.member.w * _e426.member.w)));
                                        } else {
                                            phi_593_ = true;
                                        }
                                        let _e482 = phi_593_;
                                        if _e482 {
                                            let _e486 = (_e457 - (_e226.x * _e469));
                                            let _e487 = (_e459 - (_e226.y * _e469));
                                            let _e488 = (_e461 - (_e226.z * _e469));
                                            let _e497 = ((_e426.member.w * _e426.member.w) - (((_e486 * _e486) + (_e487 * _e487)) + (_e488 * _e488)));
                                            if (_e497 < 0f) {
                                                phi_652_ = vec4<f32>();
                                                phi_653_ = true;
                                            } else {
                                                let _e499 = sqrt(_e497);
                                                let _e500 = -(_e469);
                                                let _e501 = (_e500 - _e499);
                                                if (_e501 <= 0f) {
                                                    phi_630_ = true;
                                                } else {
                                                    phi_630_ = (_e501 >= _e254.member);
                                                }
                                                let _e505 = phi_630_;
                                                if _e505 {
                                                    phi_636_ = (_e500 + _e499);
                                                } else {
                                                    phi_636_ = _e501;
                                                }
                                                let _e508 = phi_636_;
                                                if (_e508 <= 0f) {
                                                    phi_647_ = vec4<f32>();
                                                    phi_648_ = true;
                                                } else {
                                                    let _e510 = (_e508 >= _e254.member);
                                                    if _e510 {
                                                        phi_646_ = vec4<f32>();
                                                    } else {
                                                        phi_646_ = vec4<f32>(_e508, 0f, 0f, 1f);
                                                    }
                                                    let _e513 = phi_646_;
                                                    phi_647_ = _e513;
                                                    phi_648_ = _e510;
                                                }
                                                let _e515 = phi_647_;
                                                let _e517 = phi_648_;
                                                phi_652_ = _e515;
                                                phi_653_ = _e517;
                                            }
                                            let _e519 = phi_652_;
                                            let _e521 = phi_653_;
                                            phi_657_ = _e519;
                                            phi_658_ = _e521;
                                        } else {
                                            phi_657_ = vec4<f32>();
                                            phi_658_ = true;
                                        }
                                        let _e523 = phi_657_;
                                        let _e525 = phi_658_;
                                        phi_690_ = _e523;
                                        phi_691_ = _e525;
                                    } else {
                                        if (abs(_e226.y) < 0.000000000001f) {
                                            phi_688_ = vec4<f32>();
                                            phi_689_ = true;
                                        } else {
                                            let _e438 = ((_e426.member.x - _e234.y) / _e226.y);
                                            if (_e438 <= 0f) {
                                                phi_683_ = vec4<f32>();
                                                phi_684_ = true;
                                            } else {
                                                let _e440 = (_e438 >= _e254.member);
                                                if _e440 {
                                                    phi_682_ = vec4<f32>();
                                                } else {
                                                    phi_682_ = vec4<f32>(_e438, 0f, 0f, 1f);
                                                }
                                                let _e443 = phi_682_;
                                                phi_683_ = _e443;
                                                phi_684_ = _e440;
                                            }
                                            let _e445 = phi_683_;
                                            let _e447 = phi_684_;
                                            phi_688_ = _e445;
                                            phi_689_ = _e447;
                                        }
                                        let _e449 = phi_688_;
                                        let _e451 = phi_689_;
                                        phi_690_ = _e449;
                                        phi_691_ = _e451;
                                    }
                                    let _e527 = phi_690_;
                                    let _e529 = phi_691_;
                                    phi_700_ = _e527;
                                    phi_701_ = select(true, false, select(true, false, _e529));
                                }
                                let _e644 = phi_700_;
                                let _e646 = phi_701_;
                                let _e649 = select(vec4<f32>(0f, 0f, 0f, 0f), _e644, vec4(select(true, false, _e646)));
                                let _e651 = (_e649.w > 0f);
                                if _e651 {
                                    let _e657 = type_49(_e649.x, _e254.member_1, _e254.member_2, _e254.member_3);
                                    let _e662 = type_49(_e657.member, _e246, _e657.member_2, _e657.member_3);
                                    phi_722_ = _e662;
                                    phi_724_ = _e662;
                                } else {
                                    phi_722_ = type_49();
                                    phi_724_ = _e254;
                                }
                                let _e664 = phi_722_;
                                let _e666 = phi_724_;
                                phi_371_ = (_e246 + 1u);
                                phi_374_ = _e248;
                                phi_376_ = _e250;
                                phi_378_ = select(_e252, _e649, vec4(_e651));
                                phi_380_ = _e666;
                                phi_803_ = _e664;
                                phi_804_ = true;
                                phi_805_ = false;
                            } else {
                                let _e256 = (_e248 == 0u);
                                if _e256 {
                                    phi_799_ = u32();
                                    phi_800_ = u32();
                                    phi_801_ = u32();
                                } else {
                                    let _e257 = (_e248 - 1u);
                                    let _e258 = (_e257 < 100u);
                                    if _e258 {
                                    } else {
                                        phi_13126_ = true;
                                        phi_7699_ = bool();
                                        phi_7697_ = type_49();
                                        break;
                                    }
                                    let _e260 = local_1[_e257];
                                    let _e262 = global_3.member[_e260];
                                    let _e276 = ((_e262.member.x - _e234.x) * _e242.x);
                                    let _e278 = ((_e262.member.y - _e234.y) * _e242.y);
                                    let _e280 = ((_e262.member.z - _e234.z) * _e242.z);
                                    let _e287 = ((_e262.member_2.x - _e234.x) * _e242.x);
                                    let _e288 = ((_e262.member_2.y - _e234.y) * _e242.y);
                                    let _e289 = ((_e262.member_2.z - _e234.z) * _e242.z);
                                    if (_e276 == _e276) {
                                        let _e291 = (_e287 == _e287);
                                        if _e291 {
                                            let _e292 = (_e276 != _e276);
                                            if _e292 {
                                                phi_7857_ = true;
                                            } else {
                                                phi_7857_ = (_e287 <= _e276);
                                            }
                                            let _e295 = phi_7857_;
                                            let _e296 = select(_e276, _e287, _e295);
                                            if _e292 {
                                                phi_7887_ = true;
                                            } else {
                                                phi_7887_ = (_e287 >= _e276);
                                            }
                                            let _e301 = phi_7887_;
                                            let _e302 = select(_e276, _e287, _e301);
                                            phi_7834_ = type_52(select(-300000000000000000000000000000000000000f, _e296, (_e296 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e302, (_e302 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_7834_ = type_52();
                                        }
                                        let _e307 = phi_7834_;
                                        phi_7837_ = _e307;
                                        phi_7838_ = select(true, false, _e291);
                                    } else {
                                        phi_7837_ = type_52();
                                        phi_7838_ = true;
                                    }
                                    let _e310 = phi_7837_;
                                    let _e312 = phi_7838_;
                                    if _e312 {
                                        phi_7843_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_7843_ = _e310;
                                    }
                                    let _e314 = phi_7843_;
                                    if (_e278 == _e278) {
                                        let _e318 = (_e288 == _e288);
                                        if _e318 {
                                            let _e319 = (_e278 != _e278);
                                            if _e319 {
                                                phi_7944_ = true;
                                            } else {
                                                phi_7944_ = (_e288 <= _e278);
                                            }
                                            let _e322 = phi_7944_;
                                            let _e323 = select(_e278, _e288, _e322);
                                            if (_e314.member != _e314.member) {
                                                phi_7959_ = true;
                                            } else {
                                                phi_7959_ = (_e323 >= _e314.member);
                                            }
                                            let _e327 = phi_7959_;
                                            if _e319 {
                                                phi_7974_ = true;
                                            } else {
                                                phi_7974_ = (_e288 >= _e278);
                                            }
                                            let _e331 = phi_7974_;
                                            let _e332 = select(_e278, _e288, _e331);
                                            if (_e314.member_1 != _e314.member_1) {
                                                phi_7989_ = true;
                                            } else {
                                                phi_7989_ = (_e332 <= _e314.member_1);
                                            }
                                            let _e336 = phi_7989_;
                                            phi_7921_ = type_52(select(_e314.member, _e323, _e327), select(_e314.member_1, _e332, _e336));
                                        } else {
                                            phi_7921_ = type_52();
                                        }
                                        let _e340 = phi_7921_;
                                        phi_7924_ = _e340;
                                        phi_7925_ = select(true, false, _e318);
                                    } else {
                                        phi_7924_ = type_52();
                                        phi_7925_ = true;
                                    }
                                    let _e343 = phi_7924_;
                                    let _e345 = phi_7925_;
                                    if _e345 {
                                        phi_7930_ = _e314;
                                    } else {
                                        phi_7930_ = _e343;
                                    }
                                    let _e347 = phi_7930_;
                                    if (_e280 == _e280) {
                                        let _e351 = (_e289 == _e289);
                                        if _e351 {
                                            let _e352 = (_e280 != _e280);
                                            if _e352 {
                                                phi_8031_ = true;
                                            } else {
                                                phi_8031_ = (_e289 <= _e280);
                                            }
                                            let _e355 = phi_8031_;
                                            let _e356 = select(_e280, _e289, _e355);
                                            if (_e347.member != _e347.member) {
                                                phi_8046_ = true;
                                            } else {
                                                phi_8046_ = (_e356 >= _e347.member);
                                            }
                                            let _e360 = phi_8046_;
                                            if _e352 {
                                                phi_8061_ = true;
                                            } else {
                                                phi_8061_ = (_e289 >= _e280);
                                            }
                                            let _e364 = phi_8061_;
                                            let _e365 = select(_e280, _e289, _e364);
                                            if (_e347.member_1 != _e347.member_1) {
                                                phi_8076_ = true;
                                            } else {
                                                phi_8076_ = (_e365 <= _e347.member_1);
                                            }
                                            let _e369 = phi_8076_;
                                            phi_8008_ = type_52(select(_e347.member, _e356, _e360), select(_e347.member_1, _e365, _e369));
                                        } else {
                                            phi_8008_ = type_52();
                                        }
                                        let _e373 = phi_8008_;
                                        phi_8011_ = _e373;
                                        phi_8012_ = select(true, false, _e351);
                                    } else {
                                        phi_8011_ = type_52();
                                        phi_8012_ = true;
                                    }
                                    let _e376 = phi_8011_;
                                    let _e378 = phi_8012_;
                                    if _e378 {
                                        phi_8017_ = _e347;
                                    } else {
                                        phi_8017_ = _e376;
                                    }
                                    let _e380 = phi_8017_;
                                    if (_e380.member_1 < 0f) {
                                        phi_7812_ = bool();
                                        phi_7813_ = true;
                                    } else {
                                        let _e384 = (_e380.member > _e380.member_1);
                                        if _e384 {
                                            phi_7809_ = bool();
                                        } else {
                                            phi_7809_ = (_e380.member > _e254.member);
                                        }
                                        let _e387 = phi_7809_;
                                        phi_7812_ = _e387;
                                        phi_7813_ = _e384;
                                    }
                                    let _e389 = phi_7812_;
                                    let _e391 = phi_7813_;
                                    if !(select(_e389, true, _e391)) {
                                        if (_e262.member_3 > 0u) {
                                            phi_793_ = _e262.member_1;
                                            phi_794_ = _e257;
                                            phi_795_ = (_e262.member_1 + _e262.member_3);
                                        } else {
                                            let _e396 = (_e248 + 1u);
                                            let _e397 = (_e396 <= 100u);
                                            if _e397 {
                                                if _e258 {
                                                } else {
                                                    phi_13126_ = true;
                                                    phi_7699_ = bool();
                                                    phi_7697_ = type_49();
                                                    break;
                                                }
                                                local_1[_e257] = _e262.member_1;
                                                if (_e248 < 100u) {
                                                } else {
                                                    phi_13126_ = true;
                                                    phi_7699_ = bool();
                                                    phi_7697_ = type_49();
                                                    break;
                                                }
                                                local_1[_e248] = (_e260 + 1u);
                                            }
                                            phi_793_ = _e246;
                                            phi_794_ = select(_e257, _e396, _e397);
                                            phi_795_ = _e250;
                                        }
                                        let _e406 = phi_793_;
                                        let _e408 = phi_794_;
                                        let _e410 = phi_795_;
                                        phi_796_ = _e406;
                                        phi_797_ = _e408;
                                        phi_798_ = _e410;
                                    } else {
                                        phi_796_ = _e246;
                                        phi_797_ = _e257;
                                        phi_798_ = _e250;
                                    }
                                    let _e412 = phi_796_;
                                    let _e414 = phi_797_;
                                    let _e416 = phi_798_;
                                    phi_799_ = _e412;
                                    phi_800_ = _e414;
                                    phi_801_ = _e416;
                                }
                                let _e418 = phi_799_;
                                let _e420 = phi_800_;
                                let _e422 = phi_801_;
                                phi_371_ = _e418;
                                phi_374_ = _e420;
                                phi_376_ = _e422;
                                phi_378_ = _e252;
                                phi_380_ = _e254;
                                phi_803_ = type_49();
                                phi_804_ = select(true, false, _e256);
                                phi_805_ = _e256;
                            }
                            let _e671 = phi_371_;
                            let _e673 = phi_374_;
                            let _e675 = phi_376_;
                            let _e677 = phi_378_;
                            let _e679 = phi_380_;
                            let _e681 = phi_803_;
                            let _e683 = phi_804_;
                            break_if_0_ = _e683;
                            let _e685 = phi_805_;
                            continue;
                            continuing {
                                phi_370_ = _e671;
                                phi_373_ = _e673;
                                phi_375_ = _e675;
                                phi_377_ = _e677;
                                phi_379_ = _e679;
                                phi_13126_ = _e218;
                                phi_7699_ = _e685;
                                phi_7697_ = _e681;
                                let _e7319 = break_if_0_;
                                break if !(_e7319);
                            }
                        }
                        let _e688 = phi_13126_;
                        let _e690 = phi_7699_;
                        let _e692 = phi_7697_;
                        phi_16916_ = _e688;
                        phi_7728_ = bool();
                        phi_7727_ = type_49();
                        phi_7726_ = bool();
                        phi_7725_ = bool();
                        phi_7724_ = bool();
                        phi_7723_ = bool();
                        phi_7722_ = bool();
                        phi_7721_ = bool();
                        phi_7720_ = bool();
                        phi_7719_ = vec3<f32>();
                        phi_7717_ = vec3<f32>();
                        phi_7713_ = u32();
                        phi_7711_ = type_14();
                        phi_7709_ = vec3<f32>();
                        phi_7708_ = f32();
                        phi_7707_ = f32();
                        phi_7706_ = f32();
                        phi_7704_ = vec3<f32>();
                        if _e688 {
                            break;
                        }
                        if _e690 {
                            let _e694 = local_2;
                            if (_e694.member_1 != 4294967295u) {
                                let _e698 = global_2.member[_e694.member_1];
                                if (_e698.member_6.x == 0u) {
                                    let _e747 = (_e698.member_1.x - _e698.member.x);
                                    let _e748 = (_e698.member_1.y - _e698.member.y);
                                    let _e749 = (_e698.member_1.z - _e698.member.z);
                                    let _e754 = (_e698.member_2.x - _e698.member.x);
                                    let _e755 = (_e698.member_2.y - _e698.member.y);
                                    let _e756 = (_e698.member_2.z - _e698.member.z);
                                    let _e759 = ((_e748 * _e756) - (_e755 * _e749));
                                    let _e762 = ((_e749 * _e754) - (_e756 * _e747));
                                    let _e765 = ((_e747 * _e755) - (_e754 * _e748));
                                    let _e772 = (1f / sqrt((((_e759 * _e759) + (_e762 * _e762)) + (_e765 * _e765))));
                                    let _e776 = vec3<f32>((_e759 * _e772), (_e762 * _e772), (_e765 * _e772));
                                    if (_e698.member_6.w == 1u) {
                                        let _e785 = local_4;
                                        let _e789 = local_5;
                                        let _e791 = ((1f - _e785.y) - _e789.z);
                                        let _e812 = (((_e698.member_3.x * _e791) + (_e698.member_4.x * _e785.y)) + (_e698.member_5.x * _e789.z));
                                        let _e813 = (((_e698.member_3.y * _e791) + (_e698.member_4.y * _e785.y)) + (_e698.member_5.y * _e789.z));
                                        let _e814 = (((_e698.member_3.z * _e791) + (_e698.member_4.z * _e785.y)) + (_e698.member_5.z * _e789.z));
                                        let _e819 = (((_e812 * _e812) + (_e813 * _e813)) + (_e814 * _e814));
                                        if (_e819 > 0f) {
                                            let _e822 = (1f / sqrt(_e819));
                                            phi_932_ = vec3<f32>((_e812 * _e822), (_e813 * _e822), (_e814 * _e822));
                                        } else {
                                            phi_932_ = _e776;
                                        }
                                        let _e828 = phi_932_;
                                        phi_933_ = _e828;
                                    } else {
                                        phi_933_ = _e776;
                                    }
                                    let _e830 = phi_933_;
                                    phi_974_ = type_53(_e776, _e830);
                                } else {
                                    if (_e698.member_6.x == 1u) {
                                        let _e709 = local_3;
                                        let _e735 = vec3<f32>((((_e234.x + (_e226.x * _e709.x)) - _e698.member.x) / _e698.member.w), (((_e234.y + (_e226.y * _e709.x)) - _e698.member.y) / _e698.member.w), (((_e234.z + (_e226.z * _e709.x)) - _e698.member.z) / _e698.member.w));
                                        phi_973_ = type_53(_e735, _e735);
                                    } else {
                                        let _e706 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e226.y > 0f)));
                                        phi_973_ = type_53(_e706, _e706);
                                    }
                                    let _e738 = phi_973_;
                                    phi_974_ = _e738;
                                }
                                let _e833 = phi_974_;
                                let _e837 = local_6;
                                let _e842 = type_49(_e837.member, _e837.member_1, _e833.member, _e837.member_3);
                                phi_979_ = type_49(_e842.member, _e842.member_1, _e842.member_2, _e833.member_1);
                            } else {
                                let _e6653 = local_52;
                                phi_979_ = _e6653;
                            }
                            let _e849 = phi_979_;
                            phi_980_ = _e849;
                        } else {
                            phi_980_ = type_49();
                        }
                        let _e851 = phi_980_;
                        if select(true, false, _e690) {
                            phi_985_ = _e692;
                        } else {
                            phi_985_ = _e851;
                        }
                        let _e854 = phi_985_;
                        let _e856 = (_e854.member_1 == 4294967295u);
                        if _e856 {
                            phi_17011_ = _e216;
                            phi_16917_ = _e688;
                            phi_5768_ = vec3<f32>();
                            phi_5769_ = vec3<f32>();
                            phi_5770_ = f32();
                            phi_5771_ = f32();
                            phi_5772_ = f32();
                            phi_5773_ = vec3<f32>();
                            phi_5774_ = type_14();
                            phi_5775_ = u32();
                            phi_5776_ = vec3<f32>();
                            phi_335_ = vec3<f32>();
                            phi_338_ = vec3<f32>();
                            phi_340_ = f32();
                            phi_342_ = vec3<f32>();
                            phi_344_ = u32();
                            phi_346_ = bool();
                            phi_348_ = bool();
                            phi_350_ = vec3<f32>();
                            phi_5782_ = false;
                            phi_5783_ = false;
                            phi_5784_ = false;
                            phi_5785_ = false;
                            phi_5786_ = false;
                            phi_5787_ = false;
                            phi_5788_ = false;
                            phi_5789_ = false;
                        } else {
                            if ((_e211 & 8u) == 0u) {
                                phi_1316_ = _e220;
                                phi_1317_ = _e222;
                            } else {
                                if (_e854.member > 100f) {
                                    let _e862 = global_1.member.member_8;
                                    let _e875 = global_1.member.member_9;
                                    let _e886 = (abs(atan2((((_e226.x * _e875.x) + (_e226.y * _e875.y)) + (_e226.z * _e875.z)), (((_e226.x * _e862.x) + (_e226.y * _e862.y)) + (_e226.z * _e862.z)))) * 0.31830987f);
                                    let _e888 = select(_e886, 0f, (_e886 < 0f));
                                    let _e891 = (select(_e888, 1f, (_e888 > 1f)) * 31f);
                                    let _e893 = ((_e226.y + 1f) * 0.5f);
                                    let _e895 = select(_e893, 0f, (_e893 < 0f));
                                    let _e898 = (select(_e895, 1f, (_e895 > 1f)) * 63f);
                                    let _e901 = (log((_e854.member * 0.01f)) * 0.13156332f);
                                    let _e903 = select(_e901, 0f, (_e901 < 0f));
                                    let _e906 = (select(_e903, 1f, (_e903 > 1f)) * 31f);
                                    let _e911 = select(select(u32(_e891), 0u, (_e891 < 0f)), 4294967295u, (_e891 > 4294967000f));
                                    let _e913 = select(_e911, 30u, (30u < _e911));
                                    let _e918 = select(select(u32(_e898), 0u, (_e898 < 0f)), 4294967295u, (_e898 > 4294967000f));
                                    let _e920 = select(_e918, 62u, (62u < _e918));
                                    let _e925 = select(select(u32(_e906), 0u, (_e906 < 0f)), 4294967295u, (_e906 > 4294967000f));
                                    let _e927 = select(_e925, 30u, (30u < _e925));
                                    let _e929 = (_e891 - f32(_e913));
                                    let _e931 = (_e898 - f32(_e920));
                                    let _e933 = (_e906 - f32(_e927));
                                    phi_1132_ = vec3<f32>(0f, 0f, 0f);
                                    phi_1135_ = vec3<f32>(0f, 0f, 0f);
                                    phi_1137_ = 0u;
                                    loop {
                                        let _e935 = phi_1132_;
                                        let _e937 = phi_1135_;
                                        let _e939 = phi_1137_;
                                        local_7 = _e935;
                                        local_8 = _e935;
                                        local_9 = _e935;
                                        local_10 = _e937;
                                        let _e940 = (_e939 < 8u);
                                        break_if_1_ = _e940;
                                        if _e940 {
                                            let _e941 = (_e939 & 1u);
                                            let _e944 = ((_e939 >> bitcast<u32>(1i)) & 1u);
                                            let _e947 = ((_e939 >> bitcast<u32>(2i)) & 1u);
                                            if (_e941 == 1u) {
                                                phi_1155_ = _e929;
                                            } else {
                                                phi_1155_ = (1f - _e929);
                                            }
                                            let _e951 = phi_1155_;
                                            if (_e944 == 1u) {
                                                phi_1161_ = _e931;
                                            } else {
                                                phi_1161_ = (1f - _e931);
                                            }
                                            let _e955 = phi_1161_;
                                            if (_e947 == 1u) {
                                                phi_1168_ = _e933;
                                            } else {
                                                phi_1168_ = (1f - _e933);
                                            }
                                            let _e960 = phi_1168_;
                                            let _e961 = ((_e951 * _e955) * _e960);
                                            let _e967 = global_1.member.member_7[2u];
                                            let _e973 = (_e967 + (6u * (((((_e913 + _e941) * 64u) + (_e920 + _e944)) * 32u) + (_e927 + _e947))));
                                            let _e975 = global_6.member[_e973];
                                            let _e978 = global_6.member[(_e973 + 1u)];
                                            let _e981 = global_6.member[(_e973 + 2u)];
                                            let _e984 = global_6.member[(_e973 + 3u)];
                                            let _e987 = global_6.member[(_e973 + 4u)];
                                            let _e990 = global_6.member[(_e973 + 5u)];
                                            phi_1133_ = vec3<f32>((_e935.x + (_e984 * _e961)), (_e935.y + (_e987 * _e961)), (_e935.z + (_e990 * _e961)));
                                            phi_1136_ = vec3<f32>((_e937.x + (_e975 * _e961)), (_e937.y + (_e978 * _e961)), (_e937.z + (_e981 * _e961)));
                                            phi_1138_ = (_e939 + 1u);
                                        } else {
                                            phi_1133_ = vec3<f32>();
                                            phi_1136_ = vec3<f32>();
                                            phi_1138_ = u32();
                                        }
                                        let _e1013 = phi_1133_;
                                        let _e1015 = phi_1136_;
                                        let _e1017 = phi_1138_;
                                        continue;
                                        continuing {
                                            phi_1132_ = _e1013;
                                            phi_1135_ = _e1015;
                                            phi_1137_ = _e1017;
                                            let _e7322 = break_if_1_;
                                            break if !(_e7322);
                                        }
                                    }
                                    let _e1021 = local_7;
                                    let _e1026 = local_8;
                                    let _e1031 = local_9;
                                    let _e1042 = local_10;
                                    phi_1314_ = vec3<f32>((_e220.x + (_e222.x * _e1021.x)), (_e220.y + (_e222.y * _e1026.y)), (_e220.z + (_e222.z * _e1031.z)));
                                    phi_1315_ = (_e222 * _e1042);
                                } else {
                                    phi_1314_ = _e220;
                                    phi_1315_ = _e222;
                                }
                                let _e1045 = phi_1314_;
                                let _e1047 = phi_1315_;
                                phi_1316_ = _e1045;
                                phi_1317_ = _e1047;
                            }
                            let _e1049 = phi_1316_;
                            let _e1051 = phi_1317_;
                            let _e1054 = (_e226.x * _e854.member);
                            let _e1056 = (_e226.y * _e854.member);
                            let _e1058 = (_e226.z * _e854.member);
                            let _e1060 = (_e234.x + _e1054);
                            let _e1062 = (_e234.y + _e1056);
                            let _e1064 = (_e234.z + _e1058);
                            let _e1068 = global_2.member[_e854.member_1];
                            phi_1348_ = 0i;
                            phi_1351_ = _e1068.member_6.y;
                            loop {
                                let _e1072 = phi_1348_;
                                let _e1074 = phi_1351_;
                                local_11 = _e1074;
                                local_56 = _e1074;
                                if (_e1072 < 4i) {
                                    let _e1077 = global_4.member[_e1074];
                                    let _e1079 = (_e1077.member != 6u);
                                    if _e1079 {
                                        phi_1420_ = i32();
                                        phi_1421_ = u32();
                                    } else {
                                        let _e1083 = (abs(_e1060) / _e1077.member_6.z);
                                        let _e1094 = (abs(_e1064) / _e1077.member_6.w);
                                        if ((select(select(u32(_e1083), 0u, (_e1083 < 0f)), 4294967295u, (_e1083 > 4294967000f)) % 2u) == 0u) {
                                            phi_1405_ = (_e1060 > 0f);
                                        } else {
                                            phi_1405_ = (_e1060 <= 0f);
                                        }
                                        let _e1105 = phi_1405_;
                                        if ((select(select(u32(_e1094), 0u, (_e1094 < 0f)), 4294967295u, (_e1094 > 4294967000f)) % 2u) == 0u) {
                                            phi_1411_ = (_e1064 > 0f);
                                        } else {
                                            phi_1411_ = (_e1064 <= 0f);
                                        }
                                        let _e1109 = phi_1411_;
                                        if (_e1105 != _e1109) {
                                            phi_1418_ = _e1077.member_1;
                                        } else {
                                            phi_1418_ = _e1077.member_2;
                                        }
                                        let _e1114 = phi_1418_;
                                        phi_1420_ = (_e1072 + 1i);
                                        phi_1421_ = _e1114;
                                    }
                                    let _e1117 = phi_1420_;
                                    let _e1119 = phi_1421_;
                                    phi_1349_ = _e1117;
                                    phi_1352_ = _e1119;
                                    phi_1423_ = select(true, false, _e1079);
                                } else {
                                    phi_1349_ = i32();
                                    phi_1352_ = u32();
                                    phi_1423_ = false;
                                }
                                let _e1122 = phi_1349_;
                                let _e1124 = phi_1352_;
                                let _e1126 = phi_1423_;
                                break_if_2_ = _e1126;
                                continue;
                                continuing {
                                    phi_1348_ = _e1122;
                                    phi_1351_ = _e1124;
                                    let _e7325 = break_if_2_;
                                    break if !(_e7325);
                                }
                            }
                            let _e1129 = local_11;
                            let _e1131 = global_4.member[_e1129];
                            let _e1133 = (_e1131.member == 4u);
                            if _e1133 {
                                phi_17012_ = _e216;
                                phi_16918_ = _e688;
                                phi_5749_ = vec3<f32>();
                                phi_5752_ = vec3<f32>();
                                phi_5753_ = vec3<f32>();
                                phi_5754_ = f32();
                                phi_5755_ = vec3<f32>();
                                phi_5756_ = u32();
                                phi_5757_ = bool();
                                phi_5758_ = bool();
                                phi_5759_ = vec3<f32>();
                                phi_5761_ = false;
                                phi_5762_ = false;
                                phi_5763_ = false;
                                phi_5764_ = false;
                                phi_5765_ = false;
                                phi_5766_ = false;
                                phi_5767_ = false;
                            } else {
                                let _e1134 = -(_e226.x);
                                let _e1135 = -(_e226.y);
                                let _e1136 = -(_e226.z);
                                let _e1137 = -(_e226);
                                if ((((_e854.member_2.x * _e1134) + (_e854.member_2.y * _e1135)) + (_e854.member_2.z * _e1136)) < 0f) {
                                    phi_1468_ = -(_e854.member_2);
                                } else {
                                    phi_1468_ = _e854.member_2;
                                }
                                let _e1150 = phi_1468_;
                                if ((((_e854.member_3.x * _e1134) + (_e854.member_3.y * _e1135)) + (_e854.member_3.z * _e1136)) < 0f) {
                                    phi_1485_ = -(_e854.member_3);
                                } else {
                                    phi_1485_ = _e854.member_3;
                                }
                                let _e1163 = phi_1485_;
                                let _e1166 = global_1.member.member_4[2u];
                                let _e1167 = abs(_e1060);
                                let _e1168 = abs(_e1062);
                                let _e1169 = abs(_e1064);
                                let _e1171 = select(_e1168, _e1167, (_e1167 > _e1168));
                                let _e1175 = (0.00002f * (1f + select(_e1169, _e1171, (_e1171 > _e1169))));
                                if (_e1166 != _e1166) {
                                    phi_8105_ = true;
                                } else {
                                    phi_8105_ = (_e1175 >= _e1166);
                                }
                                let _e1179 = phi_8105_;
                                let _e1181 = (select(_e1166, _e1175, _e1179) * 2f);
                                let _e1188 = (_e1060 + (_e1150.x * _e1181));
                                let _e1189 = (_e1062 + (_e1150.y * _e1181));
                                let _e1190 = (_e1064 + (_e1150.z * _e1181));
                                let _e1191 = vec3<f32>(_e1188, _e1189, _e1190);
                                if _e230 {
                                    let _e1194 = global_1.member.member_4[0u];
                                    phi_1516_ = _e1194;
                                } else {
                                    phi_1516_ = 0f;
                                }
                                let _e1196 = phi_1516_;
                                let _e1199 = select(-1f, 1f, (_e1163.z >= 0f));
                                let _e1201 = (-1f / (_e1199 + _e1163.z));
                                let _e1205 = ((_e1163.x * _e1163.y) * _e1201);
                                let _e1209 = (1f + (((_e1199 * _e1163.x) * _e1163.x) * _e1201));
                                let _e1210 = (_e1199 * _e1205);
                                let _e1212 = (-(_e1199) * _e1163.x);
                                let _e1215 = (_e1199 + ((_e1163.y * _e1163.y) * _e1201));
                                let _e1216 = -(_e1163.y);
                                if (_e1131.member == 0u) {
                                    phi_13990_ = vec3<f32>(0f, 0f, 0f);
                                    phi_13799_ = 1.5f;
                                    phi_13721_ = vec3<f32>(0f, 0f, 0f);
                                    phi_13645_ = vec3<f32>(_e1131.member_4.x, _e1131.member_4.y, _e1131.member_4.z);
                                    phi_13567_ = 0u;
                                    phi_13486_ = 1f;
                                    phi_13410_ = 0u;
                                    phi_13395_ = _e216;
                                    phi_2207_ = vec3<f32>();
                                    phi_2208_ = vec3<f32>();
                                    phi_2209_ = u32();
                                    phi_2210_ = vec3<f32>();
                                    phi_2211_ = true;
                                    phi_2212_ = false;
                                    phi_2213_ = false;
                                    phi_2214_ = false;
                                } else {
                                    if (_e1131.member == 1u) {
                                        let _e1765 = (_e1131.member_6.y * _e1131.member_6.y);
                                        if (_e1765 != _e1765) {
                                            phi_8797_ = true;
                                        } else {
                                            phi_8797_ = (_e1196 >= _e1765);
                                        }
                                        let _e1769 = phi_8797_;
                                        let _e1770 = select(_e1765, _e1196, _e1769);
                                        let _e1771 = (_e1770 >= 0.001f);
                                        if _e1771 {
                                            phi_14047_ = vec3<f32>(_e1131.member_4.x, _e1131.member_4.y, _e1131.member_4.z);
                                            phi_13842_ = _e1131.member_6.x;
                                            phi_13697_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13389_ = _e216;
                                            phi_1591_ = vec3<f32>();
                                            phi_1592_ = true;
                                        } else {
                                            let _e1773 = ((_e216 * 747796405u) + 2891336453u);
                                            let _e1781 = (((_e1773 >> bitcast<u32>((((_e1773 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1773) * 277803737u);
                                            let _e1784 = ((_e1781 >> bitcast<u32>(22i)) ^ _e1781);
                                            let _e1794 = abs((((_e1134 * _e1163.x) + (_e1135 * _e1163.y)) + (_e1136 * _e1163.z)));
                                            let _e1797 = (1f / _e1131.member_6.x);
                                            let _e1801 = ((_e1797 * _e1797) * (1f - (_e1794 * _e1794)));
                                            if (_e1801 >= 1f) {
                                                phi_8837_ = 1f;
                                            } else {
                                                let _e1804 = sqrt((1f - _e1801));
                                                if (_e1794 != _e1794) {
                                                    phi_8848_ = true;
                                                } else {
                                                    phi_8848_ = (_e1804 <= _e1794);
                                                }
                                                let _e1808 = phi_8848_;
                                                let _e1813 = pow(((1f - _e1131.member_6.x) / (1f + _e1131.member_6.x)), 2f);
                                                phi_8837_ = (_e1813 + ((1f - _e1813) * pow((1f - select(_e1794, _e1804, _e1808)), 5f)));
                                            }
                                            let _e1820 = phi_8837_;
                                            let _e1821 = ((f32((_e1784 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1820);
                                            if _e1821 {
                                                let _e1834 = (((_e1163.x * _e226.x) + (_e1163.y * _e226.y)) + (_e1163.z * _e226.z));
                                                phi_13698_ = vec3<f32>(0f, 0f, 0f);
                                                phi_1589_ = (_e226 - vec3<f32>(((_e1163.x * 2f) * _e1834), ((_e1163.y * 2f) * _e1834), ((_e1163.z * 2f) * _e1834)));
                                            } else {
                                                phi_13698_ = vec3<f32>(_e1131.member_4.x, _e1131.member_4.y, _e1131.member_4.z);
                                                phi_1589_ = vec3<f32>();
                                            }
                                            let _e1841 = phi_13698_;
                                            let _e1843 = phi_1589_;
                                            phi_14047_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13842_ = 1.5f;
                                            phi_13697_ = _e1841;
                                            phi_13389_ = _e1784;
                                            phi_1591_ = _e1843;
                                            phi_1592_ = select(true, false, _e1821);
                                        }
                                        let _e1853 = phi_14047_;
                                        let _e1855 = phi_13842_;
                                        let _e1857 = phi_13697_;
                                        let _e1859 = phi_13389_;
                                        let _e1861 = phi_1591_;
                                        let _e1863 = phi_1592_;
                                        phi_13993_ = _e1853;
                                        phi_13802_ = _e1855;
                                        phi_13724_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13648_ = _e1857;
                                        phi_13570_ = select(0u, 1u, _e1771);
                                        phi_13489_ = select(1f, _e1770, _e1771);
                                        phi_13413_ = select(0u, 2u, _e1771);
                                        phi_13340_ = _e1859;
                                        phi_2077_ = _e1861;
                                        phi_2078_ = vec3<f32>(1f, 1f, 1f);
                                        phi_2079_ = _e1191;
                                        phi_2080_ = _e1863;
                                    } else {
                                        if (_e1131.member == 3u) {
                                            phi_13343_ = _e216;
                                            phi_1756_ = vec3<f32>();
                                            phi_1757_ = vec3<f32>();
                                            phi_1758_ = vec3<f32>();
                                            phi_1759_ = true;
                                            phi_1760_ = false;
                                        } else {
                                            let _e1220 = (_e1131.member == 5u);
                                            if _e1220 {
                                                phi_13344_ = _e216;
                                                phi_1752_ = vec3<f32>();
                                                phi_1753_ = vec3<f32>();
                                                phi_1754_ = vec3<f32>();
                                                phi_1755_ = false;
                                            } else {
                                                let _e1221 = (_e1131.member == 2u);
                                                if _e1221 {
                                                    let _e1226 = (((_e226.x * _e854.member_3.x) + (_e226.y * _e854.member_3.y)) + (_e226.z * _e854.member_3.z));
                                                    let _e1227 = (_e1226 > 0f);
                                                    if _e1227 {
                                                        phi_1628_ = -(_e854.member_3);
                                                    } else {
                                                        phi_1628_ = _e854.member_3;
                                                    }
                                                    let _e1230 = phi_1628_;
                                                    if _e1227 {
                                                        phi_1634_ = _e1131.member_6.x;
                                                    } else {
                                                        phi_1634_ = (1f / _e1131.member_6.x);
                                                    }
                                                    let _e1235 = phi_1634_;
                                                    let _e1236 = abs(_e1226);
                                                    let _e1237 = (_e1235 * _e1235);
                                                    let _e1240 = (_e1237 * (1f - (_e1236 * _e1236)));
                                                    if (_e1240 >= 1f) {
                                                        phi_8167_ = 1f;
                                                    } else {
                                                        let _e1243 = sqrt((1f - _e1240));
                                                        if (_e1236 != _e1236) {
                                                            phi_8178_ = true;
                                                        } else {
                                                            phi_8178_ = (_e1243 <= _e1236);
                                                        }
                                                        let _e1247 = phi_8178_;
                                                        let _e1252 = pow(((1f - _e1131.member_6.x) / (1f + _e1131.member_6.x)), 2f);
                                                        phi_8167_ = (_e1252 + ((1f - _e1252) * pow((1f - select(_e1236, _e1243, _e1247)), 5f)));
                                                    }
                                                    let _e1259 = phi_8167_;
                                                    let _e1267 = (((_e226.x * _e1230.x) + (_e226.y * _e1230.y)) + (_e226.z * _e1230.z));
                                                    let _e1271 = (1f - (_e1237 * (1f - (_e1267 * _e1267))));
                                                    if (_e1271 > 0f) {
                                                        let _e1274 = ((_e216 * 747796405u) + 2891336453u);
                                                        let _e1282 = (((_e1274 >> bitcast<u32>((((_e1274 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1274) * 277803737u);
                                                        let _e1285 = ((_e1282 >> bitcast<u32>(22i)) ^ _e1282);
                                                        let _e1290 = ((f32((_e1285 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1259);
                                                        if _e1290 {
                                                            let _e1300 = sqrt(_e1271);
                                                            let _e1304 = (((_e226.x - (_e1230.x * _e1267)) * _e1235) - (_e1230.x * _e1300));
                                                            let _e1305 = (((_e226.y - (_e1230.y * _e1267)) * _e1235) - (_e1230.y * _e1300));
                                                            let _e1306 = (((_e226.z - (_e1230.z * _e1267)) * _e1235) - (_e1230.z * _e1300));
                                                            let _e1313 = (1f / sqrt((((_e1304 * _e1304) + (_e1305 * _e1305)) + (_e1306 * _e1306))));
                                                            phi_1724_ = vec3<f32>((_e1304 * _e1313), (_e1305 * _e1313), (_e1306 * _e1313));
                                                            phi_1725_ = vec3<f32>(_e1131.member_4.x, _e1131.member_4.y, _e1131.member_4.z);
                                                            phi_1726_ = vec3<f32>((_e1060 - (_e1230.x * _e1181)), (_e1062 - (_e1230.y * _e1181)), (_e1064 - (_e1230.z * _e1181)));
                                                        } else {
                                                            phi_1724_ = vec3<f32>();
                                                            phi_1725_ = vec3<f32>();
                                                            phi_1726_ = vec3<f32>();
                                                        }
                                                        let _e1331 = phi_1724_;
                                                        let _e1333 = phi_1725_;
                                                        let _e1335 = phi_1726_;
                                                        phi_13347_ = _e1285;
                                                        phi_1728_ = _e1331;
                                                        phi_1729_ = _e1333;
                                                        phi_1730_ = _e1335;
                                                        phi_1731_ = select(true, false, _e1290);
                                                    } else {
                                                        phi_13347_ = _e216;
                                                        phi_1728_ = vec3<f32>();
                                                        phi_1729_ = vec3<f32>();
                                                        phi_1730_ = vec3<f32>();
                                                        phi_1731_ = true;
                                                    }
                                                    let _e1338 = phi_13347_;
                                                    let _e1340 = phi_1728_;
                                                    let _e1342 = phi_1729_;
                                                    let _e1344 = phi_1730_;
                                                    let _e1346 = phi_1731_;
                                                    if _e1346 {
                                                        let _e1354 = (((_e854.member_3.x * _e226.x) + (_e854.member_3.y * _e226.y)) + (_e854.member_3.z * _e226.z));
                                                        phi_1746_ = (_e226 - vec3<f32>(((_e854.member_3.x * 2f) * _e1354), ((_e854.member_3.y * 2f) * _e1354), ((_e854.member_3.z * 2f) * _e1354)));
                                                        phi_1748_ = vec3<f32>((_e1060 + (_e1230.x * _e1181)), (_e1062 + (_e1230.y * _e1181)), (_e1064 + (_e1230.z * _e1181)));
                                                    } else {
                                                        phi_1746_ = _e1340;
                                                        phi_1748_ = _e1344;
                                                    }
                                                    let _e1368 = phi_1746_;
                                                    let _e1370 = phi_1748_;
                                                    phi_13345_ = _e1338;
                                                    phi_1749_ = _e1368;
                                                    phi_1750_ = select(_e1342, vec3<f32>(1f, 1f, 1f), vec3(_e1346));
                                                    phi_1751_ = _e1370;
                                                } else {
                                                    phi_13345_ = _e216;
                                                    phi_1749_ = vec3<f32>();
                                                    phi_1750_ = vec3<f32>();
                                                    phi_1751_ = vec3<f32>();
                                                }
                                                let _e1374 = phi_13345_;
                                                let _e1376 = phi_1749_;
                                                let _e1378 = phi_1750_;
                                                let _e1380 = phi_1751_;
                                                phi_13344_ = _e1374;
                                                phi_1752_ = _e1376;
                                                phi_1753_ = _e1378;
                                                phi_1754_ = _e1380;
                                                phi_1755_ = _e1221;
                                            }
                                            let _e1382 = phi_13344_;
                                            let _e1384 = phi_1752_;
                                            let _e1386 = phi_1753_;
                                            let _e1388 = phi_1754_;
                                            let _e1390 = phi_1755_;
                                            phi_13343_ = _e1382;
                                            phi_1756_ = _e1384;
                                            phi_1757_ = _e1386;
                                            phi_1758_ = _e1388;
                                            phi_1759_ = _e1220;
                                            phi_1760_ = _e1390;
                                        }
                                        let _e1392 = phi_13343_;
                                        let _e1394 = phi_1756_;
                                        let _e1396 = phi_1757_;
                                        let _e1398 = phi_1758_;
                                        let _e1400 = phi_1759_;
                                        let _e1402 = phi_1760_;
                                        if _e1400 {
                                            let _e1405 = (_e1131.member_6.y * _e1131.member_6.y);
                                            if (_e1405 != _e1405) {
                                                phi_8251_ = true;
                                            } else {
                                                phi_8251_ = (_e1196 >= _e1405);
                                            }
                                            let _e1409 = phi_8251_;
                                            let _e1410 = select(_e1405, _e1196, _e1409);
                                            let _e1422 = select(2u, 0u, (_e1131.member == 5u));
                                            let _e1423 = (_e1410 < 0.001f);
                                            if _e1423 {
                                                let _e1431 = (((_e1163.x * _e226.x) + (_e1163.y * _e226.y)) + (_e1163.z * _e226.z));
                                                let _e1442 = abs((((_e1134 * _e1163.x) + (_e1135 * _e1163.y)) + (_e1136 * _e1163.z)));
                                                if (_e1422 == 0u) {
                                                    let _e1724 = (1f - _e1442);
                                                    if (_e1724 != _e1724) {
                                                        phi_8782_ = true;
                                                    } else {
                                                        phi_8782_ = (0f >= _e1724);
                                                    }
                                                    let _e1728 = phi_8782_;
                                                    let _e1730 = pow(select(_e1724, 0f, _e1728), 5f);
                                                    let _e1731 = (1f - _e1730);
                                                    phi_2063_ = vec3<f32>(((_e1131.member_4.x * _e1731) + _e1730), ((_e1131.member_4.y * _e1731) + _e1730), ((_e1131.member_4.z * _e1731) + _e1730));
                                                } else {
                                                    if (_e1422 == 1u) {
                                                        let _e1706 = (0.44444448f * (1f - (_e1442 * _e1442)));
                                                        if (_e1706 >= 1f) {
                                                            phi_8743_ = 1f;
                                                        } else {
                                                            let _e1709 = sqrt((1f - _e1706));
                                                            if (_e1442 != _e1442) {
                                                                phi_8754_ = true;
                                                            } else {
                                                                phi_8754_ = (_e1709 <= _e1442);
                                                            }
                                                            let _e1713 = phi_8754_;
                                                            phi_8743_ = (0.040000003f + (0.96f * pow((1f - select(_e1442, _e1709, _e1713)), 5f)));
                                                        }
                                                        let _e1720 = phi_8743_;
                                                        phi_2062_ = vec3(_e1720);
                                                    } else {
                                                        let _e1446 = select(_e1442, 0f, (_e1442 < 0f));
                                                        let _e1448 = select(_e1446, 1f, (_e1446 > 1f));
                                                        let _e1450 = (1f - (_e1448 * _e1448));
                                                        let _e1457 = ((_e1131.member_4.x * _e1131.member_4.x) - (_e1131.member_5.x * _e1131.member_5.x));
                                                        let _e1458 = ((_e1131.member_4.y * _e1131.member_4.y) - (_e1131.member_5.y * _e1131.member_5.y));
                                                        let _e1459 = ((_e1131.member_4.z * _e1131.member_4.z) - (_e1131.member_5.z * _e1131.member_5.z));
                                                        let _e1461 = (_e1131.member_4.x * (_e1131.member_5.x + _e1131.member_5.x));
                                                        let _e1463 = (_e1131.member_4.y * (_e1131.member_5.y + _e1131.member_5.y));
                                                        let _e1465 = (_e1131.member_4.z * (_e1131.member_5.z + _e1131.member_5.z));
                                                        let _e1472 = ((_e1457 * _e1457) + (_e1461 * _e1461));
                                                        let _e1473 = ((_e1458 * _e1458) + (_e1463 * _e1463));
                                                        let _e1474 = ((_e1459 * _e1459) + (_e1465 * _e1465));
                                                        let _e1479 = ((_e1450 * _e1457) / _e1472);
                                                        let _e1480 = ((_e1450 * _e1458) / _e1473);
                                                        let _e1481 = ((_e1450 * _e1459) / _e1474);
                                                        let _e1482 = (1f - _e1479);
                                                        let _e1483 = (1f - _e1480);
                                                        let _e1484 = (1f - _e1481);
                                                        let _e1486 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1479, _e1480, _e1481));
                                                        let _e1487 = (_e1450 * _e1461);
                                                        let _e1488 = (_e1450 * _e1463);
                                                        let _e1489 = (_e1450 * _e1465);
                                                        let _e1494 = (vec3<f32>(_e1487, _e1488, _e1489) / vec3<f32>(_e1472, _e1473, _e1474));
                                                        let _e1507 = ((_e1486.x * _e1486.x) + (_e1494.x * _e1494.x));
                                                        let _e1508 = ((_e1486.y * _e1486.y) + (_e1494.y * _e1494.y));
                                                        let _e1509 = ((_e1486.z * _e1486.z) + (_e1494.z * _e1494.z));
                                                        if (_e1507 != _e1507) {
                                                            phi_8413_ = true;
                                                        } else {
                                                            phi_8413_ = (0f >= _e1507);
                                                        }
                                                        let _e1513 = phi_8413_;
                                                        let _e1515 = sqrt(select(_e1507, 0f, _e1513));
                                                        if (_e1508 != _e1508) {
                                                            phi_8428_ = true;
                                                        } else {
                                                            phi_8428_ = (0f >= _e1508);
                                                        }
                                                        let _e1519 = phi_8428_;
                                                        let _e1521 = sqrt(select(_e1508, 0f, _e1519));
                                                        if (_e1509 != _e1509) {
                                                            phi_8443_ = true;
                                                        } else {
                                                            phi_8443_ = (0f >= _e1509);
                                                        }
                                                        let _e1525 = phi_8443_;
                                                        let _e1527 = sqrt(select(_e1509, 0f, _e1525));
                                                        let _e1531 = ((_e1515 + _e1482) * 0.5f);
                                                        let _e1532 = ((_e1521 + _e1483) * 0.5f);
                                                        let _e1533 = ((_e1527 + _e1484) * 0.5f);
                                                        if (_e1531 != _e1531) {
                                                            phi_8470_ = true;
                                                        } else {
                                                            phi_8470_ = (0f >= _e1531);
                                                        }
                                                        let _e1537 = phi_8470_;
                                                        let _e1539 = sqrt(select(_e1531, 0f, _e1537));
                                                        if (_e1532 != _e1532) {
                                                            phi_8485_ = true;
                                                        } else {
                                                            phi_8485_ = (0f >= _e1532);
                                                        }
                                                        let _e1543 = phi_8485_;
                                                        let _e1545 = sqrt(select(_e1532, 0f, _e1543));
                                                        if (_e1533 != _e1533) {
                                                            phi_8500_ = true;
                                                        } else {
                                                            phi_8500_ = (0f >= _e1533);
                                                        }
                                                        let _e1549 = phi_8500_;
                                                        let _e1551 = sqrt(select(_e1533, 0f, _e1549));
                                                        let _e1552 = vec3<f32>(_e1539, _e1545, _e1551);
                                                        let _e1556 = ((_e1515 - _e1482) * 0.5f);
                                                        let _e1557 = ((_e1521 - _e1483) * 0.5f);
                                                        let _e1558 = ((_e1527 - _e1484) * 0.5f);
                                                        if (_e1556 != _e1556) {
                                                            phi_8527_ = true;
                                                        } else {
                                                            phi_8527_ = (0f >= _e1556);
                                                        }
                                                        let _e1562 = phi_8527_;
                                                        let _e1564 = sqrt(select(_e1556, 0f, _e1562));
                                                        if (_e1557 != _e1557) {
                                                            phi_8542_ = true;
                                                        } else {
                                                            phi_8542_ = (0f >= _e1557);
                                                        }
                                                        let _e1568 = phi_8542_;
                                                        let _e1570 = sqrt(select(_e1557, 0f, _e1568));
                                                        if (_e1558 != _e1558) {
                                                            phi_8557_ = true;
                                                        } else {
                                                            phi_8557_ = (0f >= _e1558);
                                                        }
                                                        let _e1574 = phi_8557_;
                                                        let _e1576 = sqrt(select(_e1558, 0f, _e1574));
                                                        if ((_e1487 / _e1472) < 0f) {
                                                            phi_1939_ = -(_e1564);
                                                        } else {
                                                            phi_1939_ = _e1564;
                                                        }
                                                        let _e1580 = phi_1939_;
                                                        if ((_e1488 / _e1473) < 0f) {
                                                            phi_1947_ = -(_e1570);
                                                        } else {
                                                            phi_1947_ = _e1570;
                                                        }
                                                        let _e1584 = phi_1947_;
                                                        if ((_e1489 / _e1474) < 0f) {
                                                            phi_1955_ = -(_e1576);
                                                        } else {
                                                            phi_1955_ = _e1576;
                                                        }
                                                        let _e1588 = phi_1955_;
                                                        let _e1592 = (_e1131.member_5.x * _e1448);
                                                        let _e1593 = (_e1131.member_5.y * _e1448);
                                                        let _e1594 = (_e1131.member_5.z * _e1448);
                                                        let _e1604 = vec3<f32>(((_e1131.member_4.x * _e1539) - (_e1131.member_5.x * _e1580)), ((_e1131.member_4.y * _e1545) - (_e1131.member_5.y * _e1584)), ((_e1131.member_4.z * _e1551) - (_e1131.member_5.z * _e1588)));
                                                        let _e1611 = ((_e1131.member_4.x * _e1580) + (_e1131.member_5.x * _e1539));
                                                        let _e1612 = ((_e1131.member_4.y * _e1584) + (_e1131.member_5.y * _e1545));
                                                        let _e1613 = ((_e1131.member_4.z * _e1588) + (_e1131.member_5.z * _e1551));
                                                        let _e1615 = vec3<f32>((_e1131.member_4.x * _e1448), (_e1131.member_4.y * _e1448), (_e1131.member_4.z * _e1448));
                                                        let _e1616 = (_e1615 - _e1552);
                                                        let _e1617 = (_e1592 - _e1580);
                                                        let _e1618 = (_e1593 - _e1584);
                                                        let _e1619 = (_e1594 - _e1588);
                                                        let _e1632 = (_e1615 + _e1552);
                                                        let _e1633 = (_e1592 + _e1580);
                                                        let _e1634 = (_e1593 + _e1584);
                                                        let _e1635 = (_e1594 + _e1588);
                                                        let _e1645 = ((_e1632.x * _e1632.x) + (_e1633 * _e1633));
                                                        let _e1646 = ((_e1632.y * _e1632.y) + (_e1634 * _e1634));
                                                        let _e1647 = ((_e1632.z * _e1632.z) + (_e1635 * _e1635));
                                                        let _e1657 = vec3(_e1448);
                                                        let _e1658 = (_e1657 - _e1604);
                                                        let _e1659 = -(vec3<f32>(_e1611, _e1612, _e1613));
                                                        let _e1675 = (_e1657 + _e1604);
                                                        let _e1685 = ((_e1675.x * _e1675.x) + (_e1611 * _e1611));
                                                        let _e1686 = ((_e1675.y * _e1675.y) + (_e1612 * _e1612));
                                                        let _e1687 = ((_e1675.z * _e1675.z) + (_e1613 * _e1613));
                                                        phi_2062_ = vec3<f32>((((((_e1616.x * _e1616.x) + (_e1617 * _e1617)) / select(0.000000000000000000000000000001f, _e1645, (_e1645 > 0.000000000000000000000000000001f))) + (((_e1658.x * _e1658.x) + (_e1659.x * _e1659.x)) / select(0.000000000000000000000000000001f, _e1685, (_e1685 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1616.y * _e1616.y) + (_e1618 * _e1618)) / select(0.000000000000000000000000000001f, _e1646, (_e1646 > 0.000000000000000000000000000001f))) + (((_e1658.y * _e1658.y) + (_e1659.y * _e1659.y)) / select(0.000000000000000000000000000001f, _e1686, (_e1686 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1616.z * _e1616.z) + (_e1619 * _e1619)) / select(0.000000000000000000000000000001f, _e1647, (_e1647 > 0.000000000000000000000000000001f))) + (((_e1658.z * _e1658.z) + (_e1659.z * _e1659.z)) / select(0.000000000000000000000000000001f, _e1687, (_e1687 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e1723 = phi_2062_;
                                                    phi_2063_ = _e1723;
                                                }
                                                let _e1740 = phi_2063_;
                                                phi_2064_ = (_e226 - vec3<f32>(((_e1163.x * 2f) * _e1431), ((_e1163.y * 2f) * _e1431), ((_e1163.z * 2f) * _e1431)));
                                                phi_2065_ = _e1740;
                                            } else {
                                                phi_2064_ = vec3<f32>();
                                                phi_2065_ = vec3<f32>();
                                            }
                                            let _e1742 = phi_2064_;
                                            let _e1744 = phi_2065_;
                                            phi_13726_ = vec3<f32>(_e1131.member_5.x, _e1131.member_5.y, _e1131.member_5.z);
                                            phi_13650_ = vec3<f32>(_e1131.member_4.x, _e1131.member_4.y, _e1131.member_4.z);
                                            phi_13572_ = _e1422;
                                            phi_13491_ = _e1410;
                                            phi_2066_ = _e1742;
                                            phi_2067_ = _e1744;
                                            phi_2069_ = _e1423;
                                        } else {
                                            phi_13726_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13650_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13572_ = 0u;
                                            phi_13491_ = 1f;
                                            phi_2066_ = _e1394;
                                            phi_2067_ = _e1396;
                                            phi_2069_ = _e1402;
                                        }
                                        let _e1746 = phi_13726_;
                                        let _e1748 = phi_13650_;
                                        let _e1750 = phi_13572_;
                                        let _e1752 = phi_13491_;
                                        let _e1754 = phi_2066_;
                                        let _e1756 = phi_2067_;
                                        let _e1758 = phi_2069_;
                                        phi_13993_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13802_ = 1.5f;
                                        phi_13724_ = _e1746;
                                        phi_13648_ = _e1748;
                                        phi_13570_ = _e1750;
                                        phi_13489_ = _e1752;
                                        phi_13413_ = select(0u, 1u, _e1400);
                                        phi_13340_ = _e1392;
                                        phi_2077_ = _e1754;
                                        phi_2078_ = _e1756;
                                        phi_2079_ = select(_e1398, _e1191, vec3(_e1400));
                                        phi_2080_ = select(true, false, _e1758);
                                    }
                                    let _e1868 = phi_13993_;
                                    let _e1870 = phi_13802_;
                                    let _e1872 = phi_13724_;
                                    let _e1874 = phi_13648_;
                                    let _e1876 = phi_13570_;
                                    let _e1878 = phi_13489_;
                                    let _e1880 = phi_13413_;
                                    let _e1882 = phi_13340_;
                                    let _e1884 = phi_2077_;
                                    let _e1886 = phi_2078_;
                                    let _e1888 = phi_2079_;
                                    let _e1890 = phi_2080_;
                                    let _e1891 = select(true, false, _e1890);
                                    if _e1891 {
                                        let _e1892 = (_e228 >= _e208);
                                        if _e1892 {
                                            phi_13397_ = _e1882;
                                            phi_2195_ = vec3<f32>();
                                            phi_2196_ = vec3<f32>();
                                            phi_2197_ = u32();
                                            phi_2198_ = false;
                                            phi_2199_ = false;
                                        } else {
                                            let _e1904 = global_1.member.member_4[1u];
                                            if (sqrt((((_e1886.x * _e1886.x) + (_e1886.y * _e1886.y)) + (_e1886.z * _e1886.z))) <= _e1904) {
                                                phi_13399_ = _e1882;
                                                phi_2160_ = _e1051;
                                                phi_2161_ = false;
                                            } else {
                                                let _e1908 = global_1.member.member_7[1u];
                                                if (_e1908 == 1u) {
                                                    let _e1913 = global_1.member.member_4[3u];
                                                    if (f32(_e228) >= _e1913) {
                                                        let _e1916 = select(_e1886.y, _e1886.x, (_e1886.x > _e1886.y));
                                                        let _e1918 = select(_e1886.z, _e1916, (_e1916 > _e1886.z));
                                                        if (_e1918 != _e1918) {
                                                            phi_8914_ = true;
                                                        } else {
                                                            phi_8914_ = (1f <= _e1918);
                                                        }
                                                        let _e1922 = phi_8914_;
                                                        let _e1923 = select(_e1918, 1f, _e1922);
                                                        let _e1925 = ((_e1882 * 747796405u) + 2891336453u);
                                                        let _e1933 = (((_e1925 >> bitcast<u32>((((_e1925 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1925) * 277803737u);
                                                        let _e1936 = ((_e1933 >> bitcast<u32>(22i)) ^ _e1933);
                                                        let _e1941 = ((f32((_e1936 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1923);
                                                        if _e1941 {
                                                            phi_2149_ = vec3<f32>();
                                                        } else {
                                                            phi_2149_ = vec3<f32>((_e1051.x / _e1923), (_e1051.y / _e1923), (_e1051.z / _e1923));
                                                        }
                                                        let _e1950 = phi_2149_;
                                                        phi_13402_ = _e1936;
                                                        phi_2151_ = _e1950;
                                                        phi_2152_ = select(true, false, _e1941);
                                                    } else {
                                                        phi_13402_ = _e1882;
                                                        phi_2151_ = _e1051;
                                                        phi_2152_ = true;
                                                    }
                                                    let _e1953 = phi_13402_;
                                                    let _e1955 = phi_2151_;
                                                    let _e1957 = phi_2152_;
                                                    phi_13401_ = _e1953;
                                                    phi_2153_ = _e1955;
                                                    phi_2154_ = _e1957;
                                                } else {
                                                    phi_13401_ = _e1882;
                                                    phi_2153_ = _e1051;
                                                    phi_2154_ = true;
                                                }
                                                let _e1959 = phi_13401_;
                                                let _e1961 = phi_2153_;
                                                let _e1963 = phi_2154_;
                                                phi_13399_ = _e1959;
                                                phi_2160_ = select(_e1051, _e1961, vec3(_e1963));
                                                phi_2161_ = select(false, true, _e1963);
                                            }
                                            let _e1968 = phi_13399_;
                                            let _e1970 = phi_2160_;
                                            let _e1972 = phi_2161_;
                                            if _e1972 {
                                                phi_2191_ = (_e1970 * _e1886);
                                                phi_2192_ = (_e1884 * (1f / sqrt((((_e1884.x * _e1884.x) + (_e1884.y * _e1884.y)) + (_e1884.z * _e1884.z)))));
                                                phi_2193_ = (_e228 + 1u);
                                            } else {
                                                phi_2191_ = vec3<f32>();
                                                phi_2192_ = vec3<f32>();
                                                phi_2193_ = u32();
                                            }
                                            let _e1987 = phi_2191_;
                                            let _e1989 = phi_2192_;
                                            let _e1991 = phi_2193_;
                                            phi_13397_ = _e1968;
                                            phi_2195_ = _e1987;
                                            phi_2196_ = _e1989;
                                            phi_2197_ = _e1991;
                                            phi_2198_ = _e1972;
                                            phi_2199_ = select(true, false, _e1972);
                                        }
                                        let _e1994 = phi_13397_;
                                        let _e1996 = phi_2195_;
                                        let _e1998 = phi_2196_;
                                        let _e2000 = phi_2197_;
                                        let _e2002 = phi_2198_;
                                        let _e2004 = phi_2199_;
                                        phi_13396_ = _e1994;
                                        phi_2200_ = _e1996;
                                        phi_2201_ = _e1998;
                                        phi_2202_ = _e2000;
                                        phi_2204_ = _e2002;
                                        phi_2205_ = _e2004;
                                        phi_2206_ = _e1892;
                                    } else {
                                        phi_13396_ = _e1882;
                                        phi_2200_ = vec3<f32>();
                                        phi_2201_ = vec3<f32>();
                                        phi_2202_ = u32();
                                        phi_2204_ = false;
                                        phi_2205_ = false;
                                        phi_2206_ = false;
                                    }
                                    let _e2006 = phi_13396_;
                                    let _e2008 = phi_2200_;
                                    let _e2010 = phi_2201_;
                                    let _e2012 = phi_2202_;
                                    let _e2014 = phi_2204_;
                                    let _e2016 = phi_2205_;
                                    let _e2018 = phi_2206_;
                                    phi_13990_ = _e1868;
                                    phi_13799_ = _e1870;
                                    phi_13721_ = _e1872;
                                    phi_13645_ = _e1874;
                                    phi_13567_ = _e1876;
                                    phi_13486_ = _e1878;
                                    phi_13410_ = _e1880;
                                    phi_13395_ = _e2006;
                                    phi_2207_ = _e2008;
                                    phi_2208_ = _e2010;
                                    phi_2209_ = _e2012;
                                    phi_2210_ = _e1888;
                                    phi_2211_ = select(true, false, _e1891);
                                    phi_2212_ = _e2014;
                                    phi_2213_ = _e2016;
                                    phi_2214_ = _e2018;
                                }
                                let _e2026 = phi_13990_;
                                let _e2028 = phi_13799_;
                                let _e2030 = phi_13721_;
                                let _e2032 = phi_13645_;
                                let _e2034 = phi_13567_;
                                let _e2036 = phi_13486_;
                                let _e2038 = phi_13410_;
                                let _e2040 = phi_13395_;
                                let _e2042 = phi_2207_;
                                let _e2044 = phi_2208_;
                                let _e2046 = phi_2209_;
                                let _e2048 = phi_2210_;
                                let _e2050 = phi_2211_;
                                let _e2052 = phi_2212_;
                                let _e2054 = phi_2213_;
                                let _e2056 = phi_2214_;
                                if _e2050 {
                                    let _e2059 = global_1.member.member_1[3u];
                                    phi_15151_ = _e688;
                                    phi_13394_ = _e2040;
                                    phi_2223_ = _e1049;
                                    phi_2226_ = 0u;
                                    loop {
                                        let _e2061 = phi_15151_;
                                        let _e2063 = phi_13394_;
                                        let _e2065 = phi_2223_;
                                        let _e2067 = phi_2226_;
                                        local_54 = _e2065;
                                        local_55 = _e2065;
                                        let _e2069 = (_e2067 < (_e2059 + 3u));
                                        break_if_7_ = _e2069;
                                        if _e2069 {
                                            if (_e2067 < _e2059) {
                                                let _e3594 = global_5.member[_e2067];
                                                let _e3599 = (_e3594.member.x - _e1188);
                                                let _e3600 = (_e3594.member.y - _e1189);
                                                let _e3601 = (_e3594.member.z - _e1190);
                                                let _e3606 = (((_e3599 * _e3599) + (_e3600 * _e3600)) + (_e3601 * _e3601));
                                                let _e3607 = sqrt(_e3606);
                                                let _e3608 = (_e3599 / _e3607);
                                                let _e3609 = (_e3600 / _e3607);
                                                let _e3610 = (_e3601 / _e3607);
                                                if (_e3594.member.w < 0.5f) {
                                                    phi_2323_ = 1f;
                                                } else {
                                                    let _e3634 = (((_e3594.member_2.x * -(_e3608)) + (_e3594.member_2.y * -(_e3609))) + (_e3594.member_2.z * -(_e3610)));
                                                    if (_e3594.member_3.x == _e3594.member_2.w) {
                                                        phi_2322_ = select(1f, 0f, (_e3634 < _e3594.member_3.x));
                                                    } else {
                                                        let _e3638 = ((_e3634 - _e3594.member_3.x) / (_e3594.member_2.w - _e3594.member_3.x));
                                                        let _e3640 = select(_e3638, 0f, (_e3638 < 0f));
                                                        let _e3642 = select(_e3640, 1f, (_e3640 > 1f));
                                                        phi_2322_ = ((_e3642 * _e3642) * (3f - (2f * _e3642)));
                                                    }
                                                    let _e3650 = phi_2322_;
                                                    phi_2323_ = _e3650;
                                                }
                                                let _e3652 = phi_2323_;
                                                phi_15490_ = _e2063;
                                                phi_4094_ = vec3<f32>(((_e3594.member_1.x * _e3652) / _e3606), ((_e3594.member_1.y * _e3652) / _e3606), ((_e3594.member_1.z * _e3652) / _e3606));
                                                phi_4095_ = 0f;
                                                phi_4096_ = vec3<f32>(_e3608, _e3609, _e3610);
                                                phi_4097_ = _e3607;
                                            } else {
                                                if (_e2067 == _e2059) {
                                                    if ((_e211 & 4u) == 0u) {
                                                        phi_15529_ = _e2063;
                                                        phi_2883_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_2884_ = 0f;
                                                        phi_2885_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2922 = global_1.member.member_5;
                                                        let _e2929 = global_1.member.member_5[3u];
                                                        let _e2930 = (1f - _e2929);
                                                        let _e2931 = (6.2831855f * _e2930);
                                                        let _e2936 = (((_e2922.x * _e1163.x) + (_e2922.y * _e1163.y)) + (_e2922.z * _e1163.z));
                                                        if (_e2936 != _e2936) {
                                                            phi_9450_ = true;
                                                        } else {
                                                            phi_9450_ = (0f >= _e2936);
                                                        }
                                                        let _e2940 = phi_9450_;
                                                        let _e2942 = (select(_e2936, 0f, _e2940) * 0.31830987f);
                                                        if (_e2038 == 0u) {
                                                            if (_e2936 <= 0f) {
                                                                phi_10247_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_10247_ = (_e2032 * (_e2936 * 0.31830987f));
                                                            }
                                                            let _e3421 = phi_10247_;
                                                            phi_2731_ = type_59(_e3421, _e2942);
                                                            phi_2732_ = type_59();
                                                            phi_2733_ = true;
                                                        } else {
                                                            let _e2951 = (((_e1137.x * _e1209) + (_e1137.y * _e1210)) + (_e1137.z * _e1212));
                                                            let _e2956 = (((_e1137.x * _e1205) + (_e1137.y * _e1215)) + (_e1137.z * _e1216));
                                                            let _e2961 = (((_e1137.x * _e1163.x) + (_e1137.y * _e1163.y)) + (_e1137.z * _e1163.z));
                                                            let _e2966 = (((_e2922.x * _e1209) + (_e2922.y * _e1210)) + (_e2922.z * _e1212));
                                                            let _e2971 = (((_e2922.x * _e1205) + (_e2922.y * _e1215)) + (_e2922.z * _e1216));
                                                            if (_e2961 <= 0f) {
                                                                phi_2695_ = type_59();
                                                                phi_2696_ = true;
                                                            } else {
                                                                let _e2973 = (_e2936 <= 0f);
                                                                if _e2973 {
                                                                    phi_2694_ = type_59();
                                                                } else {
                                                                    let _e2974 = (_e2951 + _e2966);
                                                                    let _e2975 = (_e2956 + _e2971);
                                                                    let _e2976 = (_e2961 + _e2936);
                                                                    let _e2983 = (1f / sqrt((((_e2974 * _e2974) + (_e2975 * _e2975)) + (_e2976 * _e2976))));
                                                                    let _e2986 = (_e2976 * _e2983);
                                                                    if (_e2986 <= 0f) {
                                                                        phi_9560_ = 0f;
                                                                    } else {
                                                                        let _e2988 = (_e2036 * _e2036);
                                                                        let _e2992 = ((((_e2988 - 1f) * _e2986) * _e2986) + 1f);
                                                                        phi_9560_ = (_e2988 / ((3.1415927f * _e2992) * _e2992));
                                                                    }
                                                                    let _e2997 = phi_9560_;
                                                                    let _e2998 = (_e2961 * _e2961);
                                                                    if (_e2998 >= 1f) {
                                                                        phi_9588_ = 0f;
                                                                    } else {
                                                                        phi_9588_ = ((sqrt((1f + (((_e2036 * _e2036) * ((_e2951 * _e2951) + (_e2956 * _e2956))) / _e2998))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3011 = phi_9588_;
                                                                    let _e3012 = (1f + _e3011);
                                                                    let _e3013 = (_e2936 * _e2936);
                                                                    if (_e3013 >= 1f) {
                                                                        phi_9616_ = 0f;
                                                                    } else {
                                                                        phi_9616_ = ((sqrt((1f + (((_e2036 * _e2036) * ((_e2966 * _e2966) + (_e2971 * _e2971))) / _e3013))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3026 = phi_9616_;
                                                                    let _e3033 = (((_e2951 * (_e2974 * _e2983)) + (_e2956 * (_e2975 * _e2983))) + (_e2961 * _e2986));
                                                                    if (_e2034 == 0u) {
                                                                        let _e3328 = (1f - _e3033);
                                                                        if (_e3328 != _e3328) {
                                                                            phi_10119_ = true;
                                                                        } else {
                                                                            phi_10119_ = (0f >= _e3328);
                                                                        }
                                                                        let _e3332 = phi_10119_;
                                                                        let _e3334 = pow(select(_e3328, 0f, _e3332), 5f);
                                                                        let _e3335 = (1f - _e3334);
                                                                        phi_2682_ = vec3<f32>(((_e2032.x * _e3335) + _e3334), ((_e2032.y * _e3335) + _e3334), ((_e2032.z * _e3335) + _e3334));
                                                                    } else {
                                                                        if (_e2034 == 1u) {
                                                                            let _e3301 = (1f / _e2028);
                                                                            let _e3305 = ((_e3301 * _e3301) * (1f - (_e3033 * _e3033)));
                                                                            if (_e3305 >= 1f) {
                                                                                phi_10080_ = 1f;
                                                                            } else {
                                                                                let _e3308 = sqrt((1f - _e3305));
                                                                                if (_e3033 != _e3033) {
                                                                                    phi_10091_ = true;
                                                                                } else {
                                                                                    phi_10091_ = (_e3308 <= _e3033);
                                                                                }
                                                                                let _e3312 = phi_10091_;
                                                                                let _e3317 = pow(((1f - _e2028) / (1f + _e2028)), 2f);
                                                                                phi_10080_ = (_e3317 + ((1f - _e3317) * pow((1f - select(_e3033, _e3308, _e3312)), 5f)));
                                                                            }
                                                                            let _e3324 = phi_10080_;
                                                                            phi_2681_ = vec3(_e3324);
                                                                        } else {
                                                                            let _e3037 = select(_e3033, 0f, (_e3033 < 0f));
                                                                            let _e3039 = select(_e3037, 1f, (_e3037 > 1f));
                                                                            let _e3041 = (1f - (_e3039 * _e3039));
                                                                            let _e3054 = ((_e2032.x * _e2032.x) - (_e2030.x * _e2030.x));
                                                                            let _e3055 = ((_e2032.y * _e2032.y) - (_e2030.y * _e2030.y));
                                                                            let _e3056 = ((_e2032.z * _e2032.z) - (_e2030.z * _e2030.z));
                                                                            let _e3058 = (_e2032.x * (_e2030.x + _e2030.x));
                                                                            let _e3060 = (_e2032.y * (_e2030.y + _e2030.y));
                                                                            let _e3062 = (_e2032.z * (_e2030.z + _e2030.z));
                                                                            let _e3069 = ((_e3054 * _e3054) + (_e3058 * _e3058));
                                                                            let _e3070 = ((_e3055 * _e3055) + (_e3060 * _e3060));
                                                                            let _e3071 = ((_e3056 * _e3056) + (_e3062 * _e3062));
                                                                            let _e3076 = ((_e3041 * _e3054) / _e3069);
                                                                            let _e3077 = ((_e3041 * _e3055) / _e3070);
                                                                            let _e3078 = ((_e3041 * _e3056) / _e3071);
                                                                            let _e3079 = (1f - _e3076);
                                                                            let _e3080 = (1f - _e3077);
                                                                            let _e3081 = (1f - _e3078);
                                                                            let _e3083 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3076, _e3077, _e3078));
                                                                            let _e3084 = (_e3041 * _e3058);
                                                                            let _e3085 = (_e3041 * _e3060);
                                                                            let _e3086 = (_e3041 * _e3062);
                                                                            let _e3091 = (vec3<f32>(_e3084, _e3085, _e3086) / vec3<f32>(_e3069, _e3070, _e3071));
                                                                            let _e3104 = ((_e3083.x * _e3083.x) + (_e3091.x * _e3091.x));
                                                                            let _e3105 = ((_e3083.y * _e3083.y) + (_e3091.y * _e3091.y));
                                                                            let _e3106 = ((_e3083.z * _e3083.z) + (_e3091.z * _e3091.z));
                                                                            if (_e3104 != _e3104) {
                                                                                phi_9750_ = true;
                                                                            } else {
                                                                                phi_9750_ = (0f >= _e3104);
                                                                            }
                                                                            let _e3110 = phi_9750_;
                                                                            let _e3112 = sqrt(select(_e3104, 0f, _e3110));
                                                                            if (_e3105 != _e3105) {
                                                                                phi_9765_ = true;
                                                                            } else {
                                                                                phi_9765_ = (0f >= _e3105);
                                                                            }
                                                                            let _e3116 = phi_9765_;
                                                                            let _e3118 = sqrt(select(_e3105, 0f, _e3116));
                                                                            if (_e3106 != _e3106) {
                                                                                phi_9780_ = true;
                                                                            } else {
                                                                                phi_9780_ = (0f >= _e3106);
                                                                            }
                                                                            let _e3122 = phi_9780_;
                                                                            let _e3124 = sqrt(select(_e3106, 0f, _e3122));
                                                                            let _e3128 = ((_e3112 + _e3079) * 0.5f);
                                                                            let _e3129 = ((_e3118 + _e3080) * 0.5f);
                                                                            let _e3130 = ((_e3124 + _e3081) * 0.5f);
                                                                            if (_e3128 != _e3128) {
                                                                                phi_9807_ = true;
                                                                            } else {
                                                                                phi_9807_ = (0f >= _e3128);
                                                                            }
                                                                            let _e3134 = phi_9807_;
                                                                            let _e3136 = sqrt(select(_e3128, 0f, _e3134));
                                                                            if (_e3129 != _e3129) {
                                                                                phi_9822_ = true;
                                                                            } else {
                                                                                phi_9822_ = (0f >= _e3129);
                                                                            }
                                                                            let _e3140 = phi_9822_;
                                                                            let _e3142 = sqrt(select(_e3129, 0f, _e3140));
                                                                            if (_e3130 != _e3130) {
                                                                                phi_9837_ = true;
                                                                            } else {
                                                                                phi_9837_ = (0f >= _e3130);
                                                                            }
                                                                            let _e3146 = phi_9837_;
                                                                            let _e3148 = sqrt(select(_e3130, 0f, _e3146));
                                                                            let _e3149 = vec3<f32>(_e3136, _e3142, _e3148);
                                                                            let _e3153 = ((_e3112 - _e3079) * 0.5f);
                                                                            let _e3154 = ((_e3118 - _e3080) * 0.5f);
                                                                            let _e3155 = ((_e3124 - _e3081) * 0.5f);
                                                                            if (_e3153 != _e3153) {
                                                                                phi_9864_ = true;
                                                                            } else {
                                                                                phi_9864_ = (0f >= _e3153);
                                                                            }
                                                                            let _e3159 = phi_9864_;
                                                                            let _e3161 = sqrt(select(_e3153, 0f, _e3159));
                                                                            if (_e3154 != _e3154) {
                                                                                phi_9879_ = true;
                                                                            } else {
                                                                                phi_9879_ = (0f >= _e3154);
                                                                            }
                                                                            let _e3165 = phi_9879_;
                                                                            let _e3167 = sqrt(select(_e3154, 0f, _e3165));
                                                                            if (_e3155 != _e3155) {
                                                                                phi_9894_ = true;
                                                                            } else {
                                                                                phi_9894_ = (0f >= _e3155);
                                                                            }
                                                                            let _e3171 = phi_9894_;
                                                                            let _e3173 = sqrt(select(_e3155, 0f, _e3171));
                                                                            if ((_e3084 / _e3069) < 0f) {
                                                                                phi_2558_ = -(_e3161);
                                                                            } else {
                                                                                phi_2558_ = _e3161;
                                                                            }
                                                                            let _e3177 = phi_2558_;
                                                                            if ((_e3085 / _e3070) < 0f) {
                                                                                phi_2566_ = -(_e3167);
                                                                            } else {
                                                                                phi_2566_ = _e3167;
                                                                            }
                                                                            let _e3181 = phi_2566_;
                                                                            if ((_e3086 / _e3071) < 0f) {
                                                                                phi_2574_ = -(_e3173);
                                                                            } else {
                                                                                phi_2574_ = _e3173;
                                                                            }
                                                                            let _e3185 = phi_2574_;
                                                                            let _e3189 = (_e2030.x * _e3039);
                                                                            let _e3190 = (_e2030.y * _e3039);
                                                                            let _e3191 = (_e2030.z * _e3039);
                                                                            let _e3201 = vec3<f32>(((_e2032.x * _e3136) - (_e2030.x * _e3177)), ((_e2032.y * _e3142) - (_e2030.y * _e3181)), ((_e2032.z * _e3148) - (_e2030.z * _e3185)));
                                                                            let _e3208 = ((_e2032.x * _e3177) + (_e2030.x * _e3136));
                                                                            let _e3209 = ((_e2032.y * _e3181) + (_e2030.y * _e3142));
                                                                            let _e3210 = ((_e2032.z * _e3185) + (_e2030.z * _e3148));
                                                                            let _e3212 = vec3<f32>((_e2032.x * _e3039), (_e2032.y * _e3039), (_e2032.z * _e3039));
                                                                            let _e3213 = (_e3212 - _e3149);
                                                                            let _e3214 = (_e3189 - _e3177);
                                                                            let _e3215 = (_e3190 - _e3181);
                                                                            let _e3216 = (_e3191 - _e3185);
                                                                            let _e3229 = (_e3212 + _e3149);
                                                                            let _e3230 = (_e3189 + _e3177);
                                                                            let _e3231 = (_e3190 + _e3181);
                                                                            let _e3232 = (_e3191 + _e3185);
                                                                            let _e3242 = ((_e3229.x * _e3229.x) + (_e3230 * _e3230));
                                                                            let _e3243 = ((_e3229.y * _e3229.y) + (_e3231 * _e3231));
                                                                            let _e3244 = ((_e3229.z * _e3229.z) + (_e3232 * _e3232));
                                                                            let _e3254 = vec3(_e3039);
                                                                            let _e3255 = (_e3254 - _e3201);
                                                                            let _e3256 = -(vec3<f32>(_e3208, _e3209, _e3210));
                                                                            let _e3272 = (_e3254 + _e3201);
                                                                            let _e3282 = ((_e3272.x * _e3272.x) + (_e3208 * _e3208));
                                                                            let _e3283 = ((_e3272.y * _e3272.y) + (_e3209 * _e3209));
                                                                            let _e3284 = ((_e3272.z * _e3272.z) + (_e3210 * _e3210));
                                                                            phi_2681_ = vec3<f32>((((((_e3213.x * _e3213.x) + (_e3214 * _e3214)) / select(0.000000000000000000000000000001f, _e3242, (_e3242 > 0.000000000000000000000000000001f))) + (((_e3255.x * _e3255.x) + (_e3256.x * _e3256.x)) / select(0.000000000000000000000000000001f, _e3282, (_e3282 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3213.y * _e3213.y) + (_e3215 * _e3215)) / select(0.000000000000000000000000000001f, _e3243, (_e3243 > 0.000000000000000000000000000001f))) + (((_e3255.y * _e3255.y) + (_e3256.y * _e3256.y)) / select(0.000000000000000000000000000001f, _e3283, (_e3283 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3213.z * _e3213.z) + (_e3216 * _e3216)) / select(0.000000000000000000000000000001f, _e3244, (_e3244 > 0.000000000000000000000000000001f))) + (((_e3255.z * _e3255.z) + (_e3256.z * _e3256.z)) / select(0.000000000000000000000000000001f, _e3284, (_e3284 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                        }
                                                                        let _e3327 = phi_2681_;
                                                                        phi_2682_ = _e3327;
                                                                    }
                                                                    let _e3347 = phi_2682_;
                                                                    let _e3349 = (4f * _e2961);
                                                                    phi_2694_ = type_59((_e3347 * (((1f / (_e3012 + _e3026)) * _e2997) / _e3349)), ((_e2997 / _e3012) / _e3349));
                                                                }
                                                                let _e3356 = phi_2694_;
                                                                phi_2695_ = _e3356;
                                                                phi_2696_ = _e2973;
                                                            }
                                                            let _e3358 = phi_2695_;
                                                            let _e3360 = phi_2696_;
                                                            if _e3360 {
                                                                phi_2701_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                            } else {
                                                                phi_2701_ = _e3358;
                                                            }
                                                            let _e3362 = phi_2701_;
                                                            let _e3365 = (_e2038 == 1u);
                                                            if _e3365 {
                                                                phi_2729_ = _e3362;
                                                                phi_2730_ = type_59();
                                                            } else {
                                                                if (_e2961 != _e2961) {
                                                                    phi_10156_ = true;
                                                                } else {
                                                                    phi_10156_ = (0f >= _e2961);
                                                                }
                                                                let _e3369 = phi_10156_;
                                                                let _e3370 = select(_e2961, 0f, _e3369);
                                                                let _e3371 = (1f / _e2028);
                                                                let _e3375 = ((_e3371 * _e3371) * (1f - (_e3370 * _e3370)));
                                                                if (_e3375 >= 1f) {
                                                                    phi_10175_ = 1f;
                                                                } else {
                                                                    let _e3378 = sqrt((1f - _e3375));
                                                                    if (_e3370 != _e3370) {
                                                                        phi_10186_ = true;
                                                                    } else {
                                                                        phi_10186_ = (_e3378 <= _e3370);
                                                                    }
                                                                    let _e3382 = phi_10186_;
                                                                    let _e3387 = pow(((1f - _e2028) / (1f + _e2028)), 2f);
                                                                    phi_10175_ = (_e3387 + ((1f - _e3387) * pow((1f - select(_e3370, _e3378, _e3382)), 5f)));
                                                                }
                                                                let _e3394 = phi_10175_;
                                                                if (_e2936 <= 0f) {
                                                                    phi_10225_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    phi_10225_ = (_e2026 * (_e2936 * 0.31830987f));
                                                                }
                                                                let _e3399 = phi_10225_;
                                                                let _e3400 = (1f - _e3394);
                                                                phi_2729_ = type_59();
                                                                phi_2730_ = type_59((_e3362.member + vec3<f32>((_e3399.x * _e3400), (_e3399.y * _e3400), (_e3399.z * _e3400))), ((_e3394 * _e3362.member_1) + (_e3400 * _e2942)));
                                                            }
                                                            let _e3414 = phi_2729_;
                                                            let _e3416 = phi_2730_;
                                                            phi_2731_ = _e3414;
                                                            phi_2732_ = _e3416;
                                                            phi_2733_ = _e3365;
                                                        }
                                                        let _e3424 = phi_2731_;
                                                        let _e3426 = phi_2732_;
                                                        let _e3428 = phi_2733_;
                                                        if _e3428 {
                                                            phi_2737_ = _e3424;
                                                        } else {
                                                            phi_2737_ = _e3426;
                                                        }
                                                        let _e3430 = phi_2737_;
                                                        if ((_e3430.member_1 * _e2931) < 0.001f) {
                                                            let _e3562 = global_1.member.member_6;
                                                            phi_15530_ = _e2063;
                                                            phi_2880_ = vec3<f32>(_e3562.x, _e3562.y, _e3562.z);
                                                            phi_2881_ = 0f;
                                                            phi_2882_ = vec3<f32>(_e2922.x, _e2922.y, _e2922.z);
                                                        } else {
                                                            let _e3435 = ((_e2063 * 747796405u) + 2891336453u);
                                                            let _e3443 = (((_e3435 >> bitcast<u32>((((_e3435 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3435) * 277803737u);
                                                            let _e3446 = ((_e3443 >> bitcast<u32>(22i)) ^ _e3443);
                                                            let _e3452 = (1f - ((f32((_e3446 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2930));
                                                            let _e3454 = (1f - (_e3452 * _e3452));
                                                            if (_e3454 != _e3454) {
                                                                phi_10279_ = true;
                                                            } else {
                                                                phi_10279_ = (0f >= _e3454);
                                                            }
                                                            let _e3458 = phi_10279_;
                                                            let _e3460 = sqrt(select(_e3454, 0f, _e3458));
                                                            let _e3462 = ((_e3446 * 747796405u) + 2891336453u);
                                                            let _e3470 = (((_e3462 >> bitcast<u32>((((_e3462 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3462) * 277803737u);
                                                            let _e3473 = ((_e3470 >> bitcast<u32>(22i)) ^ _e3470);
                                                            let _e3477 = (f32((_e3473 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                            let _e3479 = select(-1f, 1f, (_e2922.z >= 0f));
                                                            let _e3481 = (-1f / (_e3479 + _e2922.z));
                                                            let _e3483 = ((_e2922.x * _e2922.y) * _e3481);
                                                            let _e3496 = (_e3460 * cos(_e3477));
                                                            let _e3501 = (_e3460 * sin(_e3477));
                                                            let _e3511 = ((((1f + (((_e3479 * _e2922.x) * _e2922.x) * _e3481)) * _e3496) + (_e3483 * _e3501)) + (_e2922.x * _e3452));
                                                            let _e3512 = ((((_e3479 * _e3483) * _e3496) + ((_e3479 + ((_e2922.y * _e2922.y) * _e3481)) * _e3501)) + (_e2922.y * _e3452));
                                                            let _e3513 = ((((-(_e3479) * _e2922.x) * _e3496) + (-(_e2922.y) * _e3501)) + (_e2922.z * _e3452));
                                                            let _e3519 = (((_e3511 * _e2922.x) + (_e3512 * _e2922.y)) + (_e3513 * _e2922.z));
                                                            let _e3520 = (_e3519 <= _e2929);
                                                            if _e3520 {
                                                                phi_2822_ = 0f;
                                                            } else {
                                                                phi_2822_ = (0.15915494f / _e2930);
                                                            }
                                                            let _e3523 = phi_2822_;
                                                            if (_e3523 > 0f) {
                                                                if _e3520 {
                                                                    phi_2878_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    if (_e3519 != _e3519) {
                                                                        phi_10350_ = true;
                                                                    } else {
                                                                        phi_10350_ = (1f <= _e3519);
                                                                    }
                                                                    let _e3528 = phi_10350_;
                                                                    let _e3534 = global_1.member.member_6[3u];
                                                                    let _e3537 = (1f - pow((acos(select(_e3519, 1f, _e3528)) / _e3534), 2f));
                                                                    if (_e3537 != _e3537) {
                                                                        phi_10365_ = true;
                                                                    } else {
                                                                        phi_10365_ = (0f >= _e3537);
                                                                    }
                                                                    let _e3541 = phi_10365_;
                                                                    let _e3544 = global_1.member.member_6;
                                                                    let _e3552 = (((1f - (0.6f * (1f - sqrt(select(_e3537, 0f, _e3541))))) * 1.25f) / _e2931);
                                                                    phi_2878_ = vec3<f32>((_e3544.x * _e3552), (_e3544.y * _e3552), (_e3544.z * _e3552));
                                                                }
                                                                let _e3558 = phi_2878_;
                                                                phi_2879_ = _e3558;
                                                            } else {
                                                                phi_2879_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3560 = phi_2879_;
                                                            phi_15530_ = _e3473;
                                                            phi_2880_ = _e3560;
                                                            phi_2881_ = _e3523;
                                                            phi_2882_ = vec3<f32>(_e3511, _e3512, _e3513);
                                                        }
                                                        let _e3568 = phi_15530_;
                                                        let _e3570 = phi_2880_;
                                                        let _e3572 = phi_2881_;
                                                        let _e3574 = phi_2882_;
                                                        phi_15529_ = _e3568;
                                                        phi_2883_ = _e3570;
                                                        phi_2884_ = _e3572;
                                                        phi_2885_ = _e3574;
                                                    }
                                                    let _e3576 = phi_15529_;
                                                    let _e3578 = phi_2883_;
                                                    let _e3580 = phi_2884_;
                                                    let _e3582 = phi_2885_;
                                                    phi_15491_ = _e3576;
                                                    phi_4090_ = _e3578;
                                                    phi_4091_ = _e3580;
                                                    phi_4092_ = _e3582;
                                                    phi_4093_ = 1000000000000000000000000000000f;
                                                } else {
                                                    if (_e2067 == (_e2059 + 1u)) {
                                                        if ((_e211 & 1u) == 0u) {
                                                            phi_15495_ = _e2063;
                                                            phi_3409_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_3410_ = 0f;
                                                            phi_3411_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_3412_ = 1000000000000000000000000000000f;
                                                        } else {
                                                            if (_e214 > 0u) {
                                                                let _e2412 = global_1.member.member_2[1u];
                                                                let _e2414 = ((_e2063 * 747796405u) + 2891336453u);
                                                                let _e2422 = (((_e2414 >> bitcast<u32>((((_e2414 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2414) * 277803737u);
                                                                let _e2425 = ((_e2422 >> bitcast<u32>(22i)) ^ _e2422);
                                                                phi_2906_ = 0u;
                                                                phi_2909_ = _e214;
                                                                loop {
                                                                    let _e2431 = phi_2906_;
                                                                    let _e2433 = phi_2909_;
                                                                    local_17 = _e2431;
                                                                    local_18 = _e2431;
                                                                    let _e2435 = ((_e2431 + 1u) < _e2433);
                                                                    break_if_3_ = _e2435;
                                                                    if _e2435 {
                                                                        let _e2437 = ((_e2431 + _e2433) / 2u);
                                                                        let _e2440 = global_6.member[(_e2412 + _e2437)];
                                                                        let _e2441 = (_e2440 <= (f32((_e2425 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                        phi_2907_ = select(_e2431, _e2437, _e2441);
                                                                        phi_2910_ = select(_e2437, _e2433, _e2441);
                                                                    } else {
                                                                        phi_2907_ = u32();
                                                                        phi_2910_ = u32();
                                                                    }
                                                                    let _e2445 = phi_2907_;
                                                                    let _e2447 = phi_2910_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_2906_ = _e2445;
                                                                        phi_2909_ = _e2447;
                                                                        let _e7328 = break_if_3_;
                                                                        break if !(_e7328);
                                                                    }
                                                                }
                                                                let _e2450 = local_17;
                                                                let _e2451 = (_e2412 + _e2450);
                                                                let _e2454 = global_6.member[(_e2451 + 1u)];
                                                                let _e2456 = global_6.member[_e2451];
                                                                let _e2460 = global_1.member.member_2[0u];
                                                                let _e2462 = local_18;
                                                                let _e2465 = global_6.member[(_e2460 + _e2462)];
                                                                let _e2468 = ((_e2425 * 747796405u) + 2891336453u);
                                                                let _e2476 = (((_e2468 >> bitcast<u32>((((_e2468 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2468) * 277803737u);
                                                                let _e2479 = ((_e2476 >> bitcast<u32>(22i)) ^ _e2476);
                                                                let _e2482 = f32((_e2479 >> bitcast<u32>(8i)));
                                                                let _e2483 = (_e2482 * 0.000000059604645f);
                                                                let _e2485 = ((_e2479 * 747796405u) + 2891336453u);
                                                                let _e2493 = (((_e2485 >> bitcast<u32>((((_e2485 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2485) * 277803737u);
                                                                let _e2496 = ((_e2493 >> bitcast<u32>(22i)) ^ _e2493);
                                                                let _e2499 = f32((_e2496 >> bitcast<u32>(8i)));
                                                                let _e2502 = global_2.member[bitcast<u32>(_e2465)];
                                                                let _e2505 = (_e2502.member_6.x == 1u);
                                                                if _e2505 {
                                                                    let _e2647 = (_e2502.member.x - _e1188);
                                                                    let _e2648 = (_e2502.member.y - _e1189);
                                                                    let _e2649 = (_e2502.member.z - _e1190);
                                                                    let _e2654 = (((_e2647 * _e2647) + (_e2648 * _e2648)) + (_e2649 * _e2649));
                                                                    let _e2655 = (_e2502.member.w * _e2502.member.w);
                                                                    let _e2657 = (_e2654 <= (_e2655 * 1.000001f));
                                                                    if _e2657 {
                                                                        let _e2766 = (_e2482 * 0.00000037450704f);
                                                                        let _e2768 = ((_e2499 * 0.00000011920929f) - 1f);
                                                                        let _e2770 = (1f - (_e2768 * _e2768));
                                                                        if (_e2770 != _e2770) {
                                                                            phi_9347_ = true;
                                                                        } else {
                                                                            phi_9347_ = (0f >= _e2770);
                                                                        }
                                                                        let _e2774 = phi_9347_;
                                                                        let _e2776 = sqrt(select(_e2770, 0f, _e2774));
                                                                        let _e2778 = (_e2776 * cos(_e2766));
                                                                        let _e2780 = (_e2776 * sin(_e2766));
                                                                        let _e2785 = (_e2502.member.x + (_e2778 * _e2502.member.w));
                                                                        let _e2786 = (_e2502.member.y + (_e2780 * _e2502.member.w));
                                                                        let _e2787 = (_e2502.member.z + (_e2768 * _e2502.member.w));
                                                                        if _e2657 {
                                                                            let _e2800 = (_e2785 - _e1188);
                                                                            let _e2801 = (_e2786 - _e1189);
                                                                            let _e2802 = (_e2787 - _e1190);
                                                                            let _e2807 = (((_e2800 * _e2800) + (_e2801 * _e2801)) + (_e2802 * _e2802));
                                                                            phi_9416_ = (_e2807 / (((abs((((_e2778 * _e2800) + (_e2780 * _e2801)) + (_e2768 * _e2802))) / sqrt(_e2807)) * 12.566371f) * _e2655));
                                                                        } else {
                                                                            let _e2789 = (_e2655 / _e2654);
                                                                            if (_e2789 != _e2789) {
                                                                                phi_9435_ = true;
                                                                            } else {
                                                                                phi_9435_ = (1f <= _e2789);
                                                                            }
                                                                            let _e2793 = phi_9435_;
                                                                            let _e2794 = select(_e2789, 1f, _e2793);
                                                                            phi_9416_ = (0.15915494f / (_e2794 / (1f + sqrt((1f - _e2794)))));
                                                                        }
                                                                        let _e2820 = phi_9416_;
                                                                        phi_3122_ = type_60(vec3<f32>(_e2785, _e2786, _e2787), vec3<f32>(_e2778, _e2780, _e2768), _e2820);
                                                                    } else {
                                                                        let _e2658 = (_e2655 / _e2654);
                                                                        let _e2659 = (_e2658 != _e2658);
                                                                        if _e2659 {
                                                                            phi_9199_ = true;
                                                                        } else {
                                                                            phi_9199_ = (1f <= _e2658);
                                                                        }
                                                                        let _e2662 = phi_9199_;
                                                                        let _e2663 = select(_e2658, 1f, _e2662);
                                                                        if _e2659 {
                                                                            phi_9222_ = true;
                                                                        } else {
                                                                            phi_9222_ = (1f <= _e2658);
                                                                        }
                                                                        let _e2667 = phi_9222_;
                                                                        let _e2668 = select(_e2658, 1f, _e2667);
                                                                        let _e2672 = (_e2668 / (1f + sqrt((1f - _e2668))));
                                                                        if (_e2663 < 0.00068523f) {
                                                                            phi_3057_ = (_e2663 * _e2483);
                                                                        } else {
                                                                            let _e2675 = (1f - (_e2483 * _e2672));
                                                                            let _e2677 = (1f - (_e2675 * _e2675));
                                                                            if (_e2677 != _e2677) {
                                                                                phi_9237_ = true;
                                                                            } else {
                                                                                phi_9237_ = (0f >= _e2677);
                                                                            }
                                                                            let _e2681 = phi_9237_;
                                                                            phi_3057_ = select(_e2677, 0f, _e2681);
                                                                        }
                                                                        let _e2685 = phi_3057_;
                                                                        let _e2686 = (1f - _e2685);
                                                                        if (_e2686 != _e2686) {
                                                                            phi_9252_ = true;
                                                                        } else {
                                                                            phi_9252_ = (0f >= _e2686);
                                                                        }
                                                                        let _e2690 = phi_9252_;
                                                                        let _e2695 = (1f - (_e2685 / _e2663));
                                                                        if (_e2695 != _e2695) {
                                                                            phi_9267_ = true;
                                                                        } else {
                                                                            phi_9267_ = (0f >= _e2695);
                                                                        }
                                                                        let _e2699 = phi_9267_;
                                                                        let _e2703 = ((_e2685 / sqrt(_e2663)) + (sqrt(select(_e2686, 0f, _e2690)) * sqrt(select(_e2695, 0f, _e2699))));
                                                                        let _e2705 = (1f - (_e2703 * _e2703));
                                                                        if (_e2705 != _e2705) {
                                                                            phi_9282_ = true;
                                                                        } else {
                                                                            phi_9282_ = (0f >= _e2705);
                                                                        }
                                                                        let _e2709 = phi_9282_;
                                                                        let _e2711 = sqrt(select(_e2705, 0f, _e2709));
                                                                        let _e2712 = (_e2499 * 0.00000037450704f);
                                                                        let _e2713 = sqrt(_e2654);
                                                                        let _e2714 = (_e2647 / _e2713);
                                                                        let _e2715 = (_e2648 / _e2713);
                                                                        let _e2716 = (_e2649 / _e2713);
                                                                        let _e2718 = select(-1f, 1f, (_e2716 >= 0f));
                                                                        let _e2720 = (-1f / (_e2718 + _e2716));
                                                                        let _e2722 = ((_e2714 * _e2715) * _e2720);
                                                                        let _e2735 = (_e2711 * cos(_e2712));
                                                                        let _e2740 = (_e2711 * sin(_e2712));
                                                                        let _e2753 = -(((((1f + (((_e2718 * _e2714) * _e2714) * _e2720)) * _e2735) + (_e2722 * _e2740)) + (_e2714 * _e2703)));
                                                                        let _e2754 = -(((((_e2718 * _e2722) * _e2735) + ((_e2718 + ((_e2715 * _e2715) * _e2720)) * _e2740)) + (_e2715 * _e2703)));
                                                                        let _e2755 = -(((((-(_e2718) * _e2714) * _e2735) + (-(_e2715) * _e2740)) + (_e2716 * _e2703)));
                                                                        phi_3122_ = type_60(vec3<f32>((_e2502.member.x + (_e2753 * _e2502.member.w)), (_e2502.member.y + (_e2754 * _e2502.member.w)), (_e2502.member.z + (_e2755 * _e2502.member.w))), vec3<f32>(_e2753, _e2754, _e2755), (0.15915494f / _e2672));
                                                                    }
                                                                    let _e2823 = phi_3122_;
                                                                    phi_3308_ = _e2823;
                                                                } else {
                                                                    let _e2506 = sqrt(_e2483);
                                                                    let _e2507 = (1f - _e2506);
                                                                    let _e2508 = ((_e2499 * 0.000000059604645f) * _e2506);
                                                                    let _e2530 = (_e2506 - _e2508);
                                                                    let _e2534 = (((_e2502.member.x * _e2507) + (_e2502.member_1.x * _e2508)) + (_e2502.member_2.x * _e2530));
                                                                    let _e2535 = (((_e2502.member.y * _e2507) + (_e2502.member_1.y * _e2508)) + (_e2502.member_2.y * _e2530));
                                                                    let _e2536 = (((_e2502.member.z * _e2507) + (_e2502.member_1.z * _e2508)) + (_e2502.member_2.z * _e2530));
                                                                    let _e2538 = (_e2502.member_1.x - _e2502.member.x);
                                                                    let _e2539 = (_e2502.member_1.y - _e2502.member.y);
                                                                    let _e2540 = (_e2502.member_1.z - _e2502.member.z);
                                                                    let _e2541 = (_e2502.member_2.x - _e2502.member.x);
                                                                    let _e2542 = (_e2502.member_2.y - _e2502.member.y);
                                                                    let _e2543 = (_e2502.member_2.z - _e2502.member.z);
                                                                    let _e2546 = ((_e2539 * _e2543) - (_e2542 * _e2540));
                                                                    let _e2549 = ((_e2540 * _e2541) - (_e2543 * _e2538));
                                                                    let _e2552 = ((_e2538 * _e2542) - (_e2541 * _e2539));
                                                                    let _e2558 = sqrt((((_e2546 * _e2546) + (_e2549 * _e2549)) + (_e2552 * _e2552)));
                                                                    let _e2559 = (1f / _e2558);
                                                                    let _e2560 = (_e2546 * _e2559);
                                                                    let _e2561 = (_e2549 * _e2559);
                                                                    let _e2562 = (_e2552 * _e2559);
                                                                    if _e2505 {
                                                                        let _e2595 = (_e2502.member.x - _e1188);
                                                                        let _e2596 = (_e2502.member.y - _e1189);
                                                                        let _e2597 = (_e2502.member.z - _e1190);
                                                                        let _e2602 = (((_e2595 * _e2595) + (_e2596 * _e2596)) + (_e2597 * _e2597));
                                                                        let _e2603 = (_e2502.member.w * _e2502.member.w);
                                                                        if (_e2602 <= (_e2603 * 1.000001f)) {
                                                                            let _e2617 = (_e2534 - _e1188);
                                                                            let _e2618 = (_e2535 - _e1189);
                                                                            let _e2619 = (_e2536 - _e1190);
                                                                            let _e2624 = (((_e2617 * _e2617) + (_e2618 * _e2618)) + (_e2619 * _e2619));
                                                                            phi_9165_ = (_e2624 / (((abs((((_e2560 * _e2617) + (_e2561 * _e2618)) + (_e2562 * _e2619))) / sqrt(_e2624)) * 12.566371f) * _e2603));
                                                                        } else {
                                                                            let _e2606 = (_e2603 / _e2602);
                                                                            if (_e2606 != _e2606) {
                                                                                phi_9184_ = true;
                                                                            } else {
                                                                                phi_9184_ = (1f <= _e2606);
                                                                            }
                                                                            let _e2610 = phi_9184_;
                                                                            let _e2611 = select(_e2606, 1f, _e2610);
                                                                            phi_9165_ = (0.15915494f / (_e2611 / (1f + sqrt((1f - _e2611)))));
                                                                        }
                                                                        let _e2637 = phi_9165_;
                                                                        phi_3306_ = _e2637;
                                                                    } else {
                                                                        let _e2564 = (_e2534 - _e1188);
                                                                        let _e2565 = (_e2535 - _e1189);
                                                                        let _e2566 = (_e2536 - _e1190);
                                                                        let _e2571 = (((_e2564 * _e2564) + (_e2565 * _e2565)) + (_e2566 * _e2566));
                                                                        let _e2579 = (abs((((_e2560 * _e2564) + (_e2561 * _e2565)) + (_e2562 * _e2566))) / sqrt(_e2571));
                                                                        if (_e2579 <= 0f) {
                                                                            phi_3305_ = 0f;
                                                                        } else {
                                                                            if (_e2502.member_6.x == 0u) {
                                                                                phi_3302_ = (_e2558 * 0.5f);
                                                                            } else {
                                                                                phi_3302_ = ((12.566371f * _e2502.member.w) * _e2502.member.w);
                                                                            }
                                                                            let _e2588 = phi_3302_;
                                                                            phi_3305_ = (_e2571 / (_e2579 * _e2588));
                                                                        }
                                                                        let _e2592 = phi_3305_;
                                                                        phi_3306_ = _e2592;
                                                                    }
                                                                    let _e2639 = phi_3306_;
                                                                    phi_3308_ = type_60(vec3<f32>(_e2534, _e2535, _e2536), vec3<f32>(_e2560, _e2561, _e2562), _e2639);
                                                                }
                                                                let _e2825 = phi_3308_;
                                                                let _e2830 = (_e2825.member.x - _e1188);
                                                                let _e2832 = (_e2825.member.y - _e1189);
                                                                let _e2834 = (_e2825.member.z - _e1190);
                                                                let _e2840 = sqrt((((_e2830 * _e2830) + (_e2832 * _e2832)) + (_e2834 * _e2834)));
                                                                let _e2841 = (_e2830 / _e2840);
                                                                let _e2842 = (_e2832 / _e2840);
                                                                let _e2843 = (_e2834 / _e2840);
                                                                if (_e2825.member_2 > 0f) {
                                                                    let _e2855 = global_4.member[_e2502.member_6.y];
                                                                    if (_e2855.member == 4u) {
                                                                        if (_e2855.member_1 == 1u) {
                                                                            phi_3386_ = true;
                                                                        } else {
                                                                            phi_3386_ = ((((-(_e2841) * _e2825.member_1.x) + (-(_e2842) * _e2825.member_1.y)) + (-(_e2843) * _e2825.member_1.z)) > 0f);
                                                                        }
                                                                        let _e2870 = phi_3386_;
                                                                        if _e2870 {
                                                                            phi_3395_ = vec3<f32>(_e2855.member_4.x, _e2855.member_4.y, _e2855.member_4.z);
                                                                        } else {
                                                                            phi_3395_ = vec3<f32>();
                                                                        }
                                                                        let _e2877 = phi_3395_;
                                                                        phi_3397_ = _e2877;
                                                                        phi_3398_ = select(true, false, _e2870);
                                                                    } else {
                                                                        phi_3397_ = vec3<f32>();
                                                                        phi_3398_ = true;
                                                                    }
                                                                    let _e2880 = phi_3397_;
                                                                    let _e2882 = phi_3398_;
                                                                    phi_3403_ = select(_e2880, vec3<f32>(0f, 0f, 0f), vec3(_e2882));
                                                                    phi_3404_ = ((_e2454 - _e2456) * _e2825.member_2);
                                                                } else {
                                                                    phi_3403_ = vec3<f32>(0f, 0f, 0f);
                                                                    phi_3404_ = 0f;
                                                                }
                                                                let _e2886 = phi_3403_;
                                                                let _e2888 = phi_3404_;
                                                                phi_15496_ = _e2496;
                                                                phi_3405_ = _e2886;
                                                                phi_3406_ = _e2888;
                                                                phi_3407_ = vec3<f32>(_e2841, _e2842, _e2843);
                                                                phi_3408_ = ((_e2840 * 0.9999f) - _e1181);
                                                            } else {
                                                                phi_15496_ = _e2063;
                                                                phi_3405_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3406_ = 0f;
                                                                phi_3407_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3408_ = 1000000000000000000000000000000f;
                                                            }
                                                            let _e2890 = phi_15496_;
                                                            let _e2892 = phi_3405_;
                                                            let _e2894 = phi_3406_;
                                                            let _e2896 = phi_3407_;
                                                            let _e2898 = phi_3408_;
                                                            phi_15495_ = _e2890;
                                                            phi_3409_ = _e2892;
                                                            phi_3410_ = _e2894;
                                                            phi_3411_ = _e2896;
                                                            phi_3412_ = _e2898;
                                                        }
                                                        let _e2900 = phi_15495_;
                                                        let _e2902 = phi_3409_;
                                                        let _e2904 = phi_3410_;
                                                        let _e2906 = phi_3411_;
                                                        let _e2908 = phi_3412_;
                                                        phi_15492_ = _e2900;
                                                        phi_4086_ = _e2902;
                                                        phi_4087_ = _e2904;
                                                        phi_4088_ = _e2906;
                                                        phi_4089_ = _e2908;
                                                    } else {
                                                        if ((_e211 & 2u) == 0u) {
                                                            phi_15493_ = _e2063;
                                                            phi_4083_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_4084_ = 0f;
                                                            phi_4085_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e2077 = ((_e2063 * 747796405u) + 2891336453u);
                                                            let _e2085 = (((_e2077 >> bitcast<u32>((((_e2077 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2077) * 277803737u);
                                                            let _e2088 = ((_e2085 >> bitcast<u32>(22i)) ^ _e2085);
                                                            let _e2092 = (f32((_e2088 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2094 = ((_e2088 * 747796405u) + 2891336453u);
                                                            let _e2102 = (((_e2094 >> bitcast<u32>((((_e2094 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2094) * 277803737u);
                                                            let _e2105 = ((_e2102 >> bitcast<u32>(22i)) ^ _e2102);
                                                            let _e2109 = (f32((_e2105 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2112 = global_1.member.member_3[1u];
                                                            let _e2115 = global_1.member.member_3[2u];
                                                            let _e2118 = global_1.member.member_2[3u];
                                                            phi_3433_ = 0u;
                                                            phi_3436_ = _e2115;
                                                            loop {
                                                                let _e2120 = phi_3433_;
                                                                let _e2122 = phi_3436_;
                                                                local_12 = _e2120;
                                                                local_15 = _e2120;
                                                                local_16 = _e2120;
                                                                let _e2124 = ((_e2120 + 1u) < _e2122);
                                                                break_if_4_ = _e2124;
                                                                if _e2124 {
                                                                    let _e2126 = ((_e2120 + _e2122) / 2u);
                                                                    let _e2129 = global_6.member[(_e2118 + _e2126)];
                                                                    let _e2130 = (_e2129 <= _e2092);
                                                                    phi_3434_ = select(_e2120, _e2126, _e2130);
                                                                    phi_3437_ = select(_e2126, _e2122, _e2130);
                                                                } else {
                                                                    phi_3434_ = u32();
                                                                    phi_3437_ = u32();
                                                                }
                                                                let _e2134 = phi_3434_;
                                                                let _e2136 = phi_3437_;
                                                                continue;
                                                                continuing {
                                                                    phi_3433_ = _e2134;
                                                                    phi_3436_ = _e2136;
                                                                    let _e7331 = break_if_4_;
                                                                    break if !(_e7331);
                                                                }
                                                            }
                                                            let _e2140 = global_1.member.member_3[0u];
                                                            let _e2141 = (_e2112 + 1u);
                                                            let _e2143 = local_12;
                                                            let _e2145 = (_e2140 + (_e2143 * _e2141));
                                                            phi_3474_ = 0u;
                                                            phi_3477_ = _e2112;
                                                            loop {
                                                                let _e2147 = phi_3474_;
                                                                let _e2149 = phi_3477_;
                                                                local_13 = _e2147;
                                                                local_14 = _e2147;
                                                                let _e2151 = ((_e2147 + 1u) < _e2149);
                                                                break_if_5_ = _e2151;
                                                                if _e2151 {
                                                                    let _e2153 = ((_e2147 + _e2149) / 2u);
                                                                    let _e2156 = global_6.member[(_e2145 + _e2153)];
                                                                    let _e2157 = (_e2156 <= _e2109);
                                                                    phi_3475_ = select(_e2147, _e2153, _e2157);
                                                                    phi_3478_ = select(_e2153, _e2149, _e2157);
                                                                } else {
                                                                    phi_3475_ = u32();
                                                                    phi_3478_ = u32();
                                                                }
                                                                let _e2161 = phi_3475_;
                                                                let _e2163 = phi_3478_;
                                                                continue;
                                                                continuing {
                                                                    phi_3474_ = _e2161;
                                                                    phi_3477_ = _e2163;
                                                                    let _e7334 = break_if_5_;
                                                                    break if !(_e7334);
                                                                }
                                                            }
                                                            let _e2166 = local_13;
                                                            let _e2169 = local_14;
                                                            let _e2170 = (_e2145 + _e2169);
                                                            let _e2172 = global_6.member[_e2170];
                                                            let _e2175 = global_6.member[(_e2170 + 1u)];
                                                            if (_e2175 > _e2172) {
                                                                let _e2179 = ((_e2109 - _e2172) / (_e2175 - _e2172));
                                                                let _e2181 = select(_e2179, 0f, (_e2179 < 0f));
                                                                phi_3554_ = select(_e2181, 0.99999f, (_e2181 > 0.99999f));
                                                            } else {
                                                                phi_3554_ = 0.5f;
                                                            }
                                                            let _e2185 = phi_3554_;
                                                            let _e2187 = f32(_e2112);
                                                            let _e2190 = local_15;
                                                            let _e2193 = local_16;
                                                            let _e2194 = (_e2118 + _e2193);
                                                            let _e2196 = global_6.member[_e2194];
                                                            let _e2199 = global_6.member[(_e2194 + 1u)];
                                                            if (_e2199 > _e2196) {
                                                                let _e2203 = ((_e2092 - _e2196) / (_e2199 - _e2196));
                                                                let _e2205 = select(_e2203, 0f, (_e2203 < 0f));
                                                                phi_3603_ = select(_e2205, 0.99999f, (_e2205 > 0.99999f));
                                                            } else {
                                                                phi_3603_ = 0.5f;
                                                            }
                                                            let _e2209 = phi_3603_;
                                                            let _e2211 = f32(_e2115);
                                                            let _e2214 = ((((f32(_e2166) + _e2185) / _e2187) - 0.5f) * 6.2831855f);
                                                            let _e2215 = (((f32(_e2190) + _e2209) / _e2211) * 3.1415927f);
                                                            let _e2216 = sin(_e2215);
                                                            let _e2218 = (_e2216 * cos(_e2214));
                                                            let _e2219 = cos(_e2215);
                                                            let _e2221 = (_e2216 * sin(_e2214));
                                                            let _e2224 = ((atan2(_e2221, _e2218) * 0.15915494f) + 0.5f);
                                                            let _e2228 = select(_e2219, -1f, (_e2219 < -1f));
                                                            let _e2231 = acos(select(_e2228, 1f, (_e2228 > 1f)));
                                                            let _e2233 = ((_e2224 - floor(_e2224)) * _e2187);
                                                            let _e2238 = select(select(u32(_e2233), 0u, (_e2233 < 0f)), 4294967295u, (_e2233 > 4294967000f));
                                                            let _e2239 = (_e2112 - 1u);
                                                            let _e2242 = ((_e2231 * 0.31830987f) * _e2211);
                                                            let _e2247 = select(select(u32(_e2242), 0u, (_e2242 < 0f)), 4294967295u, (_e2242 > 4294967000f));
                                                            let _e2248 = (_e2115 - 1u);
                                                            let _e2250 = select(_e2247, _e2248, (_e2248 < _e2247));
                                                            let _e2251 = sin(_e2231);
                                                            if (_e2251 <= 0f) {
                                                                phi_3759_ = 0f;
                                                            } else {
                                                                let _e2255 = (_e2118 + _e2250);
                                                                let _e2258 = global_6.member[(_e2255 + 1u)];
                                                                let _e2260 = global_6.member[_e2255];
                                                                let _e2263 = ((_e2140 + (_e2250 * _e2141)) + select(_e2238, _e2239, (_e2239 < _e2238)));
                                                                let _e2266 = global_6.member[(_e2263 + 1u)];
                                                                let _e2268 = global_6.member[_e2263];
                                                                phi_3759_ = ((((_e2258 - _e2260) * _e2211) * ((_e2266 - _e2268) * _e2187)) / (19.73921f * _e2251));
                                                            }
                                                            let _e2275 = phi_3759_;
                                                            let _e2276 = (_e2275 > 0f);
                                                            if _e2276 {
                                                                let _e2278 = (_e2233 - 0.5f);
                                                                let _e2279 = (_e2242 - 0.5f);
                                                                let _e2280 = f32(_e2248);
                                                                if (0f <= _e2280) {
                                                                } else {
                                                                    phi_15462_ = _e2105;
                                                                    phi_15449_ = true;
                                                                    break;
                                                                }
                                                                let _e2283 = select(_e2279, 0f, (_e2279 < 0f));
                                                                let _e2285 = select(_e2283, _e2280, (_e2283 > _e2280));
                                                                let _e2286 = floor(_e2278);
                                                                let _e2287 = floor(_e2285);
                                                                let _e2288 = (_e2278 - _e2286);
                                                                let _e2289 = (_e2285 - _e2287);
                                                                let _e2300 = (_e2112 == 0u);
                                                                if _e2300 {
                                                                    phi_15462_ = _e2105;
                                                                    phi_15449_ = true;
                                                                    break;
                                                                }
                                                                let _e2301 = (bitcast<u32>((select(0i, select(select(i32(_e2286), i32(-2147483648), (_e2286 < -2147483600f)), 2147483647i, (_e2286 > 2147483500f)), (_e2286 == _e2286)) + bitcast<i32>(_e2112))) % _e2112);
                                                                let _e2306 = select(select(u32(_e2287), 0u, (_e2287 < 0f)), 4294967295u, (_e2287 > 4294967000f));
                                                                let _e2307 = (_e2306 + 1u);
                                                                let _e2312 = global_1.member.member_2[2u];
                                                                let _e2313 = (_e2306 * _e2112);
                                                                if _e2300 {
                                                                    phi_15462_ = _e2105;
                                                                    phi_15449_ = true;
                                                                    break;
                                                                }
                                                                let _e2314 = (_e2301 % _e2112);
                                                                let _e2317 = (_e2312 + (3u * (_e2313 + _e2314)));
                                                                let _e2319 = global_6.member[_e2317];
                                                                let _e2322 = global_6.member[(_e2317 + 1u)];
                                                                let _e2325 = global_6.member[(_e2317 + 2u)];
                                                                if _e2300 {
                                                                    phi_15462_ = _e2105;
                                                                    phi_15449_ = true;
                                                                    break;
                                                                }
                                                                let _e2328 = ((_e2301 + 1u) % _e2112);
                                                                let _e2331 = (_e2312 + (3u * (_e2313 + _e2328)));
                                                                let _e2333 = global_6.member[_e2331];
                                                                let _e2336 = global_6.member[(_e2331 + 1u)];
                                                                let _e2339 = global_6.member[(_e2331 + 2u)];
                                                                let _e2347 = (vec3<f32>(_e2319, _e2322, _e2325) + vec3<f32>(((_e2333 - _e2319) * _e2288), ((_e2336 - _e2322) * _e2288), ((_e2339 - _e2325) * _e2288)));
                                                                let _e2348 = (select(_e2307, _e2248, (_e2248 < _e2307)) * _e2112);
                                                                if _e2300 {
                                                                    phi_15462_ = _e2105;
                                                                    phi_15449_ = true;
                                                                    break;
                                                                }
                                                                let _e2351 = (_e2312 + (3u * (_e2348 + _e2314)));
                                                                let _e2353 = global_6.member[_e2351];
                                                                let _e2356 = global_6.member[(_e2351 + 1u)];
                                                                let _e2359 = global_6.member[(_e2351 + 2u)];
                                                                if _e2300 {
                                                                    phi_15462_ = _e2105;
                                                                    phi_15449_ = true;
                                                                    break;
                                                                }
                                                                let _e2363 = (_e2312 + (3u * (_e2348 + _e2328)));
                                                                let _e2365 = global_6.member[_e2363];
                                                                let _e2368 = global_6.member[(_e2363 + 1u)];
                                                                let _e2371 = global_6.member[(_e2363 + 2u)];
                                                                let _e2379 = (vec3<f32>(_e2353, _e2356, _e2359) + vec3<f32>(((_e2365 - _e2353) * _e2288), ((_e2368 - _e2356) * _e2288), ((_e2371 - _e2359) * _e2288)));
                                                                phi_4080_ = (_e2347 + vec3<f32>(((_e2379.x - _e2347.x) * _e2289), ((_e2379.y - _e2347.y) * _e2289), ((_e2379.z - _e2347.z) * _e2289)));
                                                                phi_4082_ = vec3<f32>(_e2218, _e2219, _e2221);
                                                            } else {
                                                                phi_4080_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_4082_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2395 = phi_4080_;
                                                            let _e2397 = phi_4082_;
                                                            phi_15493_ = _e2105;
                                                            phi_4083_ = _e2395;
                                                            phi_4084_ = select(0f, _e2275, _e2276);
                                                            phi_4085_ = _e2397;
                                                        }
                                                        let _e2400 = phi_15493_;
                                                        let _e2402 = phi_4083_;
                                                        let _e2404 = phi_4084_;
                                                        let _e2406 = phi_4085_;
                                                        phi_15492_ = _e2400;
                                                        phi_4086_ = _e2402;
                                                        phi_4087_ = _e2404;
                                                        phi_4088_ = _e2406;
                                                        phi_4089_ = 1000000000000000000000000000000f;
                                                    }
                                                    let _e2910 = phi_15492_;
                                                    let _e2912 = phi_4086_;
                                                    let _e2914 = phi_4087_;
                                                    let _e2916 = phi_4088_;
                                                    let _e2918 = phi_4089_;
                                                    phi_15491_ = _e2910;
                                                    phi_4090_ = _e2912;
                                                    phi_4091_ = _e2914;
                                                    phi_4092_ = _e2916;
                                                    phi_4093_ = _e2918;
                                                }
                                                let _e3584 = phi_15491_;
                                                let _e3586 = phi_4090_;
                                                let _e3588 = phi_4091_;
                                                let _e3590 = phi_4092_;
                                                let _e3592 = phi_4093_;
                                                phi_15490_ = _e3584;
                                                phi_4094_ = _e3586;
                                                phi_4095_ = _e3588;
                                                phi_4096_ = _e3590;
                                                phi_4097_ = _e3592;
                                            }
                                            let _e3661 = phi_15490_;
                                            let _e3663 = phi_4094_;
                                            let _e3665 = phi_4095_;
                                            let _e3667 = phi_4096_;
                                            let _e3669 = phi_4097_;
                                            if ((((_e3663.x * _e3663.x) + (_e3663.y * _e3663.y)) + (_e3663.z * _e3663.z)) > 0f) {
                                                let _e3686 = (((_e3667.x * _e1163.x) + (_e3667.y * _e1163.y)) + (_e3667.z * _e1163.z));
                                                if (_e3686 != _e3686) {
                                                    phi_10380_ = true;
                                                } else {
                                                    phi_10380_ = (0f >= _e3686);
                                                }
                                                let _e3690 = phi_10380_;
                                                let _e3692 = (select(_e3686, 0f, _e3690) * 0.31830987f);
                                                if (_e2038 == 0u) {
                                                    if (_e3686 <= 0f) {
                                                        phi_11177_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_11177_ = (_e2032 * (_e3686 * 0.31830987f));
                                                    }
                                                    let _e4171 = phi_11177_;
                                                    phi_4493_ = type_59(_e4171, _e3692);
                                                    phi_4494_ = type_59();
                                                    phi_4495_ = true;
                                                } else {
                                                    let _e3701 = (((_e1137.x * _e1209) + (_e1137.y * _e1210)) + (_e1137.z * _e1212));
                                                    let _e3706 = (((_e1137.x * _e1205) + (_e1137.y * _e1215)) + (_e1137.z * _e1216));
                                                    let _e3711 = (((_e1137.x * _e1163.x) + (_e1137.y * _e1163.y)) + (_e1137.z * _e1163.z));
                                                    let _e3716 = (((_e3667.x * _e1209) + (_e3667.y * _e1210)) + (_e3667.z * _e1212));
                                                    let _e3721 = (((_e3667.x * _e1205) + (_e3667.y * _e1215)) + (_e3667.z * _e1216));
                                                    if (_e3711 <= 0f) {
                                                        phi_4457_ = type_59();
                                                        phi_4458_ = true;
                                                    } else {
                                                        let _e3723 = (_e3686 <= 0f);
                                                        if _e3723 {
                                                            phi_4456_ = type_59();
                                                        } else {
                                                            let _e3724 = (_e3701 + _e3716);
                                                            let _e3725 = (_e3706 + _e3721);
                                                            let _e3726 = (_e3711 + _e3686);
                                                            let _e3733 = (1f / sqrt((((_e3724 * _e3724) + (_e3725 * _e3725)) + (_e3726 * _e3726))));
                                                            let _e3736 = (_e3726 * _e3733);
                                                            if (_e3736 <= 0f) {
                                                                phi_10490_ = 0f;
                                                            } else {
                                                                let _e3738 = (_e2036 * _e2036);
                                                                let _e3742 = ((((_e3738 - 1f) * _e3736) * _e3736) + 1f);
                                                                phi_10490_ = (_e3738 / ((3.1415927f * _e3742) * _e3742));
                                                            }
                                                            let _e3747 = phi_10490_;
                                                            let _e3748 = (_e3711 * _e3711);
                                                            if (_e3748 >= 1f) {
                                                                phi_10518_ = 0f;
                                                            } else {
                                                                phi_10518_ = ((sqrt((1f + (((_e2036 * _e2036) * ((_e3701 * _e3701) + (_e3706 * _e3706))) / _e3748))) - 1f) * 0.5f);
                                                            }
                                                            let _e3761 = phi_10518_;
                                                            let _e3762 = (1f + _e3761);
                                                            let _e3763 = (_e3686 * _e3686);
                                                            if (_e3763 >= 1f) {
                                                                phi_10546_ = 0f;
                                                            } else {
                                                                phi_10546_ = ((sqrt((1f + (((_e2036 * _e2036) * ((_e3716 * _e3716) + (_e3721 * _e3721))) / _e3763))) - 1f) * 0.5f);
                                                            }
                                                            let _e3776 = phi_10546_;
                                                            let _e3783 = (((_e3701 * (_e3724 * _e3733)) + (_e3706 * (_e3725 * _e3733))) + (_e3711 * _e3736));
                                                            if (_e2034 == 0u) {
                                                                let _e4078 = (1f - _e3783);
                                                                if (_e4078 != _e4078) {
                                                                    phi_11049_ = true;
                                                                } else {
                                                                    phi_11049_ = (0f >= _e4078);
                                                                }
                                                                let _e4082 = phi_11049_;
                                                                let _e4084 = pow(select(_e4078, 0f, _e4082), 5f);
                                                                let _e4085 = (1f - _e4084);
                                                                phi_4444_ = vec3<f32>(((_e2032.x * _e4085) + _e4084), ((_e2032.y * _e4085) + _e4084), ((_e2032.z * _e4085) + _e4084));
                                                            } else {
                                                                if (_e2034 == 1u) {
                                                                    let _e4051 = (1f / _e2028);
                                                                    let _e4055 = ((_e4051 * _e4051) * (1f - (_e3783 * _e3783)));
                                                                    if (_e4055 >= 1f) {
                                                                        phi_11010_ = 1f;
                                                                    } else {
                                                                        let _e4058 = sqrt((1f - _e4055));
                                                                        if (_e3783 != _e3783) {
                                                                            phi_11021_ = true;
                                                                        } else {
                                                                            phi_11021_ = (_e4058 <= _e3783);
                                                                        }
                                                                        let _e4062 = phi_11021_;
                                                                        let _e4067 = pow(((1f - _e2028) / (1f + _e2028)), 2f);
                                                                        phi_11010_ = (_e4067 + ((1f - _e4067) * pow((1f - select(_e3783, _e4058, _e4062)), 5f)));
                                                                    }
                                                                    let _e4074 = phi_11010_;
                                                                    phi_4443_ = vec3(_e4074);
                                                                } else {
                                                                    let _e3787 = select(_e3783, 0f, (_e3783 < 0f));
                                                                    let _e3789 = select(_e3787, 1f, (_e3787 > 1f));
                                                                    let _e3791 = (1f - (_e3789 * _e3789));
                                                                    let _e3804 = ((_e2032.x * _e2032.x) - (_e2030.x * _e2030.x));
                                                                    let _e3805 = ((_e2032.y * _e2032.y) - (_e2030.y * _e2030.y));
                                                                    let _e3806 = ((_e2032.z * _e2032.z) - (_e2030.z * _e2030.z));
                                                                    let _e3808 = (_e2032.x * (_e2030.x + _e2030.x));
                                                                    let _e3810 = (_e2032.y * (_e2030.y + _e2030.y));
                                                                    let _e3812 = (_e2032.z * (_e2030.z + _e2030.z));
                                                                    let _e3819 = ((_e3804 * _e3804) + (_e3808 * _e3808));
                                                                    let _e3820 = ((_e3805 * _e3805) + (_e3810 * _e3810));
                                                                    let _e3821 = ((_e3806 * _e3806) + (_e3812 * _e3812));
                                                                    let _e3826 = ((_e3791 * _e3804) / _e3819);
                                                                    let _e3827 = ((_e3791 * _e3805) / _e3820);
                                                                    let _e3828 = ((_e3791 * _e3806) / _e3821);
                                                                    let _e3829 = (1f - _e3826);
                                                                    let _e3830 = (1f - _e3827);
                                                                    let _e3831 = (1f - _e3828);
                                                                    let _e3833 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3826, _e3827, _e3828));
                                                                    let _e3834 = (_e3791 * _e3808);
                                                                    let _e3835 = (_e3791 * _e3810);
                                                                    let _e3836 = (_e3791 * _e3812);
                                                                    let _e3841 = (vec3<f32>(_e3834, _e3835, _e3836) / vec3<f32>(_e3819, _e3820, _e3821));
                                                                    let _e3854 = ((_e3833.x * _e3833.x) + (_e3841.x * _e3841.x));
                                                                    let _e3855 = ((_e3833.y * _e3833.y) + (_e3841.y * _e3841.y));
                                                                    let _e3856 = ((_e3833.z * _e3833.z) + (_e3841.z * _e3841.z));
                                                                    if (_e3854 != _e3854) {
                                                                        phi_10680_ = true;
                                                                    } else {
                                                                        phi_10680_ = (0f >= _e3854);
                                                                    }
                                                                    let _e3860 = phi_10680_;
                                                                    let _e3862 = sqrt(select(_e3854, 0f, _e3860));
                                                                    if (_e3855 != _e3855) {
                                                                        phi_10695_ = true;
                                                                    } else {
                                                                        phi_10695_ = (0f >= _e3855);
                                                                    }
                                                                    let _e3866 = phi_10695_;
                                                                    let _e3868 = sqrt(select(_e3855, 0f, _e3866));
                                                                    if (_e3856 != _e3856) {
                                                                        phi_10710_ = true;
                                                                    } else {
                                                                        phi_10710_ = (0f >= _e3856);
                                                                    }
                                                                    let _e3872 = phi_10710_;
                                                                    let _e3874 = sqrt(select(_e3856, 0f, _e3872));
                                                                    let _e3878 = ((_e3862 + _e3829) * 0.5f);
                                                                    let _e3879 = ((_e3868 + _e3830) * 0.5f);
                                                                    let _e3880 = ((_e3874 + _e3831) * 0.5f);
                                                                    if (_e3878 != _e3878) {
                                                                        phi_10737_ = true;
                                                                    } else {
                                                                        phi_10737_ = (0f >= _e3878);
                                                                    }
                                                                    let _e3884 = phi_10737_;
                                                                    let _e3886 = sqrt(select(_e3878, 0f, _e3884));
                                                                    if (_e3879 != _e3879) {
                                                                        phi_10752_ = true;
                                                                    } else {
                                                                        phi_10752_ = (0f >= _e3879);
                                                                    }
                                                                    let _e3890 = phi_10752_;
                                                                    let _e3892 = sqrt(select(_e3879, 0f, _e3890));
                                                                    if (_e3880 != _e3880) {
                                                                        phi_10767_ = true;
                                                                    } else {
                                                                        phi_10767_ = (0f >= _e3880);
                                                                    }
                                                                    let _e3896 = phi_10767_;
                                                                    let _e3898 = sqrt(select(_e3880, 0f, _e3896));
                                                                    let _e3899 = vec3<f32>(_e3886, _e3892, _e3898);
                                                                    let _e3903 = ((_e3862 - _e3829) * 0.5f);
                                                                    let _e3904 = ((_e3868 - _e3830) * 0.5f);
                                                                    let _e3905 = ((_e3874 - _e3831) * 0.5f);
                                                                    if (_e3903 != _e3903) {
                                                                        phi_10794_ = true;
                                                                    } else {
                                                                        phi_10794_ = (0f >= _e3903);
                                                                    }
                                                                    let _e3909 = phi_10794_;
                                                                    let _e3911 = sqrt(select(_e3903, 0f, _e3909));
                                                                    if (_e3904 != _e3904) {
                                                                        phi_10809_ = true;
                                                                    } else {
                                                                        phi_10809_ = (0f >= _e3904);
                                                                    }
                                                                    let _e3915 = phi_10809_;
                                                                    let _e3917 = sqrt(select(_e3904, 0f, _e3915));
                                                                    if (_e3905 != _e3905) {
                                                                        phi_10824_ = true;
                                                                    } else {
                                                                        phi_10824_ = (0f >= _e3905);
                                                                    }
                                                                    let _e3921 = phi_10824_;
                                                                    let _e3923 = sqrt(select(_e3905, 0f, _e3921));
                                                                    if ((_e3834 / _e3819) < 0f) {
                                                                        phi_4320_ = -(_e3911);
                                                                    } else {
                                                                        phi_4320_ = _e3911;
                                                                    }
                                                                    let _e3927 = phi_4320_;
                                                                    if ((_e3835 / _e3820) < 0f) {
                                                                        phi_4328_ = -(_e3917);
                                                                    } else {
                                                                        phi_4328_ = _e3917;
                                                                    }
                                                                    let _e3931 = phi_4328_;
                                                                    if ((_e3836 / _e3821) < 0f) {
                                                                        phi_4336_ = -(_e3923);
                                                                    } else {
                                                                        phi_4336_ = _e3923;
                                                                    }
                                                                    let _e3935 = phi_4336_;
                                                                    let _e3939 = (_e2030.x * _e3789);
                                                                    let _e3940 = (_e2030.y * _e3789);
                                                                    let _e3941 = (_e2030.z * _e3789);
                                                                    let _e3951 = vec3<f32>(((_e2032.x * _e3886) - (_e2030.x * _e3927)), ((_e2032.y * _e3892) - (_e2030.y * _e3931)), ((_e2032.z * _e3898) - (_e2030.z * _e3935)));
                                                                    let _e3958 = ((_e2032.x * _e3927) + (_e2030.x * _e3886));
                                                                    let _e3959 = ((_e2032.y * _e3931) + (_e2030.y * _e3892));
                                                                    let _e3960 = ((_e2032.z * _e3935) + (_e2030.z * _e3898));
                                                                    let _e3962 = vec3<f32>((_e2032.x * _e3789), (_e2032.y * _e3789), (_e2032.z * _e3789));
                                                                    let _e3963 = (_e3962 - _e3899);
                                                                    let _e3964 = (_e3939 - _e3927);
                                                                    let _e3965 = (_e3940 - _e3931);
                                                                    let _e3966 = (_e3941 - _e3935);
                                                                    let _e3979 = (_e3962 + _e3899);
                                                                    let _e3980 = (_e3939 + _e3927);
                                                                    let _e3981 = (_e3940 + _e3931);
                                                                    let _e3982 = (_e3941 + _e3935);
                                                                    let _e3992 = ((_e3979.x * _e3979.x) + (_e3980 * _e3980));
                                                                    let _e3993 = ((_e3979.y * _e3979.y) + (_e3981 * _e3981));
                                                                    let _e3994 = ((_e3979.z * _e3979.z) + (_e3982 * _e3982));
                                                                    let _e4004 = vec3(_e3789);
                                                                    let _e4005 = (_e4004 - _e3951);
                                                                    let _e4006 = -(vec3<f32>(_e3958, _e3959, _e3960));
                                                                    let _e4022 = (_e4004 + _e3951);
                                                                    let _e4032 = ((_e4022.x * _e4022.x) + (_e3958 * _e3958));
                                                                    let _e4033 = ((_e4022.y * _e4022.y) + (_e3959 * _e3959));
                                                                    let _e4034 = ((_e4022.z * _e4022.z) + (_e3960 * _e3960));
                                                                    phi_4443_ = vec3<f32>((((((_e3963.x * _e3963.x) + (_e3964 * _e3964)) / select(0.000000000000000000000000000001f, _e3992, (_e3992 > 0.000000000000000000000000000001f))) + (((_e4005.x * _e4005.x) + (_e4006.x * _e4006.x)) / select(0.000000000000000000000000000001f, _e4032, (_e4032 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3963.y * _e3963.y) + (_e3965 * _e3965)) / select(0.000000000000000000000000000001f, _e3993, (_e3993 > 0.000000000000000000000000000001f))) + (((_e4005.y * _e4005.y) + (_e4006.y * _e4006.y)) / select(0.000000000000000000000000000001f, _e4033, (_e4033 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3963.z * _e3963.z) + (_e3966 * _e3966)) / select(0.000000000000000000000000000001f, _e3994, (_e3994 > 0.000000000000000000000000000001f))) + (((_e4005.z * _e4005.z) + (_e4006.z * _e4006.z)) / select(0.000000000000000000000000000001f, _e4034, (_e4034 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                }
                                                                let _e4077 = phi_4443_;
                                                                phi_4444_ = _e4077;
                                                            }
                                                            let _e4097 = phi_4444_;
                                                            let _e4099 = (4f * _e3711);
                                                            phi_4456_ = type_59((_e4097 * (((1f / (_e3762 + _e3776)) * _e3747) / _e4099)), ((_e3747 / _e3762) / _e4099));
                                                        }
                                                        let _e4106 = phi_4456_;
                                                        phi_4457_ = _e4106;
                                                        phi_4458_ = _e3723;
                                                    }
                                                    let _e4108 = phi_4457_;
                                                    let _e4110 = phi_4458_;
                                                    if _e4110 {
                                                        phi_4463_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        phi_4463_ = _e4108;
                                                    }
                                                    let _e4112 = phi_4463_;
                                                    let _e4115 = (_e2038 == 1u);
                                                    if _e4115 {
                                                        phi_4491_ = _e4112;
                                                        phi_4492_ = type_59();
                                                    } else {
                                                        if (_e3711 != _e3711) {
                                                            phi_11086_ = true;
                                                        } else {
                                                            phi_11086_ = (0f >= _e3711);
                                                        }
                                                        let _e4119 = phi_11086_;
                                                        let _e4120 = select(_e3711, 0f, _e4119);
                                                        let _e4121 = (1f / _e2028);
                                                        let _e4125 = ((_e4121 * _e4121) * (1f - (_e4120 * _e4120)));
                                                        if (_e4125 >= 1f) {
                                                            phi_11105_ = 1f;
                                                        } else {
                                                            let _e4128 = sqrt((1f - _e4125));
                                                            if (_e4120 != _e4120) {
                                                                phi_11116_ = true;
                                                            } else {
                                                                phi_11116_ = (_e4128 <= _e4120);
                                                            }
                                                            let _e4132 = phi_11116_;
                                                            let _e4137 = pow(((1f - _e2028) / (1f + _e2028)), 2f);
                                                            phi_11105_ = (_e4137 + ((1f - _e4137) * pow((1f - select(_e4120, _e4128, _e4132)), 5f)));
                                                        }
                                                        let _e4144 = phi_11105_;
                                                        if (_e3686 <= 0f) {
                                                            phi_11155_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_11155_ = (_e2026 * (_e3686 * 0.31830987f));
                                                        }
                                                        let _e4149 = phi_11155_;
                                                        let _e4150 = (1f - _e4144);
                                                        phi_4491_ = type_59();
                                                        phi_4492_ = type_59((_e4112.member + vec3<f32>((_e4149.x * _e4150), (_e4149.y * _e4150), (_e4149.z * _e4150))), ((_e4144 * _e4112.member_1) + (_e4150 * _e3692)));
                                                    }
                                                    let _e4164 = phi_4491_;
                                                    let _e4166 = phi_4492_;
                                                    phi_4493_ = _e4164;
                                                    phi_4494_ = _e4166;
                                                    phi_4495_ = _e4115;
                                                }
                                                let _e4174 = phi_4493_;
                                                let _e4176 = phi_4494_;
                                                let _e4178 = phi_4495_;
                                                if _e4178 {
                                                    phi_4499_ = _e4174;
                                                } else {
                                                    phi_4499_ = _e4176;
                                                }
                                                let _e4180 = phi_4499_;
                                                if (_e3665 > 0f) {
                                                    if (_e3665 >= 300000000000000000000000000000000000000f) {
                                                        phi_11194_ = 1f;
                                                    } else {
                                                        let _e4185 = (_e3665 * _e3665);
                                                        let _e4187 = (_e4185 + (_e4180.member_1 * _e4180.member_1));
                                                        if (_e4187 == 0f) {
                                                            phi_11191_ = 0f;
                                                        } else {
                                                            phi_11191_ = (_e4185 / _e4187);
                                                        }
                                                        let _e4191 = phi_11191_;
                                                        phi_11194_ = _e4191;
                                                    }
                                                    let _e4193 = phi_11194_;
                                                    phi_4508_ = (_e4193 / _e3665);
                                                } else {
                                                    phi_4508_ = 1f;
                                                }
                                                let _e4196 = phi_4508_;
                                                if ((((_e4180.member.x * _e4180.member.x) + (_e4180.member.y * _e4180.member.y)) + (_e4180.member.z * _e4180.member.z)) > 0f) {
                                                    let _e4207 = (vec3<f32>(1f, 1f, 1f) / _e3667);
                                                    local = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                    phi_4541_ = _e237;
                                                    phi_4544_ = _e244;
                                                    phi_4546_ = _e241;
                                                    phi_4550_ = type_49(_e3669, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                    loop {
                                                        let _e4209 = phi_4541_;
                                                        let _e4211 = phi_4544_;
                                                        let _e4213 = phi_4546_;
                                                        let _e4215 = phi_4550_;
                                                        local_53 = _e4215;
                                                        if (_e4209 < _e4213) {
                                                            let _e4384 = global_2.member[_e4209];
                                                            if (_e4384.member_6.x == 0u) {
                                                                let _e4490 = (_e4384.member_1.x - _e4384.member.x);
                                                                let _e4491 = (_e4384.member_1.y - _e4384.member.y);
                                                                let _e4492 = (_e4384.member_1.z - _e4384.member.z);
                                                                let _e4497 = (_e4384.member_2.x - _e4384.member.x);
                                                                let _e4498 = (_e4384.member_2.y - _e4384.member.y);
                                                                let _e4499 = (_e4384.member_2.z - _e4384.member.z);
                                                                let _e4502 = ((_e3667.y * _e4499) - (_e4498 * _e3667.z));
                                                                let _e4505 = ((_e3667.z * _e4497) - (_e4499 * _e3667.x));
                                                                let _e4508 = ((_e3667.x * _e4498) - (_e4497 * _e3667.y));
                                                                let _e4513 = (((_e4490 * _e4502) + (_e4491 * _e4505)) + (_e4492 * _e4508));
                                                                if (abs(_e4513) < 0.000000000001f) {
                                                                    phi_4717_ = vec4<f32>();
                                                                    phi_4718_ = true;
                                                                } else {
                                                                    let _e4516 = (1f / _e4513);
                                                                    let _e4517 = (_e1188 - _e4384.member.x);
                                                                    let _e4518 = (_e1189 - _e4384.member.y);
                                                                    let _e4519 = (_e1190 - _e4384.member.z);
                                                                    let _e4525 = ((((_e4517 * _e4502) + (_e4518 * _e4505)) + (_e4519 * _e4508)) * _e4516);
                                                                    if (_e4525 < 0f) {
                                                                        phi_4707_ = vec4<f32>();
                                                                        phi_4708_ = true;
                                                                        phi_4709_ = false;
                                                                    } else {
                                                                        let _e4527 = (_e4525 > 1f);
                                                                        if _e4527 {
                                                                            phi_4705_ = vec4<f32>();
                                                                            phi_4706_ = false;
                                                                        } else {
                                                                            let _e4530 = ((_e4518 * _e4492) - (_e4491 * _e4519));
                                                                            let _e4533 = ((_e4519 * _e4490) - (_e4492 * _e4517));
                                                                            let _e4536 = ((_e4517 * _e4491) - (_e4490 * _e4518));
                                                                            let _e4542 = ((((_e3667.x * _e4530) + (_e3667.y * _e4533)) + (_e3667.z * _e4536)) * _e4516);
                                                                            if (_e4542 < 0f) {
                                                                                phi_4695_ = vec4<f32>();
                                                                                phi_4696_ = true;
                                                                                phi_4697_ = false;
                                                                            } else {
                                                                                let _e4545 = ((_e4525 + _e4542) > 1f);
                                                                                if _e4545 {
                                                                                    phi_4693_ = vec4<f32>();
                                                                                    phi_4694_ = false;
                                                                                } else {
                                                                                    let _e4551 = ((((_e4497 * _e4530) + (_e4498 * _e4533)) + (_e4499 * _e4536)) * _e4516);
                                                                                    if (_e4551 <= 0f) {
                                                                                        phi_4688_ = vec4<f32>();
                                                                                        phi_4689_ = true;
                                                                                    } else {
                                                                                        let _e4553 = (_e4551 >= _e4215.member);
                                                                                        if _e4553 {
                                                                                            phi_4687_ = vec4<f32>();
                                                                                        } else {
                                                                                            phi_4687_ = vec4<f32>(_e4551, _e4525, _e4542, 1f);
                                                                                        }
                                                                                        let _e4556 = phi_4687_;
                                                                                        phi_4688_ = _e4556;
                                                                                        phi_4689_ = _e4553;
                                                                                    }
                                                                                    let _e4558 = phi_4688_;
                                                                                    let _e4560 = phi_4689_;
                                                                                    phi_4693_ = _e4558;
                                                                                    phi_4694_ = _e4560;
                                                                                }
                                                                                let _e4562 = phi_4693_;
                                                                                let _e4564 = phi_4694_;
                                                                                phi_4695_ = _e4562;
                                                                                phi_4696_ = _e4545;
                                                                                phi_4697_ = _e4564;
                                                                            }
                                                                            let _e4566 = phi_4695_;
                                                                            let _e4568 = phi_4696_;
                                                                            let _e4570 = phi_4697_;
                                                                            phi_4705_ = _e4566;
                                                                            phi_4706_ = select(_e4570, true, _e4568);
                                                                        }
                                                                        let _e4573 = phi_4705_;
                                                                        let _e4575 = phi_4706_;
                                                                        phi_4707_ = _e4573;
                                                                        phi_4708_ = _e4527;
                                                                        phi_4709_ = _e4575;
                                                                    }
                                                                    let _e4577 = phi_4707_;
                                                                    let _e4579 = phi_4708_;
                                                                    let _e4581 = phi_4709_;
                                                                    phi_4717_ = _e4577;
                                                                    phi_4718_ = select(_e4581, true, _e4579);
                                                                }
                                                                let _e4584 = phi_4717_;
                                                                let _e4586 = phi_4718_;
                                                                phi_4864_ = _e4584;
                                                                phi_4865_ = _e4586;
                                                            } else {
                                                                if (_e4384.member_6.x == 1u) {
                                                                    let _e4412 = (_e1188 - _e4384.member.x);
                                                                    let _e4413 = (_e1189 - _e4384.member.y);
                                                                    let _e4414 = (_e1190 - _e4384.member.z);
                                                                    let _e4419 = (((_e4412 * _e3667.x) + (_e4413 * _e3667.y)) + (_e4414 * _e3667.z));
                                                                    if (_e4419 > 0f) {
                                                                        phi_4758_ = select(true, false, ((((_e4412 * _e4412) + (_e4413 * _e4413)) + (_e4414 * _e4414)) > (_e4384.member.w * _e4384.member.w)));
                                                                    } else {
                                                                        phi_4758_ = true;
                                                                    }
                                                                    let _e4432 = phi_4758_;
                                                                    if _e4432 {
                                                                        let _e4436 = (_e4412 - (_e3667.x * _e4419));
                                                                        let _e4437 = (_e4413 - (_e3667.y * _e4419));
                                                                        let _e4438 = (_e4414 - (_e3667.z * _e4419));
                                                                        let _e4447 = ((_e4384.member.w * _e4384.member.w) - (((_e4436 * _e4436) + (_e4437 * _e4437)) + (_e4438 * _e4438)));
                                                                        if (_e4447 < 0f) {
                                                                            phi_4817_ = vec4<f32>();
                                                                            phi_4818_ = true;
                                                                        } else {
                                                                            let _e4449 = sqrt(_e4447);
                                                                            let _e4450 = -(_e4419);
                                                                            let _e4451 = (_e4450 - _e4449);
                                                                            if (_e4451 <= 0f) {
                                                                                phi_4795_ = true;
                                                                            } else {
                                                                                phi_4795_ = (_e4451 >= _e4215.member);
                                                                            }
                                                                            let _e4455 = phi_4795_;
                                                                            if _e4455 {
                                                                                phi_4801_ = (_e4450 + _e4449);
                                                                            } else {
                                                                                phi_4801_ = _e4451;
                                                                            }
                                                                            let _e4458 = phi_4801_;
                                                                            if (_e4458 <= 0f) {
                                                                                phi_4812_ = vec4<f32>();
                                                                                phi_4813_ = true;
                                                                            } else {
                                                                                let _e4460 = (_e4458 >= _e4215.member);
                                                                                if _e4460 {
                                                                                    phi_4811_ = vec4<f32>();
                                                                                } else {
                                                                                    phi_4811_ = vec4<f32>(_e4458, 0f, 0f, 1f);
                                                                                }
                                                                                let _e4463 = phi_4811_;
                                                                                phi_4812_ = _e4463;
                                                                                phi_4813_ = _e4460;
                                                                            }
                                                                            let _e4465 = phi_4812_;
                                                                            let _e4467 = phi_4813_;
                                                                            phi_4817_ = _e4465;
                                                                            phi_4818_ = _e4467;
                                                                        }
                                                                        let _e4469 = phi_4817_;
                                                                        let _e4471 = phi_4818_;
                                                                        phi_4822_ = _e4469;
                                                                        phi_4823_ = _e4471;
                                                                    } else {
                                                                        phi_4822_ = vec4<f32>();
                                                                        phi_4823_ = true;
                                                                    }
                                                                    let _e4473 = phi_4822_;
                                                                    let _e4475 = phi_4823_;
                                                                    phi_4854_ = _e4473;
                                                                    phi_4855_ = _e4475;
                                                                } else {
                                                                    if (abs(_e3667.y) < 0.000000000001f) {
                                                                        phi_4852_ = vec4<f32>();
                                                                        phi_4853_ = true;
                                                                    } else {
                                                                        let _e4394 = ((_e4384.member.x - _e1189) / _e3667.y);
                                                                        if (_e4394 <= 0f) {
                                                                            phi_4847_ = vec4<f32>();
                                                                            phi_4848_ = true;
                                                                        } else {
                                                                            let _e4396 = (_e4394 >= _e4215.member);
                                                                            if _e4396 {
                                                                                phi_4846_ = vec4<f32>();
                                                                            } else {
                                                                                phi_4846_ = vec4<f32>(_e4394, 0f, 0f, 1f);
                                                                            }
                                                                            let _e4399 = phi_4846_;
                                                                            phi_4847_ = _e4399;
                                                                            phi_4848_ = _e4396;
                                                                        }
                                                                        let _e4401 = phi_4847_;
                                                                        let _e4403 = phi_4848_;
                                                                        phi_4852_ = _e4401;
                                                                        phi_4853_ = _e4403;
                                                                    }
                                                                    let _e4405 = phi_4852_;
                                                                    let _e4407 = phi_4853_;
                                                                    phi_4854_ = _e4405;
                                                                    phi_4855_ = _e4407;
                                                                }
                                                                let _e4477 = phi_4854_;
                                                                let _e4479 = phi_4855_;
                                                                phi_4864_ = _e4477;
                                                                phi_4865_ = select(true, false, select(true, false, _e4479));
                                                            }
                                                            let _e4588 = phi_4864_;
                                                            let _e4590 = phi_4865_;
                                                            let _e4593 = select(vec4<f32>(0f, 0f, 0f, 0f), _e4588, vec4(select(true, false, _e4590)));
                                                            let _e4595 = (_e4593.w > 0f);
                                                            if _e4595 {
                                                                let _e4601 = type_49(_e4593.x, _e4215.member_1, _e4215.member_2, _e4215.member_3);
                                                                let _e4606 = type_49(_e4601.member, _e4209, _e4601.member_2, _e4601.member_3);
                                                                phi_4886_ = _e4606;
                                                                phi_4888_ = _e4606;
                                                            } else {
                                                                phi_4886_ = type_49();
                                                                phi_4888_ = _e4215;
                                                            }
                                                            let _e4608 = phi_4886_;
                                                            let _e4610 = phi_4888_;
                                                            let _e4611 = select(true, false, _e4595);
                                                            if _e4611 {
                                                                phi_4894_ = (_e4209 + 1u);
                                                            } else {
                                                                phi_4894_ = u32();
                                                            }
                                                            let _e4614 = phi_4894_;
                                                            phi_4542_ = _e4614;
                                                            phi_4545_ = _e4211;
                                                            phi_4547_ = _e4213;
                                                            phi_4551_ = _e4610;
                                                            phi_4967_ = _e4608;
                                                            phi_4968_ = _e4611;
                                                            phi_4969_ = false;
                                                        } else {
                                                            let _e4217 = (_e4211 == 0u);
                                                            if _e4217 {
                                                                phi_4963_ = u32();
                                                                phi_4964_ = u32();
                                                                phi_4965_ = u32();
                                                            } else {
                                                                let _e4218 = (_e4211 - 1u);
                                                                let _e4219 = (_e4218 < 100u);
                                                                if _e4219 {
                                                                } else {
                                                                    phi_15071_ = true;
                                                                    phi_7701_ = bool();
                                                                    phi_7700_ = type_49();
                                                                    break;
                                                                }
                                                                let _e4221 = local[_e4218];
                                                                let _e4223 = global_3.member[_e4221];
                                                                let _e4234 = ((_e4223.member.x - _e1188) * _e4207.x);
                                                                let _e4236 = ((_e4223.member.y - _e1189) * _e4207.y);
                                                                let _e4238 = ((_e4223.member.z - _e1190) * _e4207.z);
                                                                let _e4245 = ((_e4223.member_2.x - _e1188) * _e4207.x);
                                                                let _e4246 = ((_e4223.member_2.y - _e1189) * _e4207.y);
                                                                let _e4247 = ((_e4223.member_2.z - _e1190) * _e4207.z);
                                                                if (_e4234 == _e4234) {
                                                                    let _e4249 = (_e4245 == _e4245);
                                                                    if _e4249 {
                                                                        let _e4250 = (_e4234 != _e4234);
                                                                        if _e4250 {
                                                                            phi_11291_ = true;
                                                                        } else {
                                                                            phi_11291_ = (_e4245 <= _e4234);
                                                                        }
                                                                        let _e4253 = phi_11291_;
                                                                        let _e4254 = select(_e4234, _e4245, _e4253);
                                                                        if _e4250 {
                                                                            phi_11321_ = true;
                                                                        } else {
                                                                            phi_11321_ = (_e4245 >= _e4234);
                                                                        }
                                                                        let _e4259 = phi_11321_;
                                                                        let _e4260 = select(_e4234, _e4245, _e4259);
                                                                        phi_11268_ = type_52(select(-300000000000000000000000000000000000000f, _e4254, (_e4254 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4260, (_e4260 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_11268_ = type_52();
                                                                    }
                                                                    let _e4265 = phi_11268_;
                                                                    phi_11271_ = _e4265;
                                                                    phi_11272_ = select(true, false, _e4249);
                                                                } else {
                                                                    phi_11271_ = type_52();
                                                                    phi_11272_ = true;
                                                                }
                                                                let _e4268 = phi_11271_;
                                                                let _e4270 = phi_11272_;
                                                                if _e4270 {
                                                                    phi_11277_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_11277_ = _e4268;
                                                                }
                                                                let _e4272 = phi_11277_;
                                                                if (_e4236 == _e4236) {
                                                                    let _e4276 = (_e4246 == _e4246);
                                                                    if _e4276 {
                                                                        let _e4277 = (_e4236 != _e4236);
                                                                        if _e4277 {
                                                                            phi_11378_ = true;
                                                                        } else {
                                                                            phi_11378_ = (_e4246 <= _e4236);
                                                                        }
                                                                        let _e4280 = phi_11378_;
                                                                        let _e4281 = select(_e4236, _e4246, _e4280);
                                                                        if (_e4272.member != _e4272.member) {
                                                                            phi_11393_ = true;
                                                                        } else {
                                                                            phi_11393_ = (_e4281 >= _e4272.member);
                                                                        }
                                                                        let _e4285 = phi_11393_;
                                                                        if _e4277 {
                                                                            phi_11408_ = true;
                                                                        } else {
                                                                            phi_11408_ = (_e4246 >= _e4236);
                                                                        }
                                                                        let _e4289 = phi_11408_;
                                                                        let _e4290 = select(_e4236, _e4246, _e4289);
                                                                        if (_e4272.member_1 != _e4272.member_1) {
                                                                            phi_11423_ = true;
                                                                        } else {
                                                                            phi_11423_ = (_e4290 <= _e4272.member_1);
                                                                        }
                                                                        let _e4294 = phi_11423_;
                                                                        phi_11355_ = type_52(select(_e4272.member, _e4281, _e4285), select(_e4272.member_1, _e4290, _e4294));
                                                                    } else {
                                                                        phi_11355_ = type_52();
                                                                    }
                                                                    let _e4298 = phi_11355_;
                                                                    phi_11358_ = _e4298;
                                                                    phi_11359_ = select(true, false, _e4276);
                                                                } else {
                                                                    phi_11358_ = type_52();
                                                                    phi_11359_ = true;
                                                                }
                                                                let _e4301 = phi_11358_;
                                                                let _e4303 = phi_11359_;
                                                                if _e4303 {
                                                                    phi_11364_ = _e4272;
                                                                } else {
                                                                    phi_11364_ = _e4301;
                                                                }
                                                                let _e4305 = phi_11364_;
                                                                if (_e4238 == _e4238) {
                                                                    let _e4309 = (_e4247 == _e4247);
                                                                    if _e4309 {
                                                                        let _e4310 = (_e4238 != _e4238);
                                                                        if _e4310 {
                                                                            phi_11465_ = true;
                                                                        } else {
                                                                            phi_11465_ = (_e4247 <= _e4238);
                                                                        }
                                                                        let _e4313 = phi_11465_;
                                                                        let _e4314 = select(_e4238, _e4247, _e4313);
                                                                        if (_e4305.member != _e4305.member) {
                                                                            phi_11480_ = true;
                                                                        } else {
                                                                            phi_11480_ = (_e4314 >= _e4305.member);
                                                                        }
                                                                        let _e4318 = phi_11480_;
                                                                        if _e4310 {
                                                                            phi_11495_ = true;
                                                                        } else {
                                                                            phi_11495_ = (_e4247 >= _e4238);
                                                                        }
                                                                        let _e4322 = phi_11495_;
                                                                        let _e4323 = select(_e4238, _e4247, _e4322);
                                                                        if (_e4305.member_1 != _e4305.member_1) {
                                                                            phi_11510_ = true;
                                                                        } else {
                                                                            phi_11510_ = (_e4323 <= _e4305.member_1);
                                                                        }
                                                                        let _e4327 = phi_11510_;
                                                                        phi_11442_ = type_52(select(_e4305.member, _e4314, _e4318), select(_e4305.member_1, _e4323, _e4327));
                                                                    } else {
                                                                        phi_11442_ = type_52();
                                                                    }
                                                                    let _e4331 = phi_11442_;
                                                                    phi_11445_ = _e4331;
                                                                    phi_11446_ = select(true, false, _e4309);
                                                                } else {
                                                                    phi_11445_ = type_52();
                                                                    phi_11446_ = true;
                                                                }
                                                                let _e4334 = phi_11445_;
                                                                let _e4336 = phi_11446_;
                                                                if _e4336 {
                                                                    phi_11451_ = _e4305;
                                                                } else {
                                                                    phi_11451_ = _e4334;
                                                                }
                                                                let _e4338 = phi_11451_;
                                                                if (_e4338.member_1 < 0f) {
                                                                    phi_11247_ = bool();
                                                                    phi_11248_ = true;
                                                                } else {
                                                                    let _e4342 = (_e4338.member > _e4338.member_1);
                                                                    if _e4342 {
                                                                        phi_11244_ = bool();
                                                                    } else {
                                                                        phi_11244_ = (_e4338.member > _e4215.member);
                                                                    }
                                                                    let _e4345 = phi_11244_;
                                                                    phi_11247_ = _e4345;
                                                                    phi_11248_ = _e4342;
                                                                }
                                                                let _e4347 = phi_11247_;
                                                                let _e4349 = phi_11248_;
                                                                if !(select(_e4347, true, _e4349)) {
                                                                    if (_e4223.member_3 > 0u) {
                                                                        phi_4957_ = _e4223.member_1;
                                                                        phi_4958_ = _e4218;
                                                                        phi_4959_ = (_e4223.member_1 + _e4223.member_3);
                                                                    } else {
                                                                        let _e4354 = (_e4211 + 1u);
                                                                        let _e4355 = (_e4354 <= 100u);
                                                                        if _e4355 {
                                                                            if _e4219 {
                                                                            } else {
                                                                                phi_15071_ = true;
                                                                                phi_7701_ = bool();
                                                                                phi_7700_ = type_49();
                                                                                break;
                                                                            }
                                                                            local[_e4218] = _e4223.member_1;
                                                                            if (_e4211 < 100u) {
                                                                            } else {
                                                                                phi_15071_ = true;
                                                                                phi_7701_ = bool();
                                                                                phi_7700_ = type_49();
                                                                                break;
                                                                            }
                                                                            local[_e4211] = (_e4221 + 1u);
                                                                        }
                                                                        phi_4957_ = _e4209;
                                                                        phi_4958_ = select(_e4218, _e4354, _e4355);
                                                                        phi_4959_ = _e4213;
                                                                    }
                                                                    let _e4364 = phi_4957_;
                                                                    let _e4366 = phi_4958_;
                                                                    let _e4368 = phi_4959_;
                                                                    phi_4960_ = _e4364;
                                                                    phi_4961_ = _e4366;
                                                                    phi_4962_ = _e4368;
                                                                } else {
                                                                    phi_4960_ = _e4209;
                                                                    phi_4961_ = _e4218;
                                                                    phi_4962_ = _e4213;
                                                                }
                                                                let _e4370 = phi_4960_;
                                                                let _e4372 = phi_4961_;
                                                                let _e4374 = phi_4962_;
                                                                phi_4963_ = _e4370;
                                                                phi_4964_ = _e4372;
                                                                phi_4965_ = _e4374;
                                                            }
                                                            let _e4376 = phi_4963_;
                                                            let _e4378 = phi_4964_;
                                                            let _e4380 = phi_4965_;
                                                            phi_4542_ = _e4376;
                                                            phi_4545_ = _e4378;
                                                            phi_4547_ = _e4380;
                                                            phi_4551_ = _e4215;
                                                            phi_4967_ = type_49();
                                                            phi_4968_ = select(true, false, _e4217);
                                                            phi_4969_ = _e4217;
                                                        }
                                                        let _e4616 = phi_4542_;
                                                        let _e4618 = phi_4545_;
                                                        let _e4620 = phi_4547_;
                                                        let _e4622 = phi_4551_;
                                                        let _e4624 = phi_4967_;
                                                        let _e4626 = phi_4968_;
                                                        break_if_6_ = _e4626;
                                                        let _e4628 = phi_4969_;
                                                        continue;
                                                        continuing {
                                                            phi_4541_ = _e4616;
                                                            phi_4544_ = _e4618;
                                                            phi_4546_ = _e4620;
                                                            phi_4550_ = _e4622;
                                                            phi_15071_ = _e2061;
                                                            phi_7701_ = _e4628;
                                                            phi_7700_ = _e4624;
                                                            let _e7337 = break_if_6_;
                                                            break if !(_e7337);
                                                        }
                                                    }
                                                    let _e4631 = phi_15071_;
                                                    let _e4633 = phi_7701_;
                                                    let _e4635 = phi_7700_;
                                                    phi_15462_ = _e3661;
                                                    phi_15449_ = _e4631;
                                                    if _e4631 {
                                                        break;
                                                    }
                                                    if _e4633 {
                                                        let _e7068 = local_53;
                                                        phi_5141_ = _e7068;
                                                    } else {
                                                        phi_5141_ = type_49();
                                                    }
                                                    let _e4637 = phi_5141_;
                                                    if select(true, false, _e4633) {
                                                        phi_5146_ = _e4635;
                                                    } else {
                                                        phi_5146_ = _e4637;
                                                    }
                                                    let _e4640 = phi_5146_;
                                                    if (_e4640.member_1 != 4294967295u) {
                                                        phi_5179_ = _e2065;
                                                    } else {
                                                        phi_5179_ = vec3<f32>((_e2065.x + (((_e1051.x * _e4180.member.x) * _e3663.x) * _e4196)), (_e2065.y + (((_e1051.y * _e4180.member.y) * _e3663.y) * _e4196)), (_e2065.z + (((_e1051.z * _e4180.member.z) * _e3663.z) * _e4196)));
                                                    }
                                                    let _e4663 = phi_5179_;
                                                    phi_15452_ = _e4631;
                                                    phi_5180_ = _e4663;
                                                } else {
                                                    phi_15452_ = _e2061;
                                                    phi_5180_ = _e2065;
                                                }
                                                let _e4665 = phi_15452_;
                                                let _e4667 = phi_5180_;
                                                phi_15451_ = _e4665;
                                                phi_5181_ = _e4667;
                                            } else {
                                                phi_15451_ = _e2061;
                                                phi_5181_ = _e2065;
                                            }
                                            let _e4669 = phi_15451_;
                                            let _e4671 = phi_5181_;
                                            phi_15668_ = _e3661;
                                            phi_15450_ = _e4669;
                                            phi_2224_ = _e4671;
                                            phi_2227_ = (_e2067 + 1u);
                                        } else {
                                            phi_15668_ = _e2063;
                                            phi_15450_ = _e2061;
                                            phi_2224_ = vec3<f32>();
                                            phi_2227_ = u32();
                                        }
                                        let _e4674 = phi_15668_;
                                        let _e4676 = phi_15450_;
                                        let _e4678 = phi_2224_;
                                        let _e4680 = phi_2227_;
                                        continue;
                                        continuing {
                                            phi_15151_ = _e4676;
                                            phi_13394_ = _e4674;
                                            phi_2223_ = _e4678;
                                            phi_2226_ = _e4680;
                                            phi_15462_ = _e4674;
                                            phi_15449_ = _e4676;
                                            let _e7340 = break_if_7_;
                                            break if !(_e7340);
                                        }
                                    }
                                    let _e4683 = phi_15462_;
                                    let _e4685 = phi_15449_;
                                    phi_16916_ = _e4685;
                                    phi_7728_ = bool();
                                    phi_7727_ = type_49();
                                    phi_7726_ = bool();
                                    phi_7725_ = bool();
                                    phi_7724_ = bool();
                                    phi_7723_ = bool();
                                    phi_7722_ = bool();
                                    phi_7721_ = bool();
                                    phi_7720_ = bool();
                                    phi_7719_ = vec3<f32>();
                                    phi_7717_ = vec3<f32>();
                                    phi_7713_ = u32();
                                    phi_7711_ = type_14();
                                    phi_7709_ = vec3<f32>();
                                    phi_7708_ = f32();
                                    phi_7707_ = f32();
                                    phi_7706_ = f32();
                                    phi_7704_ = vec3<f32>();
                                    if _e4685 {
                                        break;
                                    }
                                    let _e4686 = (_e228 >= _e208);
                                    if _e4686 {
                                        phi_17014_ = _e4683;
                                        phi_5721_ = vec3<f32>();
                                        phi_5722_ = f32();
                                        phi_5723_ = vec3<f32>();
                                        phi_5724_ = u32();
                                        phi_5725_ = vec3<f32>();
                                        phi_5726_ = false;
                                        phi_5727_ = false;
                                        phi_5728_ = false;
                                        phi_5729_ = false;
                                    } else {
                                        let _e4688 = ((_e4683 * 747796405u) + 2891336453u);
                                        let _e4696 = (((_e4688 >> bitcast<u32>((((_e4688 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4688) * 277803737u);
                                        let _e4699 = ((_e4696 >> bitcast<u32>(22i)) ^ _e4696);
                                        let _e4702 = f32((_e4699 >> bitcast<u32>(8i)));
                                        let _e4705 = ((_e4699 * 747796405u) + 2891336453u);
                                        let _e4713 = (((_e4705 >> bitcast<u32>((((_e4705 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4705) * 277803737u);
                                        let _e4716 = ((_e4713 >> bitcast<u32>(22i)) ^ _e4713);
                                        let _e4719 = f32((_e4716 >> bitcast<u32>(8i)));
                                        let _e4721 = ((_e4716 * 747796405u) + 2891336453u);
                                        let _e4729 = (((_e4721 >> bitcast<u32>((((_e4721 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4721) * 277803737u);
                                        let _e4732 = ((_e4729 >> bitcast<u32>(22i)) ^ _e4729);
                                        let _e4737 = (_e2038 == 0u);
                                        if _e4737 {
                                            phi_11605_ = vec3<f32>();
                                            phi_11606_ = true;
                                        } else {
                                            if (_e2038 == 2u) {
                                                let _e4746 = (((_e1137.x * _e1163.x) + (_e1137.y * _e1163.y)) + (_e1137.z * _e1163.z));
                                                if (_e4746 != _e4746) {
                                                    phi_11646_ = true;
                                                } else {
                                                    phi_11646_ = (0f >= _e4746);
                                                }
                                                let _e4750 = phi_11646_;
                                                let _e4751 = select(_e4746, 0f, _e4750);
                                                let _e4752 = (1f / _e2028);
                                                let _e4756 = ((_e4752 * _e4752) * (1f - (_e4751 * _e4751)));
                                                if (_e4756 >= 1f) {
                                                    phi_11665_ = 1f;
                                                } else {
                                                    let _e4759 = sqrt((1f - _e4756));
                                                    if (_e4751 != _e4751) {
                                                        phi_11676_ = true;
                                                    } else {
                                                        phi_11676_ = (_e4759 <= _e4751);
                                                    }
                                                    let _e4763 = phi_11676_;
                                                    let _e4768 = pow(((1f - _e2028) / (1f + _e2028)), 2f);
                                                    phi_11665_ = (_e4768 + ((1f - _e4768) * pow((1f - select(_e4751, _e4759, _e4763)), 5f)));
                                                }
                                                let _e4775 = phi_11665_;
                                                phi_11596_ = select(true, false, ((f32((_e4732 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e4775));
                                            } else {
                                                phi_11596_ = true;
                                            }
                                            let _e4779 = phi_11596_;
                                            if _e4779 {
                                                let _e4787 = (((_e1137.x * _e1209) + (_e1137.y * _e1210)) + (_e1137.z * _e1212));
                                                let _e4792 = (((_e1137.x * _e1205) + (_e1137.y * _e1215)) + (_e1137.z * _e1216));
                                                let _e4797 = (((_e1137.x * _e1163.x) + (_e1137.y * _e1163.y)) + (_e1137.z * _e1163.z));
                                                if (_e4797 <= 0f) {
                                                    phi_11830_ = vec3<f32>();
                                                    phi_11831_ = true;
                                                } else {
                                                    let _e4800 = (_e2036 * _e4787);
                                                    let _e4801 = (_e2036 * _e4792);
                                                    let _e4808 = (1f / sqrt((((_e4800 * _e4800) + (_e4801 * _e4801)) + (_e4797 * _e4797))));
                                                    let _e4809 = (_e4800 * _e4808);
                                                    let _e4810 = (_e4801 * _e4808);
                                                    let _e4811 = (_e4797 * _e4808);
                                                    let _e4814 = ((_e4809 * _e4809) + (_e4810 * _e4810));
                                                    if (_e4814 > 0f) {
                                                        let _e4817 = sqrt(_e4814);
                                                        phi_11732_ = vec3<f32>((-(_e4810) / _e4817), (_e4809 / _e4817), 0f);
                                                    } else {
                                                        phi_11732_ = vec3<f32>(1f, 0f, 0f);
                                                    }
                                                    let _e4822 = phi_11732_;
                                                    let _e4835 = sqrt((_e4702 * 0.000000059604645f));
                                                    let _e4836 = (_e4719 * 0.00000037450704f);
                                                    let _e4838 = (_e4835 * cos(_e4836));
                                                    let _e4840 = (0.5f * (1f + _e4811));
                                                    let _e4843 = (1f - (_e4838 * _e4838));
                                                    if (_e4843 != _e4843) {
                                                        phi_11888_ = true;
                                                    } else {
                                                        phi_11888_ = (0f >= _e4843);
                                                    }
                                                    let _e4847 = phi_11888_;
                                                    let _e4854 = (((1f - _e4840) * sqrt(select(_e4843, 0f, _e4847))) + ((_e4840 * _e4835) * sin(_e4836)));
                                                    let _e4865 = (_e4843 - (_e4854 * _e4854));
                                                    if (_e4865 != _e4865) {
                                                        phi_11903_ = true;
                                                    } else {
                                                        phi_11903_ = (0f >= _e4865);
                                                    }
                                                    let _e4869 = phi_11903_;
                                                    let _e4871 = sqrt(select(_e4865, 0f, _e4869));
                                                    let _e4877 = (((_e4822.z * _e4838) + (((_e4809 * _e4822.y) - (_e4822.x * _e4810)) * _e4854)) + (_e4811 * _e4871));
                                                    let _e4878 = (_e2036 * (((_e4822.x * _e4838) + (((_e4810 * _e4822.z) - (_e4822.y * _e4811)) * _e4854)) + (_e4809 * _e4871)));
                                                    let _e4879 = (_e2036 * (((_e4822.y * _e4838) + (((_e4811 * _e4822.x) - (_e4822.z * _e4809)) * _e4854)) + (_e4810 * _e4871)));
                                                    if (_e4877 != _e4877) {
                                                        phi_11918_ = true;
                                                    } else {
                                                        phi_11918_ = (0f >= _e4877);
                                                    }
                                                    let _e4883 = phi_11918_;
                                                    let _e4884 = select(_e4877, 0f, _e4883);
                                                    let _e4891 = (1f / sqrt((((_e4878 * _e4878) + (_e4879 * _e4879)) + (_e4884 * _e4884))));
                                                    let _e4892 = (_e4878 * _e4891);
                                                    let _e4893 = (_e4879 * _e4891);
                                                    let _e4894 = (_e4884 * _e4891);
                                                    let _e4900 = (2f * (((_e4787 * _e4892) + (_e4792 * _e4893)) + (_e4797 * _e4894)));
                                                    let _e4903 = (_e4894 * _e4900);
                                                    let _e4906 = (vec3<f32>((_e4892 * _e4900), (_e4893 * _e4900), _e4903) - vec3<f32>(_e4787, _e4792, _e4797));
                                                    let _e4907 = ((_e4903 - _e4797) <= 0f);
                                                    if _e4907 {
                                                        phi_11827_ = vec3<f32>();
                                                    } else {
                                                        phi_11827_ = vec3<f32>((((_e1209 * _e4906.x) + (_e1205 * _e4906.y)) + (_e1163.x * _e4906.z)), (((_e1210 * _e4906.x) + (_e1215 * _e4906.y)) + (_e1163.y * _e4906.z)), (((_e1212 * _e4906.x) + (_e1216 * _e4906.y)) + (_e1163.z * _e4906.z)));
                                                    }
                                                    let _e4928 = phi_11827_;
                                                    phi_11830_ = _e4928;
                                                    phi_11831_ = _e4907;
                                                }
                                                let _e4930 = phi_11830_;
                                                let _e4932 = phi_11831_;
                                                phi_11601_ = select(_e4930, vec3<f32>(0f, 0f, 0f), vec3(_e4932));
                                            } else {
                                                phi_11601_ = vec3<f32>();
                                            }
                                            let _e4936 = phi_11601_;
                                            phi_11605_ = _e4936;
                                            phi_11606_ = select(true, false, _e4779);
                                        }
                                        let _e4939 = phi_11605_;
                                        let _e4941 = phi_11606_;
                                        if _e4941 {
                                            let _e4942 = (_e4702 * 0.00000037450704f);
                                            let _e4944 = ((_e4719 * 0.00000011920929f) - 1f);
                                            let _e4946 = (1f - (_e4944 * _e4944));
                                            if (_e4946 != _e4946) {
                                                phi_12018_ = true;
                                            } else {
                                                phi_12018_ = (0f >= _e4946);
                                            }
                                            let _e4950 = phi_12018_;
                                            let _e4952 = sqrt(select(_e4946, 0f, _e4950));
                                            let _e4957 = (_e1163.x + (_e4952 * cos(_e4942)));
                                            let _e4958 = (_e1163.y + (_e4952 * sin(_e4942)));
                                            let _e4959 = (_e1163.z + _e4944);
                                            let _e4965 = sqrt((((_e4957 * _e4957) + (_e4958 * _e4958)) + (_e4959 * _e4959)));
                                            if (_e4965 < 0.000001f) {
                                                phi_11992_ = _e1163;
                                            } else {
                                                let _e4967 = (1f / _e4965);
                                                phi_11992_ = vec3<f32>((_e4957 * _e4967), (_e4958 * _e4967), (_e4959 * _e4967));
                                            }
                                            let _e4973 = phi_11992_;
                                            phi_11613_ = _e4973;
                                        } else {
                                            phi_11613_ = _e4939;
                                        }
                                        let _e4975 = phi_11613_;
                                        let _e4983 = (((_e4975.x * _e1163.x) + (_e4975.y * _e1163.y)) + (_e4975.z * _e1163.z));
                                        if (_e4983 != _e4983) {
                                            phi_12033_ = true;
                                        } else {
                                            phi_12033_ = (0f >= _e4983);
                                        }
                                        let _e4987 = phi_12033_;
                                        let _e4989 = (select(_e4983, 0f, _e4987) * 0.31830987f);
                                        if _e4737 {
                                            if (_e4983 <= 0f) {
                                                phi_12830_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_12830_ = (_e2032 * (_e4983 * 0.31830987f));
                                            }
                                            let _e5467 = phi_12830_;
                                            phi_5571_ = type_59(_e5467, _e4989);
                                            phi_5572_ = type_59();
                                            phi_5573_ = true;
                                        } else {
                                            let _e4997 = (((_e1137.x * _e1209) + (_e1137.y * _e1210)) + (_e1137.z * _e1212));
                                            let _e5002 = (((_e1137.x * _e1205) + (_e1137.y * _e1215)) + (_e1137.z * _e1216));
                                            let _e5007 = (((_e1137.x * _e1163.x) + (_e1137.y * _e1163.y)) + (_e1137.z * _e1163.z));
                                            let _e5012 = (((_e4975.x * _e1209) + (_e4975.y * _e1210)) + (_e4975.z * _e1212));
                                            let _e5017 = (((_e4975.x * _e1205) + (_e4975.y * _e1215)) + (_e4975.z * _e1216));
                                            if (_e5007 <= 0f) {
                                                phi_5535_ = type_59();
                                                phi_5536_ = true;
                                            } else {
                                                let _e5019 = (_e4983 <= 0f);
                                                if _e5019 {
                                                    phi_5534_ = type_59();
                                                } else {
                                                    let _e5020 = (_e4997 + _e5012);
                                                    let _e5021 = (_e5002 + _e5017);
                                                    let _e5022 = (_e5007 + _e4983);
                                                    let _e5029 = (1f / sqrt((((_e5020 * _e5020) + (_e5021 * _e5021)) + (_e5022 * _e5022))));
                                                    let _e5032 = (_e5022 * _e5029);
                                                    if (_e5032 <= 0f) {
                                                        phi_12143_ = 0f;
                                                    } else {
                                                        let _e5034 = (_e2036 * _e2036);
                                                        let _e5038 = ((((_e5034 - 1f) * _e5032) * _e5032) + 1f);
                                                        phi_12143_ = (_e5034 / ((3.1415927f * _e5038) * _e5038));
                                                    }
                                                    let _e5043 = phi_12143_;
                                                    let _e5044 = (_e5007 * _e5007);
                                                    if (_e5044 >= 1f) {
                                                        phi_12171_ = 0f;
                                                    } else {
                                                        phi_12171_ = ((sqrt((1f + (((_e2036 * _e2036) * ((_e4997 * _e4997) + (_e5002 * _e5002))) / _e5044))) - 1f) * 0.5f);
                                                    }
                                                    let _e5057 = phi_12171_;
                                                    let _e5058 = (1f + _e5057);
                                                    let _e5059 = (_e4983 * _e4983);
                                                    if (_e5059 >= 1f) {
                                                        phi_12199_ = 0f;
                                                    } else {
                                                        phi_12199_ = ((sqrt((1f + (((_e2036 * _e2036) * ((_e5012 * _e5012) + (_e5017 * _e5017))) / _e5059))) - 1f) * 0.5f);
                                                    }
                                                    let _e5072 = phi_12199_;
                                                    let _e5079 = (((_e4997 * (_e5020 * _e5029)) + (_e5002 * (_e5021 * _e5029))) + (_e5007 * _e5032));
                                                    if (_e2034 == 0u) {
                                                        let _e5374 = (1f - _e5079);
                                                        if (_e5374 != _e5374) {
                                                            phi_12702_ = true;
                                                        } else {
                                                            phi_12702_ = (0f >= _e5374);
                                                        }
                                                        let _e5378 = phi_12702_;
                                                        let _e5380 = pow(select(_e5374, 0f, _e5378), 5f);
                                                        let _e5381 = (1f - _e5380);
                                                        phi_5522_ = vec3<f32>(((_e2032.x * _e5381) + _e5380), ((_e2032.y * _e5381) + _e5380), ((_e2032.z * _e5381) + _e5380));
                                                    } else {
                                                        if (_e2034 == 1u) {
                                                            let _e5347 = (1f / _e2028);
                                                            let _e5351 = ((_e5347 * _e5347) * (1f - (_e5079 * _e5079)));
                                                            if (_e5351 >= 1f) {
                                                                phi_12663_ = 1f;
                                                            } else {
                                                                let _e5354 = sqrt((1f - _e5351));
                                                                if (_e5079 != _e5079) {
                                                                    phi_12674_ = true;
                                                                } else {
                                                                    phi_12674_ = (_e5354 <= _e5079);
                                                                }
                                                                let _e5358 = phi_12674_;
                                                                let _e5363 = pow(((1f - _e2028) / (1f + _e2028)), 2f);
                                                                phi_12663_ = (_e5363 + ((1f - _e5363) * pow((1f - select(_e5079, _e5354, _e5358)), 5f)));
                                                            }
                                                            let _e5370 = phi_12663_;
                                                            phi_5521_ = vec3(_e5370);
                                                        } else {
                                                            let _e5083 = select(_e5079, 0f, (_e5079 < 0f));
                                                            let _e5085 = select(_e5083, 1f, (_e5083 > 1f));
                                                            let _e5087 = (1f - (_e5085 * _e5085));
                                                            let _e5100 = ((_e2032.x * _e2032.x) - (_e2030.x * _e2030.x));
                                                            let _e5101 = ((_e2032.y * _e2032.y) - (_e2030.y * _e2030.y));
                                                            let _e5102 = ((_e2032.z * _e2032.z) - (_e2030.z * _e2030.z));
                                                            let _e5104 = (_e2032.x * (_e2030.x + _e2030.x));
                                                            let _e5106 = (_e2032.y * (_e2030.y + _e2030.y));
                                                            let _e5108 = (_e2032.z * (_e2030.z + _e2030.z));
                                                            let _e5115 = ((_e5100 * _e5100) + (_e5104 * _e5104));
                                                            let _e5116 = ((_e5101 * _e5101) + (_e5106 * _e5106));
                                                            let _e5117 = ((_e5102 * _e5102) + (_e5108 * _e5108));
                                                            let _e5122 = ((_e5087 * _e5100) / _e5115);
                                                            let _e5123 = ((_e5087 * _e5101) / _e5116);
                                                            let _e5124 = ((_e5087 * _e5102) / _e5117);
                                                            let _e5125 = (1f - _e5122);
                                                            let _e5126 = (1f - _e5123);
                                                            let _e5127 = (1f - _e5124);
                                                            let _e5129 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5122, _e5123, _e5124));
                                                            let _e5130 = (_e5087 * _e5104);
                                                            let _e5131 = (_e5087 * _e5106);
                                                            let _e5132 = (_e5087 * _e5108);
                                                            let _e5137 = (vec3<f32>(_e5130, _e5131, _e5132) / vec3<f32>(_e5115, _e5116, _e5117));
                                                            let _e5150 = ((_e5129.x * _e5129.x) + (_e5137.x * _e5137.x));
                                                            let _e5151 = ((_e5129.y * _e5129.y) + (_e5137.y * _e5137.y));
                                                            let _e5152 = ((_e5129.z * _e5129.z) + (_e5137.z * _e5137.z));
                                                            if (_e5150 != _e5150) {
                                                                phi_12333_ = true;
                                                            } else {
                                                                phi_12333_ = (0f >= _e5150);
                                                            }
                                                            let _e5156 = phi_12333_;
                                                            let _e5158 = sqrt(select(_e5150, 0f, _e5156));
                                                            if (_e5151 != _e5151) {
                                                                phi_12348_ = true;
                                                            } else {
                                                                phi_12348_ = (0f >= _e5151);
                                                            }
                                                            let _e5162 = phi_12348_;
                                                            let _e5164 = sqrt(select(_e5151, 0f, _e5162));
                                                            if (_e5152 != _e5152) {
                                                                phi_12363_ = true;
                                                            } else {
                                                                phi_12363_ = (0f >= _e5152);
                                                            }
                                                            let _e5168 = phi_12363_;
                                                            let _e5170 = sqrt(select(_e5152, 0f, _e5168));
                                                            let _e5174 = ((_e5158 + _e5125) * 0.5f);
                                                            let _e5175 = ((_e5164 + _e5126) * 0.5f);
                                                            let _e5176 = ((_e5170 + _e5127) * 0.5f);
                                                            if (_e5174 != _e5174) {
                                                                phi_12390_ = true;
                                                            } else {
                                                                phi_12390_ = (0f >= _e5174);
                                                            }
                                                            let _e5180 = phi_12390_;
                                                            let _e5182 = sqrt(select(_e5174, 0f, _e5180));
                                                            if (_e5175 != _e5175) {
                                                                phi_12405_ = true;
                                                            } else {
                                                                phi_12405_ = (0f >= _e5175);
                                                            }
                                                            let _e5186 = phi_12405_;
                                                            let _e5188 = sqrt(select(_e5175, 0f, _e5186));
                                                            if (_e5176 != _e5176) {
                                                                phi_12420_ = true;
                                                            } else {
                                                                phi_12420_ = (0f >= _e5176);
                                                            }
                                                            let _e5192 = phi_12420_;
                                                            let _e5194 = sqrt(select(_e5176, 0f, _e5192));
                                                            let _e5195 = vec3<f32>(_e5182, _e5188, _e5194);
                                                            let _e5199 = ((_e5158 - _e5125) * 0.5f);
                                                            let _e5200 = ((_e5164 - _e5126) * 0.5f);
                                                            let _e5201 = ((_e5170 - _e5127) * 0.5f);
                                                            if (_e5199 != _e5199) {
                                                                phi_12447_ = true;
                                                            } else {
                                                                phi_12447_ = (0f >= _e5199);
                                                            }
                                                            let _e5205 = phi_12447_;
                                                            let _e5207 = sqrt(select(_e5199, 0f, _e5205));
                                                            if (_e5200 != _e5200) {
                                                                phi_12462_ = true;
                                                            } else {
                                                                phi_12462_ = (0f >= _e5200);
                                                            }
                                                            let _e5211 = phi_12462_;
                                                            let _e5213 = sqrt(select(_e5200, 0f, _e5211));
                                                            if (_e5201 != _e5201) {
                                                                phi_12477_ = true;
                                                            } else {
                                                                phi_12477_ = (0f >= _e5201);
                                                            }
                                                            let _e5217 = phi_12477_;
                                                            let _e5219 = sqrt(select(_e5201, 0f, _e5217));
                                                            if ((_e5130 / _e5115) < 0f) {
                                                                phi_5398_ = -(_e5207);
                                                            } else {
                                                                phi_5398_ = _e5207;
                                                            }
                                                            let _e5223 = phi_5398_;
                                                            if ((_e5131 / _e5116) < 0f) {
                                                                phi_5406_ = -(_e5213);
                                                            } else {
                                                                phi_5406_ = _e5213;
                                                            }
                                                            let _e5227 = phi_5406_;
                                                            if ((_e5132 / _e5117) < 0f) {
                                                                phi_5414_ = -(_e5219);
                                                            } else {
                                                                phi_5414_ = _e5219;
                                                            }
                                                            let _e5231 = phi_5414_;
                                                            let _e5235 = (_e2030.x * _e5085);
                                                            let _e5236 = (_e2030.y * _e5085);
                                                            let _e5237 = (_e2030.z * _e5085);
                                                            let _e5247 = vec3<f32>(((_e2032.x * _e5182) - (_e2030.x * _e5223)), ((_e2032.y * _e5188) - (_e2030.y * _e5227)), ((_e2032.z * _e5194) - (_e2030.z * _e5231)));
                                                            let _e5254 = ((_e2032.x * _e5223) + (_e2030.x * _e5182));
                                                            let _e5255 = ((_e2032.y * _e5227) + (_e2030.y * _e5188));
                                                            let _e5256 = ((_e2032.z * _e5231) + (_e2030.z * _e5194));
                                                            let _e5258 = vec3<f32>((_e2032.x * _e5085), (_e2032.y * _e5085), (_e2032.z * _e5085));
                                                            let _e5259 = (_e5258 - _e5195);
                                                            let _e5260 = (_e5235 - _e5223);
                                                            let _e5261 = (_e5236 - _e5227);
                                                            let _e5262 = (_e5237 - _e5231);
                                                            let _e5275 = (_e5258 + _e5195);
                                                            let _e5276 = (_e5235 + _e5223);
                                                            let _e5277 = (_e5236 + _e5227);
                                                            let _e5278 = (_e5237 + _e5231);
                                                            let _e5288 = ((_e5275.x * _e5275.x) + (_e5276 * _e5276));
                                                            let _e5289 = ((_e5275.y * _e5275.y) + (_e5277 * _e5277));
                                                            let _e5290 = ((_e5275.z * _e5275.z) + (_e5278 * _e5278));
                                                            let _e5300 = vec3(_e5085);
                                                            let _e5301 = (_e5300 - _e5247);
                                                            let _e5302 = -(vec3<f32>(_e5254, _e5255, _e5256));
                                                            let _e5318 = (_e5300 + _e5247);
                                                            let _e5328 = ((_e5318.x * _e5318.x) + (_e5254 * _e5254));
                                                            let _e5329 = ((_e5318.y * _e5318.y) + (_e5255 * _e5255));
                                                            let _e5330 = ((_e5318.z * _e5318.z) + (_e5256 * _e5256));
                                                            phi_5521_ = vec3<f32>((((((_e5259.x * _e5259.x) + (_e5260 * _e5260)) / select(0.000000000000000000000000000001f, _e5288, (_e5288 > 0.000000000000000000000000000001f))) + (((_e5301.x * _e5301.x) + (_e5302.x * _e5302.x)) / select(0.000000000000000000000000000001f, _e5328, (_e5328 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5259.y * _e5259.y) + (_e5261 * _e5261)) / select(0.000000000000000000000000000001f, _e5289, (_e5289 > 0.000000000000000000000000000001f))) + (((_e5301.y * _e5301.y) + (_e5302.y * _e5302.y)) / select(0.000000000000000000000000000001f, _e5329, (_e5329 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5259.z * _e5259.z) + (_e5262 * _e5262)) / select(0.000000000000000000000000000001f, _e5290, (_e5290 > 0.000000000000000000000000000001f))) + (((_e5301.z * _e5301.z) + (_e5302.z * _e5302.z)) / select(0.000000000000000000000000000001f, _e5330, (_e5330 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e5373 = phi_5521_;
                                                        phi_5522_ = _e5373;
                                                    }
                                                    let _e5393 = phi_5522_;
                                                    let _e5395 = (4f * _e5007);
                                                    phi_5534_ = type_59((_e5393 * (((1f / (_e5058 + _e5072)) * _e5043) / _e5395)), ((_e5043 / _e5058) / _e5395));
                                                }
                                                let _e5402 = phi_5534_;
                                                phi_5535_ = _e5402;
                                                phi_5536_ = _e5019;
                                            }
                                            let _e5404 = phi_5535_;
                                            let _e5406 = phi_5536_;
                                            if _e5406 {
                                                phi_5541_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                            } else {
                                                phi_5541_ = _e5404;
                                            }
                                            let _e5408 = phi_5541_;
                                            let _e5411 = (_e2038 == 1u);
                                            if _e5411 {
                                                phi_5569_ = _e5408;
                                                phi_5570_ = type_59();
                                            } else {
                                                if (_e5007 != _e5007) {
                                                    phi_12739_ = true;
                                                } else {
                                                    phi_12739_ = (0f >= _e5007);
                                                }
                                                let _e5415 = phi_12739_;
                                                let _e5416 = select(_e5007, 0f, _e5415);
                                                let _e5417 = (1f / _e2028);
                                                let _e5421 = ((_e5417 * _e5417) * (1f - (_e5416 * _e5416)));
                                                if (_e5421 >= 1f) {
                                                    phi_12758_ = 1f;
                                                } else {
                                                    let _e5424 = sqrt((1f - _e5421));
                                                    if (_e5416 != _e5416) {
                                                        phi_12769_ = true;
                                                    } else {
                                                        phi_12769_ = (_e5424 <= _e5416);
                                                    }
                                                    let _e5428 = phi_12769_;
                                                    let _e5433 = pow(((1f - _e2028) / (1f + _e2028)), 2f);
                                                    phi_12758_ = (_e5433 + ((1f - _e5433) * pow((1f - select(_e5416, _e5424, _e5428)), 5f)));
                                                }
                                                let _e5440 = phi_12758_;
                                                if (_e4983 <= 0f) {
                                                    phi_12808_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_12808_ = (_e2026 * (_e4983 * 0.31830987f));
                                                }
                                                let _e5445 = phi_12808_;
                                                let _e5446 = (1f - _e5440);
                                                phi_5569_ = type_59();
                                                phi_5570_ = type_59((_e5408.member + vec3<f32>((_e5445.x * _e5446), (_e5445.y * _e5446), (_e5445.z * _e5446))), ((_e5440 * _e5408.member_1) + (_e5446 * _e4989)));
                                            }
                                            let _e5460 = phi_5569_;
                                            let _e5462 = phi_5570_;
                                            phi_5571_ = _e5460;
                                            phi_5572_ = _e5462;
                                            phi_5573_ = _e5411;
                                        }
                                        let _e5470 = phi_5571_;
                                        let _e5472 = phi_5572_;
                                        let _e5474 = phi_5573_;
                                        if _e5474 {
                                            phi_5577_ = _e5470;
                                        } else {
                                            phi_5577_ = _e5472;
                                        }
                                        let _e5476 = phi_5577_;
                                        let _e5484 = ((((_e4975.x * _e4975.x) + (_e4975.y * _e4975.y)) + (_e4975.z * _e4975.z)) == 0f);
                                        if _e5484 {
                                            phi_17015_ = _e4732;
                                            phi_5715_ = vec3<f32>();
                                            phi_5716_ = u32();
                                            phi_5717_ = vec3<f32>();
                                            phi_5718_ = false;
                                            phi_5719_ = false;
                                            phi_5720_ = false;
                                        } else {
                                            let _e5485 = (_e5476.member_1 <= 0f);
                                            if _e5485 {
                                                phi_17016_ = _e4732;
                                                phi_5710_ = vec3<f32>();
                                                phi_5711_ = u32();
                                                phi_5712_ = vec3<f32>();
                                                phi_5713_ = false;
                                                phi_5714_ = false;
                                            } else {
                                                let _e5487 = (_e5476.member.x / _e5476.member_1);
                                                let _e5489 = (_e5476.member.y / _e5476.member_1);
                                                let _e5491 = (_e5476.member.z / _e5476.member_1);
                                                let _e5500 = global_1.member.member_4[1u];
                                                if (sqrt((((_e5487 * _e5487) + (_e5489 * _e5489)) + (_e5491 * _e5491))) <= _e5500) {
                                                    phi_17018_ = _e4732;
                                                    phi_5664_ = _e1051;
                                                    phi_5665_ = false;
                                                } else {
                                                    let _e5504 = global_1.member.member_7[1u];
                                                    if (_e5504 == 1u) {
                                                        let _e5509 = global_1.member.member_4[3u];
                                                        if (f32(_e228) >= _e5509) {
                                                            let _e5512 = select(_e5489, _e5487, (_e5487 > _e5489));
                                                            let _e5514 = select(_e5491, _e5512, (_e5512 > _e5491));
                                                            if (_e5514 != _e5514) {
                                                                phi_12855_ = true;
                                                            } else {
                                                                phi_12855_ = (1f <= _e5514);
                                                            }
                                                            let _e5518 = phi_12855_;
                                                            let _e5519 = select(_e5514, 1f, _e5518);
                                                            let _e5521 = ((_e4732 * 747796405u) + 2891336453u);
                                                            let _e5529 = (((_e5521 >> bitcast<u32>((((_e5521 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5521) * 277803737u);
                                                            let _e5532 = ((_e5529 >> bitcast<u32>(22i)) ^ _e5529);
                                                            let _e5537 = ((f32((_e5532 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5519);
                                                            if _e5537 {
                                                                phi_5653_ = vec3<f32>();
                                                            } else {
                                                                phi_5653_ = vec3<f32>((_e1051.x / _e5519), (_e1051.y / _e5519), (_e1051.z / _e5519));
                                                            }
                                                            let _e5546 = phi_5653_;
                                                            phi_17021_ = _e5532;
                                                            phi_5655_ = _e5546;
                                                            phi_5656_ = select(true, false, _e5537);
                                                        } else {
                                                            phi_17021_ = _e4732;
                                                            phi_5655_ = _e1051;
                                                            phi_5656_ = true;
                                                        }
                                                        let _e5549 = phi_17021_;
                                                        let _e5551 = phi_5655_;
                                                        let _e5553 = phi_5656_;
                                                        phi_17020_ = _e5549;
                                                        phi_5657_ = _e5551;
                                                        phi_5658_ = _e5553;
                                                    } else {
                                                        phi_17020_ = _e4732;
                                                        phi_5657_ = _e1051;
                                                        phi_5658_ = true;
                                                    }
                                                    let _e5555 = phi_17020_;
                                                    let _e5557 = phi_5657_;
                                                    let _e5559 = phi_5658_;
                                                    phi_17018_ = _e5555;
                                                    phi_5664_ = select(_e1051, _e5557, vec3(_e5559));
                                                    phi_5665_ = select(false, true, _e5559);
                                                }
                                                let _e5564 = phi_17018_;
                                                let _e5566 = phi_5664_;
                                                let _e5568 = phi_5665_;
                                                if _e5568 {
                                                    if ((((_e4975.x * _e1150.x) + (_e4975.y * _e1150.y)) + (_e4975.z * _e1150.z)) < 0f) {
                                                        phi_5694_ = -(_e1150);
                                                    } else {
                                                        phi_5694_ = _e1150;
                                                    }
                                                    let _e5584 = phi_5694_;
                                                    phi_5706_ = vec3<f32>((_e5566.x * _e5487), (_e5566.y * _e5489), (_e5566.z * _e5491));
                                                    phi_5707_ = (_e228 + 1u);
                                                    phi_5708_ = vec3<f32>((_e1060 + (_e5584.x * _e1181)), (_e1062 + (_e5584.y * _e1181)), (_e1064 + (_e5584.z * _e1181)));
                                                } else {
                                                    phi_5706_ = vec3<f32>();
                                                    phi_5707_ = u32();
                                                    phi_5708_ = vec3<f32>();
                                                }
                                                let _e5597 = phi_5706_;
                                                let _e5599 = phi_5707_;
                                                let _e5601 = phi_5708_;
                                                phi_17016_ = _e5564;
                                                phi_5710_ = _e5597;
                                                phi_5711_ = _e5599;
                                                phi_5712_ = _e5601;
                                                phi_5713_ = _e5568;
                                                phi_5714_ = select(true, false, _e5568);
                                            }
                                            let _e5604 = phi_17016_;
                                            let _e5606 = phi_5710_;
                                            let _e5608 = phi_5711_;
                                            let _e5610 = phi_5712_;
                                            let _e5612 = phi_5713_;
                                            let _e5614 = phi_5714_;
                                            phi_17015_ = _e5604;
                                            phi_5715_ = _e5606;
                                            phi_5716_ = _e5608;
                                            phi_5717_ = _e5610;
                                            phi_5718_ = _e5612;
                                            phi_5719_ = _e5485;
                                            phi_5720_ = _e5614;
                                        }
                                        let _e5616 = phi_17015_;
                                        let _e5618 = phi_5715_;
                                        let _e5620 = phi_5716_;
                                        let _e5622 = phi_5717_;
                                        let _e5624 = phi_5718_;
                                        let _e5626 = phi_5719_;
                                        let _e5628 = phi_5720_;
                                        phi_17014_ = _e5616;
                                        phi_5721_ = _e5618;
                                        phi_5722_ = _e5476.member_1;
                                        phi_5723_ = _e4975;
                                        phi_5724_ = _e5620;
                                        phi_5725_ = _e5622;
                                        phi_5726_ = _e5624;
                                        phi_5727_ = _e5484;
                                        phi_5728_ = _e5626;
                                        phi_5729_ = _e5628;
                                    }
                                    let _e5630 = phi_17014_;
                                    let _e5632 = phi_5721_;
                                    let _e5634 = phi_5722_;
                                    let _e5636 = phi_5723_;
                                    let _e5638 = phi_5724_;
                                    let _e5640 = phi_5725_;
                                    let _e5642 = phi_5726_;
                                    let _e5644 = phi_5727_;
                                    let _e5646 = phi_5728_;
                                    let _e5648 = phi_5729_;
                                    phi_17013_ = _e5630;
                                    phi_16919_ = _e4685;
                                    let _e7176 = local_54;
                                    phi_5730_ = _e7176;
                                    let _e7179 = local_55;
                                    phi_5733_ = _e7179;
                                    phi_5734_ = _e5632;
                                    phi_5735_ = _e5634;
                                    phi_5736_ = _e5636;
                                    phi_5737_ = _e5638;
                                    phi_5740_ = _e5640;
                                    phi_5742_ = _e5642;
                                    phi_5743_ = _e4686;
                                    phi_5744_ = _e5644;
                                    phi_5745_ = _e5646;
                                    phi_5747_ = _e5648;
                                } else {
                                    phi_17013_ = _e2040;
                                    phi_16919_ = _e688;
                                    phi_5730_ = vec3<f32>();
                                    phi_5733_ = _e1049;
                                    phi_5734_ = _e2042;
                                    phi_5735_ = _e224;
                                    phi_5736_ = _e2044;
                                    phi_5737_ = _e2046;
                                    phi_5740_ = _e2048;
                                    phi_5742_ = _e2052;
                                    phi_5743_ = false;
                                    phi_5744_ = false;
                                    phi_5745_ = false;
                                    phi_5747_ = false;
                                }
                                let _e5650 = phi_17013_;
                                let _e5652 = phi_16919_;
                                let _e5654 = phi_5730_;
                                let _e5656 = phi_5733_;
                                let _e5658 = phi_5734_;
                                let _e5660 = phi_5735_;
                                let _e5662 = phi_5736_;
                                let _e5664 = phi_5737_;
                                let _e5666 = phi_5740_;
                                let _e5668 = phi_5742_;
                                let _e5670 = phi_5743_;
                                let _e5672 = phi_5744_;
                                let _e5674 = phi_5745_;
                                let _e5676 = phi_5747_;
                                phi_17012_ = _e5650;
                                phi_16918_ = _e5652;
                                phi_5749_ = _e5654;
                                phi_5752_ = _e5656;
                                phi_5753_ = _e5658;
                                phi_5754_ = _e5660;
                                phi_5755_ = _e5662;
                                phi_5756_ = _e5664;
                                phi_5757_ = select(_e230, true, _e2050);
                                phi_5758_ = select(true, false, _e2050);
                                phi_5759_ = _e5666;
                                phi_5761_ = _e5668;
                                phi_5762_ = _e5670;
                                phi_5763_ = _e5672;
                                phi_5764_ = _e5674;
                                phi_5765_ = select(_e2054, false, _e2050);
                                phi_5766_ = _e5676;
                                phi_5767_ = select(_e2056, false, _e2050);
                            }
                            let _e5682 = phi_17012_;
                            let _e5684 = phi_16918_;
                            let _e5686 = phi_5749_;
                            let _e5688 = phi_5752_;
                            let _e5690 = phi_5753_;
                            let _e5692 = phi_5754_;
                            let _e5694 = phi_5755_;
                            let _e5696 = phi_5756_;
                            let _e5698 = phi_5757_;
                            let _e5700 = phi_5758_;
                            let _e5702 = phi_5759_;
                            let _e5704 = phi_5761_;
                            let _e5706 = phi_5762_;
                            let _e5708 = phi_5763_;
                            let _e5710 = phi_5764_;
                            let _e5712 = phi_5765_;
                            let _e5714 = phi_5766_;
                            let _e5716 = phi_5767_;
                            phi_17011_ = _e5682;
                            phi_16917_ = _e5684;
                            phi_5768_ = _e1049;
                            phi_5769_ = _e1051;
                            phi_5770_ = _e1060;
                            phi_5771_ = _e1062;
                            phi_5772_ = _e1064;
                            phi_5773_ = (_e234 + vec3<f32>(_e1054, _e1056, _e1058));
                            phi_5774_ = _e1068;
                            let _e7219 = local_56;
                            phi_5775_ = _e7219;
                            phi_5776_ = _e5686;
                            phi_335_ = _e5688;
                            phi_338_ = _e5690;
                            phi_340_ = _e5692;
                            phi_342_ = _e5694;
                            phi_344_ = _e5696;
                            phi_346_ = _e5698;
                            phi_348_ = _e5700;
                            phi_350_ = _e5702;
                            phi_5782_ = _e5704;
                            phi_5783_ = _e1133;
                            phi_5784_ = _e5706;
                            phi_5785_ = _e5708;
                            phi_5786_ = _e5710;
                            phi_5787_ = _e5712;
                            phi_5788_ = _e5714;
                            phi_5789_ = _e5716;
                        }
                        let _e5718 = phi_17011_;
                        let _e5720 = phi_16917_;
                        let _e5722 = phi_5768_;
                        let _e5724 = phi_5769_;
                        let _e5726 = phi_5770_;
                        let _e5728 = phi_5771_;
                        let _e5730 = phi_5772_;
                        let _e5732 = phi_5773_;
                        let _e5734 = phi_5774_;
                        let _e5736 = phi_5775_;
                        let _e5738 = phi_5776_;
                        let _e5740 = phi_335_;
                        let _e5742 = phi_338_;
                        let _e5744 = phi_340_;
                        let _e5746 = phi_342_;
                        let _e5748 = phi_344_;
                        let _e5750 = phi_346_;
                        let _e5752 = phi_348_;
                        let _e5754 = phi_350_;
                        let _e5756 = phi_5782_;
                        break_if_8_ = _e5756;
                        let _e5758 = phi_5783_;
                        let _e5760 = phi_5784_;
                        let _e5762 = phi_5785_;
                        let _e5764 = phi_5786_;
                        let _e5766 = phi_5787_;
                        let _e5768 = phi_5788_;
                        let _e5770 = phi_5789_;
                        continue;
                        continuing {
                            phi_13221_ = _e5718;
                            phi_13141_ = _e5720;
                            phi_334_ = _e5740;
                            phi_337_ = _e5742;
                            phi_339_ = _e5744;
                            phi_341_ = _e5746;
                            phi_343_ = _e5748;
                            phi_345_ = _e5750;
                            phi_347_ = _e5752;
                            phi_349_ = _e5754;
                            phi_16916_ = _e5720;
                            phi_7728_ = _e856;
                            phi_7727_ = _e854;
                            phi_7726_ = _e5770;
                            phi_7725_ = _e5768;
                            phi_7724_ = _e5766;
                            phi_7723_ = _e5764;
                            phi_7722_ = _e5762;
                            phi_7721_ = _e5760;
                            phi_7720_ = _e5758;
                            phi_7719_ = _e5722;
                            phi_7717_ = _e5738;
                            phi_7713_ = _e5736;
                            phi_7711_ = _e5734;
                            phi_7709_ = _e5732;
                            phi_7708_ = _e5730;
                            phi_7707_ = _e5728;
                            phi_7706_ = _e5726;
                            phi_7704_ = _e5724;
                            let _e7343 = break_if_8_;
                            break if !(_e7343);
                        }
                    }
                    let _e5773 = phi_16916_;
                    let _e5775 = phi_7728_;
                    let _e5777 = phi_7727_;
                    let _e5779 = phi_7726_;
                    let _e5781 = phi_7725_;
                    let _e5783 = phi_7724_;
                    let _e5785 = phi_7723_;
                    let _e5787 = phi_7722_;
                    let _e5789 = phi_7721_;
                    let _e5791 = phi_7720_;
                    let _e5793 = phi_7719_;
                    let _e5795 = phi_7717_;
                    let _e5797 = phi_7713_;
                    let _e5799 = phi_7711_;
                    let _e5801 = phi_7709_;
                    let _e5803 = phi_7708_;
                    let _e5805 = phi_7707_;
                    let _e5807 = phi_7706_;
                    let _e5809 = phi_7704_;
                    if _e5773 {
                        break;
                    }
                    if _e5775 {
                        let _e5811 = local_19;
                        if _e5811 {
                            phi_5948_ = 1f;
                        } else {
                            if ((_e211 & 2u) == 0u) {
                                phi_5947_ = 1f;
                            } else {
                                let _e5816 = global_1.member.member_3[1u];
                                let _e5819 = global_1.member.member_3[2u];
                                let _e5821 = local_20;
                                let _e5824 = local_21;
                                let _e5828 = ((atan2(_e5821.z, _e5824.x) * 0.15915494f) + 0.5f);
                                let _e5832 = local_22;
                                let _e5835 = select(_e5832.y, -1f, (_e5832.y < -1f));
                                let _e5838 = acos(select(_e5835, 1f, (_e5835 > 1f)));
                                let _e5840 = f32(_e5816);
                                let _e5841 = ((_e5828 - floor(_e5828)) * _e5840);
                                let _e5846 = select(select(u32(_e5841), 0u, (_e5841 < 0f)), 4294967295u, (_e5841 > 4294967000f));
                                let _e5847 = (_e5816 - 1u);
                                let _e5850 = f32(_e5819);
                                let _e5851 = ((_e5838 * 0.31830987f) * _e5850);
                                let _e5856 = select(select(u32(_e5851), 0u, (_e5851 < 0f)), 4294967295u, (_e5851 > 4294967000f));
                                let _e5857 = (_e5819 - 1u);
                                let _e5859 = select(_e5856, _e5857, (_e5857 < _e5856));
                                let _e5860 = sin(_e5838);
                                if (_e5860 <= 0f) {
                                    phi_5945_ = 0f;
                                } else {
                                    let _e5864 = global_1.member.member_2[3u];
                                    let _e5867 = global_1.member.member_3[0u];
                                    let _e5871 = (_e5864 + _e5859);
                                    let _e5874 = global_6.member[(_e5871 + 1u)];
                                    let _e5876 = global_6.member[_e5871];
                                    let _e5879 = ((_e5867 + (_e5859 * (_e5816 + 1u))) + select(_e5846, _e5847, (_e5847 < _e5846)));
                                    let _e5882 = global_6.member[(_e5879 + 1u)];
                                    let _e5884 = global_6.member[_e5879];
                                    phi_5945_ = ((((_e5874 - _e5876) * _e5850) * ((_e5882 - _e5884) * _e5840)) / (19.73921f * _e5860));
                                }
                                let _e5891 = phi_5945_;
                                let _e5893 = local_23;
                                if (_e5893 >= 300000000000000000000000000000000000000f) {
                                    phi_12901_ = 1f;
                                } else {
                                    let _e5896 = local_24;
                                    let _e5898 = local_25;
                                    let _e5899 = (_e5896 * _e5898);
                                    let _e5901 = (_e5899 + (_e5891 * _e5891));
                                    if (_e5901 == 0f) {
                                        phi_12898_ = 0f;
                                    } else {
                                        phi_12898_ = (_e5899 / _e5901);
                                    }
                                    let _e5905 = phi_12898_;
                                    phi_12901_ = _e5905;
                                }
                                let _e5907 = phi_12901_;
                                phi_5947_ = _e5907;
                            }
                            let _e5909 = phi_5947_;
                            phi_5948_ = _e5909;
                        }
                        let _e5911 = phi_5948_;
                        let _e5914 = global_1.member.member_3[1u];
                        let _e5917 = global_1.member.member_3[2u];
                        let _e5919 = local_26;
                        let _e5922 = local_27;
                        let _e5926 = ((atan2(_e5919.z, _e5922.x) * 0.15915494f) + 0.5f);
                        let _e5931 = (((_e5926 - floor(_e5926)) * f32(_e5914)) - 0.5f);
                        let _e5933 = local_28;
                        let _e5936 = select(_e5933.y, -1f, (_e5933.y < -1f));
                        let _e5943 = (((acos(select(_e5936, 1f, (_e5936 > 1f))) * 0.31830987f) * f32(_e5917)) - 0.5f);
                        let _e5944 = (_e5917 - 1u);
                        let _e5945 = f32(_e5944);
                        if (0f <= _e5945) {
                        } else {
                            break;
                        }
                        let _e5948 = select(_e5943, 0f, (_e5943 < 0f));
                        let _e5950 = select(_e5948, _e5945, (_e5948 > _e5945));
                        let _e5951 = floor(_e5931);
                        let _e5952 = floor(_e5950);
                        let _e5953 = (_e5931 - _e5951);
                        let _e5954 = (_e5950 - _e5952);
                        let _e5965 = (_e5914 == 0u);
                        if _e5965 {
                            break;
                        }
                        let _e5966 = (bitcast<u32>((select(0i, select(select(i32(_e5951), i32(-2147483648), (_e5951 < -2147483600f)), 2147483647i, (_e5951 > 2147483500f)), (_e5951 == _e5951)) + bitcast<i32>(_e5914))) % _e5914);
                        let _e5971 = select(select(u32(_e5952), 0u, (_e5952 < 0f)), 4294967295u, (_e5952 > 4294967000f));
                        let _e5972 = (_e5971 + 1u);
                        let _e5977 = global_1.member.member_2[2u];
                        let _e5978 = (_e5971 * _e5914);
                        if _e5965 {
                            break;
                        }
                        let _e5979 = (_e5966 % _e5914);
                        let _e5982 = (_e5977 + (3u * (_e5978 + _e5979)));
                        let _e5984 = global_6.member[_e5982];
                        let _e5987 = global_6.member[(_e5982 + 1u)];
                        let _e5990 = global_6.member[(_e5982 + 2u)];
                        if _e5965 {
                            break;
                        }
                        let _e5993 = ((_e5966 + 1u) % _e5914);
                        let _e5996 = (_e5977 + (3u * (_e5978 + _e5993)));
                        let _e5998 = global_6.member[_e5996];
                        let _e6001 = global_6.member[(_e5996 + 1u)];
                        let _e6004 = global_6.member[(_e5996 + 2u)];
                        let _e6012 = (vec3<f32>(_e5984, _e5987, _e5990) + vec3<f32>(((_e5998 - _e5984) * _e5953), ((_e6001 - _e5987) * _e5953), ((_e6004 - _e5990) * _e5953)));
                        let _e6013 = (select(_e5972, _e5944, (_e5944 < _e5972)) * _e5914);
                        if _e5965 {
                            break;
                        }
                        let _e6016 = (_e5977 + (3u * (_e6013 + _e5979)));
                        let _e6018 = global_6.member[_e6016];
                        let _e6021 = global_6.member[(_e6016 + 1u)];
                        let _e6024 = global_6.member[(_e6016 + 2u)];
                        if _e5965 {
                            break;
                        }
                        let _e6028 = (_e5977 + (3u * (_e6013 + _e5993)));
                        let _e6030 = global_6.member[_e6028];
                        let _e6033 = global_6.member[(_e6028 + 1u)];
                        let _e6036 = global_6.member[(_e6028 + 2u)];
                        let _e6044 = (vec3<f32>(_e6018, _e6021, _e6024) + vec3<f32>(((_e6030 - _e6018) * _e5953), ((_e6033 - _e6021) * _e5953), ((_e6036 - _e6024) * _e5953)));
                        let _e6058 = (_e6012 + vec3<f32>(((_e6044.x - _e6012.x) * _e5954), ((_e6044.y - _e6012.y) * _e5954), ((_e6044.z - _e6012.z) * _e5954)));
                        let _e6060 = local_29;
                        let _e6065 = local_30;
                        let _e6070 = local_31;
                        let _e6078 = local_32;
                        let _e6080 = (_e6078.x + ((_e6060.x * _e6058.x) * _e5911));
                        let _e6082 = local_33;
                        let _e6084 = (_e6082.y + ((_e6065.y * _e6058.y) * _e5911));
                        let _e6086 = local_34;
                        let _e6088 = (_e6086.z + ((_e6070.z * _e6058.z) * _e5911));
                        if ((_e211 & 4u) == 0u) {
                            phi_6400_ = vec3<f32>(_e6080, _e6084, _e6088);
                        } else {
                            let _e6093 = local_35;
                            if _e6093 {
                                phi_6328_ = 1f;
                            } else {
                                let _e6095 = global_1.member.member_5;
                                let _e6106 = global_1.member.member_5[3u];
                                if ((((_e5922.x * _e6095.x) + (_e5933.y * _e6095.y)) + (_e5919.z * _e6095.z)) <= _e6106) {
                                    phi_6326_ = 0f;
                                } else {
                                    phi_6326_ = (0.15915494f / (1f - _e6106));
                                }
                                let _e6111 = phi_6326_;
                                let _e6113 = local_36;
                                if (_e6113 >= 300000000000000000000000000000000000000f) {
                                    phi_12970_ = 1f;
                                } else {
                                    let _e6116 = local_37;
                                    let _e6118 = local_38;
                                    let _e6119 = (_e6116 * _e6118);
                                    let _e6121 = (_e6119 + (_e6111 * _e6111));
                                    if (_e6121 == 0f) {
                                        phi_12967_ = 0f;
                                    } else {
                                        phi_12967_ = (_e6119 / _e6121);
                                    }
                                    let _e6125 = phi_12967_;
                                    phi_12970_ = _e6125;
                                }
                                let _e6127 = phi_12970_;
                                phi_6328_ = _e6127;
                            }
                            let _e6129 = phi_6328_;
                            let _e6131 = global_1.member.member_5;
                            let _e6139 = (((_e5922.x * _e6131.x) + (_e5933.y * _e6131.y)) + (_e5919.z * _e6131.z));
                            let _e6142 = global_1.member.member_5[3u];
                            if (_e6139 <= _e6142) {
                                phi_6383_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                if (_e6139 != _e6139) {
                                    phi_12981_ = true;
                                } else {
                                    phi_12981_ = (1f <= _e6139);
                                }
                                let _e6147 = phi_12981_;
                                let _e6153 = global_1.member.member_6[3u];
                                let _e6156 = (1f - pow((acos(select(_e6139, 1f, _e6147)) / _e6153), 2f));
                                if (_e6156 != _e6156) {
                                    phi_12996_ = true;
                                } else {
                                    phi_12996_ = (0f >= _e6156);
                                }
                                let _e6160 = phi_12996_;
                                let _e6165 = global_1.member.member_6;
                                let _e6173 = (((1f - (0.6f * (1f - sqrt(select(_e6156, 0f, _e6160))))) * 1.25f) / (6.2831855f * (1f - _e6142)));
                                phi_6383_ = vec3<f32>((_e6165.x * _e6173), (_e6165.y * _e6173), (_e6165.z * _e6173));
                            }
                            let _e6179 = phi_6383_;
                            phi_6400_ = vec3<f32>((_e6080 + ((_e6060.x * _e6179.x) * _e6129)), (_e6084 + ((_e6065.y * _e6179.y) * _e6129)), (_e6088 + ((_e6070.z * _e6179.z) * _e6129)));
                        }
                        let _e6194 = phi_6400_;
                        phi_6401_ = _e6194;
                    } else {
                        phi_6401_ = vec3<f32>();
                    }
                    let _e6196 = phi_6401_;
                    let _e6197 = select(_e5791, false, _e5775);
                    if _e6197 {
                        let _e6206 = local_39;
                        let _e6210 = local_40;
                        let _e6214 = local_41;
                        let _e6218 = global_4.member[_e5797];
                        if (_e6218.member == 4u) {
                            if (_e6218.member_1 == 1u) {
                                phi_6453_ = true;
                            } else {
                                phi_6453_ = ((((-(_e6206.x) * _e5777.member_3.x) + (-(_e6210.y) * _e5777.member_3.y)) + (-(_e6214.z) * _e5777.member_3.z)) > 0f);
                            }
                            let _e6233 = phi_6453_;
                            if _e6233 {
                                phi_6462_ = vec3<f32>(_e6218.member_4.x, _e6218.member_4.y, _e6218.member_4.z);
                            } else {
                                phi_6462_ = vec3<f32>();
                            }
                            let _e6240 = phi_6462_;
                            phi_6464_ = _e6240;
                            phi_6465_ = select(true, false, _e6233);
                        } else {
                            phi_6464_ = vec3<f32>();
                            phi_6465_ = true;
                        }
                        let _e6243 = phi_6464_;
                        let _e6245 = phi_6465_;
                        let _e6247 = select(_e6243, vec3<f32>(0f, 0f, 0f), vec3(_e6245));
                        let _e6249 = local_42;
                        if _e6249 {
                            phi_6638_ = 1f;
                        } else {
                            if ((_e211 & 1u) == 0u) {
                                phi_6637_ = 1f;
                            } else {
                                if (_e5799.member_6.z > 0u) {
                                    let _e6258 = global_1.member.member_2[1u];
                                    let _e6259 = (_e6258 + (_e5799.member_6.z - 1u));
                                    let _e6262 = global_6.member[(_e6259 + 1u)];
                                    let _e6264 = global_6.member[_e6259];
                                    let _e6269 = global_2.member[_e5777.member_1];
                                    if (_e6269.member_6.x == 1u) {
                                        let _e6354 = local_46;
                                        let _e6356 = (_e6269.member.x - _e6354.x);
                                        let _e6358 = local_47;
                                        let _e6360 = (_e6269.member.y - _e6358.y);
                                        let _e6362 = local_48;
                                        let _e6364 = (_e6269.member.z - _e6362.z);
                                        let _e6369 = (((_e6356 * _e6356) + (_e6360 * _e6360)) + (_e6364 * _e6364));
                                        let _e6370 = (_e6269.member.w * _e6269.member.w);
                                        if (_e6369 <= (_e6370 * 1.000001f)) {
                                            let _e6385 = (_e5801.x - _e6354.x);
                                            let _e6387 = (_e5801.y - _e6358.y);
                                            let _e6389 = (_e5801.z - _e6362.z);
                                            let _e6394 = (((_e6385 * _e6385) + (_e6387 * _e6387)) + (_e6389 * _e6389));
                                            phi_13065_ = (_e6394 / (((abs((((_e5777.member_2.x * _e6385) + (_e5777.member_2.y * _e6387)) + (_e5777.member_2.z * _e6389))) / sqrt(_e6394)) * 12.566371f) * _e6370));
                                        } else {
                                            let _e6373 = (_e6370 / _e6369);
                                            if (_e6373 != _e6373) {
                                                phi_13084_ = true;
                                            } else {
                                                phi_13084_ = (1f <= _e6373);
                                            }
                                            let _e6377 = phi_13084_;
                                            let _e6378 = select(_e6373, 1f, _e6377);
                                            phi_13065_ = (0.15915494f / (_e6378 / (1f + sqrt((1f - _e6378)))));
                                        }
                                        let _e6410 = phi_13065_;
                                        phi_6633_ = _e6410;
                                    } else {
                                        let _e6274 = local_43;
                                        let _e6276 = (_e5807 - _e6274.x);
                                        let _e6278 = local_44;
                                        let _e6280 = (_e5805 - _e6278.y);
                                        let _e6282 = local_45;
                                        let _e6284 = (_e5803 - _e6282.z);
                                        let _e6289 = (((_e6276 * _e6276) + (_e6280 * _e6280)) + (_e6284 * _e6284));
                                        let _e6300 = (abs((((_e5777.member_2.x * _e6276) + (_e5777.member_2.y * _e6280)) + (_e5777.member_2.z * _e6284))) / sqrt(_e6289));
                                        if (_e6300 <= 0f) {
                                            phi_6632_ = 0f;
                                        } else {
                                            if (_e6269.member_6.x == 0u) {
                                                let _e6315 = (_e6269.member_1.x - _e6269.member.x);
                                                let _e6316 = (_e6269.member_1.y - _e6269.member.y);
                                                let _e6317 = (_e6269.member_1.z - _e6269.member.z);
                                                let _e6322 = (_e6269.member_2.x - _e6269.member.x);
                                                let _e6323 = (_e6269.member_2.y - _e6269.member.y);
                                                let _e6324 = (_e6269.member_2.z - _e6269.member.z);
                                                let _e6327 = ((_e6316 * _e6324) - (_e6323 * _e6317));
                                                let _e6330 = ((_e6317 * _e6322) - (_e6324 * _e6315));
                                                let _e6333 = ((_e6315 * _e6323) - (_e6322 * _e6316));
                                                phi_6629_ = (sqrt((((_e6327 * _e6327) + (_e6330 * _e6330)) + (_e6333 * _e6333))) * 0.5f);
                                            } else {
                                                phi_6629_ = ((12.566371f * _e6269.member.w) * _e6269.member.w);
                                            }
                                            let _e6342 = phi_6629_;
                                            phi_6632_ = (_e6289 / (_e6300 * _e6342));
                                        }
                                        let _e6346 = phi_6632_;
                                        phi_6633_ = _e6346;
                                    }
                                    let _e6412 = phi_6633_;
                                    let _e6413 = ((_e6262 - _e6264) * _e6412);
                                    let _e6415 = local_49;
                                    if (_e6415 >= 300000000000000000000000000000000000000f) {
                                        phi_13105_ = 1f;
                                    } else {
                                        let _e6418 = local_50;
                                        let _e6420 = local_51;
                                        let _e6421 = (_e6418 * _e6420);
                                        let _e6423 = (_e6421 + (_e6413 * _e6413));
                                        if (_e6423 == 0f) {
                                            phi_13102_ = 0f;
                                        } else {
                                            phi_13102_ = (_e6421 / _e6423);
                                        }
                                        let _e6427 = phi_13102_;
                                        phi_13105_ = _e6427;
                                    }
                                    let _e6429 = phi_13105_;
                                    phi_6636_ = _e6429;
                                } else {
                                    phi_6636_ = 1f;
                                }
                                let _e6431 = phi_6636_;
                                phi_6637_ = _e6431;
                            }
                            let _e6433 = phi_6637_;
                            phi_6638_ = _e6433;
                        }
                        let _e6435 = phi_6638_;
                        phi_6660_ = vec3<f32>((_e5793.x + ((_e5809.x * _e6247.x) * _e6435)), (_e5793.y + ((_e5809.y * _e6247.y) * _e6435)), (_e5793.z + ((_e5809.z * _e6247.z) * _e6435)));
                    } else {
                        phi_6660_ = vec3<f32>();
                    }
                    let _e6456 = phi_6660_;
                    let _e6457 = select(select(_e5789, false, _e5775), false, _e6197);
                    let _e6463 = select(select(select(_e5787, false, _e5775), false, _e6197), false, _e6457);
                    let _e6469 = select(select(select(select(_e5785, false, _e5775), false, _e6197), false, _e6457), false, _e6463);
                    let _e6475 = select(_e6463, true, _e6469);
                    let _e6481 = select(select(select(select(select(select(_e5783, false, _e5775), false, _e6197), false, _e6457), false, _e6463), false, _e6469), false, _e6475);
                    let _e6486 = select(select(select(select(select(select(select(_e5781, false, _e5775), false, _e6197), false, _e6457), false, _e6463), false, _e6469), false, _e6475), false, _e6481);
                    let _e6491 = select(select(_e6475, false, _e6481), true, _e6486);
                    let _e6496 = select(select(select(select(select(select(_e6457, false, _e6463), false, _e6469), false, _e6475), false, _e6481), false, _e6486), true, _e6491);
                    let _e6500 = select(select(select(select(select(select(select(select(select(select(_e5779, false, _e5775), false, _e6197), false, _e6457), false, _e6463), false, _e6469), false, _e6475), false, _e6481), false, _e6486), false, _e6491), false, _e6496);
                    let _e6503 = select(select(select(select(_e6481, false, _e6486), false, _e6491), false, _e6496), true, _e6500);
                    let _e6508 = select(select(_e6496, false, _e6500), true, _e6503);
                    let _e6514 = select(_e6196, select(_e6456, select(_e5795, _e5793, vec3(_e6503)), vec3(_e6508)), vec3(select(select(select(select(select(select(select(select(select(select(select(_e6197, false, _e6457), false, _e6463), false, _e6469), false, _e6475), false, _e6481), false, _e6486), false, _e6491), false, _e6496), false, _e6500), false, _e6503), true, _e6508)));
                    let _e6517 = (_e6514.x * _e144.w);
                    let _e6519 = (_e6514.y * _e144.w);
                    let _e6521 = (_e6514.z * _e144.w);
                    if (_e6517 == _e6517) {
                        if (_e6519 == _e6519) {
                            if (_e6521 == _e6521) {
                                if (_e129 < arrayLength((&global_8.member))) {
                                } else {
                                    break;
                                }
                                let _e6529 = global_8.member[_e129][0u];
                                global_8.member[_e129][0u] = (_e6529 + _e6517);
                                let _e6534 = global_8.member[_e129][1u];
                                global_8.member[_e129][1u] = (_e6534 + _e6519);
                                let _e6539 = global_8.member[_e129][2u];
                                global_8.member[_e129][2u] = (_e6539 + _e6521);
                                let _e6544 = global_8.member[_e129][3u];
                                global_8.member[_e129][3u] = (_e6544 + 1f);
                            }
                        }
                    }
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
