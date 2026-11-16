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
    var phi_13166_: u32;
    var phi_13089_: bool;
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
    var phi_7805_: bool;
    var phi_7835_: bool;
    var phi_7782_: type_52;
    var phi_7785_: type_52;
    var phi_7786_: bool;
    var phi_7791_: type_52;
    var phi_7892_: bool;
    var phi_7907_: bool;
    var phi_7922_: bool;
    var phi_7937_: bool;
    var phi_7869_: type_52;
    var phi_7872_: type_52;
    var phi_7873_: bool;
    var phi_7878_: type_52;
    var phi_7979_: bool;
    var phi_7994_: bool;
    var phi_8009_: bool;
    var phi_8024_: bool;
    var phi_7956_: type_52;
    var phi_7959_: type_52;
    var phi_7960_: bool;
    var phi_7965_: type_52;
    var phi_7757_: bool;
    var phi_7760_: bool;
    var phi_7761_: bool;
    var phi_767_: u32;
    var phi_768_: u32;
    var phi_769_: u32;
    var phi_770_: u32;
    var phi_771_: u32;
    var phi_772_: u32;
    var phi_773_: u32;
    var phi_774_: u32;
    var phi_775_: u32;
    var phi_656_: vec4<f32>;
    var phi_657_: vec4<f32>;
    var phi_658_: bool;
    var phi_662_: vec4<f32>;
    var phi_663_: bool;
    var phi_609_: bool;
    var phi_615_: f32;
    var phi_625_: vec4<f32>;
    var phi_626_: vec4<f32>;
    var phi_627_: bool;
    var phi_631_: vec4<f32>;
    var phi_632_: bool;
    var phi_664_: vec4<f32>;
    var phi_665_: bool;
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
    var phi_674_: vec4<f32>;
    var phi_675_: bool;
    var phi_696_: type_49;
    var phi_698_: type_49;
    var phi_371_: u32;
    var phi_374_: u32;
    var phi_376_: u32;
    var phi_378_: vec4<f32>;
    var phi_380_: type_49;
    var phi_777_: type_49;
    var phi_778_: bool;
    var phi_779_: bool;
    var phi_13074_: bool;
    var phi_7647_: bool;
    var phi_7645_: type_49;
    var local_2: type_49;
    var local_3: vec4<f32>;
    var phi_947_: type_53;
    var local_4: vec4<f32>;
    var local_5: vec4<f32>;
    var phi_906_: vec3<f32>;
    var phi_907_: vec3<f32>;
    var phi_948_: type_53;
    var local_6: type_49;
    var phi_953_: type_49;
    var phi_954_: type_49;
    var phi_959_: type_49;
    var phi_1106_: vec3<f32>;
    var phi_1109_: vec3<f32>;
    var phi_1111_: u32;
    var phi_1129_: f32;
    var phi_1135_: f32;
    var phi_1142_: f32;
    var phi_1107_: vec3<f32>;
    var phi_1110_: vec3<f32>;
    var phi_1112_: u32;
    var local_7: vec3<f32>;
    var local_8: vec3<f32>;
    var local_9: vec3<f32>;
    var local_10: vec3<f32>;
    var phi_1288_: vec3<f32>;
    var phi_1289_: vec3<f32>;
    var phi_1290_: vec3<f32>;
    var phi_1291_: vec3<f32>;
    var phi_1322_: i32;
    var phi_1325_: u32;
    var phi_1379_: bool;
    var phi_1385_: bool;
    var phi_1392_: u32;
    var phi_1394_: i32;
    var phi_1395_: u32;
    var phi_1323_: i32;
    var phi_1326_: u32;
    var phi_1397_: bool;
    var local_11: u32;
    var phi_1442_: vec3<f32>;
    var phi_1459_: vec3<f32>;
    var phi_8053_: bool;
    var phi_1490_: f32;
    var phi_1602_: vec3<f32>;
    var phi_1608_: f32;
    var phi_8126_: bool;
    var phi_8115_: f32;
    var phi_1698_: vec3<f32>;
    var phi_1699_: vec3<f32>;
    var phi_1700_: vec3<f32>;
    var phi_13289_: u32;
    var phi_1702_: vec3<f32>;
    var phi_1703_: vec3<f32>;
    var phi_1704_: vec3<f32>;
    var phi_1705_: bool;
    var phi_1720_: vec3<f32>;
    var phi_1722_: vec3<f32>;
    var phi_13287_: u32;
    var phi_1723_: vec3<f32>;
    var phi_1724_: vec3<f32>;
    var phi_1725_: vec3<f32>;
    var phi_13286_: u32;
    var phi_1726_: vec3<f32>;
    var phi_1727_: vec3<f32>;
    var phi_1728_: vec3<f32>;
    var phi_1729_: bool;
    var phi_13285_: u32;
    var phi_1730_: vec3<f32>;
    var phi_1731_: vec3<f32>;
    var phi_1732_: vec3<f32>;
    var phi_1733_: bool;
    var phi_1734_: bool;
    var phi_8199_: bool;
    var phi_8361_: bool;
    var phi_8376_: bool;
    var phi_8391_: bool;
    var phi_8418_: bool;
    var phi_8433_: bool;
    var phi_8448_: bool;
    var phi_8475_: bool;
    var phi_8490_: bool;
    var phi_8505_: bool;
    var phi_1913_: f32;
    var phi_1921_: f32;
    var phi_1929_: f32;
    var phi_8702_: bool;
    var phi_8691_: f32;
    var phi_2036_: vec3<f32>;
    var phi_8730_: bool;
    var phi_2037_: vec3<f32>;
    var phi_2038_: vec3<f32>;
    var phi_2039_: vec3<f32>;
    var phi_13668_: vec3<f32>;
    var phi_13592_: vec3<f32>;
    var phi_13514_: u32;
    var phi_13433_: f32;
    var phi_2040_: vec3<f32>;
    var phi_2041_: vec3<f32>;
    var phi_2043_: bool;
    var phi_8745_: bool;
    var phi_8796_: bool;
    var phi_8785_: f32;
    var phi_13640_: vec3<f32>;
    var phi_1563_: vec3<f32>;
    var phi_13989_: vec3<f32>;
    var phi_13784_: f32;
    var phi_13639_: vec3<f32>;
    var phi_13331_: u32;
    var phi_1565_: vec3<f32>;
    var phi_1566_: bool;
    var phi_13935_: vec3<f32>;
    var phi_13744_: f32;
    var phi_13666_: vec3<f32>;
    var phi_13590_: vec3<f32>;
    var phi_13512_: u32;
    var phi_13431_: f32;
    var phi_13355_: u32;
    var phi_13282_: u32;
    var phi_2051_: vec3<f32>;
    var phi_2052_: vec3<f32>;
    var phi_2053_: vec3<f32>;
    var phi_2054_: bool;
    var phi_8862_: bool;
    var phi_2123_: vec3<f32>;
    var phi_13344_: u32;
    var phi_2125_: vec3<f32>;
    var phi_2126_: bool;
    var phi_13343_: u32;
    var phi_2127_: vec3<f32>;
    var phi_2128_: bool;
    var phi_13341_: u32;
    var phi_2134_: vec3<f32>;
    var phi_2135_: bool;
    var phi_2165_: vec3<f32>;
    var phi_2166_: vec3<f32>;
    var phi_2167_: u32;
    var phi_13339_: u32;
    var phi_2169_: vec3<f32>;
    var phi_2170_: vec3<f32>;
    var phi_2171_: u32;
    var phi_2172_: bool;
    var phi_2173_: bool;
    var phi_13338_: u32;
    var phi_2174_: vec3<f32>;
    var phi_2175_: vec3<f32>;
    var phi_2176_: u32;
    var phi_2178_: bool;
    var phi_2179_: bool;
    var phi_2180_: bool;
    var phi_13932_: vec3<f32>;
    var phi_13741_: f32;
    var phi_13663_: vec3<f32>;
    var phi_13587_: vec3<f32>;
    var phi_13509_: u32;
    var phi_13428_: f32;
    var phi_13352_: u32;
    var phi_13337_: u32;
    var phi_2181_: vec3<f32>;
    var phi_2182_: vec3<f32>;
    var phi_2183_: u32;
    var phi_2184_: vec3<f32>;
    var phi_2185_: bool;
    var phi_2186_: bool;
    var phi_2187_: bool;
    var phi_2188_: bool;
    var phi_15093_: bool;
    var phi_13336_: u32;
    var phi_2197_: vec3<f32>;
    var phi_2200_: u32;
    var phi_3407_: u32;
    var phi_3410_: u32;
    var phi_3408_: u32;
    var phi_3411_: u32;
    var local_12: u32;
    var phi_3448_: u32;
    var phi_3451_: u32;
    var phi_3449_: u32;
    var phi_3452_: u32;
    var local_13: u32;
    var local_14: u32;
    var phi_3528_: f32;
    var local_15: u32;
    var local_16: u32;
    var phi_3577_: f32;
    var phi_3733_: f32;
    var phi_4054_: vec3<f32>;
    var phi_4056_: vec3<f32>;
    var phi_15432_: u32;
    var phi_4057_: vec3<f32>;
    var phi_4058_: f32;
    var phi_4059_: vec3<f32>;
    var phi_2880_: u32;
    var phi_2883_: u32;
    var phi_2881_: u32;
    var phi_2884_: u32;
    var local_17: u32;
    var local_18: u32;
    var phi_3276_: f32;
    var phi_3279_: f32;
    var phi_9132_: bool;
    var phi_9113_: f32;
    var phi_3280_: f32;
    var phi_9147_: bool;
    var phi_9170_: bool;
    var phi_9185_: bool;
    var phi_3031_: f32;
    var phi_9200_: bool;
    var phi_9215_: bool;
    var phi_9230_: bool;
    var phi_9295_: bool;
    var phi_9383_: bool;
    var phi_9364_: f32;
    var phi_3096_: type_60;
    var phi_3282_: type_60;
    var phi_3360_: bool;
    var phi_3369_: vec3<f32>;
    var phi_3371_: vec3<f32>;
    var phi_3372_: bool;
    var phi_3377_: vec3<f32>;
    var phi_3378_: f32;
    var phi_15435_: u32;
    var phi_3379_: vec3<f32>;
    var phi_3380_: f32;
    var phi_3381_: vec3<f32>;
    var phi_3382_: f32;
    var phi_15434_: u32;
    var phi_3383_: vec3<f32>;
    var phi_3384_: f32;
    var phi_3385_: vec3<f32>;
    var phi_3386_: f32;
    var phi_15431_: u32;
    var phi_4060_: vec3<f32>;
    var phi_4061_: f32;
    var phi_4062_: vec3<f32>;
    var phi_4063_: f32;
    var phi_9398_: bool;
    var phi_9508_: f32;
    var phi_9536_: f32;
    var phi_9564_: f32;
    var phi_9698_: bool;
    var phi_9713_: bool;
    var phi_9728_: bool;
    var phi_9755_: bool;
    var phi_9770_: bool;
    var phi_9785_: bool;
    var phi_9812_: bool;
    var phi_9827_: bool;
    var phi_9842_: bool;
    var phi_2532_: f32;
    var phi_2540_: f32;
    var phi_2548_: f32;
    var phi_10039_: bool;
    var phi_10028_: f32;
    var phi_2655_: vec3<f32>;
    var phi_10067_: bool;
    var phi_2656_: vec3<f32>;
    var phi_2668_: type_59;
    var phi_2669_: type_59;
    var phi_2670_: bool;
    var phi_2675_: type_59;
    var phi_10104_: bool;
    var phi_10134_: bool;
    var phi_10123_: f32;
    var phi_10173_: vec3<f32>;
    var phi_2703_: type_59;
    var phi_2704_: type_59;
    var phi_10195_: vec3<f32>;
    var phi_2705_: type_59;
    var phi_2706_: type_59;
    var phi_2707_: bool;
    var phi_2711_: type_59;
    var phi_10227_: bool;
    var phi_2796_: f32;
    var phi_10298_: bool;
    var phi_10313_: bool;
    var phi_2852_: vec3<f32>;
    var phi_2853_: vec3<f32>;
    var phi_15469_: u32;
    var phi_2854_: vec3<f32>;
    var phi_2855_: f32;
    var phi_2856_: vec3<f32>;
    var phi_15468_: u32;
    var phi_2857_: vec3<f32>;
    var phi_2858_: f32;
    var phi_2859_: vec3<f32>;
    var phi_15430_: u32;
    var phi_4064_: vec3<f32>;
    var phi_4065_: f32;
    var phi_4066_: vec3<f32>;
    var phi_4067_: f32;
    var phi_2296_: f32;
    var phi_2297_: f32;
    var phi_15429_: u32;
    var phi_4068_: vec3<f32>;
    var phi_4069_: f32;
    var phi_4070_: vec3<f32>;
    var phi_4071_: f32;
    var phi_10328_: bool;
    var phi_10438_: f32;
    var phi_10466_: f32;
    var phi_10494_: f32;
    var phi_10628_: bool;
    var phi_10643_: bool;
    var phi_10658_: bool;
    var phi_10685_: bool;
    var phi_10700_: bool;
    var phi_10715_: bool;
    var phi_10742_: bool;
    var phi_10757_: bool;
    var phi_10772_: bool;
    var phi_4294_: f32;
    var phi_4302_: f32;
    var phi_4310_: f32;
    var phi_10969_: bool;
    var phi_10958_: f32;
    var phi_4417_: vec3<f32>;
    var phi_10997_: bool;
    var phi_4418_: vec3<f32>;
    var phi_4430_: type_59;
    var phi_4431_: type_59;
    var phi_4432_: bool;
    var phi_4437_: type_59;
    var phi_11034_: bool;
    var phi_11064_: bool;
    var phi_11053_: f32;
    var phi_11103_: vec3<f32>;
    var phi_4465_: type_59;
    var phi_4466_: type_59;
    var phi_11125_: vec3<f32>;
    var phi_4467_: type_59;
    var phi_4468_: type_59;
    var phi_4469_: bool;
    var phi_4473_: type_59;
    var phi_11139_: f32;
    var phi_11142_: f32;
    var phi_4482_: f32;
    var phi_4515_: u32;
    var phi_4518_: u32;
    var phi_4520_: u32;
    var phi_4524_: type_49;
    var phi_11239_: bool;
    var phi_11269_: bool;
    var phi_11216_: type_52;
    var phi_11219_: type_52;
    var phi_11220_: bool;
    var phi_11225_: type_52;
    var phi_11326_: bool;
    var phi_11341_: bool;
    var phi_11356_: bool;
    var phi_11371_: bool;
    var phi_11303_: type_52;
    var phi_11306_: type_52;
    var phi_11307_: bool;
    var phi_11312_: type_52;
    var phi_11413_: bool;
    var phi_11428_: bool;
    var phi_11443_: bool;
    var phi_11458_: bool;
    var phi_11390_: type_52;
    var phi_11393_: type_52;
    var phi_11394_: bool;
    var phi_11399_: type_52;
    var phi_11192_: bool;
    var phi_11195_: bool;
    var phi_11196_: bool;
    var phi_4905_: u32;
    var phi_4906_: u32;
    var phi_4907_: u32;
    var phi_4908_: u32;
    var phi_4909_: u32;
    var phi_4910_: u32;
    var phi_4911_: u32;
    var phi_4912_: u32;
    var phi_4913_: u32;
    var phi_4794_: vec4<f32>;
    var phi_4795_: vec4<f32>;
    var phi_4796_: bool;
    var phi_4800_: vec4<f32>;
    var phi_4801_: bool;
    var phi_4748_: bool;
    var phi_4754_: f32;
    var phi_4764_: vec4<f32>;
    var phi_4765_: vec4<f32>;
    var phi_4766_: bool;
    var phi_4770_: vec4<f32>;
    var phi_4771_: bool;
    var phi_4802_: vec4<f32>;
    var phi_4803_: bool;
    var phi_4661_: vec4<f32>;
    var phi_4662_: vec4<f32>;
    var phi_4663_: bool;
    var phi_4667_: vec4<f32>;
    var phi_4668_: bool;
    var phi_4669_: vec4<f32>;
    var phi_4670_: bool;
    var phi_4671_: bool;
    var phi_4679_: vec4<f32>;
    var phi_4680_: bool;
    var phi_4681_: vec4<f32>;
    var phi_4682_: bool;
    var phi_4683_: bool;
    var phi_4691_: vec4<f32>;
    var phi_4692_: bool;
    var phi_4812_: vec4<f32>;
    var phi_4813_: bool;
    var phi_4834_: type_49;
    var phi_4836_: type_49;
    var phi_4842_: u32;
    var phi_4516_: u32;
    var phi_4519_: u32;
    var phi_4521_: u32;
    var phi_4525_: type_49;
    var phi_4915_: type_49;
    var phi_4916_: bool;
    var phi_4917_: bool;
    var phi_15013_: bool;
    var phi_7649_: bool;
    var phi_7648_: type_49;
    var phi_5089_: type_49;
    var phi_5094_: type_49;
    var phi_5127_: vec3<f32>;
    var phi_15391_: bool;
    var phi_5128_: vec3<f32>;
    var phi_15390_: bool;
    var phi_5129_: vec3<f32>;
    var phi_15604_: u32;
    var phi_15389_: bool;
    var phi_2198_: vec3<f32>;
    var phi_2201_: u32;
    var phi_15401_: u32;
    var phi_15388_: bool;
    var phi_11594_: bool;
    var phi_11624_: bool;
    var phi_11613_: f32;
    var phi_11544_: bool;
    var phi_11680_: vec3<f32>;
    var phi_11836_: bool;
    var phi_11851_: bool;
    var phi_11866_: bool;
    var phi_11775_: vec3<f32>;
    var phi_11778_: vec3<f32>;
    var phi_11779_: bool;
    var phi_11549_: vec3<f32>;
    var phi_11553_: vec3<f32>;
    var phi_11554_: bool;
    var phi_11966_: bool;
    var phi_11940_: vec3<f32>;
    var phi_11561_: vec3<f32>;
    var phi_11981_: bool;
    var phi_12091_: f32;
    var phi_12119_: f32;
    var phi_12147_: f32;
    var phi_12281_: bool;
    var phi_12296_: bool;
    var phi_12311_: bool;
    var phi_12338_: bool;
    var phi_12353_: bool;
    var phi_12368_: bool;
    var phi_12395_: bool;
    var phi_12410_: bool;
    var phi_12425_: bool;
    var phi_5346_: f32;
    var phi_5354_: f32;
    var phi_5362_: f32;
    var phi_12622_: bool;
    var phi_12611_: f32;
    var phi_5469_: vec3<f32>;
    var phi_12650_: bool;
    var phi_5470_: vec3<f32>;
    var phi_5482_: type_59;
    var phi_5483_: type_59;
    var phi_5484_: bool;
    var phi_5489_: type_59;
    var phi_12687_: bool;
    var phi_12717_: bool;
    var phi_12706_: f32;
    var phi_12756_: vec3<f32>;
    var phi_5517_: type_59;
    var phi_5518_: type_59;
    var phi_12778_: vec3<f32>;
    var phi_5519_: type_59;
    var phi_5520_: type_59;
    var phi_5521_: bool;
    var phi_5525_: type_59;
    var phi_12803_: bool;
    var phi_5601_: vec3<f32>;
    var phi_16936_: u32;
    var phi_5603_: vec3<f32>;
    var phi_5604_: bool;
    var phi_16935_: u32;
    var phi_5605_: vec3<f32>;
    var phi_5606_: bool;
    var phi_16933_: u32;
    var phi_5612_: vec3<f32>;
    var phi_5613_: bool;
    var phi_5642_: vec3<f32>;
    var phi_5654_: vec3<f32>;
    var phi_5655_: u32;
    var phi_5656_: vec3<f32>;
    var phi_16931_: u32;
    var phi_5658_: vec3<f32>;
    var phi_5659_: u32;
    var phi_5660_: vec3<f32>;
    var phi_5661_: bool;
    var phi_5662_: bool;
    var phi_16930_: u32;
    var phi_5663_: vec3<f32>;
    var phi_5664_: u32;
    var phi_5665_: vec3<f32>;
    var phi_5666_: bool;
    var phi_5667_: bool;
    var phi_5668_: bool;
    var phi_16929_: u32;
    var phi_5669_: vec3<f32>;
    var phi_5670_: f32;
    var phi_5671_: vec3<f32>;
    var phi_5672_: u32;
    var phi_5673_: vec3<f32>;
    var phi_5674_: bool;
    var phi_5675_: bool;
    var phi_5676_: bool;
    var phi_5677_: bool;
    var phi_16928_: u32;
    var phi_16834_: bool;
    var phi_5678_: vec3<f32>;
    var phi_5681_: vec3<f32>;
    var phi_5682_: vec3<f32>;
    var phi_5683_: f32;
    var phi_5684_: vec3<f32>;
    var phi_5685_: u32;
    var phi_5688_: vec3<f32>;
    var phi_5690_: bool;
    var phi_5691_: bool;
    var phi_5692_: bool;
    var phi_5693_: bool;
    var phi_5695_: bool;
    var phi_16927_: u32;
    var phi_16833_: bool;
    var phi_5697_: vec3<f32>;
    var phi_5700_: vec3<f32>;
    var phi_5701_: vec3<f32>;
    var phi_5702_: f32;
    var phi_5703_: vec3<f32>;
    var phi_5704_: u32;
    var phi_5705_: bool;
    var phi_5706_: bool;
    var phi_5707_: vec3<f32>;
    var phi_5709_: bool;
    var phi_5710_: bool;
    var phi_5711_: bool;
    var phi_5712_: bool;
    var phi_5713_: bool;
    var phi_5714_: bool;
    var phi_5715_: bool;
    var phi_16926_: u32;
    var phi_16832_: bool;
    var phi_5716_: vec3<f32>;
    var phi_5717_: vec3<f32>;
    var phi_5718_: f32;
    var phi_5719_: f32;
    var phi_5720_: f32;
    var phi_5721_: vec3<f32>;
    var phi_5722_: type_14;
    var phi_5723_: u32;
    var phi_5724_: vec3<f32>;
    var phi_335_: vec3<f32>;
    var phi_338_: vec3<f32>;
    var phi_340_: f32;
    var phi_342_: vec3<f32>;
    var phi_344_: u32;
    var phi_346_: bool;
    var phi_348_: bool;
    var phi_350_: vec3<f32>;
    var phi_5730_: bool;
    var phi_5731_: bool;
    var phi_5732_: bool;
    var phi_5733_: bool;
    var phi_5734_: bool;
    var phi_5735_: bool;
    var phi_5736_: bool;
    var phi_5737_: bool;
    var phi_16831_: bool;
    var phi_7676_: bool;
    var phi_7675_: type_49;
    var phi_7674_: bool;
    var phi_7673_: bool;
    var phi_7672_: bool;
    var phi_7671_: bool;
    var phi_7670_: bool;
    var phi_7669_: bool;
    var phi_7668_: bool;
    var phi_7667_: vec3<f32>;
    var phi_7665_: vec3<f32>;
    var phi_7661_: u32;
    var phi_7659_: type_14;
    var phi_7657_: vec3<f32>;
    var phi_7656_: f32;
    var phi_7655_: f32;
    var phi_7654_: f32;
    var phi_7652_: vec3<f32>;
    var local_19: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_5893_: f32;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_12846_: f32;
    var phi_12849_: f32;
    var phi_5895_: f32;
    var phi_5896_: f32;
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
    var phi_6274_: f32;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_12915_: f32;
    var phi_12918_: f32;
    var phi_6276_: f32;
    var phi_12929_: bool;
    var phi_12944_: bool;
    var phi_6331_: vec3<f32>;
    var phi_6348_: vec3<f32>;
    var phi_6349_: vec3<f32>;
    var local_39: vec3<f32>;
    var local_40: vec3<f32>;
    var local_41: vec3<f32>;
    var phi_6401_: bool;
    var phi_6410_: vec3<f32>;
    var phi_6412_: vec3<f32>;
    var phi_6413_: bool;
    var local_42: bool;
    var local_43: vec3<f32>;
    var local_44: vec3<f32>;
    var local_45: vec3<f32>;
    var phi_6577_: f32;
    var phi_6580_: f32;
    var local_46: vec3<f32>;
    var local_47: vec3<f32>;
    var local_48: vec3<f32>;
    var phi_13032_: bool;
    var phi_13013_: f32;
    var phi_6581_: f32;
    var local_49: f32;
    var local_50: f32;
    var local_51: f32;
    var phi_13050_: f32;
    var phi_13053_: f32;
    var phi_6584_: f32;
    var phi_6585_: f32;
    var phi_6586_: f32;
    var phi_6608_: vec3<f32>;
    var local_52: type_49;
    var local_53: type_49;
    var local_54: vec3<f32>;
    var local_55: vec3<f32>;
    var local_56: u32;

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
                    phi_13166_ = ((_e184 >> bitcast<u32>(22i)) ^ _e184);
                    phi_13089_ = false;
                    phi_334_ = vec3<f32>(0f, 0f, 0f);
                    phi_337_ = vec3<f32>(1f, 1f, 1f);
                    phi_339_ = 0f;
                    phi_341_ = vec3<f32>((_e148.x * _e201), (_e148.y * _e201), (_e148.z * _e201));
                    phi_343_ = 0u;
                    phi_345_ = false;
                    phi_347_ = true;
                    phi_349_ = vec3<f32>(_e144.x, _e144.y, _e144.z);
                    loop {
                        let _e216 = phi_13166_;
                        let _e218 = phi_13089_;
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
                                    let _e523 = (_e426.member_1.x - _e426.member.x);
                                    let _e524 = (_e426.member_1.y - _e426.member.y);
                                    let _e525 = (_e426.member_1.z - _e426.member.z);
                                    let _e530 = (_e426.member_2.x - _e426.member.x);
                                    let _e531 = (_e426.member_2.y - _e426.member.y);
                                    let _e532 = (_e426.member_2.z - _e426.member.z);
                                    let _e537 = ((_e226.y * _e532) - (_e531 * _e226.z));
                                    let _e541 = ((_e226.z * _e530) - (_e532 * _e226.x));
                                    let _e544 = ((_e226.x * _e531) - (_e530 * _e226.y));
                                    let _e549 = (((_e523 * _e537) + (_e524 * _e541)) + (_e525 * _e544));
                                    if (abs(_e549) < 0.000000000001f) {
                                        phi_549_ = vec4<f32>();
                                        phi_550_ = true;
                                    } else {
                                        let _e552 = (1f / _e549);
                                        let _e554 = (_e234.x - _e426.member.x);
                                        let _e556 = (_e234.y - _e426.member.y);
                                        let _e558 = (_e234.z - _e426.member.z);
                                        let _e564 = ((((_e554 * _e537) + (_e556 * _e541)) + (_e558 * _e544)) * _e552);
                                        if (_e564 < 0f) {
                                            phi_539_ = vec4<f32>();
                                            phi_540_ = true;
                                            phi_541_ = false;
                                        } else {
                                            let _e566 = (_e564 > 1f);
                                            if _e566 {
                                                phi_537_ = vec4<f32>();
                                                phi_538_ = false;
                                            } else {
                                                let _e569 = ((_e556 * _e525) - (_e524 * _e558));
                                                let _e572 = ((_e558 * _e523) - (_e525 * _e554));
                                                let _e575 = ((_e554 * _e524) - (_e523 * _e556));
                                                let _e581 = ((((_e226.x * _e569) + (_e226.y * _e572)) + (_e226.z * _e575)) * _e552);
                                                if (_e581 < 0f) {
                                                    phi_527_ = vec4<f32>();
                                                    phi_528_ = true;
                                                    phi_529_ = false;
                                                } else {
                                                    let _e584 = ((_e564 + _e581) > 1f);
                                                    if _e584 {
                                                        phi_525_ = vec4<f32>();
                                                        phi_526_ = false;
                                                    } else {
                                                        let _e590 = ((((_e530 * _e569) + (_e531 * _e572)) + (_e532 * _e575)) * _e552);
                                                        if (_e590 <= 0f) {
                                                            phi_520_ = vec4<f32>();
                                                            phi_521_ = true;
                                                        } else {
                                                            let _e592 = (_e590 >= _e254.member);
                                                            if _e592 {
                                                                phi_519_ = vec4<f32>();
                                                            } else {
                                                                phi_519_ = vec4<f32>(_e590, _e564, _e581, 1f);
                                                            }
                                                            let _e595 = phi_519_;
                                                            phi_520_ = _e595;
                                                            phi_521_ = _e592;
                                                        }
                                                        let _e597 = phi_520_;
                                                        let _e599 = phi_521_;
                                                        phi_525_ = _e597;
                                                        phi_526_ = _e599;
                                                    }
                                                    let _e601 = phi_525_;
                                                    let _e603 = phi_526_;
                                                    phi_527_ = _e601;
                                                    phi_528_ = _e584;
                                                    phi_529_ = _e603;
                                                }
                                                let _e605 = phi_527_;
                                                let _e607 = phi_528_;
                                                let _e609 = phi_529_;
                                                phi_537_ = _e605;
                                                phi_538_ = select(_e609, true, _e607);
                                            }
                                            let _e612 = phi_537_;
                                            let _e614 = phi_538_;
                                            phi_539_ = _e612;
                                            phi_540_ = _e566;
                                            phi_541_ = _e614;
                                        }
                                        let _e616 = phi_539_;
                                        let _e618 = phi_540_;
                                        let _e620 = phi_541_;
                                        phi_549_ = _e616;
                                        phi_550_ = select(_e620, true, _e618);
                                    }
                                    let _e623 = phi_549_;
                                    let _e625 = phi_550_;
                                    phi_674_ = _e623;
                                    phi_675_ = _e625;
                                } else {
                                    if (_e426.member_6.x == 1u) {
                                        let _e457 = (_e234.x - _e426.member.x);
                                        let _e459 = (_e234.y - _e426.member.y);
                                        let _e461 = (_e234.z - _e426.member.z);
                                        let _e469 = (((_e457 * _e226.x) + (_e459 * _e226.y)) + (_e461 * _e226.z));
                                        let _e473 = (_e457 - (_e226.x * _e469));
                                        let _e474 = (_e459 - (_e226.y * _e469));
                                        let _e475 = (_e461 - (_e226.z * _e469));
                                        let _e484 = ((_e426.member.w * _e426.member.w) - (((_e473 * _e473) + (_e474 * _e474)) + (_e475 * _e475)));
                                        if (_e484 < 0f) {
                                            phi_631_ = vec4<f32>();
                                            phi_632_ = true;
                                        } else {
                                            let _e486 = sqrt(_e484);
                                            let _e487 = -(_e469);
                                            let _e488 = (_e487 - _e486);
                                            if (_e488 <= 0f) {
                                                phi_609_ = true;
                                            } else {
                                                phi_609_ = (_e488 >= _e254.member);
                                            }
                                            let _e492 = phi_609_;
                                            if _e492 {
                                                phi_615_ = (_e487 + _e486);
                                            } else {
                                                phi_615_ = _e488;
                                            }
                                            let _e495 = phi_615_;
                                            if (_e495 <= 0f) {
                                                phi_626_ = vec4<f32>();
                                                phi_627_ = true;
                                            } else {
                                                let _e497 = (_e495 >= _e254.member);
                                                if _e497 {
                                                    phi_625_ = vec4<f32>();
                                                } else {
                                                    phi_625_ = vec4<f32>(_e495, 0f, 0f, 1f);
                                                }
                                                let _e500 = phi_625_;
                                                phi_626_ = _e500;
                                                phi_627_ = _e497;
                                            }
                                            let _e502 = phi_626_;
                                            let _e504 = phi_627_;
                                            phi_631_ = _e502;
                                            phi_632_ = _e504;
                                        }
                                        let _e506 = phi_631_;
                                        let _e508 = phi_632_;
                                        phi_664_ = _e506;
                                        phi_665_ = _e508;
                                    } else {
                                        if (abs(_e226.y) < 0.000000000001f) {
                                            phi_662_ = vec4<f32>();
                                            phi_663_ = true;
                                        } else {
                                            let _e438 = ((_e426.member.x - _e234.y) / _e226.y);
                                            if (_e438 <= 0f) {
                                                phi_657_ = vec4<f32>();
                                                phi_658_ = true;
                                            } else {
                                                let _e440 = (_e438 >= _e254.member);
                                                if _e440 {
                                                    phi_656_ = vec4<f32>();
                                                } else {
                                                    phi_656_ = vec4<f32>(_e438, 0f, 0f, 1f);
                                                }
                                                let _e443 = phi_656_;
                                                phi_657_ = _e443;
                                                phi_658_ = _e440;
                                            }
                                            let _e445 = phi_657_;
                                            let _e447 = phi_658_;
                                            phi_662_ = _e445;
                                            phi_663_ = _e447;
                                        }
                                        let _e449 = phi_662_;
                                        let _e451 = phi_663_;
                                        phi_664_ = _e449;
                                        phi_665_ = _e451;
                                    }
                                    let _e510 = phi_664_;
                                    let _e512 = phi_665_;
                                    phi_674_ = _e510;
                                    phi_675_ = select(true, false, select(true, false, _e512));
                                }
                                let _e627 = phi_674_;
                                let _e629 = phi_675_;
                                let _e632 = select(vec4<f32>(0f, 0f, 0f, 0f), _e627, vec4(select(true, false, _e629)));
                                let _e634 = (_e632.w > 0f);
                                if _e634 {
                                    let _e640 = type_49(_e632.x, _e254.member_1, _e254.member_2, _e254.member_3);
                                    let _e645 = type_49(_e640.member, _e246, _e640.member_2, _e640.member_3);
                                    phi_696_ = _e645;
                                    phi_698_ = _e645;
                                } else {
                                    phi_696_ = type_49();
                                    phi_698_ = _e254;
                                }
                                let _e647 = phi_696_;
                                let _e649 = phi_698_;
                                phi_371_ = (_e246 + 1u);
                                phi_374_ = _e248;
                                phi_376_ = _e250;
                                phi_378_ = select(_e252, _e632, vec4(_e634));
                                phi_380_ = _e649;
                                phi_777_ = _e647;
                                phi_778_ = true;
                                phi_779_ = false;
                            } else {
                                let _e256 = (_e248 == 0u);
                                if _e256 {
                                    phi_773_ = u32();
                                    phi_774_ = u32();
                                    phi_775_ = u32();
                                } else {
                                    let _e257 = (_e248 - 1u);
                                    let _e258 = (_e257 < 100u);
                                    if _e258 {
                                    } else {
                                        phi_13074_ = true;
                                        phi_7647_ = bool();
                                        phi_7645_ = type_49();
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
                                                phi_7805_ = true;
                                            } else {
                                                phi_7805_ = (_e287 <= _e276);
                                            }
                                            let _e295 = phi_7805_;
                                            let _e296 = select(_e276, _e287, _e295);
                                            if _e292 {
                                                phi_7835_ = true;
                                            } else {
                                                phi_7835_ = (_e287 >= _e276);
                                            }
                                            let _e301 = phi_7835_;
                                            let _e302 = select(_e276, _e287, _e301);
                                            phi_7782_ = type_52(select(-300000000000000000000000000000000000000f, _e296, (_e296 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e302, (_e302 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_7782_ = type_52();
                                        }
                                        let _e307 = phi_7782_;
                                        phi_7785_ = _e307;
                                        phi_7786_ = select(true, false, _e291);
                                    } else {
                                        phi_7785_ = type_52();
                                        phi_7786_ = true;
                                    }
                                    let _e310 = phi_7785_;
                                    let _e312 = phi_7786_;
                                    if _e312 {
                                        phi_7791_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_7791_ = _e310;
                                    }
                                    let _e314 = phi_7791_;
                                    if (_e278 == _e278) {
                                        let _e318 = (_e288 == _e288);
                                        if _e318 {
                                            let _e319 = (_e278 != _e278);
                                            if _e319 {
                                                phi_7892_ = true;
                                            } else {
                                                phi_7892_ = (_e288 <= _e278);
                                            }
                                            let _e322 = phi_7892_;
                                            let _e323 = select(_e278, _e288, _e322);
                                            if (_e314.member != _e314.member) {
                                                phi_7907_ = true;
                                            } else {
                                                phi_7907_ = (_e323 >= _e314.member);
                                            }
                                            let _e327 = phi_7907_;
                                            if _e319 {
                                                phi_7922_ = true;
                                            } else {
                                                phi_7922_ = (_e288 >= _e278);
                                            }
                                            let _e331 = phi_7922_;
                                            let _e332 = select(_e278, _e288, _e331);
                                            if (_e314.member_1 != _e314.member_1) {
                                                phi_7937_ = true;
                                            } else {
                                                phi_7937_ = (_e332 <= _e314.member_1);
                                            }
                                            let _e336 = phi_7937_;
                                            phi_7869_ = type_52(select(_e314.member, _e323, _e327), select(_e314.member_1, _e332, _e336));
                                        } else {
                                            phi_7869_ = type_52();
                                        }
                                        let _e340 = phi_7869_;
                                        phi_7872_ = _e340;
                                        phi_7873_ = select(true, false, _e318);
                                    } else {
                                        phi_7872_ = type_52();
                                        phi_7873_ = true;
                                    }
                                    let _e343 = phi_7872_;
                                    let _e345 = phi_7873_;
                                    if _e345 {
                                        phi_7878_ = _e314;
                                    } else {
                                        phi_7878_ = _e343;
                                    }
                                    let _e347 = phi_7878_;
                                    if (_e280 == _e280) {
                                        let _e351 = (_e289 == _e289);
                                        if _e351 {
                                            let _e352 = (_e280 != _e280);
                                            if _e352 {
                                                phi_7979_ = true;
                                            } else {
                                                phi_7979_ = (_e289 <= _e280);
                                            }
                                            let _e355 = phi_7979_;
                                            let _e356 = select(_e280, _e289, _e355);
                                            if (_e347.member != _e347.member) {
                                                phi_7994_ = true;
                                            } else {
                                                phi_7994_ = (_e356 >= _e347.member);
                                            }
                                            let _e360 = phi_7994_;
                                            if _e352 {
                                                phi_8009_ = true;
                                            } else {
                                                phi_8009_ = (_e289 >= _e280);
                                            }
                                            let _e364 = phi_8009_;
                                            let _e365 = select(_e280, _e289, _e364);
                                            if (_e347.member_1 != _e347.member_1) {
                                                phi_8024_ = true;
                                            } else {
                                                phi_8024_ = (_e365 <= _e347.member_1);
                                            }
                                            let _e369 = phi_8024_;
                                            phi_7956_ = type_52(select(_e347.member, _e356, _e360), select(_e347.member_1, _e365, _e369));
                                        } else {
                                            phi_7956_ = type_52();
                                        }
                                        let _e373 = phi_7956_;
                                        phi_7959_ = _e373;
                                        phi_7960_ = select(true, false, _e351);
                                    } else {
                                        phi_7959_ = type_52();
                                        phi_7960_ = true;
                                    }
                                    let _e376 = phi_7959_;
                                    let _e378 = phi_7960_;
                                    if _e378 {
                                        phi_7965_ = _e347;
                                    } else {
                                        phi_7965_ = _e376;
                                    }
                                    let _e380 = phi_7965_;
                                    if (_e380.member_1 < 0f) {
                                        phi_7760_ = bool();
                                        phi_7761_ = true;
                                    } else {
                                        let _e384 = (_e380.member > _e380.member_1);
                                        if _e384 {
                                            phi_7757_ = bool();
                                        } else {
                                            phi_7757_ = (_e380.member > _e254.member);
                                        }
                                        let _e387 = phi_7757_;
                                        phi_7760_ = _e387;
                                        phi_7761_ = _e384;
                                    }
                                    let _e389 = phi_7760_;
                                    let _e391 = phi_7761_;
                                    if !(select(_e389, true, _e391)) {
                                        if (_e262.member_3 > 0u) {
                                            phi_767_ = _e262.member_1;
                                            phi_768_ = _e257;
                                            phi_769_ = (_e262.member_1 + _e262.member_3);
                                        } else {
                                            let _e396 = (_e248 + 1u);
                                            let _e397 = (_e396 <= 100u);
                                            if _e397 {
                                                if _e258 {
                                                } else {
                                                    phi_13074_ = true;
                                                    phi_7647_ = bool();
                                                    phi_7645_ = type_49();
                                                    break;
                                                }
                                                local_1[_e257] = _e262.member_1;
                                                if (_e248 < 100u) {
                                                } else {
                                                    phi_13074_ = true;
                                                    phi_7647_ = bool();
                                                    phi_7645_ = type_49();
                                                    break;
                                                }
                                                local_1[_e248] = (_e260 + 1u);
                                            }
                                            phi_767_ = _e246;
                                            phi_768_ = select(_e257, _e396, _e397);
                                            phi_769_ = _e250;
                                        }
                                        let _e406 = phi_767_;
                                        let _e408 = phi_768_;
                                        let _e410 = phi_769_;
                                        phi_770_ = _e406;
                                        phi_771_ = _e408;
                                        phi_772_ = _e410;
                                    } else {
                                        phi_770_ = _e246;
                                        phi_771_ = _e257;
                                        phi_772_ = _e250;
                                    }
                                    let _e412 = phi_770_;
                                    let _e414 = phi_771_;
                                    let _e416 = phi_772_;
                                    phi_773_ = _e412;
                                    phi_774_ = _e414;
                                    phi_775_ = _e416;
                                }
                                let _e418 = phi_773_;
                                let _e420 = phi_774_;
                                let _e422 = phi_775_;
                                phi_371_ = _e418;
                                phi_374_ = _e420;
                                phi_376_ = _e422;
                                phi_378_ = _e252;
                                phi_380_ = _e254;
                                phi_777_ = type_49();
                                phi_778_ = select(true, false, _e256);
                                phi_779_ = _e256;
                            }
                            let _e654 = phi_371_;
                            let _e656 = phi_374_;
                            let _e658 = phi_376_;
                            let _e660 = phi_378_;
                            let _e662 = phi_380_;
                            let _e664 = phi_777_;
                            let _e666 = phi_778_;
                            let _e668 = phi_779_;
                            continue;
                            continuing {
                                phi_370_ = _e654;
                                phi_373_ = _e656;
                                phi_375_ = _e658;
                                phi_377_ = _e660;
                                phi_379_ = _e662;
                                phi_13074_ = _e218;
                                phi_7647_ = _e668;
                                phi_7645_ = _e664;
                                break if !(_e666);
                            }
                        }
                        let _e671 = phi_13074_;
                        let _e673 = phi_7647_;
                        let _e675 = phi_7645_;
                        phi_16831_ = _e671;
                        phi_7676_ = bool();
                        phi_7675_ = type_49();
                        phi_7674_ = bool();
                        phi_7673_ = bool();
                        phi_7672_ = bool();
                        phi_7671_ = bool();
                        phi_7670_ = bool();
                        phi_7669_ = bool();
                        phi_7668_ = bool();
                        phi_7667_ = vec3<f32>();
                        phi_7665_ = vec3<f32>();
                        phi_7661_ = u32();
                        phi_7659_ = type_14();
                        phi_7657_ = vec3<f32>();
                        phi_7656_ = f32();
                        phi_7655_ = f32();
                        phi_7654_ = f32();
                        phi_7652_ = vec3<f32>();
                        if _e671 {
                            break;
                        }
                        if _e673 {
                            let _e677 = local_2;
                            if (_e677.member_1 != 4294967295u) {
                                let _e681 = global_2.member[_e677.member_1];
                                if (_e681.member_6.x == 0u) {
                                    let _e730 = (_e681.member_1.x - _e681.member.x);
                                    let _e731 = (_e681.member_1.y - _e681.member.y);
                                    let _e732 = (_e681.member_1.z - _e681.member.z);
                                    let _e737 = (_e681.member_2.x - _e681.member.x);
                                    let _e738 = (_e681.member_2.y - _e681.member.y);
                                    let _e739 = (_e681.member_2.z - _e681.member.z);
                                    let _e742 = ((_e731 * _e739) - (_e738 * _e732));
                                    let _e745 = ((_e732 * _e737) - (_e739 * _e730));
                                    let _e748 = ((_e730 * _e738) - (_e737 * _e731));
                                    let _e755 = (1f / sqrt((((_e742 * _e742) + (_e745 * _e745)) + (_e748 * _e748))));
                                    let _e759 = vec3<f32>((_e742 * _e755), (_e745 * _e755), (_e748 * _e755));
                                    if (_e681.member_6.w == 1u) {
                                        let _e768 = local_4;
                                        let _e772 = local_5;
                                        let _e774 = ((1f - _e768.y) - _e772.z);
                                        let _e795 = (((_e681.member_3.x * _e774) + (_e681.member_4.x * _e768.y)) + (_e681.member_5.x * _e772.z));
                                        let _e796 = (((_e681.member_3.y * _e774) + (_e681.member_4.y * _e768.y)) + (_e681.member_5.y * _e772.z));
                                        let _e797 = (((_e681.member_3.z * _e774) + (_e681.member_4.z * _e768.y)) + (_e681.member_5.z * _e772.z));
                                        let _e802 = (((_e795 * _e795) + (_e796 * _e796)) + (_e797 * _e797));
                                        if (_e802 > 0f) {
                                            let _e805 = (1f / sqrt(_e802));
                                            phi_906_ = vec3<f32>((_e795 * _e805), (_e796 * _e805), (_e797 * _e805));
                                        } else {
                                            phi_906_ = _e759;
                                        }
                                        let _e811 = phi_906_;
                                        phi_907_ = _e811;
                                    } else {
                                        phi_907_ = _e759;
                                    }
                                    let _e813 = phi_907_;
                                    phi_948_ = type_53(_e759, _e813);
                                } else {
                                    if (_e681.member_6.x == 1u) {
                                        let _e692 = local_3;
                                        let _e718 = vec3<f32>((((_e234.x + (_e226.x * _e692.x)) - _e681.member.x) / _e681.member.w), (((_e234.y + (_e226.y * _e692.x)) - _e681.member.y) / _e681.member.w), (((_e234.z + (_e226.z * _e692.x)) - _e681.member.z) / _e681.member.w));
                                        phi_947_ = type_53(_e718, _e718);
                                    } else {
                                        let _e689 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e226.y > 0f)));
                                        phi_947_ = type_53(_e689, _e689);
                                    }
                                    let _e721 = phi_947_;
                                    phi_948_ = _e721;
                                }
                                let _e816 = phi_948_;
                                let _e820 = local_6;
                                let _e825 = type_49(_e820.member, _e820.member_1, _e816.member, _e820.member_3);
                                phi_953_ = type_49(_e825.member, _e825.member_1, _e825.member_2, _e816.member_1);
                            } else {
                                let _e6616 = local_52;
                                phi_953_ = _e6616;
                            }
                            let _e832 = phi_953_;
                            phi_954_ = _e832;
                        } else {
                            phi_954_ = type_49();
                        }
                        let _e834 = phi_954_;
                        if select(true, false, _e673) {
                            phi_959_ = _e675;
                        } else {
                            phi_959_ = _e834;
                        }
                        let _e837 = phi_959_;
                        let _e839 = (_e837.member_1 == 4294967295u);
                        if _e839 {
                            phi_16926_ = _e216;
                            phi_16832_ = _e671;
                            phi_5716_ = vec3<f32>();
                            phi_5717_ = vec3<f32>();
                            phi_5718_ = f32();
                            phi_5719_ = f32();
                            phi_5720_ = f32();
                            phi_5721_ = vec3<f32>();
                            phi_5722_ = type_14();
                            phi_5723_ = u32();
                            phi_5724_ = vec3<f32>();
                            phi_335_ = vec3<f32>();
                            phi_338_ = vec3<f32>();
                            phi_340_ = f32();
                            phi_342_ = vec3<f32>();
                            phi_344_ = u32();
                            phi_346_ = bool();
                            phi_348_ = bool();
                            phi_350_ = vec3<f32>();
                            phi_5730_ = false;
                            phi_5731_ = false;
                            phi_5732_ = false;
                            phi_5733_ = false;
                            phi_5734_ = false;
                            phi_5735_ = false;
                            phi_5736_ = false;
                            phi_5737_ = false;
                        } else {
                            if ((_e211 & 8u) == 0u) {
                                phi_1290_ = _e220;
                                phi_1291_ = _e222;
                            } else {
                                if (_e837.member > 100f) {
                                    let _e845 = global_1.member.member_8;
                                    let _e858 = global_1.member.member_9;
                                    let _e869 = (abs(atan2((((_e226.x * _e858.x) + (_e226.y * _e858.y)) + (_e226.z * _e858.z)), (((_e226.x * _e845.x) + (_e226.y * _e845.y)) + (_e226.z * _e845.z)))) * 0.31830987f);
                                    let _e871 = select(_e869, 0f, (_e869 < 0f));
                                    let _e874 = (select(_e871, 1f, (_e871 > 1f)) * 31f);
                                    let _e876 = ((_e226.y + 1f) * 0.5f);
                                    let _e878 = select(_e876, 0f, (_e876 < 0f));
                                    let _e881 = (select(_e878, 1f, (_e878 > 1f)) * 63f);
                                    let _e884 = (log((_e837.member * 0.01f)) * 0.13156332f);
                                    let _e886 = select(_e884, 0f, (_e884 < 0f));
                                    let _e889 = (select(_e886, 1f, (_e886 > 1f)) * 31f);
                                    let _e894 = select(select(u32(_e874), 0u, (_e874 < 0f)), 4294967295u, (_e874 > 4294967000f));
                                    let _e896 = select(_e894, 30u, (30u < _e894));
                                    let _e901 = select(select(u32(_e881), 0u, (_e881 < 0f)), 4294967295u, (_e881 > 4294967000f));
                                    let _e903 = select(_e901, 62u, (62u < _e901));
                                    let _e908 = select(select(u32(_e889), 0u, (_e889 < 0f)), 4294967295u, (_e889 > 4294967000f));
                                    let _e910 = select(_e908, 30u, (30u < _e908));
                                    let _e912 = (_e874 - f32(_e896));
                                    let _e914 = (_e881 - f32(_e903));
                                    let _e916 = (_e889 - f32(_e910));
                                    phi_1106_ = vec3<f32>(0f, 0f, 0f);
                                    phi_1109_ = vec3<f32>(0f, 0f, 0f);
                                    phi_1111_ = 0u;
                                    loop {
                                        let _e918 = phi_1106_;
                                        let _e920 = phi_1109_;
                                        let _e922 = phi_1111_;
                                        local_7 = _e918;
                                        local_8 = _e918;
                                        local_9 = _e918;
                                        local_10 = _e920;
                                        let _e923 = (_e922 < 8u);
                                        if _e923 {
                                            let _e924 = (_e922 & 1u);
                                            let _e927 = ((_e922 >> bitcast<u32>(1i)) & 1u);
                                            let _e930 = ((_e922 >> bitcast<u32>(2i)) & 1u);
                                            if (_e924 == 1u) {
                                                phi_1129_ = _e912;
                                            } else {
                                                phi_1129_ = (1f - _e912);
                                            }
                                            let _e934 = phi_1129_;
                                            if (_e927 == 1u) {
                                                phi_1135_ = _e914;
                                            } else {
                                                phi_1135_ = (1f - _e914);
                                            }
                                            let _e938 = phi_1135_;
                                            if (_e930 == 1u) {
                                                phi_1142_ = _e916;
                                            } else {
                                                phi_1142_ = (1f - _e916);
                                            }
                                            let _e943 = phi_1142_;
                                            let _e944 = ((_e934 * _e938) * _e943);
                                            let _e950 = global_1.member.member_7[2u];
                                            let _e956 = (_e950 + (6u * (((((_e896 + _e924) * 64u) + (_e903 + _e927)) * 32u) + (_e910 + _e930))));
                                            let _e958 = global_6.member[_e956];
                                            let _e961 = global_6.member[(_e956 + 1u)];
                                            let _e964 = global_6.member[(_e956 + 2u)];
                                            let _e967 = global_6.member[(_e956 + 3u)];
                                            let _e970 = global_6.member[(_e956 + 4u)];
                                            let _e973 = global_6.member[(_e956 + 5u)];
                                            phi_1107_ = vec3<f32>((_e918.x + (_e967 * _e944)), (_e918.y + (_e970 * _e944)), (_e918.z + (_e973 * _e944)));
                                            phi_1110_ = vec3<f32>((_e920.x + (_e958 * _e944)), (_e920.y + (_e961 * _e944)), (_e920.z + (_e964 * _e944)));
                                            phi_1112_ = (_e922 + 1u);
                                        } else {
                                            phi_1107_ = vec3<f32>();
                                            phi_1110_ = vec3<f32>();
                                            phi_1112_ = u32();
                                        }
                                        let _e996 = phi_1107_;
                                        let _e998 = phi_1110_;
                                        let _e1000 = phi_1112_;
                                        continue;
                                        continuing {
                                            phi_1106_ = _e996;
                                            phi_1109_ = _e998;
                                            phi_1111_ = _e1000;
                                            break if !(_e923);
                                        }
                                    }
                                    let _e1004 = local_7;
                                    let _e1009 = local_8;
                                    let _e1014 = local_9;
                                    let _e1025 = local_10;
                                    phi_1288_ = vec3<f32>((_e220.x + (_e222.x * _e1004.x)), (_e220.y + (_e222.y * _e1009.y)), (_e220.z + (_e222.z * _e1014.z)));
                                    phi_1289_ = (_e222 * _e1025);
                                } else {
                                    phi_1288_ = _e220;
                                    phi_1289_ = _e222;
                                }
                                let _e1028 = phi_1288_;
                                let _e1030 = phi_1289_;
                                phi_1290_ = _e1028;
                                phi_1291_ = _e1030;
                            }
                            let _e1032 = phi_1290_;
                            let _e1034 = phi_1291_;
                            let _e1037 = (_e226.x * _e837.member);
                            let _e1039 = (_e226.y * _e837.member);
                            let _e1041 = (_e226.z * _e837.member);
                            let _e1043 = (_e234.x + _e1037);
                            let _e1045 = (_e234.y + _e1039);
                            let _e1047 = (_e234.z + _e1041);
                            let _e1051 = global_2.member[_e837.member_1];
                            phi_1322_ = 0i;
                            phi_1325_ = _e1051.member_6.y;
                            loop {
                                let _e1055 = phi_1322_;
                                let _e1057 = phi_1325_;
                                local_11 = _e1057;
                                local_56 = _e1057;
                                if (_e1055 < 4i) {
                                    let _e1060 = global_4.member[_e1057];
                                    let _e1062 = (_e1060.member != 6u);
                                    if _e1062 {
                                        phi_1394_ = i32();
                                        phi_1395_ = u32();
                                    } else {
                                        let _e1066 = (abs(_e1043) / _e1060.member_6.z);
                                        let _e1077 = (abs(_e1047) / _e1060.member_6.w);
                                        if ((select(select(u32(_e1066), 0u, (_e1066 < 0f)), 4294967295u, (_e1066 > 4294967000f)) % 2u) == 0u) {
                                            phi_1379_ = (_e1043 > 0f);
                                        } else {
                                            phi_1379_ = (_e1043 <= 0f);
                                        }
                                        let _e1088 = phi_1379_;
                                        if ((select(select(u32(_e1077), 0u, (_e1077 < 0f)), 4294967295u, (_e1077 > 4294967000f)) % 2u) == 0u) {
                                            phi_1385_ = (_e1047 > 0f);
                                        } else {
                                            phi_1385_ = (_e1047 <= 0f);
                                        }
                                        let _e1092 = phi_1385_;
                                        if (_e1088 != _e1092) {
                                            phi_1392_ = _e1060.member_1;
                                        } else {
                                            phi_1392_ = _e1060.member_2;
                                        }
                                        let _e1097 = phi_1392_;
                                        phi_1394_ = (_e1055 + 1i);
                                        phi_1395_ = _e1097;
                                    }
                                    let _e1100 = phi_1394_;
                                    let _e1102 = phi_1395_;
                                    phi_1323_ = _e1100;
                                    phi_1326_ = _e1102;
                                    phi_1397_ = select(true, false, _e1062);
                                } else {
                                    phi_1323_ = i32();
                                    phi_1326_ = u32();
                                    phi_1397_ = false;
                                }
                                let _e1105 = phi_1323_;
                                let _e1107 = phi_1326_;
                                let _e1109 = phi_1397_;
                                continue;
                                continuing {
                                    phi_1322_ = _e1105;
                                    phi_1325_ = _e1107;
                                    break if !(_e1109);
                                }
                            }
                            let _e1112 = local_11;
                            let _e1114 = global_4.member[_e1112];
                            let _e1116 = (_e1114.member == 4u);
                            if _e1116 {
                                phi_16927_ = _e216;
                                phi_16833_ = _e671;
                                phi_5697_ = vec3<f32>();
                                phi_5700_ = vec3<f32>();
                                phi_5701_ = vec3<f32>();
                                phi_5702_ = f32();
                                phi_5703_ = vec3<f32>();
                                phi_5704_ = u32();
                                phi_5705_ = bool();
                                phi_5706_ = bool();
                                phi_5707_ = vec3<f32>();
                                phi_5709_ = false;
                                phi_5710_ = false;
                                phi_5711_ = false;
                                phi_5712_ = false;
                                phi_5713_ = false;
                                phi_5714_ = false;
                                phi_5715_ = false;
                            } else {
                                let _e1117 = -(_e226.x);
                                let _e1118 = -(_e226.y);
                                let _e1119 = -(_e226.z);
                                let _e1120 = -(_e226);
                                if ((((_e837.member_2.x * _e1117) + (_e837.member_2.y * _e1118)) + (_e837.member_2.z * _e1119)) < 0f) {
                                    phi_1442_ = -(_e837.member_2);
                                } else {
                                    phi_1442_ = _e837.member_2;
                                }
                                let _e1133 = phi_1442_;
                                if ((((_e837.member_3.x * _e1117) + (_e837.member_3.y * _e1118)) + (_e837.member_3.z * _e1119)) < 0f) {
                                    phi_1459_ = -(_e837.member_3);
                                } else {
                                    phi_1459_ = _e837.member_3;
                                }
                                let _e1146 = phi_1459_;
                                let _e1149 = global_1.member.member_4[2u];
                                let _e1150 = abs(_e1043);
                                let _e1151 = abs(_e1045);
                                let _e1152 = abs(_e1047);
                                let _e1154 = select(_e1151, _e1150, (_e1150 > _e1151));
                                let _e1158 = (0.00002f * (1f + select(_e1152, _e1154, (_e1154 > _e1152))));
                                if (_e1149 != _e1149) {
                                    phi_8053_ = true;
                                } else {
                                    phi_8053_ = (_e1158 >= _e1149);
                                }
                                let _e1162 = phi_8053_;
                                let _e1164 = (select(_e1149, _e1158, _e1162) * 2f);
                                let _e1171 = (_e1043 + (_e1133.x * _e1164));
                                let _e1172 = (_e1045 + (_e1133.y * _e1164));
                                let _e1173 = (_e1047 + (_e1133.z * _e1164));
                                let _e1174 = vec3<f32>(_e1171, _e1172, _e1173);
                                if _e230 {
                                    let _e1177 = global_1.member.member_4[0u];
                                    phi_1490_ = _e1177;
                                } else {
                                    phi_1490_ = 0f;
                                }
                                let _e1179 = phi_1490_;
                                let _e1182 = select(-1f, 1f, (_e1146.z >= 0f));
                                let _e1184 = (-1f / (_e1182 + _e1146.z));
                                let _e1188 = ((_e1146.x * _e1146.y) * _e1184);
                                let _e1192 = (1f + (((_e1182 * _e1146.x) * _e1146.x) * _e1184));
                                let _e1193 = (_e1182 * _e1188);
                                let _e1195 = (-(_e1182) * _e1146.x);
                                let _e1198 = (_e1182 + ((_e1146.y * _e1146.y) * _e1184));
                                let _e1199 = -(_e1146.y);
                                if (_e1114.member == 0u) {
                                    phi_13932_ = vec3<f32>(0f, 0f, 0f);
                                    phi_13741_ = 1.5f;
                                    phi_13663_ = vec3<f32>(0f, 0f, 0f);
                                    phi_13587_ = vec3<f32>(_e1114.member_4.x, _e1114.member_4.y, _e1114.member_4.z);
                                    phi_13509_ = 0u;
                                    phi_13428_ = 1f;
                                    phi_13352_ = 0u;
                                    phi_13337_ = _e216;
                                    phi_2181_ = vec3<f32>();
                                    phi_2182_ = vec3<f32>();
                                    phi_2183_ = u32();
                                    phi_2184_ = vec3<f32>();
                                    phi_2185_ = true;
                                    phi_2186_ = false;
                                    phi_2187_ = false;
                                    phi_2188_ = false;
                                } else {
                                    if (_e1114.member == 1u) {
                                        let _e1748 = (_e1114.member_6.y * _e1114.member_6.y);
                                        if (_e1748 != _e1748) {
                                            phi_8745_ = true;
                                        } else {
                                            phi_8745_ = (_e1179 >= _e1748);
                                        }
                                        let _e1752 = phi_8745_;
                                        let _e1753 = select(_e1748, _e1179, _e1752);
                                        let _e1754 = (_e1753 >= 0.001f);
                                        if _e1754 {
                                            phi_13989_ = vec3<f32>(_e1114.member_4.x, _e1114.member_4.y, _e1114.member_4.z);
                                            phi_13784_ = _e1114.member_6.x;
                                            phi_13639_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13331_ = _e216;
                                            phi_1565_ = vec3<f32>();
                                            phi_1566_ = true;
                                        } else {
                                            let _e1756 = ((_e216 * 747796405u) + 2891336453u);
                                            let _e1764 = (((_e1756 >> bitcast<u32>((((_e1756 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1756) * 277803737u);
                                            let _e1767 = ((_e1764 >> bitcast<u32>(22i)) ^ _e1764);
                                            let _e1777 = abs((((_e1117 * _e1146.x) + (_e1118 * _e1146.y)) + (_e1119 * _e1146.z)));
                                            let _e1780 = (1f / _e1114.member_6.x);
                                            let _e1784 = ((_e1780 * _e1780) * (1f - (_e1777 * _e1777)));
                                            if (_e1784 >= 1f) {
                                                phi_8785_ = 1f;
                                            } else {
                                                let _e1787 = sqrt((1f - _e1784));
                                                if (_e1777 != _e1777) {
                                                    phi_8796_ = true;
                                                } else {
                                                    phi_8796_ = (_e1787 <= _e1777);
                                                }
                                                let _e1791 = phi_8796_;
                                                let _e1796 = pow(((1f - _e1114.member_6.x) / (1f + _e1114.member_6.x)), 2f);
                                                phi_8785_ = (_e1796 + ((1f - _e1796) * pow((1f - select(_e1777, _e1787, _e1791)), 5f)));
                                            }
                                            let _e1803 = phi_8785_;
                                            let _e1804 = ((f32((_e1767 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1803);
                                            if _e1804 {
                                                let _e1817 = (((_e1146.x * _e226.x) + (_e1146.y * _e226.y)) + (_e1146.z * _e226.z));
                                                phi_13640_ = vec3<f32>(0f, 0f, 0f);
                                                phi_1563_ = (_e226 - vec3<f32>(((_e1146.x * 2f) * _e1817), ((_e1146.y * 2f) * _e1817), ((_e1146.z * 2f) * _e1817)));
                                            } else {
                                                phi_13640_ = vec3<f32>(_e1114.member_4.x, _e1114.member_4.y, _e1114.member_4.z);
                                                phi_1563_ = vec3<f32>();
                                            }
                                            let _e1824 = phi_13640_;
                                            let _e1826 = phi_1563_;
                                            phi_13989_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13784_ = 1.5f;
                                            phi_13639_ = _e1824;
                                            phi_13331_ = _e1767;
                                            phi_1565_ = _e1826;
                                            phi_1566_ = select(true, false, _e1804);
                                        }
                                        let _e1836 = phi_13989_;
                                        let _e1838 = phi_13784_;
                                        let _e1840 = phi_13639_;
                                        let _e1842 = phi_13331_;
                                        let _e1844 = phi_1565_;
                                        let _e1846 = phi_1566_;
                                        phi_13935_ = _e1836;
                                        phi_13744_ = _e1838;
                                        phi_13666_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13590_ = _e1840;
                                        phi_13512_ = select(0u, 1u, _e1754);
                                        phi_13431_ = select(1f, _e1753, _e1754);
                                        phi_13355_ = select(0u, 2u, _e1754);
                                        phi_13282_ = _e1842;
                                        phi_2051_ = _e1844;
                                        phi_2052_ = vec3<f32>(1f, 1f, 1f);
                                        phi_2053_ = _e1174;
                                        phi_2054_ = _e1846;
                                    } else {
                                        if (_e1114.member == 3u) {
                                            phi_13285_ = _e216;
                                            phi_1730_ = vec3<f32>();
                                            phi_1731_ = vec3<f32>();
                                            phi_1732_ = vec3<f32>();
                                            phi_1733_ = true;
                                            phi_1734_ = false;
                                        } else {
                                            let _e1203 = (_e1114.member == 5u);
                                            if _e1203 {
                                                phi_13286_ = _e216;
                                                phi_1726_ = vec3<f32>();
                                                phi_1727_ = vec3<f32>();
                                                phi_1728_ = vec3<f32>();
                                                phi_1729_ = false;
                                            } else {
                                                let _e1204 = (_e1114.member == 2u);
                                                if _e1204 {
                                                    let _e1209 = (((_e226.x * _e837.member_3.x) + (_e226.y * _e837.member_3.y)) + (_e226.z * _e837.member_3.z));
                                                    let _e1210 = (_e1209 > 0f);
                                                    if _e1210 {
                                                        phi_1602_ = -(_e837.member_3);
                                                    } else {
                                                        phi_1602_ = _e837.member_3;
                                                    }
                                                    let _e1213 = phi_1602_;
                                                    if _e1210 {
                                                        phi_1608_ = _e1114.member_6.x;
                                                    } else {
                                                        phi_1608_ = (1f / _e1114.member_6.x);
                                                    }
                                                    let _e1218 = phi_1608_;
                                                    let _e1219 = abs(_e1209);
                                                    let _e1220 = (_e1218 * _e1218);
                                                    let _e1223 = (_e1220 * (1f - (_e1219 * _e1219)));
                                                    if (_e1223 >= 1f) {
                                                        phi_8115_ = 1f;
                                                    } else {
                                                        let _e1226 = sqrt((1f - _e1223));
                                                        if (_e1219 != _e1219) {
                                                            phi_8126_ = true;
                                                        } else {
                                                            phi_8126_ = (_e1226 <= _e1219);
                                                        }
                                                        let _e1230 = phi_8126_;
                                                        let _e1235 = pow(((1f - _e1114.member_6.x) / (1f + _e1114.member_6.x)), 2f);
                                                        phi_8115_ = (_e1235 + ((1f - _e1235) * pow((1f - select(_e1219, _e1226, _e1230)), 5f)));
                                                    }
                                                    let _e1242 = phi_8115_;
                                                    let _e1250 = (((_e226.x * _e1213.x) + (_e226.y * _e1213.y)) + (_e226.z * _e1213.z));
                                                    let _e1254 = (1f - (_e1220 * (1f - (_e1250 * _e1250))));
                                                    if (_e1254 > 0f) {
                                                        let _e1257 = ((_e216 * 747796405u) + 2891336453u);
                                                        let _e1265 = (((_e1257 >> bitcast<u32>((((_e1257 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1257) * 277803737u);
                                                        let _e1268 = ((_e1265 >> bitcast<u32>(22i)) ^ _e1265);
                                                        let _e1273 = ((f32((_e1268 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1242);
                                                        if _e1273 {
                                                            let _e1283 = sqrt(_e1254);
                                                            let _e1287 = (((_e226.x - (_e1213.x * _e1250)) * _e1218) - (_e1213.x * _e1283));
                                                            let _e1288 = (((_e226.y - (_e1213.y * _e1250)) * _e1218) - (_e1213.y * _e1283));
                                                            let _e1289 = (((_e226.z - (_e1213.z * _e1250)) * _e1218) - (_e1213.z * _e1283));
                                                            let _e1296 = (1f / sqrt((((_e1287 * _e1287) + (_e1288 * _e1288)) + (_e1289 * _e1289))));
                                                            phi_1698_ = vec3<f32>((_e1287 * _e1296), (_e1288 * _e1296), (_e1289 * _e1296));
                                                            phi_1699_ = vec3<f32>(_e1114.member_4.x, _e1114.member_4.y, _e1114.member_4.z);
                                                            phi_1700_ = vec3<f32>((_e1043 - (_e1213.x * _e1164)), (_e1045 - (_e1213.y * _e1164)), (_e1047 - (_e1213.z * _e1164)));
                                                        } else {
                                                            phi_1698_ = vec3<f32>();
                                                            phi_1699_ = vec3<f32>();
                                                            phi_1700_ = vec3<f32>();
                                                        }
                                                        let _e1314 = phi_1698_;
                                                        let _e1316 = phi_1699_;
                                                        let _e1318 = phi_1700_;
                                                        phi_13289_ = _e1268;
                                                        phi_1702_ = _e1314;
                                                        phi_1703_ = _e1316;
                                                        phi_1704_ = _e1318;
                                                        phi_1705_ = select(true, false, _e1273);
                                                    } else {
                                                        phi_13289_ = _e216;
                                                        phi_1702_ = vec3<f32>();
                                                        phi_1703_ = vec3<f32>();
                                                        phi_1704_ = vec3<f32>();
                                                        phi_1705_ = true;
                                                    }
                                                    let _e1321 = phi_13289_;
                                                    let _e1323 = phi_1702_;
                                                    let _e1325 = phi_1703_;
                                                    let _e1327 = phi_1704_;
                                                    let _e1329 = phi_1705_;
                                                    if _e1329 {
                                                        let _e1337 = (((_e837.member_3.x * _e226.x) + (_e837.member_3.y * _e226.y)) + (_e837.member_3.z * _e226.z));
                                                        phi_1720_ = (_e226 - vec3<f32>(((_e837.member_3.x * 2f) * _e1337), ((_e837.member_3.y * 2f) * _e1337), ((_e837.member_3.z * 2f) * _e1337)));
                                                        phi_1722_ = vec3<f32>((_e1043 + (_e1213.x * _e1164)), (_e1045 + (_e1213.y * _e1164)), (_e1047 + (_e1213.z * _e1164)));
                                                    } else {
                                                        phi_1720_ = _e1323;
                                                        phi_1722_ = _e1327;
                                                    }
                                                    let _e1351 = phi_1720_;
                                                    let _e1353 = phi_1722_;
                                                    phi_13287_ = _e1321;
                                                    phi_1723_ = _e1351;
                                                    phi_1724_ = select(_e1325, vec3<f32>(1f, 1f, 1f), vec3(_e1329));
                                                    phi_1725_ = _e1353;
                                                } else {
                                                    phi_13287_ = _e216;
                                                    phi_1723_ = vec3<f32>();
                                                    phi_1724_ = vec3<f32>();
                                                    phi_1725_ = vec3<f32>();
                                                }
                                                let _e1357 = phi_13287_;
                                                let _e1359 = phi_1723_;
                                                let _e1361 = phi_1724_;
                                                let _e1363 = phi_1725_;
                                                phi_13286_ = _e1357;
                                                phi_1726_ = _e1359;
                                                phi_1727_ = _e1361;
                                                phi_1728_ = _e1363;
                                                phi_1729_ = _e1204;
                                            }
                                            let _e1365 = phi_13286_;
                                            let _e1367 = phi_1726_;
                                            let _e1369 = phi_1727_;
                                            let _e1371 = phi_1728_;
                                            let _e1373 = phi_1729_;
                                            phi_13285_ = _e1365;
                                            phi_1730_ = _e1367;
                                            phi_1731_ = _e1369;
                                            phi_1732_ = _e1371;
                                            phi_1733_ = _e1203;
                                            phi_1734_ = _e1373;
                                        }
                                        let _e1375 = phi_13285_;
                                        let _e1377 = phi_1730_;
                                        let _e1379 = phi_1731_;
                                        let _e1381 = phi_1732_;
                                        let _e1383 = phi_1733_;
                                        let _e1385 = phi_1734_;
                                        if _e1383 {
                                            let _e1388 = (_e1114.member_6.y * _e1114.member_6.y);
                                            if (_e1388 != _e1388) {
                                                phi_8199_ = true;
                                            } else {
                                                phi_8199_ = (_e1179 >= _e1388);
                                            }
                                            let _e1392 = phi_8199_;
                                            let _e1393 = select(_e1388, _e1179, _e1392);
                                            let _e1405 = select(2u, 0u, (_e1114.member == 5u));
                                            let _e1406 = (_e1393 < 0.001f);
                                            if _e1406 {
                                                let _e1414 = (((_e1146.x * _e226.x) + (_e1146.y * _e226.y)) + (_e1146.z * _e226.z));
                                                let _e1425 = abs((((_e1117 * _e1146.x) + (_e1118 * _e1146.y)) + (_e1119 * _e1146.z)));
                                                if (_e1405 == 0u) {
                                                    let _e1707 = (1f - _e1425);
                                                    if (_e1707 != _e1707) {
                                                        phi_8730_ = true;
                                                    } else {
                                                        phi_8730_ = (0f >= _e1707);
                                                    }
                                                    let _e1711 = phi_8730_;
                                                    let _e1713 = pow(select(_e1707, 0f, _e1711), 5f);
                                                    let _e1714 = (1f - _e1713);
                                                    phi_2037_ = vec3<f32>(((_e1114.member_4.x * _e1714) + _e1713), ((_e1114.member_4.y * _e1714) + _e1713), ((_e1114.member_4.z * _e1714) + _e1713));
                                                } else {
                                                    if (_e1405 == 1u) {
                                                        let _e1689 = (0.44444448f * (1f - (_e1425 * _e1425)));
                                                        if (_e1689 >= 1f) {
                                                            phi_8691_ = 1f;
                                                        } else {
                                                            let _e1692 = sqrt((1f - _e1689));
                                                            if (_e1425 != _e1425) {
                                                                phi_8702_ = true;
                                                            } else {
                                                                phi_8702_ = (_e1692 <= _e1425);
                                                            }
                                                            let _e1696 = phi_8702_;
                                                            phi_8691_ = (0.040000003f + (0.96f * pow((1f - select(_e1425, _e1692, _e1696)), 5f)));
                                                        }
                                                        let _e1703 = phi_8691_;
                                                        phi_2036_ = vec3(_e1703);
                                                    } else {
                                                        let _e1429 = select(_e1425, 0f, (_e1425 < 0f));
                                                        let _e1431 = select(_e1429, 1f, (_e1429 > 1f));
                                                        let _e1433 = (1f - (_e1431 * _e1431));
                                                        let _e1440 = ((_e1114.member_4.x * _e1114.member_4.x) - (_e1114.member_5.x * _e1114.member_5.x));
                                                        let _e1441 = ((_e1114.member_4.y * _e1114.member_4.y) - (_e1114.member_5.y * _e1114.member_5.y));
                                                        let _e1442 = ((_e1114.member_4.z * _e1114.member_4.z) - (_e1114.member_5.z * _e1114.member_5.z));
                                                        let _e1444 = (_e1114.member_4.x * (_e1114.member_5.x + _e1114.member_5.x));
                                                        let _e1446 = (_e1114.member_4.y * (_e1114.member_5.y + _e1114.member_5.y));
                                                        let _e1448 = (_e1114.member_4.z * (_e1114.member_5.z + _e1114.member_5.z));
                                                        let _e1455 = ((_e1440 * _e1440) + (_e1444 * _e1444));
                                                        let _e1456 = ((_e1441 * _e1441) + (_e1446 * _e1446));
                                                        let _e1457 = ((_e1442 * _e1442) + (_e1448 * _e1448));
                                                        let _e1462 = ((_e1433 * _e1440) / _e1455);
                                                        let _e1463 = ((_e1433 * _e1441) / _e1456);
                                                        let _e1464 = ((_e1433 * _e1442) / _e1457);
                                                        let _e1465 = (1f - _e1462);
                                                        let _e1466 = (1f - _e1463);
                                                        let _e1467 = (1f - _e1464);
                                                        let _e1469 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1462, _e1463, _e1464));
                                                        let _e1470 = (_e1433 * _e1444);
                                                        let _e1471 = (_e1433 * _e1446);
                                                        let _e1472 = (_e1433 * _e1448);
                                                        let _e1477 = (vec3<f32>(_e1470, _e1471, _e1472) / vec3<f32>(_e1455, _e1456, _e1457));
                                                        let _e1490 = ((_e1469.x * _e1469.x) + (_e1477.x * _e1477.x));
                                                        let _e1491 = ((_e1469.y * _e1469.y) + (_e1477.y * _e1477.y));
                                                        let _e1492 = ((_e1469.z * _e1469.z) + (_e1477.z * _e1477.z));
                                                        if (_e1490 != _e1490) {
                                                            phi_8361_ = true;
                                                        } else {
                                                            phi_8361_ = (0f >= _e1490);
                                                        }
                                                        let _e1496 = phi_8361_;
                                                        let _e1498 = sqrt(select(_e1490, 0f, _e1496));
                                                        if (_e1491 != _e1491) {
                                                            phi_8376_ = true;
                                                        } else {
                                                            phi_8376_ = (0f >= _e1491);
                                                        }
                                                        let _e1502 = phi_8376_;
                                                        let _e1504 = sqrt(select(_e1491, 0f, _e1502));
                                                        if (_e1492 != _e1492) {
                                                            phi_8391_ = true;
                                                        } else {
                                                            phi_8391_ = (0f >= _e1492);
                                                        }
                                                        let _e1508 = phi_8391_;
                                                        let _e1510 = sqrt(select(_e1492, 0f, _e1508));
                                                        let _e1514 = ((_e1498 + _e1465) * 0.5f);
                                                        let _e1515 = ((_e1504 + _e1466) * 0.5f);
                                                        let _e1516 = ((_e1510 + _e1467) * 0.5f);
                                                        if (_e1514 != _e1514) {
                                                            phi_8418_ = true;
                                                        } else {
                                                            phi_8418_ = (0f >= _e1514);
                                                        }
                                                        let _e1520 = phi_8418_;
                                                        let _e1522 = sqrt(select(_e1514, 0f, _e1520));
                                                        if (_e1515 != _e1515) {
                                                            phi_8433_ = true;
                                                        } else {
                                                            phi_8433_ = (0f >= _e1515);
                                                        }
                                                        let _e1526 = phi_8433_;
                                                        let _e1528 = sqrt(select(_e1515, 0f, _e1526));
                                                        if (_e1516 != _e1516) {
                                                            phi_8448_ = true;
                                                        } else {
                                                            phi_8448_ = (0f >= _e1516);
                                                        }
                                                        let _e1532 = phi_8448_;
                                                        let _e1534 = sqrt(select(_e1516, 0f, _e1532));
                                                        let _e1535 = vec3<f32>(_e1522, _e1528, _e1534);
                                                        let _e1539 = ((_e1498 - _e1465) * 0.5f);
                                                        let _e1540 = ((_e1504 - _e1466) * 0.5f);
                                                        let _e1541 = ((_e1510 - _e1467) * 0.5f);
                                                        if (_e1539 != _e1539) {
                                                            phi_8475_ = true;
                                                        } else {
                                                            phi_8475_ = (0f >= _e1539);
                                                        }
                                                        let _e1545 = phi_8475_;
                                                        let _e1547 = sqrt(select(_e1539, 0f, _e1545));
                                                        if (_e1540 != _e1540) {
                                                            phi_8490_ = true;
                                                        } else {
                                                            phi_8490_ = (0f >= _e1540);
                                                        }
                                                        let _e1551 = phi_8490_;
                                                        let _e1553 = sqrt(select(_e1540, 0f, _e1551));
                                                        if (_e1541 != _e1541) {
                                                            phi_8505_ = true;
                                                        } else {
                                                            phi_8505_ = (0f >= _e1541);
                                                        }
                                                        let _e1557 = phi_8505_;
                                                        let _e1559 = sqrt(select(_e1541, 0f, _e1557));
                                                        if ((_e1470 / _e1455) < 0f) {
                                                            phi_1913_ = -(_e1547);
                                                        } else {
                                                            phi_1913_ = _e1547;
                                                        }
                                                        let _e1563 = phi_1913_;
                                                        if ((_e1471 / _e1456) < 0f) {
                                                            phi_1921_ = -(_e1553);
                                                        } else {
                                                            phi_1921_ = _e1553;
                                                        }
                                                        let _e1567 = phi_1921_;
                                                        if ((_e1472 / _e1457) < 0f) {
                                                            phi_1929_ = -(_e1559);
                                                        } else {
                                                            phi_1929_ = _e1559;
                                                        }
                                                        let _e1571 = phi_1929_;
                                                        let _e1575 = (_e1114.member_5.x * _e1431);
                                                        let _e1576 = (_e1114.member_5.y * _e1431);
                                                        let _e1577 = (_e1114.member_5.z * _e1431);
                                                        let _e1587 = vec3<f32>(((_e1114.member_4.x * _e1522) - (_e1114.member_5.x * _e1563)), ((_e1114.member_4.y * _e1528) - (_e1114.member_5.y * _e1567)), ((_e1114.member_4.z * _e1534) - (_e1114.member_5.z * _e1571)));
                                                        let _e1594 = ((_e1114.member_4.x * _e1563) + (_e1114.member_5.x * _e1522));
                                                        let _e1595 = ((_e1114.member_4.y * _e1567) + (_e1114.member_5.y * _e1528));
                                                        let _e1596 = ((_e1114.member_4.z * _e1571) + (_e1114.member_5.z * _e1534));
                                                        let _e1598 = vec3<f32>((_e1114.member_4.x * _e1431), (_e1114.member_4.y * _e1431), (_e1114.member_4.z * _e1431));
                                                        let _e1599 = (_e1598 - _e1535);
                                                        let _e1600 = (_e1575 - _e1563);
                                                        let _e1601 = (_e1576 - _e1567);
                                                        let _e1602 = (_e1577 - _e1571);
                                                        let _e1615 = (_e1598 + _e1535);
                                                        let _e1616 = (_e1575 + _e1563);
                                                        let _e1617 = (_e1576 + _e1567);
                                                        let _e1618 = (_e1577 + _e1571);
                                                        let _e1628 = ((_e1615.x * _e1615.x) + (_e1616 * _e1616));
                                                        let _e1629 = ((_e1615.y * _e1615.y) + (_e1617 * _e1617));
                                                        let _e1630 = ((_e1615.z * _e1615.z) + (_e1618 * _e1618));
                                                        let _e1640 = vec3(_e1431);
                                                        let _e1641 = (_e1640 - _e1587);
                                                        let _e1642 = -(vec3<f32>(_e1594, _e1595, _e1596));
                                                        let _e1658 = (_e1640 + _e1587);
                                                        let _e1668 = ((_e1658.x * _e1658.x) + (_e1594 * _e1594));
                                                        let _e1669 = ((_e1658.y * _e1658.y) + (_e1595 * _e1595));
                                                        let _e1670 = ((_e1658.z * _e1658.z) + (_e1596 * _e1596));
                                                        phi_2036_ = vec3<f32>((((((_e1599.x * _e1599.x) + (_e1600 * _e1600)) / select(0.000000000000000000000000000001f, _e1628, (_e1628 > 0.000000000000000000000000000001f))) + (((_e1641.x * _e1641.x) + (_e1642.x * _e1642.x)) / select(0.000000000000000000000000000001f, _e1668, (_e1668 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1599.y * _e1599.y) + (_e1601 * _e1601)) / select(0.000000000000000000000000000001f, _e1629, (_e1629 > 0.000000000000000000000000000001f))) + (((_e1641.y * _e1641.y) + (_e1642.y * _e1642.y)) / select(0.000000000000000000000000000001f, _e1669, (_e1669 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1599.z * _e1599.z) + (_e1602 * _e1602)) / select(0.000000000000000000000000000001f, _e1630, (_e1630 > 0.000000000000000000000000000001f))) + (((_e1641.z * _e1641.z) + (_e1642.z * _e1642.z)) / select(0.000000000000000000000000000001f, _e1670, (_e1670 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e1706 = phi_2036_;
                                                    phi_2037_ = _e1706;
                                                }
                                                let _e1723 = phi_2037_;
                                                phi_2038_ = (_e226 - vec3<f32>(((_e1146.x * 2f) * _e1414), ((_e1146.y * 2f) * _e1414), ((_e1146.z * 2f) * _e1414)));
                                                phi_2039_ = _e1723;
                                            } else {
                                                phi_2038_ = vec3<f32>();
                                                phi_2039_ = vec3<f32>();
                                            }
                                            let _e1725 = phi_2038_;
                                            let _e1727 = phi_2039_;
                                            phi_13668_ = vec3<f32>(_e1114.member_5.x, _e1114.member_5.y, _e1114.member_5.z);
                                            phi_13592_ = vec3<f32>(_e1114.member_4.x, _e1114.member_4.y, _e1114.member_4.z);
                                            phi_13514_ = _e1405;
                                            phi_13433_ = _e1393;
                                            phi_2040_ = _e1725;
                                            phi_2041_ = _e1727;
                                            phi_2043_ = _e1406;
                                        } else {
                                            phi_13668_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13592_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13514_ = 0u;
                                            phi_13433_ = 1f;
                                            phi_2040_ = _e1377;
                                            phi_2041_ = _e1379;
                                            phi_2043_ = _e1385;
                                        }
                                        let _e1729 = phi_13668_;
                                        let _e1731 = phi_13592_;
                                        let _e1733 = phi_13514_;
                                        let _e1735 = phi_13433_;
                                        let _e1737 = phi_2040_;
                                        let _e1739 = phi_2041_;
                                        let _e1741 = phi_2043_;
                                        phi_13935_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13744_ = 1.5f;
                                        phi_13666_ = _e1729;
                                        phi_13590_ = _e1731;
                                        phi_13512_ = _e1733;
                                        phi_13431_ = _e1735;
                                        phi_13355_ = select(0u, 1u, _e1383);
                                        phi_13282_ = _e1375;
                                        phi_2051_ = _e1737;
                                        phi_2052_ = _e1739;
                                        phi_2053_ = select(_e1381, _e1174, vec3(_e1383));
                                        phi_2054_ = select(true, false, _e1741);
                                    }
                                    let _e1851 = phi_13935_;
                                    let _e1853 = phi_13744_;
                                    let _e1855 = phi_13666_;
                                    let _e1857 = phi_13590_;
                                    let _e1859 = phi_13512_;
                                    let _e1861 = phi_13431_;
                                    let _e1863 = phi_13355_;
                                    let _e1865 = phi_13282_;
                                    let _e1867 = phi_2051_;
                                    let _e1869 = phi_2052_;
                                    let _e1871 = phi_2053_;
                                    let _e1873 = phi_2054_;
                                    let _e1874 = select(true, false, _e1873);
                                    if _e1874 {
                                        let _e1875 = (_e228 >= _e208);
                                        if _e1875 {
                                            phi_13339_ = _e1865;
                                            phi_2169_ = vec3<f32>();
                                            phi_2170_ = vec3<f32>();
                                            phi_2171_ = u32();
                                            phi_2172_ = false;
                                            phi_2173_ = false;
                                        } else {
                                            let _e1887 = global_1.member.member_4[1u];
                                            if (sqrt((((_e1869.x * _e1869.x) + (_e1869.y * _e1869.y)) + (_e1869.z * _e1869.z))) <= _e1887) {
                                                phi_13341_ = _e1865;
                                                phi_2134_ = _e1034;
                                                phi_2135_ = false;
                                            } else {
                                                let _e1891 = global_1.member.member_7[1u];
                                                if (_e1891 == 1u) {
                                                    let _e1896 = global_1.member.member_4[3u];
                                                    if (f32(_e228) >= _e1896) {
                                                        let _e1899 = select(_e1869.y, _e1869.x, (_e1869.x > _e1869.y));
                                                        let _e1901 = select(_e1869.z, _e1899, (_e1899 > _e1869.z));
                                                        if (_e1901 != _e1901) {
                                                            phi_8862_ = true;
                                                        } else {
                                                            phi_8862_ = (1f <= _e1901);
                                                        }
                                                        let _e1905 = phi_8862_;
                                                        let _e1906 = select(_e1901, 1f, _e1905);
                                                        let _e1908 = ((_e1865 * 747796405u) + 2891336453u);
                                                        let _e1916 = (((_e1908 >> bitcast<u32>((((_e1908 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1908) * 277803737u);
                                                        let _e1919 = ((_e1916 >> bitcast<u32>(22i)) ^ _e1916);
                                                        let _e1924 = ((f32((_e1919 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1906);
                                                        if _e1924 {
                                                            phi_2123_ = vec3<f32>();
                                                        } else {
                                                            phi_2123_ = vec3<f32>((_e1034.x / _e1906), (_e1034.y / _e1906), (_e1034.z / _e1906));
                                                        }
                                                        let _e1933 = phi_2123_;
                                                        phi_13344_ = _e1919;
                                                        phi_2125_ = _e1933;
                                                        phi_2126_ = select(true, false, _e1924);
                                                    } else {
                                                        phi_13344_ = _e1865;
                                                        phi_2125_ = _e1034;
                                                        phi_2126_ = true;
                                                    }
                                                    let _e1936 = phi_13344_;
                                                    let _e1938 = phi_2125_;
                                                    let _e1940 = phi_2126_;
                                                    phi_13343_ = _e1936;
                                                    phi_2127_ = _e1938;
                                                    phi_2128_ = _e1940;
                                                } else {
                                                    phi_13343_ = _e1865;
                                                    phi_2127_ = _e1034;
                                                    phi_2128_ = true;
                                                }
                                                let _e1942 = phi_13343_;
                                                let _e1944 = phi_2127_;
                                                let _e1946 = phi_2128_;
                                                phi_13341_ = _e1942;
                                                phi_2134_ = select(_e1034, _e1944, vec3(_e1946));
                                                phi_2135_ = select(false, true, _e1946);
                                            }
                                            let _e1951 = phi_13341_;
                                            let _e1953 = phi_2134_;
                                            let _e1955 = phi_2135_;
                                            if _e1955 {
                                                phi_2165_ = (_e1953 * _e1869);
                                                phi_2166_ = (_e1867 * (1f / sqrt((((_e1867.x * _e1867.x) + (_e1867.y * _e1867.y)) + (_e1867.z * _e1867.z)))));
                                                phi_2167_ = (_e228 + 1u);
                                            } else {
                                                phi_2165_ = vec3<f32>();
                                                phi_2166_ = vec3<f32>();
                                                phi_2167_ = u32();
                                            }
                                            let _e1970 = phi_2165_;
                                            let _e1972 = phi_2166_;
                                            let _e1974 = phi_2167_;
                                            phi_13339_ = _e1951;
                                            phi_2169_ = _e1970;
                                            phi_2170_ = _e1972;
                                            phi_2171_ = _e1974;
                                            phi_2172_ = _e1955;
                                            phi_2173_ = select(true, false, _e1955);
                                        }
                                        let _e1977 = phi_13339_;
                                        let _e1979 = phi_2169_;
                                        let _e1981 = phi_2170_;
                                        let _e1983 = phi_2171_;
                                        let _e1985 = phi_2172_;
                                        let _e1987 = phi_2173_;
                                        phi_13338_ = _e1977;
                                        phi_2174_ = _e1979;
                                        phi_2175_ = _e1981;
                                        phi_2176_ = _e1983;
                                        phi_2178_ = _e1985;
                                        phi_2179_ = _e1987;
                                        phi_2180_ = _e1875;
                                    } else {
                                        phi_13338_ = _e1865;
                                        phi_2174_ = vec3<f32>();
                                        phi_2175_ = vec3<f32>();
                                        phi_2176_ = u32();
                                        phi_2178_ = false;
                                        phi_2179_ = false;
                                        phi_2180_ = false;
                                    }
                                    let _e1989 = phi_13338_;
                                    let _e1991 = phi_2174_;
                                    let _e1993 = phi_2175_;
                                    let _e1995 = phi_2176_;
                                    let _e1997 = phi_2178_;
                                    let _e1999 = phi_2179_;
                                    let _e2001 = phi_2180_;
                                    phi_13932_ = _e1851;
                                    phi_13741_ = _e1853;
                                    phi_13663_ = _e1855;
                                    phi_13587_ = _e1857;
                                    phi_13509_ = _e1859;
                                    phi_13428_ = _e1861;
                                    phi_13352_ = _e1863;
                                    phi_13337_ = _e1989;
                                    phi_2181_ = _e1991;
                                    phi_2182_ = _e1993;
                                    phi_2183_ = _e1995;
                                    phi_2184_ = _e1871;
                                    phi_2185_ = select(true, false, _e1874);
                                    phi_2186_ = _e1997;
                                    phi_2187_ = _e1999;
                                    phi_2188_ = _e2001;
                                }
                                let _e2009 = phi_13932_;
                                let _e2011 = phi_13741_;
                                let _e2013 = phi_13663_;
                                let _e2015 = phi_13587_;
                                let _e2017 = phi_13509_;
                                let _e2019 = phi_13428_;
                                let _e2021 = phi_13352_;
                                let _e2023 = phi_13337_;
                                let _e2025 = phi_2181_;
                                let _e2027 = phi_2182_;
                                let _e2029 = phi_2183_;
                                let _e2031 = phi_2184_;
                                let _e2033 = phi_2185_;
                                let _e2035 = phi_2186_;
                                let _e2037 = phi_2187_;
                                let _e2039 = phi_2188_;
                                if _e2033 {
                                    let _e2042 = global_1.member.member_1[3u];
                                    phi_15093_ = _e671;
                                    phi_13336_ = _e2023;
                                    phi_2197_ = _e1032;
                                    phi_2200_ = 0u;
                                    loop {
                                        let _e2044 = phi_15093_;
                                        let _e2046 = phi_13336_;
                                        let _e2048 = phi_2197_;
                                        let _e2050 = phi_2200_;
                                        local_54 = _e2048;
                                        local_55 = _e2048;
                                        let _e2052 = (_e2050 < (_e2042 + 3u));
                                        if _e2052 {
                                            if (_e2050 < _e2042) {
                                                let _e3577 = global_5.member[_e2050];
                                                let _e3582 = (_e3577.member.x - _e1171);
                                                let _e3583 = (_e3577.member.y - _e1172);
                                                let _e3584 = (_e3577.member.z - _e1173);
                                                let _e3589 = (((_e3582 * _e3582) + (_e3583 * _e3583)) + (_e3584 * _e3584));
                                                let _e3590 = sqrt(_e3589);
                                                let _e3591 = (_e3582 / _e3590);
                                                let _e3592 = (_e3583 / _e3590);
                                                let _e3593 = (_e3584 / _e3590);
                                                if (_e3577.member.w < 0.5f) {
                                                    phi_2297_ = 1f;
                                                } else {
                                                    let _e3617 = (((_e3577.member_2.x * -(_e3591)) + (_e3577.member_2.y * -(_e3592))) + (_e3577.member_2.z * -(_e3593)));
                                                    if (_e3577.member_3.x == _e3577.member_2.w) {
                                                        phi_2296_ = select(1f, 0f, (_e3617 < _e3577.member_3.x));
                                                    } else {
                                                        let _e3621 = ((_e3617 - _e3577.member_3.x) / (_e3577.member_2.w - _e3577.member_3.x));
                                                        let _e3623 = select(_e3621, 0f, (_e3621 < 0f));
                                                        let _e3625 = select(_e3623, 1f, (_e3623 > 1f));
                                                        phi_2296_ = ((_e3625 * _e3625) * (3f - (2f * _e3625)));
                                                    }
                                                    let _e3633 = phi_2296_;
                                                    phi_2297_ = _e3633;
                                                }
                                                let _e3635 = phi_2297_;
                                                phi_15429_ = _e2046;
                                                phi_4068_ = vec3<f32>(((_e3577.member_1.x * _e3635) / _e3589), ((_e3577.member_1.y * _e3635) / _e3589), ((_e3577.member_1.z * _e3635) / _e3589));
                                                phi_4069_ = 0f;
                                                phi_4070_ = vec3<f32>(_e3591, _e3592, _e3593);
                                                phi_4071_ = _e3590;
                                            } else {
                                                if (_e2050 == _e2042) {
                                                    if ((_e211 & 4u) == 0u) {
                                                        phi_15468_ = _e2046;
                                                        phi_2857_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_2858_ = 0f;
                                                        phi_2859_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2905 = global_1.member.member_5;
                                                        let _e2912 = global_1.member.member_5[3u];
                                                        let _e2913 = (1f - _e2912);
                                                        let _e2914 = (6.2831855f * _e2913);
                                                        let _e2919 = (((_e2905.x * _e1146.x) + (_e2905.y * _e1146.y)) + (_e2905.z * _e1146.z));
                                                        if (_e2919 != _e2919) {
                                                            phi_9398_ = true;
                                                        } else {
                                                            phi_9398_ = (0f >= _e2919);
                                                        }
                                                        let _e2923 = phi_9398_;
                                                        let _e2925 = (select(_e2919, 0f, _e2923) * 0.31830987f);
                                                        if (_e2021 == 0u) {
                                                            if (_e2919 <= 0f) {
                                                                phi_10195_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_10195_ = (_e2015 * (_e2919 * 0.31830987f));
                                                            }
                                                            let _e3404 = phi_10195_;
                                                            phi_2705_ = type_59(_e3404, _e2925);
                                                            phi_2706_ = type_59();
                                                            phi_2707_ = true;
                                                        } else {
                                                            let _e2934 = (((_e1120.x * _e1192) + (_e1120.y * _e1193)) + (_e1120.z * _e1195));
                                                            let _e2939 = (((_e1120.x * _e1188) + (_e1120.y * _e1198)) + (_e1120.z * _e1199));
                                                            let _e2944 = (((_e1120.x * _e1146.x) + (_e1120.y * _e1146.y)) + (_e1120.z * _e1146.z));
                                                            let _e2949 = (((_e2905.x * _e1192) + (_e2905.y * _e1193)) + (_e2905.z * _e1195));
                                                            let _e2954 = (((_e2905.x * _e1188) + (_e2905.y * _e1198)) + (_e2905.z * _e1199));
                                                            if (_e2944 <= 0f) {
                                                                phi_2669_ = type_59();
                                                                phi_2670_ = true;
                                                            } else {
                                                                let _e2956 = (_e2919 <= 0f);
                                                                if _e2956 {
                                                                    phi_2668_ = type_59();
                                                                } else {
                                                                    let _e2957 = (_e2934 + _e2949);
                                                                    let _e2958 = (_e2939 + _e2954);
                                                                    let _e2959 = (_e2944 + _e2919);
                                                                    let _e2966 = (1f / sqrt((((_e2957 * _e2957) + (_e2958 * _e2958)) + (_e2959 * _e2959))));
                                                                    let _e2969 = (_e2959 * _e2966);
                                                                    if (_e2969 <= 0f) {
                                                                        phi_9508_ = 0f;
                                                                    } else {
                                                                        let _e2971 = (_e2019 * _e2019);
                                                                        let _e2975 = ((((_e2971 - 1f) * _e2969) * _e2969) + 1f);
                                                                        phi_9508_ = (_e2971 / ((3.1415927f * _e2975) * _e2975));
                                                                    }
                                                                    let _e2980 = phi_9508_;
                                                                    let _e2981 = (_e2944 * _e2944);
                                                                    if (_e2981 >= 1f) {
                                                                        phi_9536_ = 0f;
                                                                    } else {
                                                                        phi_9536_ = ((sqrt((1f + (((_e2019 * _e2019) * ((_e2934 * _e2934) + (_e2939 * _e2939))) / _e2981))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e2994 = phi_9536_;
                                                                    let _e2995 = (1f + _e2994);
                                                                    let _e2996 = (_e2919 * _e2919);
                                                                    if (_e2996 >= 1f) {
                                                                        phi_9564_ = 0f;
                                                                    } else {
                                                                        phi_9564_ = ((sqrt((1f + (((_e2019 * _e2019) * ((_e2949 * _e2949) + (_e2954 * _e2954))) / _e2996))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3009 = phi_9564_;
                                                                    let _e3016 = (((_e2934 * (_e2957 * _e2966)) + (_e2939 * (_e2958 * _e2966))) + (_e2944 * _e2969));
                                                                    if (_e2017 == 0u) {
                                                                        let _e3311 = (1f - _e3016);
                                                                        if (_e3311 != _e3311) {
                                                                            phi_10067_ = true;
                                                                        } else {
                                                                            phi_10067_ = (0f >= _e3311);
                                                                        }
                                                                        let _e3315 = phi_10067_;
                                                                        let _e3317 = pow(select(_e3311, 0f, _e3315), 5f);
                                                                        let _e3318 = (1f - _e3317);
                                                                        phi_2656_ = vec3<f32>(((_e2015.x * _e3318) + _e3317), ((_e2015.y * _e3318) + _e3317), ((_e2015.z * _e3318) + _e3317));
                                                                    } else {
                                                                        if (_e2017 == 1u) {
                                                                            let _e3284 = (1f / _e2011);
                                                                            let _e3288 = ((_e3284 * _e3284) * (1f - (_e3016 * _e3016)));
                                                                            if (_e3288 >= 1f) {
                                                                                phi_10028_ = 1f;
                                                                            } else {
                                                                                let _e3291 = sqrt((1f - _e3288));
                                                                                if (_e3016 != _e3016) {
                                                                                    phi_10039_ = true;
                                                                                } else {
                                                                                    phi_10039_ = (_e3291 <= _e3016);
                                                                                }
                                                                                let _e3295 = phi_10039_;
                                                                                let _e3300 = pow(((1f - _e2011) / (1f + _e2011)), 2f);
                                                                                phi_10028_ = (_e3300 + ((1f - _e3300) * pow((1f - select(_e3016, _e3291, _e3295)), 5f)));
                                                                            }
                                                                            let _e3307 = phi_10028_;
                                                                            phi_2655_ = vec3(_e3307);
                                                                        } else {
                                                                            let _e3020 = select(_e3016, 0f, (_e3016 < 0f));
                                                                            let _e3022 = select(_e3020, 1f, (_e3020 > 1f));
                                                                            let _e3024 = (1f - (_e3022 * _e3022));
                                                                            let _e3037 = ((_e2015.x * _e2015.x) - (_e2013.x * _e2013.x));
                                                                            let _e3038 = ((_e2015.y * _e2015.y) - (_e2013.y * _e2013.y));
                                                                            let _e3039 = ((_e2015.z * _e2015.z) - (_e2013.z * _e2013.z));
                                                                            let _e3041 = (_e2015.x * (_e2013.x + _e2013.x));
                                                                            let _e3043 = (_e2015.y * (_e2013.y + _e2013.y));
                                                                            let _e3045 = (_e2015.z * (_e2013.z + _e2013.z));
                                                                            let _e3052 = ((_e3037 * _e3037) + (_e3041 * _e3041));
                                                                            let _e3053 = ((_e3038 * _e3038) + (_e3043 * _e3043));
                                                                            let _e3054 = ((_e3039 * _e3039) + (_e3045 * _e3045));
                                                                            let _e3059 = ((_e3024 * _e3037) / _e3052);
                                                                            let _e3060 = ((_e3024 * _e3038) / _e3053);
                                                                            let _e3061 = ((_e3024 * _e3039) / _e3054);
                                                                            let _e3062 = (1f - _e3059);
                                                                            let _e3063 = (1f - _e3060);
                                                                            let _e3064 = (1f - _e3061);
                                                                            let _e3066 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3059, _e3060, _e3061));
                                                                            let _e3067 = (_e3024 * _e3041);
                                                                            let _e3068 = (_e3024 * _e3043);
                                                                            let _e3069 = (_e3024 * _e3045);
                                                                            let _e3074 = (vec3<f32>(_e3067, _e3068, _e3069) / vec3<f32>(_e3052, _e3053, _e3054));
                                                                            let _e3087 = ((_e3066.x * _e3066.x) + (_e3074.x * _e3074.x));
                                                                            let _e3088 = ((_e3066.y * _e3066.y) + (_e3074.y * _e3074.y));
                                                                            let _e3089 = ((_e3066.z * _e3066.z) + (_e3074.z * _e3074.z));
                                                                            if (_e3087 != _e3087) {
                                                                                phi_9698_ = true;
                                                                            } else {
                                                                                phi_9698_ = (0f >= _e3087);
                                                                            }
                                                                            let _e3093 = phi_9698_;
                                                                            let _e3095 = sqrt(select(_e3087, 0f, _e3093));
                                                                            if (_e3088 != _e3088) {
                                                                                phi_9713_ = true;
                                                                            } else {
                                                                                phi_9713_ = (0f >= _e3088);
                                                                            }
                                                                            let _e3099 = phi_9713_;
                                                                            let _e3101 = sqrt(select(_e3088, 0f, _e3099));
                                                                            if (_e3089 != _e3089) {
                                                                                phi_9728_ = true;
                                                                            } else {
                                                                                phi_9728_ = (0f >= _e3089);
                                                                            }
                                                                            let _e3105 = phi_9728_;
                                                                            let _e3107 = sqrt(select(_e3089, 0f, _e3105));
                                                                            let _e3111 = ((_e3095 + _e3062) * 0.5f);
                                                                            let _e3112 = ((_e3101 + _e3063) * 0.5f);
                                                                            let _e3113 = ((_e3107 + _e3064) * 0.5f);
                                                                            if (_e3111 != _e3111) {
                                                                                phi_9755_ = true;
                                                                            } else {
                                                                                phi_9755_ = (0f >= _e3111);
                                                                            }
                                                                            let _e3117 = phi_9755_;
                                                                            let _e3119 = sqrt(select(_e3111, 0f, _e3117));
                                                                            if (_e3112 != _e3112) {
                                                                                phi_9770_ = true;
                                                                            } else {
                                                                                phi_9770_ = (0f >= _e3112);
                                                                            }
                                                                            let _e3123 = phi_9770_;
                                                                            let _e3125 = sqrt(select(_e3112, 0f, _e3123));
                                                                            if (_e3113 != _e3113) {
                                                                                phi_9785_ = true;
                                                                            } else {
                                                                                phi_9785_ = (0f >= _e3113);
                                                                            }
                                                                            let _e3129 = phi_9785_;
                                                                            let _e3131 = sqrt(select(_e3113, 0f, _e3129));
                                                                            let _e3132 = vec3<f32>(_e3119, _e3125, _e3131);
                                                                            let _e3136 = ((_e3095 - _e3062) * 0.5f);
                                                                            let _e3137 = ((_e3101 - _e3063) * 0.5f);
                                                                            let _e3138 = ((_e3107 - _e3064) * 0.5f);
                                                                            if (_e3136 != _e3136) {
                                                                                phi_9812_ = true;
                                                                            } else {
                                                                                phi_9812_ = (0f >= _e3136);
                                                                            }
                                                                            let _e3142 = phi_9812_;
                                                                            let _e3144 = sqrt(select(_e3136, 0f, _e3142));
                                                                            if (_e3137 != _e3137) {
                                                                                phi_9827_ = true;
                                                                            } else {
                                                                                phi_9827_ = (0f >= _e3137);
                                                                            }
                                                                            let _e3148 = phi_9827_;
                                                                            let _e3150 = sqrt(select(_e3137, 0f, _e3148));
                                                                            if (_e3138 != _e3138) {
                                                                                phi_9842_ = true;
                                                                            } else {
                                                                                phi_9842_ = (0f >= _e3138);
                                                                            }
                                                                            let _e3154 = phi_9842_;
                                                                            let _e3156 = sqrt(select(_e3138, 0f, _e3154));
                                                                            if ((_e3067 / _e3052) < 0f) {
                                                                                phi_2532_ = -(_e3144);
                                                                            } else {
                                                                                phi_2532_ = _e3144;
                                                                            }
                                                                            let _e3160 = phi_2532_;
                                                                            if ((_e3068 / _e3053) < 0f) {
                                                                                phi_2540_ = -(_e3150);
                                                                            } else {
                                                                                phi_2540_ = _e3150;
                                                                            }
                                                                            let _e3164 = phi_2540_;
                                                                            if ((_e3069 / _e3054) < 0f) {
                                                                                phi_2548_ = -(_e3156);
                                                                            } else {
                                                                                phi_2548_ = _e3156;
                                                                            }
                                                                            let _e3168 = phi_2548_;
                                                                            let _e3172 = (_e2013.x * _e3022);
                                                                            let _e3173 = (_e2013.y * _e3022);
                                                                            let _e3174 = (_e2013.z * _e3022);
                                                                            let _e3184 = vec3<f32>(((_e2015.x * _e3119) - (_e2013.x * _e3160)), ((_e2015.y * _e3125) - (_e2013.y * _e3164)), ((_e2015.z * _e3131) - (_e2013.z * _e3168)));
                                                                            let _e3191 = ((_e2015.x * _e3160) + (_e2013.x * _e3119));
                                                                            let _e3192 = ((_e2015.y * _e3164) + (_e2013.y * _e3125));
                                                                            let _e3193 = ((_e2015.z * _e3168) + (_e2013.z * _e3131));
                                                                            let _e3195 = vec3<f32>((_e2015.x * _e3022), (_e2015.y * _e3022), (_e2015.z * _e3022));
                                                                            let _e3196 = (_e3195 - _e3132);
                                                                            let _e3197 = (_e3172 - _e3160);
                                                                            let _e3198 = (_e3173 - _e3164);
                                                                            let _e3199 = (_e3174 - _e3168);
                                                                            let _e3212 = (_e3195 + _e3132);
                                                                            let _e3213 = (_e3172 + _e3160);
                                                                            let _e3214 = (_e3173 + _e3164);
                                                                            let _e3215 = (_e3174 + _e3168);
                                                                            let _e3225 = ((_e3212.x * _e3212.x) + (_e3213 * _e3213));
                                                                            let _e3226 = ((_e3212.y * _e3212.y) + (_e3214 * _e3214));
                                                                            let _e3227 = ((_e3212.z * _e3212.z) + (_e3215 * _e3215));
                                                                            let _e3237 = vec3(_e3022);
                                                                            let _e3238 = (_e3237 - _e3184);
                                                                            let _e3239 = -(vec3<f32>(_e3191, _e3192, _e3193));
                                                                            let _e3255 = (_e3237 + _e3184);
                                                                            let _e3265 = ((_e3255.x * _e3255.x) + (_e3191 * _e3191));
                                                                            let _e3266 = ((_e3255.y * _e3255.y) + (_e3192 * _e3192));
                                                                            let _e3267 = ((_e3255.z * _e3255.z) + (_e3193 * _e3193));
                                                                            phi_2655_ = vec3<f32>((((((_e3196.x * _e3196.x) + (_e3197 * _e3197)) / select(0.000000000000000000000000000001f, _e3225, (_e3225 > 0.000000000000000000000000000001f))) + (((_e3238.x * _e3238.x) + (_e3239.x * _e3239.x)) / select(0.000000000000000000000000000001f, _e3265, (_e3265 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3196.y * _e3196.y) + (_e3198 * _e3198)) / select(0.000000000000000000000000000001f, _e3226, (_e3226 > 0.000000000000000000000000000001f))) + (((_e3238.y * _e3238.y) + (_e3239.y * _e3239.y)) / select(0.000000000000000000000000000001f, _e3266, (_e3266 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3196.z * _e3196.z) + (_e3199 * _e3199)) / select(0.000000000000000000000000000001f, _e3227, (_e3227 > 0.000000000000000000000000000001f))) + (((_e3238.z * _e3238.z) + (_e3239.z * _e3239.z)) / select(0.000000000000000000000000000001f, _e3267, (_e3267 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                        }
                                                                        let _e3310 = phi_2655_;
                                                                        phi_2656_ = _e3310;
                                                                    }
                                                                    let _e3330 = phi_2656_;
                                                                    let _e3332 = (4f * _e2944);
                                                                    phi_2668_ = type_59((_e3330 * (((1f / (_e2995 + _e3009)) * _e2980) / _e3332)), ((_e2980 / _e2995) / _e3332));
                                                                }
                                                                let _e3339 = phi_2668_;
                                                                phi_2669_ = _e3339;
                                                                phi_2670_ = _e2956;
                                                            }
                                                            let _e3341 = phi_2669_;
                                                            let _e3343 = phi_2670_;
                                                            if _e3343 {
                                                                phi_2675_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                            } else {
                                                                phi_2675_ = _e3341;
                                                            }
                                                            let _e3345 = phi_2675_;
                                                            let _e3348 = (_e2021 == 1u);
                                                            if _e3348 {
                                                                phi_2703_ = _e3345;
                                                                phi_2704_ = type_59();
                                                            } else {
                                                                if (_e2944 != _e2944) {
                                                                    phi_10104_ = true;
                                                                } else {
                                                                    phi_10104_ = (0f >= _e2944);
                                                                }
                                                                let _e3352 = phi_10104_;
                                                                let _e3353 = select(_e2944, 0f, _e3352);
                                                                let _e3354 = (1f / _e2011);
                                                                let _e3358 = ((_e3354 * _e3354) * (1f - (_e3353 * _e3353)));
                                                                if (_e3358 >= 1f) {
                                                                    phi_10123_ = 1f;
                                                                } else {
                                                                    let _e3361 = sqrt((1f - _e3358));
                                                                    if (_e3353 != _e3353) {
                                                                        phi_10134_ = true;
                                                                    } else {
                                                                        phi_10134_ = (_e3361 <= _e3353);
                                                                    }
                                                                    let _e3365 = phi_10134_;
                                                                    let _e3370 = pow(((1f - _e2011) / (1f + _e2011)), 2f);
                                                                    phi_10123_ = (_e3370 + ((1f - _e3370) * pow((1f - select(_e3353, _e3361, _e3365)), 5f)));
                                                                }
                                                                let _e3377 = phi_10123_;
                                                                if (_e2919 <= 0f) {
                                                                    phi_10173_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    phi_10173_ = (_e2009 * (_e2919 * 0.31830987f));
                                                                }
                                                                let _e3382 = phi_10173_;
                                                                let _e3383 = (1f - _e3377);
                                                                phi_2703_ = type_59();
                                                                phi_2704_ = type_59((_e3345.member + vec3<f32>((_e3382.x * _e3383), (_e3382.y * _e3383), (_e3382.z * _e3383))), ((_e3377 * _e3345.member_1) + (_e3383 * _e2925)));
                                                            }
                                                            let _e3397 = phi_2703_;
                                                            let _e3399 = phi_2704_;
                                                            phi_2705_ = _e3397;
                                                            phi_2706_ = _e3399;
                                                            phi_2707_ = _e3348;
                                                        }
                                                        let _e3407 = phi_2705_;
                                                        let _e3409 = phi_2706_;
                                                        let _e3411 = phi_2707_;
                                                        if _e3411 {
                                                            phi_2711_ = _e3407;
                                                        } else {
                                                            phi_2711_ = _e3409;
                                                        }
                                                        let _e3413 = phi_2711_;
                                                        if ((_e3413.member_1 * _e2914) < 0.001f) {
                                                            let _e3545 = global_1.member.member_6;
                                                            phi_15469_ = _e2046;
                                                            phi_2854_ = vec3<f32>(_e3545.x, _e3545.y, _e3545.z);
                                                            phi_2855_ = 0f;
                                                            phi_2856_ = vec3<f32>(_e2905.x, _e2905.y, _e2905.z);
                                                        } else {
                                                            let _e3418 = ((_e2046 * 747796405u) + 2891336453u);
                                                            let _e3426 = (((_e3418 >> bitcast<u32>((((_e3418 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3418) * 277803737u);
                                                            let _e3429 = ((_e3426 >> bitcast<u32>(22i)) ^ _e3426);
                                                            let _e3435 = (1f - ((f32((_e3429 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2913));
                                                            let _e3437 = (1f - (_e3435 * _e3435));
                                                            if (_e3437 != _e3437) {
                                                                phi_10227_ = true;
                                                            } else {
                                                                phi_10227_ = (0f >= _e3437);
                                                            }
                                                            let _e3441 = phi_10227_;
                                                            let _e3443 = sqrt(select(_e3437, 0f, _e3441));
                                                            let _e3445 = ((_e3429 * 747796405u) + 2891336453u);
                                                            let _e3453 = (((_e3445 >> bitcast<u32>((((_e3445 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3445) * 277803737u);
                                                            let _e3456 = ((_e3453 >> bitcast<u32>(22i)) ^ _e3453);
                                                            let _e3460 = (f32((_e3456 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                            let _e3462 = select(-1f, 1f, (_e2905.z >= 0f));
                                                            let _e3464 = (-1f / (_e3462 + _e2905.z));
                                                            let _e3466 = ((_e2905.x * _e2905.y) * _e3464);
                                                            let _e3479 = (_e3443 * cos(_e3460));
                                                            let _e3484 = (_e3443 * sin(_e3460));
                                                            let _e3494 = ((((1f + (((_e3462 * _e2905.x) * _e2905.x) * _e3464)) * _e3479) + (_e3466 * _e3484)) + (_e2905.x * _e3435));
                                                            let _e3495 = ((((_e3462 * _e3466) * _e3479) + ((_e3462 + ((_e2905.y * _e2905.y) * _e3464)) * _e3484)) + (_e2905.y * _e3435));
                                                            let _e3496 = ((((-(_e3462) * _e2905.x) * _e3479) + (-(_e2905.y) * _e3484)) + (_e2905.z * _e3435));
                                                            let _e3502 = (((_e3494 * _e2905.x) + (_e3495 * _e2905.y)) + (_e3496 * _e2905.z));
                                                            let _e3503 = (_e3502 <= _e2912);
                                                            if _e3503 {
                                                                phi_2796_ = 0f;
                                                            } else {
                                                                phi_2796_ = (0.15915494f / _e2913);
                                                            }
                                                            let _e3506 = phi_2796_;
                                                            if (_e3506 > 0f) {
                                                                if _e3503 {
                                                                    phi_2852_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    if (_e3502 != _e3502) {
                                                                        phi_10298_ = true;
                                                                    } else {
                                                                        phi_10298_ = (1f <= _e3502);
                                                                    }
                                                                    let _e3511 = phi_10298_;
                                                                    let _e3517 = global_1.member.member_6[3u];
                                                                    let _e3520 = (1f - pow((acos(select(_e3502, 1f, _e3511)) / _e3517), 2f));
                                                                    if (_e3520 != _e3520) {
                                                                        phi_10313_ = true;
                                                                    } else {
                                                                        phi_10313_ = (0f >= _e3520);
                                                                    }
                                                                    let _e3524 = phi_10313_;
                                                                    let _e3527 = global_1.member.member_6;
                                                                    let _e3535 = (((1f - (0.6f * (1f - sqrt(select(_e3520, 0f, _e3524))))) * 1.25f) / _e2914);
                                                                    phi_2852_ = vec3<f32>((_e3527.x * _e3535), (_e3527.y * _e3535), (_e3527.z * _e3535));
                                                                }
                                                                let _e3541 = phi_2852_;
                                                                phi_2853_ = _e3541;
                                                            } else {
                                                                phi_2853_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3543 = phi_2853_;
                                                            phi_15469_ = _e3456;
                                                            phi_2854_ = _e3543;
                                                            phi_2855_ = _e3506;
                                                            phi_2856_ = vec3<f32>(_e3494, _e3495, _e3496);
                                                        }
                                                        let _e3551 = phi_15469_;
                                                        let _e3553 = phi_2854_;
                                                        let _e3555 = phi_2855_;
                                                        let _e3557 = phi_2856_;
                                                        phi_15468_ = _e3551;
                                                        phi_2857_ = _e3553;
                                                        phi_2858_ = _e3555;
                                                        phi_2859_ = _e3557;
                                                    }
                                                    let _e3559 = phi_15468_;
                                                    let _e3561 = phi_2857_;
                                                    let _e3563 = phi_2858_;
                                                    let _e3565 = phi_2859_;
                                                    phi_15430_ = _e3559;
                                                    phi_4064_ = _e3561;
                                                    phi_4065_ = _e3563;
                                                    phi_4066_ = _e3565;
                                                    phi_4067_ = 1000000000000000000000000000000f;
                                                } else {
                                                    if (_e2050 == (_e2042 + 1u)) {
                                                        if ((_e211 & 1u) == 0u) {
                                                            phi_15434_ = _e2046;
                                                            phi_3383_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_3384_ = 0f;
                                                            phi_3385_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_3386_ = 1000000000000000000000000000000f;
                                                        } else {
                                                            if (_e214 > 0u) {
                                                                let _e2395 = global_1.member.member_2[1u];
                                                                let _e2397 = ((_e2046 * 747796405u) + 2891336453u);
                                                                let _e2405 = (((_e2397 >> bitcast<u32>((((_e2397 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2397) * 277803737u);
                                                                let _e2408 = ((_e2405 >> bitcast<u32>(22i)) ^ _e2405);
                                                                phi_2880_ = 0u;
                                                                phi_2883_ = _e214;
                                                                loop {
                                                                    let _e2414 = phi_2880_;
                                                                    let _e2416 = phi_2883_;
                                                                    local_17 = _e2414;
                                                                    local_18 = _e2414;
                                                                    let _e2418 = ((_e2414 + 1u) < _e2416);
                                                                    if _e2418 {
                                                                        let _e2420 = ((_e2414 + _e2416) / 2u);
                                                                        let _e2423 = global_6.member[(_e2395 + _e2420)];
                                                                        let _e2424 = (_e2423 <= (f32((_e2408 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                        phi_2881_ = select(_e2414, _e2420, _e2424);
                                                                        phi_2884_ = select(_e2420, _e2416, _e2424);
                                                                    } else {
                                                                        phi_2881_ = u32();
                                                                        phi_2884_ = u32();
                                                                    }
                                                                    let _e2428 = phi_2881_;
                                                                    let _e2430 = phi_2884_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_2880_ = _e2428;
                                                                        phi_2883_ = _e2430;
                                                                        break if !(_e2418);
                                                                    }
                                                                }
                                                                let _e2433 = local_17;
                                                                let _e2434 = (_e2395 + _e2433);
                                                                let _e2437 = global_6.member[(_e2434 + 1u)];
                                                                let _e2439 = global_6.member[_e2434];
                                                                let _e2443 = global_1.member.member_2[0u];
                                                                let _e2445 = local_18;
                                                                let _e2448 = global_6.member[(_e2443 + _e2445)];
                                                                let _e2451 = ((_e2408 * 747796405u) + 2891336453u);
                                                                let _e2459 = (((_e2451 >> bitcast<u32>((((_e2451 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2451) * 277803737u);
                                                                let _e2462 = ((_e2459 >> bitcast<u32>(22i)) ^ _e2459);
                                                                let _e2465 = f32((_e2462 >> bitcast<u32>(8i)));
                                                                let _e2466 = (_e2465 * 0.000000059604645f);
                                                                let _e2468 = ((_e2462 * 747796405u) + 2891336453u);
                                                                let _e2476 = (((_e2468 >> bitcast<u32>((((_e2468 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2468) * 277803737u);
                                                                let _e2479 = ((_e2476 >> bitcast<u32>(22i)) ^ _e2476);
                                                                let _e2482 = f32((_e2479 >> bitcast<u32>(8i)));
                                                                let _e2485 = global_2.member[bitcast<u32>(_e2448)];
                                                                let _e2488 = (_e2485.member_6.x == 1u);
                                                                if _e2488 {
                                                                    let _e2630 = (_e2485.member.x - _e1171);
                                                                    let _e2631 = (_e2485.member.y - _e1172);
                                                                    let _e2632 = (_e2485.member.z - _e1173);
                                                                    let _e2637 = (((_e2630 * _e2630) + (_e2631 * _e2631)) + (_e2632 * _e2632));
                                                                    let _e2638 = (_e2485.member.w * _e2485.member.w);
                                                                    let _e2640 = (_e2637 <= (_e2638 * 1.000001f));
                                                                    if _e2640 {
                                                                        let _e2749 = (_e2465 * 0.00000037450704f);
                                                                        let _e2751 = ((_e2482 * 0.00000011920929f) - 1f);
                                                                        let _e2753 = (1f - (_e2751 * _e2751));
                                                                        if (_e2753 != _e2753) {
                                                                            phi_9295_ = true;
                                                                        } else {
                                                                            phi_9295_ = (0f >= _e2753);
                                                                        }
                                                                        let _e2757 = phi_9295_;
                                                                        let _e2759 = sqrt(select(_e2753, 0f, _e2757));
                                                                        let _e2761 = (_e2759 * cos(_e2749));
                                                                        let _e2763 = (_e2759 * sin(_e2749));
                                                                        let _e2768 = (_e2485.member.x + (_e2761 * _e2485.member.w));
                                                                        let _e2769 = (_e2485.member.y + (_e2763 * _e2485.member.w));
                                                                        let _e2770 = (_e2485.member.z + (_e2751 * _e2485.member.w));
                                                                        if _e2640 {
                                                                            let _e2783 = (_e2768 - _e1171);
                                                                            let _e2784 = (_e2769 - _e1172);
                                                                            let _e2785 = (_e2770 - _e1173);
                                                                            let _e2790 = (((_e2783 * _e2783) + (_e2784 * _e2784)) + (_e2785 * _e2785));
                                                                            phi_9364_ = (_e2790 / (((abs((((_e2761 * _e2783) + (_e2763 * _e2784)) + (_e2751 * _e2785))) / sqrt(_e2790)) * 12.566371f) * _e2638));
                                                                        } else {
                                                                            let _e2772 = (_e2638 / _e2637);
                                                                            if (_e2772 != _e2772) {
                                                                                phi_9383_ = true;
                                                                            } else {
                                                                                phi_9383_ = (1f <= _e2772);
                                                                            }
                                                                            let _e2776 = phi_9383_;
                                                                            let _e2777 = select(_e2772, 1f, _e2776);
                                                                            phi_9364_ = (0.15915494f / (_e2777 / (1f + sqrt((1f - _e2777)))));
                                                                        }
                                                                        let _e2803 = phi_9364_;
                                                                        phi_3096_ = type_60(vec3<f32>(_e2768, _e2769, _e2770), vec3<f32>(_e2761, _e2763, _e2751), _e2803);
                                                                    } else {
                                                                        let _e2641 = (_e2638 / _e2637);
                                                                        let _e2642 = (_e2641 != _e2641);
                                                                        if _e2642 {
                                                                            phi_9147_ = true;
                                                                        } else {
                                                                            phi_9147_ = (1f <= _e2641);
                                                                        }
                                                                        let _e2645 = phi_9147_;
                                                                        let _e2646 = select(_e2641, 1f, _e2645);
                                                                        if _e2642 {
                                                                            phi_9170_ = true;
                                                                        } else {
                                                                            phi_9170_ = (1f <= _e2641);
                                                                        }
                                                                        let _e2650 = phi_9170_;
                                                                        let _e2651 = select(_e2641, 1f, _e2650);
                                                                        let _e2655 = (_e2651 / (1f + sqrt((1f - _e2651))));
                                                                        if (_e2646 < 0.00068523f) {
                                                                            phi_3031_ = (_e2646 * _e2466);
                                                                        } else {
                                                                            let _e2658 = (1f - (_e2466 * _e2655));
                                                                            let _e2660 = (1f - (_e2658 * _e2658));
                                                                            if (_e2660 != _e2660) {
                                                                                phi_9185_ = true;
                                                                            } else {
                                                                                phi_9185_ = (0f >= _e2660);
                                                                            }
                                                                            let _e2664 = phi_9185_;
                                                                            phi_3031_ = select(_e2660, 0f, _e2664);
                                                                        }
                                                                        let _e2668 = phi_3031_;
                                                                        let _e2669 = (1f - _e2668);
                                                                        if (_e2669 != _e2669) {
                                                                            phi_9200_ = true;
                                                                        } else {
                                                                            phi_9200_ = (0f >= _e2669);
                                                                        }
                                                                        let _e2673 = phi_9200_;
                                                                        let _e2678 = (1f - (_e2668 / _e2646));
                                                                        if (_e2678 != _e2678) {
                                                                            phi_9215_ = true;
                                                                        } else {
                                                                            phi_9215_ = (0f >= _e2678);
                                                                        }
                                                                        let _e2682 = phi_9215_;
                                                                        let _e2686 = ((_e2668 / sqrt(_e2646)) + (sqrt(select(_e2669, 0f, _e2673)) * sqrt(select(_e2678, 0f, _e2682))));
                                                                        let _e2688 = (1f - (_e2686 * _e2686));
                                                                        if (_e2688 != _e2688) {
                                                                            phi_9230_ = true;
                                                                        } else {
                                                                            phi_9230_ = (0f >= _e2688);
                                                                        }
                                                                        let _e2692 = phi_9230_;
                                                                        let _e2694 = sqrt(select(_e2688, 0f, _e2692));
                                                                        let _e2695 = (_e2482 * 0.00000037450704f);
                                                                        let _e2696 = sqrt(_e2637);
                                                                        let _e2697 = (_e2630 / _e2696);
                                                                        let _e2698 = (_e2631 / _e2696);
                                                                        let _e2699 = (_e2632 / _e2696);
                                                                        let _e2701 = select(-1f, 1f, (_e2699 >= 0f));
                                                                        let _e2703 = (-1f / (_e2701 + _e2699));
                                                                        let _e2705 = ((_e2697 * _e2698) * _e2703);
                                                                        let _e2718 = (_e2694 * cos(_e2695));
                                                                        let _e2723 = (_e2694 * sin(_e2695));
                                                                        let _e2736 = -(((((1f + (((_e2701 * _e2697) * _e2697) * _e2703)) * _e2718) + (_e2705 * _e2723)) + (_e2697 * _e2686)));
                                                                        let _e2737 = -(((((_e2701 * _e2705) * _e2718) + ((_e2701 + ((_e2698 * _e2698) * _e2703)) * _e2723)) + (_e2698 * _e2686)));
                                                                        let _e2738 = -(((((-(_e2701) * _e2697) * _e2718) + (-(_e2698) * _e2723)) + (_e2699 * _e2686)));
                                                                        phi_3096_ = type_60(vec3<f32>((_e2485.member.x + (_e2736 * _e2485.member.w)), (_e2485.member.y + (_e2737 * _e2485.member.w)), (_e2485.member.z + (_e2738 * _e2485.member.w))), vec3<f32>(_e2736, _e2737, _e2738), (0.15915494f / _e2655));
                                                                    }
                                                                    let _e2806 = phi_3096_;
                                                                    phi_3282_ = _e2806;
                                                                } else {
                                                                    let _e2489 = sqrt(_e2466);
                                                                    let _e2490 = (1f - _e2489);
                                                                    let _e2491 = ((_e2482 * 0.000000059604645f) * _e2489);
                                                                    let _e2513 = (_e2489 - _e2491);
                                                                    let _e2517 = (((_e2485.member.x * _e2490) + (_e2485.member_1.x * _e2491)) + (_e2485.member_2.x * _e2513));
                                                                    let _e2518 = (((_e2485.member.y * _e2490) + (_e2485.member_1.y * _e2491)) + (_e2485.member_2.y * _e2513));
                                                                    let _e2519 = (((_e2485.member.z * _e2490) + (_e2485.member_1.z * _e2491)) + (_e2485.member_2.z * _e2513));
                                                                    let _e2521 = (_e2485.member_1.x - _e2485.member.x);
                                                                    let _e2522 = (_e2485.member_1.y - _e2485.member.y);
                                                                    let _e2523 = (_e2485.member_1.z - _e2485.member.z);
                                                                    let _e2524 = (_e2485.member_2.x - _e2485.member.x);
                                                                    let _e2525 = (_e2485.member_2.y - _e2485.member.y);
                                                                    let _e2526 = (_e2485.member_2.z - _e2485.member.z);
                                                                    let _e2529 = ((_e2522 * _e2526) - (_e2525 * _e2523));
                                                                    let _e2532 = ((_e2523 * _e2524) - (_e2526 * _e2521));
                                                                    let _e2535 = ((_e2521 * _e2525) - (_e2524 * _e2522));
                                                                    let _e2541 = sqrt((((_e2529 * _e2529) + (_e2532 * _e2532)) + (_e2535 * _e2535)));
                                                                    let _e2542 = (1f / _e2541);
                                                                    let _e2543 = (_e2529 * _e2542);
                                                                    let _e2544 = (_e2532 * _e2542);
                                                                    let _e2545 = (_e2535 * _e2542);
                                                                    if _e2488 {
                                                                        let _e2578 = (_e2485.member.x - _e1171);
                                                                        let _e2579 = (_e2485.member.y - _e1172);
                                                                        let _e2580 = (_e2485.member.z - _e1173);
                                                                        let _e2585 = (((_e2578 * _e2578) + (_e2579 * _e2579)) + (_e2580 * _e2580));
                                                                        let _e2586 = (_e2485.member.w * _e2485.member.w);
                                                                        if (_e2585 <= (_e2586 * 1.000001f)) {
                                                                            let _e2600 = (_e2517 - _e1171);
                                                                            let _e2601 = (_e2518 - _e1172);
                                                                            let _e2602 = (_e2519 - _e1173);
                                                                            let _e2607 = (((_e2600 * _e2600) + (_e2601 * _e2601)) + (_e2602 * _e2602));
                                                                            phi_9113_ = (_e2607 / (((abs((((_e2543 * _e2600) + (_e2544 * _e2601)) + (_e2545 * _e2602))) / sqrt(_e2607)) * 12.566371f) * _e2586));
                                                                        } else {
                                                                            let _e2589 = (_e2586 / _e2585);
                                                                            if (_e2589 != _e2589) {
                                                                                phi_9132_ = true;
                                                                            } else {
                                                                                phi_9132_ = (1f <= _e2589);
                                                                            }
                                                                            let _e2593 = phi_9132_;
                                                                            let _e2594 = select(_e2589, 1f, _e2593);
                                                                            phi_9113_ = (0.15915494f / (_e2594 / (1f + sqrt((1f - _e2594)))));
                                                                        }
                                                                        let _e2620 = phi_9113_;
                                                                        phi_3280_ = _e2620;
                                                                    } else {
                                                                        let _e2547 = (_e2517 - _e1171);
                                                                        let _e2548 = (_e2518 - _e1172);
                                                                        let _e2549 = (_e2519 - _e1173);
                                                                        let _e2554 = (((_e2547 * _e2547) + (_e2548 * _e2548)) + (_e2549 * _e2549));
                                                                        let _e2562 = (abs((((_e2543 * _e2547) + (_e2544 * _e2548)) + (_e2545 * _e2549))) / sqrt(_e2554));
                                                                        if (_e2562 <= 0f) {
                                                                            phi_3279_ = 0f;
                                                                        } else {
                                                                            if (_e2485.member_6.x == 0u) {
                                                                                phi_3276_ = (_e2541 * 0.5f);
                                                                            } else {
                                                                                phi_3276_ = ((12.566371f * _e2485.member.w) * _e2485.member.w);
                                                                            }
                                                                            let _e2571 = phi_3276_;
                                                                            phi_3279_ = (_e2554 / (_e2562 * _e2571));
                                                                        }
                                                                        let _e2575 = phi_3279_;
                                                                        phi_3280_ = _e2575;
                                                                    }
                                                                    let _e2622 = phi_3280_;
                                                                    phi_3282_ = type_60(vec3<f32>(_e2517, _e2518, _e2519), vec3<f32>(_e2543, _e2544, _e2545), _e2622);
                                                                }
                                                                let _e2808 = phi_3282_;
                                                                let _e2813 = (_e2808.member.x - _e1171);
                                                                let _e2815 = (_e2808.member.y - _e1172);
                                                                let _e2817 = (_e2808.member.z - _e1173);
                                                                let _e2823 = sqrt((((_e2813 * _e2813) + (_e2815 * _e2815)) + (_e2817 * _e2817)));
                                                                let _e2824 = (_e2813 / _e2823);
                                                                let _e2825 = (_e2815 / _e2823);
                                                                let _e2826 = (_e2817 / _e2823);
                                                                if (_e2808.member_2 > 0f) {
                                                                    let _e2838 = global_4.member[_e2485.member_6.y];
                                                                    if (_e2838.member == 4u) {
                                                                        if (_e2838.member_1 == 1u) {
                                                                            phi_3360_ = true;
                                                                        } else {
                                                                            phi_3360_ = ((((-(_e2824) * _e2808.member_1.x) + (-(_e2825) * _e2808.member_1.y)) + (-(_e2826) * _e2808.member_1.z)) > 0f);
                                                                        }
                                                                        let _e2853 = phi_3360_;
                                                                        if _e2853 {
                                                                            phi_3369_ = vec3<f32>(_e2838.member_4.x, _e2838.member_4.y, _e2838.member_4.z);
                                                                        } else {
                                                                            phi_3369_ = vec3<f32>();
                                                                        }
                                                                        let _e2860 = phi_3369_;
                                                                        phi_3371_ = _e2860;
                                                                        phi_3372_ = select(true, false, _e2853);
                                                                    } else {
                                                                        phi_3371_ = vec3<f32>();
                                                                        phi_3372_ = true;
                                                                    }
                                                                    let _e2863 = phi_3371_;
                                                                    let _e2865 = phi_3372_;
                                                                    phi_3377_ = select(_e2863, vec3<f32>(0f, 0f, 0f), vec3(_e2865));
                                                                    phi_3378_ = ((_e2437 - _e2439) * _e2808.member_2);
                                                                } else {
                                                                    phi_3377_ = vec3<f32>(0f, 0f, 0f);
                                                                    phi_3378_ = 0f;
                                                                }
                                                                let _e2869 = phi_3377_;
                                                                let _e2871 = phi_3378_;
                                                                phi_15435_ = _e2479;
                                                                phi_3379_ = _e2869;
                                                                phi_3380_ = _e2871;
                                                                phi_3381_ = vec3<f32>(_e2824, _e2825, _e2826);
                                                                phi_3382_ = ((_e2823 * 0.9999f) - _e1164);
                                                            } else {
                                                                phi_15435_ = _e2046;
                                                                phi_3379_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3380_ = 0f;
                                                                phi_3381_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_3382_ = 1000000000000000000000000000000f;
                                                            }
                                                            let _e2873 = phi_15435_;
                                                            let _e2875 = phi_3379_;
                                                            let _e2877 = phi_3380_;
                                                            let _e2879 = phi_3381_;
                                                            let _e2881 = phi_3382_;
                                                            phi_15434_ = _e2873;
                                                            phi_3383_ = _e2875;
                                                            phi_3384_ = _e2877;
                                                            phi_3385_ = _e2879;
                                                            phi_3386_ = _e2881;
                                                        }
                                                        let _e2883 = phi_15434_;
                                                        let _e2885 = phi_3383_;
                                                        let _e2887 = phi_3384_;
                                                        let _e2889 = phi_3385_;
                                                        let _e2891 = phi_3386_;
                                                        phi_15431_ = _e2883;
                                                        phi_4060_ = _e2885;
                                                        phi_4061_ = _e2887;
                                                        phi_4062_ = _e2889;
                                                        phi_4063_ = _e2891;
                                                    } else {
                                                        if ((_e211 & 2u) == 0u) {
                                                            phi_15432_ = _e2046;
                                                            phi_4057_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_4058_ = 0f;
                                                            phi_4059_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e2060 = ((_e2046 * 747796405u) + 2891336453u);
                                                            let _e2068 = (((_e2060 >> bitcast<u32>((((_e2060 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2060) * 277803737u);
                                                            let _e2071 = ((_e2068 >> bitcast<u32>(22i)) ^ _e2068);
                                                            let _e2075 = (f32((_e2071 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2077 = ((_e2071 * 747796405u) + 2891336453u);
                                                            let _e2085 = (((_e2077 >> bitcast<u32>((((_e2077 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2077) * 277803737u);
                                                            let _e2088 = ((_e2085 >> bitcast<u32>(22i)) ^ _e2085);
                                                            let _e2092 = (f32((_e2088 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2095 = global_1.member.member_3[1u];
                                                            let _e2098 = global_1.member.member_3[2u];
                                                            let _e2101 = global_1.member.member_2[3u];
                                                            phi_3407_ = 0u;
                                                            phi_3410_ = _e2098;
                                                            loop {
                                                                let _e2103 = phi_3407_;
                                                                let _e2105 = phi_3410_;
                                                                local_12 = _e2103;
                                                                local_15 = _e2103;
                                                                local_16 = _e2103;
                                                                let _e2107 = ((_e2103 + 1u) < _e2105);
                                                                if _e2107 {
                                                                    let _e2109 = ((_e2103 + _e2105) / 2u);
                                                                    let _e2112 = global_6.member[(_e2101 + _e2109)];
                                                                    let _e2113 = (_e2112 <= _e2075);
                                                                    phi_3408_ = select(_e2103, _e2109, _e2113);
                                                                    phi_3411_ = select(_e2109, _e2105, _e2113);
                                                                } else {
                                                                    phi_3408_ = u32();
                                                                    phi_3411_ = u32();
                                                                }
                                                                let _e2117 = phi_3408_;
                                                                let _e2119 = phi_3411_;
                                                                continue;
                                                                continuing {
                                                                    phi_3407_ = _e2117;
                                                                    phi_3410_ = _e2119;
                                                                    break if !(_e2107);
                                                                }
                                                            }
                                                            let _e2123 = global_1.member.member_3[0u];
                                                            let _e2124 = (_e2095 + 1u);
                                                            let _e2126 = local_12;
                                                            let _e2128 = (_e2123 + (_e2126 * _e2124));
                                                            phi_3448_ = 0u;
                                                            phi_3451_ = _e2095;
                                                            loop {
                                                                let _e2130 = phi_3448_;
                                                                let _e2132 = phi_3451_;
                                                                local_13 = _e2130;
                                                                local_14 = _e2130;
                                                                let _e2134 = ((_e2130 + 1u) < _e2132);
                                                                if _e2134 {
                                                                    let _e2136 = ((_e2130 + _e2132) / 2u);
                                                                    let _e2139 = global_6.member[(_e2128 + _e2136)];
                                                                    let _e2140 = (_e2139 <= _e2092);
                                                                    phi_3449_ = select(_e2130, _e2136, _e2140);
                                                                    phi_3452_ = select(_e2136, _e2132, _e2140);
                                                                } else {
                                                                    phi_3449_ = u32();
                                                                    phi_3452_ = u32();
                                                                }
                                                                let _e2144 = phi_3449_;
                                                                let _e2146 = phi_3452_;
                                                                continue;
                                                                continuing {
                                                                    phi_3448_ = _e2144;
                                                                    phi_3451_ = _e2146;
                                                                    break if !(_e2134);
                                                                }
                                                            }
                                                            let _e2149 = local_13;
                                                            let _e2152 = local_14;
                                                            let _e2153 = (_e2128 + _e2152);
                                                            let _e2155 = global_6.member[_e2153];
                                                            let _e2158 = global_6.member[(_e2153 + 1u)];
                                                            if (_e2158 > _e2155) {
                                                                let _e2162 = ((_e2092 - _e2155) / (_e2158 - _e2155));
                                                                let _e2164 = select(_e2162, 0f, (_e2162 < 0f));
                                                                phi_3528_ = select(_e2164, 0.99999f, (_e2164 > 0.99999f));
                                                            } else {
                                                                phi_3528_ = 0.5f;
                                                            }
                                                            let _e2168 = phi_3528_;
                                                            let _e2170 = f32(_e2095);
                                                            let _e2173 = local_15;
                                                            let _e2176 = local_16;
                                                            let _e2177 = (_e2101 + _e2176);
                                                            let _e2179 = global_6.member[_e2177];
                                                            let _e2182 = global_6.member[(_e2177 + 1u)];
                                                            if (_e2182 > _e2179) {
                                                                let _e2186 = ((_e2075 - _e2179) / (_e2182 - _e2179));
                                                                let _e2188 = select(_e2186, 0f, (_e2186 < 0f));
                                                                phi_3577_ = select(_e2188, 0.99999f, (_e2188 > 0.99999f));
                                                            } else {
                                                                phi_3577_ = 0.5f;
                                                            }
                                                            let _e2192 = phi_3577_;
                                                            let _e2194 = f32(_e2098);
                                                            let _e2197 = ((((f32(_e2149) + _e2168) / _e2170) - 0.5f) * 6.2831855f);
                                                            let _e2198 = (((f32(_e2173) + _e2192) / _e2194) * 3.1415927f);
                                                            let _e2199 = sin(_e2198);
                                                            let _e2201 = (_e2199 * cos(_e2197));
                                                            let _e2202 = cos(_e2198);
                                                            let _e2204 = (_e2199 * sin(_e2197));
                                                            let _e2207 = ((atan2(_e2204, _e2201) * 0.15915494f) + 0.5f);
                                                            let _e2211 = select(_e2202, -1f, (_e2202 < -1f));
                                                            let _e2214 = acos(select(_e2211, 1f, (_e2211 > 1f)));
                                                            let _e2216 = ((_e2207 - floor(_e2207)) * _e2170);
                                                            let _e2221 = select(select(u32(_e2216), 0u, (_e2216 < 0f)), 4294967295u, (_e2216 > 4294967000f));
                                                            let _e2222 = (_e2095 - 1u);
                                                            let _e2225 = ((_e2214 * 0.31830987f) * _e2194);
                                                            let _e2230 = select(select(u32(_e2225), 0u, (_e2225 < 0f)), 4294967295u, (_e2225 > 4294967000f));
                                                            let _e2231 = (_e2098 - 1u);
                                                            let _e2233 = select(_e2230, _e2231, (_e2231 < _e2230));
                                                            let _e2234 = sin(_e2214);
                                                            if (_e2234 <= 0f) {
                                                                phi_3733_ = 0f;
                                                            } else {
                                                                let _e2238 = (_e2101 + _e2233);
                                                                let _e2241 = global_6.member[(_e2238 + 1u)];
                                                                let _e2243 = global_6.member[_e2238];
                                                                let _e2246 = ((_e2123 + (_e2233 * _e2124)) + select(_e2221, _e2222, (_e2222 < _e2221)));
                                                                let _e2249 = global_6.member[(_e2246 + 1u)];
                                                                let _e2251 = global_6.member[_e2246];
                                                                phi_3733_ = ((((_e2241 - _e2243) * _e2194) * ((_e2249 - _e2251) * _e2170)) / (19.73921f * _e2234));
                                                            }
                                                            let _e2258 = phi_3733_;
                                                            let _e2259 = (_e2258 > 0f);
                                                            if _e2259 {
                                                                let _e2261 = (_e2216 - 0.5f);
                                                                let _e2262 = (_e2225 - 0.5f);
                                                                let _e2263 = f32(_e2231);
                                                                if (0f <= _e2263) {
                                                                } else {
                                                                    phi_15401_ = _e2088;
                                                                    phi_15388_ = true;
                                                                    break;
                                                                }
                                                                let _e2266 = select(_e2262, 0f, (_e2262 < 0f));
                                                                let _e2268 = select(_e2266, _e2263, (_e2266 > _e2263));
                                                                let _e2269 = floor(_e2261);
                                                                let _e2270 = floor(_e2268);
                                                                let _e2271 = (_e2261 - _e2269);
                                                                let _e2272 = (_e2268 - _e2270);
                                                                let _e2283 = (_e2095 == 0u);
                                                                if _e2283 {
                                                                    phi_15401_ = _e2088;
                                                                    phi_15388_ = true;
                                                                    break;
                                                                }
                                                                let _e2284 = (bitcast<u32>((select(0i, select(select(i32(_e2269), i32(-2147483648), (_e2269 < -2147483600f)), 2147483647i, (_e2269 > 2147483500f)), (_e2269 == _e2269)) + bitcast<i32>(_e2095))) % _e2095);
                                                                let _e2289 = select(select(u32(_e2270), 0u, (_e2270 < 0f)), 4294967295u, (_e2270 > 4294967000f));
                                                                let _e2290 = (_e2289 + 1u);
                                                                let _e2295 = global_1.member.member_2[2u];
                                                                let _e2296 = (_e2289 * _e2095);
                                                                if _e2283 {
                                                                    phi_15401_ = _e2088;
                                                                    phi_15388_ = true;
                                                                    break;
                                                                }
                                                                let _e2297 = (_e2284 % _e2095);
                                                                let _e2300 = (_e2295 + (3u * (_e2296 + _e2297)));
                                                                let _e2302 = global_6.member[_e2300];
                                                                let _e2305 = global_6.member[(_e2300 + 1u)];
                                                                let _e2308 = global_6.member[(_e2300 + 2u)];
                                                                if _e2283 {
                                                                    phi_15401_ = _e2088;
                                                                    phi_15388_ = true;
                                                                    break;
                                                                }
                                                                let _e2311 = ((_e2284 + 1u) % _e2095);
                                                                let _e2314 = (_e2295 + (3u * (_e2296 + _e2311)));
                                                                let _e2316 = global_6.member[_e2314];
                                                                let _e2319 = global_6.member[(_e2314 + 1u)];
                                                                let _e2322 = global_6.member[(_e2314 + 2u)];
                                                                let _e2330 = (vec3<f32>(_e2302, _e2305, _e2308) + vec3<f32>(((_e2316 - _e2302) * _e2271), ((_e2319 - _e2305) * _e2271), ((_e2322 - _e2308) * _e2271)));
                                                                let _e2331 = (select(_e2290, _e2231, (_e2231 < _e2290)) * _e2095);
                                                                if _e2283 {
                                                                    phi_15401_ = _e2088;
                                                                    phi_15388_ = true;
                                                                    break;
                                                                }
                                                                let _e2334 = (_e2295 + (3u * (_e2331 + _e2297)));
                                                                let _e2336 = global_6.member[_e2334];
                                                                let _e2339 = global_6.member[(_e2334 + 1u)];
                                                                let _e2342 = global_6.member[(_e2334 + 2u)];
                                                                if _e2283 {
                                                                    phi_15401_ = _e2088;
                                                                    phi_15388_ = true;
                                                                    break;
                                                                }
                                                                let _e2346 = (_e2295 + (3u * (_e2331 + _e2311)));
                                                                let _e2348 = global_6.member[_e2346];
                                                                let _e2351 = global_6.member[(_e2346 + 1u)];
                                                                let _e2354 = global_6.member[(_e2346 + 2u)];
                                                                let _e2362 = (vec3<f32>(_e2336, _e2339, _e2342) + vec3<f32>(((_e2348 - _e2336) * _e2271), ((_e2351 - _e2339) * _e2271), ((_e2354 - _e2342) * _e2271)));
                                                                phi_4054_ = (_e2330 + vec3<f32>(((_e2362.x - _e2330.x) * _e2272), ((_e2362.y - _e2330.y) * _e2272), ((_e2362.z - _e2330.z) * _e2272)));
                                                                phi_4056_ = vec3<f32>(_e2201, _e2202, _e2204);
                                                            } else {
                                                                phi_4054_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_4056_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2378 = phi_4054_;
                                                            let _e2380 = phi_4056_;
                                                            phi_15432_ = _e2088;
                                                            phi_4057_ = _e2378;
                                                            phi_4058_ = select(0f, _e2258, _e2259);
                                                            phi_4059_ = _e2380;
                                                        }
                                                        let _e2383 = phi_15432_;
                                                        let _e2385 = phi_4057_;
                                                        let _e2387 = phi_4058_;
                                                        let _e2389 = phi_4059_;
                                                        phi_15431_ = _e2383;
                                                        phi_4060_ = _e2385;
                                                        phi_4061_ = _e2387;
                                                        phi_4062_ = _e2389;
                                                        phi_4063_ = 1000000000000000000000000000000f;
                                                    }
                                                    let _e2893 = phi_15431_;
                                                    let _e2895 = phi_4060_;
                                                    let _e2897 = phi_4061_;
                                                    let _e2899 = phi_4062_;
                                                    let _e2901 = phi_4063_;
                                                    phi_15430_ = _e2893;
                                                    phi_4064_ = _e2895;
                                                    phi_4065_ = _e2897;
                                                    phi_4066_ = _e2899;
                                                    phi_4067_ = _e2901;
                                                }
                                                let _e3567 = phi_15430_;
                                                let _e3569 = phi_4064_;
                                                let _e3571 = phi_4065_;
                                                let _e3573 = phi_4066_;
                                                let _e3575 = phi_4067_;
                                                phi_15429_ = _e3567;
                                                phi_4068_ = _e3569;
                                                phi_4069_ = _e3571;
                                                phi_4070_ = _e3573;
                                                phi_4071_ = _e3575;
                                            }
                                            let _e3644 = phi_15429_;
                                            let _e3646 = phi_4068_;
                                            let _e3648 = phi_4069_;
                                            let _e3650 = phi_4070_;
                                            let _e3652 = phi_4071_;
                                            if ((((_e3646.x * _e3646.x) + (_e3646.y * _e3646.y)) + (_e3646.z * _e3646.z)) > 0f) {
                                                let _e3669 = (((_e3650.x * _e1146.x) + (_e3650.y * _e1146.y)) + (_e3650.z * _e1146.z));
                                                if (_e3669 != _e3669) {
                                                    phi_10328_ = true;
                                                } else {
                                                    phi_10328_ = (0f >= _e3669);
                                                }
                                                let _e3673 = phi_10328_;
                                                let _e3675 = (select(_e3669, 0f, _e3673) * 0.31830987f);
                                                if (_e2021 == 0u) {
                                                    if (_e3669 <= 0f) {
                                                        phi_11125_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_11125_ = (_e2015 * (_e3669 * 0.31830987f));
                                                    }
                                                    let _e4154 = phi_11125_;
                                                    phi_4467_ = type_59(_e4154, _e3675);
                                                    phi_4468_ = type_59();
                                                    phi_4469_ = true;
                                                } else {
                                                    let _e3684 = (((_e1120.x * _e1192) + (_e1120.y * _e1193)) + (_e1120.z * _e1195));
                                                    let _e3689 = (((_e1120.x * _e1188) + (_e1120.y * _e1198)) + (_e1120.z * _e1199));
                                                    let _e3694 = (((_e1120.x * _e1146.x) + (_e1120.y * _e1146.y)) + (_e1120.z * _e1146.z));
                                                    let _e3699 = (((_e3650.x * _e1192) + (_e3650.y * _e1193)) + (_e3650.z * _e1195));
                                                    let _e3704 = (((_e3650.x * _e1188) + (_e3650.y * _e1198)) + (_e3650.z * _e1199));
                                                    if (_e3694 <= 0f) {
                                                        phi_4431_ = type_59();
                                                        phi_4432_ = true;
                                                    } else {
                                                        let _e3706 = (_e3669 <= 0f);
                                                        if _e3706 {
                                                            phi_4430_ = type_59();
                                                        } else {
                                                            let _e3707 = (_e3684 + _e3699);
                                                            let _e3708 = (_e3689 + _e3704);
                                                            let _e3709 = (_e3694 + _e3669);
                                                            let _e3716 = (1f / sqrt((((_e3707 * _e3707) + (_e3708 * _e3708)) + (_e3709 * _e3709))));
                                                            let _e3719 = (_e3709 * _e3716);
                                                            if (_e3719 <= 0f) {
                                                                phi_10438_ = 0f;
                                                            } else {
                                                                let _e3721 = (_e2019 * _e2019);
                                                                let _e3725 = ((((_e3721 - 1f) * _e3719) * _e3719) + 1f);
                                                                phi_10438_ = (_e3721 / ((3.1415927f * _e3725) * _e3725));
                                                            }
                                                            let _e3730 = phi_10438_;
                                                            let _e3731 = (_e3694 * _e3694);
                                                            if (_e3731 >= 1f) {
                                                                phi_10466_ = 0f;
                                                            } else {
                                                                phi_10466_ = ((sqrt((1f + (((_e2019 * _e2019) * ((_e3684 * _e3684) + (_e3689 * _e3689))) / _e3731))) - 1f) * 0.5f);
                                                            }
                                                            let _e3744 = phi_10466_;
                                                            let _e3745 = (1f + _e3744);
                                                            let _e3746 = (_e3669 * _e3669);
                                                            if (_e3746 >= 1f) {
                                                                phi_10494_ = 0f;
                                                            } else {
                                                                phi_10494_ = ((sqrt((1f + (((_e2019 * _e2019) * ((_e3699 * _e3699) + (_e3704 * _e3704))) / _e3746))) - 1f) * 0.5f);
                                                            }
                                                            let _e3759 = phi_10494_;
                                                            let _e3766 = (((_e3684 * (_e3707 * _e3716)) + (_e3689 * (_e3708 * _e3716))) + (_e3694 * _e3719));
                                                            if (_e2017 == 0u) {
                                                                let _e4061 = (1f - _e3766);
                                                                if (_e4061 != _e4061) {
                                                                    phi_10997_ = true;
                                                                } else {
                                                                    phi_10997_ = (0f >= _e4061);
                                                                }
                                                                let _e4065 = phi_10997_;
                                                                let _e4067 = pow(select(_e4061, 0f, _e4065), 5f);
                                                                let _e4068 = (1f - _e4067);
                                                                phi_4418_ = vec3<f32>(((_e2015.x * _e4068) + _e4067), ((_e2015.y * _e4068) + _e4067), ((_e2015.z * _e4068) + _e4067));
                                                            } else {
                                                                if (_e2017 == 1u) {
                                                                    let _e4034 = (1f / _e2011);
                                                                    let _e4038 = ((_e4034 * _e4034) * (1f - (_e3766 * _e3766)));
                                                                    if (_e4038 >= 1f) {
                                                                        phi_10958_ = 1f;
                                                                    } else {
                                                                        let _e4041 = sqrt((1f - _e4038));
                                                                        if (_e3766 != _e3766) {
                                                                            phi_10969_ = true;
                                                                        } else {
                                                                            phi_10969_ = (_e4041 <= _e3766);
                                                                        }
                                                                        let _e4045 = phi_10969_;
                                                                        let _e4050 = pow(((1f - _e2011) / (1f + _e2011)), 2f);
                                                                        phi_10958_ = (_e4050 + ((1f - _e4050) * pow((1f - select(_e3766, _e4041, _e4045)), 5f)));
                                                                    }
                                                                    let _e4057 = phi_10958_;
                                                                    phi_4417_ = vec3(_e4057);
                                                                } else {
                                                                    let _e3770 = select(_e3766, 0f, (_e3766 < 0f));
                                                                    let _e3772 = select(_e3770, 1f, (_e3770 > 1f));
                                                                    let _e3774 = (1f - (_e3772 * _e3772));
                                                                    let _e3787 = ((_e2015.x * _e2015.x) - (_e2013.x * _e2013.x));
                                                                    let _e3788 = ((_e2015.y * _e2015.y) - (_e2013.y * _e2013.y));
                                                                    let _e3789 = ((_e2015.z * _e2015.z) - (_e2013.z * _e2013.z));
                                                                    let _e3791 = (_e2015.x * (_e2013.x + _e2013.x));
                                                                    let _e3793 = (_e2015.y * (_e2013.y + _e2013.y));
                                                                    let _e3795 = (_e2015.z * (_e2013.z + _e2013.z));
                                                                    let _e3802 = ((_e3787 * _e3787) + (_e3791 * _e3791));
                                                                    let _e3803 = ((_e3788 * _e3788) + (_e3793 * _e3793));
                                                                    let _e3804 = ((_e3789 * _e3789) + (_e3795 * _e3795));
                                                                    let _e3809 = ((_e3774 * _e3787) / _e3802);
                                                                    let _e3810 = ((_e3774 * _e3788) / _e3803);
                                                                    let _e3811 = ((_e3774 * _e3789) / _e3804);
                                                                    let _e3812 = (1f - _e3809);
                                                                    let _e3813 = (1f - _e3810);
                                                                    let _e3814 = (1f - _e3811);
                                                                    let _e3816 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3809, _e3810, _e3811));
                                                                    let _e3817 = (_e3774 * _e3791);
                                                                    let _e3818 = (_e3774 * _e3793);
                                                                    let _e3819 = (_e3774 * _e3795);
                                                                    let _e3824 = (vec3<f32>(_e3817, _e3818, _e3819) / vec3<f32>(_e3802, _e3803, _e3804));
                                                                    let _e3837 = ((_e3816.x * _e3816.x) + (_e3824.x * _e3824.x));
                                                                    let _e3838 = ((_e3816.y * _e3816.y) + (_e3824.y * _e3824.y));
                                                                    let _e3839 = ((_e3816.z * _e3816.z) + (_e3824.z * _e3824.z));
                                                                    if (_e3837 != _e3837) {
                                                                        phi_10628_ = true;
                                                                    } else {
                                                                        phi_10628_ = (0f >= _e3837);
                                                                    }
                                                                    let _e3843 = phi_10628_;
                                                                    let _e3845 = sqrt(select(_e3837, 0f, _e3843));
                                                                    if (_e3838 != _e3838) {
                                                                        phi_10643_ = true;
                                                                    } else {
                                                                        phi_10643_ = (0f >= _e3838);
                                                                    }
                                                                    let _e3849 = phi_10643_;
                                                                    let _e3851 = sqrt(select(_e3838, 0f, _e3849));
                                                                    if (_e3839 != _e3839) {
                                                                        phi_10658_ = true;
                                                                    } else {
                                                                        phi_10658_ = (0f >= _e3839);
                                                                    }
                                                                    let _e3855 = phi_10658_;
                                                                    let _e3857 = sqrt(select(_e3839, 0f, _e3855));
                                                                    let _e3861 = ((_e3845 + _e3812) * 0.5f);
                                                                    let _e3862 = ((_e3851 + _e3813) * 0.5f);
                                                                    let _e3863 = ((_e3857 + _e3814) * 0.5f);
                                                                    if (_e3861 != _e3861) {
                                                                        phi_10685_ = true;
                                                                    } else {
                                                                        phi_10685_ = (0f >= _e3861);
                                                                    }
                                                                    let _e3867 = phi_10685_;
                                                                    let _e3869 = sqrt(select(_e3861, 0f, _e3867));
                                                                    if (_e3862 != _e3862) {
                                                                        phi_10700_ = true;
                                                                    } else {
                                                                        phi_10700_ = (0f >= _e3862);
                                                                    }
                                                                    let _e3873 = phi_10700_;
                                                                    let _e3875 = sqrt(select(_e3862, 0f, _e3873));
                                                                    if (_e3863 != _e3863) {
                                                                        phi_10715_ = true;
                                                                    } else {
                                                                        phi_10715_ = (0f >= _e3863);
                                                                    }
                                                                    let _e3879 = phi_10715_;
                                                                    let _e3881 = sqrt(select(_e3863, 0f, _e3879));
                                                                    let _e3882 = vec3<f32>(_e3869, _e3875, _e3881);
                                                                    let _e3886 = ((_e3845 - _e3812) * 0.5f);
                                                                    let _e3887 = ((_e3851 - _e3813) * 0.5f);
                                                                    let _e3888 = ((_e3857 - _e3814) * 0.5f);
                                                                    if (_e3886 != _e3886) {
                                                                        phi_10742_ = true;
                                                                    } else {
                                                                        phi_10742_ = (0f >= _e3886);
                                                                    }
                                                                    let _e3892 = phi_10742_;
                                                                    let _e3894 = sqrt(select(_e3886, 0f, _e3892));
                                                                    if (_e3887 != _e3887) {
                                                                        phi_10757_ = true;
                                                                    } else {
                                                                        phi_10757_ = (0f >= _e3887);
                                                                    }
                                                                    let _e3898 = phi_10757_;
                                                                    let _e3900 = sqrt(select(_e3887, 0f, _e3898));
                                                                    if (_e3888 != _e3888) {
                                                                        phi_10772_ = true;
                                                                    } else {
                                                                        phi_10772_ = (0f >= _e3888);
                                                                    }
                                                                    let _e3904 = phi_10772_;
                                                                    let _e3906 = sqrt(select(_e3888, 0f, _e3904));
                                                                    if ((_e3817 / _e3802) < 0f) {
                                                                        phi_4294_ = -(_e3894);
                                                                    } else {
                                                                        phi_4294_ = _e3894;
                                                                    }
                                                                    let _e3910 = phi_4294_;
                                                                    if ((_e3818 / _e3803) < 0f) {
                                                                        phi_4302_ = -(_e3900);
                                                                    } else {
                                                                        phi_4302_ = _e3900;
                                                                    }
                                                                    let _e3914 = phi_4302_;
                                                                    if ((_e3819 / _e3804) < 0f) {
                                                                        phi_4310_ = -(_e3906);
                                                                    } else {
                                                                        phi_4310_ = _e3906;
                                                                    }
                                                                    let _e3918 = phi_4310_;
                                                                    let _e3922 = (_e2013.x * _e3772);
                                                                    let _e3923 = (_e2013.y * _e3772);
                                                                    let _e3924 = (_e2013.z * _e3772);
                                                                    let _e3934 = vec3<f32>(((_e2015.x * _e3869) - (_e2013.x * _e3910)), ((_e2015.y * _e3875) - (_e2013.y * _e3914)), ((_e2015.z * _e3881) - (_e2013.z * _e3918)));
                                                                    let _e3941 = ((_e2015.x * _e3910) + (_e2013.x * _e3869));
                                                                    let _e3942 = ((_e2015.y * _e3914) + (_e2013.y * _e3875));
                                                                    let _e3943 = ((_e2015.z * _e3918) + (_e2013.z * _e3881));
                                                                    let _e3945 = vec3<f32>((_e2015.x * _e3772), (_e2015.y * _e3772), (_e2015.z * _e3772));
                                                                    let _e3946 = (_e3945 - _e3882);
                                                                    let _e3947 = (_e3922 - _e3910);
                                                                    let _e3948 = (_e3923 - _e3914);
                                                                    let _e3949 = (_e3924 - _e3918);
                                                                    let _e3962 = (_e3945 + _e3882);
                                                                    let _e3963 = (_e3922 + _e3910);
                                                                    let _e3964 = (_e3923 + _e3914);
                                                                    let _e3965 = (_e3924 + _e3918);
                                                                    let _e3975 = ((_e3962.x * _e3962.x) + (_e3963 * _e3963));
                                                                    let _e3976 = ((_e3962.y * _e3962.y) + (_e3964 * _e3964));
                                                                    let _e3977 = ((_e3962.z * _e3962.z) + (_e3965 * _e3965));
                                                                    let _e3987 = vec3(_e3772);
                                                                    let _e3988 = (_e3987 - _e3934);
                                                                    let _e3989 = -(vec3<f32>(_e3941, _e3942, _e3943));
                                                                    let _e4005 = (_e3987 + _e3934);
                                                                    let _e4015 = ((_e4005.x * _e4005.x) + (_e3941 * _e3941));
                                                                    let _e4016 = ((_e4005.y * _e4005.y) + (_e3942 * _e3942));
                                                                    let _e4017 = ((_e4005.z * _e4005.z) + (_e3943 * _e3943));
                                                                    phi_4417_ = vec3<f32>((((((_e3946.x * _e3946.x) + (_e3947 * _e3947)) / select(0.000000000000000000000000000001f, _e3975, (_e3975 > 0.000000000000000000000000000001f))) + (((_e3988.x * _e3988.x) + (_e3989.x * _e3989.x)) / select(0.000000000000000000000000000001f, _e4015, (_e4015 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3946.y * _e3946.y) + (_e3948 * _e3948)) / select(0.000000000000000000000000000001f, _e3976, (_e3976 > 0.000000000000000000000000000001f))) + (((_e3988.y * _e3988.y) + (_e3989.y * _e3989.y)) / select(0.000000000000000000000000000001f, _e4016, (_e4016 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3946.z * _e3946.z) + (_e3949 * _e3949)) / select(0.000000000000000000000000000001f, _e3977, (_e3977 > 0.000000000000000000000000000001f))) + (((_e3988.z * _e3988.z) + (_e3989.z * _e3989.z)) / select(0.000000000000000000000000000001f, _e4017, (_e4017 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                }
                                                                let _e4060 = phi_4417_;
                                                                phi_4418_ = _e4060;
                                                            }
                                                            let _e4080 = phi_4418_;
                                                            let _e4082 = (4f * _e3694);
                                                            phi_4430_ = type_59((_e4080 * (((1f / (_e3745 + _e3759)) * _e3730) / _e4082)), ((_e3730 / _e3745) / _e4082));
                                                        }
                                                        let _e4089 = phi_4430_;
                                                        phi_4431_ = _e4089;
                                                        phi_4432_ = _e3706;
                                                    }
                                                    let _e4091 = phi_4431_;
                                                    let _e4093 = phi_4432_;
                                                    if _e4093 {
                                                        phi_4437_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        phi_4437_ = _e4091;
                                                    }
                                                    let _e4095 = phi_4437_;
                                                    let _e4098 = (_e2021 == 1u);
                                                    if _e4098 {
                                                        phi_4465_ = _e4095;
                                                        phi_4466_ = type_59();
                                                    } else {
                                                        if (_e3694 != _e3694) {
                                                            phi_11034_ = true;
                                                        } else {
                                                            phi_11034_ = (0f >= _e3694);
                                                        }
                                                        let _e4102 = phi_11034_;
                                                        let _e4103 = select(_e3694, 0f, _e4102);
                                                        let _e4104 = (1f / _e2011);
                                                        let _e4108 = ((_e4104 * _e4104) * (1f - (_e4103 * _e4103)));
                                                        if (_e4108 >= 1f) {
                                                            phi_11053_ = 1f;
                                                        } else {
                                                            let _e4111 = sqrt((1f - _e4108));
                                                            if (_e4103 != _e4103) {
                                                                phi_11064_ = true;
                                                            } else {
                                                                phi_11064_ = (_e4111 <= _e4103);
                                                            }
                                                            let _e4115 = phi_11064_;
                                                            let _e4120 = pow(((1f - _e2011) / (1f + _e2011)), 2f);
                                                            phi_11053_ = (_e4120 + ((1f - _e4120) * pow((1f - select(_e4103, _e4111, _e4115)), 5f)));
                                                        }
                                                        let _e4127 = phi_11053_;
                                                        if (_e3669 <= 0f) {
                                                            phi_11103_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_11103_ = (_e2009 * (_e3669 * 0.31830987f));
                                                        }
                                                        let _e4132 = phi_11103_;
                                                        let _e4133 = (1f - _e4127);
                                                        phi_4465_ = type_59();
                                                        phi_4466_ = type_59((_e4095.member + vec3<f32>((_e4132.x * _e4133), (_e4132.y * _e4133), (_e4132.z * _e4133))), ((_e4127 * _e4095.member_1) + (_e4133 * _e3675)));
                                                    }
                                                    let _e4147 = phi_4465_;
                                                    let _e4149 = phi_4466_;
                                                    phi_4467_ = _e4147;
                                                    phi_4468_ = _e4149;
                                                    phi_4469_ = _e4098;
                                                }
                                                let _e4157 = phi_4467_;
                                                let _e4159 = phi_4468_;
                                                let _e4161 = phi_4469_;
                                                if _e4161 {
                                                    phi_4473_ = _e4157;
                                                } else {
                                                    phi_4473_ = _e4159;
                                                }
                                                let _e4163 = phi_4473_;
                                                if (_e3648 > 0f) {
                                                    if (_e3648 >= 300000000000000000000000000000000000000f) {
                                                        phi_11142_ = 1f;
                                                    } else {
                                                        let _e4168 = (_e3648 * _e3648);
                                                        let _e4170 = (_e4168 + (_e4163.member_1 * _e4163.member_1));
                                                        if (_e4170 == 0f) {
                                                            phi_11139_ = 0f;
                                                        } else {
                                                            phi_11139_ = (_e4168 / _e4170);
                                                        }
                                                        let _e4174 = phi_11139_;
                                                        phi_11142_ = _e4174;
                                                    }
                                                    let _e4176 = phi_11142_;
                                                    phi_4482_ = (_e4176 / _e3648);
                                                } else {
                                                    phi_4482_ = 1f;
                                                }
                                                let _e4179 = phi_4482_;
                                                if ((((_e4163.member.x * _e4163.member.x) + (_e4163.member.y * _e4163.member.y)) + (_e4163.member.z * _e4163.member.z)) > 0f) {
                                                    let _e4190 = (vec3<f32>(1f, 1f, 1f) / _e3650);
                                                    local = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                    phi_4515_ = _e237;
                                                    phi_4518_ = _e244;
                                                    phi_4520_ = _e241;
                                                    phi_4524_ = type_49(_e3652, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                    loop {
                                                        let _e4192 = phi_4515_;
                                                        let _e4194 = phi_4518_;
                                                        let _e4196 = phi_4520_;
                                                        let _e4198 = phi_4524_;
                                                        local_53 = _e4198;
                                                        if (_e4192 < _e4196) {
                                                            let _e4367 = global_2.member[_e4192];
                                                            if (_e4367.member_6.x == 0u) {
                                                                let _e4456 = (_e4367.member_1.x - _e4367.member.x);
                                                                let _e4457 = (_e4367.member_1.y - _e4367.member.y);
                                                                let _e4458 = (_e4367.member_1.z - _e4367.member.z);
                                                                let _e4463 = (_e4367.member_2.x - _e4367.member.x);
                                                                let _e4464 = (_e4367.member_2.y - _e4367.member.y);
                                                                let _e4465 = (_e4367.member_2.z - _e4367.member.z);
                                                                let _e4468 = ((_e3650.y * _e4465) - (_e4464 * _e3650.z));
                                                                let _e4471 = ((_e3650.z * _e4463) - (_e4465 * _e3650.x));
                                                                let _e4474 = ((_e3650.x * _e4464) - (_e4463 * _e3650.y));
                                                                let _e4479 = (((_e4456 * _e4468) + (_e4457 * _e4471)) + (_e4458 * _e4474));
                                                                if (abs(_e4479) < 0.000000000001f) {
                                                                    phi_4691_ = vec4<f32>();
                                                                    phi_4692_ = true;
                                                                } else {
                                                                    let _e4482 = (1f / _e4479);
                                                                    let _e4483 = (_e1171 - _e4367.member.x);
                                                                    let _e4484 = (_e1172 - _e4367.member.y);
                                                                    let _e4485 = (_e1173 - _e4367.member.z);
                                                                    let _e4491 = ((((_e4483 * _e4468) + (_e4484 * _e4471)) + (_e4485 * _e4474)) * _e4482);
                                                                    if (_e4491 < 0f) {
                                                                        phi_4681_ = vec4<f32>();
                                                                        phi_4682_ = true;
                                                                        phi_4683_ = false;
                                                                    } else {
                                                                        let _e4493 = (_e4491 > 1f);
                                                                        if _e4493 {
                                                                            phi_4679_ = vec4<f32>();
                                                                            phi_4680_ = false;
                                                                        } else {
                                                                            let _e4496 = ((_e4484 * _e4458) - (_e4457 * _e4485));
                                                                            let _e4499 = ((_e4485 * _e4456) - (_e4458 * _e4483));
                                                                            let _e4502 = ((_e4483 * _e4457) - (_e4456 * _e4484));
                                                                            let _e4508 = ((((_e3650.x * _e4496) + (_e3650.y * _e4499)) + (_e3650.z * _e4502)) * _e4482);
                                                                            if (_e4508 < 0f) {
                                                                                phi_4669_ = vec4<f32>();
                                                                                phi_4670_ = true;
                                                                                phi_4671_ = false;
                                                                            } else {
                                                                                let _e4511 = ((_e4491 + _e4508) > 1f);
                                                                                if _e4511 {
                                                                                    phi_4667_ = vec4<f32>();
                                                                                    phi_4668_ = false;
                                                                                } else {
                                                                                    let _e4517 = ((((_e4463 * _e4496) + (_e4464 * _e4499)) + (_e4465 * _e4502)) * _e4482);
                                                                                    if (_e4517 <= 0f) {
                                                                                        phi_4662_ = vec4<f32>();
                                                                                        phi_4663_ = true;
                                                                                    } else {
                                                                                        let _e4519 = (_e4517 >= _e4198.member);
                                                                                        if _e4519 {
                                                                                            phi_4661_ = vec4<f32>();
                                                                                        } else {
                                                                                            phi_4661_ = vec4<f32>(_e4517, _e4491, _e4508, 1f);
                                                                                        }
                                                                                        let _e4522 = phi_4661_;
                                                                                        phi_4662_ = _e4522;
                                                                                        phi_4663_ = _e4519;
                                                                                    }
                                                                                    let _e4524 = phi_4662_;
                                                                                    let _e4526 = phi_4663_;
                                                                                    phi_4667_ = _e4524;
                                                                                    phi_4668_ = _e4526;
                                                                                }
                                                                                let _e4528 = phi_4667_;
                                                                                let _e4530 = phi_4668_;
                                                                                phi_4669_ = _e4528;
                                                                                phi_4670_ = _e4511;
                                                                                phi_4671_ = _e4530;
                                                                            }
                                                                            let _e4532 = phi_4669_;
                                                                            let _e4534 = phi_4670_;
                                                                            let _e4536 = phi_4671_;
                                                                            phi_4679_ = _e4532;
                                                                            phi_4680_ = select(_e4536, true, _e4534);
                                                                        }
                                                                        let _e4539 = phi_4679_;
                                                                        let _e4541 = phi_4680_;
                                                                        phi_4681_ = _e4539;
                                                                        phi_4682_ = _e4493;
                                                                        phi_4683_ = _e4541;
                                                                    }
                                                                    let _e4543 = phi_4681_;
                                                                    let _e4545 = phi_4682_;
                                                                    let _e4547 = phi_4683_;
                                                                    phi_4691_ = _e4543;
                                                                    phi_4692_ = select(_e4547, true, _e4545);
                                                                }
                                                                let _e4550 = phi_4691_;
                                                                let _e4552 = phi_4692_;
                                                                phi_4812_ = _e4550;
                                                                phi_4813_ = _e4552;
                                                            } else {
                                                                if (_e4367.member_6.x == 1u) {
                                                                    let _e4395 = (_e1171 - _e4367.member.x);
                                                                    let _e4396 = (_e1172 - _e4367.member.y);
                                                                    let _e4397 = (_e1173 - _e4367.member.z);
                                                                    let _e4402 = (((_e4395 * _e3650.x) + (_e4396 * _e3650.y)) + (_e4397 * _e3650.z));
                                                                    let _e4406 = (_e4395 - (_e3650.x * _e4402));
                                                                    let _e4407 = (_e4396 - (_e3650.y * _e4402));
                                                                    let _e4408 = (_e4397 - (_e3650.z * _e4402));
                                                                    let _e4417 = ((_e4367.member.w * _e4367.member.w) - (((_e4406 * _e4406) + (_e4407 * _e4407)) + (_e4408 * _e4408)));
                                                                    if (_e4417 < 0f) {
                                                                        phi_4770_ = vec4<f32>();
                                                                        phi_4771_ = true;
                                                                    } else {
                                                                        let _e4419 = sqrt(_e4417);
                                                                        let _e4420 = -(_e4402);
                                                                        let _e4421 = (_e4420 - _e4419);
                                                                        if (_e4421 <= 0f) {
                                                                            phi_4748_ = true;
                                                                        } else {
                                                                            phi_4748_ = (_e4421 >= _e4198.member);
                                                                        }
                                                                        let _e4425 = phi_4748_;
                                                                        if _e4425 {
                                                                            phi_4754_ = (_e4420 + _e4419);
                                                                        } else {
                                                                            phi_4754_ = _e4421;
                                                                        }
                                                                        let _e4428 = phi_4754_;
                                                                        if (_e4428 <= 0f) {
                                                                            phi_4765_ = vec4<f32>();
                                                                            phi_4766_ = true;
                                                                        } else {
                                                                            let _e4430 = (_e4428 >= _e4198.member);
                                                                            if _e4430 {
                                                                                phi_4764_ = vec4<f32>();
                                                                            } else {
                                                                                phi_4764_ = vec4<f32>(_e4428, 0f, 0f, 1f);
                                                                            }
                                                                            let _e4433 = phi_4764_;
                                                                            phi_4765_ = _e4433;
                                                                            phi_4766_ = _e4430;
                                                                        }
                                                                        let _e4435 = phi_4765_;
                                                                        let _e4437 = phi_4766_;
                                                                        phi_4770_ = _e4435;
                                                                        phi_4771_ = _e4437;
                                                                    }
                                                                    let _e4439 = phi_4770_;
                                                                    let _e4441 = phi_4771_;
                                                                    phi_4802_ = _e4439;
                                                                    phi_4803_ = _e4441;
                                                                } else {
                                                                    if (abs(_e3650.y) < 0.000000000001f) {
                                                                        phi_4800_ = vec4<f32>();
                                                                        phi_4801_ = true;
                                                                    } else {
                                                                        let _e4377 = ((_e4367.member.x - _e1172) / _e3650.y);
                                                                        if (_e4377 <= 0f) {
                                                                            phi_4795_ = vec4<f32>();
                                                                            phi_4796_ = true;
                                                                        } else {
                                                                            let _e4379 = (_e4377 >= _e4198.member);
                                                                            if _e4379 {
                                                                                phi_4794_ = vec4<f32>();
                                                                            } else {
                                                                                phi_4794_ = vec4<f32>(_e4377, 0f, 0f, 1f);
                                                                            }
                                                                            let _e4382 = phi_4794_;
                                                                            phi_4795_ = _e4382;
                                                                            phi_4796_ = _e4379;
                                                                        }
                                                                        let _e4384 = phi_4795_;
                                                                        let _e4386 = phi_4796_;
                                                                        phi_4800_ = _e4384;
                                                                        phi_4801_ = _e4386;
                                                                    }
                                                                    let _e4388 = phi_4800_;
                                                                    let _e4390 = phi_4801_;
                                                                    phi_4802_ = _e4388;
                                                                    phi_4803_ = _e4390;
                                                                }
                                                                let _e4443 = phi_4802_;
                                                                let _e4445 = phi_4803_;
                                                                phi_4812_ = _e4443;
                                                                phi_4813_ = select(true, false, select(true, false, _e4445));
                                                            }
                                                            let _e4554 = phi_4812_;
                                                            let _e4556 = phi_4813_;
                                                            let _e4559 = select(vec4<f32>(0f, 0f, 0f, 0f), _e4554, vec4(select(true, false, _e4556)));
                                                            let _e4561 = (_e4559.w > 0f);
                                                            if _e4561 {
                                                                let _e4567 = type_49(_e4559.x, _e4198.member_1, _e4198.member_2, _e4198.member_3);
                                                                let _e4572 = type_49(_e4567.member, _e4192, _e4567.member_2, _e4567.member_3);
                                                                phi_4834_ = _e4572;
                                                                phi_4836_ = _e4572;
                                                            } else {
                                                                phi_4834_ = type_49();
                                                                phi_4836_ = _e4198;
                                                            }
                                                            let _e4574 = phi_4834_;
                                                            let _e4576 = phi_4836_;
                                                            let _e4577 = select(true, false, _e4561);
                                                            if _e4577 {
                                                                phi_4842_ = (_e4192 + 1u);
                                                            } else {
                                                                phi_4842_ = u32();
                                                            }
                                                            let _e4580 = phi_4842_;
                                                            phi_4516_ = _e4580;
                                                            phi_4519_ = _e4194;
                                                            phi_4521_ = _e4196;
                                                            phi_4525_ = _e4576;
                                                            phi_4915_ = _e4574;
                                                            phi_4916_ = _e4577;
                                                            phi_4917_ = false;
                                                        } else {
                                                            let _e4200 = (_e4194 == 0u);
                                                            if _e4200 {
                                                                phi_4911_ = u32();
                                                                phi_4912_ = u32();
                                                                phi_4913_ = u32();
                                                            } else {
                                                                let _e4201 = (_e4194 - 1u);
                                                                let _e4202 = (_e4201 < 100u);
                                                                if _e4202 {
                                                                } else {
                                                                    phi_15013_ = true;
                                                                    phi_7649_ = bool();
                                                                    phi_7648_ = type_49();
                                                                    break;
                                                                }
                                                                let _e4204 = local[_e4201];
                                                                let _e4206 = global_3.member[_e4204];
                                                                let _e4217 = ((_e4206.member.x - _e1171) * _e4190.x);
                                                                let _e4219 = ((_e4206.member.y - _e1172) * _e4190.y);
                                                                let _e4221 = ((_e4206.member.z - _e1173) * _e4190.z);
                                                                let _e4228 = ((_e4206.member_2.x - _e1171) * _e4190.x);
                                                                let _e4229 = ((_e4206.member_2.y - _e1172) * _e4190.y);
                                                                let _e4230 = ((_e4206.member_2.z - _e1173) * _e4190.z);
                                                                if (_e4217 == _e4217) {
                                                                    let _e4232 = (_e4228 == _e4228);
                                                                    if _e4232 {
                                                                        let _e4233 = (_e4217 != _e4217);
                                                                        if _e4233 {
                                                                            phi_11239_ = true;
                                                                        } else {
                                                                            phi_11239_ = (_e4228 <= _e4217);
                                                                        }
                                                                        let _e4236 = phi_11239_;
                                                                        let _e4237 = select(_e4217, _e4228, _e4236);
                                                                        if _e4233 {
                                                                            phi_11269_ = true;
                                                                        } else {
                                                                            phi_11269_ = (_e4228 >= _e4217);
                                                                        }
                                                                        let _e4242 = phi_11269_;
                                                                        let _e4243 = select(_e4217, _e4228, _e4242);
                                                                        phi_11216_ = type_52(select(-300000000000000000000000000000000000000f, _e4237, (_e4237 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4243, (_e4243 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_11216_ = type_52();
                                                                    }
                                                                    let _e4248 = phi_11216_;
                                                                    phi_11219_ = _e4248;
                                                                    phi_11220_ = select(true, false, _e4232);
                                                                } else {
                                                                    phi_11219_ = type_52();
                                                                    phi_11220_ = true;
                                                                }
                                                                let _e4251 = phi_11219_;
                                                                let _e4253 = phi_11220_;
                                                                if _e4253 {
                                                                    phi_11225_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_11225_ = _e4251;
                                                                }
                                                                let _e4255 = phi_11225_;
                                                                if (_e4219 == _e4219) {
                                                                    let _e4259 = (_e4229 == _e4229);
                                                                    if _e4259 {
                                                                        let _e4260 = (_e4219 != _e4219);
                                                                        if _e4260 {
                                                                            phi_11326_ = true;
                                                                        } else {
                                                                            phi_11326_ = (_e4229 <= _e4219);
                                                                        }
                                                                        let _e4263 = phi_11326_;
                                                                        let _e4264 = select(_e4219, _e4229, _e4263);
                                                                        if (_e4255.member != _e4255.member) {
                                                                            phi_11341_ = true;
                                                                        } else {
                                                                            phi_11341_ = (_e4264 >= _e4255.member);
                                                                        }
                                                                        let _e4268 = phi_11341_;
                                                                        if _e4260 {
                                                                            phi_11356_ = true;
                                                                        } else {
                                                                            phi_11356_ = (_e4229 >= _e4219);
                                                                        }
                                                                        let _e4272 = phi_11356_;
                                                                        let _e4273 = select(_e4219, _e4229, _e4272);
                                                                        if (_e4255.member_1 != _e4255.member_1) {
                                                                            phi_11371_ = true;
                                                                        } else {
                                                                            phi_11371_ = (_e4273 <= _e4255.member_1);
                                                                        }
                                                                        let _e4277 = phi_11371_;
                                                                        phi_11303_ = type_52(select(_e4255.member, _e4264, _e4268), select(_e4255.member_1, _e4273, _e4277));
                                                                    } else {
                                                                        phi_11303_ = type_52();
                                                                    }
                                                                    let _e4281 = phi_11303_;
                                                                    phi_11306_ = _e4281;
                                                                    phi_11307_ = select(true, false, _e4259);
                                                                } else {
                                                                    phi_11306_ = type_52();
                                                                    phi_11307_ = true;
                                                                }
                                                                let _e4284 = phi_11306_;
                                                                let _e4286 = phi_11307_;
                                                                if _e4286 {
                                                                    phi_11312_ = _e4255;
                                                                } else {
                                                                    phi_11312_ = _e4284;
                                                                }
                                                                let _e4288 = phi_11312_;
                                                                if (_e4221 == _e4221) {
                                                                    let _e4292 = (_e4230 == _e4230);
                                                                    if _e4292 {
                                                                        let _e4293 = (_e4221 != _e4221);
                                                                        if _e4293 {
                                                                            phi_11413_ = true;
                                                                        } else {
                                                                            phi_11413_ = (_e4230 <= _e4221);
                                                                        }
                                                                        let _e4296 = phi_11413_;
                                                                        let _e4297 = select(_e4221, _e4230, _e4296);
                                                                        if (_e4288.member != _e4288.member) {
                                                                            phi_11428_ = true;
                                                                        } else {
                                                                            phi_11428_ = (_e4297 >= _e4288.member);
                                                                        }
                                                                        let _e4301 = phi_11428_;
                                                                        if _e4293 {
                                                                            phi_11443_ = true;
                                                                        } else {
                                                                            phi_11443_ = (_e4230 >= _e4221);
                                                                        }
                                                                        let _e4305 = phi_11443_;
                                                                        let _e4306 = select(_e4221, _e4230, _e4305);
                                                                        if (_e4288.member_1 != _e4288.member_1) {
                                                                            phi_11458_ = true;
                                                                        } else {
                                                                            phi_11458_ = (_e4306 <= _e4288.member_1);
                                                                        }
                                                                        let _e4310 = phi_11458_;
                                                                        phi_11390_ = type_52(select(_e4288.member, _e4297, _e4301), select(_e4288.member_1, _e4306, _e4310));
                                                                    } else {
                                                                        phi_11390_ = type_52();
                                                                    }
                                                                    let _e4314 = phi_11390_;
                                                                    phi_11393_ = _e4314;
                                                                    phi_11394_ = select(true, false, _e4292);
                                                                } else {
                                                                    phi_11393_ = type_52();
                                                                    phi_11394_ = true;
                                                                }
                                                                let _e4317 = phi_11393_;
                                                                let _e4319 = phi_11394_;
                                                                if _e4319 {
                                                                    phi_11399_ = _e4288;
                                                                } else {
                                                                    phi_11399_ = _e4317;
                                                                }
                                                                let _e4321 = phi_11399_;
                                                                if (_e4321.member_1 < 0f) {
                                                                    phi_11195_ = bool();
                                                                    phi_11196_ = true;
                                                                } else {
                                                                    let _e4325 = (_e4321.member > _e4321.member_1);
                                                                    if _e4325 {
                                                                        phi_11192_ = bool();
                                                                    } else {
                                                                        phi_11192_ = (_e4321.member > _e4198.member);
                                                                    }
                                                                    let _e4328 = phi_11192_;
                                                                    phi_11195_ = _e4328;
                                                                    phi_11196_ = _e4325;
                                                                }
                                                                let _e4330 = phi_11195_;
                                                                let _e4332 = phi_11196_;
                                                                if !(select(_e4330, true, _e4332)) {
                                                                    if (_e4206.member_3 > 0u) {
                                                                        phi_4905_ = _e4206.member_1;
                                                                        phi_4906_ = _e4201;
                                                                        phi_4907_ = (_e4206.member_1 + _e4206.member_3);
                                                                    } else {
                                                                        let _e4337 = (_e4194 + 1u);
                                                                        let _e4338 = (_e4337 <= 100u);
                                                                        if _e4338 {
                                                                            if _e4202 {
                                                                            } else {
                                                                                phi_15013_ = true;
                                                                                phi_7649_ = bool();
                                                                                phi_7648_ = type_49();
                                                                                break;
                                                                            }
                                                                            local[_e4201] = _e4206.member_1;
                                                                            if (_e4194 < 100u) {
                                                                            } else {
                                                                                phi_15013_ = true;
                                                                                phi_7649_ = bool();
                                                                                phi_7648_ = type_49();
                                                                                break;
                                                                            }
                                                                            local[_e4194] = (_e4204 + 1u);
                                                                        }
                                                                        phi_4905_ = _e4192;
                                                                        phi_4906_ = select(_e4201, _e4337, _e4338);
                                                                        phi_4907_ = _e4196;
                                                                    }
                                                                    let _e4347 = phi_4905_;
                                                                    let _e4349 = phi_4906_;
                                                                    let _e4351 = phi_4907_;
                                                                    phi_4908_ = _e4347;
                                                                    phi_4909_ = _e4349;
                                                                    phi_4910_ = _e4351;
                                                                } else {
                                                                    phi_4908_ = _e4192;
                                                                    phi_4909_ = _e4201;
                                                                    phi_4910_ = _e4196;
                                                                }
                                                                let _e4353 = phi_4908_;
                                                                let _e4355 = phi_4909_;
                                                                let _e4357 = phi_4910_;
                                                                phi_4911_ = _e4353;
                                                                phi_4912_ = _e4355;
                                                                phi_4913_ = _e4357;
                                                            }
                                                            let _e4359 = phi_4911_;
                                                            let _e4361 = phi_4912_;
                                                            let _e4363 = phi_4913_;
                                                            phi_4516_ = _e4359;
                                                            phi_4519_ = _e4361;
                                                            phi_4521_ = _e4363;
                                                            phi_4525_ = _e4198;
                                                            phi_4915_ = type_49();
                                                            phi_4916_ = select(true, false, _e4200);
                                                            phi_4917_ = _e4200;
                                                        }
                                                        let _e4582 = phi_4516_;
                                                        let _e4584 = phi_4519_;
                                                        let _e4586 = phi_4521_;
                                                        let _e4588 = phi_4525_;
                                                        let _e4590 = phi_4915_;
                                                        let _e4592 = phi_4916_;
                                                        let _e4594 = phi_4917_;
                                                        continue;
                                                        continuing {
                                                            phi_4515_ = _e4582;
                                                            phi_4518_ = _e4584;
                                                            phi_4520_ = _e4586;
                                                            phi_4524_ = _e4588;
                                                            phi_15013_ = _e2044;
                                                            phi_7649_ = _e4594;
                                                            phi_7648_ = _e4590;
                                                            break if !(_e4592);
                                                        }
                                                    }
                                                    let _e4597 = phi_15013_;
                                                    let _e4599 = phi_7649_;
                                                    let _e4601 = phi_7648_;
                                                    phi_15401_ = _e3644;
                                                    phi_15388_ = _e4597;
                                                    if _e4597 {
                                                        break;
                                                    }
                                                    if _e4599 {
                                                        let _e7028 = local_53;
                                                        phi_5089_ = _e7028;
                                                    } else {
                                                        phi_5089_ = type_49();
                                                    }
                                                    let _e4603 = phi_5089_;
                                                    if select(true, false, _e4599) {
                                                        phi_5094_ = _e4601;
                                                    } else {
                                                        phi_5094_ = _e4603;
                                                    }
                                                    let _e4606 = phi_5094_;
                                                    if (_e4606.member_1 != 4294967295u) {
                                                        phi_5127_ = _e2048;
                                                    } else {
                                                        phi_5127_ = vec3<f32>((_e2048.x + (((_e1034.x * _e4163.member.x) * _e3646.x) * _e4179)), (_e2048.y + (((_e1034.y * _e4163.member.y) * _e3646.y) * _e4179)), (_e2048.z + (((_e1034.z * _e4163.member.z) * _e3646.z) * _e4179)));
                                                    }
                                                    let _e4629 = phi_5127_;
                                                    phi_15391_ = _e4597;
                                                    phi_5128_ = _e4629;
                                                } else {
                                                    phi_15391_ = _e2044;
                                                    phi_5128_ = _e2048;
                                                }
                                                let _e4631 = phi_15391_;
                                                let _e4633 = phi_5128_;
                                                phi_15390_ = _e4631;
                                                phi_5129_ = _e4633;
                                            } else {
                                                phi_15390_ = _e2044;
                                                phi_5129_ = _e2048;
                                            }
                                            let _e4635 = phi_15390_;
                                            let _e4637 = phi_5129_;
                                            phi_15604_ = _e3644;
                                            phi_15389_ = _e4635;
                                            phi_2198_ = _e4637;
                                            phi_2201_ = (_e2050 + 1u);
                                        } else {
                                            phi_15604_ = _e2046;
                                            phi_15389_ = _e2044;
                                            phi_2198_ = vec3<f32>();
                                            phi_2201_ = u32();
                                        }
                                        let _e4640 = phi_15604_;
                                        let _e4642 = phi_15389_;
                                        let _e4644 = phi_2198_;
                                        let _e4646 = phi_2201_;
                                        continue;
                                        continuing {
                                            phi_15093_ = _e4642;
                                            phi_13336_ = _e4640;
                                            phi_2197_ = _e4644;
                                            phi_2200_ = _e4646;
                                            phi_15401_ = _e4640;
                                            phi_15388_ = _e4642;
                                            break if !(_e2052);
                                        }
                                    }
                                    let _e4649 = phi_15401_;
                                    let _e4651 = phi_15388_;
                                    phi_16831_ = _e4651;
                                    phi_7676_ = bool();
                                    phi_7675_ = type_49();
                                    phi_7674_ = bool();
                                    phi_7673_ = bool();
                                    phi_7672_ = bool();
                                    phi_7671_ = bool();
                                    phi_7670_ = bool();
                                    phi_7669_ = bool();
                                    phi_7668_ = bool();
                                    phi_7667_ = vec3<f32>();
                                    phi_7665_ = vec3<f32>();
                                    phi_7661_ = u32();
                                    phi_7659_ = type_14();
                                    phi_7657_ = vec3<f32>();
                                    phi_7656_ = f32();
                                    phi_7655_ = f32();
                                    phi_7654_ = f32();
                                    phi_7652_ = vec3<f32>();
                                    if _e4651 {
                                        break;
                                    }
                                    let _e4652 = (_e228 >= _e208);
                                    if _e4652 {
                                        phi_16929_ = _e4649;
                                        phi_5669_ = vec3<f32>();
                                        phi_5670_ = f32();
                                        phi_5671_ = vec3<f32>();
                                        phi_5672_ = u32();
                                        phi_5673_ = vec3<f32>();
                                        phi_5674_ = false;
                                        phi_5675_ = false;
                                        phi_5676_ = false;
                                        phi_5677_ = false;
                                    } else {
                                        let _e4654 = ((_e4649 * 747796405u) + 2891336453u);
                                        let _e4662 = (((_e4654 >> bitcast<u32>((((_e4654 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4654) * 277803737u);
                                        let _e4665 = ((_e4662 >> bitcast<u32>(22i)) ^ _e4662);
                                        let _e4668 = f32((_e4665 >> bitcast<u32>(8i)));
                                        let _e4671 = ((_e4665 * 747796405u) + 2891336453u);
                                        let _e4679 = (((_e4671 >> bitcast<u32>((((_e4671 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4671) * 277803737u);
                                        let _e4682 = ((_e4679 >> bitcast<u32>(22i)) ^ _e4679);
                                        let _e4685 = f32((_e4682 >> bitcast<u32>(8i)));
                                        let _e4687 = ((_e4682 * 747796405u) + 2891336453u);
                                        let _e4695 = (((_e4687 >> bitcast<u32>((((_e4687 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e4687) * 277803737u);
                                        let _e4698 = ((_e4695 >> bitcast<u32>(22i)) ^ _e4695);
                                        let _e4703 = (_e2021 == 0u);
                                        if _e4703 {
                                            phi_11553_ = vec3<f32>();
                                            phi_11554_ = true;
                                        } else {
                                            if (_e2021 == 2u) {
                                                let _e4712 = (((_e1120.x * _e1146.x) + (_e1120.y * _e1146.y)) + (_e1120.z * _e1146.z));
                                                if (_e4712 != _e4712) {
                                                    phi_11594_ = true;
                                                } else {
                                                    phi_11594_ = (0f >= _e4712);
                                                }
                                                let _e4716 = phi_11594_;
                                                let _e4717 = select(_e4712, 0f, _e4716);
                                                let _e4718 = (1f / _e2011);
                                                let _e4722 = ((_e4718 * _e4718) * (1f - (_e4717 * _e4717)));
                                                if (_e4722 >= 1f) {
                                                    phi_11613_ = 1f;
                                                } else {
                                                    let _e4725 = sqrt((1f - _e4722));
                                                    if (_e4717 != _e4717) {
                                                        phi_11624_ = true;
                                                    } else {
                                                        phi_11624_ = (_e4725 <= _e4717);
                                                    }
                                                    let _e4729 = phi_11624_;
                                                    let _e4734 = pow(((1f - _e2011) / (1f + _e2011)), 2f);
                                                    phi_11613_ = (_e4734 + ((1f - _e4734) * pow((1f - select(_e4717, _e4725, _e4729)), 5f)));
                                                }
                                                let _e4741 = phi_11613_;
                                                phi_11544_ = select(true, false, ((f32((_e4698 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e4741));
                                            } else {
                                                phi_11544_ = true;
                                            }
                                            let _e4745 = phi_11544_;
                                            if _e4745 {
                                                let _e4753 = (((_e1120.x * _e1192) + (_e1120.y * _e1193)) + (_e1120.z * _e1195));
                                                let _e4758 = (((_e1120.x * _e1188) + (_e1120.y * _e1198)) + (_e1120.z * _e1199));
                                                let _e4763 = (((_e1120.x * _e1146.x) + (_e1120.y * _e1146.y)) + (_e1120.z * _e1146.z));
                                                if (_e4763 <= 0f) {
                                                    phi_11778_ = vec3<f32>();
                                                    phi_11779_ = true;
                                                } else {
                                                    let _e4766 = (_e2019 * _e4753);
                                                    let _e4767 = (_e2019 * _e4758);
                                                    let _e4774 = (1f / sqrt((((_e4766 * _e4766) + (_e4767 * _e4767)) + (_e4763 * _e4763))));
                                                    let _e4775 = (_e4766 * _e4774);
                                                    let _e4776 = (_e4767 * _e4774);
                                                    let _e4777 = (_e4763 * _e4774);
                                                    let _e4780 = ((_e4775 * _e4775) + (_e4776 * _e4776));
                                                    if (_e4780 > 0f) {
                                                        let _e4783 = sqrt(_e4780);
                                                        phi_11680_ = vec3<f32>((-(_e4776) / _e4783), (_e4775 / _e4783), 0f);
                                                    } else {
                                                        phi_11680_ = vec3<f32>(1f, 0f, 0f);
                                                    }
                                                    let _e4788 = phi_11680_;
                                                    let _e4801 = sqrt((_e4668 * 0.000000059604645f));
                                                    let _e4802 = (_e4685 * 0.00000037450704f);
                                                    let _e4804 = (_e4801 * cos(_e4802));
                                                    let _e4806 = (0.5f * (1f + _e4777));
                                                    let _e4809 = (1f - (_e4804 * _e4804));
                                                    if (_e4809 != _e4809) {
                                                        phi_11836_ = true;
                                                    } else {
                                                        phi_11836_ = (0f >= _e4809);
                                                    }
                                                    let _e4813 = phi_11836_;
                                                    let _e4820 = (((1f - _e4806) * sqrt(select(_e4809, 0f, _e4813))) + ((_e4806 * _e4801) * sin(_e4802)));
                                                    let _e4831 = (_e4809 - (_e4820 * _e4820));
                                                    if (_e4831 != _e4831) {
                                                        phi_11851_ = true;
                                                    } else {
                                                        phi_11851_ = (0f >= _e4831);
                                                    }
                                                    let _e4835 = phi_11851_;
                                                    let _e4837 = sqrt(select(_e4831, 0f, _e4835));
                                                    let _e4843 = (((_e4788.z * _e4804) + (((_e4775 * _e4788.y) - (_e4788.x * _e4776)) * _e4820)) + (_e4777 * _e4837));
                                                    let _e4844 = (_e2019 * (((_e4788.x * _e4804) + (((_e4776 * _e4788.z) - (_e4788.y * _e4777)) * _e4820)) + (_e4775 * _e4837)));
                                                    let _e4845 = (_e2019 * (((_e4788.y * _e4804) + (((_e4777 * _e4788.x) - (_e4788.z * _e4775)) * _e4820)) + (_e4776 * _e4837)));
                                                    if (_e4843 != _e4843) {
                                                        phi_11866_ = true;
                                                    } else {
                                                        phi_11866_ = (0f >= _e4843);
                                                    }
                                                    let _e4849 = phi_11866_;
                                                    let _e4850 = select(_e4843, 0f, _e4849);
                                                    let _e4857 = (1f / sqrt((((_e4844 * _e4844) + (_e4845 * _e4845)) + (_e4850 * _e4850))));
                                                    let _e4858 = (_e4844 * _e4857);
                                                    let _e4859 = (_e4845 * _e4857);
                                                    let _e4860 = (_e4850 * _e4857);
                                                    let _e4866 = (2f * (((_e4753 * _e4858) + (_e4758 * _e4859)) + (_e4763 * _e4860)));
                                                    let _e4869 = (_e4860 * _e4866);
                                                    let _e4872 = (vec3<f32>((_e4858 * _e4866), (_e4859 * _e4866), _e4869) - vec3<f32>(_e4753, _e4758, _e4763));
                                                    let _e4873 = ((_e4869 - _e4763) <= 0f);
                                                    if _e4873 {
                                                        phi_11775_ = vec3<f32>();
                                                    } else {
                                                        phi_11775_ = vec3<f32>((((_e1192 * _e4872.x) + (_e1188 * _e4872.y)) + (_e1146.x * _e4872.z)), (((_e1193 * _e4872.x) + (_e1198 * _e4872.y)) + (_e1146.y * _e4872.z)), (((_e1195 * _e4872.x) + (_e1199 * _e4872.y)) + (_e1146.z * _e4872.z)));
                                                    }
                                                    let _e4894 = phi_11775_;
                                                    phi_11778_ = _e4894;
                                                    phi_11779_ = _e4873;
                                                }
                                                let _e4896 = phi_11778_;
                                                let _e4898 = phi_11779_;
                                                phi_11549_ = select(_e4896, vec3<f32>(0f, 0f, 0f), vec3(_e4898));
                                            } else {
                                                phi_11549_ = vec3<f32>();
                                            }
                                            let _e4902 = phi_11549_;
                                            phi_11553_ = _e4902;
                                            phi_11554_ = select(true, false, _e4745);
                                        }
                                        let _e4905 = phi_11553_;
                                        let _e4907 = phi_11554_;
                                        if _e4907 {
                                            let _e4908 = (_e4668 * 0.00000037450704f);
                                            let _e4910 = ((_e4685 * 0.00000011920929f) - 1f);
                                            let _e4912 = (1f - (_e4910 * _e4910));
                                            if (_e4912 != _e4912) {
                                                phi_11966_ = true;
                                            } else {
                                                phi_11966_ = (0f >= _e4912);
                                            }
                                            let _e4916 = phi_11966_;
                                            let _e4918 = sqrt(select(_e4912, 0f, _e4916));
                                            let _e4923 = (_e1146.x + (_e4918 * cos(_e4908)));
                                            let _e4924 = (_e1146.y + (_e4918 * sin(_e4908)));
                                            let _e4925 = (_e1146.z + _e4910);
                                            let _e4931 = sqrt((((_e4923 * _e4923) + (_e4924 * _e4924)) + (_e4925 * _e4925)));
                                            if (_e4931 < 0.000001f) {
                                                phi_11940_ = _e1146;
                                            } else {
                                                let _e4933 = (1f / _e4931);
                                                phi_11940_ = vec3<f32>((_e4923 * _e4933), (_e4924 * _e4933), (_e4925 * _e4933));
                                            }
                                            let _e4939 = phi_11940_;
                                            phi_11561_ = _e4939;
                                        } else {
                                            phi_11561_ = _e4905;
                                        }
                                        let _e4941 = phi_11561_;
                                        let _e4949 = (((_e4941.x * _e1146.x) + (_e4941.y * _e1146.y)) + (_e4941.z * _e1146.z));
                                        if (_e4949 != _e4949) {
                                            phi_11981_ = true;
                                        } else {
                                            phi_11981_ = (0f >= _e4949);
                                        }
                                        let _e4953 = phi_11981_;
                                        let _e4955 = (select(_e4949, 0f, _e4953) * 0.31830987f);
                                        if _e4703 {
                                            if (_e4949 <= 0f) {
                                                phi_12778_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_12778_ = (_e2015 * (_e4949 * 0.31830987f));
                                            }
                                            let _e5433 = phi_12778_;
                                            phi_5519_ = type_59(_e5433, _e4955);
                                            phi_5520_ = type_59();
                                            phi_5521_ = true;
                                        } else {
                                            let _e4963 = (((_e1120.x * _e1192) + (_e1120.y * _e1193)) + (_e1120.z * _e1195));
                                            let _e4968 = (((_e1120.x * _e1188) + (_e1120.y * _e1198)) + (_e1120.z * _e1199));
                                            let _e4973 = (((_e1120.x * _e1146.x) + (_e1120.y * _e1146.y)) + (_e1120.z * _e1146.z));
                                            let _e4978 = (((_e4941.x * _e1192) + (_e4941.y * _e1193)) + (_e4941.z * _e1195));
                                            let _e4983 = (((_e4941.x * _e1188) + (_e4941.y * _e1198)) + (_e4941.z * _e1199));
                                            if (_e4973 <= 0f) {
                                                phi_5483_ = type_59();
                                                phi_5484_ = true;
                                            } else {
                                                let _e4985 = (_e4949 <= 0f);
                                                if _e4985 {
                                                    phi_5482_ = type_59();
                                                } else {
                                                    let _e4986 = (_e4963 + _e4978);
                                                    let _e4987 = (_e4968 + _e4983);
                                                    let _e4988 = (_e4973 + _e4949);
                                                    let _e4995 = (1f / sqrt((((_e4986 * _e4986) + (_e4987 * _e4987)) + (_e4988 * _e4988))));
                                                    let _e4998 = (_e4988 * _e4995);
                                                    if (_e4998 <= 0f) {
                                                        phi_12091_ = 0f;
                                                    } else {
                                                        let _e5000 = (_e2019 * _e2019);
                                                        let _e5004 = ((((_e5000 - 1f) * _e4998) * _e4998) + 1f);
                                                        phi_12091_ = (_e5000 / ((3.1415927f * _e5004) * _e5004));
                                                    }
                                                    let _e5009 = phi_12091_;
                                                    let _e5010 = (_e4973 * _e4973);
                                                    if (_e5010 >= 1f) {
                                                        phi_12119_ = 0f;
                                                    } else {
                                                        phi_12119_ = ((sqrt((1f + (((_e2019 * _e2019) * ((_e4963 * _e4963) + (_e4968 * _e4968))) / _e5010))) - 1f) * 0.5f);
                                                    }
                                                    let _e5023 = phi_12119_;
                                                    let _e5024 = (1f + _e5023);
                                                    let _e5025 = (_e4949 * _e4949);
                                                    if (_e5025 >= 1f) {
                                                        phi_12147_ = 0f;
                                                    } else {
                                                        phi_12147_ = ((sqrt((1f + (((_e2019 * _e2019) * ((_e4978 * _e4978) + (_e4983 * _e4983))) / _e5025))) - 1f) * 0.5f);
                                                    }
                                                    let _e5038 = phi_12147_;
                                                    let _e5045 = (((_e4963 * (_e4986 * _e4995)) + (_e4968 * (_e4987 * _e4995))) + (_e4973 * _e4998));
                                                    if (_e2017 == 0u) {
                                                        let _e5340 = (1f - _e5045);
                                                        if (_e5340 != _e5340) {
                                                            phi_12650_ = true;
                                                        } else {
                                                            phi_12650_ = (0f >= _e5340);
                                                        }
                                                        let _e5344 = phi_12650_;
                                                        let _e5346 = pow(select(_e5340, 0f, _e5344), 5f);
                                                        let _e5347 = (1f - _e5346);
                                                        phi_5470_ = vec3<f32>(((_e2015.x * _e5347) + _e5346), ((_e2015.y * _e5347) + _e5346), ((_e2015.z * _e5347) + _e5346));
                                                    } else {
                                                        if (_e2017 == 1u) {
                                                            let _e5313 = (1f / _e2011);
                                                            let _e5317 = ((_e5313 * _e5313) * (1f - (_e5045 * _e5045)));
                                                            if (_e5317 >= 1f) {
                                                                phi_12611_ = 1f;
                                                            } else {
                                                                let _e5320 = sqrt((1f - _e5317));
                                                                if (_e5045 != _e5045) {
                                                                    phi_12622_ = true;
                                                                } else {
                                                                    phi_12622_ = (_e5320 <= _e5045);
                                                                }
                                                                let _e5324 = phi_12622_;
                                                                let _e5329 = pow(((1f - _e2011) / (1f + _e2011)), 2f);
                                                                phi_12611_ = (_e5329 + ((1f - _e5329) * pow((1f - select(_e5045, _e5320, _e5324)), 5f)));
                                                            }
                                                            let _e5336 = phi_12611_;
                                                            phi_5469_ = vec3(_e5336);
                                                        } else {
                                                            let _e5049 = select(_e5045, 0f, (_e5045 < 0f));
                                                            let _e5051 = select(_e5049, 1f, (_e5049 > 1f));
                                                            let _e5053 = (1f - (_e5051 * _e5051));
                                                            let _e5066 = ((_e2015.x * _e2015.x) - (_e2013.x * _e2013.x));
                                                            let _e5067 = ((_e2015.y * _e2015.y) - (_e2013.y * _e2013.y));
                                                            let _e5068 = ((_e2015.z * _e2015.z) - (_e2013.z * _e2013.z));
                                                            let _e5070 = (_e2015.x * (_e2013.x + _e2013.x));
                                                            let _e5072 = (_e2015.y * (_e2013.y + _e2013.y));
                                                            let _e5074 = (_e2015.z * (_e2013.z + _e2013.z));
                                                            let _e5081 = ((_e5066 * _e5066) + (_e5070 * _e5070));
                                                            let _e5082 = ((_e5067 * _e5067) + (_e5072 * _e5072));
                                                            let _e5083 = ((_e5068 * _e5068) + (_e5074 * _e5074));
                                                            let _e5088 = ((_e5053 * _e5066) / _e5081);
                                                            let _e5089 = ((_e5053 * _e5067) / _e5082);
                                                            let _e5090 = ((_e5053 * _e5068) / _e5083);
                                                            let _e5091 = (1f - _e5088);
                                                            let _e5092 = (1f - _e5089);
                                                            let _e5093 = (1f - _e5090);
                                                            let _e5095 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5088, _e5089, _e5090));
                                                            let _e5096 = (_e5053 * _e5070);
                                                            let _e5097 = (_e5053 * _e5072);
                                                            let _e5098 = (_e5053 * _e5074);
                                                            let _e5103 = (vec3<f32>(_e5096, _e5097, _e5098) / vec3<f32>(_e5081, _e5082, _e5083));
                                                            let _e5116 = ((_e5095.x * _e5095.x) + (_e5103.x * _e5103.x));
                                                            let _e5117 = ((_e5095.y * _e5095.y) + (_e5103.y * _e5103.y));
                                                            let _e5118 = ((_e5095.z * _e5095.z) + (_e5103.z * _e5103.z));
                                                            if (_e5116 != _e5116) {
                                                                phi_12281_ = true;
                                                            } else {
                                                                phi_12281_ = (0f >= _e5116);
                                                            }
                                                            let _e5122 = phi_12281_;
                                                            let _e5124 = sqrt(select(_e5116, 0f, _e5122));
                                                            if (_e5117 != _e5117) {
                                                                phi_12296_ = true;
                                                            } else {
                                                                phi_12296_ = (0f >= _e5117);
                                                            }
                                                            let _e5128 = phi_12296_;
                                                            let _e5130 = sqrt(select(_e5117, 0f, _e5128));
                                                            if (_e5118 != _e5118) {
                                                                phi_12311_ = true;
                                                            } else {
                                                                phi_12311_ = (0f >= _e5118);
                                                            }
                                                            let _e5134 = phi_12311_;
                                                            let _e5136 = sqrt(select(_e5118, 0f, _e5134));
                                                            let _e5140 = ((_e5124 + _e5091) * 0.5f);
                                                            let _e5141 = ((_e5130 + _e5092) * 0.5f);
                                                            let _e5142 = ((_e5136 + _e5093) * 0.5f);
                                                            if (_e5140 != _e5140) {
                                                                phi_12338_ = true;
                                                            } else {
                                                                phi_12338_ = (0f >= _e5140);
                                                            }
                                                            let _e5146 = phi_12338_;
                                                            let _e5148 = sqrt(select(_e5140, 0f, _e5146));
                                                            if (_e5141 != _e5141) {
                                                                phi_12353_ = true;
                                                            } else {
                                                                phi_12353_ = (0f >= _e5141);
                                                            }
                                                            let _e5152 = phi_12353_;
                                                            let _e5154 = sqrt(select(_e5141, 0f, _e5152));
                                                            if (_e5142 != _e5142) {
                                                                phi_12368_ = true;
                                                            } else {
                                                                phi_12368_ = (0f >= _e5142);
                                                            }
                                                            let _e5158 = phi_12368_;
                                                            let _e5160 = sqrt(select(_e5142, 0f, _e5158));
                                                            let _e5161 = vec3<f32>(_e5148, _e5154, _e5160);
                                                            let _e5165 = ((_e5124 - _e5091) * 0.5f);
                                                            let _e5166 = ((_e5130 - _e5092) * 0.5f);
                                                            let _e5167 = ((_e5136 - _e5093) * 0.5f);
                                                            if (_e5165 != _e5165) {
                                                                phi_12395_ = true;
                                                            } else {
                                                                phi_12395_ = (0f >= _e5165);
                                                            }
                                                            let _e5171 = phi_12395_;
                                                            let _e5173 = sqrt(select(_e5165, 0f, _e5171));
                                                            if (_e5166 != _e5166) {
                                                                phi_12410_ = true;
                                                            } else {
                                                                phi_12410_ = (0f >= _e5166);
                                                            }
                                                            let _e5177 = phi_12410_;
                                                            let _e5179 = sqrt(select(_e5166, 0f, _e5177));
                                                            if (_e5167 != _e5167) {
                                                                phi_12425_ = true;
                                                            } else {
                                                                phi_12425_ = (0f >= _e5167);
                                                            }
                                                            let _e5183 = phi_12425_;
                                                            let _e5185 = sqrt(select(_e5167, 0f, _e5183));
                                                            if ((_e5096 / _e5081) < 0f) {
                                                                phi_5346_ = -(_e5173);
                                                            } else {
                                                                phi_5346_ = _e5173;
                                                            }
                                                            let _e5189 = phi_5346_;
                                                            if ((_e5097 / _e5082) < 0f) {
                                                                phi_5354_ = -(_e5179);
                                                            } else {
                                                                phi_5354_ = _e5179;
                                                            }
                                                            let _e5193 = phi_5354_;
                                                            if ((_e5098 / _e5083) < 0f) {
                                                                phi_5362_ = -(_e5185);
                                                            } else {
                                                                phi_5362_ = _e5185;
                                                            }
                                                            let _e5197 = phi_5362_;
                                                            let _e5201 = (_e2013.x * _e5051);
                                                            let _e5202 = (_e2013.y * _e5051);
                                                            let _e5203 = (_e2013.z * _e5051);
                                                            let _e5213 = vec3<f32>(((_e2015.x * _e5148) - (_e2013.x * _e5189)), ((_e2015.y * _e5154) - (_e2013.y * _e5193)), ((_e2015.z * _e5160) - (_e2013.z * _e5197)));
                                                            let _e5220 = ((_e2015.x * _e5189) + (_e2013.x * _e5148));
                                                            let _e5221 = ((_e2015.y * _e5193) + (_e2013.y * _e5154));
                                                            let _e5222 = ((_e2015.z * _e5197) + (_e2013.z * _e5160));
                                                            let _e5224 = vec3<f32>((_e2015.x * _e5051), (_e2015.y * _e5051), (_e2015.z * _e5051));
                                                            let _e5225 = (_e5224 - _e5161);
                                                            let _e5226 = (_e5201 - _e5189);
                                                            let _e5227 = (_e5202 - _e5193);
                                                            let _e5228 = (_e5203 - _e5197);
                                                            let _e5241 = (_e5224 + _e5161);
                                                            let _e5242 = (_e5201 + _e5189);
                                                            let _e5243 = (_e5202 + _e5193);
                                                            let _e5244 = (_e5203 + _e5197);
                                                            let _e5254 = ((_e5241.x * _e5241.x) + (_e5242 * _e5242));
                                                            let _e5255 = ((_e5241.y * _e5241.y) + (_e5243 * _e5243));
                                                            let _e5256 = ((_e5241.z * _e5241.z) + (_e5244 * _e5244));
                                                            let _e5266 = vec3(_e5051);
                                                            let _e5267 = (_e5266 - _e5213);
                                                            let _e5268 = -(vec3<f32>(_e5220, _e5221, _e5222));
                                                            let _e5284 = (_e5266 + _e5213);
                                                            let _e5294 = ((_e5284.x * _e5284.x) + (_e5220 * _e5220));
                                                            let _e5295 = ((_e5284.y * _e5284.y) + (_e5221 * _e5221));
                                                            let _e5296 = ((_e5284.z * _e5284.z) + (_e5222 * _e5222));
                                                            phi_5469_ = vec3<f32>((((((_e5225.x * _e5225.x) + (_e5226 * _e5226)) / select(0.000000000000000000000000000001f, _e5254, (_e5254 > 0.000000000000000000000000000001f))) + (((_e5267.x * _e5267.x) + (_e5268.x * _e5268.x)) / select(0.000000000000000000000000000001f, _e5294, (_e5294 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5225.y * _e5225.y) + (_e5227 * _e5227)) / select(0.000000000000000000000000000001f, _e5255, (_e5255 > 0.000000000000000000000000000001f))) + (((_e5267.y * _e5267.y) + (_e5268.y * _e5268.y)) / select(0.000000000000000000000000000001f, _e5295, (_e5295 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5225.z * _e5225.z) + (_e5228 * _e5228)) / select(0.000000000000000000000000000001f, _e5256, (_e5256 > 0.000000000000000000000000000001f))) + (((_e5267.z * _e5267.z) + (_e5268.z * _e5268.z)) / select(0.000000000000000000000000000001f, _e5296, (_e5296 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e5339 = phi_5469_;
                                                        phi_5470_ = _e5339;
                                                    }
                                                    let _e5359 = phi_5470_;
                                                    let _e5361 = (4f * _e4973);
                                                    phi_5482_ = type_59((_e5359 * (((1f / (_e5024 + _e5038)) * _e5009) / _e5361)), ((_e5009 / _e5024) / _e5361));
                                                }
                                                let _e5368 = phi_5482_;
                                                phi_5483_ = _e5368;
                                                phi_5484_ = _e4985;
                                            }
                                            let _e5370 = phi_5483_;
                                            let _e5372 = phi_5484_;
                                            if _e5372 {
                                                phi_5489_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                            } else {
                                                phi_5489_ = _e5370;
                                            }
                                            let _e5374 = phi_5489_;
                                            let _e5377 = (_e2021 == 1u);
                                            if _e5377 {
                                                phi_5517_ = _e5374;
                                                phi_5518_ = type_59();
                                            } else {
                                                if (_e4973 != _e4973) {
                                                    phi_12687_ = true;
                                                } else {
                                                    phi_12687_ = (0f >= _e4973);
                                                }
                                                let _e5381 = phi_12687_;
                                                let _e5382 = select(_e4973, 0f, _e5381);
                                                let _e5383 = (1f / _e2011);
                                                let _e5387 = ((_e5383 * _e5383) * (1f - (_e5382 * _e5382)));
                                                if (_e5387 >= 1f) {
                                                    phi_12706_ = 1f;
                                                } else {
                                                    let _e5390 = sqrt((1f - _e5387));
                                                    if (_e5382 != _e5382) {
                                                        phi_12717_ = true;
                                                    } else {
                                                        phi_12717_ = (_e5390 <= _e5382);
                                                    }
                                                    let _e5394 = phi_12717_;
                                                    let _e5399 = pow(((1f - _e2011) / (1f + _e2011)), 2f);
                                                    phi_12706_ = (_e5399 + ((1f - _e5399) * pow((1f - select(_e5382, _e5390, _e5394)), 5f)));
                                                }
                                                let _e5406 = phi_12706_;
                                                if (_e4949 <= 0f) {
                                                    phi_12756_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_12756_ = (_e2009 * (_e4949 * 0.31830987f));
                                                }
                                                let _e5411 = phi_12756_;
                                                let _e5412 = (1f - _e5406);
                                                phi_5517_ = type_59();
                                                phi_5518_ = type_59((_e5374.member + vec3<f32>((_e5411.x * _e5412), (_e5411.y * _e5412), (_e5411.z * _e5412))), ((_e5406 * _e5374.member_1) + (_e5412 * _e4955)));
                                            }
                                            let _e5426 = phi_5517_;
                                            let _e5428 = phi_5518_;
                                            phi_5519_ = _e5426;
                                            phi_5520_ = _e5428;
                                            phi_5521_ = _e5377;
                                        }
                                        let _e5436 = phi_5519_;
                                        let _e5438 = phi_5520_;
                                        let _e5440 = phi_5521_;
                                        if _e5440 {
                                            phi_5525_ = _e5436;
                                        } else {
                                            phi_5525_ = _e5438;
                                        }
                                        let _e5442 = phi_5525_;
                                        let _e5450 = ((((_e4941.x * _e4941.x) + (_e4941.y * _e4941.y)) + (_e4941.z * _e4941.z)) == 0f);
                                        if _e5450 {
                                            phi_16930_ = _e4698;
                                            phi_5663_ = vec3<f32>();
                                            phi_5664_ = u32();
                                            phi_5665_ = vec3<f32>();
                                            phi_5666_ = false;
                                            phi_5667_ = false;
                                            phi_5668_ = false;
                                        } else {
                                            let _e5451 = (_e5442.member_1 <= 0f);
                                            if _e5451 {
                                                phi_16931_ = _e4698;
                                                phi_5658_ = vec3<f32>();
                                                phi_5659_ = u32();
                                                phi_5660_ = vec3<f32>();
                                                phi_5661_ = false;
                                                phi_5662_ = false;
                                            } else {
                                                let _e5453 = (_e5442.member.x / _e5442.member_1);
                                                let _e5455 = (_e5442.member.y / _e5442.member_1);
                                                let _e5457 = (_e5442.member.z / _e5442.member_1);
                                                let _e5466 = global_1.member.member_4[1u];
                                                if (sqrt((((_e5453 * _e5453) + (_e5455 * _e5455)) + (_e5457 * _e5457))) <= _e5466) {
                                                    phi_16933_ = _e4698;
                                                    phi_5612_ = _e1034;
                                                    phi_5613_ = false;
                                                } else {
                                                    let _e5470 = global_1.member.member_7[1u];
                                                    if (_e5470 == 1u) {
                                                        let _e5475 = global_1.member.member_4[3u];
                                                        if (f32(_e228) >= _e5475) {
                                                            let _e5478 = select(_e5455, _e5453, (_e5453 > _e5455));
                                                            let _e5480 = select(_e5457, _e5478, (_e5478 > _e5457));
                                                            if (_e5480 != _e5480) {
                                                                phi_12803_ = true;
                                                            } else {
                                                                phi_12803_ = (1f <= _e5480);
                                                            }
                                                            let _e5484 = phi_12803_;
                                                            let _e5485 = select(_e5480, 1f, _e5484);
                                                            let _e5487 = ((_e4698 * 747796405u) + 2891336453u);
                                                            let _e5495 = (((_e5487 >> bitcast<u32>((((_e5487 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5487) * 277803737u);
                                                            let _e5498 = ((_e5495 >> bitcast<u32>(22i)) ^ _e5495);
                                                            let _e5503 = ((f32((_e5498 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5485);
                                                            if _e5503 {
                                                                phi_5601_ = vec3<f32>();
                                                            } else {
                                                                phi_5601_ = vec3<f32>((_e1034.x / _e5485), (_e1034.y / _e5485), (_e1034.z / _e5485));
                                                            }
                                                            let _e5512 = phi_5601_;
                                                            phi_16936_ = _e5498;
                                                            phi_5603_ = _e5512;
                                                            phi_5604_ = select(true, false, _e5503);
                                                        } else {
                                                            phi_16936_ = _e4698;
                                                            phi_5603_ = _e1034;
                                                            phi_5604_ = true;
                                                        }
                                                        let _e5515 = phi_16936_;
                                                        let _e5517 = phi_5603_;
                                                        let _e5519 = phi_5604_;
                                                        phi_16935_ = _e5515;
                                                        phi_5605_ = _e5517;
                                                        phi_5606_ = _e5519;
                                                    } else {
                                                        phi_16935_ = _e4698;
                                                        phi_5605_ = _e1034;
                                                        phi_5606_ = true;
                                                    }
                                                    let _e5521 = phi_16935_;
                                                    let _e5523 = phi_5605_;
                                                    let _e5525 = phi_5606_;
                                                    phi_16933_ = _e5521;
                                                    phi_5612_ = select(_e1034, _e5523, vec3(_e5525));
                                                    phi_5613_ = select(false, true, _e5525);
                                                }
                                                let _e5530 = phi_16933_;
                                                let _e5532 = phi_5612_;
                                                let _e5534 = phi_5613_;
                                                if _e5534 {
                                                    if ((((_e4941.x * _e1133.x) + (_e4941.y * _e1133.y)) + (_e4941.z * _e1133.z)) < 0f) {
                                                        phi_5642_ = -(_e1133);
                                                    } else {
                                                        phi_5642_ = _e1133;
                                                    }
                                                    let _e5550 = phi_5642_;
                                                    phi_5654_ = vec3<f32>((_e5532.x * _e5453), (_e5532.y * _e5455), (_e5532.z * _e5457));
                                                    phi_5655_ = (_e228 + 1u);
                                                    phi_5656_ = vec3<f32>((_e1043 + (_e5550.x * _e1164)), (_e1045 + (_e5550.y * _e1164)), (_e1047 + (_e5550.z * _e1164)));
                                                } else {
                                                    phi_5654_ = vec3<f32>();
                                                    phi_5655_ = u32();
                                                    phi_5656_ = vec3<f32>();
                                                }
                                                let _e5563 = phi_5654_;
                                                let _e5565 = phi_5655_;
                                                let _e5567 = phi_5656_;
                                                phi_16931_ = _e5530;
                                                phi_5658_ = _e5563;
                                                phi_5659_ = _e5565;
                                                phi_5660_ = _e5567;
                                                phi_5661_ = _e5534;
                                                phi_5662_ = select(true, false, _e5534);
                                            }
                                            let _e5570 = phi_16931_;
                                            let _e5572 = phi_5658_;
                                            let _e5574 = phi_5659_;
                                            let _e5576 = phi_5660_;
                                            let _e5578 = phi_5661_;
                                            let _e5580 = phi_5662_;
                                            phi_16930_ = _e5570;
                                            phi_5663_ = _e5572;
                                            phi_5664_ = _e5574;
                                            phi_5665_ = _e5576;
                                            phi_5666_ = _e5578;
                                            phi_5667_ = _e5451;
                                            phi_5668_ = _e5580;
                                        }
                                        let _e5582 = phi_16930_;
                                        let _e5584 = phi_5663_;
                                        let _e5586 = phi_5664_;
                                        let _e5588 = phi_5665_;
                                        let _e5590 = phi_5666_;
                                        let _e5592 = phi_5667_;
                                        let _e5594 = phi_5668_;
                                        phi_16929_ = _e5582;
                                        phi_5669_ = _e5584;
                                        phi_5670_ = _e5442.member_1;
                                        phi_5671_ = _e4941;
                                        phi_5672_ = _e5586;
                                        phi_5673_ = _e5588;
                                        phi_5674_ = _e5590;
                                        phi_5675_ = _e5450;
                                        phi_5676_ = _e5592;
                                        phi_5677_ = _e5594;
                                    }
                                    let _e5596 = phi_16929_;
                                    let _e5598 = phi_5669_;
                                    let _e5600 = phi_5670_;
                                    let _e5602 = phi_5671_;
                                    let _e5604 = phi_5672_;
                                    let _e5606 = phi_5673_;
                                    let _e5608 = phi_5674_;
                                    let _e5610 = phi_5675_;
                                    let _e5612 = phi_5676_;
                                    let _e5614 = phi_5677_;
                                    phi_16928_ = _e5596;
                                    phi_16834_ = _e4651;
                                    let _e7136 = local_54;
                                    phi_5678_ = _e7136;
                                    let _e7139 = local_55;
                                    phi_5681_ = _e7139;
                                    phi_5682_ = _e5598;
                                    phi_5683_ = _e5600;
                                    phi_5684_ = _e5602;
                                    phi_5685_ = _e5604;
                                    phi_5688_ = _e5606;
                                    phi_5690_ = _e5608;
                                    phi_5691_ = _e4652;
                                    phi_5692_ = _e5610;
                                    phi_5693_ = _e5612;
                                    phi_5695_ = _e5614;
                                } else {
                                    phi_16928_ = _e2023;
                                    phi_16834_ = _e671;
                                    phi_5678_ = vec3<f32>();
                                    phi_5681_ = _e1032;
                                    phi_5682_ = _e2025;
                                    phi_5683_ = _e224;
                                    phi_5684_ = _e2027;
                                    phi_5685_ = _e2029;
                                    phi_5688_ = _e2031;
                                    phi_5690_ = _e2035;
                                    phi_5691_ = false;
                                    phi_5692_ = false;
                                    phi_5693_ = false;
                                    phi_5695_ = false;
                                }
                                let _e5616 = phi_16928_;
                                let _e5618 = phi_16834_;
                                let _e5620 = phi_5678_;
                                let _e5622 = phi_5681_;
                                let _e5624 = phi_5682_;
                                let _e5626 = phi_5683_;
                                let _e5628 = phi_5684_;
                                let _e5630 = phi_5685_;
                                let _e5632 = phi_5688_;
                                let _e5634 = phi_5690_;
                                let _e5636 = phi_5691_;
                                let _e5638 = phi_5692_;
                                let _e5640 = phi_5693_;
                                let _e5642 = phi_5695_;
                                phi_16927_ = _e5616;
                                phi_16833_ = _e5618;
                                phi_5697_ = _e5620;
                                phi_5700_ = _e5622;
                                phi_5701_ = _e5624;
                                phi_5702_ = _e5626;
                                phi_5703_ = _e5628;
                                phi_5704_ = _e5630;
                                phi_5705_ = select(_e230, true, _e2033);
                                phi_5706_ = select(true, false, _e2033);
                                phi_5707_ = _e5632;
                                phi_5709_ = _e5634;
                                phi_5710_ = _e5636;
                                phi_5711_ = _e5638;
                                phi_5712_ = _e5640;
                                phi_5713_ = select(_e2037, false, _e2033);
                                phi_5714_ = _e5642;
                                phi_5715_ = select(_e2039, false, _e2033);
                            }
                            let _e5648 = phi_16927_;
                            let _e5650 = phi_16833_;
                            let _e5652 = phi_5697_;
                            let _e5654 = phi_5700_;
                            let _e5656 = phi_5701_;
                            let _e5658 = phi_5702_;
                            let _e5660 = phi_5703_;
                            let _e5662 = phi_5704_;
                            let _e5664 = phi_5705_;
                            let _e5666 = phi_5706_;
                            let _e5668 = phi_5707_;
                            let _e5670 = phi_5709_;
                            let _e5672 = phi_5710_;
                            let _e5674 = phi_5711_;
                            let _e5676 = phi_5712_;
                            let _e5678 = phi_5713_;
                            let _e5680 = phi_5714_;
                            let _e5682 = phi_5715_;
                            phi_16926_ = _e5648;
                            phi_16832_ = _e5650;
                            phi_5716_ = _e1032;
                            phi_5717_ = _e1034;
                            phi_5718_ = _e1043;
                            phi_5719_ = _e1045;
                            phi_5720_ = _e1047;
                            phi_5721_ = (_e234 + vec3<f32>(_e1037, _e1039, _e1041));
                            phi_5722_ = _e1051;
                            let _e7179 = local_56;
                            phi_5723_ = _e7179;
                            phi_5724_ = _e5652;
                            phi_335_ = _e5654;
                            phi_338_ = _e5656;
                            phi_340_ = _e5658;
                            phi_342_ = _e5660;
                            phi_344_ = _e5662;
                            phi_346_ = _e5664;
                            phi_348_ = _e5666;
                            phi_350_ = _e5668;
                            phi_5730_ = _e5670;
                            phi_5731_ = _e1116;
                            phi_5732_ = _e5672;
                            phi_5733_ = _e5674;
                            phi_5734_ = _e5676;
                            phi_5735_ = _e5678;
                            phi_5736_ = _e5680;
                            phi_5737_ = _e5682;
                        }
                        let _e5684 = phi_16926_;
                        let _e5686 = phi_16832_;
                        let _e5688 = phi_5716_;
                        let _e5690 = phi_5717_;
                        let _e5692 = phi_5718_;
                        let _e5694 = phi_5719_;
                        let _e5696 = phi_5720_;
                        let _e5698 = phi_5721_;
                        let _e5700 = phi_5722_;
                        let _e5702 = phi_5723_;
                        let _e5704 = phi_5724_;
                        let _e5706 = phi_335_;
                        let _e5708 = phi_338_;
                        let _e5710 = phi_340_;
                        let _e5712 = phi_342_;
                        let _e5714 = phi_344_;
                        let _e5716 = phi_346_;
                        let _e5718 = phi_348_;
                        let _e5720 = phi_350_;
                        let _e5722 = phi_5730_;
                        let _e5724 = phi_5731_;
                        let _e5726 = phi_5732_;
                        let _e5728 = phi_5733_;
                        let _e5730 = phi_5734_;
                        let _e5732 = phi_5735_;
                        let _e5734 = phi_5736_;
                        let _e5736 = phi_5737_;
                        continue;
                        continuing {
                            phi_13166_ = _e5684;
                            phi_13089_ = _e5686;
                            phi_334_ = _e5706;
                            phi_337_ = _e5708;
                            phi_339_ = _e5710;
                            phi_341_ = _e5712;
                            phi_343_ = _e5714;
                            phi_345_ = _e5716;
                            phi_347_ = _e5718;
                            phi_349_ = _e5720;
                            phi_16831_ = _e5686;
                            phi_7676_ = _e839;
                            phi_7675_ = _e837;
                            phi_7674_ = _e5736;
                            phi_7673_ = _e5734;
                            phi_7672_ = _e5732;
                            phi_7671_ = _e5730;
                            phi_7670_ = _e5728;
                            phi_7669_ = _e5726;
                            phi_7668_ = _e5724;
                            phi_7667_ = _e5688;
                            phi_7665_ = _e5704;
                            phi_7661_ = _e5702;
                            phi_7659_ = _e5700;
                            phi_7657_ = _e5698;
                            phi_7656_ = _e5696;
                            phi_7655_ = _e5694;
                            phi_7654_ = _e5692;
                            phi_7652_ = _e5690;
                            break if !(_e5722);
                        }
                    }
                    let _e5739 = phi_16831_;
                    let _e5741 = phi_7676_;
                    let _e5743 = phi_7675_;
                    let _e5745 = phi_7674_;
                    let _e5747 = phi_7673_;
                    let _e5749 = phi_7672_;
                    let _e5751 = phi_7671_;
                    let _e5753 = phi_7670_;
                    let _e5755 = phi_7669_;
                    let _e5757 = phi_7668_;
                    let _e5759 = phi_7667_;
                    let _e5761 = phi_7665_;
                    let _e5763 = phi_7661_;
                    let _e5765 = phi_7659_;
                    let _e5767 = phi_7657_;
                    let _e5769 = phi_7656_;
                    let _e5771 = phi_7655_;
                    let _e5773 = phi_7654_;
                    let _e5775 = phi_7652_;
                    if _e5739 {
                        break;
                    }
                    if _e5741 {
                        let _e5777 = local_19;
                        if _e5777 {
                            phi_5896_ = 1f;
                        } else {
                            if ((_e211 & 2u) == 0u) {
                                phi_5895_ = 1f;
                            } else {
                                let _e5782 = global_1.member.member_3[1u];
                                let _e5785 = global_1.member.member_3[2u];
                                let _e5787 = local_20;
                                let _e5790 = local_21;
                                let _e5794 = ((atan2(_e5787.z, _e5790.x) * 0.15915494f) + 0.5f);
                                let _e5798 = local_22;
                                let _e5801 = select(_e5798.y, -1f, (_e5798.y < -1f));
                                let _e5804 = acos(select(_e5801, 1f, (_e5801 > 1f)));
                                let _e5806 = f32(_e5782);
                                let _e5807 = ((_e5794 - floor(_e5794)) * _e5806);
                                let _e5812 = select(select(u32(_e5807), 0u, (_e5807 < 0f)), 4294967295u, (_e5807 > 4294967000f));
                                let _e5813 = (_e5782 - 1u);
                                let _e5816 = f32(_e5785);
                                let _e5817 = ((_e5804 * 0.31830987f) * _e5816);
                                let _e5822 = select(select(u32(_e5817), 0u, (_e5817 < 0f)), 4294967295u, (_e5817 > 4294967000f));
                                let _e5823 = (_e5785 - 1u);
                                let _e5825 = select(_e5822, _e5823, (_e5823 < _e5822));
                                let _e5826 = sin(_e5804);
                                if (_e5826 <= 0f) {
                                    phi_5893_ = 0f;
                                } else {
                                    let _e5830 = global_1.member.member_2[3u];
                                    let _e5833 = global_1.member.member_3[0u];
                                    let _e5837 = (_e5830 + _e5825);
                                    let _e5840 = global_6.member[(_e5837 + 1u)];
                                    let _e5842 = global_6.member[_e5837];
                                    let _e5845 = ((_e5833 + (_e5825 * (_e5782 + 1u))) + select(_e5812, _e5813, (_e5813 < _e5812)));
                                    let _e5848 = global_6.member[(_e5845 + 1u)];
                                    let _e5850 = global_6.member[_e5845];
                                    phi_5893_ = ((((_e5840 - _e5842) * _e5816) * ((_e5848 - _e5850) * _e5806)) / (19.73921f * _e5826));
                                }
                                let _e5857 = phi_5893_;
                                let _e5859 = local_23;
                                if (_e5859 >= 300000000000000000000000000000000000000f) {
                                    phi_12849_ = 1f;
                                } else {
                                    let _e5862 = local_24;
                                    let _e5864 = local_25;
                                    let _e5865 = (_e5862 * _e5864);
                                    let _e5867 = (_e5865 + (_e5857 * _e5857));
                                    if (_e5867 == 0f) {
                                        phi_12846_ = 0f;
                                    } else {
                                        phi_12846_ = (_e5865 / _e5867);
                                    }
                                    let _e5871 = phi_12846_;
                                    phi_12849_ = _e5871;
                                }
                                let _e5873 = phi_12849_;
                                phi_5895_ = _e5873;
                            }
                            let _e5875 = phi_5895_;
                            phi_5896_ = _e5875;
                        }
                        let _e5877 = phi_5896_;
                        let _e5880 = global_1.member.member_3[1u];
                        let _e5883 = global_1.member.member_3[2u];
                        let _e5885 = local_26;
                        let _e5888 = local_27;
                        let _e5892 = ((atan2(_e5885.z, _e5888.x) * 0.15915494f) + 0.5f);
                        let _e5897 = (((_e5892 - floor(_e5892)) * f32(_e5880)) - 0.5f);
                        let _e5899 = local_28;
                        let _e5902 = select(_e5899.y, -1f, (_e5899.y < -1f));
                        let _e5909 = (((acos(select(_e5902, 1f, (_e5902 > 1f))) * 0.31830987f) * f32(_e5883)) - 0.5f);
                        let _e5910 = (_e5883 - 1u);
                        let _e5911 = f32(_e5910);
                        if (0f <= _e5911) {
                        } else {
                            break;
                        }
                        let _e5914 = select(_e5909, 0f, (_e5909 < 0f));
                        let _e5916 = select(_e5914, _e5911, (_e5914 > _e5911));
                        let _e5917 = floor(_e5897);
                        let _e5918 = floor(_e5916);
                        let _e5919 = (_e5897 - _e5917);
                        let _e5920 = (_e5916 - _e5918);
                        let _e5931 = (_e5880 == 0u);
                        if _e5931 {
                            break;
                        }
                        let _e5932 = (bitcast<u32>((select(0i, select(select(i32(_e5917), i32(-2147483648), (_e5917 < -2147483600f)), 2147483647i, (_e5917 > 2147483500f)), (_e5917 == _e5917)) + bitcast<i32>(_e5880))) % _e5880);
                        let _e5937 = select(select(u32(_e5918), 0u, (_e5918 < 0f)), 4294967295u, (_e5918 > 4294967000f));
                        let _e5938 = (_e5937 + 1u);
                        let _e5943 = global_1.member.member_2[2u];
                        let _e5944 = (_e5937 * _e5880);
                        if _e5931 {
                            break;
                        }
                        let _e5945 = (_e5932 % _e5880);
                        let _e5948 = (_e5943 + (3u * (_e5944 + _e5945)));
                        let _e5950 = global_6.member[_e5948];
                        let _e5953 = global_6.member[(_e5948 + 1u)];
                        let _e5956 = global_6.member[(_e5948 + 2u)];
                        if _e5931 {
                            break;
                        }
                        let _e5959 = ((_e5932 + 1u) % _e5880);
                        let _e5962 = (_e5943 + (3u * (_e5944 + _e5959)));
                        let _e5964 = global_6.member[_e5962];
                        let _e5967 = global_6.member[(_e5962 + 1u)];
                        let _e5970 = global_6.member[(_e5962 + 2u)];
                        let _e5978 = (vec3<f32>(_e5950, _e5953, _e5956) + vec3<f32>(((_e5964 - _e5950) * _e5919), ((_e5967 - _e5953) * _e5919), ((_e5970 - _e5956) * _e5919)));
                        let _e5979 = (select(_e5938, _e5910, (_e5910 < _e5938)) * _e5880);
                        if _e5931 {
                            break;
                        }
                        let _e5982 = (_e5943 + (3u * (_e5979 + _e5945)));
                        let _e5984 = global_6.member[_e5982];
                        let _e5987 = global_6.member[(_e5982 + 1u)];
                        let _e5990 = global_6.member[(_e5982 + 2u)];
                        if _e5931 {
                            break;
                        }
                        let _e5994 = (_e5943 + (3u * (_e5979 + _e5959)));
                        let _e5996 = global_6.member[_e5994];
                        let _e5999 = global_6.member[(_e5994 + 1u)];
                        let _e6002 = global_6.member[(_e5994 + 2u)];
                        let _e6010 = (vec3<f32>(_e5984, _e5987, _e5990) + vec3<f32>(((_e5996 - _e5984) * _e5919), ((_e5999 - _e5987) * _e5919), ((_e6002 - _e5990) * _e5919)));
                        let _e6024 = (_e5978 + vec3<f32>(((_e6010.x - _e5978.x) * _e5920), ((_e6010.y - _e5978.y) * _e5920), ((_e6010.z - _e5978.z) * _e5920)));
                        let _e6026 = local_29;
                        let _e6031 = local_30;
                        let _e6036 = local_31;
                        let _e6044 = local_32;
                        let _e6046 = (_e6044.x + ((_e6026.x * _e6024.x) * _e5877));
                        let _e6048 = local_33;
                        let _e6050 = (_e6048.y + ((_e6031.y * _e6024.y) * _e5877));
                        let _e6052 = local_34;
                        let _e6054 = (_e6052.z + ((_e6036.z * _e6024.z) * _e5877));
                        if ((_e211 & 4u) == 0u) {
                            phi_6348_ = vec3<f32>(_e6046, _e6050, _e6054);
                        } else {
                            let _e6059 = local_35;
                            if _e6059 {
                                phi_6276_ = 1f;
                            } else {
                                let _e6061 = global_1.member.member_5;
                                let _e6072 = global_1.member.member_5[3u];
                                if ((((_e5888.x * _e6061.x) + (_e5899.y * _e6061.y)) + (_e5885.z * _e6061.z)) <= _e6072) {
                                    phi_6274_ = 0f;
                                } else {
                                    phi_6274_ = (0.15915494f / (1f - _e6072));
                                }
                                let _e6077 = phi_6274_;
                                let _e6079 = local_36;
                                if (_e6079 >= 300000000000000000000000000000000000000f) {
                                    phi_12918_ = 1f;
                                } else {
                                    let _e6082 = local_37;
                                    let _e6084 = local_38;
                                    let _e6085 = (_e6082 * _e6084);
                                    let _e6087 = (_e6085 + (_e6077 * _e6077));
                                    if (_e6087 == 0f) {
                                        phi_12915_ = 0f;
                                    } else {
                                        phi_12915_ = (_e6085 / _e6087);
                                    }
                                    let _e6091 = phi_12915_;
                                    phi_12918_ = _e6091;
                                }
                                let _e6093 = phi_12918_;
                                phi_6276_ = _e6093;
                            }
                            let _e6095 = phi_6276_;
                            let _e6097 = global_1.member.member_5;
                            let _e6105 = (((_e5888.x * _e6097.x) + (_e5899.y * _e6097.y)) + (_e5885.z * _e6097.z));
                            let _e6108 = global_1.member.member_5[3u];
                            if (_e6105 <= _e6108) {
                                phi_6331_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                if (_e6105 != _e6105) {
                                    phi_12929_ = true;
                                } else {
                                    phi_12929_ = (1f <= _e6105);
                                }
                                let _e6113 = phi_12929_;
                                let _e6119 = global_1.member.member_6[3u];
                                let _e6122 = (1f - pow((acos(select(_e6105, 1f, _e6113)) / _e6119), 2f));
                                if (_e6122 != _e6122) {
                                    phi_12944_ = true;
                                } else {
                                    phi_12944_ = (0f >= _e6122);
                                }
                                let _e6126 = phi_12944_;
                                let _e6131 = global_1.member.member_6;
                                let _e6139 = (((1f - (0.6f * (1f - sqrt(select(_e6122, 0f, _e6126))))) * 1.25f) / (6.2831855f * (1f - _e6108)));
                                phi_6331_ = vec3<f32>((_e6131.x * _e6139), (_e6131.y * _e6139), (_e6131.z * _e6139));
                            }
                            let _e6145 = phi_6331_;
                            phi_6348_ = vec3<f32>((_e6046 + ((_e6026.x * _e6145.x) * _e6095)), (_e6050 + ((_e6031.y * _e6145.y) * _e6095)), (_e6054 + ((_e6036.z * _e6145.z) * _e6095)));
                        }
                        let _e6160 = phi_6348_;
                        phi_6349_ = _e6160;
                    } else {
                        phi_6349_ = vec3<f32>();
                    }
                    let _e6162 = phi_6349_;
                    let _e6163 = select(_e5757, false, _e5741);
                    if _e6163 {
                        let _e6172 = local_39;
                        let _e6176 = local_40;
                        let _e6180 = local_41;
                        let _e6184 = global_4.member[_e5763];
                        if (_e6184.member == 4u) {
                            if (_e6184.member_1 == 1u) {
                                phi_6401_ = true;
                            } else {
                                phi_6401_ = ((((-(_e6172.x) * _e5743.member_3.x) + (-(_e6176.y) * _e5743.member_3.y)) + (-(_e6180.z) * _e5743.member_3.z)) > 0f);
                            }
                            let _e6199 = phi_6401_;
                            if _e6199 {
                                phi_6410_ = vec3<f32>(_e6184.member_4.x, _e6184.member_4.y, _e6184.member_4.z);
                            } else {
                                phi_6410_ = vec3<f32>();
                            }
                            let _e6206 = phi_6410_;
                            phi_6412_ = _e6206;
                            phi_6413_ = select(true, false, _e6199);
                        } else {
                            phi_6412_ = vec3<f32>();
                            phi_6413_ = true;
                        }
                        let _e6209 = phi_6412_;
                        let _e6211 = phi_6413_;
                        let _e6213 = select(_e6209, vec3<f32>(0f, 0f, 0f), vec3(_e6211));
                        let _e6215 = local_42;
                        if _e6215 {
                            phi_6586_ = 1f;
                        } else {
                            if ((_e211 & 1u) == 0u) {
                                phi_6585_ = 1f;
                            } else {
                                if (_e5765.member_6.z > 0u) {
                                    let _e6224 = global_1.member.member_2[1u];
                                    let _e6225 = (_e6224 + (_e5765.member_6.z - 1u));
                                    let _e6228 = global_6.member[(_e6225 + 1u)];
                                    let _e6230 = global_6.member[_e6225];
                                    let _e6235 = global_2.member[_e5743.member_1];
                                    if (_e6235.member_6.x == 1u) {
                                        let _e6320 = local_46;
                                        let _e6322 = (_e6235.member.x - _e6320.x);
                                        let _e6324 = local_47;
                                        let _e6326 = (_e6235.member.y - _e6324.y);
                                        let _e6328 = local_48;
                                        let _e6330 = (_e6235.member.z - _e6328.z);
                                        let _e6335 = (((_e6322 * _e6322) + (_e6326 * _e6326)) + (_e6330 * _e6330));
                                        let _e6336 = (_e6235.member.w * _e6235.member.w);
                                        if (_e6335 <= (_e6336 * 1.000001f)) {
                                            let _e6351 = (_e5767.x - _e6320.x);
                                            let _e6353 = (_e5767.y - _e6324.y);
                                            let _e6355 = (_e5767.z - _e6328.z);
                                            let _e6360 = (((_e6351 * _e6351) + (_e6353 * _e6353)) + (_e6355 * _e6355));
                                            phi_13013_ = (_e6360 / (((abs((((_e5743.member_2.x * _e6351) + (_e5743.member_2.y * _e6353)) + (_e5743.member_2.z * _e6355))) / sqrt(_e6360)) * 12.566371f) * _e6336));
                                        } else {
                                            let _e6339 = (_e6336 / _e6335);
                                            if (_e6339 != _e6339) {
                                                phi_13032_ = true;
                                            } else {
                                                phi_13032_ = (1f <= _e6339);
                                            }
                                            let _e6343 = phi_13032_;
                                            let _e6344 = select(_e6339, 1f, _e6343);
                                            phi_13013_ = (0.15915494f / (_e6344 / (1f + sqrt((1f - _e6344)))));
                                        }
                                        let _e6376 = phi_13013_;
                                        phi_6581_ = _e6376;
                                    } else {
                                        let _e6240 = local_43;
                                        let _e6242 = (_e5773 - _e6240.x);
                                        let _e6244 = local_44;
                                        let _e6246 = (_e5771 - _e6244.y);
                                        let _e6248 = local_45;
                                        let _e6250 = (_e5769 - _e6248.z);
                                        let _e6255 = (((_e6242 * _e6242) + (_e6246 * _e6246)) + (_e6250 * _e6250));
                                        let _e6266 = (abs((((_e5743.member_2.x * _e6242) + (_e5743.member_2.y * _e6246)) + (_e5743.member_2.z * _e6250))) / sqrt(_e6255));
                                        if (_e6266 <= 0f) {
                                            phi_6580_ = 0f;
                                        } else {
                                            if (_e6235.member_6.x == 0u) {
                                                let _e6281 = (_e6235.member_1.x - _e6235.member.x);
                                                let _e6282 = (_e6235.member_1.y - _e6235.member.y);
                                                let _e6283 = (_e6235.member_1.z - _e6235.member.z);
                                                let _e6288 = (_e6235.member_2.x - _e6235.member.x);
                                                let _e6289 = (_e6235.member_2.y - _e6235.member.y);
                                                let _e6290 = (_e6235.member_2.z - _e6235.member.z);
                                                let _e6293 = ((_e6282 * _e6290) - (_e6289 * _e6283));
                                                let _e6296 = ((_e6283 * _e6288) - (_e6290 * _e6281));
                                                let _e6299 = ((_e6281 * _e6289) - (_e6288 * _e6282));
                                                phi_6577_ = (sqrt((((_e6293 * _e6293) + (_e6296 * _e6296)) + (_e6299 * _e6299))) * 0.5f);
                                            } else {
                                                phi_6577_ = ((12.566371f * _e6235.member.w) * _e6235.member.w);
                                            }
                                            let _e6308 = phi_6577_;
                                            phi_6580_ = (_e6255 / (_e6266 * _e6308));
                                        }
                                        let _e6312 = phi_6580_;
                                        phi_6581_ = _e6312;
                                    }
                                    let _e6378 = phi_6581_;
                                    let _e6379 = ((_e6228 - _e6230) * _e6378);
                                    let _e6381 = local_49;
                                    if (_e6381 >= 300000000000000000000000000000000000000f) {
                                        phi_13053_ = 1f;
                                    } else {
                                        let _e6384 = local_50;
                                        let _e6386 = local_51;
                                        let _e6387 = (_e6384 * _e6386);
                                        let _e6389 = (_e6387 + (_e6379 * _e6379));
                                        if (_e6389 == 0f) {
                                            phi_13050_ = 0f;
                                        } else {
                                            phi_13050_ = (_e6387 / _e6389);
                                        }
                                        let _e6393 = phi_13050_;
                                        phi_13053_ = _e6393;
                                    }
                                    let _e6395 = phi_13053_;
                                    phi_6584_ = _e6395;
                                } else {
                                    phi_6584_ = 1f;
                                }
                                let _e6397 = phi_6584_;
                                phi_6585_ = _e6397;
                            }
                            let _e6399 = phi_6585_;
                            phi_6586_ = _e6399;
                        }
                        let _e6401 = phi_6586_;
                        phi_6608_ = vec3<f32>((_e5759.x + ((_e5775.x * _e6213.x) * _e6401)), (_e5759.y + ((_e5775.y * _e6213.y) * _e6401)), (_e5759.z + ((_e5775.z * _e6213.z) * _e6401)));
                    } else {
                        phi_6608_ = vec3<f32>();
                    }
                    let _e6422 = phi_6608_;
                    let _e6423 = select(select(_e5755, false, _e5741), false, _e6163);
                    let _e6429 = select(select(select(_e5753, false, _e5741), false, _e6163), false, _e6423);
                    let _e6435 = select(select(select(select(_e5751, false, _e5741), false, _e6163), false, _e6423), false, _e6429);
                    let _e6441 = select(_e6429, true, _e6435);
                    let _e6447 = select(select(select(select(select(select(_e5749, false, _e5741), false, _e6163), false, _e6423), false, _e6429), false, _e6435), false, _e6441);
                    let _e6452 = select(select(select(select(select(select(select(_e5747, false, _e5741), false, _e6163), false, _e6423), false, _e6429), false, _e6435), false, _e6441), false, _e6447);
                    let _e6457 = select(select(_e6441, false, _e6447), true, _e6452);
                    let _e6462 = select(select(select(select(select(select(_e6423, false, _e6429), false, _e6435), false, _e6441), false, _e6447), false, _e6452), true, _e6457);
                    let _e6466 = select(select(select(select(select(select(select(select(select(select(_e5745, false, _e5741), false, _e6163), false, _e6423), false, _e6429), false, _e6435), false, _e6441), false, _e6447), false, _e6452), false, _e6457), false, _e6462);
                    let _e6469 = select(select(select(select(_e6447, false, _e6452), false, _e6457), false, _e6462), true, _e6466);
                    let _e6474 = select(select(_e6462, false, _e6466), true, _e6469);
                    let _e6480 = select(_e6162, select(_e6422, select(_e5761, _e5759, vec3(_e6469)), vec3(_e6474)), vec3(select(select(select(select(select(select(select(select(select(select(select(_e6163, false, _e6423), false, _e6429), false, _e6435), false, _e6441), false, _e6447), false, _e6452), false, _e6457), false, _e6462), false, _e6466), false, _e6469), true, _e6474)));
                    let _e6483 = (_e6480.x * _e144.w);
                    let _e6485 = (_e6480.y * _e144.w);
                    let _e6487 = (_e6480.z * _e144.w);
                    if (_e6483 == _e6483) {
                        if (_e6485 == _e6485) {
                            if (_e6487 == _e6487) {
                                if (_e129 < arrayLength((&global_8.member))) {
                                } else {
                                    break;
                                }
                                let _e6495 = global_8.member[_e129][0u];
                                global_8.member[_e129][0u] = (_e6495 + _e6483);
                                let _e6500 = global_8.member[_e129][1u];
                                global_8.member[_e129][1u] = (_e6500 + _e6485);
                                let _e6505 = global_8.member[_e129][2u];
                                global_8.member[_e129][2u] = (_e6505 + _e6487);
                                let _e6510 = global_8.member[_e129][3u];
                                global_8.member[_e129][3u] = (_e6510 + 1f);
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
