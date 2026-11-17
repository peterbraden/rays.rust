struct type_7 {
    member: vec4<u32>,
    member_1: vec4<u32>,
    member_2: vec4<u32>,
    member_3: vec4<u32>,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<f32>,
    member_7: vec4<u32>,
    member_8: vec4<u32>,
}

struct type_9 {
    member: type_7,
}

struct type_11 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<u32>,
}

struct type_14 {
    member: array<type_11>,
}

struct type_16 {
    member: vec3<f32>,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: u32,
    member_4: vec3<f32>,
    member_5: u32,
    member_6: vec3<f32>,
    member_7: u32,
}

struct type_19 {
    member: array<type_16>,
}

struct type_21 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<f32>,
}

struct type_24 {
    member: array<type_21>,
}

struct type_26 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_29 {
    member: array<type_26>,
}

struct type_33 {
    member: array<f32>,
}

struct type_35 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
    member_4: vec4<u32>,
}

struct type_37 {
    member: array<type_35>,
}

struct type_39 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
}

struct type_41 {
    member: array<type_39>,
}

struct type_45 {
    member: u32,
    member_1: u32,
}

struct type_49 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_52 {
    member: u32,
    member_1: type_49,
}

struct type_53 {
    member: f32,
    member_1: f32,
}

struct type_54 {
    member: u32,
    member_1: type_53,
}

struct type_55 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_56 {
    member: vec3<f32>,
    member_1: f32,
}

struct type_57 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_64 {
    member: array<vec4<f32>>,
}

struct type_66 {
    member: f32,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
}

struct type_71 {
    member: type_35,
    member_1: type_39,
    member_2: type_39,
}

struct type_74 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<uniform> global_1: type_9;
@group(0) @binding(1) 
var<storage> global_2: type_14;
@group(0) @binding(2) 
var<storage> global_3: type_19;
@group(0) @binding(3) 
var<storage> global_4: type_24;
@group(0) @binding(4) 
var<storage> global_5: type_29;
@group(0) @binding(5) 
var<storage> global_6: type_33;
@group(0) @binding(6) 
var<storage, read_write> global_7: type_37;
@group(0) @binding(8) 
var<storage, read_write> global_8: type_41;
@group(0) @binding(7) 
var<storage, read_write> global_9: type_64;

fn function_() {
    var phi_450_: type_45;
    var phi_32362_: bool;
    var phi_470_: vec3<f32>;
    var phi_473_: u32;
    var phi_20625_: f32;
    var phi_20637_: type_53;
    var phi_20648_: type_54;
    var phi_20649_: type_54;
    var phi_20653_: type_54;
    var phi_20654_: type_54;
    var phi_20655_: bool;
    var phi_20659_: type_54;
    var phi_20546_: type_54;
    var phi_20697_: bool;
    var phi_20777_: f32;
    var phi_20789_: type_53;
    var phi_20800_: type_54;
    var phi_20801_: type_54;
    var phi_20805_: type_54;
    var phi_20806_: type_54;
    var phi_20807_: bool;
    var phi_20811_: type_54;
    var phi_20745_: f32;
    var phi_20849_: bool;
    var phi_20574_: f32;
    var phi_20576_: f32;
    var phi_20585_: type_54;
    var phi_20589_: type_54;
    var phi_20593_: type_54;
    var phi_20864_: bool;
    var phi_9473_: vec3<f32>;
    var phi_9476_: vec3<f32>;
    var phi_9478_: vec3<f32>;
    var phi_9480_: vec3<f32>;
    var phi_9482_: u32;
    var phi_9566_: u32;
    var phi_9581_: type_53;
    var phi_9582_: type_53;
    var phi_9583_: type_53;
    var phi_21064_: bool;
    var phi_21191_: bool;
    var phi_21206_: bool;
    var phi_9911_: vec3<f32>;
    var phi_9984_: vec3<f32>;
    var phi_9985_: vec3<f32>;
    var phi_9986_: bool;
    var phi_9991_: vec3<f32>;
    var phi_9992_: bool;
    var phi_21330_: bool;
    var phi_21345_: bool;
    var phi_10283_: vec3<f32>;
    var phi_9474_: vec3<f32>;
    var phi_9477_: vec3<f32>;
    var phi_9479_: vec3<f32>;
    var phi_9481_: vec3<f32>;
    var phi_9483_: u32;
    var phi_32345_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_32829_: bool;
    var phi_10340_: type_57;
    var phi_32828_: bool;
    var phi_10342_: type_57;
    var phi_10343_: bool;
    var phi_10348_: type_57;
    var phi_849_: vec3<f32>;
    var phi_851_: vec3<f32>;
    var phi_852_: bool;
    var phi_32437_: bool;
    var phi_862_: vec3<f32>;
    var phi_865_: i32;
    var phi_867_: u32;
    var phi_21614_: bool;
    var phi_1289_: u32;
    var phi_1319_: u32;
    var phi_1290_: u32;
    var phi_1320_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1338_: u32;
    var phi_2643_: type_52;
    var phi_2651_: type_49;
    var phi_2653_: type_49;
    var phi_2654_: bool;
    var phi_2655_: type_49;
    var phi_2656_: bool;
    var phi_2657_: bool;
    var phi_21725_: f32;
    var phi_21737_: type_53;
    var phi_21748_: type_54;
    var phi_21749_: type_54;
    var phi_21753_: type_54;
    var phi_21754_: type_54;
    var phi_21755_: bool;
    var phi_21759_: type_54;
    var phi_21646_: type_54;
    var phi_21797_: bool;
    var phi_21877_: f32;
    var phi_21889_: type_53;
    var phi_21900_: type_54;
    var phi_21901_: type_54;
    var phi_21905_: type_54;
    var phi_21906_: type_54;
    var phi_21907_: bool;
    var phi_21911_: type_54;
    var phi_21845_: f32;
    var phi_21949_: bool;
    var phi_21674_: f32;
    var phi_21676_: f32;
    var phi_21685_: type_54;
    var phi_21689_: type_54;
    var phi_21693_: type_54;
    var phi_2675_: vec3<f32>;
    var phi_2678_: vec3<f32>;
    var phi_2680_: vec3<f32>;
    var phi_2682_: vec3<f32>;
    var phi_2684_: u32;
    var phi_2768_: u32;
    var phi_2783_: type_53;
    var phi_2784_: type_53;
    var phi_2785_: type_53;
    var phi_22148_: bool;
    var phi_22275_: bool;
    var phi_22290_: bool;
    var phi_3113_: vec3<f32>;
    var phi_22412_: f32;
    var phi_22424_: type_53;
    var phi_22435_: type_54;
    var phi_22436_: type_54;
    var phi_22440_: type_54;
    var phi_22441_: type_54;
    var phi_22442_: bool;
    var phi_22446_: type_54;
    var phi_22380_: f32;
    var phi_22522_: f32;
    var phi_22534_: type_53;
    var phi_22545_: type_54;
    var phi_22546_: type_54;
    var phi_22550_: type_54;
    var phi_22551_: type_54;
    var phi_22552_: bool;
    var phi_22556_: type_54;
    var phi_22490_: f32;
    var phi_22317_: vec3<f32>;
    var phi_22318_: u32;
    var phi_22688_: bool;
    var phi_22351_: vec3<f32>;
    var phi_22352_: u32;
    var local_11: vec3<f32>;
    var phi_22359_: type_55;
    var phi_22363_: type_55;
    var phi_3172_: vec3<f32>;
    var phi_3173_: vec3<f32>;
    var phi_2676_: vec3<f32>;
    var phi_2679_: vec3<f32>;
    var phi_2681_: vec3<f32>;
    var phi_2683_: vec3<f32>;
    var phi_2685_: u32;
    var phi_32405_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_32499_: bool;
    var phi_3193_: type_49;
    var phi_32498_: bool;
    var phi_3194_: type_49;
    var phi_32497_: bool;
    var phi_3217_: vec3<f32>;
    var phi_3218_: i32;
    var phi_3219_: u32;
    var phi_32496_: bool;
    var phi_863_: vec3<f32>;
    var phi_866_: i32;
    var phi_868_: u32;
    var phi_3220_: bool;
    var phi_32495_: bool;
    var phi_3554_: vec3<f32>;
    var phi_3557_: u32;
    var phi_22816_: f32;
    var phi_22831_: f32;
    var phi_22846_: f32;
    var phi_3628_: i32;
    var phi_3658_: i32;
    var phi_3666_: vec3<f32>;
    var phi_3669_: i32;
    var phi_3692_: vec3<f32>;
    var phi_3695_: i32;
    var phi_3693_: vec3<f32>;
    var phi_3696_: i32;
    var phi_3667_: vec3<f32>;
    var phi_3670_: i32;
    var phi_3894_: vec3<f32>;
    var phi_3895_: vec3<f32>;
    var phi_3896_: vec3<f32>;
    var phi_3555_: vec3<f32>;
    var phi_3558_: u32;
    var phi_32500_: bool;
    var local_15: vec3<f32>;
    var phi_32548_: bool;
    var phi_3907_: f32;
    var phi_3931_: f32;
    var phi_3933_: u32;
    var phi_3936_: f32;
    var phi_4039_: f32;
    var phi_4040_: f32;
    var phi_4056_: f32;
    var phi_3934_: u32;
    var phi_3937_: f32;
    var phi_4059_: u32;
    var phi_4062_: f32;
    var phi_4170_: f32;
    var phi_4060_: u32;
    var phi_4063_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_22962_: f32;
    var phi_22974_: type_53;
    var phi_22985_: type_54;
    var phi_22986_: type_54;
    var phi_22990_: type_54;
    var phi_22991_: type_54;
    var phi_22992_: bool;
    var phi_22996_: type_54;
    var phi_22930_: f32;
    var phi_4571_: bool;
    var phi_23122_: f32;
    var phi_23134_: type_53;
    var phi_23145_: type_54;
    var phi_23146_: type_54;
    var phi_23150_: type_54;
    var phi_23151_: type_54;
    var phi_23152_: bool;
    var phi_23156_: type_54;
    var phi_23043_: type_54;
    var phi_23194_: bool;
    var phi_23274_: f32;
    var phi_23286_: type_53;
    var phi_23297_: type_54;
    var phi_23298_: type_54;
    var phi_23302_: type_54;
    var phi_23303_: type_54;
    var phi_23304_: bool;
    var phi_23308_: type_54;
    var phi_23242_: f32;
    var phi_23346_: bool;
    var phi_23071_: f32;
    var phi_23073_: f32;
    var phi_23082_: type_54;
    var phi_23086_: type_54;
    var phi_23090_: type_54;
    var phi_4585_: vec3<f32>;
    var phi_4588_: u32;
    var phi_4672_: u32;
    var phi_4687_: type_53;
    var phi_4688_: type_53;
    var phi_4689_: type_53;
    var phi_23545_: bool;
    var phi_4586_: vec3<f32>;
    var phi_4589_: u32;
    var phi_32527_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_32594_: bool;
    var phi_4714_: vec3<f32>;
    var phi_32593_: bool;
    var phi_4716_: vec3<f32>;
    var phi_4718_: bool;
    var phi_32589_: bool;
    var phi_4729_: vec3<f32>;
    var phi_4730_: bool;
    var phi_32588_: bool;
    var phi_4732_: vec3<f32>;
    var phi_4733_: bool;
    var phi_4734_: bool;
    var phi_32587_: bool;
    var phi_4738_: vec3<f32>;
    var phi_4739_: vec3<f32>;
    var phi_4740_: bool;
    var phi_4741_: vec3<f32>;
    var phi_4742_: bool;
    var phi_5062_: vec3<f32>;
    var phi_5065_: u32;
    var phi_23616_: f32;
    var phi_23631_: f32;
    var phi_23646_: f32;
    var phi_5136_: i32;
    var phi_5166_: i32;
    var phi_5174_: vec3<f32>;
    var phi_5177_: i32;
    var phi_5200_: vec3<f32>;
    var phi_5203_: i32;
    var phi_5274_: vec3<f32>;
    var phi_5277_: u32;
    var phi_5397_: vec3<f32>;
    var phi_5275_: vec3<f32>;
    var phi_5278_: u32;
    var phi_5201_: vec3<f32>;
    var phi_5204_: i32;
    var phi_5175_: vec3<f32>;
    var phi_5178_: i32;
    var phi_5402_: vec3<f32>;
    var phi_5403_: vec3<f32>;
    var phi_5404_: vec3<f32>;
    var phi_5063_: vec3<f32>;
    var phi_5066_: u32;
    var phi_32582_: bool;
    var phi_32624_: bool;
    var phi_5424_: vec3<f32>;
    var phi_5637_: u32;
    var phi_5638_: bool;
    var phi_5641_: u32;
    var phi_5642_: bool;
    var phi_5643_: bool;
    var phi_5653_: f32;
    var phi_5656_: f32;
    var phi_5658_: u32;
    var phi_5660_: vec3<f32>;
    var phi_5662_: vec3<f32>;
    var phi_5664_: f32;
    var phi_5666_: f32;
    var phi_5704_: u32;
    var phi_5719_: vec3<f32>;
    var phi_5739_: vec3<f32>;
    var phi_23661_: vec3<f32>;
    var phi_5740_: u32;
    var phi_5741_: vec3<f32>;
    var phi_5742_: u32;
    var phi_5743_: vec3<f32>;
    var phi_5744_: u32;
    var phi_5746_: vec3<f32>;
    var phi_5881_: u32;
    var phi_5884_: f32;
    var phi_5886_: f32;
    var phi_5888_: f32;
    var phi_5890_: vec3<f32>;
    var phi_23716_: f32;
    var phi_23722_: f32;
    var phi_23750_: f32;
    var phi_23756_: f32;
    var phi_23784_: f32;
    var phi_23790_: f32;
    var phi_23818_: f32;
    var phi_23824_: f32;
    var phi_23852_: f32;
    var phi_23858_: f32;
    var phi_23886_: f32;
    var phi_23892_: f32;
    var phi_23920_: f32;
    var phi_23926_: f32;
    var phi_23954_: f32;
    var phi_23960_: f32;
    var phi_5882_: u32;
    var phi_5885_: f32;
    var phi_5887_: f32;
    var phi_5889_: f32;
    var phi_5891_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6307_: f32;
    var phi_6359_: vec3<f32>;
    var phi_6360_: f32;
    var phi_6361_: vec3<f32>;
    var phi_6362_: f32;
    var phi_6363_: vec3<f32>;
    var phi_6364_: f32;
    var phi_6366_: vec3<f32>;
    var phi_6367_: f32;
    var phi_6368_: f32;
    var phi_6369_: vec3<f32>;
    var phi_6370_: vec3<f32>;
    var phi_6371_: f32;
    var phi_6372_: f32;
    var phi_6373_: vec3<f32>;
    var phi_6374_: vec3<f32>;
    var phi_6375_: f32;
    var phi_6376_: f32;
    var phi_6377_: vec3<f32>;
    var phi_6378_: vec3<f32>;
    var phi_6379_: f32;
    var phi_6380_: f32;
    var phi_6381_: vec3<f32>;
    var phi_6382_: vec3<f32>;
    var phi_6384_: f32;
    var phi_6385_: f32;
    var phi_6386_: vec3<f32>;
    var phi_6387_: vec3<f32>;
    var phi_6388_: f32;
    var phi_6389_: f32;
    var phi_6390_: f32;
    var phi_6391_: f32;
    var phi_6392_: vec3<f32>;
    var phi_6393_: vec3<f32>;
    var phi_6394_: f32;
    var phi_6395_: f32;
    var phi_5654_: f32;
    var phi_5657_: f32;
    var phi_5659_: u32;
    var phi_5661_: vec3<f32>;
    var phi_5663_: vec3<f32>;
    var phi_5665_: f32;
    var phi_5667_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_24036_: bool;
    var local_29: f32;
    var phi_6427_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6506_: f32;
    var phi_6507_: vec3<f32>;
    var phi_6508_: f32;
    var phi_6510_: vec3<f32>;
    var phi_6513_: u32;
    var phi_6961_: f32;
    var phi_7141_: u32;
    var phi_7144_: f32;
    var phi_7146_: f32;
    var phi_7148_: f32;
    var phi_7150_: vec3<f32>;
    var phi_24125_: f32;
    var phi_24131_: f32;
    var phi_24159_: f32;
    var phi_24165_: f32;
    var phi_24193_: f32;
    var phi_24199_: f32;
    var phi_24227_: f32;
    var phi_24233_: f32;
    var phi_24261_: f32;
    var phi_24267_: f32;
    var phi_24295_: f32;
    var phi_24301_: f32;
    var phi_24329_: f32;
    var phi_24335_: f32;
    var phi_24363_: f32;
    var phi_24369_: f32;
    var phi_7142_: u32;
    var phi_7145_: f32;
    var phi_7147_: f32;
    var phi_7149_: f32;
    var phi_7151_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7564_: f32;
    var phi_24415_: bool;
    var phi_24430_: bool;
    var phi_7657_: vec3<f32>;
    var phi_7658_: vec3<f32>;
    var phi_7659_: vec3<f32>;
    var phi_7660_: vec3<f32>;
    var phi_6511_: vec3<f32>;
    var phi_6514_: u32;
    var phi_7679_: vec3<f32>;
    var phi_7682_: u32;
    var phi_7684_: f32;
    var phi_7985_: vec3<f32>;
    var phi_7986_: u32;
    var phi_7987_: f32;
    var phi_7988_: vec3<f32>;
    var phi_8005_: vec3<f32>;
    var phi_8007_: u32;
    var phi_8010_: vec3<f32>;
    var phi_8012_: f32;
    var phi_8014_: f32;
    var phi_8016_: f32;
    var phi_8036_: bool;
    var phi_8052_: bool;
    var phi_8054_: bool;
    var phi_8056_: bool;
    var phi_8057_: bool;
    var phi_8058_: bool;
    var phi_8059_: bool;
    var phi_8060_: bool;
    var phi_8104_: vec3<f32>;
    var phi_24449_: vec3<f32>;
    var phi_8105_: u32;
    var phi_8106_: vec3<f32>;
    var phi_8107_: u32;
    var phi_8108_: vec3<f32>;
    var phi_8243_: u32;
    var phi_8246_: f32;
    var phi_8248_: f32;
    var phi_8250_: f32;
    var phi_8252_: vec3<f32>;
    var phi_24504_: f32;
    var phi_24510_: f32;
    var phi_24538_: f32;
    var phi_24544_: f32;
    var phi_24572_: f32;
    var phi_24578_: f32;
    var phi_24606_: f32;
    var phi_24612_: f32;
    var phi_24640_: f32;
    var phi_24646_: f32;
    var phi_24674_: f32;
    var phi_24680_: f32;
    var phi_24708_: f32;
    var phi_24714_: f32;
    var phi_24742_: f32;
    var phi_24748_: f32;
    var phi_8244_: u32;
    var phi_8247_: f32;
    var phi_8249_: f32;
    var phi_8251_: f32;
    var phi_8253_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8666_: f32;
    var phi_8692_: vec3<f32>;
    var phi_8693_: f32;
    var phi_8694_: f32;
    var phi_8695_: vec3<f32>;
    var phi_8696_: f32;
    var phi_8697_: f32;
    var phi_8698_: vec3<f32>;
    var phi_8699_: f32;
    var phi_8700_: f32;
    var phi_8701_: vec3<f32>;
    var phi_8702_: f32;
    var phi_8703_: f32;
    var phi_8705_: vec3<f32>;
    var phi_8706_: f32;
    var phi_8707_: f32;
    var phi_8708_: f32;
    var phi_8008_: u32;
    var phi_8011_: vec3<f32>;
    var phi_8013_: f32;
    var phi_8015_: f32;
    var phi_8017_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_8998_: type_56;
    var local_42: f32;
    var phi_8999_: type_56;
    var local_43: f32;
    var local_44: f32;
    var phi_9000_: type_56;
    var phi_9001_: type_56;
    var phi_7680_: vec3<f32>;
    var phi_7683_: u32;
    var phi_7685_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9030_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_32832_: bool;
    var phi_9077_: vec3<f32>;
    var phi_32831_: bool;
    var phi_9078_: vec3<f32>;
    var phi_32830_: bool;
    var phi_9079_: vec3<f32>;
    var phi_32826_: bool;
    var phi_10352_: vec3<f32>;
    var phi_32825_: bool;
    var phi_10353_: vec3<f32>;
    var phi_32824_: bool;
    var phi_471_: vec3<f32>;
    var phi_474_: u32;
    var phi_32823_: bool;
    var local_55: vec3<f32>;
    var local_56: vec3<f32>;
    var local_57: vec3<f32>;
    var local_58: vec3<f32>;
    var local_59: vec3<f32>;
    var local_60: f32;
    var local_61: vec3<f32>;
    var local_62: vec3<f32>;
    var local_63: vec3<f32>;
    var local_64: vec3<f32>;
    var local_65: vec3<f32>;
    var local_66: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e274 = global;
            let _e284 = (_e274.x + (_e274.y * 4194240u));
            let _e288 = global_1.member.member[0u];
            let _e292 = global_1.member.member[1u];
            if (_e284 >= (_e288 * _e292)) {
                phi_450_ = type_45(0u, type_45().member_1);
            } else {
                phi_450_ = type_45(1u, _e284);
            }
            let _e300 = phi_450_;
            if (bitcast<i32>(_e300.member) != 0i) {
                phi_32362_ = false;
                phi_470_ = vec3<f32>(0f, 0f, 0f);
                phi_473_ = 0u;
                loop {
                    let _e306 = phi_32362_;
                    let _e308 = phi_470_;
                    let _e310 = phi_473_;
                    local_55 = _e308;
                    local_56 = _e308;
                    local_57 = _e308;
                    let _e311 = (_e310 < 2u);
                    if _e311 {
                        let _e313 = ((2u * _e300.member_1) + _e310);
                        if (_e313 < arrayLength((&global_8.member))) {
                        } else {
                            phi_32823_ = true;
                            break;
                        }
                        let _e317 = global_8.member[_e313];
                        if ((((_e317.member_2.x * _e317.member_2.x) + (_e317.member_2.y * _e317.member_2.y)) + (_e317.member_2.z * _e317.member_2.z)) == 0f) {
                            phi_32825_ = _e306;
                            phi_10353_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            if (_e317.member.w < 0f) {
                                let _e1272 = global_1.member.member_8[0u];
                                let _e1275 = global_1.member.member_8[3u];
                                if (_e1272 == 4294967295u) {
                                    let _e1277 = (_e1275 == 4294967295u);
                                    if _e1277 {
                                        let _e1280 = global_1.member.member_3[1u];
                                        let _e1283 = global_1.member.member_3[2u];
                                        let _e1286 = ((atan2(_e317.member_1.z, _e317.member_1.x) * 0.15915494f) + 0.5f);
                                        let _e1291 = (((_e1286 - floor(_e1286)) * f32(_e1280)) - 0.5f);
                                        let _e1293 = select(_e317.member_1.y, -1f, (_e317.member_1.y < -1f));
                                        let _e1300 = (((acos(select(_e1293, 1f, (_e1293 > 1f))) * 0.31830987f) * f32(_e1283)) - 0.5f);
                                        let _e1301 = (_e1283 - 1u);
                                        let _e1302 = f32(_e1301);
                                        if (0f <= _e1302) {
                                        } else {
                                            phi_32823_ = true;
                                            break;
                                        }
                                        let _e1305 = select(_e1300, 0f, (_e1300 < 0f));
                                        let _e1307 = select(_e1305, _e1302, (_e1305 > _e1302));
                                        let _e1308 = floor(_e1291);
                                        let _e1309 = floor(_e1307);
                                        let _e1310 = (_e1291 - _e1308);
                                        let _e1311 = (_e1307 - _e1309);
                                        let _e1322 = (_e1280 == 0u);
                                        if _e1322 {
                                            phi_32823_ = true;
                                            break;
                                        }
                                        let _e1323 = (bitcast<u32>((select(0i, select(select(i32(_e1308), i32(-2147483648), (_e1308 < -2147483600f)), 2147483647i, (_e1308 > 2147483500f)), (_e1308 == _e1308)) + bitcast<i32>(_e1280))) % _e1280);
                                        let _e1328 = select(select(u32(_e1309), 0u, (_e1309 < 0f)), 4294967295u, (_e1309 > 4294967000f));
                                        let _e1329 = (_e1328 + 1u);
                                        let _e1334 = global_1.member.member_2[2u];
                                        let _e1335 = (_e1328 * _e1280);
                                        if _e1322 {
                                            phi_32823_ = true;
                                            break;
                                        }
                                        let _e1336 = (_e1323 % _e1280);
                                        let _e1339 = (_e1334 + (3u * (_e1335 + _e1336)));
                                        let _e1341 = global_6.member[_e1339];
                                        let _e1344 = global_6.member[(_e1339 + 1u)];
                                        let _e1347 = global_6.member[(_e1339 + 2u)];
                                        if _e1322 {
                                            phi_32823_ = true;
                                            break;
                                        }
                                        let _e1350 = ((_e1323 + 1u) % _e1280);
                                        let _e1353 = (_e1334 + (3u * (_e1335 + _e1350)));
                                        let _e1355 = global_6.member[_e1353];
                                        let _e1358 = global_6.member[(_e1353 + 1u)];
                                        let _e1361 = global_6.member[(_e1353 + 2u)];
                                        let _e1369 = (vec3<f32>(_e1341, _e1344, _e1347) + vec3<f32>(((_e1355 - _e1341) * _e1310), ((_e1358 - _e1344) * _e1310), ((_e1361 - _e1347) * _e1310)));
                                        let _e1370 = (select(_e1329, _e1301, (_e1301 < _e1329)) * _e1280);
                                        if _e1322 {
                                            phi_32823_ = true;
                                            break;
                                        }
                                        let _e1373 = (_e1334 + (3u * (_e1370 + _e1336)));
                                        let _e1375 = global_6.member[_e1373];
                                        let _e1378 = global_6.member[(_e1373 + 1u)];
                                        let _e1381 = global_6.member[(_e1373 + 2u)];
                                        if _e1322 {
                                            phi_32823_ = true;
                                            break;
                                        }
                                        let _e1385 = (_e1334 + (3u * (_e1370 + _e1350)));
                                        let _e1387 = global_6.member[_e1385];
                                        let _e1390 = global_6.member[(_e1385 + 1u)];
                                        let _e1393 = global_6.member[(_e1385 + 2u)];
                                        let _e1401 = (vec3<f32>(_e1375, _e1378, _e1381) + vec3<f32>(((_e1387 - _e1375) * _e1310), ((_e1390 - _e1378) * _e1310), ((_e1393 - _e1381) * _e1310)));
                                        phi_849_ = (_e1369 + vec3<f32>(((_e1401.x - _e1369.x) * _e1311), ((_e1401.y - _e1369.y) * _e1311), ((_e1401.z - _e1369.z) * _e1311)));
                                    } else {
                                        phi_849_ = vec3<f32>();
                                    }
                                    let _e1417 = phi_849_;
                                    phi_851_ = _e1417;
                                    phi_852_ = select(true, false, _e1277);
                                } else {
                                    phi_851_ = vec3<f32>();
                                    phi_852_ = true;
                                }
                                let _e1420 = phi_851_;
                                let _e1422 = phi_852_;
                                if _e1422 {
                                    let _e1423 = (_e1275 != 4294967295u);
                                    if (_e1272 != 4294967295u) {
                                        phi_32437_ = _e306;
                                        phi_862_ = vec3<f32>(0f, 0f, 0f);
                                        phi_865_ = 0i;
                                        phi_867_ = _e1272;
                                        loop {
                                            let _e1426 = phi_32437_;
                                            let _e1428 = phi_862_;
                                            let _e1430 = phi_865_;
                                            let _e1432 = phi_867_;
                                            local_17 = _e1428;
                                            local_18 = _e1428;
                                            local_19 = _e1428;
                                            local_61 = _e1428;
                                            local_62 = _e1428;
                                            if (_e1432 != 4294967295u) {
                                                let _e1434 = (_e1430 < 2i);
                                                if _e1434 {
                                                    let _e1436 = global_6.member[_e1432];
                                                    let _e1439 = global_6.member[(_e1432 + 1u)];
                                                    let _e1442 = global_6.member[(_e1432 + 2u)];
                                                    let _e1445 = global_6.member[(_e1432 + 3u)];
                                                    let _e1448 = global_6.member[(_e1432 + 4u)];
                                                    let _e1451 = global_6.member[(_e1432 + 5u)];
                                                    let _e1454 = global_6.member[(_e1432 + 6u)];
                                                    let _e1457 = global_6.member[(_e1432 + 7u)];
                                                    let _e1460 = global_6.member[(_e1432 + 8u)];
                                                    let _e1463 = global_6.member[(_e1432 + 9u)];
                                                    let _e1466 = global_6.member[(_e1432 + 10u)];
                                                    let _e1469 = global_6.member[(_e1432 + 11u)];
                                                    let _e1472 = global_6.member[(_e1432 + 12u)];
                                                    let _e1475 = global_6.member[(_e1432 + 13u)];
                                                    let _e1478 = global_6.member[(_e1432 + 14u)];
                                                    let _e1481 = global_6.member[(_e1432 + 15u)];
                                                    let _e1485 = global_6.member[(_e1432 + 16u)];
                                                    let _e1488 = global_6.member[(_e1432 + 17u)];
                                                    let _e1491 = global_6.member[(_e1432 + 18u)];
                                                    let _e1494 = global_6.member[(_e1432 + 19u)];
                                                    let _e1497 = global_6.member[(_e1432 + 20u)];
                                                    let _e1500 = global_6.member[(_e1432 + 30u)];
                                                    let _e1501 = bitcast<u32>(_e1500);
                                                    let _e1504 = global_6.member[(_e1432 + 31u)];
                                                    let _e1505 = bitcast<u32>(_e1504);
                                                    let _e1508 = global_6.member[(_e1432 + 32u)];
                                                    let _e1510 = (_e317.member_1.y < 0f);
                                                    if _e1510 {
                                                        phi_2655_ = type_49();
                                                        phi_2656_ = true;
                                                        phi_2657_ = false;
                                                    } else {
                                                        let _e1511 = (_e1505 == 4294967295u);
                                                        if _e1511 {
                                                            phi_2653_ = type_49();
                                                            phi_2654_ = false;
                                                        } else {
                                                            let _e1525 = (_e317.member.x - _e1436);
                                                            let _e1526 = (_e317.member.y - _e1439);
                                                            let _e1527 = (_e317.member.z - _e1442);
                                                            let _e1535 = ((((((_e317.member.x * _e317.member.x) + (_e317.member.y * _e317.member.y)) + (_e317.member.z * _e317.member.z)) - (2f * (((_e317.member.x * _e1436) + (_e317.member.y * _e1439)) + (_e317.member.z * _e1442)))) + _e1451) / (sqrt((((_e1525 * _e1525) + (_e1526 * _e1526)) + (_e1527 * _e1527))) + _e1445));
                                                            let _e1536 = (_e1432 + 21u);
                                                            let _e1539 = global_6.member[(_e1432 + 28u)];
                                                            if (_e1535 > _e1539) {
                                                                phi_2643_ = type_52(0u, type_52().member_1);
                                                            } else {
                                                                let _e1542 = global_6.member[_e1536];
                                                                if (_e1535 != _e1535) {
                                                                    phi_21614_ = true;
                                                                } else {
                                                                    phi_21614_ = (_e1542 >= _e1535);
                                                                }
                                                                let _e1546 = phi_21614_;
                                                                let _e1547 = select(_e1535, _e1542, _e1546);
                                                                phi_1289_ = 0u;
                                                                loop {
                                                                    let _e1549 = phi_1289_;
                                                                    local_8 = _e1549;
                                                                    local_9 = _e1549;
                                                                    local_10 = _e1549;
                                                                    if (_e1549 < 8u) {
                                                                        let _e1553 = global_6.member[(_e1536 + _e1549)];
                                                                        let _e1554 = (_e1553 <= _e1547);
                                                                        if _e1554 {
                                                                            phi_1319_ = (_e1549 + 1u);
                                                                        } else {
                                                                            phi_1319_ = u32();
                                                                        }
                                                                        let _e1557 = phi_1319_;
                                                                        phi_1290_ = _e1557;
                                                                        phi_1320_ = _e1554;
                                                                    } else {
                                                                        phi_1290_ = u32();
                                                                        phi_1320_ = false;
                                                                    }
                                                                    let _e1559 = phi_1290_;
                                                                    let _e1561 = phi_1320_;
                                                                    phi_1289_ = _e1559;
                                                                    if !(_e1561) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1564 = local_8;
                                                                if (_e1564 < 1u) {
                                                                    phi_1338_ = 1u;
                                                                } else {
                                                                    let _e1567 = local_9;
                                                                    let _e1570 = local_10;
                                                                    phi_1338_ = select(_e1570, 7u, (_e1567 > 7u));
                                                                }
                                                                let _e1573 = phi_1338_;
                                                                let _e1574 = (_e1573 - 1u);
                                                                let _e1577 = global_6.member[(_e1536 + _e1574)];
                                                                let _e1580 = global_6.member[(_e1536 + _e1573)];
                                                                let _e1583 = ((_e1547 - _e1577) / (_e1580 - _e1577));
                                                                let _e1584 = select(_e317.member_1.y, 0f, _e1510);
                                                                let _e1590 = (sqrt((asin(select(_e1584, 1f, (_e1584 > 1f))) * 0.63661975f)) * 127f);
                                                                let _e1592 = (atan2(_e317.member_1.z, _e317.member_1.x) * 0.15915494f);
                                                                let _e1595 = ((_e1592 - floor(_e1592)) * 256f);
                                                                let _e1596 = floor(_e1595);
                                                                let _e1602 = (select(select(u32(_e1596), 0u, (_e1596 < 0f)), 4294967295u, (_e1596 > 4294967000f)) % 256u);
                                                                let _e1603 = floor(_e1590);
                                                                let _e1608 = select(select(u32(_e1603), 0u, (_e1603 < 0f)), 4294967295u, (_e1603 > 4294967000f));
                                                                let _e1610 = select(_e1608, 126u, (126u < _e1608));
                                                                let _e1612 = ((_e1602 + 1u) % 256u);
                                                                let _e1614 = (_e1595 - _e1596);
                                                                let _e1616 = (_e1590 - f32(_e1610));
                                                                let _e1618 = (_e1505 + (_e1574 * 294912u));
                                                                let _e1619 = (_e1610 * 256u);
                                                                let _e1621 = (9u * (_e1619 + _e1602));
                                                                let _e1622 = (_e1618 + _e1621);
                                                                let _e1624 = (9u * (_e1619 + _e1612));
                                                                let _e1625 = (_e1618 + _e1624);
                                                                let _e1626 = ((_e1610 + 1u) * 256u);
                                                                let _e1628 = (9u * (_e1626 + _e1602));
                                                                let _e1629 = (_e1618 + _e1628);
                                                                let _e1631 = (9u * (_e1626 + _e1612));
                                                                let _e1632 = (_e1618 + _e1631);
                                                                let _e1633 = (1f - _e1614);
                                                                let _e1634 = (1f - _e1616);
                                                                let _e1635 = (_e1633 * _e1634);
                                                                let _e1636 = (_e1614 * _e1634);
                                                                let _e1637 = (_e1633 * _e1616);
                                                                let _e1638 = (_e1614 * _e1616);
                                                                let _e1640 = global_6.member[_e1622];
                                                                let _e1643 = global_6.member[(_e1622 + 1u)];
                                                                let _e1646 = global_6.member[(_e1622 + 2u)];
                                                                let _e1651 = global_6.member[_e1625];
                                                                let _e1654 = global_6.member[(_e1625 + 1u)];
                                                                let _e1657 = global_6.member[(_e1625 + 2u)];
                                                                let _e1665 = global_6.member[_e1629];
                                                                let _e1668 = global_6.member[(_e1629 + 1u)];
                                                                let _e1671 = global_6.member[(_e1629 + 2u)];
                                                                let _e1679 = global_6.member[_e1632];
                                                                let _e1682 = global_6.member[(_e1632 + 1u)];
                                                                let _e1685 = global_6.member[(_e1632 + 2u)];
                                                                let _e1694 = global_6.member[(_e1622 + 3u)];
                                                                let _e1697 = global_6.member[(_e1622 + 4u)];
                                                                let _e1700 = global_6.member[(_e1622 + 5u)];
                                                                let _e1706 = global_6.member[(_e1625 + 3u)];
                                                                let _e1709 = global_6.member[(_e1625 + 4u)];
                                                                let _e1712 = global_6.member[(_e1625 + 5u)];
                                                                let _e1721 = global_6.member[(_e1629 + 3u)];
                                                                let _e1724 = global_6.member[(_e1629 + 4u)];
                                                                let _e1727 = global_6.member[(_e1629 + 5u)];
                                                                let _e1736 = global_6.member[(_e1632 + 3u)];
                                                                let _e1739 = global_6.member[(_e1632 + 4u)];
                                                                let _e1742 = global_6.member[(_e1632 + 5u)];
                                                                let _e1751 = global_6.member[(_e1622 + 6u)];
                                                                let _e1754 = global_6.member[(_e1622 + 7u)];
                                                                let _e1757 = global_6.member[(_e1622 + 8u)];
                                                                let _e1763 = global_6.member[(_e1625 + 6u)];
                                                                let _e1766 = global_6.member[(_e1625 + 7u)];
                                                                let _e1769 = global_6.member[(_e1625 + 8u)];
                                                                let _e1778 = global_6.member[(_e1629 + 6u)];
                                                                let _e1781 = global_6.member[(_e1629 + 7u)];
                                                                let _e1784 = global_6.member[(_e1629 + 8u)];
                                                                let _e1793 = global_6.member[(_e1632 + 6u)];
                                                                let _e1796 = global_6.member[(_e1632 + 7u)];
                                                                let _e1799 = global_6.member[(_e1632 + 8u)];
                                                                let _e1807 = (_e1505 + (_e1573 * 294912u));
                                                                let _e1808 = (_e1807 + _e1621);
                                                                let _e1809 = (_e1807 + _e1624);
                                                                let _e1810 = (_e1807 + _e1628);
                                                                let _e1811 = (_e1807 + _e1631);
                                                                let _e1813 = global_6.member[_e1808];
                                                                let _e1816 = global_6.member[(_e1808 + 1u)];
                                                                let _e1819 = global_6.member[(_e1808 + 2u)];
                                                                let _e1824 = global_6.member[_e1809];
                                                                let _e1827 = global_6.member[(_e1809 + 1u)];
                                                                let _e1830 = global_6.member[(_e1809 + 2u)];
                                                                let _e1838 = global_6.member[_e1810];
                                                                let _e1841 = global_6.member[(_e1810 + 1u)];
                                                                let _e1844 = global_6.member[(_e1810 + 2u)];
                                                                let _e1852 = global_6.member[_e1811];
                                                                let _e1855 = global_6.member[(_e1811 + 1u)];
                                                                let _e1858 = global_6.member[(_e1811 + 2u)];
                                                                let _e1867 = global_6.member[(_e1808 + 3u)];
                                                                let _e1870 = global_6.member[(_e1808 + 4u)];
                                                                let _e1873 = global_6.member[(_e1808 + 5u)];
                                                                let _e1879 = global_6.member[(_e1809 + 3u)];
                                                                let _e1882 = global_6.member[(_e1809 + 4u)];
                                                                let _e1885 = global_6.member[(_e1809 + 5u)];
                                                                let _e1894 = global_6.member[(_e1810 + 3u)];
                                                                let _e1897 = global_6.member[(_e1810 + 4u)];
                                                                let _e1900 = global_6.member[(_e1810 + 5u)];
                                                                let _e1909 = global_6.member[(_e1811 + 3u)];
                                                                let _e1912 = global_6.member[(_e1811 + 4u)];
                                                                let _e1915 = global_6.member[(_e1811 + 5u)];
                                                                let _e1924 = global_6.member[(_e1808 + 6u)];
                                                                let _e1927 = global_6.member[(_e1808 + 7u)];
                                                                let _e1930 = global_6.member[(_e1808 + 8u)];
                                                                let _e1936 = global_6.member[(_e1809 + 6u)];
                                                                let _e1939 = global_6.member[(_e1809 + 7u)];
                                                                let _e1942 = global_6.member[(_e1809 + 8u)];
                                                                let _e1951 = global_6.member[(_e1810 + 6u)];
                                                                let _e1954 = global_6.member[(_e1810 + 7u)];
                                                                let _e1957 = global_6.member[(_e1810 + 8u)];
                                                                let _e1966 = global_6.member[(_e1811 + 6u)];
                                                                let _e1969 = global_6.member[(_e1811 + 7u)];
                                                                let _e1972 = global_6.member[(_e1811 + 8u)];
                                                                let _e1979 = (1f - _e1583);
                                                                phi_2643_ = type_52(1u, type_49(vec3<f32>(((((((_e1640 * _e1635) + (_e1651 * _e1636)) + (_e1665 * _e1637)) + (_e1679 * _e1638)) * _e1979) + (((((_e1813 * _e1635) + (_e1824 * _e1636)) + (_e1838 * _e1637)) + (_e1852 * _e1638)) * _e1583)), ((((((_e1643 * _e1635) + (_e1654 * _e1636)) + (_e1668 * _e1637)) + (_e1682 * _e1638)) * _e1979) + (((((_e1816 * _e1635) + (_e1827 * _e1636)) + (_e1841 * _e1637)) + (_e1855 * _e1638)) * _e1583)), ((((((_e1646 * _e1635) + (_e1657 * _e1636)) + (_e1671 * _e1637)) + (_e1685 * _e1638)) * _e1979) + (((((_e1819 * _e1635) + (_e1830 * _e1636)) + (_e1844 * _e1637)) + (_e1858 * _e1638)) * _e1583))), vec3<f32>(((((((_e1694 * _e1635) + (_e1706 * _e1636)) + (_e1721 * _e1637)) + (_e1736 * _e1638)) * _e1979) + (((((_e1867 * _e1635) + (_e1879 * _e1636)) + (_e1894 * _e1637)) + (_e1909 * _e1638)) * _e1583)), ((((((_e1697 * _e1635) + (_e1709 * _e1636)) + (_e1724 * _e1637)) + (_e1739 * _e1638)) * _e1979) + (((((_e1870 * _e1635) + (_e1882 * _e1636)) + (_e1897 * _e1637)) + (_e1912 * _e1638)) * _e1583)), ((((((_e1700 * _e1635) + (_e1712 * _e1636)) + (_e1727 * _e1637)) + (_e1742 * _e1638)) * _e1979) + (((((_e1873 * _e1635) + (_e1885 * _e1636)) + (_e1900 * _e1637)) + (_e1915 * _e1638)) * _e1583))), vec3<f32>(((((((_e1751 * _e1635) + (_e1763 * _e1636)) + (_e1778 * _e1637)) + (_e1793 * _e1638)) * _e1979) + (((((_e1924 * _e1635) + (_e1936 * _e1636)) + (_e1951 * _e1637)) + (_e1966 * _e1638)) * _e1583)), ((((((_e1754 * _e1635) + (_e1766 * _e1636)) + (_e1781 * _e1637)) + (_e1796 * _e1638)) * _e1979) + (((((_e1927 * _e1635) + (_e1939 * _e1636)) + (_e1954 * _e1637)) + (_e1969 * _e1638)) * _e1583)), ((((((_e1757 * _e1635) + (_e1769 * _e1636)) + (_e1784 * _e1637)) + (_e1799 * _e1638)) * _e1979) + (((((_e1930 * _e1635) + (_e1942 * _e1636)) + (_e1957 * _e1637)) + (_e1972 * _e1638)) * _e1583)))));
                                                            }
                                                            let _e2016 = phi_2643_;
                                                            let _e2019 = (bitcast<i32>(_e2016.member) != 0i);
                                                            if _e2019 {
                                                                phi_2651_ = _e2016.member_1;
                                                            } else {
                                                                phi_2651_ = type_49();
                                                            }
                                                            let _e2022 = phi_2651_;
                                                            phi_2653_ = _e2022;
                                                            phi_2654_ = select(true, false, _e2019);
                                                        }
                                                        let _e2025 = phi_2653_;
                                                        let _e2027 = phi_2654_;
                                                        phi_2655_ = _e2025;
                                                        phi_2656_ = _e1511;
                                                        phi_2657_ = _e2027;
                                                    }
                                                    let _e2029 = phi_2655_;
                                                    let _e2031 = phi_2656_;
                                                    let _e2033 = phi_2657_;
                                                    if select(_e2033, true, _e2031) {
                                                        let _e2042 = ((((_e317.member.x - _e1436) * _e317.member_1.x) + ((_e317.member.y - _e1439) * _e317.member_1.y)) + ((_e317.member.z - _e1442) * _e317.member_1.z));
                                                        let _e2054 = ((((_e317.member.x * _e317.member.x) + (_e317.member.y * _e317.member.y)) + (_e317.member.z * _e317.member.z)) - (2f * (((_e317.member.x * _e1436) + (_e317.member.y * _e1439)) + (_e317.member.z * _e1442))));
                                                        let _e2055 = (_e2054 + _e1454);
                                                        let _e2056 = (_e2042 * _e2042);
                                                        let _e2057 = (_e2056 - _e2055);
                                                        if (_e2057 < 0f) {
                                                            phi_21753_ = type_54(0u, type_54().member_1);
                                                            phi_21754_ = type_54();
                                                            phi_21755_ = true;
                                                        } else {
                                                            if (_e2042 < 0f) {
                                                                phi_21725_ = -(sqrt(_e2057));
                                                            } else {
                                                                phi_21725_ = sqrt(_e2057);
                                                            }
                                                            let _e2064 = phi_21725_;
                                                            let _e2066 = -((_e2042 + _e2064));
                                                            let _e2067 = (_e2066 == 0f);
                                                            if _e2067 {
                                                                phi_21748_ = type_54(1u, type_53(0f, 0f));
                                                                phi_21749_ = type_54();
                                                            } else {
                                                                let _e2068 = (_e2055 / _e2066);
                                                                if (_e2066 < _e2068) {
                                                                    phi_21737_ = type_53(_e2066, _e2068);
                                                                } else {
                                                                    phi_21737_ = type_53(_e2068, _e2066);
                                                                }
                                                                let _e2073 = phi_21737_;
                                                                phi_21748_ = type_54();
                                                                phi_21749_ = type_54(1u, _e2073);
                                                            }
                                                            let _e2076 = phi_21748_;
                                                            let _e2078 = phi_21749_;
                                                            phi_21753_ = _e2076;
                                                            phi_21754_ = _e2078;
                                                            phi_21755_ = _e2067;
                                                        }
                                                        let _e2083 = phi_21753_;
                                                        let _e2085 = phi_21754_;
                                                        let _e2087 = phi_21755_;
                                                        if _e2087 {
                                                            phi_21759_ = _e2083;
                                                        } else {
                                                            phi_21759_ = _e2085;
                                                        }
                                                        let _e2089 = phi_21759_;
                                                        if (bitcast<i32>(_e2089.member) != 0i) {
                                                            phi_21646_ = type_54(0u, _e2089.member_1);
                                                        } else {
                                                            phi_21646_ = type_54(1u, type_54().member_1);
                                                        }
                                                        let _e2099 = phi_21646_;
                                                        if (bitcast<i32>(_e2099.member) != 0i) {
                                                            phi_21693_ = type_54(0u, type_54().member_1);
                                                        } else {
                                                            if (_e2099.member_1.member_1 <= 0f) {
                                                                phi_21689_ = type_54(0u, type_54().member_1);
                                                            } else {
                                                                if (_e2099.member_1.member != _e2099.member_1.member) {
                                                                    phi_21797_ = true;
                                                                } else {
                                                                    phi_21797_ = (0f >= _e2099.member_1.member);
                                                                }
                                                                let _e2111 = phi_21797_;
                                                                let _e2112 = select(_e2099.member_1.member, 0f, _e2111);
                                                                let _e2113 = (_e2054 + _e1451);
                                                                if (_e2113 > 0f) {
                                                                    let _e2115 = (_e2056 - _e2113);
                                                                    if (_e2115 < 0f) {
                                                                        phi_21905_ = type_54(0u, type_54().member_1);
                                                                        phi_21906_ = type_54();
                                                                        phi_21907_ = true;
                                                                    } else {
                                                                        if (_e2042 < 0f) {
                                                                            phi_21877_ = -(sqrt(_e2115));
                                                                        } else {
                                                                            phi_21877_ = sqrt(_e2115);
                                                                        }
                                                                        let _e2122 = phi_21877_;
                                                                        let _e2124 = -((_e2042 + _e2122));
                                                                        let _e2125 = (_e2124 == 0f);
                                                                        if _e2125 {
                                                                            phi_21900_ = type_54(1u, type_53(0f, 0f));
                                                                            phi_21901_ = type_54();
                                                                        } else {
                                                                            let _e2126 = (_e2113 / _e2124);
                                                                            if (_e2124 < _e2126) {
                                                                                phi_21889_ = type_53(_e2124, _e2126);
                                                                            } else {
                                                                                phi_21889_ = type_53(_e2126, _e2124);
                                                                            }
                                                                            let _e2131 = phi_21889_;
                                                                            phi_21900_ = type_54();
                                                                            phi_21901_ = type_54(1u, _e2131);
                                                                        }
                                                                        let _e2134 = phi_21900_;
                                                                        let _e2136 = phi_21901_;
                                                                        phi_21905_ = _e2134;
                                                                        phi_21906_ = _e2136;
                                                                        phi_21907_ = _e2125;
                                                                    }
                                                                    let _e2141 = phi_21905_;
                                                                    let _e2143 = phi_21906_;
                                                                    let _e2145 = phi_21907_;
                                                                    if _e2145 {
                                                                        phi_21911_ = _e2141;
                                                                    } else {
                                                                        phi_21911_ = _e2143;
                                                                    }
                                                                    let _e2147 = phi_21911_;
                                                                    if (bitcast<i32>(_e2147.member) != 0i) {
                                                                        phi_21845_ = select(_e2147.member_1.member_1, _e2147.member_1.member, (_e2147.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_21845_ = -1f;
                                                                    }
                                                                    let _e2158 = phi_21845_;
                                                                    if (_e2158 >= 0f) {
                                                                        if (_e2099.member_1.member_1 != _e2099.member_1.member_1) {
                                                                            phi_21949_ = true;
                                                                        } else {
                                                                            phi_21949_ = (_e2158 <= _e2099.member_1.member_1);
                                                                        }
                                                                        let _e2163 = phi_21949_;
                                                                        phi_21674_ = select(_e2099.member_1.member_1, _e2158, _e2163);
                                                                    } else {
                                                                        phi_21674_ = _e2099.member_1.member_1;
                                                                    }
                                                                    let _e2166 = phi_21674_;
                                                                    phi_21676_ = _e2166;
                                                                } else {
                                                                    phi_21676_ = _e2099.member_1.member_1;
                                                                }
                                                                let _e2168 = phi_21676_;
                                                                if (_e2168 <= _e2112) {
                                                                    phi_21685_ = type_54(0u, type_54().member_1);
                                                                } else {
                                                                    phi_21685_ = type_54(1u, type_53(_e2112, _e2168));
                                                                }
                                                                let _e2176 = phi_21685_;
                                                                phi_21689_ = _e2176;
                                                            }
                                                            let _e2181 = phi_21689_;
                                                            phi_21693_ = _e2181;
                                                        }
                                                        let _e2186 = phi_21693_;
                                                        if (bitcast<i32>(_e2186.member) != 0i) {
                                                            phi_2675_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2678_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2680_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2682_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2684_ = 0u;
                                                            loop {
                                                                let _e2195 = phi_2675_;
                                                                let _e2197 = phi_2678_;
                                                                let _e2199 = phi_2680_;
                                                                let _e2201 = phi_2682_;
                                                                let _e2203 = phi_2684_;
                                                                local_12 = _e2201;
                                                                local_13 = _e2199;
                                                                local_14 = _e2197;
                                                                let _e2204 = (_e2203 < 32u);
                                                                if _e2204 {
                                                                    let _e2205 = -(_e2042);
                                                                    if (_e2186.member_1.member <= _e2186.member_1.member_1) {
                                                                    } else {
                                                                        phi_32405_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2208 = select(_e2205, _e2186.member_1.member, (_e2205 < _e2186.member_1.member));
                                                                    let _e2210 = select(_e2208, _e2186.member_1.member_1, (_e2208 > _e2186.member_1.member_1));
                                                                    let _e2211 = (_e2210 - _e2186.member_1.member);
                                                                    let _e2212 = (_e2186.member_1.member_1 - _e2210);
                                                                    let _e2213 = (_e2186.member_1.member_1 - _e2186.member_1.member);
                                                                    let _e2214 = (0.000000001f * _e2213);
                                                                    if (_e2211 <= _e2214) {
                                                                        let _e2289 = (f32(_e2203) * 0.03125f);
                                                                        let _e2292 = (f32((_e2203 + 1u)) * 0.03125f);
                                                                        let _e2294 = ((_e2289 + _e2292) * 0.5f);
                                                                        phi_2785_ = type_53((_e2186.member_1.member + ((_e2213 * _e2294) * _e2294)), (_e2213 * ((_e2292 * _e2292) - (_e2289 * _e2289))));
                                                                    } else {
                                                                        if (_e2212 <= _e2214) {
                                                                            let _e2272 = (f32((31u - _e2203)) * 0.03125f);
                                                                            let _e2275 = (f32((32u - _e2203)) * 0.03125f);
                                                                            let _e2277 = ((_e2272 + _e2275) * 0.5f);
                                                                            phi_2784_ = type_53((_e2186.member_1.member_1 - ((_e2213 * _e2277) * _e2277)), (_e2213 * ((_e2275 * _e2275) - (_e2272 * _e2272))));
                                                                        } else {
                                                                            let _e2220 = round(((32f * _e2211) / (_e2211 + _e2212)));
                                                                            let _e2225 = select(select(u32(_e2220), 0u, (_e2220 < 0f)), 4294967295u, (_e2220 > 4294967000f));
                                                                            if (_e2225 < 1u) {
                                                                                phi_2768_ = 1u;
                                                                            } else {
                                                                                phi_2768_ = select(_e2225, 31u, (_e2225 > 31u));
                                                                            }
                                                                            let _e2230 = phi_2768_;
                                                                            if (_e2203 < _e2230) {
                                                                                let _e2251 = ((_e2230 - 1u) - _e2203);
                                                                                let _e2253 = f32(_e2230);
                                                                                let _e2254 = (f32(_e2251) / _e2253);
                                                                                let _e2257 = (f32((_e2251 + 1u)) / _e2253);
                                                                                let _e2259 = ((_e2254 + _e2257) * 0.5f);
                                                                                phi_2783_ = type_53((_e2210 - ((_e2211 * _e2259) * _e2259)), (_e2211 * ((_e2257 * _e2257) - (_e2254 * _e2254))));
                                                                            } else {
                                                                                let _e2233 = (_e2203 - _e2230);
                                                                                let _e2235 = f32((32u - _e2230));
                                                                                let _e2236 = (f32(_e2233) / _e2235);
                                                                                let _e2239 = (f32((_e2233 + 1u)) / _e2235);
                                                                                let _e2241 = ((_e2236 + _e2239) * 0.5f);
                                                                                phi_2783_ = type_53((_e2210 + ((_e2212 * _e2241) * _e2241)), (_e2212 * ((_e2239 * _e2239) - (_e2236 * _e2236))));
                                                                            }
                                                                            let _e2269 = phi_2783_;
                                                                            phi_2784_ = _e2269;
                                                                        }
                                                                        let _e2287 = phi_2784_;
                                                                        phi_2785_ = _e2287;
                                                                    }
                                                                    let _e2304 = phi_2785_;
                                                                    let _e2310 = (_e317.member.x + (_e317.member_1.x * _e2304.member));
                                                                    let _e2311 = (_e317.member.y + (_e317.member_1.y * _e2304.member));
                                                                    let _e2312 = (_e317.member.z + (_e317.member_1.z * _e2304.member));
                                                                    let _e2325 = ((((_e2310 * _e2310) + (_e2311 * _e2311)) + (_e2312 * _e2312)) - (2f * (((_e2310 * _e1436) + (_e2311 * _e1439)) + (_e2312 * _e1442))));
                                                                    let _e2326 = (_e2325 + _e1451);
                                                                    let _e2327 = (_e2310 - _e1436);
                                                                    let _e2328 = (_e2311 - _e1439);
                                                                    let _e2329 = (_e2312 - _e1442);
                                                                    let _e2335 = sqrt((((_e2327 * _e2327) + (_e2328 * _e2328)) + (_e2329 * _e2329)));
                                                                    let _e2337 = (_e2326 / (_e2335 + _e1445));
                                                                    let _e2341 = (1f - (abs((_e2337 - 25000f)) * 0.00006666667f));
                                                                    if (_e2341 != _e2341) {
                                                                        phi_22148_ = true;
                                                                    } else {
                                                                        phi_22148_ = (0f >= _e2341);
                                                                    }
                                                                    let _e2345 = phi_22148_;
                                                                    let _e2347 = -(_e2337);
                                                                    let _e2350 = (exp((_e2347 / _e1466)) * _e2304.member_1);
                                                                    let _e2353 = (exp((_e2347 / _e1469)) * _e2304.member_1);
                                                                    let _e2355 = (_e2350 * 0.5f);
                                                                    let _e2356 = (_e2353 * 0.5f);
                                                                    let _e2357 = ((select(_e2341, 0f, _e2345) * _e2304.member_1) * 0.5f);
                                                                    let _e2359 = (_e2195.x + _e2355);
                                                                    let _e2361 = (_e2195.y + _e2356);
                                                                    let _e2363 = (_e2195.z + _e2357);
                                                                    let _e2368 = (_e1485 * 1.1f);
                                                                    let _e2369 = (_e2368 * _e2361);
                                                                    let _e2388 = (_e1485 * _e2353);
                                                                    if (_e1501 == 4294967295u) {
                                                                        phi_3113_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2396 = (1f / _e2335);
                                                                        let _e2404 = ((((_e2327 * _e2396) * _e1457) + ((_e2328 * _e2396) * _e1460)) + ((_e2329 * _e2396) * _e1463));
                                                                        if (_e2337 != _e2337) {
                                                                            phi_22275_ = true;
                                                                        } else {
                                                                            phi_22275_ = (0f >= _e2337);
                                                                        }
                                                                        let _e2408 = phi_22275_;
                                                                        let _e2412 = sqrt((select(_e2337, 0f, _e2408) / (_e1448 - _e1445)));
                                                                        if (_e2412 != _e2412) {
                                                                            phi_22290_ = true;
                                                                        } else {
                                                                            phi_22290_ = (1f <= _e2412);
                                                                        }
                                                                        let _e2416 = phi_22290_;
                                                                        let _e2418 = (select(_e2412, 1f, _e2416) * 31f);
                                                                        let _e2420 = select(_e2404, -1f, (_e2404 < -1f));
                                                                        let _e2424 = ((select(_e2420, 1f, (_e2420 > 1f)) + 1f) * 15.5f);
                                                                        let _e2429 = select(select(u32(_e2418), 0u, (_e2418 < 0f)), 4294967295u, (_e2418 > 4294967000f));
                                                                        let _e2431 = select(_e2429, 30u, (30u < _e2429));
                                                                        let _e2436 = select(select(u32(_e2424), 0u, (_e2424 < 0f)), 4294967295u, (_e2424 > 4294967000f));
                                                                        let _e2438 = select(_e2436, 30u, (30u < _e2436));
                                                                        let _e2440 = (_e2418 - f32(_e2431));
                                                                        let _e2442 = (_e2424 - f32(_e2438));
                                                                        let _e2446 = (_e1501 + (3u * ((_e2431 * 32u) + _e2438)));
                                                                        let _e2451 = (_e1501 + (3u * (((_e2431 + 1u) * 32u) + _e2438)));
                                                                        let _e2453 = global_6.member[_e2446];
                                                                        let _e2456 = global_6.member[(_e2446 + 1u)];
                                                                        let _e2459 = global_6.member[(_e2446 + 2u)];
                                                                        let _e2460 = (1f - _e2442);
                                                                        let _e2466 = global_6.member[(_e2446 + 3u)];
                                                                        let _e2469 = global_6.member[(_e2446 + 4u)];
                                                                        let _e2472 = global_6.member[(_e2446 + 5u)];
                                                                        let _e2479 = (1f - _e2440);
                                                                        let _e2484 = global_6.member[_e2451];
                                                                        let _e2487 = global_6.member[(_e2451 + 1u)];
                                                                        let _e2490 = global_6.member[(_e2451 + 2u)];
                                                                        let _e2496 = global_6.member[(_e2451 + 3u)];
                                                                        let _e2499 = global_6.member[(_e2451 + 4u)];
                                                                        let _e2502 = global_6.member[(_e2451 + 5u)];
                                                                        phi_3113_ = vec3<f32>(((((_e2453 * _e2460) + (_e2466 * _e2442)) * _e2479) + (((_e2484 * _e2460) + (_e2496 * _e2442)) * _e2440)), ((((_e2456 * _e2460) + (_e2469 * _e2442)) * _e2479) + (((_e2487 * _e2460) + (_e2499 * _e2442)) * _e2440)), ((((_e2459 * _e2460) + (_e2472 * _e2442)) * _e2479) + (((_e2490 * _e2460) + (_e2502 * _e2442)) * _e2440)));
                                                                    }
                                                                    let _e2517 = phi_3113_;
                                                                    let _e2535 = (((_e2327 * _e1457) + (_e2328 * _e1460)) + (_e2329 * _e1463));
                                                                    let _e2536 = (_e2535 * _e2535);
                                                                    let _e2537 = (_e2536 - _e2326);
                                                                    if (_e2537 < 0f) {
                                                                        phi_22440_ = type_54(0u, type_54().member_1);
                                                                        phi_22441_ = type_54();
                                                                        phi_22442_ = true;
                                                                    } else {
                                                                        if (_e2535 < 0f) {
                                                                            phi_22412_ = -(sqrt(_e2537));
                                                                        } else {
                                                                            phi_22412_ = sqrt(_e2537);
                                                                        }
                                                                        let _e2544 = phi_22412_;
                                                                        let _e2546 = -((_e2535 + _e2544));
                                                                        let _e2547 = (_e2546 == 0f);
                                                                        if _e2547 {
                                                                            phi_22435_ = type_54(1u, type_53(0f, 0f));
                                                                            phi_22436_ = type_54();
                                                                        } else {
                                                                            let _e2548 = (_e2326 / _e2546);
                                                                            if (_e2546 < _e2548) {
                                                                                phi_22424_ = type_53(_e2546, _e2548);
                                                                            } else {
                                                                                phi_22424_ = type_53(_e2548, _e2546);
                                                                            }
                                                                            let _e2553 = phi_22424_;
                                                                            phi_22435_ = type_54();
                                                                            phi_22436_ = type_54(1u, _e2553);
                                                                        }
                                                                        let _e2556 = phi_22435_;
                                                                        let _e2558 = phi_22436_;
                                                                        phi_22440_ = _e2556;
                                                                        phi_22441_ = _e2558;
                                                                        phi_22442_ = _e2547;
                                                                    }
                                                                    let _e2563 = phi_22440_;
                                                                    let _e2565 = phi_22441_;
                                                                    let _e2567 = phi_22442_;
                                                                    if _e2567 {
                                                                        phi_22446_ = _e2563;
                                                                    } else {
                                                                        phi_22446_ = _e2565;
                                                                    }
                                                                    let _e2569 = phi_22446_;
                                                                    if (bitcast<i32>(_e2569.member) != 0i) {
                                                                        phi_22380_ = select(_e2569.member_1.member_1, _e2569.member_1.member, (_e2569.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_22380_ = -1f;
                                                                    }
                                                                    let _e2580 = phi_22380_;
                                                                    if (_e2580 >= 0f) {
                                                                        phi_22363_ = type_55(0u, type_55().member_1);
                                                                    } else {
                                                                        let _e2582 = (_e2325 + _e1454);
                                                                        let _e2583 = (_e2536 - _e2582);
                                                                        if (_e2583 < 0f) {
                                                                            phi_22550_ = type_54(0u, type_54().member_1);
                                                                            phi_22551_ = type_54();
                                                                            phi_22552_ = true;
                                                                        } else {
                                                                            if (_e2535 < 0f) {
                                                                                phi_22522_ = -(sqrt(_e2583));
                                                                            } else {
                                                                                phi_22522_ = sqrt(_e2583);
                                                                            }
                                                                            let _e2590 = phi_22522_;
                                                                            let _e2592 = -((_e2535 + _e2590));
                                                                            let _e2593 = (_e2592 == 0f);
                                                                            if _e2593 {
                                                                                phi_22545_ = type_54(1u, type_53(0f, 0f));
                                                                                phi_22546_ = type_54();
                                                                            } else {
                                                                                let _e2594 = (_e2582 / _e2592);
                                                                                if (_e2592 < _e2594) {
                                                                                    phi_22534_ = type_53(_e2592, _e2594);
                                                                                } else {
                                                                                    phi_22534_ = type_53(_e2594, _e2592);
                                                                                }
                                                                                let _e2599 = phi_22534_;
                                                                                phi_22545_ = type_54();
                                                                                phi_22546_ = type_54(1u, _e2599);
                                                                            }
                                                                            let _e2602 = phi_22545_;
                                                                            let _e2604 = phi_22546_;
                                                                            phi_22550_ = _e2602;
                                                                            phi_22551_ = _e2604;
                                                                            phi_22552_ = _e2593;
                                                                        }
                                                                        let _e2609 = phi_22550_;
                                                                        let _e2611 = phi_22551_;
                                                                        let _e2613 = phi_22552_;
                                                                        if _e2613 {
                                                                            phi_22556_ = _e2609;
                                                                        } else {
                                                                            phi_22556_ = _e2611;
                                                                        }
                                                                        let _e2615 = phi_22556_;
                                                                        if (bitcast<i32>(_e2615.member) != 0i) {
                                                                            phi_22490_ = select(_e2615.member_1.member_1, _e2615.member_1.member, (_e2615.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_22490_ = -1f;
                                                                        }
                                                                        let _e2626 = phi_22490_;
                                                                        if (_e2626 < 0f) {
                                                                            phi_22359_ = type_55(0u, type_55().member_1);
                                                                        } else {
                                                                            phi_22317_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_22318_ = 0u;
                                                                            loop {
                                                                                let _e2629 = phi_22317_;
                                                                                let _e2631 = phi_22318_;
                                                                                local_11 = _e2629;
                                                                                let _e2632 = (_e2631 < 16u);
                                                                                if _e2632 {
                                                                                    let _e2634 = (f32(_e2631) * 0.0625f);
                                                                                    let _e2635 = (_e2631 + 1u);
                                                                                    let _e2637 = (f32(_e2635) * 0.0625f);
                                                                                    let _e2639 = ((_e2634 + _e2637) * 0.5f);
                                                                                    let _e2641 = ((_e2626 * _e2639) * _e2639);
                                                                                    let _e2645 = (_e2626 * ((_e2637 * _e2637) - (_e2634 * _e2634)));
                                                                                    let _e2650 = (vec3<f32>(_e2310, _e2311, _e2312) + vec3<f32>((_e1457 * _e2641), (_e1460 * _e2641), (_e1463 * _e2641)));
                                                                                    let _e2667 = (_e2650.x - _e1436);
                                                                                    let _e2668 = (_e2650.y - _e1439);
                                                                                    let _e2669 = (_e2650.z - _e1442);
                                                                                    let _e2677 = ((((((_e2650.x * _e2650.x) + (_e2650.y * _e2650.y)) + (_e2650.z * _e2650.z)) - (2f * (((_e2650.x * _e1436) + (_e2650.y * _e1439)) + (_e2650.z * _e1442)))) + _e1451) / (sqrt((((_e2667 * _e2667) + (_e2668 * _e2668)) + (_e2669 * _e2669))) + _e1445));
                                                                                    let _e2681 = (1f - (abs((_e2677 - 25000f)) * 0.00006666667f));
                                                                                    if (_e2681 != _e2681) {
                                                                                        phi_22688_ = true;
                                                                                    } else {
                                                                                        phi_22688_ = (0f >= _e2681);
                                                                                    }
                                                                                    let _e2685 = phi_22688_;
                                                                                    let _e2687 = -(_e2677);
                                                                                    phi_22351_ = (_e2629 + vec3<f32>((exp((_e2687 / _e1466)) * _e2645), (exp((_e2687 / _e1469)) * _e2645), (select(_e2681, 0f, _e2685) * _e2645)));
                                                                                    phi_22352_ = _e2635;
                                                                                } else {
                                                                                    phi_22351_ = vec3<f32>();
                                                                                    phi_22352_ = u32();
                                                                                }
                                                                                let _e2698 = phi_22351_;
                                                                                let _e2700 = phi_22352_;
                                                                                phi_22317_ = _e2698;
                                                                                phi_22318_ = _e2700;
                                                                                if !(_e2632) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2703 = local_11;
                                                                            phi_22359_ = type_55(1u, _e2703);
                                                                        }
                                                                        let _e2709 = phi_22359_;
                                                                        phi_22363_ = _e2709;
                                                                    }
                                                                    let _e2714 = phi_22363_;
                                                                    if (bitcast<i32>(_e2714.member) != 0i) {
                                                                        let _e2719 = (vec3<f32>(_e2359, _e2361, _e2363) + _e2714.member_1);
                                                                        let _e2725 = (_e2368 * _e2719.y);
                                                                        let _e2737 = exp(-((((_e1475 * _e2719.x) + _e2725) + (_e1491 * _e2719.z))));
                                                                        let _e2739 = exp(-((((_e1478 * _e2719.x) + _e2725) + (_e1494 * _e2719.z))));
                                                                        let _e2741 = exp(-((((_e1481 * _e2719.x) + _e2725) + (_e1497 * _e2719.z))));
                                                                        phi_3172_ = vec3<f32>((_e2199.x + (_e2737 * _e2353)), (_e2199.y + (_e2739 * _e2353)), (_e2199.z + (_e2741 * _e2353)));
                                                                        phi_3173_ = vec3<f32>((_e2201.x + (_e2737 * _e2350)), (_e2201.y + (_e2739 * _e2350)), (_e2201.z + (_e2741 * _e2350)));
                                                                    } else {
                                                                        phi_3172_ = _e2199;
                                                                        phi_3173_ = _e2201;
                                                                    }
                                                                    let _e2763 = phi_3172_;
                                                                    let _e2765 = phi_3173_;
                                                                    phi_2676_ = vec3<f32>((_e2359 + _e2355), (_e2361 + _e2356), (_e2363 + _e2357));
                                                                    phi_2679_ = vec3<f32>((_e2197.x + ((exp(-((((_e1475 * _e2359) + _e2369) + (_e1491 * _e2363)))) * ((_e1475 * _e2350) + _e2388)) * _e2517.x)), (_e2197.y + ((exp(-((((_e1478 * _e2359) + _e2369) + (_e1494 * _e2363)))) * ((_e1478 * _e2350) + _e2388)) * _e2517.y)), (_e2197.z + ((exp(-((((_e1481 * _e2359) + _e2369) + (_e1497 * _e2363)))) * ((_e1481 * _e2350) + _e2388)) * _e2517.z)));
                                                                    phi_2681_ = _e2763;
                                                                    phi_2683_ = _e2765;
                                                                    phi_2685_ = (_e2203 + 1u);
                                                                } else {
                                                                    phi_2676_ = vec3<f32>();
                                                                    phi_2679_ = vec3<f32>();
                                                                    phi_2681_ = vec3<f32>();
                                                                    phi_2683_ = vec3<f32>();
                                                                    phi_2685_ = u32();
                                                                }
                                                                let _e2772 = phi_2676_;
                                                                let _e2774 = phi_2679_;
                                                                let _e2776 = phi_2681_;
                                                                let _e2778 = phi_2683_;
                                                                let _e2780 = phi_2685_;
                                                                phi_2675_ = _e2772;
                                                                phi_2678_ = _e2774;
                                                                phi_2680_ = _e2776;
                                                                phi_2682_ = _e2778;
                                                                phi_2684_ = _e2780;
                                                                phi_32405_ = _e1426;
                                                                if !(_e2204) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2783 = phi_32405_;
                                                            phi_32495_ = _e2783;
                                                            if _e2783 {
                                                                break;
                                                            }
                                                            let _e2785 = local_12;
                                                            let _e2788 = local_13;
                                                            let _e2791 = local_14;
                                                            phi_32499_ = _e2783;
                                                            phi_3193_ = type_49((_e2785 * vec3<f32>(_e1475, _e1478, _e1481)), (_e2788 * _e1485), _e2791);
                                                        } else {
                                                            phi_32499_ = _e1426;
                                                            phi_3193_ = type_49(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2794 = phi_32499_;
                                                        let _e2796 = phi_3193_;
                                                        phi_32498_ = _e2794;
                                                        phi_3194_ = _e2796;
                                                    } else {
                                                        phi_32498_ = _e1426;
                                                        phi_3194_ = _e2029;
                                                    }
                                                    let _e2798 = phi_32498_;
                                                    let _e2800 = phi_3194_;
                                                    let _e2808 = (((_e317.member_1.x * _e1457) + (_e317.member_1.y * _e1460)) + (_e317.member_1.z * _e1463));
                                                    let _e2810 = (1f + (_e2808 * _e2808));
                                                    let _e2811 = (0.059683103f * _e2810);
                                                    let _e2818 = (_e1488 * _e1488);
                                                    let _e2829 = ((0.119366206f * ((1f - _e2818) * _e2810)) / ((2f + _e2818) * pow(((1f + _e2818) - ((2f * _e1488) * _e2808)), 1.5f)));
                                                    phi_32497_ = _e2798;
                                                    phi_3217_ = vec3<f32>((_e1428.x + (((((_e2800.member.x * _e2811) + (_e2800.member_1.x * _e2829)) + _e2800.member_2.x) * _e1472) * 0.38317f)), (_e1428.y + (((((_e2800.member.y * _e2811) + (_e2800.member_1.y * _e2829)) + _e2800.member_2.y) * _e1472) * 0.38317f)), (_e1428.z + (((((_e2800.member.z * _e2811) + (_e2800.member_1.z * _e2829)) + _e2800.member_2.z) * _e1472) * 0.38317f)));
                                                    phi_3218_ = (_e1430 + 1i);
                                                    phi_3219_ = bitcast<u32>(_e1508);
                                                } else {
                                                    phi_32497_ = _e1426;
                                                    phi_3217_ = vec3<f32>();
                                                    phi_3218_ = i32();
                                                    phi_3219_ = u32();
                                                }
                                                let _e2860 = phi_32497_;
                                                let _e2862 = phi_3217_;
                                                let _e2864 = phi_3218_;
                                                let _e2866 = phi_3219_;
                                                phi_32496_ = _e2860;
                                                phi_863_ = _e2862;
                                                phi_866_ = _e2864;
                                                phi_868_ = _e2866;
                                                phi_3220_ = _e1434;
                                            } else {
                                                phi_32496_ = _e1426;
                                                phi_863_ = vec3<f32>();
                                                phi_866_ = i32();
                                                phi_868_ = u32();
                                                phi_3220_ = false;
                                            }
                                            let _e2868 = phi_32496_;
                                            let _e2870 = phi_863_;
                                            let _e2872 = phi_866_;
                                            let _e2874 = phi_868_;
                                            let _e2876 = phi_3220_;
                                            phi_32437_ = _e2868;
                                            phi_862_ = _e2870;
                                            phi_865_ = _e2872;
                                            phi_867_ = _e2874;
                                            phi_32495_ = _e2868;
                                            if !(_e2876) {
                                                break;
                                            }
                                        }
                                        let _e2879 = phi_32495_;
                                        phi_32823_ = _e2879;
                                        if _e2879 {
                                            break;
                                        }
                                        if _e1423 {
                                            let _e2882 = global_6.member[(_e1275 + 10u)];
                                            let _e2885 = global_6.member[(_e1275 + 21u)];
                                            let _e2886 = bitcast<u32>(_e2885);
                                            let _e2889 = global_6.member[(_e1275 + 22u)];
                                            let _e2890 = bitcast<u32>(_e2889);
                                            let _e2893 = global_6.member[(_e1275 + 23u)];
                                            let _e2897 = global_6.member[(_e1275 + 24u)];
                                            let _e2901 = global_6.member[(_e1275 + 25u)];
                                            let _e2905 = global_6.member[(_e1275 + 26u)];
                                            let _e2908 = global_6.member[_e1275];
                                            let _e2911 = global_6.member[(_e1275 + 1u)];
                                            let _e2914 = global_6.member[(_e1275 + 2u)];
                                            let _e2917 = global_6.member[(_e1275 + 3u)];
                                            let _e2920 = global_6.member[(_e1275 + 4u)];
                                            let _e2923 = global_6.member[(_e1275 + 5u)];
                                            let _e2926 = global_6.member[(_e1275 + 6u)];
                                            let _e2929 = global_6.member[(_e1275 + 7u)];
                                            let _e2932 = global_6.member[(_e1275 + 8u)];
                                            let _e2937 = (((_e317.member_1.x * _e2917) + (_e317.member_1.y * _e2920)) + (_e317.member_1.z * _e2923));
                                            let _e2939 = select(_e2937, -1f, (_e2937 < -1f));
                                            let _e2942 = asin(select(_e2939, 1f, (_e2939 > 1f)));
                                            let _e2953 = atan2((((_e317.member_1.x * _e2926) + (_e317.member_1.y * _e2929)) + (_e317.member_1.z * _e2932)), (((_e317.member_1.x * _e2908) + (_e317.member_1.y * _e2911)) + (_e317.member_1.z * _e2914)));
                                            if (_e2886 != 4294967295u) {
                                                let _e2956 = global_6.member[_e2886];
                                                let _e2959 = global_6.member[(_e2886 + 1u)];
                                                let _e2960 = bitcast<u32>(_e2959);
                                                let _e2963 = global_6.member[(_e2886 + 4u)];
                                                phi_3554_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3557_ = 0u;
                                                loop {
                                                    let _e2969 = phi_3554_;
                                                    let _e2971 = phi_3557_;
                                                    local_15 = _e2969;
                                                    let _e2972 = (_e2971 < 6u);
                                                    if _e2972 {
                                                        let _e2973 = (_e2971 / 2u);
                                                        if (_e2973 == 0u) {
                                                            phi_22816_ = _e317.member_1.x;
                                                        } else {
                                                            phi_22816_ = select(_e317.member_1.z, _e317.member_1.y, (_e2973 == 1u));
                                                        }
                                                        let _e2981 = phi_22816_;
                                                        let _e2982 = (_e2981 * select(-1f, 1f, ((_e2971 % 2u) == 0u)));
                                                        if (_e2982 > 0f) {
                                                            let _e2986 = ((_e2973 + 1u) % 3u);
                                                            if (_e2986 == 0u) {
                                                                phi_22831_ = _e317.member_1.x;
                                                            } else {
                                                                phi_22831_ = select(_e317.member_1.z, _e317.member_1.y, (_e2986 == 1u));
                                                            }
                                                            let _e2991 = phi_22831_;
                                                            let _e2992 = (_e2991 / _e2982);
                                                            let _e2994 = ((_e2973 + 2u) % 3u);
                                                            if (_e2994 == 0u) {
                                                                phi_22846_ = _e317.member_1.x;
                                                            } else {
                                                                phi_22846_ = select(_e317.member_1.z, _e317.member_1.y, (_e2994 == 1u));
                                                            }
                                                            let _e2999 = phi_22846_;
                                                            let _e3000 = (_e2999 / _e2982);
                                                            let _e3002 = (1f + (_e2956 * 12f));
                                                            if (abs(_e2992) <= _e3002) {
                                                                if (abs(_e3000) <= _e3002) {
                                                                    let _e3008 = f32(_e2960);
                                                                    let _e3010 = floor((((_e2992 + 1f) * 0.5f) * _e3008));
                                                                    let _e3017 = select(0i, select(select(i32(_e3010), i32(-2147483648), (_e3010 < -2147483600f)), 2147483647i, (_e3010 > 2147483500f)), (_e3010 == _e3010));
                                                                    let _e3019 = (bitcast<i32>(_e2960) - 1i);
                                                                    let _e3020 = (0i <= _e3019);
                                                                    if _e3020 {
                                                                    } else {
                                                                        phi_32500_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3017 < 0i) {
                                                                        phi_3628_ = 0i;
                                                                    } else {
                                                                        phi_3628_ = select(_e3017, _e3019, (_e3017 > _e3019));
                                                                    }
                                                                    let _e3025 = phi_3628_;
                                                                    let _e3029 = floor((((_e3000 + 1f) * 0.5f) * _e3008));
                                                                    let _e3036 = select(0i, select(select(i32(_e3029), i32(-2147483648), (_e3029 < -2147483600f)), 2147483647i, (_e3029 > 2147483500f)), (_e3029 == _e3029));
                                                                    if _e3020 {
                                                                    } else {
                                                                        phi_32500_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3036 < 0i) {
                                                                        phi_3658_ = 0i;
                                                                    } else {
                                                                        phi_3658_ = select(_e3036, _e3019, (_e3036 > _e3019));
                                                                    }
                                                                    let _e3041 = phi_3658_;
                                                                    let _e3042 = (_e3025 - 1i);
                                                                    phi_3666_ = _e2969;
                                                                    phi_3669_ = select(0i, _e3042, (0i < _e3042));
                                                                    loop {
                                                                        let _e3046 = phi_3666_;
                                                                        let _e3048 = phi_3669_;
                                                                        let _e3049 = (_e3025 + 1i);
                                                                        local_59 = _e3046;
                                                                        let _e3052 = (_e3048 <= select(_e3049, _e3019, (_e3019 < _e3049)));
                                                                        if _e3052 {
                                                                            let _e3053 = (_e3041 - 1i);
                                                                            phi_3692_ = _e3046;
                                                                            phi_3695_ = select(0i, _e3053, (0i < _e3053));
                                                                            loop {
                                                                                let _e3057 = phi_3692_;
                                                                                let _e3059 = phi_3695_;
                                                                                let _e3060 = (_e3041 + 1i);
                                                                                local_58 = _e3057;
                                                                                let _e3063 = (_e3059 <= select(_e3060, _e3019, (_e3019 < _e3060)));
                                                                                if _e3063 {
                                                                                    let _e3072 = global_6.member[(bitcast<u32>(_e2963) + ((((_e2971 * _e2960) + bitcast<u32>(_e3048)) * _e2960) + bitcast<u32>(_e3059)))];
                                                                                    phi_3693_ = vec3<f32>((_e3057.x + _e3072), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3696_ = (_e3059 + 1i);
                                                                                } else {
                                                                                    phi_3693_ = vec3<f32>();
                                                                                    phi_3696_ = i32();
                                                                                }
                                                                                let _e3081 = phi_3693_;
                                                                                let _e3083 = phi_3696_;
                                                                                phi_3692_ = _e3081;
                                                                                phi_3695_ = _e3083;
                                                                                if !(_e3063) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6918 = local_58;
                                                                            phi_3667_ = _e6918;
                                                                            phi_3670_ = (_e3048 + 1i);
                                                                        } else {
                                                                            phi_3667_ = vec3<f32>();
                                                                            phi_3670_ = i32();
                                                                        }
                                                                        let _e3087 = phi_3667_;
                                                                        let _e3089 = phi_3670_;
                                                                        phi_3666_ = _e3087;
                                                                        phi_3669_ = _e3089;
                                                                        if !(_e3052) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6922 = local_59;
                                                                    phi_3894_ = _e6922;
                                                                } else {
                                                                    phi_3894_ = _e2969;
                                                                }
                                                                let _e3092 = phi_3894_;
                                                                phi_3895_ = _e3092;
                                                            } else {
                                                                phi_3895_ = _e2969;
                                                            }
                                                            let _e3094 = phi_3895_;
                                                            phi_3896_ = _e3094;
                                                        } else {
                                                            phi_3896_ = _e2969;
                                                        }
                                                        let _e3096 = phi_3896_;
                                                        phi_3555_ = _e3096;
                                                        phi_3558_ = (_e2971 + 1u);
                                                    } else {
                                                        phi_3555_ = vec3<f32>();
                                                        phi_3558_ = u32();
                                                    }
                                                    let _e3099 = phi_3555_;
                                                    let _e3101 = phi_3558_;
                                                    phi_3554_ = _e3099;
                                                    phi_3557_ = _e3101;
                                                    phi_32500_ = _e2879;
                                                    if !(_e2972) {
                                                        break;
                                                    }
                                                }
                                                let _e3104 = phi_32500_;
                                                phi_32823_ = _e3104;
                                                if _e3104 {
                                                    break;
                                                }
                                                let _e3106 = local_15;
                                                phi_32548_ = _e3104;
                                                phi_3907_ = (_e3106.x * (1f / ((6.2831855f * _e2956) * _e2956)));
                                            } else {
                                                phi_32548_ = _e2879;
                                                phi_3907_ = 0f;
                                            }
                                            let _e3110 = phi_32548_;
                                            let _e3112 = phi_3907_;
                                            if (_e2890 != 4294967295u) {
                                                let _e3129 = (_e2942 * 1.3333334f);
                                                let _e3136 = global_6.member[_e2890];
                                                phi_3931_ = (_e3112 + ((_e3136 * ((exp((-(abs(_e2942)) / (0.06f + (abs(_e2953) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2953 * -0.71428573f) * (_e2953 * 0.71428573f)))))) + (1.3f * exp((((_e2953 * _e2953) + (_e3129 * _e3129)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3931_ = _e3112;
                                            }
                                            let _e3142 = phi_3931_;
                                            phi_3933_ = 0u;
                                            phi_3936_ = _e3142;
                                            loop {
                                                let _e3144 = phi_3933_;
                                                let _e3146 = phi_3936_;
                                                local_60 = _e3146;
                                                let _e3147 = (_e3144 < bitcast<u32>(_e2897));
                                                if _e3147 {
                                                    let _e3149 = (bitcast<u32>(_e2893) + (_e3144 * 18u));
                                                    let _e3152 = global_6.member[(_e3149 + 1u)];
                                                    let _e3155 = global_6.member[(_e3149 + 2u)];
                                                    let _e3158 = global_6.member[(_e3149 + 3u)];
                                                    let _e3166 = global_6.member[(_e3149 + 5u)];
                                                    if ((((_e317.member_1.x * _e3152) + (_e317.member_1.y * _e3155)) + (_e317.member_1.z * _e3158)) >= _e3166) {
                                                        let _e3169 = global_6.member[_e3149];
                                                        let _e3174 = select(select(u32(_e3169), 0u, (_e3169 < 0f)), 4294967295u, (_e3169 > 4294967000f));
                                                        if (_e3174 == 0u) {
                                                            phi_4040_ = 1.8f;
                                                        } else {
                                                            if (_e3174 == 1u) {
                                                                phi_4039_ = 1f;
                                                            } else {
                                                                phi_4039_ = select(0f, 4.7f, (_e3174 == 2u));
                                                            }
                                                            let _e3180 = phi_4039_;
                                                            phi_4040_ = _e3180;
                                                        }
                                                        let _e3182 = phi_4040_;
                                                        let _e3185 = global_6.member[(_e3149 + 6u)];
                                                        phi_4056_ = (_e3146 + (_e3185 * _e3182));
                                                    } else {
                                                        phi_4056_ = _e3146;
                                                    }
                                                    let _e3189 = phi_4056_;
                                                    phi_3934_ = (_e3144 + 1u);
                                                    phi_3937_ = _e3189;
                                                } else {
                                                    phi_3934_ = u32();
                                                    phi_3937_ = f32();
                                                }
                                                let _e3192 = phi_3934_;
                                                let _e3194 = phi_3937_;
                                                phi_3933_ = _e3192;
                                                phi_3936_ = _e3194;
                                                if !(_e3147) {
                                                    break;
                                                }
                                            }
                                            phi_4059_ = 0u;
                                            let _e6942 = local_60;
                                            phi_4062_ = _e6942;
                                            loop {
                                                let _e3197 = phi_4059_;
                                                let _e3199 = phi_4062_;
                                                local_16 = _e3199;
                                                let _e3200 = (_e3197 < bitcast<u32>(_e2905));
                                                if _e3200 {
                                                    let _e3202 = (bitcast<u32>(_e2901) + (_e3197 * 29u));
                                                    let _e3205 = global_6.member[(_e3202 + 1u)];
                                                    let _e3208 = global_6.member[(_e3202 + 2u)];
                                                    let _e3211 = global_6.member[(_e3202 + 3u)];
                                                    let _e3219 = global_6.member[(_e3202 + 5u)];
                                                    if ((((_e317.member_1.x * _e3205) + (_e317.member_1.y * _e3208)) + (_e317.member_1.z * _e3211)) >= _e3219) {
                                                        let _e3222 = global_6.member[_e3202];
                                                        let _e3226 = global_6.member[(_e3202 + 6u)];
                                                        phi_4170_ = (_e3199 + (_e3226 * select(1f, 3.3808f, (_e3222 < 0.5f))));
                                                    } else {
                                                        phi_4170_ = _e3199;
                                                    }
                                                    let _e3231 = phi_4170_;
                                                    phi_4060_ = (_e3197 + 1u);
                                                    phi_4063_ = _e3231;
                                                } else {
                                                    phi_4060_ = u32();
                                                    phi_4063_ = f32();
                                                }
                                                let _e3234 = phi_4060_;
                                                let _e3236 = phi_4063_;
                                                phi_4059_ = _e3234;
                                                phi_4062_ = _e3236;
                                                if !(_e3200) {
                                                    break;
                                                }
                                            }
                                            let _e3240 = global_6.member[(_e1275 + 9u)];
                                            let _e3242 = local_16;
                                            let _e3245 = local_17;
                                            let _e3249 = local_18;
                                            let _e3254 = local_19;
                                            let _e3259 = ((_e3242 * _e3240) <= (_e2882 * (((0.2126f * _e3245.x) + (0.7152f * _e3249.y)) + (0.0722f * _e3254.z))));
                                            if _e3259 {
                                                phi_32589_ = _e3110;
                                                phi_4729_ = vec3<f32>();
                                                phi_4730_ = bool();
                                            } else {
                                                let _e3261 = global_6.member[_e1272];
                                                let _e3264 = global_6.member[(_e1272 + 1u)];
                                                let _e3267 = global_6.member[(_e1272 + 2u)];
                                                let _e3270 = global_6.member[(_e1272 + 3u)];
                                                let _e3273 = global_6.member[(_e1272 + 5u)];
                                                let _e3276 = global_6.member[(_e1272 + 6u)];
                                                let _e3279 = global_6.member[(_e1272 + 10u)];
                                                let _e3282 = global_6.member[(_e1272 + 11u)];
                                                let _e3285 = global_6.member[(_e1272 + 13u)];
                                                let _e3288 = global_6.member[(_e1272 + 14u)];
                                                let _e3291 = global_6.member[(_e1272 + 15u)];
                                                let _e3294 = global_6.member[(_e1272 + 16u)];
                                                let _e3297 = global_6.member[(_e1272 + 18u)];
                                                let _e3300 = global_6.member[(_e1272 + 19u)];
                                                let _e3303 = global_6.member[(_e1272 + 20u)];
                                                let _e3315 = ((((_e317.member.x * _e317.member.x) + (_e317.member.y * _e317.member.y)) + (_e317.member.z * _e317.member.z)) - (2f * (((_e317.member.x * _e3261) + (_e317.member.y * _e3264)) + (_e317.member.z * _e3267))));
                                                let _e3316 = (_e3315 + _e3273);
                                                let _e3317 = (_e3316 > 0f);
                                                if _e3317 {
                                                    let _e3325 = ((((_e317.member.x - _e3261) * _e317.member_1.x) + ((_e317.member.y - _e3264) * _e317.member_1.y)) + ((_e317.member.z - _e3267) * _e317.member_1.z));
                                                    let _e3327 = ((_e3325 * _e3325) - _e3316);
                                                    if (_e3327 < 0f) {
                                                        phi_22990_ = type_54(0u, type_54().member_1);
                                                        phi_22991_ = type_54();
                                                        phi_22992_ = true;
                                                    } else {
                                                        if (_e3325 < 0f) {
                                                            phi_22962_ = -(sqrt(_e3327));
                                                        } else {
                                                            phi_22962_ = sqrt(_e3327);
                                                        }
                                                        let _e3334 = phi_22962_;
                                                        let _e3336 = -((_e3325 + _e3334));
                                                        let _e3337 = (_e3336 == 0f);
                                                        if _e3337 {
                                                            phi_22985_ = type_54(1u, type_53(0f, 0f));
                                                            phi_22986_ = type_54();
                                                        } else {
                                                            let _e3338 = (_e3316 / _e3336);
                                                            if (_e3336 < _e3338) {
                                                                phi_22974_ = type_53(_e3336, _e3338);
                                                            } else {
                                                                phi_22974_ = type_53(_e3338, _e3336);
                                                            }
                                                            let _e3343 = phi_22974_;
                                                            phi_22985_ = type_54();
                                                            phi_22986_ = type_54(1u, _e3343);
                                                        }
                                                        let _e3346 = phi_22985_;
                                                        let _e3348 = phi_22986_;
                                                        phi_22990_ = _e3346;
                                                        phi_22991_ = _e3348;
                                                        phi_22992_ = _e3337;
                                                    }
                                                    let _e3353 = phi_22990_;
                                                    let _e3355 = phi_22991_;
                                                    let _e3357 = phi_22992_;
                                                    if _e3357 {
                                                        phi_22996_ = _e3353;
                                                    } else {
                                                        phi_22996_ = _e3355;
                                                    }
                                                    let _e3359 = phi_22996_;
                                                    if (bitcast<i32>(_e3359.member) != 0i) {
                                                        phi_22930_ = select(_e3359.member_1.member_1, _e3359.member_1.member, (_e3359.member_1.member >= 0f));
                                                    } else {
                                                        phi_22930_ = -1f;
                                                    }
                                                    let _e3370 = phi_22930_;
                                                    phi_4571_ = select(true, false, (_e3370 >= 0f));
                                                } else {
                                                    phi_4571_ = true;
                                                }
                                                let _e3374 = phi_4571_;
                                                if _e3374 {
                                                    let _e3382 = ((((_e317.member.x - _e3261) * _e317.member_1.x) + ((_e317.member.y - _e3264) * _e317.member_1.y)) + ((_e317.member.z - _e3267) * _e317.member_1.z));
                                                    let _e3383 = (_e3315 + _e3276);
                                                    let _e3384 = (_e3382 * _e3382);
                                                    let _e3385 = (_e3384 - _e3383);
                                                    if (_e3385 < 0f) {
                                                        phi_23150_ = type_54(0u, type_54().member_1);
                                                        phi_23151_ = type_54();
                                                        phi_23152_ = true;
                                                    } else {
                                                        if (_e3382 < 0f) {
                                                            phi_23122_ = -(sqrt(_e3385));
                                                        } else {
                                                            phi_23122_ = sqrt(_e3385);
                                                        }
                                                        let _e3392 = phi_23122_;
                                                        let _e3394 = -((_e3382 + _e3392));
                                                        let _e3395 = (_e3394 == 0f);
                                                        if _e3395 {
                                                            phi_23145_ = type_54(1u, type_53(0f, 0f));
                                                            phi_23146_ = type_54();
                                                        } else {
                                                            let _e3396 = (_e3383 / _e3394);
                                                            if (_e3394 < _e3396) {
                                                                phi_23134_ = type_53(_e3394, _e3396);
                                                            } else {
                                                                phi_23134_ = type_53(_e3396, _e3394);
                                                            }
                                                            let _e3401 = phi_23134_;
                                                            phi_23145_ = type_54();
                                                            phi_23146_ = type_54(1u, _e3401);
                                                        }
                                                        let _e3404 = phi_23145_;
                                                        let _e3406 = phi_23146_;
                                                        phi_23150_ = _e3404;
                                                        phi_23151_ = _e3406;
                                                        phi_23152_ = _e3395;
                                                    }
                                                    let _e3411 = phi_23150_;
                                                    let _e3413 = phi_23151_;
                                                    let _e3415 = phi_23152_;
                                                    if _e3415 {
                                                        phi_23156_ = _e3411;
                                                    } else {
                                                        phi_23156_ = _e3413;
                                                    }
                                                    let _e3417 = phi_23156_;
                                                    if (bitcast<i32>(_e3417.member) != 0i) {
                                                        phi_23043_ = type_54(0u, _e3417.member_1);
                                                    } else {
                                                        phi_23043_ = type_54(1u, type_54().member_1);
                                                    }
                                                    let _e3427 = phi_23043_;
                                                    if (bitcast<i32>(_e3427.member) != 0i) {
                                                        phi_23090_ = type_54(0u, type_54().member_1);
                                                    } else {
                                                        if (_e3427.member_1.member_1 <= 0f) {
                                                            phi_23086_ = type_54(0u, type_54().member_1);
                                                        } else {
                                                            if (_e3427.member_1.member != _e3427.member_1.member) {
                                                                phi_23194_ = true;
                                                            } else {
                                                                phi_23194_ = (0f >= _e3427.member_1.member);
                                                            }
                                                            let _e3439 = phi_23194_;
                                                            let _e3440 = select(_e3427.member_1.member, 0f, _e3439);
                                                            if _e3317 {
                                                                let _e3441 = (_e3384 - _e3316);
                                                                if (_e3441 < 0f) {
                                                                    phi_23302_ = type_54(0u, type_54().member_1);
                                                                    phi_23303_ = type_54();
                                                                    phi_23304_ = true;
                                                                } else {
                                                                    if (_e3382 < 0f) {
                                                                        phi_23274_ = -(sqrt(_e3441));
                                                                    } else {
                                                                        phi_23274_ = sqrt(_e3441);
                                                                    }
                                                                    let _e3448 = phi_23274_;
                                                                    let _e3450 = -((_e3382 + _e3448));
                                                                    let _e3451 = (_e3450 == 0f);
                                                                    if _e3451 {
                                                                        phi_23297_ = type_54(1u, type_53(0f, 0f));
                                                                        phi_23298_ = type_54();
                                                                    } else {
                                                                        let _e3452 = (_e3316 / _e3450);
                                                                        if (_e3450 < _e3452) {
                                                                            phi_23286_ = type_53(_e3450, _e3452);
                                                                        } else {
                                                                            phi_23286_ = type_53(_e3452, _e3450);
                                                                        }
                                                                        let _e3457 = phi_23286_;
                                                                        phi_23297_ = type_54();
                                                                        phi_23298_ = type_54(1u, _e3457);
                                                                    }
                                                                    let _e3460 = phi_23297_;
                                                                    let _e3462 = phi_23298_;
                                                                    phi_23302_ = _e3460;
                                                                    phi_23303_ = _e3462;
                                                                    phi_23304_ = _e3451;
                                                                }
                                                                let _e3467 = phi_23302_;
                                                                let _e3469 = phi_23303_;
                                                                let _e3471 = phi_23304_;
                                                                if _e3471 {
                                                                    phi_23308_ = _e3467;
                                                                } else {
                                                                    phi_23308_ = _e3469;
                                                                }
                                                                let _e3473 = phi_23308_;
                                                                if (bitcast<i32>(_e3473.member) != 0i) {
                                                                    phi_23242_ = select(_e3473.member_1.member_1, _e3473.member_1.member, (_e3473.member_1.member >= 0f));
                                                                } else {
                                                                    phi_23242_ = -1f;
                                                                }
                                                                let _e3484 = phi_23242_;
                                                                if (_e3484 >= 0f) {
                                                                    if (_e3427.member_1.member_1 != _e3427.member_1.member_1) {
                                                                        phi_23346_ = true;
                                                                    } else {
                                                                        phi_23346_ = (_e3484 <= _e3427.member_1.member_1);
                                                                    }
                                                                    let _e3489 = phi_23346_;
                                                                    phi_23071_ = select(_e3427.member_1.member_1, _e3484, _e3489);
                                                                } else {
                                                                    phi_23071_ = _e3427.member_1.member_1;
                                                                }
                                                                let _e3492 = phi_23071_;
                                                                phi_23073_ = _e3492;
                                                            } else {
                                                                phi_23073_ = _e3427.member_1.member_1;
                                                            }
                                                            let _e3494 = phi_23073_;
                                                            if (_e3494 <= _e3440) {
                                                                phi_23082_ = type_54(0u, type_54().member_1);
                                                            } else {
                                                                phi_23082_ = type_54(1u, type_53(_e3440, _e3494));
                                                            }
                                                            let _e3502 = phi_23082_;
                                                            phi_23086_ = _e3502;
                                                        }
                                                        let _e3507 = phi_23086_;
                                                        phi_23090_ = _e3507;
                                                    }
                                                    let _e3512 = phi_23090_;
                                                    let _e3515 = (bitcast<i32>(_e3512.member) != 0i);
                                                    if _e3515 {
                                                        phi_4585_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4588_ = 0u;
                                                        loop {
                                                            let _e3521 = phi_4585_;
                                                            let _e3523 = phi_4588_;
                                                            local_20 = _e3521;
                                                            local_21 = _e3521;
                                                            local_22 = _e3521;
                                                            let _e3524 = (_e3523 < 32u);
                                                            if _e3524 {
                                                                let _e3525 = -(_e3382);
                                                                if (_e3512.member_1.member <= _e3512.member_1.member_1) {
                                                                } else {
                                                                    phi_32527_ = true;
                                                                    break;
                                                                }
                                                                let _e3528 = select(_e3525, _e3512.member_1.member, (_e3525 < _e3512.member_1.member));
                                                                let _e3530 = select(_e3528, _e3512.member_1.member_1, (_e3528 > _e3512.member_1.member_1));
                                                                let _e3531 = (_e3530 - _e3512.member_1.member);
                                                                let _e3532 = (_e3512.member_1.member_1 - _e3530);
                                                                let _e3533 = (_e3512.member_1.member_1 - _e3512.member_1.member);
                                                                let _e3534 = (0.000000001f * _e3533);
                                                                if (_e3531 <= _e3534) {
                                                                    let _e3609 = (f32(_e3523) * 0.03125f);
                                                                    let _e3612 = (f32((_e3523 + 1u)) * 0.03125f);
                                                                    let _e3614 = ((_e3609 + _e3612) * 0.5f);
                                                                    phi_4689_ = type_53((_e3512.member_1.member + ((_e3533 * _e3614) * _e3614)), (_e3533 * ((_e3612 * _e3612) - (_e3609 * _e3609))));
                                                                } else {
                                                                    if (_e3532 <= _e3534) {
                                                                        let _e3592 = (f32((31u - _e3523)) * 0.03125f);
                                                                        let _e3595 = (f32((32u - _e3523)) * 0.03125f);
                                                                        let _e3597 = ((_e3592 + _e3595) * 0.5f);
                                                                        phi_4688_ = type_53((_e3512.member_1.member_1 - ((_e3533 * _e3597) * _e3597)), (_e3533 * ((_e3595 * _e3595) - (_e3592 * _e3592))));
                                                                    } else {
                                                                        let _e3540 = round(((32f * _e3531) / (_e3531 + _e3532)));
                                                                        let _e3545 = select(select(u32(_e3540), 0u, (_e3540 < 0f)), 4294967295u, (_e3540 > 4294967000f));
                                                                        if (_e3545 < 1u) {
                                                                            phi_4672_ = 1u;
                                                                        } else {
                                                                            phi_4672_ = select(_e3545, 31u, (_e3545 > 31u));
                                                                        }
                                                                        let _e3550 = phi_4672_;
                                                                        if (_e3523 < _e3550) {
                                                                            let _e3571 = ((_e3550 - 1u) - _e3523);
                                                                            let _e3573 = f32(_e3550);
                                                                            let _e3574 = (f32(_e3571) / _e3573);
                                                                            let _e3577 = (f32((_e3571 + 1u)) / _e3573);
                                                                            let _e3579 = ((_e3574 + _e3577) * 0.5f);
                                                                            phi_4687_ = type_53((_e3530 - ((_e3531 * _e3579) * _e3579)), (_e3531 * ((_e3577 * _e3577) - (_e3574 * _e3574))));
                                                                        } else {
                                                                            let _e3553 = (_e3523 - _e3550);
                                                                            let _e3555 = f32((32u - _e3550));
                                                                            let _e3556 = (f32(_e3553) / _e3555);
                                                                            let _e3559 = (f32((_e3553 + 1u)) / _e3555);
                                                                            let _e3561 = ((_e3556 + _e3559) * 0.5f);
                                                                            phi_4687_ = type_53((_e3530 + ((_e3532 * _e3561) * _e3561)), (_e3532 * ((_e3559 * _e3559) - (_e3556 * _e3556))));
                                                                        }
                                                                        let _e3589 = phi_4687_;
                                                                        phi_4688_ = _e3589;
                                                                    }
                                                                    let _e3607 = phi_4688_;
                                                                    phi_4689_ = _e3607;
                                                                }
                                                                let _e3624 = phi_4689_;
                                                                let _e3630 = (_e317.member.x + (_e317.member_1.x * _e3624.member));
                                                                let _e3631 = (_e317.member.y + (_e317.member_1.y * _e3624.member));
                                                                let _e3632 = (_e317.member.z + (_e317.member_1.z * _e3624.member));
                                                                let _e3646 = (_e3630 - _e3261);
                                                                let _e3647 = (_e3631 - _e3264);
                                                                let _e3648 = (_e3632 - _e3267);
                                                                let _e3656 = ((((((_e3630 * _e3630) + (_e3631 * _e3631)) + (_e3632 * _e3632)) - (2f * (((_e3630 * _e3261) + (_e3631 * _e3264)) + (_e3632 * _e3267)))) + _e3273) / (sqrt((((_e3646 * _e3646) + (_e3647 * _e3647)) + (_e3648 * _e3648))) + _e3270));
                                                                let _e3660 = (1f - (abs((_e3656 - 25000f)) * 0.00006666667f));
                                                                if (_e3660 != _e3660) {
                                                                    phi_23545_ = true;
                                                                } else {
                                                                    phi_23545_ = (0f >= _e3660);
                                                                }
                                                                let _e3664 = phi_23545_;
                                                                let _e3666 = -(_e3656);
                                                                phi_4586_ = (_e3521 + vec3<f32>((exp((_e3666 / _e3279)) * _e3624.member_1), (exp((_e3666 / _e3282)) * _e3624.member_1), (select(_e3660, 0f, _e3664) * _e3624.member_1)));
                                                                phi_4589_ = (_e3523 + 1u);
                                                            } else {
                                                                phi_4586_ = vec3<f32>();
                                                                phi_4589_ = u32();
                                                            }
                                                            let _e3678 = phi_4586_;
                                                            let _e3680 = phi_4589_;
                                                            phi_4585_ = _e3678;
                                                            phi_4588_ = _e3680;
                                                            phi_32527_ = _e3110;
                                                            if !(_e3524) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3683 = phi_32527_;
                                                        phi_32823_ = _e3683;
                                                        if _e3683 {
                                                            break;
                                                        }
                                                        let _e3685 = local_20;
                                                        let _e3692 = local_21;
                                                        let _e3694 = ((_e3294 * 1.1f) * _e3692.y);
                                                        let _e3699 = local_22;
                                                        phi_32594_ = _e3683;
                                                        phi_4714_ = vec3<f32>(exp(-((((_e3285 * _e3685.x) + _e3694) + (_e3297 * _e3699.z)))), exp(-((((_e3288 * _e3685.x) + _e3694) + (_e3300 * _e3699.z)))), exp(-((((_e3291 * _e3685.x) + _e3694) + (_e3303 * _e3699.z)))));
                                                    } else {
                                                        phi_32594_ = _e3110;
                                                        phi_4714_ = vec3<f32>();
                                                    }
                                                    let _e3715 = phi_32594_;
                                                    let _e3717 = phi_4714_;
                                                    phi_32593_ = _e3715;
                                                    phi_4716_ = _e3717;
                                                    phi_4718_ = select(true, false, _e3515);
                                                } else {
                                                    phi_32593_ = _e3110;
                                                    phi_4716_ = vec3<f32>();
                                                    phi_4718_ = true;
                                                }
                                                let _e3720 = phi_32593_;
                                                let _e3722 = phi_4716_;
                                                let _e3724 = phi_4718_;
                                                let _e3728 = select(_e3722, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3374)), vec3(_e3724));
                                                let _e3732 = select(_e3728.y, _e3728.x, (_e3728.x > _e3728.y));
                                                phi_32589_ = _e3720;
                                                phi_4729_ = _e3728;
                                                phi_4730_ = (select(_e3728.z, _e3732, (_e3732 > _e3728.z)) > 0f);
                                            }
                                            let _e3738 = phi_32589_;
                                            let _e3740 = phi_4729_;
                                            let _e3742 = phi_4730_;
                                            phi_32588_ = _e3738;
                                            phi_4732_ = _e3740;
                                            phi_4733_ = _e3742;
                                            phi_4734_ = select(true, false, _e3259);
                                        } else {
                                            phi_32588_ = _e2879;
                                            phi_4732_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4733_ = _e1423;
                                            phi_4734_ = true;
                                        }
                                        let _e3745 = phi_32588_;
                                        let _e3747 = phi_4732_;
                                        let _e3749 = phi_4733_;
                                        let _e3751 = phi_4734_;
                                        phi_32587_ = _e3745;
                                        let _e7013 = local_61;
                                        phi_4738_ = _e7013;
                                        phi_4739_ = _e3747;
                                        phi_4740_ = _e3749;
                                        let _e7018 = local_62;
                                        phi_4741_ = _e7018;
                                        phi_4742_ = _e3751;
                                    } else {
                                        phi_32587_ = _e306;
                                        phi_4738_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4739_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4740_ = _e1423;
                                        phi_4741_ = vec3<f32>();
                                        phi_4742_ = true;
                                    }
                                    let _e3753 = phi_32587_;
                                    let _e3755 = phi_4738_;
                                    let _e3757 = phi_4739_;
                                    let _e3759 = phi_4740_;
                                    let _e3761 = phi_4741_;
                                    let _e3763 = phi_4742_;
                                    if _e3763 {
                                        if _e3759 {
                                            let _e3764 = (_e1275 + 20u);
                                            let _e3767 = global_6.member[(_e1275 + 21u)];
                                            let _e3768 = bitcast<u32>(_e3767);
                                            let _e3771 = global_6.member[(_e1275 + 22u)];
                                            let _e3772 = bitcast<u32>(_e3771);
                                            let _e3775 = global_6.member[(_e1275 + 23u)];
                                            let _e3779 = global_6.member[(_e1275 + 24u)];
                                            let _e3783 = global_6.member[(_e1275 + 25u)];
                                            let _e3787 = global_6.member[(_e1275 + 26u)];
                                            let _e3790 = global_6.member[_e1275];
                                            let _e3793 = global_6.member[(_e1275 + 1u)];
                                            let _e3796 = global_6.member[(_e1275 + 2u)];
                                            let _e3799 = global_6.member[(_e1275 + 3u)];
                                            let _e3802 = global_6.member[(_e1275 + 4u)];
                                            let _e3805 = global_6.member[(_e1275 + 5u)];
                                            let _e3808 = global_6.member[(_e1275 + 6u)];
                                            let _e3811 = global_6.member[(_e1275 + 7u)];
                                            let _e3814 = global_6.member[(_e1275 + 8u)];
                                            let _e3819 = (((_e317.member_1.x * _e3799) + (_e317.member_1.y * _e3802)) + (_e317.member_1.z * _e3805));
                                            let _e3821 = select(_e3819, -1f, (_e3819 < -1f));
                                            let _e3824 = asin(select(_e3821, 1f, (_e3821 > 1f)));
                                            let _e3835 = atan2((((_e317.member_1.x * _e3808) + (_e317.member_1.y * _e3811)) + (_e317.member_1.z * _e3814)), (((_e317.member_1.x * _e3790) + (_e317.member_1.y * _e3793)) + (_e317.member_1.z * _e3796)));
                                            if (_e3768 != 4294967295u) {
                                                let _e3838 = global_6.member[_e3768];
                                                let _e3841 = global_6.member[(_e3768 + 1u)];
                                                let _e3842 = bitcast<u32>(_e3841);
                                                let _e3845 = global_6.member[(_e3768 + 2u)];
                                                let _e3849 = global_6.member[(_e3768 + 3u)];
                                                let _e3851 = (4f * _e3838);
                                                phi_5062_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5065_ = 0u;
                                                loop {
                                                    let _e3856 = phi_5062_;
                                                    let _e3858 = phi_5065_;
                                                    local_66 = _e3856;
                                                    let _e3859 = (_e3858 < 6u);
                                                    if _e3859 {
                                                        let _e3860 = (_e3858 / 2u);
                                                        if (_e3860 == 0u) {
                                                            phi_23616_ = _e317.member_1.x;
                                                        } else {
                                                            phi_23616_ = select(_e317.member_1.z, _e317.member_1.y, (_e3860 == 1u));
                                                        }
                                                        let _e3868 = phi_23616_;
                                                        let _e3869 = (_e3868 * select(-1f, 1f, ((_e3858 % 2u) == 0u)));
                                                        if (_e3869 > 0f) {
                                                            let _e3873 = ((_e3860 + 1u) % 3u);
                                                            if (_e3873 == 0u) {
                                                                phi_23631_ = _e317.member_1.x;
                                                            } else {
                                                                phi_23631_ = select(_e317.member_1.z, _e317.member_1.y, (_e3873 == 1u));
                                                            }
                                                            let _e3878 = phi_23631_;
                                                            let _e3879 = (_e3878 / _e3869);
                                                            let _e3881 = ((_e3860 + 2u) % 3u);
                                                            if (_e3881 == 0u) {
                                                                phi_23646_ = _e317.member_1.x;
                                                            } else {
                                                                phi_23646_ = select(_e317.member_1.z, _e317.member_1.y, (_e3881 == 1u));
                                                            }
                                                            let _e3886 = phi_23646_;
                                                            let _e3887 = (_e3886 / _e3869);
                                                            let _e3889 = (1f + (_e3838 * 12f));
                                                            if (abs(_e3879) <= _e3889) {
                                                                if (abs(_e3887) <= _e3889) {
                                                                    let _e3895 = f32(_e3842);
                                                                    let _e3897 = floor((((_e3879 + 1f) * 0.5f) * _e3895));
                                                                    let _e3904 = select(0i, select(select(i32(_e3897), i32(-2147483648), (_e3897 < -2147483600f)), 2147483647i, (_e3897 > 2147483500f)), (_e3897 == _e3897));
                                                                    let _e3906 = (bitcast<i32>(_e3842) - 1i);
                                                                    let _e3907 = (0i <= _e3906);
                                                                    if _e3907 {
                                                                    } else {
                                                                        phi_32582_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3904 < 0i) {
                                                                        phi_5136_ = 0i;
                                                                    } else {
                                                                        phi_5136_ = select(_e3904, _e3906, (_e3904 > _e3906));
                                                                    }
                                                                    let _e3912 = phi_5136_;
                                                                    let _e3916 = floor((((_e3887 + 1f) * 0.5f) * _e3895));
                                                                    let _e3923 = select(0i, select(select(i32(_e3916), i32(-2147483648), (_e3916 < -2147483600f)), 2147483647i, (_e3916 > 2147483500f)), (_e3916 == _e3916));
                                                                    if _e3907 {
                                                                    } else {
                                                                        phi_32582_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3923 < 0i) {
                                                                        phi_5166_ = 0i;
                                                                    } else {
                                                                        phi_5166_ = select(_e3923, _e3906, (_e3923 > _e3906));
                                                                    }
                                                                    let _e3928 = phi_5166_;
                                                                    let _e3929 = (_e3912 - 1i);
                                                                    phi_5174_ = _e3856;
                                                                    phi_5177_ = select(0i, _e3929, (0i < _e3929));
                                                                    loop {
                                                                        let _e3933 = phi_5174_;
                                                                        let _e3935 = phi_5177_;
                                                                        let _e3936 = (_e3912 + 1i);
                                                                        local_65 = _e3933;
                                                                        let _e3939 = (_e3935 <= select(_e3936, _e3906, (_e3906 < _e3936)));
                                                                        if _e3939 {
                                                                            let _e3940 = (_e3928 - 1i);
                                                                            phi_5200_ = _e3933;
                                                                            phi_5203_ = select(0i, _e3940, (0i < _e3940));
                                                                            loop {
                                                                                let _e3944 = phi_5200_;
                                                                                let _e3946 = phi_5203_;
                                                                                let _e3947 = (_e3928 + 1i);
                                                                                local_64 = _e3944;
                                                                                let _e3950 = (_e3946 <= select(_e3947, _e3906, (_e3906 < _e3947)));
                                                                                if _e3950 {
                                                                                    let _e3957 = (bitcast<u32>(_e3845) + ((((_e3858 * _e3842) + bitcast<u32>(_e3935)) * _e3842) + bitcast<u32>(_e3946)));
                                                                                    let _e3959 = global_6.member[_e3957];
                                                                                    let _e3963 = global_6.member[(_e3957 + 1u)];
                                                                                    phi_5274_ = _e3944;
                                                                                    phi_5277_ = bitcast<u32>(_e3959);
                                                                                    loop {
                                                                                        let _e3966 = phi_5274_;
                                                                                        let _e3968 = phi_5277_;
                                                                                        local_63 = _e3966;
                                                                                        let _e3969 = (_e3968 < bitcast<u32>(_e3963));
                                                                                        if _e3969 {
                                                                                            let _e3971 = (bitcast<u32>(_e3849) + (6u * _e3968));
                                                                                            let _e3973 = global_6.member[_e3971];
                                                                                            let _e3976 = global_6.member[(_e3971 + 1u)];
                                                                                            let _e3979 = global_6.member[(_e3971 + 2u)];
                                                                                            let _e3982 = global_6.member[(_e3971 + 3u)];
                                                                                            let _e3985 = global_6.member[(_e3971 + 4u)];
                                                                                            let _e3988 = global_6.member[(_e3971 + 5u)];
                                                                                            let _e3989 = (_e317.member_1.x - _e3973);
                                                                                            let _e3990 = (_e317.member_1.y - _e3976);
                                                                                            let _e3991 = (_e317.member_1.z - _e3979);
                                                                                            let _e3996 = (((_e3989 * _e3989) + (_e3990 * _e3990)) + (_e3991 * _e3991));
                                                                                            if (_e3996 < (_e3851 * _e3851)) {
                                                                                                let _e4004 = ((1f / ((6.2831855f * _e3838) * _e3838)) * exp((-(_e3996) / ((2f * _e3838) * _e3838))));
                                                                                                phi_5397_ = vec3<f32>((_e3966.x + (_e3982 * _e4004)), (_e3966.y + (_e3985 * _e4004)), (_e3966.z + (_e3988 * _e4004)));
                                                                                            } else {
                                                                                                phi_5397_ = _e3966;
                                                                                            }
                                                                                            let _e4016 = phi_5397_;
                                                                                            phi_5275_ = _e4016;
                                                                                            phi_5278_ = (_e3968 + 1u);
                                                                                        } else {
                                                                                            phi_5275_ = vec3<f32>();
                                                                                            phi_5278_ = u32();
                                                                                        }
                                                                                        let _e4019 = phi_5275_;
                                                                                        let _e4021 = phi_5278_;
                                                                                        phi_5274_ = _e4019;
                                                                                        phi_5277_ = _e4021;
                                                                                        if !(_e3969) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7038 = local_63;
                                                                                    phi_5201_ = _e7038;
                                                                                    phi_5204_ = (_e3946 + 1i);
                                                                                } else {
                                                                                    phi_5201_ = vec3<f32>();
                                                                                    phi_5204_ = i32();
                                                                                }
                                                                                let _e4025 = phi_5201_;
                                                                                let _e4027 = phi_5204_;
                                                                                phi_5200_ = _e4025;
                                                                                phi_5203_ = _e4027;
                                                                                if !(_e3950) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7042 = local_64;
                                                                            phi_5175_ = _e7042;
                                                                            phi_5178_ = (_e3935 + 1i);
                                                                        } else {
                                                                            phi_5175_ = vec3<f32>();
                                                                            phi_5178_ = i32();
                                                                        }
                                                                        let _e4031 = phi_5175_;
                                                                        let _e4033 = phi_5178_;
                                                                        phi_5174_ = _e4031;
                                                                        phi_5177_ = _e4033;
                                                                        if !(_e3939) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7046 = local_65;
                                                                    phi_5402_ = _e7046;
                                                                } else {
                                                                    phi_5402_ = _e3856;
                                                                }
                                                                let _e4036 = phi_5402_;
                                                                phi_5403_ = _e4036;
                                                            } else {
                                                                phi_5403_ = _e3856;
                                                            }
                                                            let _e4038 = phi_5403_;
                                                            phi_5404_ = _e4038;
                                                        } else {
                                                            phi_5404_ = _e3856;
                                                        }
                                                        let _e4040 = phi_5404_;
                                                        phi_5063_ = _e4040;
                                                        phi_5066_ = (_e3858 + 1u);
                                                    } else {
                                                        phi_5063_ = vec3<f32>();
                                                        phi_5066_ = u32();
                                                    }
                                                    let _e4043 = phi_5063_;
                                                    let _e4045 = phi_5066_;
                                                    phi_5062_ = _e4043;
                                                    phi_5065_ = _e4045;
                                                    phi_32582_ = _e3753;
                                                    if !(_e3859) {
                                                        break;
                                                    }
                                                }
                                                let _e4048 = phi_32582_;
                                                phi_32823_ = _e4048;
                                                if _e4048 {
                                                    break;
                                                }
                                                phi_32624_ = _e4048;
                                                let _e7055 = local_66;
                                                phi_5424_ = _e7055;
                                            } else {
                                                phi_32624_ = _e3753;
                                                phi_5424_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4050 = phi_32624_;
                                            let _e4052 = phi_5424_;
                                            if (_e3772 != 4294967295u) {
                                                let _e4055 = global_6.member[_e3772];
                                                let _e4058 = global_6.member[(_e3772 + 1u)];
                                                let _e4061 = global_6.member[(_e3772 + 2u)];
                                                let _e4064 = global_6.member[(_e3772 + 3u)];
                                                let _e4067 = global_6.member[(_e3772 + 4u)];
                                                let _e4070 = global_6.member[(_e3772 + 5u)];
                                                let _e4073 = global_6.member[(_e3772 + 6u)];
                                                let _e4076 = global_6.member[(_e3772 + 7u)];
                                                let _e4079 = global_6.member[(_e3772 + 8u)];
                                                let _e4082 = global_6.member[(_e3772 + 9u)];
                                                let _e4085 = global_6.member[(_e3772 + 10u)];
                                                let _e4089 = global_6.member[(_e3772 + 11u)];
                                                let _e4092 = global_6.member[(_e3772 + 12u)];
                                                let _e4095 = global_6.member[(_e3772 + 13u)];
                                                let _e4097 = (_e4055 > 0f);
                                                let _e4098 = (_e4058 > 0f);
                                                if _e4098 {
                                                    let _e4101 = (abs(_e3824) <= 0.8f);
                                                    let _e4102 = select(2u, 0u, _e4097);
                                                    phi_5637_ = _e4102;
                                                    phi_5638_ = _e4101;
                                                    phi_5641_ = _e4102;
                                                    phi_5642_ = _e4101;
                                                    phi_5643_ = select(true, false, _e4101);
                                                } else {
                                                    phi_5637_ = u32();
                                                    phi_5638_ = bool();
                                                    phi_5641_ = select(2u, 0u, _e4097);
                                                    phi_5642_ = false;
                                                    phi_5643_ = true;
                                                }
                                                let _e4105 = phi_5637_;
                                                let _e4107 = phi_5638_;
                                                let _e4109 = phi_5641_;
                                                let _e4111 = phi_5642_;
                                                let _e4113 = phi_5643_;
                                                let _e4115 = select(select(f32(), 0f, _e4098), 0f, _e4113);
                                                phi_5653_ = _e4115;
                                                phi_5656_ = _e4115;
                                                phi_5658_ = select(_e4105, _e4109, _e4113);
                                                phi_5660_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5662_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5664_ = 0f;
                                                phi_5666_ = 1f;
                                                loop {
                                                    let _e4120 = phi_5653_;
                                                    let _e4122 = phi_5656_;
                                                    let _e4124 = phi_5658_;
                                                    let _e4126 = phi_5660_;
                                                    let _e4128 = phi_5662_;
                                                    let _e4130 = phi_5664_;
                                                    let _e4132 = phi_5666_;
                                                    local_26 = _e4132;
                                                    local_27 = _e4132;
                                                    local_28 = _e4132;
                                                    local_29 = _e4132;
                                                    local_30 = _e4130;
                                                    local_31 = _e4122;
                                                    local_32 = _e4120;
                                                    let _e4133 = (_e4124 < select(11u, 2u, _e4113));
                                                    if _e4133 {
                                                        let _e4134 = (_e4124 < 2u);
                                                        if _e4134 {
                                                            let _e4190 = select(23f, 4f, (_e4124 == 0u));
                                                            phi_5744_ = 4u;
                                                            phi_5746_ = vec3<f32>((_e317.member_1.x * _e4190), (_e317.member_1.y * _e4190), (_e317.member_1.z * _e4190));
                                                        } else {
                                                            if (_e4124 == 2u) {
                                                                phi_5742_ = 3u;
                                                                phi_5743_ = vec3<f32>(((_e317.member_1.x * 1.5f) + 5f), ((_e317.member_1.y * 1.5f) + 5f), ((_e317.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4136 = (_e4124 >= 7u);
                                                                if _e4136 {
                                                                    phi_5704_ = (_e4124 - 7u);
                                                                } else {
                                                                    phi_5704_ = (_e4124 - 3u);
                                                                }
                                                                let _e4140 = phi_5704_;
                                                                if _e4136 {
                                                                    phi_5719_ = vec3<f32>(((_e317.member_1.x * 11f) + -3f), ((_e317.member_1.y * 11f) + 1f), ((_e317.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5719_ = vec3<f32>((_e317.member_1.x * 7f), (_e317.member_1.y * 7f), (_e317.member_1.z * 7f));
                                                                }
                                                                let _e4153 = phi_5719_;
                                                                if (_e4140 < 3u) {
                                                                    if (_e4140 == 0u) {
                                                                        phi_23661_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_23661_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4140 == 1u)));
                                                                    }
                                                                    let _e4172 = phi_23661_;
                                                                    phi_5740_ = 4u;
                                                                    phi_5741_ = (_e4153 + _e4172);
                                                                } else {
                                                                    if _e4136 {
                                                                        phi_5739_ = (_e4153 + _e4126);
                                                                    } else {
                                                                        phi_5739_ = (_e4153 + vec3<f32>((_e4128.x * 1.1f), (_e4128.y * 1.1f), (_e4128.z * 1.1f)));
                                                                    }
                                                                    let _e4165 = phi_5739_;
                                                                    phi_5740_ = select(7u, 5u, _e4136);
                                                                    phi_5741_ = _e4165;
                                                                }
                                                                let _e4175 = phi_5740_;
                                                                let _e4177 = phi_5741_;
                                                                phi_5742_ = _e4175;
                                                                phi_5743_ = _e4177;
                                                            }
                                                            let _e4186 = phi_5742_;
                                                            let _e4188 = phi_5743_;
                                                            phi_5744_ = _e4186;
                                                            phi_5746_ = _e4188;
                                                        }
                                                        let _e4196 = phi_5744_;
                                                        let _e4198 = phi_5746_;
                                                        let _e4200 = select(vec3<f32>(_e4089, _e4092, _e4095), vec3<f32>(_e4079, _e4082, _e4085), vec3(_e4134));
                                                        let _e4202 = global_6.member[_e3764];
                                                        let _e4203 = bitcast<u32>(_e4202);
                                                        let _e4206 = global_6.member[(_e1275 + 11u)];
                                                        let _e4209 = global_6.member[(_e1275 + 12u)];
                                                        let _e4212 = global_6.member[(_e1275 + 13u)];
                                                        let _e4215 = global_6.member[(_e1275 + 14u)];
                                                        let _e4218 = global_6.member[(_e1275 + 15u)];
                                                        let _e4221 = global_6.member[(_e1275 + 16u)];
                                                        let _e4224 = global_6.member[(_e1275 + 17u)];
                                                        let _e4227 = global_6.member[(_e1275 + 18u)];
                                                        let _e4230 = global_6.member[(_e1275 + 19u)];
                                                        phi_5881_ = 0u;
                                                        phi_5884_ = 0f;
                                                        phi_5886_ = 1f;
                                                        phi_5888_ = 0f;
                                                        phi_5890_ = _e4198;
                                                        loop {
                                                            let _e4232 = phi_5881_;
                                                            let _e4234 = phi_5884_;
                                                            let _e4236 = phi_5886_;
                                                            let _e4238 = phi_5888_;
                                                            let _e4240 = phi_5890_;
                                                            local_23 = _e4234;
                                                            local_24 = _e4238;
                                                            local_25 = _e4234;
                                                            let _e4241 = (_e4232 < _e4196);
                                                            if _e4241 {
                                                                let _e4244 = (_e4240.x + _e4200.x);
                                                                let _e4247 = (_e4240.y + _e4200.y);
                                                                let _e4250 = (_e4240.z + _e4200.z);
                                                                let _e4251 = floor(_e4244);
                                                                let _e4252 = floor(_e4247);
                                                                let _e4253 = floor(_e4250);
                                                                let _e4262 = bitcast<u32>((select(0i, select(select(i32(_e4251), i32(-2147483648), (_e4251 < -2147483600f)), 2147483647i, (_e4251 > 2147483500f)), (_e4251 == _e4251)) & 255i));
                                                                let _e4271 = bitcast<u32>((select(0i, select(select(i32(_e4252), i32(-2147483648), (_e4252 < -2147483600f)), 2147483647i, (_e4252 > 2147483500f)), (_e4252 == _e4252)) & 255i));
                                                                let _e4280 = bitcast<u32>((select(0i, select(select(i32(_e4253), i32(-2147483648), (_e4253 < -2147483600f)), 2147483647i, (_e4253 > 2147483500f)), (_e4253 == _e4253)) & 255i));
                                                                let _e4281 = (_e4244 - _e4251);
                                                                let _e4282 = (_e4247 - _e4252);
                                                                let _e4283 = (_e4250 - _e4253);
                                                                let _e4290 = (((_e4281 * _e4281) * _e4281) * ((_e4281 * ((_e4281 * 6f) - 15f)) + 10f));
                                                                let _e4297 = (((_e4282 * _e4282) * _e4282) * ((_e4282 * ((_e4282 * 6f) - 15f)) + 10f));
                                                                let _e4307 = global_6.member[(_e4203 + _e4262)];
                                                                let _e4313 = (select(select(u32(_e4307), 0u, (_e4307 < 0f)), 4294967295u, (_e4307 > 4294967000f)) + _e4271);
                                                                let _e4316 = global_6.member[(_e4203 + _e4313)];
                                                                let _e4322 = (select(select(u32(_e4316), 0u, (_e4316 < 0f)), 4294967295u, (_e4316 > 4294967000f)) + _e4280);
                                                                let _e4326 = global_6.member[(_e4203 + (_e4313 + 1u))];
                                                                let _e4332 = (select(select(u32(_e4326), 0u, (_e4326 < 0f)), 4294967295u, (_e4326 > 4294967000f)) + _e4280);
                                                                let _e4336 = global_6.member[(_e4203 + (_e4262 + 1u))];
                                                                let _e4342 = (select(select(u32(_e4336), 0u, (_e4336 < 0f)), 4294967295u, (_e4336 > 4294967000f)) + _e4271);
                                                                let _e4345 = global_6.member[(_e4203 + _e4342)];
                                                                let _e4351 = (select(select(u32(_e4345), 0u, (_e4345 < 0f)), 4294967295u, (_e4345 > 4294967000f)) + _e4280);
                                                                let _e4355 = global_6.member[(_e4203 + (_e4342 + 1u))];
                                                                let _e4361 = (select(select(u32(_e4355), 0u, (_e4355 < 0f)), 4294967295u, (_e4355 > 4294967000f)) + _e4280);
                                                                let _e4364 = global_6.member[(_e4203 + _e4322)];
                                                                let _e4370 = (select(select(u32(_e4364), 0u, (_e4364 < 0f)), 4294967295u, (_e4364 > 4294967000f)) % 12u);
                                                                let _e4373 = select(-1f, 1f, ((_e4370 & 1u) == 0u));
                                                                let _e4376 = select(-1f, 1f, ((_e4370 & 2u) == 0u));
                                                                if (_e4370 < 4u) {
                                                                    phi_23722_ = ((_e4373 * _e4281) + (_e4376 * _e4282));
                                                                } else {
                                                                    if (_e4370 < 8u) {
                                                                        phi_23716_ = ((_e4373 * _e4281) + (_e4376 * _e4283));
                                                                    } else {
                                                                        phi_23716_ = ((_e4373 * _e4282) + (_e4376 * _e4283));
                                                                    }
                                                                    let _e4386 = phi_23716_;
                                                                    phi_23722_ = _e4386;
                                                                }
                                                                let _e4391 = phi_23722_;
                                                                let _e4394 = global_6.member[(_e4203 + _e4351)];
                                                                let _e4400 = (_e4281 - 1f);
                                                                let _e4401 = (select(select(u32(_e4394), 0u, (_e4394 < 0f)), 4294967295u, (_e4394 > 4294967000f)) % 12u);
                                                                let _e4404 = select(-1f, 1f, ((_e4401 & 1u) == 0u));
                                                                let _e4407 = select(-1f, 1f, ((_e4401 & 2u) == 0u));
                                                                if (_e4401 < 4u) {
                                                                    phi_23756_ = ((_e4404 * _e4400) + (_e4407 * _e4282));
                                                                } else {
                                                                    if (_e4401 < 8u) {
                                                                        phi_23750_ = ((_e4404 * _e4400) + (_e4407 * _e4283));
                                                                    } else {
                                                                        phi_23750_ = ((_e4404 * _e4282) + (_e4407 * _e4283));
                                                                    }
                                                                    let _e4417 = phi_23750_;
                                                                    phi_23756_ = _e4417;
                                                                }
                                                                let _e4422 = phi_23756_;
                                                                let _e4425 = global_6.member[(_e4203 + _e4332)];
                                                                let _e4431 = (_e4282 - 1f);
                                                                let _e4432 = (select(select(u32(_e4425), 0u, (_e4425 < 0f)), 4294967295u, (_e4425 > 4294967000f)) % 12u);
                                                                let _e4435 = select(-1f, 1f, ((_e4432 & 1u) == 0u));
                                                                let _e4438 = select(-1f, 1f, ((_e4432 & 2u) == 0u));
                                                                if (_e4432 < 4u) {
                                                                    phi_23790_ = ((_e4435 * _e4281) + (_e4438 * _e4431));
                                                                } else {
                                                                    if (_e4432 < 8u) {
                                                                        phi_23784_ = ((_e4435 * _e4281) + (_e4438 * _e4283));
                                                                    } else {
                                                                        phi_23784_ = ((_e4435 * _e4431) + (_e4438 * _e4283));
                                                                    }
                                                                    let _e4448 = phi_23784_;
                                                                    phi_23790_ = _e4448;
                                                                }
                                                                let _e4453 = phi_23790_;
                                                                let _e4456 = global_6.member[(_e4203 + _e4361)];
                                                                let _e4462 = (select(select(u32(_e4456), 0u, (_e4456 < 0f)), 4294967295u, (_e4456 > 4294967000f)) % 12u);
                                                                let _e4465 = select(-1f, 1f, ((_e4462 & 1u) == 0u));
                                                                let _e4468 = select(-1f, 1f, ((_e4462 & 2u) == 0u));
                                                                if (_e4462 < 4u) {
                                                                    phi_23824_ = ((_e4465 * _e4400) + (_e4468 * _e4431));
                                                                } else {
                                                                    if (_e4462 < 8u) {
                                                                        phi_23818_ = ((_e4465 * _e4400) + (_e4468 * _e4283));
                                                                    } else {
                                                                        phi_23818_ = ((_e4465 * _e4431) + (_e4468 * _e4283));
                                                                    }
                                                                    let _e4478 = phi_23818_;
                                                                    phi_23824_ = _e4478;
                                                                }
                                                                let _e4483 = phi_23824_;
                                                                let _e4487 = global_6.member[(_e4203 + (_e4322 + 1u))];
                                                                let _e4493 = (_e4283 - 1f);
                                                                let _e4494 = (select(select(u32(_e4487), 0u, (_e4487 < 0f)), 4294967295u, (_e4487 > 4294967000f)) % 12u);
                                                                let _e4497 = select(-1f, 1f, ((_e4494 & 1u) == 0u));
                                                                let _e4500 = select(-1f, 1f, ((_e4494 & 2u) == 0u));
                                                                if (_e4494 < 4u) {
                                                                    phi_23858_ = ((_e4497 * _e4281) + (_e4500 * _e4282));
                                                                } else {
                                                                    if (_e4494 < 8u) {
                                                                        phi_23852_ = ((_e4497 * _e4281) + (_e4500 * _e4493));
                                                                    } else {
                                                                        phi_23852_ = ((_e4497 * _e4282) + (_e4500 * _e4493));
                                                                    }
                                                                    let _e4510 = phi_23852_;
                                                                    phi_23858_ = _e4510;
                                                                }
                                                                let _e4515 = phi_23858_;
                                                                let _e4519 = global_6.member[(_e4203 + (_e4351 + 1u))];
                                                                let _e4525 = (select(select(u32(_e4519), 0u, (_e4519 < 0f)), 4294967295u, (_e4519 > 4294967000f)) % 12u);
                                                                let _e4528 = select(-1f, 1f, ((_e4525 & 1u) == 0u));
                                                                let _e4531 = select(-1f, 1f, ((_e4525 & 2u) == 0u));
                                                                if (_e4525 < 4u) {
                                                                    phi_23892_ = ((_e4528 * _e4400) + (_e4531 * _e4282));
                                                                } else {
                                                                    if (_e4525 < 8u) {
                                                                        phi_23886_ = ((_e4528 * _e4400) + (_e4531 * _e4493));
                                                                    } else {
                                                                        phi_23886_ = ((_e4528 * _e4282) + (_e4531 * _e4493));
                                                                    }
                                                                    let _e4541 = phi_23886_;
                                                                    phi_23892_ = _e4541;
                                                                }
                                                                let _e4546 = phi_23892_;
                                                                let _e4550 = global_6.member[(_e4203 + (_e4332 + 1u))];
                                                                let _e4556 = (select(select(u32(_e4550), 0u, (_e4550 < 0f)), 4294967295u, (_e4550 > 4294967000f)) % 12u);
                                                                let _e4559 = select(-1f, 1f, ((_e4556 & 1u) == 0u));
                                                                let _e4562 = select(-1f, 1f, ((_e4556 & 2u) == 0u));
                                                                if (_e4556 < 4u) {
                                                                    phi_23926_ = ((_e4559 * _e4281) + (_e4562 * _e4431));
                                                                } else {
                                                                    if (_e4556 < 8u) {
                                                                        phi_23920_ = ((_e4559 * _e4281) + (_e4562 * _e4493));
                                                                    } else {
                                                                        phi_23920_ = ((_e4559 * _e4431) + (_e4562 * _e4493));
                                                                    }
                                                                    let _e4572 = phi_23920_;
                                                                    phi_23926_ = _e4572;
                                                                }
                                                                let _e4577 = phi_23926_;
                                                                let _e4581 = global_6.member[(_e4203 + (_e4361 + 1u))];
                                                                let _e4587 = (select(select(u32(_e4581), 0u, (_e4581 < 0f)), 4294967295u, (_e4581 > 4294967000f)) % 12u);
                                                                let _e4590 = select(-1f, 1f, ((_e4587 & 1u) == 0u));
                                                                let _e4593 = select(-1f, 1f, ((_e4587 & 2u) == 0u));
                                                                if (_e4587 < 4u) {
                                                                    phi_23960_ = ((_e4590 * _e4400) + (_e4593 * _e4431));
                                                                } else {
                                                                    if (_e4587 < 8u) {
                                                                        phi_23954_ = ((_e4590 * _e4400) + (_e4593 * _e4493));
                                                                    } else {
                                                                        phi_23954_ = ((_e4590 * _e4431) + (_e4593 * _e4493));
                                                                    }
                                                                    let _e4603 = phi_23954_;
                                                                    phi_23960_ = _e4603;
                                                                }
                                                                let _e4608 = phi_23960_;
                                                                let _e4611 = (_e4391 + (_e4290 * (_e4422 - _e4391)));
                                                                let _e4617 = (_e4515 + (_e4290 * (_e4546 - _e4515)));
                                                                let _e4623 = (_e4611 + (_e4297 * ((_e4453 + (_e4290 * (_e4483 - _e4453))) - _e4611)));
                                                                phi_5882_ = (_e4232 + 1u);
                                                                phi_5885_ = (_e4234 + _e4236);
                                                                phi_5887_ = (_e4236 * 0.5f);
                                                                phi_5889_ = (_e4238 + ((_e4623 + ((((_e4283 * _e4283) * _e4283) * ((_e4283 * ((_e4283 * 6f) - 15f)) + 10f)) * ((_e4617 + (_e4297 * ((_e4577 + (_e4290 * (_e4608 - _e4577))) - _e4617))) - _e4623))) * _e4236));
                                                                phi_5891_ = vec3<f32>(((((_e4206 * _e4240.x) + (_e4209 * _e4240.y)) + (_e4212 * _e4240.z)) * 2.03f), ((((_e4215 * _e4240.x) + (_e4218 * _e4240.y)) + (_e4221 * _e4240.z)) * 2.03f), ((((_e4224 * _e4240.x) + (_e4227 * _e4240.y)) + (_e4230 * _e4240.z)) * 2.03f));
                                                            } else {
                                                                phi_5882_ = u32();
                                                                phi_5885_ = f32();
                                                                phi_5887_ = f32();
                                                                phi_5889_ = f32();
                                                                phi_5891_ = vec3<f32>();
                                                            }
                                                            let _e4655 = phi_5882_;
                                                            let _e4657 = phi_5885_;
                                                            let _e4659 = phi_5887_;
                                                            let _e4661 = phi_5889_;
                                                            let _e4663 = phi_5891_;
                                                            phi_5881_ = _e4655;
                                                            phi_5884_ = _e4657;
                                                            phi_5886_ = _e4659;
                                                            phi_5888_ = _e4661;
                                                            phi_5890_ = _e4663;
                                                            if !(_e4241) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4666 = local_23;
                                                        if (_e4666 > 0f) {
                                                            let _e4669 = local_24;
                                                            let _e4671 = local_25;
                                                            phi_6307_ = (_e4669 / _e4671);
                                                        } else {
                                                            phi_6307_ = 0f;
                                                        }
                                                        let _e4674 = phi_6307_;
                                                        if (_e4124 == 0u) {
                                                            phi_6390_ = _e4120;
                                                            phi_6391_ = _e4122;
                                                            phi_6392_ = _e4126;
                                                            phi_6393_ = _e4128;
                                                            phi_6394_ = _e4130;
                                                            phi_6395_ = (_e4132 + (1.5f * _e4674));
                                                        } else {
                                                            if (_e4124 == 1u) {
                                                                phi_6384_ = _e4120;
                                                                phi_6385_ = _e4122;
                                                                phi_6386_ = _e4126;
                                                                phi_6387_ = _e4128;
                                                                phi_6388_ = _e4130;
                                                                phi_6389_ = (_e4132 + (0.8f * _e4674));
                                                            } else {
                                                                let _e4677 = (_e4124 == 2u);
                                                                if _e4677 {
                                                                    phi_6379_ = _e4120;
                                                                    phi_6380_ = _e4122;
                                                                    phi_6381_ = _e4126;
                                                                    phi_6382_ = _e4128;
                                                                } else {
                                                                    if (_e4124 == 3u) {
                                                                        phi_6375_ = _e4120;
                                                                        phi_6376_ = _e4122;
                                                                        phi_6377_ = _e4126;
                                                                        phi_6378_ = vec3<f32>(_e4674, _e4128.y, _e4128.z);
                                                                    } else {
                                                                        if (_e4124 == 4u) {
                                                                            phi_6371_ = _e4120;
                                                                            phi_6372_ = _e4122;
                                                                            phi_6373_ = _e4126;
                                                                            phi_6374_ = vec3<f32>(_e4128.x, _e4674, _e4128.z);
                                                                        } else {
                                                                            if (_e4124 == 5u) {
                                                                                phi_6367_ = _e4120;
                                                                                phi_6368_ = _e4122;
                                                                                phi_6369_ = _e4126;
                                                                                phi_6370_ = vec3<f32>(_e4128.x, _e4128.y, _e4674);
                                                                            } else {
                                                                                let _e4681 = (_e4124 == 6u);
                                                                                if _e4681 {
                                                                                    phi_6364_ = _e4120;
                                                                                    phi_6366_ = _e4126;
                                                                                } else {
                                                                                    if (_e4124 == 7u) {
                                                                                        phi_6362_ = _e4120;
                                                                                        phi_6363_ = vec3<f32>(_e4674, _e4126.y, _e4126.z);
                                                                                    } else {
                                                                                        if (_e4124 == 8u) {
                                                                                            phi_6360_ = _e4120;
                                                                                            phi_6361_ = vec3<f32>(_e4126.x, _e4674, _e4126.z);
                                                                                        } else {
                                                                                            let _e4684 = (_e4124 == 9u);
                                                                                            if _e4684 {
                                                                                                phi_6359_ = vec3<f32>(_e4126.x, _e4126.y, _e4674);
                                                                                            } else {
                                                                                                phi_6359_ = _e4126;
                                                                                            }
                                                                                            let _e4690 = phi_6359_;
                                                                                            phi_6360_ = select(_e4674, _e4120, _e4684);
                                                                                            phi_6361_ = _e4690;
                                                                                        }
                                                                                        let _e4697 = phi_6360_;
                                                                                        let _e4699 = phi_6361_;
                                                                                        phi_6362_ = _e4697;
                                                                                        phi_6363_ = _e4699;
                                                                                    }
                                                                                    let _e4705 = phi_6362_;
                                                                                    let _e4707 = phi_6363_;
                                                                                    phi_6364_ = _e4705;
                                                                                    phi_6366_ = _e4707;
                                                                                }
                                                                                let _e4709 = phi_6364_;
                                                                                let _e4711 = phi_6366_;
                                                                                phi_6367_ = _e4709;
                                                                                phi_6368_ = select(_e4122, _e4674, _e4681);
                                                                                phi_6369_ = _e4711;
                                                                                phi_6370_ = _e4128;
                                                                            }
                                                                            let _e4718 = phi_6367_;
                                                                            let _e4720 = phi_6368_;
                                                                            let _e4722 = phi_6369_;
                                                                            let _e4724 = phi_6370_;
                                                                            phi_6371_ = _e4718;
                                                                            phi_6372_ = _e4720;
                                                                            phi_6373_ = _e4722;
                                                                            phi_6374_ = _e4724;
                                                                        }
                                                                        let _e4730 = phi_6371_;
                                                                        let _e4732 = phi_6372_;
                                                                        let _e4734 = phi_6373_;
                                                                        let _e4736 = phi_6374_;
                                                                        phi_6375_ = _e4730;
                                                                        phi_6376_ = _e4732;
                                                                        phi_6377_ = _e4734;
                                                                        phi_6378_ = _e4736;
                                                                    }
                                                                    let _e4742 = phi_6375_;
                                                                    let _e4744 = phi_6376_;
                                                                    let _e4746 = phi_6377_;
                                                                    let _e4748 = phi_6378_;
                                                                    phi_6379_ = _e4742;
                                                                    phi_6380_ = _e4744;
                                                                    phi_6381_ = _e4746;
                                                                    phi_6382_ = _e4748;
                                                                }
                                                                let _e4750 = phi_6379_;
                                                                let _e4752 = phi_6380_;
                                                                let _e4754 = phi_6381_;
                                                                let _e4756 = phi_6382_;
                                                                phi_6384_ = _e4750;
                                                                phi_6385_ = _e4752;
                                                                phi_6386_ = _e4754;
                                                                phi_6387_ = _e4756;
                                                                phi_6388_ = select(_e4130, _e4674, _e4677);
                                                                phi_6389_ = _e4132;
                                                            }
                                                            let _e4761 = phi_6384_;
                                                            let _e4763 = phi_6385_;
                                                            let _e4765 = phi_6386_;
                                                            let _e4767 = phi_6387_;
                                                            let _e4769 = phi_6388_;
                                                            let _e4771 = phi_6389_;
                                                            phi_6390_ = _e4761;
                                                            phi_6391_ = _e4763;
                                                            phi_6392_ = _e4765;
                                                            phi_6393_ = _e4767;
                                                            phi_6394_ = _e4769;
                                                            phi_6395_ = _e4771;
                                                        }
                                                        let _e4775 = phi_6390_;
                                                        let _e4777 = phi_6391_;
                                                        let _e4779 = phi_6392_;
                                                        let _e4781 = phi_6393_;
                                                        let _e4783 = phi_6394_;
                                                        let _e4785 = phi_6395_;
                                                        phi_5654_ = _e4775;
                                                        phi_5657_ = _e4777;
                                                        phi_5659_ = (_e4124 + 1u);
                                                        phi_5661_ = _e4779;
                                                        phi_5663_ = _e4781;
                                                        phi_5665_ = _e4783;
                                                        phi_5667_ = _e4785;
                                                    } else {
                                                        phi_5654_ = f32();
                                                        phi_5657_ = f32();
                                                        phi_5659_ = u32();
                                                        phi_5661_ = vec3<f32>();
                                                        phi_5663_ = vec3<f32>();
                                                        phi_5665_ = f32();
                                                        phi_5667_ = f32();
                                                    }
                                                    let _e4788 = phi_5654_;
                                                    let _e4790 = phi_5657_;
                                                    let _e4792 = phi_5659_;
                                                    let _e4794 = phi_5661_;
                                                    let _e4796 = phi_5663_;
                                                    let _e4798 = phi_5665_;
                                                    let _e4800 = phi_5667_;
                                                    phi_5653_ = _e4788;
                                                    phi_5656_ = _e4790;
                                                    phi_5658_ = _e4792;
                                                    phi_5660_ = _e4794;
                                                    phi_5662_ = _e4796;
                                                    phi_5664_ = _e4798;
                                                    phi_5666_ = _e4800;
                                                    if !(_e4133) {
                                                        break;
                                                    }
                                                }
                                                if _e4097 {
                                                    let _e4815 = (exp((-(abs(_e3824)) / (0.06f + (abs(_e3835) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3835 * -0.71428573f) * (_e3835 * 0.71428573f))))));
                                                    let _e4817 = (_e3824 * 1.3333334f);
                                                    let _e4822 = (1.3f * exp((((_e3835 * _e3835) + (_e4817 * _e4817)) * -12.499999f)));
                                                    let _e4833 = local_26;
                                                    let _e4835 = local_27;
                                                    if (_e4833 != _e4835) {
                                                        phi_24036_ = true;
                                                    } else {
                                                        let _e4838 = local_28;
                                                        phi_24036_ = (0.1f >= _e4838);
                                                    }
                                                    let _e4841 = phi_24036_;
                                                    let _e4843 = local_29;
                                                    let _e4846 = (pow(select(_e4843, 0.1f, _e4841), 1.3f) * _e4055);
                                                    phi_6427_ = vec3<f32>((_e4052.x + (((_e4061 * _e4815) + (_e4070 * _e4822)) * _e4846)), (_e4052.y + (((_e4064 * _e4815) + (_e4073 * _e4822)) * _e4846)), (_e4052.z + (((_e4067 * _e4815) + (_e4076 * _e4822)) * _e4846)));
                                                } else {
                                                    phi_6427_ = _e4052;
                                                }
                                                let _e4858 = phi_6427_;
                                                if select(_e4107, _e4111, _e4113) {
                                                    let _e4860 = local_30;
                                                    let _e4866 = ((_e3824 - (0.025f * _e4860)) / (0.045f + (abs(_e3835) * 0.0095492955f)));
                                                    let _e4871 = local_31;
                                                    let _e4872 = (_e4871 * 3.3333333f);
                                                    let _e4874 = select(_e4872, 0f, (_e4872 < 0f));
                                                    let _e4876 = select(_e4874, 1f, (_e4874 > 1f));
                                                    let _e4892 = local_32;
                                                    let _e4894 = ((_e4892 - 0.25f) * 5.0000005f);
                                                    let _e4896 = select(_e4894, 0f, (_e4894 < 0f));
                                                    let _e4898 = select(_e4896, 1f, (_e4896 > 1f));
                                                    phi_6506_ = (_e4058 * (((exp((-(_e4866) * _e4866)) * (0.15f + (3f * ((_e4876 * _e4876) * (3f - (2f * _e4876)))))) * (0.5f + (0.9f * exp(((_e3835 * -0.6666667f) * (_e3835 * 0.6666667f)))))) + (1.2f * (exp(((_e3824 * -3.3333333f) * (_e3824 * 3.3333333f))) * ((_e4898 * _e4898) * (3f - (2f * _e4898)))))));
                                                } else {
                                                    phi_6506_ = 0f;
                                                }
                                                let _e4912 = phi_6506_;
                                                phi_6507_ = _e4858;
                                                phi_6508_ = _e4912;
                                            } else {
                                                phi_6507_ = _e4052;
                                                phi_6508_ = 0f;
                                            }
                                            let _e4914 = phi_6507_;
                                            let _e4916 = phi_6508_;
                                            phi_6510_ = _e4914;
                                            phi_6513_ = 0u;
                                            loop {
                                                let _e4918 = phi_6510_;
                                                let _e4920 = phi_6513_;
                                                local_49 = _e4918;
                                                local_50 = _e4918;
                                                local_51 = _e4918;
                                                let _e4921 = (_e4920 < bitcast<u32>(_e3787));
                                                if _e4921 {
                                                    let _e4923 = (bitcast<u32>(_e3783) + (_e4920 * 29u));
                                                    let _e4925 = global_6.member[_e4923];
                                                    let _e4926 = (_e4925 > 0.5f);
                                                    let _e4929 = global_6.member[(_e4923 + 1u)];
                                                    let _e4932 = global_6.member[(_e4923 + 2u)];
                                                    let _e4935 = global_6.member[(_e4923 + 3u)];
                                                    let _e4938 = global_6.member[(_e4923 + 4u)];
                                                    let _e4941 = global_6.member[(_e4923 + 5u)];
                                                    let _e4944 = global_6.member[(_e4923 + 6u)];
                                                    let _e4949 = (((_e317.member_1.x * _e4929) + (_e317.member_1.y * _e4932)) + (_e317.member_1.z * _e4935));
                                                    let _e4950 = (_e4949 < _e4941);
                                                    if _e4950 {
                                                        phi_7659_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7660_ = vec3<f32>();
                                                    } else {
                                                        let _e4953 = global_6.member[(_e4923 + 7u)];
                                                        let _e4956 = global_6.member[(_e4923 + 8u)];
                                                        let _e4959 = global_6.member[(_e4923 + 9u)];
                                                        let _e4962 = global_6.member[(_e4923 + 10u)];
                                                        let _e4965 = global_6.member[(_e4923 + 11u)];
                                                        let _e4968 = global_6.member[(_e4923 + 12u)];
                                                        let _e4971 = global_6.member[(_e4923 + 13u)];
                                                        let _e4974 = global_6.member[(_e4923 + 14u)];
                                                        let _e4977 = global_6.member[(_e4923 + 15u)];
                                                        let _e4980 = global_6.member[(_e4923 + 16u)];
                                                        let _e4983 = global_6.member[(_e4923 + 17u)];
                                                        let _e4986 = global_6.member[(_e4923 + 18u)];
                                                        let _e4989 = global_6.member[(_e4923 + 19u)];
                                                        let _e4992 = global_6.member[(_e4923 + 20u)];
                                                        let _e4995 = global_6.member[(_e4923 + 21u)];
                                                        let _e4998 = global_6.member[(_e4923 + 22u)];
                                                        let _e5001 = global_6.member[(_e4923 + 23u)];
                                                        let _e5004 = global_6.member[(_e4923 + 24u)];
                                                        let _e5007 = global_6.member[(_e4923 + 25u)];
                                                        let _e5010 = global_6.member[(_e4923 + 26u)];
                                                        let _e5013 = global_6.member[(_e4923 + 27u)];
                                                        let _e5016 = global_6.member[(_e4923 + 28u)];
                                                        let _e5023 = (((_e317.member_1.x / _e4949) - _e4929) / _e4938);
                                                        let _e5024 = (((_e317.member_1.y / _e4949) - _e4932) / _e4938);
                                                        let _e5025 = (((_e317.member_1.z / _e4949) - _e4935) / _e4938);
                                                        if _e4926 {
                                                            let _e5630 = (((_e5023 * _e4980) + (_e5024 * _e4983)) + (_e5025 * _e4986));
                                                            let _e5636 = ((((_e5023 * _e4989) + (_e5024 * _e4992)) + (_e5025 * _e4995)) / _e4998);
                                                            phi_7657_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4944 * exp((-2.5f * pow((sqrt(((_e5630 * _e5630) + (_e5636 * _e5636))) * 3.3333333f), 0.7f)))));
                                                            phi_7658_ = vec3<f32>();
                                                        } else {
                                                            let _e5030 = (((_e4953 * _e4929) + (_e4956 * _e4932)) + (_e4959 * _e4935));
                                                            if (abs(_e5030) < 0.12f) {
                                                                phi_6961_ = select(0.12f, -0.12f, (_e5030 < 0f));
                                                            } else {
                                                                phi_6961_ = _e5030;
                                                            }
                                                            let _e5036 = phi_6961_;
                                                            let _e5042 = ((((_e4953 * _e5023) + (_e4956 * _e5024)) + (_e4959 * _e5025)) / _e5036);
                                                            let _e5046 = (_e5023 - (_e4929 * _e5042));
                                                            let _e5047 = (_e5024 - (_e4932 * _e5042));
                                                            let _e5048 = (_e5025 - (_e4935 * _e5042));
                                                            let _e5053 = (((_e5046 * _e4962) + (_e5047 * _e4965)) + (_e5048 * _e4968));
                                                            let _e5058 = (((_e5046 * _e4971) + (_e5047 * _e4974)) + (_e5048 * _e4977));
                                                            let _e5062 = sqrt(((_e5053 * _e5053) + (_e5058 * _e5058)));
                                                            let _e5074 = abs(_e5036);
                                                            let _e5076 = (pow((0.5f + (0.5f * cos(((_e5001 * (atan2(_e5058, _e5053) - (log((_e5062 + 0.03f)) / _e5004))) + _e5007)))), 3f) * sqrt(_e5074));
                                                            let _e5081 = global_6.member[_e3764];
                                                            let _e5082 = bitcast<u32>(_e5081);
                                                            let _e5085 = global_6.member[(_e1275 + 11u)];
                                                            let _e5088 = global_6.member[(_e1275 + 12u)];
                                                            let _e5091 = global_6.member[(_e1275 + 13u)];
                                                            let _e5094 = global_6.member[(_e1275 + 14u)];
                                                            let _e5097 = global_6.member[(_e1275 + 15u)];
                                                            let _e5100 = global_6.member[(_e1275 + 16u)];
                                                            let _e5103 = global_6.member[(_e1275 + 17u)];
                                                            let _e5106 = global_6.member[(_e1275 + 18u)];
                                                            let _e5109 = global_6.member[(_e1275 + 19u)];
                                                            phi_7141_ = 0u;
                                                            phi_7144_ = 0f;
                                                            phi_7146_ = 1f;
                                                            phi_7148_ = 0f;
                                                            phi_7150_ = vec3<f32>((_e5053 * 9f), (_e5058 * 9f), 0f);
                                                            loop {
                                                                let _e5111 = phi_7141_;
                                                                let _e5113 = phi_7144_;
                                                                let _e5115 = phi_7146_;
                                                                let _e5117 = phi_7148_;
                                                                let _e5119 = phi_7150_;
                                                                local_33 = _e5113;
                                                                local_34 = _e5117;
                                                                local_35 = _e5113;
                                                                let _e5120 = (_e5111 < 4u);
                                                                if _e5120 {
                                                                    let _e5122 = (_e5119.x + _e5010);
                                                                    let _e5124 = (_e5119.y + _e5013);
                                                                    let _e5126 = (_e5119.z + _e5016);
                                                                    let _e5127 = floor(_e5122);
                                                                    let _e5128 = floor(_e5124);
                                                                    let _e5129 = floor(_e5126);
                                                                    let _e5138 = bitcast<u32>((select(0i, select(select(i32(_e5127), i32(-2147483648), (_e5127 < -2147483600f)), 2147483647i, (_e5127 > 2147483500f)), (_e5127 == _e5127)) & 255i));
                                                                    let _e5147 = bitcast<u32>((select(0i, select(select(i32(_e5128), i32(-2147483648), (_e5128 < -2147483600f)), 2147483647i, (_e5128 > 2147483500f)), (_e5128 == _e5128)) & 255i));
                                                                    let _e5156 = bitcast<u32>((select(0i, select(select(i32(_e5129), i32(-2147483648), (_e5129 < -2147483600f)), 2147483647i, (_e5129 > 2147483500f)), (_e5129 == _e5129)) & 255i));
                                                                    let _e5157 = (_e5122 - _e5127);
                                                                    let _e5158 = (_e5124 - _e5128);
                                                                    let _e5159 = (_e5126 - _e5129);
                                                                    let _e5166 = (((_e5157 * _e5157) * _e5157) * ((_e5157 * ((_e5157 * 6f) - 15f)) + 10f));
                                                                    let _e5173 = (((_e5158 * _e5158) * _e5158) * ((_e5158 * ((_e5158 * 6f) - 15f)) + 10f));
                                                                    let _e5183 = global_6.member[(_e5082 + _e5138)];
                                                                    let _e5189 = (select(select(u32(_e5183), 0u, (_e5183 < 0f)), 4294967295u, (_e5183 > 4294967000f)) + _e5147);
                                                                    let _e5192 = global_6.member[(_e5082 + _e5189)];
                                                                    let _e5198 = (select(select(u32(_e5192), 0u, (_e5192 < 0f)), 4294967295u, (_e5192 > 4294967000f)) + _e5156);
                                                                    let _e5202 = global_6.member[(_e5082 + (_e5189 + 1u))];
                                                                    let _e5208 = (select(select(u32(_e5202), 0u, (_e5202 < 0f)), 4294967295u, (_e5202 > 4294967000f)) + _e5156);
                                                                    let _e5212 = global_6.member[(_e5082 + (_e5138 + 1u))];
                                                                    let _e5218 = (select(select(u32(_e5212), 0u, (_e5212 < 0f)), 4294967295u, (_e5212 > 4294967000f)) + _e5147);
                                                                    let _e5221 = global_6.member[(_e5082 + _e5218)];
                                                                    let _e5227 = (select(select(u32(_e5221), 0u, (_e5221 < 0f)), 4294967295u, (_e5221 > 4294967000f)) + _e5156);
                                                                    let _e5231 = global_6.member[(_e5082 + (_e5218 + 1u))];
                                                                    let _e5237 = (select(select(u32(_e5231), 0u, (_e5231 < 0f)), 4294967295u, (_e5231 > 4294967000f)) + _e5156);
                                                                    let _e5240 = global_6.member[(_e5082 + _e5198)];
                                                                    let _e5246 = (select(select(u32(_e5240), 0u, (_e5240 < 0f)), 4294967295u, (_e5240 > 4294967000f)) % 12u);
                                                                    let _e5249 = select(-1f, 1f, ((_e5246 & 1u) == 0u));
                                                                    let _e5252 = select(-1f, 1f, ((_e5246 & 2u) == 0u));
                                                                    if (_e5246 < 4u) {
                                                                        phi_24131_ = ((_e5249 * _e5157) + (_e5252 * _e5158));
                                                                    } else {
                                                                        if (_e5246 < 8u) {
                                                                            phi_24125_ = ((_e5249 * _e5157) + (_e5252 * _e5159));
                                                                        } else {
                                                                            phi_24125_ = ((_e5249 * _e5158) + (_e5252 * _e5159));
                                                                        }
                                                                        let _e5262 = phi_24125_;
                                                                        phi_24131_ = _e5262;
                                                                    }
                                                                    let _e5267 = phi_24131_;
                                                                    let _e5270 = global_6.member[(_e5082 + _e5227)];
                                                                    let _e5276 = (_e5157 - 1f);
                                                                    let _e5277 = (select(select(u32(_e5270), 0u, (_e5270 < 0f)), 4294967295u, (_e5270 > 4294967000f)) % 12u);
                                                                    let _e5280 = select(-1f, 1f, ((_e5277 & 1u) == 0u));
                                                                    let _e5283 = select(-1f, 1f, ((_e5277 & 2u) == 0u));
                                                                    if (_e5277 < 4u) {
                                                                        phi_24165_ = ((_e5280 * _e5276) + (_e5283 * _e5158));
                                                                    } else {
                                                                        if (_e5277 < 8u) {
                                                                            phi_24159_ = ((_e5280 * _e5276) + (_e5283 * _e5159));
                                                                        } else {
                                                                            phi_24159_ = ((_e5280 * _e5158) + (_e5283 * _e5159));
                                                                        }
                                                                        let _e5293 = phi_24159_;
                                                                        phi_24165_ = _e5293;
                                                                    }
                                                                    let _e5298 = phi_24165_;
                                                                    let _e5301 = global_6.member[(_e5082 + _e5208)];
                                                                    let _e5307 = (_e5158 - 1f);
                                                                    let _e5308 = (select(select(u32(_e5301), 0u, (_e5301 < 0f)), 4294967295u, (_e5301 > 4294967000f)) % 12u);
                                                                    let _e5311 = select(-1f, 1f, ((_e5308 & 1u) == 0u));
                                                                    let _e5314 = select(-1f, 1f, ((_e5308 & 2u) == 0u));
                                                                    if (_e5308 < 4u) {
                                                                        phi_24199_ = ((_e5311 * _e5157) + (_e5314 * _e5307));
                                                                    } else {
                                                                        if (_e5308 < 8u) {
                                                                            phi_24193_ = ((_e5311 * _e5157) + (_e5314 * _e5159));
                                                                        } else {
                                                                            phi_24193_ = ((_e5311 * _e5307) + (_e5314 * _e5159));
                                                                        }
                                                                        let _e5324 = phi_24193_;
                                                                        phi_24199_ = _e5324;
                                                                    }
                                                                    let _e5329 = phi_24199_;
                                                                    let _e5332 = global_6.member[(_e5082 + _e5237)];
                                                                    let _e5338 = (select(select(u32(_e5332), 0u, (_e5332 < 0f)), 4294967295u, (_e5332 > 4294967000f)) % 12u);
                                                                    let _e5341 = select(-1f, 1f, ((_e5338 & 1u) == 0u));
                                                                    let _e5344 = select(-1f, 1f, ((_e5338 & 2u) == 0u));
                                                                    if (_e5338 < 4u) {
                                                                        phi_24233_ = ((_e5341 * _e5276) + (_e5344 * _e5307));
                                                                    } else {
                                                                        if (_e5338 < 8u) {
                                                                            phi_24227_ = ((_e5341 * _e5276) + (_e5344 * _e5159));
                                                                        } else {
                                                                            phi_24227_ = ((_e5341 * _e5307) + (_e5344 * _e5159));
                                                                        }
                                                                        let _e5354 = phi_24227_;
                                                                        phi_24233_ = _e5354;
                                                                    }
                                                                    let _e5359 = phi_24233_;
                                                                    let _e5363 = global_6.member[(_e5082 + (_e5198 + 1u))];
                                                                    let _e5369 = (_e5159 - 1f);
                                                                    let _e5370 = (select(select(u32(_e5363), 0u, (_e5363 < 0f)), 4294967295u, (_e5363 > 4294967000f)) % 12u);
                                                                    let _e5373 = select(-1f, 1f, ((_e5370 & 1u) == 0u));
                                                                    let _e5376 = select(-1f, 1f, ((_e5370 & 2u) == 0u));
                                                                    if (_e5370 < 4u) {
                                                                        phi_24267_ = ((_e5373 * _e5157) + (_e5376 * _e5158));
                                                                    } else {
                                                                        if (_e5370 < 8u) {
                                                                            phi_24261_ = ((_e5373 * _e5157) + (_e5376 * _e5369));
                                                                        } else {
                                                                            phi_24261_ = ((_e5373 * _e5158) + (_e5376 * _e5369));
                                                                        }
                                                                        let _e5386 = phi_24261_;
                                                                        phi_24267_ = _e5386;
                                                                    }
                                                                    let _e5391 = phi_24267_;
                                                                    let _e5395 = global_6.member[(_e5082 + (_e5227 + 1u))];
                                                                    let _e5401 = (select(select(u32(_e5395), 0u, (_e5395 < 0f)), 4294967295u, (_e5395 > 4294967000f)) % 12u);
                                                                    let _e5404 = select(-1f, 1f, ((_e5401 & 1u) == 0u));
                                                                    let _e5407 = select(-1f, 1f, ((_e5401 & 2u) == 0u));
                                                                    if (_e5401 < 4u) {
                                                                        phi_24301_ = ((_e5404 * _e5276) + (_e5407 * _e5158));
                                                                    } else {
                                                                        if (_e5401 < 8u) {
                                                                            phi_24295_ = ((_e5404 * _e5276) + (_e5407 * _e5369));
                                                                        } else {
                                                                            phi_24295_ = ((_e5404 * _e5158) + (_e5407 * _e5369));
                                                                        }
                                                                        let _e5417 = phi_24295_;
                                                                        phi_24301_ = _e5417;
                                                                    }
                                                                    let _e5422 = phi_24301_;
                                                                    let _e5426 = global_6.member[(_e5082 + (_e5208 + 1u))];
                                                                    let _e5432 = (select(select(u32(_e5426), 0u, (_e5426 < 0f)), 4294967295u, (_e5426 > 4294967000f)) % 12u);
                                                                    let _e5435 = select(-1f, 1f, ((_e5432 & 1u) == 0u));
                                                                    let _e5438 = select(-1f, 1f, ((_e5432 & 2u) == 0u));
                                                                    if (_e5432 < 4u) {
                                                                        phi_24335_ = ((_e5435 * _e5157) + (_e5438 * _e5307));
                                                                    } else {
                                                                        if (_e5432 < 8u) {
                                                                            phi_24329_ = ((_e5435 * _e5157) + (_e5438 * _e5369));
                                                                        } else {
                                                                            phi_24329_ = ((_e5435 * _e5307) + (_e5438 * _e5369));
                                                                        }
                                                                        let _e5448 = phi_24329_;
                                                                        phi_24335_ = _e5448;
                                                                    }
                                                                    let _e5453 = phi_24335_;
                                                                    let _e5457 = global_6.member[(_e5082 + (_e5237 + 1u))];
                                                                    let _e5463 = (select(select(u32(_e5457), 0u, (_e5457 < 0f)), 4294967295u, (_e5457 > 4294967000f)) % 12u);
                                                                    let _e5466 = select(-1f, 1f, ((_e5463 & 1u) == 0u));
                                                                    let _e5469 = select(-1f, 1f, ((_e5463 & 2u) == 0u));
                                                                    if (_e5463 < 4u) {
                                                                        phi_24369_ = ((_e5466 * _e5276) + (_e5469 * _e5307));
                                                                    } else {
                                                                        if (_e5463 < 8u) {
                                                                            phi_24363_ = ((_e5466 * _e5276) + (_e5469 * _e5369));
                                                                        } else {
                                                                            phi_24363_ = ((_e5466 * _e5307) + (_e5469 * _e5369));
                                                                        }
                                                                        let _e5479 = phi_24363_;
                                                                        phi_24369_ = _e5479;
                                                                    }
                                                                    let _e5484 = phi_24369_;
                                                                    let _e5487 = (_e5267 + (_e5166 * (_e5298 - _e5267)));
                                                                    let _e5493 = (_e5391 + (_e5166 * (_e5422 - _e5391)));
                                                                    let _e5499 = (_e5487 + (_e5173 * ((_e5329 + (_e5166 * (_e5359 - _e5329))) - _e5487)));
                                                                    phi_7142_ = (_e5111 + 1u);
                                                                    phi_7145_ = (_e5113 + _e5115);
                                                                    phi_7147_ = (_e5115 * 0.5f);
                                                                    phi_7149_ = (_e5117 + ((_e5499 + ((((_e5159 * _e5159) * _e5159) * ((_e5159 * ((_e5159 * 6f) - 15f)) + 10f)) * ((_e5493 + (_e5173 * ((_e5453 + (_e5166 * (_e5484 - _e5453))) - _e5493))) - _e5499))) * _e5115));
                                                                    phi_7151_ = vec3<f32>(((((_e5085 * _e5119.x) + (_e5088 * _e5119.y)) + (_e5091 * _e5119.z)) * 2.03f), ((((_e5094 * _e5119.x) + (_e5097 * _e5119.y)) + (_e5100 * _e5119.z)) * 2.03f), ((((_e5103 * _e5119.x) + (_e5106 * _e5119.y)) + (_e5109 * _e5119.z)) * 2.03f));
                                                                } else {
                                                                    phi_7142_ = u32();
                                                                    phi_7145_ = f32();
                                                                    phi_7147_ = f32();
                                                                    phi_7149_ = f32();
                                                                    phi_7151_ = vec3<f32>();
                                                                }
                                                                let _e5531 = phi_7142_;
                                                                let _e5533 = phi_7145_;
                                                                let _e5535 = phi_7147_;
                                                                let _e5537 = phi_7149_;
                                                                let _e5539 = phi_7151_;
                                                                phi_7141_ = _e5531;
                                                                phi_7144_ = _e5533;
                                                                phi_7146_ = _e5535;
                                                                phi_7148_ = _e5537;
                                                                phi_7150_ = _e5539;
                                                                if !(_e5120) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5542 = local_33;
                                                            if (_e5542 > 0f) {
                                                                let _e5545 = local_34;
                                                                let _e5547 = local_35;
                                                                phi_7564_ = (_e5545 / _e5547);
                                                            } else {
                                                                phi_7564_ = 0f;
                                                            }
                                                            let _e5550 = phi_7564_;
                                                            let _e5552 = (1f + (1.2f * _e5550));
                                                            if (_e5552 != _e5552) {
                                                                phi_24415_ = true;
                                                            } else {
                                                                phi_24415_ = (0f >= _e5552);
                                                            }
                                                            let _e5556 = phi_24415_;
                                                            let _e5561 = ((_e5062 - 1.15f) * -2.8571432f);
                                                            let _e5563 = select(_e5561, 0f, (_e5561 < 0f));
                                                            let _e5565 = select(_e5563, 1f, (_e5563 > 1f));
                                                            let _e5571 = (1f / _e5074);
                                                            if (_e5571 != _e5571) {
                                                                phi_24430_ = true;
                                                            } else {
                                                                phi_24430_ = (4f <= _e5571);
                                                            }
                                                            let _e5575 = phi_24430_;
                                                            let _e5578 = (((exp((_e5062 * -3.5714285f)) * ((_e5565 * _e5565) * (3f - (2f * _e5565)))) * select(_e5571, 4f, _e5575)) * 0.12f);
                                                            let _e5584 = sqrt((((_e5023 * _e5023) + (_e5024 * _e5024)) + (_e5025 * _e5025)));
                                                            let _e5591 = (((_e5023 * _e4989) + (_e5024 * _e4992)) + (_e5025 * _e4995));
                                                            let _e5592 = (1f - _e5074);
                                                            let _e5608 = ((_e5578 * (0.6f + (0.4f * (1f - _e5076)))) + exp((_e5584 * -16.666668f)));
                                                            let _e5614 = (((_e5578 * _e5076) * 1.8f) * select(_e5552, 0f, _e5556));
                                                            let _e5621 = (_e4944 * (1f - ((((0.75f * _e5592) * _e5592) * exp(((_e5591 * -28.571428f) * (_e5591 * 28.571428f)))) * exp((_e5584 * -1.4285715f)))));
                                                            phi_7657_ = vec3<f32>();
                                                            phi_7658_ = vec3<f32>((((1.1633643f * _e5608) + (0.86689025f * _e5614)) * _e5621), (((0.97722596f * _e5608) + (1.0155001f * _e5614)) * _e5621), (((0.74455315f * _e5608) + (1.2384146f * _e5614)) * _e5621));
                                                        }
                                                        let _e5648 = phi_7657_;
                                                        let _e5650 = phi_7658_;
                                                        phi_7659_ = _e5648;
                                                        phi_7660_ = _e5650;
                                                    }
                                                    let _e5652 = phi_7659_;
                                                    let _e5654 = phi_7660_;
                                                    phi_6511_ = (_e4918 + select(_e5654, _e5652, vec3(select(_e4926, true, _e4950))));
                                                    phi_6514_ = (_e4920 + 1u);
                                                } else {
                                                    phi_6511_ = vec3<f32>();
                                                    phi_6514_ = u32();
                                                }
                                                let _e5661 = phi_6511_;
                                                let _e5663 = phi_6514_;
                                                phi_6510_ = _e5661;
                                                phi_6513_ = _e5663;
                                                if !(_e4921) {
                                                    break;
                                                }
                                            }
                                            phi_32823_ = _e4050;
                                            if _e4050 {
                                                break;
                                            }
                                            phi_7679_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7682_ = 0u;
                                            phi_7684_ = _e4916;
                                            loop {
                                                let _e5666 = phi_7679_;
                                                let _e5668 = phi_7682_;
                                                let _e5670 = phi_7684_;
                                                local_45 = _e5670;
                                                local_46 = _e5670;
                                                local_47 = _e5670;
                                                local_48 = _e5670;
                                                local_52 = _e5666;
                                                local_53 = _e5666;
                                                local_54 = _e5666;
                                                let _e5671 = (_e5668 < bitcast<u32>(_e3779));
                                                if _e5671 {
                                                    let _e5673 = (bitcast<u32>(_e3775) + (_e5668 * 18u));
                                                    let _e5675 = global_6.member[_e5673];
                                                    let _e5680 = select(select(u32(_e5675), 0u, (_e5675 < 0f)), 4294967295u, (_e5675 > 4294967000f));
                                                    let _e5683 = global_6.member[(_e5673 + 1u)];
                                                    let _e5686 = global_6.member[(_e5673 + 2u)];
                                                    let _e5689 = global_6.member[(_e5673 + 3u)];
                                                    let _e5692 = global_6.member[(_e5673 + 4u)];
                                                    let _e5695 = global_6.member[(_e5673 + 5u)];
                                                    let _e5698 = global_6.member[(_e5673 + 6u)];
                                                    let _e5701 = global_6.member[(_e5673 + 7u)];
                                                    let _e5704 = global_6.member[(_e5673 + 8u)];
                                                    let _e5707 = global_6.member[(_e5673 + 9u)];
                                                    let _e5710 = global_6.member[(_e5673 + 10u)];
                                                    let _e5713 = global_6.member[(_e5673 + 11u)];
                                                    let _e5716 = global_6.member[(_e5673 + 12u)];
                                                    let _e5719 = global_6.member[(_e5673 + 13u)];
                                                    let _e5722 = global_6.member[(_e5673 + 14u)];
                                                    let _e5725 = global_6.member[(_e5673 + 15u)];
                                                    let _e5728 = global_6.member[(_e5673 + 16u)];
                                                    let _e5731 = global_6.member[(_e5673 + 17u)];
                                                    let _e5736 = (((_e317.member_1.x * _e5683) + (_e317.member_1.y * _e5686)) + (_e317.member_1.z * _e5689));
                                                    if (_e5736 < _e5695) {
                                                        phi_9001_ = type_56(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        let _e5741 = ((_e317.member_1.x / _e5736) - _e5683);
                                                        let _e5742 = ((_e317.member_1.y / _e5736) - _e5686);
                                                        let _e5743 = ((_e317.member_1.z / _e5736) - _e5689);
                                                        let _e5750 = ((((_e5741 * _e5704) + (_e5742 * _e5707)) + (_e5743 * _e5710)) / (_e5692 * _e5722));
                                                        let _e5756 = ((((_e5741 * _e5713) + (_e5742 * _e5716)) + (_e5743 * _e5719)) / _e5692);
                                                        let _e5760 = sqrt(((_e5750 * _e5750) + (_e5756 * _e5756)));
                                                        let _e5763 = vec3<f32>((_e5750 * 2.2f), (_e5756 * 2.2f), 0f);
                                                        let _e5764 = (_e5680 == 0u);
                                                        if _e5764 {
                                                            phi_7986_ = 6u;
                                                            phi_7987_ = 1.2f;
                                                            phi_7988_ = _e5763;
                                                        } else {
                                                            let _e5765 = (_e5680 == 1u);
                                                            if _e5765 {
                                                                phi_7985_ = vec3<f32>((_e5750 * 1.54f), (_e5756 * 1.54f), 0f);
                                                            } else {
                                                                phi_7985_ = _e5763;
                                                            }
                                                            let _e5770 = phi_7985_;
                                                            phi_7986_ = select(6u, 5u, _e5765);
                                                            phi_7987_ = select(1f, 0.8f, _e5765);
                                                            phi_7988_ = _e5770;
                                                        }
                                                        let _e5774 = phi_7986_;
                                                        let _e5776 = phi_7987_;
                                                        let _e5778 = phi_7988_;
                                                        if _e5764 {
                                                            phi_8005_ = vec3<f32>(((_e5750 * 3.7400002f) + -7f), ((_e5756 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_8005_ = vec3<f32>((_e5750 * 3.3000002f), (_e5756 * 3.3000002f), 0f);
                                                        }
                                                        let _e5788 = phi_8005_;
                                                        phi_8007_ = 0u;
                                                        phi_8010_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8012_ = 0f;
                                                        phi_8014_ = 0f;
                                                        phi_8016_ = 0f;
                                                        loop {
                                                            let _e5791 = phi_8007_;
                                                            let _e5793 = phi_8010_;
                                                            let _e5795 = phi_8012_;
                                                            let _e5797 = phi_8014_;
                                                            let _e5799 = phi_8016_;
                                                            local_39 = _e5799;
                                                            local_40 = _e5797;
                                                            local_41 = _e5795;
                                                            local_42 = _e5797;
                                                            local_43 = _e5797;
                                                            local_44 = _e5795;
                                                            let _e5800 = (_e5791 < 6u);
                                                            if _e5800 {
                                                                let _e5801 = (_e5791 == 0u);
                                                                if _e5801 {
                                                                    phi_8058_ = true;
                                                                    phi_8059_ = false;
                                                                    phi_8060_ = false;
                                                                } else {
                                                                    if (_e5791 <= 4u) {
                                                                        phi_8036_ = (_e5680 == 2u);
                                                                    } else {
                                                                        phi_8036_ = true;
                                                                    }
                                                                    let _e5805 = phi_8036_;
                                                                    if _e5805 {
                                                                        let _e5806 = (_e5791 == 5u);
                                                                        if _e5806 {
                                                                            if _e5764 {
                                                                                phi_8052_ = false;
                                                                            } else {
                                                                                phi_8052_ = (_e5680 == 2u);
                                                                            }
                                                                            let _e5809 = phi_8052_;
                                                                            phi_8054_ = _e5809;
                                                                        } else {
                                                                            phi_8054_ = false;
                                                                        }
                                                                        let _e5811 = phi_8054_;
                                                                        phi_8056_ = select(false, _e5764, _e5806);
                                                                        phi_8057_ = _e5811;
                                                                    } else {
                                                                        phi_8056_ = false;
                                                                        phi_8057_ = false;
                                                                    }
                                                                    let _e5814 = phi_8056_;
                                                                    let _e5816 = phi_8057_;
                                                                    phi_8058_ = select(true, false, _e5805);
                                                                    phi_8059_ = _e5814;
                                                                    phi_8060_ = _e5816;
                                                                }
                                                                let _e5819 = phi_8058_;
                                                                let _e5821 = phi_8059_;
                                                                let _e5823 = phi_8060_;
                                                                if select(select(_e5823, false, _e5819), true, select(_e5821, true, _e5819)) {
                                                                    if _e5801 {
                                                                        phi_8107_ = 3u;
                                                                        phi_8108_ = vec3<f32>(((_e5750 * 1.7600001f) + 3f), ((_e5756 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5791 < 4u) {
                                                                            let _e5840 = (_e5791 - 1u);
                                                                            if (_e5840 == 0u) {
                                                                                phi_24449_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_24449_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5840 == 1u)));
                                                                            }
                                                                            let _e5846 = phi_24449_;
                                                                            phi_8105_ = 4u;
                                                                            phi_8106_ = (_e5778 + _e5846);
                                                                        } else {
                                                                            let _e5828 = (_e5791 == 4u);
                                                                            if _e5828 {
                                                                                phi_8104_ = (_e5778 + vec3<f32>((_e5793.x * _e5776), (_e5793.y * _e5776), (_e5793.z * _e5776)));
                                                                            } else {
                                                                                phi_8104_ = _e5788;
                                                                            }
                                                                            let _e5838 = phi_8104_;
                                                                            phi_8105_ = select(select(4u, 5u, _e5764), _e5774, _e5828);
                                                                            phi_8106_ = _e5838;
                                                                        }
                                                                        let _e5849 = phi_8105_;
                                                                        let _e5851 = phi_8106_;
                                                                        phi_8107_ = _e5849;
                                                                        phi_8108_ = _e5851;
                                                                    }
                                                                    let _e5858 = phi_8107_;
                                                                    let _e5860 = phi_8108_;
                                                                    let _e5862 = global_6.member[_e3764];
                                                                    let _e5863 = bitcast<u32>(_e5862);
                                                                    let _e5866 = global_6.member[(_e1275 + 11u)];
                                                                    let _e5869 = global_6.member[(_e1275 + 12u)];
                                                                    let _e5872 = global_6.member[(_e1275 + 13u)];
                                                                    let _e5875 = global_6.member[(_e1275 + 14u)];
                                                                    let _e5878 = global_6.member[(_e1275 + 15u)];
                                                                    let _e5881 = global_6.member[(_e1275 + 16u)];
                                                                    let _e5884 = global_6.member[(_e1275 + 17u)];
                                                                    let _e5887 = global_6.member[(_e1275 + 18u)];
                                                                    let _e5890 = global_6.member[(_e1275 + 19u)];
                                                                    phi_8243_ = 0u;
                                                                    phi_8246_ = 0f;
                                                                    phi_8248_ = 1f;
                                                                    phi_8250_ = 0f;
                                                                    phi_8252_ = _e5860;
                                                                    loop {
                                                                        let _e5892 = phi_8243_;
                                                                        let _e5894 = phi_8246_;
                                                                        let _e5896 = phi_8248_;
                                                                        let _e5898 = phi_8250_;
                                                                        let _e5900 = phi_8252_;
                                                                        local_36 = _e5894;
                                                                        local_37 = _e5898;
                                                                        local_38 = _e5894;
                                                                        let _e5901 = (_e5892 < _e5858);
                                                                        if _e5901 {
                                                                            let _e5903 = (_e5900.x + _e5725);
                                                                            let _e5905 = (_e5900.y + _e5728);
                                                                            let _e5907 = (_e5900.z + _e5731);
                                                                            let _e5908 = floor(_e5903);
                                                                            let _e5909 = floor(_e5905);
                                                                            let _e5910 = floor(_e5907);
                                                                            let _e5919 = bitcast<u32>((select(0i, select(select(i32(_e5908), i32(-2147483648), (_e5908 < -2147483600f)), 2147483647i, (_e5908 > 2147483500f)), (_e5908 == _e5908)) & 255i));
                                                                            let _e5928 = bitcast<u32>((select(0i, select(select(i32(_e5909), i32(-2147483648), (_e5909 < -2147483600f)), 2147483647i, (_e5909 > 2147483500f)), (_e5909 == _e5909)) & 255i));
                                                                            let _e5937 = bitcast<u32>((select(0i, select(select(i32(_e5910), i32(-2147483648), (_e5910 < -2147483600f)), 2147483647i, (_e5910 > 2147483500f)), (_e5910 == _e5910)) & 255i));
                                                                            let _e5938 = (_e5903 - _e5908);
                                                                            let _e5939 = (_e5905 - _e5909);
                                                                            let _e5940 = (_e5907 - _e5910);
                                                                            let _e5947 = (((_e5938 * _e5938) * _e5938) * ((_e5938 * ((_e5938 * 6f) - 15f)) + 10f));
                                                                            let _e5954 = (((_e5939 * _e5939) * _e5939) * ((_e5939 * ((_e5939 * 6f) - 15f)) + 10f));
                                                                            let _e5964 = global_6.member[(_e5863 + _e5919)];
                                                                            let _e5970 = (select(select(u32(_e5964), 0u, (_e5964 < 0f)), 4294967295u, (_e5964 > 4294967000f)) + _e5928);
                                                                            let _e5973 = global_6.member[(_e5863 + _e5970)];
                                                                            let _e5979 = (select(select(u32(_e5973), 0u, (_e5973 < 0f)), 4294967295u, (_e5973 > 4294967000f)) + _e5937);
                                                                            let _e5983 = global_6.member[(_e5863 + (_e5970 + 1u))];
                                                                            let _e5989 = (select(select(u32(_e5983), 0u, (_e5983 < 0f)), 4294967295u, (_e5983 > 4294967000f)) + _e5937);
                                                                            let _e5993 = global_6.member[(_e5863 + (_e5919 + 1u))];
                                                                            let _e5999 = (select(select(u32(_e5993), 0u, (_e5993 < 0f)), 4294967295u, (_e5993 > 4294967000f)) + _e5928);
                                                                            let _e6002 = global_6.member[(_e5863 + _e5999)];
                                                                            let _e6008 = (select(select(u32(_e6002), 0u, (_e6002 < 0f)), 4294967295u, (_e6002 > 4294967000f)) + _e5937);
                                                                            let _e6012 = global_6.member[(_e5863 + (_e5999 + 1u))];
                                                                            let _e6018 = (select(select(u32(_e6012), 0u, (_e6012 < 0f)), 4294967295u, (_e6012 > 4294967000f)) + _e5937);
                                                                            let _e6021 = global_6.member[(_e5863 + _e5979)];
                                                                            let _e6027 = (select(select(u32(_e6021), 0u, (_e6021 < 0f)), 4294967295u, (_e6021 > 4294967000f)) % 12u);
                                                                            let _e6030 = select(-1f, 1f, ((_e6027 & 1u) == 0u));
                                                                            let _e6033 = select(-1f, 1f, ((_e6027 & 2u) == 0u));
                                                                            if (_e6027 < 4u) {
                                                                                phi_24510_ = ((_e6030 * _e5938) + (_e6033 * _e5939));
                                                                            } else {
                                                                                if (_e6027 < 8u) {
                                                                                    phi_24504_ = ((_e6030 * _e5938) + (_e6033 * _e5940));
                                                                                } else {
                                                                                    phi_24504_ = ((_e6030 * _e5939) + (_e6033 * _e5940));
                                                                                }
                                                                                let _e6043 = phi_24504_;
                                                                                phi_24510_ = _e6043;
                                                                            }
                                                                            let _e6048 = phi_24510_;
                                                                            let _e6051 = global_6.member[(_e5863 + _e6008)];
                                                                            let _e6057 = (_e5938 - 1f);
                                                                            let _e6058 = (select(select(u32(_e6051), 0u, (_e6051 < 0f)), 4294967295u, (_e6051 > 4294967000f)) % 12u);
                                                                            let _e6061 = select(-1f, 1f, ((_e6058 & 1u) == 0u));
                                                                            let _e6064 = select(-1f, 1f, ((_e6058 & 2u) == 0u));
                                                                            if (_e6058 < 4u) {
                                                                                phi_24544_ = ((_e6061 * _e6057) + (_e6064 * _e5939));
                                                                            } else {
                                                                                if (_e6058 < 8u) {
                                                                                    phi_24538_ = ((_e6061 * _e6057) + (_e6064 * _e5940));
                                                                                } else {
                                                                                    phi_24538_ = ((_e6061 * _e5939) + (_e6064 * _e5940));
                                                                                }
                                                                                let _e6074 = phi_24538_;
                                                                                phi_24544_ = _e6074;
                                                                            }
                                                                            let _e6079 = phi_24544_;
                                                                            let _e6082 = global_6.member[(_e5863 + _e5989)];
                                                                            let _e6088 = (_e5939 - 1f);
                                                                            let _e6089 = (select(select(u32(_e6082), 0u, (_e6082 < 0f)), 4294967295u, (_e6082 > 4294967000f)) % 12u);
                                                                            let _e6092 = select(-1f, 1f, ((_e6089 & 1u) == 0u));
                                                                            let _e6095 = select(-1f, 1f, ((_e6089 & 2u) == 0u));
                                                                            if (_e6089 < 4u) {
                                                                                phi_24578_ = ((_e6092 * _e5938) + (_e6095 * _e6088));
                                                                            } else {
                                                                                if (_e6089 < 8u) {
                                                                                    phi_24572_ = ((_e6092 * _e5938) + (_e6095 * _e5940));
                                                                                } else {
                                                                                    phi_24572_ = ((_e6092 * _e6088) + (_e6095 * _e5940));
                                                                                }
                                                                                let _e6105 = phi_24572_;
                                                                                phi_24578_ = _e6105;
                                                                            }
                                                                            let _e6110 = phi_24578_;
                                                                            let _e6113 = global_6.member[(_e5863 + _e6018)];
                                                                            let _e6119 = (select(select(u32(_e6113), 0u, (_e6113 < 0f)), 4294967295u, (_e6113 > 4294967000f)) % 12u);
                                                                            let _e6122 = select(-1f, 1f, ((_e6119 & 1u) == 0u));
                                                                            let _e6125 = select(-1f, 1f, ((_e6119 & 2u) == 0u));
                                                                            if (_e6119 < 4u) {
                                                                                phi_24612_ = ((_e6122 * _e6057) + (_e6125 * _e6088));
                                                                            } else {
                                                                                if (_e6119 < 8u) {
                                                                                    phi_24606_ = ((_e6122 * _e6057) + (_e6125 * _e5940));
                                                                                } else {
                                                                                    phi_24606_ = ((_e6122 * _e6088) + (_e6125 * _e5940));
                                                                                }
                                                                                let _e6135 = phi_24606_;
                                                                                phi_24612_ = _e6135;
                                                                            }
                                                                            let _e6140 = phi_24612_;
                                                                            let _e6144 = global_6.member[(_e5863 + (_e5979 + 1u))];
                                                                            let _e6150 = (_e5940 - 1f);
                                                                            let _e6151 = (select(select(u32(_e6144), 0u, (_e6144 < 0f)), 4294967295u, (_e6144 > 4294967000f)) % 12u);
                                                                            let _e6154 = select(-1f, 1f, ((_e6151 & 1u) == 0u));
                                                                            let _e6157 = select(-1f, 1f, ((_e6151 & 2u) == 0u));
                                                                            if (_e6151 < 4u) {
                                                                                phi_24646_ = ((_e6154 * _e5938) + (_e6157 * _e5939));
                                                                            } else {
                                                                                if (_e6151 < 8u) {
                                                                                    phi_24640_ = ((_e6154 * _e5938) + (_e6157 * _e6150));
                                                                                } else {
                                                                                    phi_24640_ = ((_e6154 * _e5939) + (_e6157 * _e6150));
                                                                                }
                                                                                let _e6167 = phi_24640_;
                                                                                phi_24646_ = _e6167;
                                                                            }
                                                                            let _e6172 = phi_24646_;
                                                                            let _e6176 = global_6.member[(_e5863 + (_e6008 + 1u))];
                                                                            let _e6182 = (select(select(u32(_e6176), 0u, (_e6176 < 0f)), 4294967295u, (_e6176 > 4294967000f)) % 12u);
                                                                            let _e6185 = select(-1f, 1f, ((_e6182 & 1u) == 0u));
                                                                            let _e6188 = select(-1f, 1f, ((_e6182 & 2u) == 0u));
                                                                            if (_e6182 < 4u) {
                                                                                phi_24680_ = ((_e6185 * _e6057) + (_e6188 * _e5939));
                                                                            } else {
                                                                                if (_e6182 < 8u) {
                                                                                    phi_24674_ = ((_e6185 * _e6057) + (_e6188 * _e6150));
                                                                                } else {
                                                                                    phi_24674_ = ((_e6185 * _e5939) + (_e6188 * _e6150));
                                                                                }
                                                                                let _e6198 = phi_24674_;
                                                                                phi_24680_ = _e6198;
                                                                            }
                                                                            let _e6203 = phi_24680_;
                                                                            let _e6207 = global_6.member[(_e5863 + (_e5989 + 1u))];
                                                                            let _e6213 = (select(select(u32(_e6207), 0u, (_e6207 < 0f)), 4294967295u, (_e6207 > 4294967000f)) % 12u);
                                                                            let _e6216 = select(-1f, 1f, ((_e6213 & 1u) == 0u));
                                                                            let _e6219 = select(-1f, 1f, ((_e6213 & 2u) == 0u));
                                                                            if (_e6213 < 4u) {
                                                                                phi_24714_ = ((_e6216 * _e5938) + (_e6219 * _e6088));
                                                                            } else {
                                                                                if (_e6213 < 8u) {
                                                                                    phi_24708_ = ((_e6216 * _e5938) + (_e6219 * _e6150));
                                                                                } else {
                                                                                    phi_24708_ = ((_e6216 * _e6088) + (_e6219 * _e6150));
                                                                                }
                                                                                let _e6229 = phi_24708_;
                                                                                phi_24714_ = _e6229;
                                                                            }
                                                                            let _e6234 = phi_24714_;
                                                                            let _e6238 = global_6.member[(_e5863 + (_e6018 + 1u))];
                                                                            let _e6244 = (select(select(u32(_e6238), 0u, (_e6238 < 0f)), 4294967295u, (_e6238 > 4294967000f)) % 12u);
                                                                            let _e6247 = select(-1f, 1f, ((_e6244 & 1u) == 0u));
                                                                            let _e6250 = select(-1f, 1f, ((_e6244 & 2u) == 0u));
                                                                            if (_e6244 < 4u) {
                                                                                phi_24748_ = ((_e6247 * _e6057) + (_e6250 * _e6088));
                                                                            } else {
                                                                                if (_e6244 < 8u) {
                                                                                    phi_24742_ = ((_e6247 * _e6057) + (_e6250 * _e6150));
                                                                                } else {
                                                                                    phi_24742_ = ((_e6247 * _e6088) + (_e6250 * _e6150));
                                                                                }
                                                                                let _e6260 = phi_24742_;
                                                                                phi_24748_ = _e6260;
                                                                            }
                                                                            let _e6265 = phi_24748_;
                                                                            let _e6268 = (_e6048 + (_e5947 * (_e6079 - _e6048)));
                                                                            let _e6274 = (_e6172 + (_e5947 * (_e6203 - _e6172)));
                                                                            let _e6280 = (_e6268 + (_e5954 * ((_e6110 + (_e5947 * (_e6140 - _e6110))) - _e6268)));
                                                                            phi_8244_ = (_e5892 + 1u);
                                                                            phi_8247_ = (_e5894 + _e5896);
                                                                            phi_8249_ = (_e5896 * 0.5f);
                                                                            phi_8251_ = (_e5898 + ((_e6280 + ((((_e5940 * _e5940) * _e5940) * ((_e5940 * ((_e5940 * 6f) - 15f)) + 10f)) * ((_e6274 + (_e5954 * ((_e6234 + (_e5947 * (_e6265 - _e6234))) - _e6274))) - _e6280))) * _e5896));
                                                                            phi_8253_ = vec3<f32>(((((_e5866 * _e5900.x) + (_e5869 * _e5900.y)) + (_e5872 * _e5900.z)) * 2.03f), ((((_e5875 * _e5900.x) + (_e5878 * _e5900.y)) + (_e5881 * _e5900.z)) * 2.03f), ((((_e5884 * _e5900.x) + (_e5887 * _e5900.y)) + (_e5890 * _e5900.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8244_ = u32();
                                                                            phi_8247_ = f32();
                                                                            phi_8249_ = f32();
                                                                            phi_8251_ = f32();
                                                                            phi_8253_ = vec3<f32>();
                                                                        }
                                                                        let _e6312 = phi_8244_;
                                                                        let _e6314 = phi_8247_;
                                                                        let _e6316 = phi_8249_;
                                                                        let _e6318 = phi_8251_;
                                                                        let _e6320 = phi_8253_;
                                                                        phi_8243_ = _e6312;
                                                                        phi_8246_ = _e6314;
                                                                        phi_8248_ = _e6316;
                                                                        phi_8250_ = _e6318;
                                                                        phi_8252_ = _e6320;
                                                                        if !(_e5901) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6323 = local_36;
                                                                    if (_e6323 > 0f) {
                                                                        let _e6326 = local_37;
                                                                        let _e6328 = local_38;
                                                                        phi_8666_ = (_e6326 / _e6328);
                                                                    } else {
                                                                        phi_8666_ = 0f;
                                                                    }
                                                                    let _e6331 = phi_8666_;
                                                                    if _e5801 {
                                                                        phi_8701_ = _e5793;
                                                                        phi_8702_ = _e5795;
                                                                        phi_8703_ = _e5797;
                                                                    } else {
                                                                        if (_e5791 == 1u) {
                                                                            phi_8698_ = vec3<f32>(_e6331, _e5793.y, _e5793.z);
                                                                            phi_8699_ = _e5795;
                                                                            phi_8700_ = _e5797;
                                                                        } else {
                                                                            if (_e5791 == 2u) {
                                                                                phi_8695_ = vec3<f32>(_e5793.x, _e6331, _e5793.z);
                                                                                phi_8696_ = _e5795;
                                                                                phi_8697_ = _e5797;
                                                                            } else {
                                                                                if (_e5791 == 3u) {
                                                                                    phi_8692_ = vec3<f32>(_e5793.x, _e5793.y, _e6331);
                                                                                    phi_8693_ = _e5795;
                                                                                    phi_8694_ = _e5797;
                                                                                } else {
                                                                                    let _e6335 = (_e5791 == 4u);
                                                                                    phi_8692_ = _e5793;
                                                                                    phi_8693_ = select(_e6331, _e5795, _e6335);
                                                                                    phi_8694_ = select(_e5797, _e6331, _e6335);
                                                                                }
                                                                                let _e6343 = phi_8692_;
                                                                                let _e6345 = phi_8693_;
                                                                                let _e6347 = phi_8694_;
                                                                                phi_8695_ = _e6343;
                                                                                phi_8696_ = _e6345;
                                                                                phi_8697_ = _e6347;
                                                                            }
                                                                            let _e6353 = phi_8695_;
                                                                            let _e6355 = phi_8696_;
                                                                            let _e6357 = phi_8697_;
                                                                            phi_8698_ = _e6353;
                                                                            phi_8699_ = _e6355;
                                                                            phi_8700_ = _e6357;
                                                                        }
                                                                        let _e6363 = phi_8698_;
                                                                        let _e6365 = phi_8699_;
                                                                        let _e6367 = phi_8700_;
                                                                        phi_8701_ = _e6363;
                                                                        phi_8702_ = _e6365;
                                                                        phi_8703_ = _e6367;
                                                                    }
                                                                    let _e6369 = phi_8701_;
                                                                    let _e6371 = phi_8702_;
                                                                    let _e6373 = phi_8703_;
                                                                    phi_8705_ = _e6369;
                                                                    phi_8706_ = _e6371;
                                                                    phi_8707_ = _e6373;
                                                                    phi_8708_ = select(_e5799, _e6331, _e5801);
                                                                } else {
                                                                    phi_8705_ = _e5793;
                                                                    phi_8706_ = _e5795;
                                                                    phi_8707_ = _e5797;
                                                                    phi_8708_ = _e5799;
                                                                }
                                                                let _e6376 = phi_8705_;
                                                                let _e6378 = phi_8706_;
                                                                let _e6380 = phi_8707_;
                                                                let _e6382 = phi_8708_;
                                                                phi_8008_ = (_e5791 + 1u);
                                                                phi_8011_ = _e6376;
                                                                phi_8013_ = _e6378;
                                                                phi_8015_ = _e6380;
                                                                phi_8017_ = _e6382;
                                                            } else {
                                                                phi_8008_ = u32();
                                                                phi_8011_ = vec3<f32>();
                                                                phi_8013_ = f32();
                                                                phi_8015_ = f32();
                                                                phi_8017_ = f32();
                                                            }
                                                            let _e6385 = phi_8008_;
                                                            let _e6387 = phi_8011_;
                                                            let _e6389 = phi_8013_;
                                                            let _e6391 = phi_8015_;
                                                            let _e6393 = phi_8017_;
                                                            phi_8007_ = _e6385;
                                                            phi_8010_ = _e6387;
                                                            phi_8012_ = _e6389;
                                                            phi_8014_ = _e6391;
                                                            phi_8016_ = _e6393;
                                                            if !(_e5800) {
                                                                break;
                                                            }
                                                        }
                                                        let _e6396 = local_39;
                                                        let _e6399 = (_e5760 * (1f + (0.45f * _e6396)));
                                                        if _e5764 {
                                                            let _e6502 = local_43;
                                                            let _e6504 = ((_e6502 - -0.25f) * 1.4285715f);
                                                            let _e6506 = select(_e6504, 0f, (_e6504 < 0f));
                                                            let _e6508 = select(_e6506, 1f, (_e6506 > 1f));
                                                            let _e6512 = ((_e6508 * _e6508) * (3f - (2f * _e6508)));
                                                            let _e6514 = ((_e6399 - 1.05f) * -1.1764706f);
                                                            let _e6516 = select(_e6514, 0f, (_e6514 < 0f));
                                                            let _e6518 = select(_e6516, 1f, (_e6516 > 1f));
                                                            let _e6522 = ((_e6518 * _e6518) * (3f - (2f * _e6518)));
                                                            let _e6523 = (_e6512 * _e6522);
                                                            let _e6525 = ((_e6399 - 0.65f) * -1.5384616f);
                                                            let _e6527 = select(_e6525, 0f, (_e6525 < 0f));
                                                            let _e6529 = select(_e6527, 1f, (_e6527 > 1f));
                                                            let _e6534 = (((_e6529 * _e6529) * (3f - (2f * _e6529))) * _e6512);
                                                            let _e6536 = local_44;
                                                            let _e6538 = ((_e6536 - 0.1f) * 2.857143f);
                                                            let _e6540 = select(_e6538, 0f, (_e6538 < 0f));
                                                            let _e6542 = select(_e6540, 1f, (_e6540 > 1f));
                                                            let _e6547 = (((_e6542 * _e6542) * (3f - (2f * _e6542))) * _e6522);
                                                            let _e6552 = ((0.8f * _e6534) * _e6534);
                                                            let _e6561 = (_e5698 * exp((-2f * _e6547)));
                                                            phi_9000_ = type_56(vec3<f32>((((3.1815925f * _e6523) + (0.14634217f * _e6552)) * _e6561), (((0.38179109f * _e6523) + (1.2439084f * _e6552)) * _e6561), (((0.69995034f * _e6523) + (1.0975662f * _e6552)) * _e6561)), (_e5701 * _e6547));
                                                        } else {
                                                            if (_e5680 == 1u) {
                                                                let _e6472 = local_42;
                                                                let _e6474 = ((_e6472 - -0.3f) * 1.25f);
                                                                let _e6476 = select(_e6474, 0f, (_e6474 < 0f));
                                                                let _e6478 = select(_e6476, 1f, (_e6476 > 1f));
                                                                let _e6484 = ((_e6399 - 1f) * -1f);
                                                                let _e6486 = select(_e6484, 0f, (_e6484 < 0f));
                                                                let _e6488 = select(_e6486, 1f, (_e6486 > 1f));
                                                                let _e6494 = (((_e6478 * _e6478) * (3f - (2f * _e6478))) * pow(((_e6488 * _e6488) * (3f - (2f * _e6488))), 1.5f));
                                                                phi_8999_ = type_56((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5698 * _e6494)), (_e5701 * _e6494));
                                                            } else {
                                                                if (_e5680 == 2u) {
                                                                    let _e6428 = local_41;
                                                                    let _e6431 = (_e5760 * (1f + (0.15f * _e6428)));
                                                                    let _e6432 = (_e6431 - 0.65f);
                                                                    let _e6436 = exp(((_e6432 * -5.5555553f) * (_e6432 * 5.5555553f)));
                                                                    let _e6438 = ((_e6431 - 0.75f) * -1.8181818f);
                                                                    let _e6440 = select(_e6438, 0f, (_e6438 < 0f));
                                                                    let _e6442 = select(_e6440, 1f, (_e6440 > 1f));
                                                                    let _e6446 = ((_e6442 * _e6442) * (3f - (2f * _e6442)));
                                                                    let _e6451 = (exp(((_e5760 * -25f) * (_e5760 * 25f))) * 3f);
                                                                    phi_8998_ = type_56(vec3<f32>(((((3.1815925f * _e6436) + (_e6446 * 0.10243952f)) + _e6451) * _e5698), ((((0.38179109f * _e6436) + (_e6446 * 0.8707359f)) + _e6451) * _e5698), ((((0.69995034f * _e6436) + (_e6446 * 0.76829636f)) + _e6451) * _e5698)), 0f);
                                                                } else {
                                                                    let _e6403 = local_40;
                                                                    let _e6405 = ((_e6403 - -0.2f) * 1.6666666f);
                                                                    let _e6407 = select(_e6405, 0f, (_e6405 < 0f));
                                                                    let _e6409 = select(_e6407, 1f, (_e6407 > 1f));
                                                                    let _e6415 = ((_e6399 - 1f) * -1.4285715f);
                                                                    let _e6417 = select(_e6415, 0f, (_e6415 < 0f));
                                                                    let _e6419 = select(_e6417, 1f, (_e6417 > 1f));
                                                                    phi_8998_ = type_56(vec3<f32>(0f, 0f, 0f), (_e5701 * (((_e6409 * _e6409) * (3f - (2f * _e6409))) * ((_e6419 * _e6419) * (3f - (2f * _e6419))))));
                                                                }
                                                                let _e6470 = phi_8998_;
                                                                phi_8999_ = _e6470;
                                                            }
                                                            let _e6500 = phi_8999_;
                                                            phi_9000_ = _e6500;
                                                        }
                                                        let _e6569 = phi_9000_;
                                                        phi_9001_ = _e6569;
                                                    }
                                                    let _e6571 = phi_9001_;
                                                    phi_7680_ = (_e5666 + _e6571.member);
                                                    phi_7683_ = (_e5668 + 1u);
                                                    phi_7685_ = (_e5670 + _e6571.member_1);
                                                } else {
                                                    phi_7680_ = vec3<f32>();
                                                    phi_7683_ = u32();
                                                    phi_7685_ = f32();
                                                }
                                                let _e6578 = phi_7680_;
                                                let _e6580 = phi_7683_;
                                                let _e6582 = phi_7685_;
                                                phi_7679_ = _e6578;
                                                phi_7682_ = _e6580;
                                                phi_7684_ = _e6582;
                                                if !(_e5671) {
                                                    break;
                                                }
                                            }
                                            phi_32823_ = _e4050;
                                            if _e4050 {
                                                break;
                                            }
                                            let _e6585 = local_45;
                                            if (_e6585 <= 0f) {
                                                phi_9030_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6588 = local_46;
                                                let _e6591 = local_47;
                                                let _e6596 = local_48;
                                                phi_9030_ = vec3<f32>(exp((_e6591 * -0.81f)), exp(-(_e6588)), exp((_e6596 * -1.25f)));
                                            }
                                            let _e6601 = phi_9030_;
                                            let _e6603 = local_49;
                                            let _e6608 = local_50;
                                            let _e6613 = local_51;
                                            let _e6618 = local_52;
                                            let _e6622 = local_53;
                                            let _e6626 = local_54;
                                            let _e6631 = global_6.member[(_e1275 + 9u)];
                                            phi_32832_ = _e4050;
                                            phi_9077_ = vec3<f32>((_e3755.x + ((((_e6603.x * _e6601.x) + _e6618.x) * _e6631) * _e3757.x)), (_e3755.y + ((((_e6608.y * _e6601.y) + _e6622.y) * _e6631) * _e3757.y)), (_e3755.z + ((((_e6613.z * _e6601.z) + _e6626.z) * _e6631) * _e3757.z)));
                                        } else {
                                            phi_32832_ = _e3753;
                                            phi_9077_ = _e3755;
                                        }
                                        let _e6649 = phi_32832_;
                                        let _e6651 = phi_9077_;
                                        phi_32831_ = _e6649;
                                        phi_9078_ = _e6651;
                                    } else {
                                        phi_32831_ = _e3753;
                                        phi_9078_ = _e3761;
                                    }
                                    let _e6653 = phi_32831_;
                                    let _e6655 = phi_9078_;
                                    phi_32830_ = _e6653;
                                    phi_9079_ = _e6655;
                                } else {
                                    phi_32830_ = _e306;
                                    phi_9079_ = _e1420;
                                }
                                let _e6657 = phi_32830_;
                                let _e6659 = phi_9079_;
                                phi_32826_ = _e6657;
                                phi_10352_ = (vec3<f32>(_e317.member_2.x, _e317.member_2.y, _e317.member_2.z) * _e6659);
                            } else {
                                let _e333 = global_1.member.member_8[1u];
                                let _e343 = global_6.member[_e333];
                                let _e346 = global_6.member[(_e333 + 1u)];
                                let _e349 = global_6.member[(_e333 + 2u)];
                                let _e352 = global_6.member[(_e333 + 3u)];
                                let _e355 = global_6.member[(_e333 + 4u)];
                                let _e358 = global_6.member[(_e333 + 5u)];
                                let _e361 = global_6.member[(_e333 + 6u)];
                                let _e364 = global_6.member[(_e333 + 7u)];
                                let _e367 = global_6.member[(_e333 + 8u)];
                                let _e370 = global_6.member[(_e333 + 9u)];
                                let _e373 = global_6.member[(_e333 + 10u)];
                                let _e376 = global_6.member[(_e333 + 11u)];
                                let _e379 = global_6.member[(_e333 + 12u)];
                                let _e382 = global_6.member[(_e333 + 13u)];
                                let _e385 = global_6.member[(_e333 + 14u)];
                                let _e388 = global_6.member[(_e333 + 15u)];
                                let _e392 = global_6.member[(_e333 + 16u)];
                                let _e395 = global_6.member[(_e333 + 17u)];
                                let _e398 = global_6.member[(_e333 + 18u)];
                                let _e401 = global_6.member[(_e333 + 19u)];
                                let _e404 = global_6.member[(_e333 + 20u)];
                                let _e407 = global_6.member[(_e333 + 29u)];
                                let _e408 = bitcast<u32>(_e407);
                                let _e411 = global_6.member[(_e333 + 30u)];
                                let _e412 = bitcast<u32>(_e411);
                                let _e420 = ((((_e317.member.x - _e343) * _e317.member_1.x) + ((_e317.member.y - _e346) * _e317.member_1.y)) + ((_e317.member.z - _e349) * _e317.member_1.z));
                                let _e432 = ((((_e317.member.x * _e317.member.x) + (_e317.member.y * _e317.member.y)) + (_e317.member.z * _e317.member.z)) - (2f * (((_e317.member.x * _e343) + (_e317.member.y * _e346)) + (_e317.member.z * _e349))));
                                let _e433 = (_e432 + _e361);
                                let _e434 = (_e420 * _e420);
                                let _e435 = (_e434 - _e433);
                                if (_e435 < 0f) {
                                    phi_20653_ = type_54(0u, type_54().member_1);
                                    phi_20654_ = type_54();
                                    phi_20655_ = true;
                                } else {
                                    if (_e420 < 0f) {
                                        phi_20625_ = -(sqrt(_e435));
                                    } else {
                                        phi_20625_ = sqrt(_e435);
                                    }
                                    let _e442 = phi_20625_;
                                    let _e444 = -((_e420 + _e442));
                                    let _e445 = (_e444 == 0f);
                                    if _e445 {
                                        phi_20648_ = type_54(1u, type_53(0f, 0f));
                                        phi_20649_ = type_54();
                                    } else {
                                        let _e446 = (_e433 / _e444);
                                        if (_e444 < _e446) {
                                            phi_20637_ = type_53(_e444, _e446);
                                        } else {
                                            phi_20637_ = type_53(_e446, _e444);
                                        }
                                        let _e451 = phi_20637_;
                                        phi_20648_ = type_54();
                                        phi_20649_ = type_54(1u, _e451);
                                    }
                                    let _e454 = phi_20648_;
                                    let _e456 = phi_20649_;
                                    phi_20653_ = _e454;
                                    phi_20654_ = _e456;
                                    phi_20655_ = _e445;
                                }
                                let _e461 = phi_20653_;
                                let _e463 = phi_20654_;
                                let _e465 = phi_20655_;
                                if _e465 {
                                    phi_20659_ = _e461;
                                } else {
                                    phi_20659_ = _e463;
                                }
                                let _e467 = phi_20659_;
                                if (bitcast<i32>(_e467.member) != 0i) {
                                    phi_20546_ = type_54(0u, _e467.member_1);
                                } else {
                                    phi_20546_ = type_54(1u, type_54().member_1);
                                }
                                let _e477 = phi_20546_;
                                if (bitcast<i32>(_e477.member) != 0i) {
                                    phi_20593_ = type_54(0u, type_54().member_1);
                                } else {
                                    if (_e477.member_1.member_1 <= 0f) {
                                        phi_20589_ = type_54(0u, type_54().member_1);
                                    } else {
                                        if (_e477.member_1.member != _e477.member_1.member) {
                                            phi_20697_ = true;
                                        } else {
                                            phi_20697_ = (0f >= _e477.member_1.member);
                                        }
                                        let _e489 = phi_20697_;
                                        let _e490 = select(_e477.member_1.member, 0f, _e489);
                                        let _e491 = (_e432 + _e358);
                                        if (_e491 > 0f) {
                                            let _e493 = (_e434 - _e491);
                                            if (_e493 < 0f) {
                                                phi_20805_ = type_54(0u, type_54().member_1);
                                                phi_20806_ = type_54();
                                                phi_20807_ = true;
                                            } else {
                                                if (_e420 < 0f) {
                                                    phi_20777_ = -(sqrt(_e493));
                                                } else {
                                                    phi_20777_ = sqrt(_e493);
                                                }
                                                let _e500 = phi_20777_;
                                                let _e502 = -((_e420 + _e500));
                                                let _e503 = (_e502 == 0f);
                                                if _e503 {
                                                    phi_20800_ = type_54(1u, type_53(0f, 0f));
                                                    phi_20801_ = type_54();
                                                } else {
                                                    let _e504 = (_e491 / _e502);
                                                    if (_e502 < _e504) {
                                                        phi_20789_ = type_53(_e502, _e504);
                                                    } else {
                                                        phi_20789_ = type_53(_e504, _e502);
                                                    }
                                                    let _e509 = phi_20789_;
                                                    phi_20800_ = type_54();
                                                    phi_20801_ = type_54(1u, _e509);
                                                }
                                                let _e512 = phi_20800_;
                                                let _e514 = phi_20801_;
                                                phi_20805_ = _e512;
                                                phi_20806_ = _e514;
                                                phi_20807_ = _e503;
                                            }
                                            let _e519 = phi_20805_;
                                            let _e521 = phi_20806_;
                                            let _e523 = phi_20807_;
                                            if _e523 {
                                                phi_20811_ = _e519;
                                            } else {
                                                phi_20811_ = _e521;
                                            }
                                            let _e525 = phi_20811_;
                                            if (bitcast<i32>(_e525.member) != 0i) {
                                                phi_20745_ = select(_e525.member_1.member_1, _e525.member_1.member, (_e525.member_1.member >= 0f));
                                            } else {
                                                phi_20745_ = -1f;
                                            }
                                            let _e536 = phi_20745_;
                                            if (_e536 >= 0f) {
                                                if (_e477.member_1.member_1 != _e477.member_1.member_1) {
                                                    phi_20849_ = true;
                                                } else {
                                                    phi_20849_ = (_e536 <= _e477.member_1.member_1);
                                                }
                                                let _e541 = phi_20849_;
                                                phi_20574_ = select(_e477.member_1.member_1, _e536, _e541);
                                            } else {
                                                phi_20574_ = _e477.member_1.member_1;
                                            }
                                            let _e544 = phi_20574_;
                                            phi_20576_ = _e544;
                                        } else {
                                            phi_20576_ = _e477.member_1.member_1;
                                        }
                                        let _e546 = phi_20576_;
                                        if (_e546 <= _e490) {
                                            phi_20585_ = type_54(0u, type_54().member_1);
                                        } else {
                                            phi_20585_ = type_54(1u, type_53(_e490, _e546));
                                        }
                                        let _e554 = phi_20585_;
                                        phi_20589_ = _e554;
                                    }
                                    let _e559 = phi_20589_;
                                    phi_20593_ = _e559;
                                }
                                let _e564 = phi_20593_;
                                if (bitcast<i32>(_e564.member) != 0i) {
                                    let _e570 = (_e564.member_1.member < _e317.member.w);
                                    if _e570 {
                                        if (_e564.member_1.member_1 != _e564.member_1.member_1) {
                                            phi_20864_ = true;
                                        } else {
                                            phi_20864_ = (_e317.member.w <= _e564.member_1.member_1);
                                        }
                                        let _e576 = phi_20864_;
                                        let _e577 = select(_e564.member_1.member_1, _e317.member.w, _e576);
                                        phi_9473_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9476_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9478_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9480_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9482_ = 0u;
                                        loop {
                                            let _e579 = phi_9473_;
                                            let _e581 = phi_9476_;
                                            let _e583 = phi_9478_;
                                            let _e585 = phi_9480_;
                                            let _e587 = phi_9482_;
                                            local = _e583;
                                            local_1 = _e581;
                                            local_2 = _e579;
                                            local_3 = _e579;
                                            local_4 = _e579;
                                            local_5 = _e585;
                                            local_6 = _e585;
                                            local_7 = _e585;
                                            let _e588 = (_e587 < 32u);
                                            if _e588 {
                                                let _e589 = -(_e420);
                                                if (_e564.member_1.member <= _e577) {
                                                } else {
                                                    phi_32345_ = true;
                                                    break;
                                                }
                                                let _e592 = select(_e589, _e564.member_1.member, (_e589 < _e564.member_1.member));
                                                let _e594 = select(_e592, _e577, (_e592 > _e577));
                                                let _e595 = (_e594 - _e564.member_1.member);
                                                let _e596 = (_e577 - _e594);
                                                let _e597 = (_e577 - _e564.member_1.member);
                                                let _e598 = (0.000000001f * _e597);
                                                if (_e595 <= _e598) {
                                                    let _e673 = (f32(_e587) * 0.03125f);
                                                    let _e676 = (f32((_e587 + 1u)) * 0.03125f);
                                                    let _e678 = ((_e673 + _e676) * 0.5f);
                                                    phi_9583_ = type_53((_e564.member_1.member + ((_e597 * _e678) * _e678)), (_e597 * ((_e676 * _e676) - (_e673 * _e673))));
                                                } else {
                                                    if (_e596 <= _e598) {
                                                        let _e656 = (f32((31u - _e587)) * 0.03125f);
                                                        let _e659 = (f32((32u - _e587)) * 0.03125f);
                                                        let _e661 = ((_e656 + _e659) * 0.5f);
                                                        phi_9582_ = type_53((_e577 - ((_e597 * _e661) * _e661)), (_e597 * ((_e659 * _e659) - (_e656 * _e656))));
                                                    } else {
                                                        let _e604 = round(((32f * _e595) / (_e595 + _e596)));
                                                        let _e609 = select(select(u32(_e604), 0u, (_e604 < 0f)), 4294967295u, (_e604 > 4294967000f));
                                                        if (_e609 < 1u) {
                                                            phi_9566_ = 1u;
                                                        } else {
                                                            phi_9566_ = select(_e609, 31u, (_e609 > 31u));
                                                        }
                                                        let _e614 = phi_9566_;
                                                        if (_e587 < _e614) {
                                                            let _e635 = ((_e614 - 1u) - _e587);
                                                            let _e637 = f32(_e614);
                                                            let _e638 = (f32(_e635) / _e637);
                                                            let _e641 = (f32((_e635 + 1u)) / _e637);
                                                            let _e643 = ((_e638 + _e641) * 0.5f);
                                                            phi_9581_ = type_53((_e594 - ((_e595 * _e643) * _e643)), (_e595 * ((_e641 * _e641) - (_e638 * _e638))));
                                                        } else {
                                                            let _e617 = (_e587 - _e614);
                                                            let _e619 = f32((32u - _e614));
                                                            let _e620 = (f32(_e617) / _e619);
                                                            let _e623 = (f32((_e617 + 1u)) / _e619);
                                                            let _e625 = ((_e620 + _e623) * 0.5f);
                                                            phi_9581_ = type_53((_e594 + ((_e596 * _e625) * _e625)), (_e596 * ((_e623 * _e623) - (_e620 * _e620))));
                                                        }
                                                        let _e653 = phi_9581_;
                                                        phi_9582_ = _e653;
                                                    }
                                                    let _e671 = phi_9582_;
                                                    phi_9583_ = _e671;
                                                }
                                                let _e688 = phi_9583_;
                                                let _e694 = (_e317.member.x + (_e317.member_1.x * _e688.member));
                                                let _e695 = (_e317.member.y + (_e317.member_1.y * _e688.member));
                                                let _e696 = (_e317.member.z + (_e317.member_1.z * _e688.member));
                                                let _e709 = ((((_e694 * _e694) + (_e695 * _e695)) + (_e696 * _e696)) - (2f * (((_e694 * _e343) + (_e695 * _e346)) + (_e696 * _e349))));
                                                let _e711 = (_e694 - _e343);
                                                let _e712 = (_e695 - _e346);
                                                let _e713 = (_e696 - _e349);
                                                let _e719 = sqrt((((_e711 * _e711) + (_e712 * _e712)) + (_e713 * _e713)));
                                                let _e721 = ((_e709 + _e358) / (_e719 + _e352));
                                                let _e725 = (1f - (abs((_e721 - 25000f)) * 0.00006666667f));
                                                if (_e725 != _e725) {
                                                    phi_21064_ = true;
                                                } else {
                                                    phi_21064_ = (0f >= _e725);
                                                }
                                                let _e729 = phi_21064_;
                                                let _e731 = -(_e721);
                                                let _e734 = (exp((_e731 / _e373)) * _e688.member_1);
                                                let _e737 = (exp((_e731 / _e376)) * _e688.member_1);
                                                let _e739 = (_e734 * 0.5f);
                                                let _e740 = (_e737 * 0.5f);
                                                let _e741 = ((select(_e725, 0f, _e729) * _e688.member_1) * 0.5f);
                                                let _e743 = (_e585.x + _e739);
                                                let _e745 = (_e585.y + _e740);
                                                let _e747 = (_e585.z + _e741);
                                                let _e752 = (_e392 * 1.1f);
                                                let _e753 = (_e752 * _e745);
                                                let _e772 = (_e392 * _e737);
                                                if (_e412 == 4294967295u) {
                                                    phi_9911_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e780 = (1f / _e719);
                                                    let _e788 = ((((_e711 * _e780) * _e364) + ((_e712 * _e780) * _e367)) + ((_e713 * _e780) * _e370));
                                                    if (_e721 != _e721) {
                                                        phi_21191_ = true;
                                                    } else {
                                                        phi_21191_ = (0f >= _e721);
                                                    }
                                                    let _e792 = phi_21191_;
                                                    let _e796 = sqrt((select(_e721, 0f, _e792) / (_e355 - _e352)));
                                                    if (_e796 != _e796) {
                                                        phi_21206_ = true;
                                                    } else {
                                                        phi_21206_ = (1f <= _e796);
                                                    }
                                                    let _e800 = phi_21206_;
                                                    let _e802 = (select(_e796, 1f, _e800) * 31f);
                                                    let _e804 = select(_e788, -1f, (_e788 < -1f));
                                                    let _e808 = ((select(_e804, 1f, (_e804 > 1f)) + 1f) * 15.5f);
                                                    let _e813 = select(select(u32(_e802), 0u, (_e802 < 0f)), 4294967295u, (_e802 > 4294967000f));
                                                    let _e815 = select(_e813, 30u, (30u < _e813));
                                                    let _e820 = select(select(u32(_e808), 0u, (_e808 < 0f)), 4294967295u, (_e808 > 4294967000f));
                                                    let _e822 = select(_e820, 30u, (30u < _e820));
                                                    let _e824 = (_e802 - f32(_e815));
                                                    let _e826 = (_e808 - f32(_e822));
                                                    let _e830 = (_e412 + (3u * ((_e815 * 32u) + _e822)));
                                                    let _e835 = (_e412 + (3u * (((_e815 + 1u) * 32u) + _e822)));
                                                    let _e837 = global_6.member[_e830];
                                                    let _e840 = global_6.member[(_e830 + 1u)];
                                                    let _e843 = global_6.member[(_e830 + 2u)];
                                                    let _e844 = (1f - _e826);
                                                    let _e850 = global_6.member[(_e830 + 3u)];
                                                    let _e853 = global_6.member[(_e830 + 4u)];
                                                    let _e856 = global_6.member[(_e830 + 5u)];
                                                    let _e863 = (1f - _e824);
                                                    let _e868 = global_6.member[_e835];
                                                    let _e871 = global_6.member[(_e835 + 1u)];
                                                    let _e874 = global_6.member[(_e835 + 2u)];
                                                    let _e880 = global_6.member[(_e835 + 3u)];
                                                    let _e883 = global_6.member[(_e835 + 4u)];
                                                    let _e886 = global_6.member[(_e835 + 5u)];
                                                    phi_9911_ = vec3<f32>(((((_e837 * _e844) + (_e850 * _e826)) * _e863) + (((_e868 * _e844) + (_e880 * _e826)) * _e824)), ((((_e840 * _e844) + (_e853 * _e826)) * _e863) + (((_e871 * _e844) + (_e883 * _e826)) * _e824)), ((((_e843 * _e844) + (_e856 * _e826)) * _e863) + (((_e874 * _e844) + (_e886 * _e826)) * _e824)));
                                                }
                                                let _e901 = phi_9911_;
                                                let _e915 = (_e709 + _e361);
                                                if (_e915 > 0f) {
                                                    let _e921 = (((_e711 * _e364) + (_e712 * _e367)) + (_e713 * _e370));
                                                    let _e923 = ((_e921 * _e921) - _e915);
                                                    if (_e921 >= 0f) {
                                                        phi_9985_ = vec3<f32>();
                                                        phi_9986_ = true;
                                                    } else {
                                                        let _e925 = (_e923 <= 0f);
                                                        if _e925 {
                                                            phi_9984_ = vec3<f32>();
                                                        } else {
                                                            let _e928 = (-(_e921) - sqrt(_e923));
                                                            phi_9984_ = vec3<f32>((_e694 + (_e364 * _e928)), (_e695 + (_e367 * _e928)), (_e696 + (_e370 * _e928)));
                                                        }
                                                        let _e937 = phi_9984_;
                                                        phi_9985_ = _e937;
                                                        phi_9986_ = _e925;
                                                    }
                                                    let _e939 = phi_9985_;
                                                    let _e941 = phi_9986_;
                                                    phi_9991_ = _e939;
                                                    phi_9992_ = select(true, false, _e941);
                                                } else {
                                                    phi_9991_ = vec3<f32>(_e694, _e695, _e696);
                                                    phi_9992_ = true;
                                                }
                                                let _e944 = phi_9991_;
                                                let _e946 = phi_9992_;
                                                if _e946 {
                                                    let _e948 = (_e944.x - _e343);
                                                    let _e950 = (_e944.y - _e346);
                                                    let _e952 = (_e944.z - _e349);
                                                    let _e958 = sqrt((((_e948 * _e948) + (_e950 * _e950)) + (_e952 * _e952)));
                                                    let _e959 = (1f / _e958);
                                                    let _e977 = ((((((_e944.x * _e944.x) + (_e944.y * _e944.y)) + (_e944.z * _e944.z)) - (2f * (((_e944.x * _e343) + (_e944.y * _e346)) + (_e944.z * _e349)))) + _e358) / (_e958 + _e352));
                                                    if (_e977 != _e977) {
                                                        phi_21330_ = true;
                                                    } else {
                                                        phi_21330_ = (0f >= _e977);
                                                    }
                                                    let _e981 = phi_21330_;
                                                    let _e985 = sqrt((select(_e977, 0f, _e981) / (_e355 - _e352)));
                                                    if (_e985 != _e985) {
                                                        phi_21345_ = true;
                                                    } else {
                                                        phi_21345_ = (1f <= _e985);
                                                    }
                                                    let _e989 = phi_21345_;
                                                    let _e991 = (select(_e985, 1f, _e989) * 63f);
                                                    let _e996 = ((((_e948 * _e959) * _e364) + ((_e950 * _e959) * _e367)) + ((_e952 * _e959) * _e370));
                                                    let _e998 = select(_e996, -1f, (_e996 < -1f));
                                                    let _e1002 = ((select(_e998, 1f, (_e998 > 1f)) + 1f) * 127.5f);
                                                    let _e1007 = select(select(u32(_e991), 0u, (_e991 < 0f)), 4294967295u, (_e991 > 4294967000f));
                                                    let _e1009 = select(_e1007, 62u, (62u < _e1007));
                                                    let _e1014 = select(select(u32(_e1002), 0u, (_e1002 < 0f)), 4294967295u, (_e1002 > 4294967000f));
                                                    let _e1016 = select(_e1014, 254u, (254u < _e1014));
                                                    let _e1018 = (_e991 - f32(_e1009));
                                                    let _e1020 = (_e1002 - f32(_e1016));
                                                    let _e1024 = (_e408 + (3u * ((_e1009 * 256u) + _e1016)));
                                                    let _e1029 = (_e408 + (3u * (((_e1009 + 1u) * 256u) + _e1016)));
                                                    let _e1031 = global_6.member[_e1024];
                                                    let _e1034 = global_6.member[(_e1024 + 1u)];
                                                    let _e1037 = global_6.member[(_e1024 + 2u)];
                                                    let _e1038 = (1f - _e1020);
                                                    let _e1044 = global_6.member[(_e1024 + 3u)];
                                                    let _e1047 = global_6.member[(_e1024 + 4u)];
                                                    let _e1050 = global_6.member[(_e1024 + 5u)];
                                                    let _e1057 = (1f - _e1018);
                                                    let _e1062 = global_6.member[_e1029];
                                                    let _e1065 = global_6.member[(_e1029 + 1u)];
                                                    let _e1068 = global_6.member[(_e1029 + 2u)];
                                                    let _e1074 = global_6.member[(_e1029 + 3u)];
                                                    let _e1077 = global_6.member[(_e1029 + 4u)];
                                                    let _e1080 = global_6.member[(_e1029 + 5u)];
                                                    phi_10283_ = vec3<f32>(((((_e1031 * _e1038) + (_e1044 * _e1020)) * _e1057) + (((_e1062 * _e1038) + (_e1074 * _e1020)) * _e1018)), ((((_e1034 * _e1038) + (_e1047 * _e1020)) * _e1057) + (((_e1065 * _e1038) + (_e1077 * _e1020)) * _e1018)), ((((_e1037 * _e1038) + (_e1050 * _e1020)) * _e1057) + (((_e1068 * _e1038) + (_e1080 * _e1020)) * _e1018)));
                                                } else {
                                                    phi_10283_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1095 = phi_10283_;
                                                let _e1096 = (vec3<f32>(_e743, _e745, _e747) + _e1095);
                                                let _e1102 = (_e752 * _e1096.y);
                                                let _e1114 = exp(-((((_e382 * _e1096.x) + _e1102) + (_e398 * _e1096.z))));
                                                let _e1116 = exp(-((((_e385 * _e1096.x) + _e1102) + (_e401 * _e1096.z))));
                                                let _e1118 = exp(-((((_e388 * _e1096.x) + _e1102) + (_e404 * _e1096.z))));
                                                phi_9474_ = vec3<f32>((_e579.x + ((exp(-((((_e382 * _e743) + _e753) + (_e398 * _e747)))) * ((_e382 * _e734) + _e772)) * _e901.x)), (_e579.y + ((exp(-((((_e385 * _e743) + _e753) + (_e401 * _e747)))) * ((_e385 * _e734) + _e772)) * _e901.y)), (_e579.z + ((exp(-((((_e388 * _e743) + _e753) + (_e404 * _e747)))) * ((_e388 * _e734) + _e772)) * _e901.z)));
                                                phi_9477_ = vec3<f32>((_e581.x + (_e1114 * _e737)), (_e581.y + (_e1116 * _e737)), (_e581.z + (_e1118 * _e737)));
                                                phi_9479_ = vec3<f32>((_e583.x + (_e1114 * _e734)), (_e583.y + (_e1116 * _e734)), (_e583.z + (_e1118 * _e734)));
                                                phi_9481_ = vec3<f32>((_e743 + _e739), (_e745 + _e740), (_e747 + _e741));
                                                phi_9483_ = (_e587 + 1u);
                                            } else {
                                                phi_9474_ = vec3<f32>();
                                                phi_9477_ = vec3<f32>();
                                                phi_9479_ = vec3<f32>();
                                                phi_9481_ = vec3<f32>();
                                                phi_9483_ = u32();
                                            }
                                            let _e1145 = phi_9474_;
                                            let _e1147 = phi_9477_;
                                            let _e1149 = phi_9479_;
                                            let _e1151 = phi_9481_;
                                            let _e1153 = phi_9483_;
                                            phi_9473_ = _e1145;
                                            phi_9476_ = _e1147;
                                            phi_9478_ = _e1149;
                                            phi_9480_ = _e1151;
                                            phi_9482_ = _e1153;
                                            phi_32345_ = _e306;
                                            if !(_e588) {
                                                break;
                                            }
                                        }
                                        let _e1156 = phi_32345_;
                                        phi_32823_ = _e1156;
                                        if _e1156 {
                                            break;
                                        }
                                        let _e1158 = local;
                                        let _e1159 = (_e1158 * vec3<f32>(_e382, _e385, _e388));
                                        let _e1161 = local_1;
                                        let _e1162 = (_e1161 * _e392);
                                        let _e1167 = (((_e317.member_1.x * _e364) + (_e317.member_1.y * _e367)) + (_e317.member_1.z * _e370));
                                        let _e1169 = (1f + (_e1167 * _e1167));
                                        let _e1170 = (0.059683103f * _e1169);
                                        let _e1177 = (_e395 * _e395);
                                        let _e1188 = ((0.119366206f * ((1f - _e1177) * _e1169)) / ((2f + _e1177) * pow(((1f + _e1177) - ((2f * _e395) * _e1167)), 1.5f)));
                                        let _e1199 = local_2;
                                        let _e1203 = local_3;
                                        let _e1207 = local_4;
                                        let _e1216 = local_5;
                                        let _e1223 = local_6;
                                        let _e1225 = ((_e392 * 1.1f) * _e1223.y);
                                        let _e1230 = local_7;
                                        phi_32829_ = _e1156;
                                        phi_10340_ = type_57(vec3<f32>(exp(-((((_e382 * _e1216.x) + _e1225) + (_e398 * _e1230.z)))), exp(-((((_e385 * _e1216.x) + _e1225) + (_e401 * _e1230.z)))), exp(-((((_e388 * _e1216.x) + _e1225) + (_e404 * _e1230.z))))), (vec3<f32>(((((_e1159.x * _e1170) + (_e1162.x * _e1188)) + _e1199.x) * _e379), ((((_e1159.y * _e1170) + (_e1162.y * _e1188)) + _e1203.y) * _e379), ((((_e1159.z * _e1170) + (_e1162.z * _e1188)) + _e1207.z) * _e379)) * 0.38317f));
                                    } else {
                                        phi_32829_ = _e306;
                                        phi_10340_ = type_57();
                                    }
                                    let _e1247 = phi_32829_;
                                    let _e1249 = phi_10340_;
                                    phi_32828_ = _e1247;
                                    phi_10342_ = _e1249;
                                    phi_10343_ = select(true, false, _e570);
                                } else {
                                    phi_32828_ = _e306;
                                    phi_10342_ = type_57();
                                    phi_10343_ = true;
                                }
                                let _e1252 = phi_32828_;
                                let _e1254 = phi_10342_;
                                let _e1256 = phi_10343_;
                                if _e1256 {
                                    phi_10348_ = type_57(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10348_ = _e1254;
                                }
                                let _e1258 = phi_10348_;
                                phi_32826_ = _e1252;
                                phi_10352_ = (vec3<f32>(_e317.member_2.x, _e317.member_2.y, _e317.member_2.z) * _e1258.member_1);
                            }
                            let _e6663 = phi_32826_;
                            let _e6665 = phi_10352_;
                            phi_32825_ = _e6663;
                            phi_10353_ = _e6665;
                        }
                        let _e6667 = phi_32825_;
                        let _e6669 = phi_10353_;
                        phi_32824_ = _e6667;
                        phi_471_ = (_e308 + _e6669);
                        phi_474_ = (_e310 + 1u);
                    } else {
                        phi_32824_ = _e306;
                        phi_471_ = vec3<f32>();
                        phi_474_ = u32();
                    }
                    let _e6673 = phi_32824_;
                    let _e6675 = phi_471_;
                    let _e6677 = phi_474_;
                    phi_32362_ = _e6673;
                    phi_470_ = _e6675;
                    phi_473_ = _e6677;
                    phi_32823_ = _e6673;
                    if !(_e311) {
                        break;
                    }
                }
                let _e6680 = phi_32823_;
                if _e6680 {
                    break;
                }
                let _e6682 = local_55;
                let _e6686 = local_56;
                let _e6691 = local_57;
                if ((((_e6682.x * _e6682.x) + (_e6686.y * _e6686.y)) + (_e6691.z * _e6691.z)) > 0f) {
                    if (_e300.member_1 < arrayLength((&global_7.member))) {
                    } else {
                        break;
                    }
                    let _e6701 = global_7.member[_e300.member_1].member_3[0u];
                    global_7.member[_e300.member_1].member_3[0u] = (_e6701 + _e6682.x);
                    let _e6707 = global_7.member[_e300.member_1].member_3[1u];
                    global_7.member[_e300.member_1].member_3[1u] = (_e6707 + _e6686.y);
                    let _e6713 = global_7.member[_e300.member_1].member_3[2u];
                    global_7.member[_e300.member_1].member_3[2u] = (_e6713 + _e6691.z);
                    let _e6719 = global_7.member[_e300.member_1].member_3[3u];
                    global_7.member[_e300.member_1].member_3[3u] = _e6719;
                }
            }
            break;
        }
    }
    return;
}

fn function_1() {
    var local_67: array<u32, 64>;
    var local_68: array<f32, 64>;
    var local_69: array<u32, 64>;
    var local_70: array<f32, 64>;
    var phi_11127_: type_45;
    var phi_11316_: u32;
    var phi_11319_: u32;
    var phi_11321_: u32;
    var phi_11323_: u32;
    var phi_11325_: vec4<f32>;
    var phi_11327_: type_66;
    var phi_11683_: u32;
    var phi_11685_: u32;
    var phi_11686_: u32;
    var phi_11688_: bool;
    var phi_11689_: bool;
    var phi_11690_: u32;
    var phi_11691_: u32;
    var phi_11692_: u32;
    var phi_11693_: bool;
    var phi_11694_: bool;
    var phi_11695_: bool;
    var phi_24926_: bool;
    var phi_24956_: bool;
    var phi_24903_: type_53;
    var phi_24906_: type_53;
    var phi_24907_: bool;
    var phi_24912_: type_53;
    var phi_25013_: bool;
    var phi_25028_: bool;
    var phi_25043_: bool;
    var phi_25058_: bool;
    var phi_24990_: type_53;
    var phi_24993_: type_53;
    var phi_24994_: bool;
    var phi_24999_: type_53;
    var phi_25100_: bool;
    var phi_25115_: bool;
    var phi_25130_: bool;
    var phi_25145_: bool;
    var phi_25077_: type_53;
    var phi_25080_: type_53;
    var phi_25081_: bool;
    var phi_25086_: type_53;
    var phi_24881_: bool;
    var phi_24884_: bool;
    var phi_11725_: f32;
    var phi_25247_: bool;
    var phi_25277_: bool;
    var phi_25224_: type_53;
    var phi_25227_: type_53;
    var phi_25228_: bool;
    var phi_25233_: type_53;
    var phi_25334_: bool;
    var phi_25349_: bool;
    var phi_25364_: bool;
    var phi_25379_: bool;
    var phi_25311_: type_53;
    var phi_25314_: type_53;
    var phi_25315_: bool;
    var phi_25320_: type_53;
    var phi_25421_: bool;
    var phi_25436_: bool;
    var phi_25451_: bool;
    var phi_25466_: bool;
    var phi_25398_: type_53;
    var phi_25401_: type_53;
    var phi_25402_: bool;
    var phi_25407_: type_53;
    var phi_25202_: bool;
    var phi_25205_: bool;
    var phi_11735_: f32;
    var phi_11746_: u32;
    var phi_11757_: u32;
    var phi_11796_: u32;
    var phi_11797_: u32;
    var phi_11808_: u32;
    var phi_11809_: u32;
    var phi_11810_: u32;
    var phi_11811_: u32;
    var phi_11813_: u32;
    var phi_11814_: u32;
    var phi_11816_: u32;
    var phi_11817_: u32;
    var phi_11818_: bool;
    var phi_11819_: bool;
    var phi_11606_: vec4<f32>;
    var phi_11607_: vec4<f32>;
    var phi_11608_: bool;
    var phi_11612_: vec4<f32>;
    var phi_11613_: bool;
    var phi_11523_: bool;
    var phi_11557_: bool;
    var phi_11563_: f32;
    var phi_11573_: vec4<f32>;
    var phi_11574_: vec4<f32>;
    var phi_11575_: bool;
    var phi_11579_: vec4<f32>;
    var phi_11580_: bool;
    var phi_11584_: vec4<f32>;
    var phi_11585_: bool;
    var phi_11614_: vec4<f32>;
    var phi_11615_: bool;
    var phi_11455_: vec4<f32>;
    var phi_11456_: vec4<f32>;
    var phi_11457_: bool;
    var phi_11461_: vec4<f32>;
    var phi_11462_: bool;
    var phi_11463_: vec4<f32>;
    var phi_11464_: bool;
    var phi_11465_: bool;
    var phi_11473_: vec4<f32>;
    var phi_11474_: bool;
    var phi_11475_: vec4<f32>;
    var phi_11476_: bool;
    var phi_11477_: bool;
    var phi_11485_: vec4<f32>;
    var phi_11486_: bool;
    var phi_11624_: vec4<f32>;
    var phi_11625_: bool;
    var phi_11646_: type_66;
    var phi_11648_: type_66;
    var phi_11317_: u32;
    var phi_11320_: u32;
    var phi_11322_: u32;
    var phi_11324_: u32;
    var phi_11326_: vec4<f32>;
    var phi_11328_: type_66;
    var phi_11830_: type_66;
    var phi_11831_: bool;
    var phi_11832_: bool;
    var phi_32834_: bool;
    var phi_20523_: bool;
    var phi_20522_: type_66;
    var local_71: type_66;
    var local_72: vec4<f32>;
    var phi_11993_: type_57;
    var local_73: vec4<f32>;
    var local_74: vec4<f32>;
    var phi_11959_: vec3<f32>;
    var phi_11960_: vec3<f32>;
    var phi_11994_: type_57;
    var local_75: type_66;
    var phi_11999_: type_66;
    var phi_12000_: type_66;
    var phi_12005_: type_66;
    var phi_25585_: f32;
    var phi_25597_: type_53;
    var phi_25608_: type_54;
    var phi_25609_: type_54;
    var phi_25613_: type_54;
    var phi_25614_: type_54;
    var phi_25615_: bool;
    var phi_25619_: type_54;
    var phi_25506_: type_54;
    var phi_25657_: bool;
    var phi_25737_: f32;
    var phi_25749_: type_53;
    var phi_25760_: type_54;
    var phi_25761_: type_54;
    var phi_25765_: type_54;
    var phi_25766_: type_54;
    var phi_25767_: bool;
    var phi_25771_: type_54;
    var phi_25705_: f32;
    var phi_25809_: bool;
    var phi_25534_: f32;
    var phi_25536_: f32;
    var phi_25545_: type_54;
    var phi_25549_: type_54;
    var phi_25553_: type_54;
    var phi_25824_: bool;
    var phi_13424_: vec3<f32>;
    var phi_13427_: u32;
    var phi_13511_: u32;
    var phi_13526_: type_53;
    var phi_13527_: type_53;
    var phi_13528_: type_53;
    var phi_26023_: bool;
    var phi_13425_: vec3<f32>;
    var phi_13428_: u32;
    var phi_32956_: bool;
    var local_76: vec3<f32>;
    var local_77: vec3<f32>;
    var local_78: vec3<f32>;
    var phi_35224_: bool;
    var phi_13553_: vec3<f32>;
    var phi_35223_: bool;
    var phi_13555_: vec3<f32>;
    var phi_13556_: bool;
    var phi_35221_: bool;
    var phi_13563_: vec3<f32>;
    var phi_13564_: type_39;
    var phi_35220_: bool;
    var phi_13565_: vec3<f32>;
    var phi_13566_: type_39;
    var phi_13590_: i32;
    var phi_13593_: u32;
    var phi_13647_: bool;
    var phi_13653_: bool;
    var phi_13660_: u32;
    var phi_13662_: i32;
    var phi_13663_: u32;
    var phi_13591_: i32;
    var phi_13594_: u32;
    var phi_13665_: bool;
    var local_79: u32;
    var phi_13946_: vec3<f32>;
    var phi_13963_: vec3<f32>;
    var phi_26090_: bool;
    var phi_13996_: f32;
    var phi_14105_: vec3<f32>;
    var phi_14111_: f32;
    var phi_26163_: bool;
    var phi_26152_: f32;
    var phi_14192_: vec3<f32>;
    var phi_14193_: vec3<f32>;
    var phi_14194_: vec3<f32>;
    var phi_33221_: u32;
    var phi_14196_: vec3<f32>;
    var phi_14197_: vec3<f32>;
    var phi_14198_: vec3<f32>;
    var phi_14199_: bool;
    var phi_14214_: vec3<f32>;
    var phi_14215_: vec3<f32>;
    var phi_33219_: u32;
    var phi_14217_: vec3<f32>;
    var phi_14218_: vec3<f32>;
    var phi_14219_: vec3<f32>;
    var phi_33218_: u32;
    var phi_14220_: vec3<f32>;
    var phi_14221_: vec3<f32>;
    var phi_14222_: vec3<f32>;
    var phi_14223_: bool;
    var phi_33217_: u32;
    var phi_14224_: vec3<f32>;
    var phi_14225_: vec3<f32>;
    var phi_14226_: vec3<f32>;
    var phi_14227_: bool;
    var phi_14228_: bool;
    var phi_26236_: bool;
    var phi_26398_: bool;
    var phi_26413_: bool;
    var phi_26428_: bool;
    var phi_26455_: bool;
    var phi_26470_: bool;
    var phi_26485_: bool;
    var phi_26512_: bool;
    var phi_26527_: bool;
    var phi_26542_: bool;
    var phi_14407_: f32;
    var phi_14415_: f32;
    var phi_14423_: f32;
    var phi_26739_: bool;
    var phi_26728_: f32;
    var phi_14530_: vec3<f32>;
    var phi_26767_: bool;
    var phi_14531_: vec3<f32>;
    var phi_14532_: vec3<f32>;
    var phi_14533_: vec3<f32>;
    var phi_33605_: vec3<f32>;
    var phi_33528_: vec3<f32>;
    var phi_33449_: u32;
    var phi_33367_: f32;
    var phi_14535_: vec3<f32>;
    var phi_14536_: vec3<f32>;
    var phi_14537_: bool;
    var phi_26782_: bool;
    var phi_26833_: bool;
    var phi_26822_: f32;
    var phi_33576_: vec3<f32>;
    var phi_14069_: vec3<f32>;
    var phi_33928_: vec3<f32>;
    var phi_33722_: f32;
    var phi_33575_: vec3<f32>;
    var phi_33263_: u32;
    var phi_14071_: vec3<f32>;
    var phi_14072_: bool;
    var phi_33874_: vec3<f32>;
    var phi_33682_: f32;
    var phi_33603_: vec3<f32>;
    var phi_33526_: vec3<f32>;
    var phi_33447_: u32;
    var phi_33365_: f32;
    var phi_33288_: u32;
    var phi_33214_: u32;
    var phi_14545_: vec3<f32>;
    var phi_14546_: vec3<f32>;
    var phi_14547_: vec3<f32>;
    var phi_14548_: bool;
    var phi_26899_: bool;
    var phi_14617_: vec3<f32>;
    var phi_33271_: u32;
    var phi_14619_: vec3<f32>;
    var phi_14620_: bool;
    var phi_33270_: u32;
    var phi_14621_: vec3<f32>;
    var phi_14622_: bool;
    var phi_33268_: u32;
    var phi_14628_: vec3<f32>;
    var phi_14629_: bool;
    var phi_14662_: type_71;
    var phi_33273_: u32;
    var phi_14664_: vec3<f32>;
    var phi_14665_: type_71;
    var phi_14666_: bool;
    var phi_14672_: type_71;
    var phi_33277_: u32;
    var phi_14673_: type_71;
    var phi_33871_: vec3<f32>;
    var phi_33679_: f32;
    var phi_33600_: vec3<f32>;
    var phi_33523_: vec3<f32>;
    var phi_33444_: u32;
    var phi_33362_: f32;
    var phi_33285_: u32;
    var phi_33276_: u32;
    var phi_14675_: type_71;
    var phi_14676_: bool;
    var phi_35200_: bool;
    var phi_33275_: u32;
    var phi_14687_: vec3<f32>;
    var phi_14690_: type_39;
    var phi_14692_: u32;
    var phi_14694_: u32;
    var phi_17383_: u32;
    var phi_17386_: u32;
    var phi_17384_: u32;
    var phi_17387_: u32;
    var local_80: u32;
    var phi_17424_: u32;
    var phi_17427_: u32;
    var phi_17425_: u32;
    var phi_17428_: u32;
    var local_81: u32;
    var local_82: u32;
    var phi_17504_: f32;
    var local_83: u32;
    var local_84: u32;
    var phi_17553_: f32;
    var phi_17709_: f32;
    var phi_17716_: vec3<f32>;
    var phi_35621_: u32;
    var phi_17718_: f32;
    var phi_17719_: vec3<f32>;
    var phi_17720_: vec3<f32>;
    var phi_16856_: u32;
    var phi_16859_: u32;
    var phi_16857_: u32;
    var phi_16860_: u32;
    var local_85: u32;
    var local_86: u32;
    var phi_17252_: f32;
    var phi_17255_: f32;
    var phi_27172_: bool;
    var phi_27153_: f32;
    var phi_17256_: f32;
    var phi_27187_: bool;
    var phi_27210_: bool;
    var phi_27225_: bool;
    var phi_17007_: f32;
    var phi_27240_: bool;
    var phi_27255_: bool;
    var phi_27270_: bool;
    var phi_27335_: bool;
    var phi_27423_: bool;
    var phi_27404_: f32;
    var phi_17072_: type_74;
    var phi_17258_: type_74;
    var phi_17336_: bool;
    var phi_17345_: vec3<f32>;
    var phi_17347_: vec3<f32>;
    var phi_17348_: bool;
    var phi_17353_: f32;
    var phi_17354_: vec3<f32>;
    var phi_35629_: u32;
    var phi_17355_: f32;
    var phi_17356_: vec3<f32>;
    var phi_17357_: vec3<f32>;
    var phi_17358_: f32;
    var phi_35628_: u32;
    var phi_17359_: f32;
    var phi_17360_: vec3<f32>;
    var phi_17361_: vec3<f32>;
    var phi_17362_: f32;
    var phi_35620_: u32;
    var phi_17721_: f32;
    var phi_17722_: vec3<f32>;
    var phi_17723_: vec3<f32>;
    var phi_17724_: f32;
    var phi_27438_: bool;
    var phi_27548_: f32;
    var phi_27576_: f32;
    var phi_27604_: f32;
    var phi_27738_: bool;
    var phi_27753_: bool;
    var phi_27768_: bool;
    var phi_27795_: bool;
    var phi_27810_: bool;
    var phi_27825_: bool;
    var phi_27852_: bool;
    var phi_27867_: bool;
    var phi_27882_: bool;
    var phi_15028_: f32;
    var phi_15036_: f32;
    var phi_15044_: f32;
    var phi_28079_: bool;
    var phi_28068_: f32;
    var phi_15151_: vec3<f32>;
    var phi_28107_: bool;
    var phi_15152_: vec3<f32>;
    var phi_15164_: type_56;
    var phi_15165_: type_56;
    var phi_15166_: bool;
    var phi_15171_: type_56;
    var phi_28144_: bool;
    var phi_28174_: bool;
    var phi_28163_: f32;
    var phi_28213_: vec3<f32>;
    var phi_15199_: type_56;
    var phi_15200_: type_56;
    var phi_28235_: vec3<f32>;
    var phi_15201_: type_56;
    var phi_15202_: type_56;
    var phi_15203_: bool;
    var phi_15207_: type_56;
    var phi_28267_: bool;
    var phi_28338_: bool;
    var phi_16027_: f32;
    var phi_28353_: bool;
    var phi_28368_: bool;
    var phi_16500_: vec3<f32>;
    var phi_16501_: vec3<f32>;
    var phi_16502_: bool;
    var phi_16507_: vec3<f32>;
    var phi_16508_: bool;
    var phi_28492_: bool;
    var phi_28507_: bool;
    var phi_16799_: vec3<f32>;
    var phi_16801_: vec3<f32>;
    var phi_16828_: vec3<f32>;
    var phi_16829_: vec3<f32>;
    var phi_15645_: vec3<f32>;
    var phi_15646_: vec3<f32>;
    var phi_15647_: bool;
    var phi_15652_: vec3<f32>;
    var phi_15653_: bool;
    var phi_28669_: bool;
    var phi_28684_: bool;
    var phi_15944_: vec3<f32>;
    var phi_15946_: vec3<f32>;
    var phi_35663_: u32;
    var phi_16830_: f32;
    var phi_16831_: vec3<f32>;
    var phi_16832_: vec3<f32>;
    var phi_35662_: u32;
    var phi_16833_: f32;
    var phi_16834_: vec3<f32>;
    var phi_16835_: vec3<f32>;
    var phi_35619_: u32;
    var phi_17725_: f32;
    var phi_17726_: vec3<f32>;
    var phi_17727_: vec3<f32>;
    var phi_17728_: f32;
    var phi_14790_: f32;
    var phi_14791_: f32;
    var phi_35618_: u32;
    var phi_17729_: f32;
    var phi_17730_: vec3<f32>;
    var phi_17731_: vec3<f32>;
    var phi_17732_: f32;
    var phi_28737_: bool;
    var phi_28847_: f32;
    var phi_28875_: f32;
    var phi_28903_: f32;
    var phi_29037_: bool;
    var phi_29052_: bool;
    var phi_29067_: bool;
    var phi_29094_: bool;
    var phi_29109_: bool;
    var phi_29124_: bool;
    var phi_29151_: bool;
    var phi_29166_: bool;
    var phi_29181_: bool;
    var phi_17955_: f32;
    var phi_17963_: f32;
    var phi_17971_: f32;
    var phi_29378_: bool;
    var phi_29367_: f32;
    var phi_18078_: vec3<f32>;
    var phi_29406_: bool;
    var phi_18079_: vec3<f32>;
    var phi_18091_: type_56;
    var phi_18092_: type_56;
    var phi_18093_: bool;
    var phi_18098_: type_56;
    var phi_29443_: bool;
    var phi_29473_: bool;
    var phi_29462_: f32;
    var phi_29512_: vec3<f32>;
    var phi_18126_: type_56;
    var phi_18127_: type_56;
    var phi_29534_: vec3<f32>;
    var phi_18128_: type_56;
    var phi_18129_: type_56;
    var phi_18130_: bool;
    var phi_18134_: type_56;
    var phi_29548_: f32;
    var phi_29551_: f32;
    var phi_18143_: f32;
    var phi_18177_: u32;
    var phi_18180_: u32;
    var phi_18182_: u32;
    var phi_18184_: u32;
    var phi_18188_: type_66;
    var phi_18561_: u32;
    var phi_18563_: u32;
    var phi_18564_: u32;
    var phi_18566_: bool;
    var phi_18567_: bool;
    var phi_18568_: u32;
    var phi_18569_: u32;
    var phi_18570_: u32;
    var phi_18571_: bool;
    var phi_18572_: bool;
    var phi_18573_: bool;
    var phi_29649_: bool;
    var phi_29679_: bool;
    var phi_29626_: type_53;
    var phi_29629_: type_53;
    var phi_29630_: bool;
    var phi_29635_: type_53;
    var phi_29736_: bool;
    var phi_29751_: bool;
    var phi_29766_: bool;
    var phi_29781_: bool;
    var phi_29713_: type_53;
    var phi_29716_: type_53;
    var phi_29717_: bool;
    var phi_29722_: type_53;
    var phi_29823_: bool;
    var phi_29838_: bool;
    var phi_29853_: bool;
    var phi_29868_: bool;
    var phi_29800_: type_53;
    var phi_29803_: type_53;
    var phi_29804_: bool;
    var phi_29809_: type_53;
    var phi_29604_: bool;
    var phi_29607_: bool;
    var phi_18603_: f32;
    var phi_29970_: bool;
    var phi_30000_: bool;
    var phi_29947_: type_53;
    var phi_29950_: type_53;
    var phi_29951_: bool;
    var phi_29956_: type_53;
    var phi_30057_: bool;
    var phi_30072_: bool;
    var phi_30087_: bool;
    var phi_30102_: bool;
    var phi_30034_: type_53;
    var phi_30037_: type_53;
    var phi_30038_: bool;
    var phi_30043_: type_53;
    var phi_30144_: bool;
    var phi_30159_: bool;
    var phi_30174_: bool;
    var phi_30189_: bool;
    var phi_30121_: type_53;
    var phi_30124_: type_53;
    var phi_30125_: bool;
    var phi_30130_: type_53;
    var phi_29925_: bool;
    var phi_29928_: bool;
    var phi_18613_: f32;
    var phi_18624_: u32;
    var phi_18635_: u32;
    var phi_18674_: u32;
    var phi_18675_: u32;
    var phi_18686_: u32;
    var phi_18687_: u32;
    var phi_18688_: u32;
    var phi_18689_: u32;
    var phi_18691_: u32;
    var phi_18692_: u32;
    var phi_18694_: u32;
    var phi_18695_: u32;
    var phi_18696_: bool;
    var phi_18697_: bool;
    var phi_18484_: vec4<f32>;
    var phi_18485_: vec4<f32>;
    var phi_18486_: bool;
    var phi_18490_: vec4<f32>;
    var phi_18491_: bool;
    var phi_18396_: bool;
    var phi_18433_: bool;
    var phi_18439_: f32;
    var phi_18449_: vec4<f32>;
    var phi_18450_: vec4<f32>;
    var phi_18451_: bool;
    var phi_18455_: vec4<f32>;
    var phi_18456_: bool;
    var phi_18460_: vec4<f32>;
    var phi_18461_: bool;
    var phi_18492_: vec4<f32>;
    var phi_18493_: bool;
    var phi_18325_: vec4<f32>;
    var phi_18326_: vec4<f32>;
    var phi_18327_: bool;
    var phi_18331_: vec4<f32>;
    var phi_18332_: bool;
    var phi_18333_: vec4<f32>;
    var phi_18334_: bool;
    var phi_18335_: bool;
    var phi_18343_: vec4<f32>;
    var phi_18344_: bool;
    var phi_18345_: vec4<f32>;
    var phi_18346_: bool;
    var phi_18347_: bool;
    var phi_18355_: vec4<f32>;
    var phi_18356_: bool;
    var phi_18502_: vec4<f32>;
    var phi_18503_: bool;
    var phi_18524_: type_66;
    var phi_18526_: type_66;
    var phi_18532_: u32;
    var phi_18178_: u32;
    var phi_18181_: u32;
    var phi_18183_: u32;
    var phi_18185_: u32;
    var phi_18189_: type_66;
    var phi_18708_: type_66;
    var phi_18709_: bool;
    var phi_18710_: bool;
    var phi_35128_: bool;
    var phi_20525_: bool;
    var phi_20524_: type_66;
    var phi_18882_: type_66;
    var phi_18887_: type_66;
    var phi_18939_: vec3<f32>;
    var phi_18940_: type_39;
    var phi_18941_: vec3<f32>;
    var phi_18942_: type_39;
    var phi_35565_: bool;
    var phi_18943_: vec3<f32>;
    var phi_18944_: type_39;
    var phi_18945_: u32;
    var phi_35564_: bool;
    var phi_18946_: vec3<f32>;
    var phi_18947_: type_39;
    var phi_18948_: u32;
    var phi_35856_: u32;
    var phi_35563_: bool;
    var phi_14688_: vec3<f32>;
    var phi_14691_: type_39;
    var phi_14693_: u32;
    var phi_14695_: u32;
    var phi_35576_: u32;
    var phi_35562_: bool;
    var phi_30340_: bool;
    var phi_30370_: bool;
    var phi_30359_: f32;
    var phi_30290_: bool;
    var phi_30426_: vec3<f32>;
    var phi_30582_: bool;
    var phi_30597_: bool;
    var phi_30612_: bool;
    var phi_30521_: vec3<f32>;
    var phi_30524_: vec3<f32>;
    var phi_30525_: bool;
    var phi_30295_: vec3<f32>;
    var phi_30299_: vec3<f32>;
    var phi_30300_: bool;
    var phi_30712_: bool;
    var phi_30686_: vec3<f32>;
    var phi_30307_: vec3<f32>;
    var phi_30727_: bool;
    var phi_30837_: f32;
    var phi_30865_: f32;
    var phi_30893_: f32;
    var phi_31027_: bool;
    var phi_31042_: bool;
    var phi_31057_: bool;
    var phi_31084_: bool;
    var phi_31099_: bool;
    var phi_31114_: bool;
    var phi_31141_: bool;
    var phi_31156_: bool;
    var phi_31171_: bool;
    var phi_19167_: f32;
    var phi_19175_: f32;
    var phi_19183_: f32;
    var phi_31368_: bool;
    var phi_31357_: f32;
    var phi_19290_: vec3<f32>;
    var phi_31396_: bool;
    var phi_19291_: vec3<f32>;
    var phi_19303_: type_56;
    var phi_19304_: type_56;
    var phi_19305_: bool;
    var phi_19310_: type_56;
    var phi_31433_: bool;
    var phi_31463_: bool;
    var phi_31452_: f32;
    var phi_31502_: vec3<f32>;
    var phi_19338_: type_56;
    var phi_19339_: type_56;
    var phi_31524_: vec3<f32>;
    var phi_19340_: type_56;
    var phi_19341_: type_56;
    var phi_19342_: bool;
    var phi_19346_: type_56;
    var phi_31549_: bool;
    var phi_19422_: vec3<f32>;
    var phi_37458_: u32;
    var phi_19424_: vec3<f32>;
    var phi_19425_: bool;
    var phi_37457_: u32;
    var phi_19426_: vec3<f32>;
    var phi_19427_: bool;
    var phi_37455_: u32;
    var phi_19433_: vec3<f32>;
    var phi_19434_: bool;
    var local_87: u32;
    var local_88: vec3<f32>;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: type_39;
    var phi_19463_: vec3<f32>;
    var local_92: u32;
    var local_93: vec3<f32>;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: type_39;
    var phi_19479_: type_71;
    var phi_19480_: type_71;
    var phi_37462_: u32;
    var phi_19482_: type_71;
    var phi_19483_: type_71;
    var phi_19484_: bool;
    var phi_37461_: u32;
    var phi_19485_: type_71;
    var phi_19486_: type_71;
    var phi_19487_: bool;
    var phi_19488_: bool;
    var local_97: u32;
    var local_98: vec3<f32>;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: type_39;
    var phi_19494_: type_71;
    var local_102: u32;
    var local_103: vec3<f32>;
    var local_104: vec3<f32>;
    var local_105: vec3<f32>;
    var local_106: type_39;
    var phi_19499_: type_71;
    var phi_19500_: type_71;
    var phi_19501_: bool;
    var phi_19505_: type_71;
    var phi_19506_: type_71;
    var phi_19507_: bool;
    var phi_13725_: bool;
    var phi_13734_: vec3<f32>;
    var phi_13736_: vec3<f32>;
    var phi_13737_: bool;
    var phi_13898_: f32;
    var phi_13901_: f32;
    var phi_31770_: bool;
    var phi_31751_: f32;
    var phi_13902_: f32;
    var phi_31788_: f32;
    var phi_31791_: f32;
    var phi_13905_: f32;
    var phi_13906_: f32;
    var phi_13907_: f32;
    var phi_19511_: type_71;
    var phi_19512_: type_71;
    var phi_19513_: bool;
    var phi_12161_: f32;
    var phi_31838_: f32;
    var phi_31841_: f32;
    var phi_12163_: f32;
    var phi_12164_: f32;
    var phi_31867_: bool;
    var phi_12215_: f32;
    var phi_31885_: f32;
    var phi_31888_: f32;
    var phi_12217_: f32;
    var phi_31899_: bool;
    var phi_31914_: bool;
    var phi_12686_: vec3<f32>;
    var phi_12687_: vec3<f32>;
    var phi_12688_: bool;
    var phi_12693_: vec3<f32>;
    var phi_12694_: bool;
    var phi_32038_: bool;
    var phi_32053_: bool;
    var phi_12985_: vec3<f32>;
    var phi_12987_: vec3<f32>;
    var phi_13014_: vec3<f32>;
    var phi_13028_: vec3<f32>;
    var phi_19517_: type_71;
    var phi_19518_: type_71;
    var phi_19519_: bool;
    var phi_19523_: type_71;
    var phi_11196_: bool;
    var phi_11197_: bool;
    var phi_11198_: bool;
    var phi_11219_: vec4<f32>;
    var phi_11220_: type_35;
    var phi_19527_: vec4<f32>;
    var phi_19528_: type_39;
    var phi_19529_: type_39;
    var phi_19530_: type_35;
    var local_107: type_66;
    var local_108: type_66;

    switch bitcast<i32>(0u) {
        default: {
            let _e278 = global;
            let _e290 = arrayLength((&global_8.member));
            let _e294 = (_e278.x + (_e278.y * 4194240u));
            let _e298 = global_1.member.member[0u];
            let _e302 = global_1.member.member[1u];
            if (_e294 >= (_e298 * _e302)) {
                phi_11127_ = type_45(0u, type_45().member_1);
            } else {
                phi_11127_ = type_45(1u, _e294);
            }
            let _e310 = phi_11127_;
            if (bitcast<i32>(_e310.member) != 0i) {
                let _e315 = (_e310.member_1 < arrayLength((&global_7.member)));
                if _e315 {
                } else {
                    break;
                }
                let _e318 = global_7.member[_e310.member_1];
                if ((_e318.member_4.y & 1u) == 0u) {
                    if ((_e318.member_4.y & 8u) == 0u) {
                        let _e7711 = (_e318.member_3.x * _e318.member.w);
                        let _e7712 = (_e318.member_3.y * _e318.member.w);
                        let _e7713 = (_e318.member_3.z * _e318.member.w);
                        if (_e7711 == _e7711) {
                            let _e7716 = (_e7712 == _e7712);
                            if _e7716 {
                                phi_11196_ = select(true, false, (_e7713 == _e7713));
                            } else {
                                phi_11196_ = false;
                            }
                            let _e7720 = phi_11196_;
                            phi_11197_ = select(true, false, _e7716);
                            phi_11198_ = _e7720;
                        } else {
                            phi_11197_ = true;
                            phi_11198_ = false;
                        }
                        let _e7723 = phi_11197_;
                        let _e7725 = phi_11198_;
                        let _e7728 = select(vec3<f32>(_e7711, _e7712, _e7713), vec3<f32>(0f, 0f, 0f), vec3(select(_e7725, true, _e7723)));
                        phi_11219_ = vec4<f32>(_e7728.x, _e7728.y, _e7728.z, _e318.member_3.w);
                        phi_11220_ = type_35(_e318.member, _e318.member_1, _e318.member_2, _e318.member_3, vec4<u32>(_e318.member_4.x, (_e318.member_4.y | 8u), _e318.member_4.z, _e318.member_4.w));
                    } else {
                        phi_11219_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_11220_ = _e318;
                    }
                    let _e7747 = phi_11219_;
                    let _e7749 = phi_11220_;
                    phi_19527_ = _e7747;
                    phi_19528_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_19529_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_19530_ = _e7749;
                } else {
                    let _e334 = vec3<f32>(_e318.member_1.x, _e318.member_1.y, _e318.member_1.z);
                    let _e339 = vec3<f32>(_e318.member_2.x, _e318.member_2.y, _e318.member_2.z);
                    let _e345 = select(select(u32(_e318.member_2.w), 0u, (_e318.member_2.w < 0f)), 4294967295u, (_e318.member_2.w > 4294967000f));
                    let _e349 = vec3<f32>(_e318.member_3.x, _e318.member_3.y, _e318.member_3.z);
                    let _e358 = ((_e318.member_4.y & 2u) != 0u);
                    let _e361 = global_1.member.member[3u];
                    let _e364 = global_1.member.member_3[3u];
                    let _e367 = global_1.member.member_1[2u];
                    let _e370 = global_1.member.member_1[0u];
                    let _e373 = global_1.member.member_1[1u];
                    let _e374 = (_e370 + _e373);
                    let _e375 = (vec3<f32>(1f, 1f, 1f) / _e334);
                    local_69 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                    local_70 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                    let _e377 = select(4294967295u, 0u, (_e370 > 0u));
                    phi_11316_ = _e377;
                    phi_11319_ = 0u;
                    phi_11321_ = _e374;
                    phi_11323_ = _e370;
                    phi_11325_ = vec4<f32>(0f, 0f, 0f, 0f);
                    phi_11327_ = type_66(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    loop {
                        let _e379 = phi_11316_;
                        let _e381 = phi_11319_;
                        let _e383 = phi_11321_;
                        let _e385 = phi_11323_;
                        let _e387 = phi_11325_;
                        let _e389 = phi_11327_;
                        local_71 = _e389;
                        local_72 = _e387;
                        local_73 = _e387;
                        local_74 = _e387;
                        local_75 = _e389;
                        local_107 = _e389;
                        if (_e385 < _e383) {
                            let _e757 = global_2.member[_e385];
                            if (_e757.member_6.x == 0u) {
                                let _e863 = (_e757.member_1.x - _e757.member.x);
                                let _e864 = (_e757.member_1.y - _e757.member.y);
                                let _e865 = (_e757.member_1.z - _e757.member.z);
                                let _e870 = (_e757.member_2.x - _e757.member.x);
                                let _e871 = (_e757.member_2.y - _e757.member.y);
                                let _e872 = (_e757.member_2.z - _e757.member.z);
                                let _e875 = ((_e318.member_1.y * _e872) - (_e871 * _e318.member_1.z));
                                let _e878 = ((_e318.member_1.z * _e870) - (_e872 * _e318.member_1.x));
                                let _e881 = ((_e318.member_1.x * _e871) - (_e870 * _e318.member_1.y));
                                let _e886 = (((_e863 * _e875) + (_e864 * _e878)) + (_e865 * _e881));
                                if (abs(_e886) < 0.000000000001f) {
                                    phi_11485_ = vec4<f32>();
                                    phi_11486_ = true;
                                } else {
                                    let _e889 = (1f / _e886);
                                    let _e890 = (_e318.member.x - _e757.member.x);
                                    let _e891 = (_e318.member.y - _e757.member.y);
                                    let _e892 = (_e318.member.z - _e757.member.z);
                                    let _e898 = ((((_e890 * _e875) + (_e891 * _e878)) + (_e892 * _e881)) * _e889);
                                    if (_e898 < 0f) {
                                        phi_11475_ = vec4<f32>();
                                        phi_11476_ = true;
                                        phi_11477_ = false;
                                    } else {
                                        let _e900 = (_e898 > 1f);
                                        if _e900 {
                                            phi_11473_ = vec4<f32>();
                                            phi_11474_ = false;
                                        } else {
                                            let _e903 = ((_e891 * _e865) - (_e864 * _e892));
                                            let _e906 = ((_e892 * _e863) - (_e865 * _e890));
                                            let _e909 = ((_e890 * _e864) - (_e863 * _e891));
                                            let _e915 = ((((_e318.member_1.x * _e903) + (_e318.member_1.y * _e906)) + (_e318.member_1.z * _e909)) * _e889);
                                            if (_e915 < 0f) {
                                                phi_11463_ = vec4<f32>();
                                                phi_11464_ = true;
                                                phi_11465_ = false;
                                            } else {
                                                let _e918 = ((_e898 + _e915) > 1f);
                                                if _e918 {
                                                    phi_11461_ = vec4<f32>();
                                                    phi_11462_ = false;
                                                } else {
                                                    let _e924 = ((((_e870 * _e903) + (_e871 * _e906)) + (_e872 * _e909)) * _e889);
                                                    if (_e924 <= 0f) {
                                                        phi_11456_ = vec4<f32>();
                                                        phi_11457_ = true;
                                                    } else {
                                                        let _e926 = (_e924 >= _e389.member);
                                                        if _e926 {
                                                            phi_11455_ = vec4<f32>();
                                                        } else {
                                                            phi_11455_ = vec4<f32>(_e924, _e898, _e915, 1f);
                                                        }
                                                        let _e929 = phi_11455_;
                                                        phi_11456_ = _e929;
                                                        phi_11457_ = _e926;
                                                    }
                                                    let _e931 = phi_11456_;
                                                    let _e933 = phi_11457_;
                                                    phi_11461_ = _e931;
                                                    phi_11462_ = _e933;
                                                }
                                                let _e935 = phi_11461_;
                                                let _e937 = phi_11462_;
                                                phi_11463_ = _e935;
                                                phi_11464_ = _e918;
                                                phi_11465_ = _e937;
                                            }
                                            let _e939 = phi_11463_;
                                            let _e941 = phi_11464_;
                                            let _e943 = phi_11465_;
                                            phi_11473_ = _e939;
                                            phi_11474_ = select(_e943, true, _e941);
                                        }
                                        let _e946 = phi_11473_;
                                        let _e948 = phi_11474_;
                                        phi_11475_ = _e946;
                                        phi_11476_ = _e900;
                                        phi_11477_ = _e948;
                                    }
                                    let _e950 = phi_11475_;
                                    let _e952 = phi_11476_;
                                    let _e954 = phi_11477_;
                                    phi_11485_ = _e950;
                                    phi_11486_ = select(_e954, true, _e952);
                                }
                                let _e957 = phi_11485_;
                                let _e959 = phi_11486_;
                                phi_11624_ = _e957;
                                phi_11625_ = _e959;
                            } else {
                                if (_e757.member_6.x == 1u) {
                                    let _e785 = (_e318.member.x - _e757.member.x);
                                    let _e786 = (_e318.member.y - _e757.member.y);
                                    let _e787 = (_e318.member.z - _e757.member.z);
                                    let _e792 = (((_e785 * _e318.member_1.x) + (_e786 * _e318.member_1.y)) + (_e787 * _e318.member_1.z));
                                    if (_e792 > 0f) {
                                        phi_11523_ = select(true, false, ((((_e785 * _e785) + (_e786 * _e786)) + (_e787 * _e787)) > (_e757.member.w * _e757.member.w)));
                                    } else {
                                        phi_11523_ = true;
                                    }
                                    let _e805 = phi_11523_;
                                    if _e805 {
                                        let _e809 = (_e785 - (_e318.member_1.x * _e792));
                                        let _e810 = (_e786 - (_e318.member_1.y * _e792));
                                        let _e811 = (_e787 - (_e318.member_1.z * _e792));
                                        let _e820 = ((_e757.member.w * _e757.member.w) - (((_e809 * _e809) + (_e810 * _e810)) + (_e811 * _e811)));
                                        if (_e820 < 0f) {
                                            phi_11579_ = vec4<f32>();
                                            phi_11580_ = true;
                                        } else {
                                            let _e822 = sqrt(_e820);
                                            let _e823 = -(_e792);
                                            let _e824 = (_e823 - _e822);
                                            if (_e824 <= 0f) {
                                                phi_11557_ = true;
                                            } else {
                                                phi_11557_ = (_e824 >= _e389.member);
                                            }
                                            let _e828 = phi_11557_;
                                            if _e828 {
                                                phi_11563_ = (_e823 + _e822);
                                            } else {
                                                phi_11563_ = _e824;
                                            }
                                            let _e831 = phi_11563_;
                                            if (_e831 <= 0f) {
                                                phi_11574_ = vec4<f32>();
                                                phi_11575_ = true;
                                            } else {
                                                let _e833 = (_e831 >= _e389.member);
                                                if _e833 {
                                                    phi_11573_ = vec4<f32>();
                                                } else {
                                                    phi_11573_ = vec4<f32>(_e831, 0f, 0f, 1f);
                                                }
                                                let _e836 = phi_11573_;
                                                phi_11574_ = _e836;
                                                phi_11575_ = _e833;
                                            }
                                            let _e838 = phi_11574_;
                                            let _e840 = phi_11575_;
                                            phi_11579_ = _e838;
                                            phi_11580_ = _e840;
                                        }
                                        let _e842 = phi_11579_;
                                        let _e844 = phi_11580_;
                                        phi_11584_ = _e842;
                                        phi_11585_ = _e844;
                                    } else {
                                        phi_11584_ = vec4<f32>();
                                        phi_11585_ = true;
                                    }
                                    let _e846 = phi_11584_;
                                    let _e848 = phi_11585_;
                                    phi_11614_ = _e846;
                                    phi_11615_ = _e848;
                                } else {
                                    if (abs(_e318.member_1.y) < 0.000000000001f) {
                                        phi_11612_ = vec4<f32>();
                                        phi_11613_ = true;
                                    } else {
                                        let _e767 = ((_e757.member.x - _e318.member.y) / _e318.member_1.y);
                                        if (_e767 <= 0f) {
                                            phi_11607_ = vec4<f32>();
                                            phi_11608_ = true;
                                        } else {
                                            let _e769 = (_e767 >= _e389.member);
                                            if _e769 {
                                                phi_11606_ = vec4<f32>();
                                            } else {
                                                phi_11606_ = vec4<f32>(_e767, 0f, 0f, 1f);
                                            }
                                            let _e772 = phi_11606_;
                                            phi_11607_ = _e772;
                                            phi_11608_ = _e769;
                                        }
                                        let _e774 = phi_11607_;
                                        let _e776 = phi_11608_;
                                        phi_11612_ = _e774;
                                        phi_11613_ = _e776;
                                    }
                                    let _e778 = phi_11612_;
                                    let _e780 = phi_11613_;
                                    phi_11614_ = _e778;
                                    phi_11615_ = _e780;
                                }
                                let _e850 = phi_11614_;
                                let _e852 = phi_11615_;
                                phi_11624_ = _e850;
                                phi_11625_ = select(true, false, select(true, false, _e852));
                            }
                            let _e961 = phi_11624_;
                            let _e963 = phi_11625_;
                            let _e966 = select(vec4<f32>(0f, 0f, 0f, 0f), _e961, vec4(select(true, false, _e963)));
                            let _e968 = (_e966.w > 0f);
                            if _e968 {
                                let _e974 = type_66(_e966.x, _e389.member_1, _e389.member_2, _e389.member_3);
                                let _e979 = type_66(_e974.member, _e385, _e974.member_2, _e974.member_3);
                                phi_11646_ = _e979;
                                phi_11648_ = _e979;
                            } else {
                                phi_11646_ = type_66();
                                phi_11648_ = _e389;
                            }
                            let _e981 = phi_11646_;
                            let _e983 = phi_11648_;
                            phi_11317_ = _e379;
                            phi_11320_ = _e381;
                            phi_11322_ = _e383;
                            phi_11324_ = (_e385 + 1u);
                            phi_11326_ = select(_e387, _e966, vec4(_e968));
                            phi_11328_ = _e983;
                            phi_11830_ = _e981;
                            phi_11831_ = true;
                            phi_11832_ = false;
                        } else {
                            if (_e379 == 4294967295u) {
                                let _e392 = (_e381 == 0u);
                                if _e392 {
                                    phi_11685_ = u32();
                                    phi_11686_ = u32();
                                    phi_11688_ = false;
                                    phi_11689_ = false;
                                } else {
                                    let _e393 = (_e381 - 1u);
                                    let _e394 = (_e393 < 64u);
                                    if _e394 {
                                    } else {
                                        phi_32834_ = true;
                                        phi_20523_ = bool();
                                        phi_20522_ = type_66();
                                        break;
                                    }
                                    let _e396 = local_70[_e393];
                                    let _e398 = (_e396 >= _e389.member);
                                    if _e398 {
                                        phi_11683_ = u32();
                                    } else {
                                        if _e394 {
                                        } else {
                                            phi_32834_ = true;
                                            phi_20523_ = bool();
                                            phi_20522_ = type_66();
                                            break;
                                        }
                                        let _e400 = local_69[_e393];
                                        phi_11683_ = _e400;
                                    }
                                    let _e402 = phi_11683_;
                                    phi_11685_ = _e393;
                                    phi_11686_ = _e402;
                                    phi_11688_ = select(true, false, _e398);
                                    phi_11689_ = _e398;
                                }
                                let _e405 = phi_11685_;
                                let _e407 = phi_11686_;
                                let _e409 = phi_11688_;
                                let _e411 = phi_11689_;
                                phi_11690_ = _e405;
                                phi_11691_ = _e407;
                                phi_11692_ = _e405;
                                phi_11693_ = _e409;
                                phi_11694_ = _e411;
                                phi_11695_ = _e392;
                            } else {
                                phi_11690_ = u32();
                                phi_11691_ = _e379;
                                phi_11692_ = _e381;
                                phi_11693_ = true;
                                phi_11694_ = false;
                                phi_11695_ = false;
                            }
                            let _e413 = phi_11690_;
                            let _e415 = phi_11691_;
                            let _e417 = phi_11692_;
                            let _e419 = phi_11693_;
                            let _e421 = phi_11694_;
                            let _e423 = phi_11695_;
                            if _e419 {
                                let _e425 = ((_e415 & 2147483648u) == 0u);
                                if _e425 {
                                    let _e433 = global_3.member[_e415];
                                    if (_e433.member_1 == 4294967295u) {
                                        phi_11725_ = 300000000000000000000000000000000000000f;
                                    } else {
                                        let _e446 = ((_e433.member.x - _e318.member.x) * _e375.x);
                                        let _e448 = ((_e433.member.y - _e318.member.y) * _e375.y);
                                        let _e450 = ((_e433.member.z - _e318.member.z) * _e375.z);
                                        let _e457 = ((_e433.member_2.x - _e318.member.x) * _e375.x);
                                        let _e458 = ((_e433.member_2.y - _e318.member.y) * _e375.y);
                                        let _e459 = ((_e433.member_2.z - _e318.member.z) * _e375.z);
                                        if (_e446 == _e446) {
                                            let _e461 = (_e457 == _e457);
                                            if _e461 {
                                                let _e462 = (_e446 != _e446);
                                                if _e462 {
                                                    phi_24926_ = true;
                                                } else {
                                                    phi_24926_ = (_e457 <= _e446);
                                                }
                                                let _e465 = phi_24926_;
                                                let _e466 = select(_e446, _e457, _e465);
                                                if _e462 {
                                                    phi_24956_ = true;
                                                } else {
                                                    phi_24956_ = (_e457 >= _e446);
                                                }
                                                let _e471 = phi_24956_;
                                                let _e472 = select(_e446, _e457, _e471);
                                                phi_24903_ = type_53(select(-300000000000000000000000000000000000000f, _e466, (_e466 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e472, (_e472 <= 300000000000000000000000000000000000000f)));
                                            } else {
                                                phi_24903_ = type_53();
                                            }
                                            let _e477 = phi_24903_;
                                            phi_24906_ = _e477;
                                            phi_24907_ = select(true, false, _e461);
                                        } else {
                                            phi_24906_ = type_53();
                                            phi_24907_ = true;
                                        }
                                        let _e480 = phi_24906_;
                                        let _e482 = phi_24907_;
                                        if _e482 {
                                            phi_24912_ = type_53(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                        } else {
                                            phi_24912_ = _e480;
                                        }
                                        let _e484 = phi_24912_;
                                        if (_e448 == _e448) {
                                            let _e488 = (_e458 == _e458);
                                            if _e488 {
                                                let _e489 = (_e448 != _e448);
                                                if _e489 {
                                                    phi_25013_ = true;
                                                } else {
                                                    phi_25013_ = (_e458 <= _e448);
                                                }
                                                let _e492 = phi_25013_;
                                                let _e493 = select(_e448, _e458, _e492);
                                                if (_e484.member != _e484.member) {
                                                    phi_25028_ = true;
                                                } else {
                                                    phi_25028_ = (_e493 >= _e484.member);
                                                }
                                                let _e497 = phi_25028_;
                                                if _e489 {
                                                    phi_25043_ = true;
                                                } else {
                                                    phi_25043_ = (_e458 >= _e448);
                                                }
                                                let _e501 = phi_25043_;
                                                let _e502 = select(_e448, _e458, _e501);
                                                if (_e484.member_1 != _e484.member_1) {
                                                    phi_25058_ = true;
                                                } else {
                                                    phi_25058_ = (_e502 <= _e484.member_1);
                                                }
                                                let _e506 = phi_25058_;
                                                phi_24990_ = type_53(select(_e484.member, _e493, _e497), select(_e484.member_1, _e502, _e506));
                                            } else {
                                                phi_24990_ = type_53();
                                            }
                                            let _e510 = phi_24990_;
                                            phi_24993_ = _e510;
                                            phi_24994_ = select(true, false, _e488);
                                        } else {
                                            phi_24993_ = type_53();
                                            phi_24994_ = true;
                                        }
                                        let _e513 = phi_24993_;
                                        let _e515 = phi_24994_;
                                        if _e515 {
                                            phi_24999_ = _e484;
                                        } else {
                                            phi_24999_ = _e513;
                                        }
                                        let _e517 = phi_24999_;
                                        if (_e450 == _e450) {
                                            let _e521 = (_e459 == _e459);
                                            if _e521 {
                                                let _e522 = (_e450 != _e450);
                                                if _e522 {
                                                    phi_25100_ = true;
                                                } else {
                                                    phi_25100_ = (_e459 <= _e450);
                                                }
                                                let _e525 = phi_25100_;
                                                let _e526 = select(_e450, _e459, _e525);
                                                if (_e517.member != _e517.member) {
                                                    phi_25115_ = true;
                                                } else {
                                                    phi_25115_ = (_e526 >= _e517.member);
                                                }
                                                let _e530 = phi_25115_;
                                                if _e522 {
                                                    phi_25130_ = true;
                                                } else {
                                                    phi_25130_ = (_e459 >= _e450);
                                                }
                                                let _e534 = phi_25130_;
                                                let _e535 = select(_e450, _e459, _e534);
                                                if (_e517.member_1 != _e517.member_1) {
                                                    phi_25145_ = true;
                                                } else {
                                                    phi_25145_ = (_e535 <= _e517.member_1);
                                                }
                                                let _e539 = phi_25145_;
                                                phi_25077_ = type_53(select(_e517.member, _e526, _e530), select(_e517.member_1, _e535, _e539));
                                            } else {
                                                phi_25077_ = type_53();
                                            }
                                            let _e543 = phi_25077_;
                                            phi_25080_ = _e543;
                                            phi_25081_ = select(true, false, _e521);
                                        } else {
                                            phi_25080_ = type_53();
                                            phi_25081_ = true;
                                        }
                                        let _e546 = phi_25080_;
                                        let _e548 = phi_25081_;
                                        if _e548 {
                                            phi_25086_ = _e517;
                                        } else {
                                            phi_25086_ = _e546;
                                        }
                                        let _e550 = phi_25086_;
                                        if (_e550.member_1 < 0f) {
                                            phi_24884_ = true;
                                        } else {
                                            if (_e550.member > _e550.member_1) {
                                                phi_24881_ = true;
                                            } else {
                                                phi_24881_ = (_e550.member > _e389.member);
                                            }
                                            let _e557 = phi_24881_;
                                            phi_24884_ = _e557;
                                        }
                                        let _e559 = phi_24884_;
                                        phi_11725_ = select(_e550.member, 300000000000000000000000000000000000000f, _e559);
                                    }
                                    let _e562 = phi_11725_;
                                    if (_e433.member_5 == 4294967295u) {
                                        phi_11735_ = 300000000000000000000000000000000000000f;
                                    } else {
                                        let _e575 = ((_e433.member_4.x - _e318.member.x) * _e375.x);
                                        let _e577 = ((_e433.member_4.y - _e318.member.y) * _e375.y);
                                        let _e579 = ((_e433.member_4.z - _e318.member.z) * _e375.z);
                                        let _e586 = ((_e433.member_6.x - _e318.member.x) * _e375.x);
                                        let _e587 = ((_e433.member_6.y - _e318.member.y) * _e375.y);
                                        let _e588 = ((_e433.member_6.z - _e318.member.z) * _e375.z);
                                        if (_e575 == _e575) {
                                            let _e590 = (_e586 == _e586);
                                            if _e590 {
                                                let _e591 = (_e575 != _e575);
                                                if _e591 {
                                                    phi_25247_ = true;
                                                } else {
                                                    phi_25247_ = (_e586 <= _e575);
                                                }
                                                let _e594 = phi_25247_;
                                                let _e595 = select(_e575, _e586, _e594);
                                                if _e591 {
                                                    phi_25277_ = true;
                                                } else {
                                                    phi_25277_ = (_e586 >= _e575);
                                                }
                                                let _e600 = phi_25277_;
                                                let _e601 = select(_e575, _e586, _e600);
                                                phi_25224_ = type_53(select(-300000000000000000000000000000000000000f, _e595, (_e595 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e601, (_e601 <= 300000000000000000000000000000000000000f)));
                                            } else {
                                                phi_25224_ = type_53();
                                            }
                                            let _e606 = phi_25224_;
                                            phi_25227_ = _e606;
                                            phi_25228_ = select(true, false, _e590);
                                        } else {
                                            phi_25227_ = type_53();
                                            phi_25228_ = true;
                                        }
                                        let _e609 = phi_25227_;
                                        let _e611 = phi_25228_;
                                        if _e611 {
                                            phi_25233_ = type_53(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                        } else {
                                            phi_25233_ = _e609;
                                        }
                                        let _e613 = phi_25233_;
                                        if (_e577 == _e577) {
                                            let _e617 = (_e587 == _e587);
                                            if _e617 {
                                                let _e618 = (_e577 != _e577);
                                                if _e618 {
                                                    phi_25334_ = true;
                                                } else {
                                                    phi_25334_ = (_e587 <= _e577);
                                                }
                                                let _e621 = phi_25334_;
                                                let _e622 = select(_e577, _e587, _e621);
                                                if (_e613.member != _e613.member) {
                                                    phi_25349_ = true;
                                                } else {
                                                    phi_25349_ = (_e622 >= _e613.member);
                                                }
                                                let _e626 = phi_25349_;
                                                if _e618 {
                                                    phi_25364_ = true;
                                                } else {
                                                    phi_25364_ = (_e587 >= _e577);
                                                }
                                                let _e630 = phi_25364_;
                                                let _e631 = select(_e577, _e587, _e630);
                                                if (_e613.member_1 != _e613.member_1) {
                                                    phi_25379_ = true;
                                                } else {
                                                    phi_25379_ = (_e631 <= _e613.member_1);
                                                }
                                                let _e635 = phi_25379_;
                                                phi_25311_ = type_53(select(_e613.member, _e622, _e626), select(_e613.member_1, _e631, _e635));
                                            } else {
                                                phi_25311_ = type_53();
                                            }
                                            let _e639 = phi_25311_;
                                            phi_25314_ = _e639;
                                            phi_25315_ = select(true, false, _e617);
                                        } else {
                                            phi_25314_ = type_53();
                                            phi_25315_ = true;
                                        }
                                        let _e642 = phi_25314_;
                                        let _e644 = phi_25315_;
                                        if _e644 {
                                            phi_25320_ = _e613;
                                        } else {
                                            phi_25320_ = _e642;
                                        }
                                        let _e646 = phi_25320_;
                                        if (_e579 == _e579) {
                                            let _e650 = (_e588 == _e588);
                                            if _e650 {
                                                let _e651 = (_e579 != _e579);
                                                if _e651 {
                                                    phi_25421_ = true;
                                                } else {
                                                    phi_25421_ = (_e588 <= _e579);
                                                }
                                                let _e654 = phi_25421_;
                                                let _e655 = select(_e579, _e588, _e654);
                                                if (_e646.member != _e646.member) {
                                                    phi_25436_ = true;
                                                } else {
                                                    phi_25436_ = (_e655 >= _e646.member);
                                                }
                                                let _e659 = phi_25436_;
                                                if _e651 {
                                                    phi_25451_ = true;
                                                } else {
                                                    phi_25451_ = (_e588 >= _e579);
                                                }
                                                let _e663 = phi_25451_;
                                                let _e664 = select(_e579, _e588, _e663);
                                                if (_e646.member_1 != _e646.member_1) {
                                                    phi_25466_ = true;
                                                } else {
                                                    phi_25466_ = (_e664 <= _e646.member_1);
                                                }
                                                let _e668 = phi_25466_;
                                                phi_25398_ = type_53(select(_e646.member, _e655, _e659), select(_e646.member_1, _e664, _e668));
                                            } else {
                                                phi_25398_ = type_53();
                                            }
                                            let _e672 = phi_25398_;
                                            phi_25401_ = _e672;
                                            phi_25402_ = select(true, false, _e650);
                                        } else {
                                            phi_25401_ = type_53();
                                            phi_25402_ = true;
                                        }
                                        let _e675 = phi_25401_;
                                        let _e677 = phi_25402_;
                                        if _e677 {
                                            phi_25407_ = _e646;
                                        } else {
                                            phi_25407_ = _e675;
                                        }
                                        let _e679 = phi_25407_;
                                        if (_e679.member_1 < 0f) {
                                            phi_25205_ = true;
                                        } else {
                                            if (_e679.member > _e679.member_1) {
                                                phi_25202_ = true;
                                            } else {
                                                phi_25202_ = (_e679.member > _e389.member);
                                            }
                                            let _e686 = phi_25202_;
                                            phi_25205_ = _e686;
                                        }
                                        let _e688 = phi_25205_;
                                        phi_11735_ = select(_e679.member, 300000000000000000000000000000000000000f, _e688);
                                    }
                                    let _e691 = phi_11735_;
                                    if (_e433.member_3 > 0u) {
                                        phi_11746_ = ((2147483648u | ((_e433.member_3 - 1u) << bitcast<u32>(28u))) | _e433.member_1);
                                    } else {
                                        phi_11746_ = _e433.member_1;
                                    }
                                    let _e700 = phi_11746_;
                                    if (_e433.member_7 > 0u) {
                                        phi_11757_ = ((2147483648u | ((_e433.member_7 - 1u) << bitcast<u32>(28u))) | _e433.member_5);
                                    } else {
                                        phi_11757_ = _e433.member_5;
                                    }
                                    let _e709 = phi_11757_;
                                    let _e710 = (_e562 <= _e691);
                                    let _e714 = select(_e562, _e691, _e710);
                                    if (_e714 < 300000000000000000000000000000000000000f) {
                                        let _e716 = (_e417 < 64u);
                                        if _e716 {
                                            if _e716 {
                                            } else {
                                                phi_32834_ = true;
                                                phi_20523_ = bool();
                                                phi_20522_ = type_66();
                                                break;
                                            }
                                            local_69[_e417] = select(_e700, _e709, _e710);
                                            if _e716 {
                                            } else {
                                                phi_32834_ = true;
                                                phi_20523_ = bool();
                                                phi_20522_ = type_66();
                                                break;
                                            }
                                            local_70[_e417] = _e714;
                                            phi_11796_ = (_e417 + 1u);
                                        } else {
                                            phi_11796_ = _e417;
                                        }
                                        let _e721 = phi_11796_;
                                        phi_11797_ = _e721;
                                    } else {
                                        phi_11797_ = _e417;
                                    }
                                    let _e723 = phi_11797_;
                                    phi_11808_ = select(4294967295u, select(_e709, _e700, _e710), (select(_e691, _e562, _e710) < 300000000000000000000000000000000000000f));
                                    phi_11809_ = _e723;
                                    phi_11810_ = u32();
                                    phi_11811_ = u32();
                                } else {
                                    let _e426 = (_e415 & 268435455u);
                                    phi_11808_ = u32();
                                    phi_11809_ = u32();
                                    phi_11810_ = ((_e426 + ((_e415 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                    phi_11811_ = _e426;
                                }
                                let _e727 = phi_11808_;
                                let _e729 = phi_11809_;
                                let _e731 = phi_11810_;
                                let _e733 = phi_11811_;
                                phi_11813_ = _e727;
                                phi_11814_ = _e729;
                                phi_11816_ = _e731;
                                phi_11817_ = _e733;
                                phi_11818_ = select(true, false, _e425);
                                phi_11819_ = _e425;
                            } else {
                                phi_11813_ = u32();
                                phi_11814_ = u32();
                                phi_11816_ = _e383;
                                phi_11817_ = _e385;
                                phi_11818_ = _e421;
                                phi_11819_ = false;
                            }
                            let _e736 = phi_11813_;
                            let _e738 = phi_11814_;
                            let _e740 = phi_11816_;
                            let _e742 = phi_11817_;
                            let _e744 = phi_11818_;
                            let _e746 = phi_11819_;
                            phi_11317_ = select(_e736, 4294967295u, _e744);
                            phi_11320_ = select(_e738, select(_e413, _e417, _e419), _e744);
                            phi_11322_ = select(_e383, _e740, _e744);
                            phi_11324_ = select(_e385, _e742, _e744);
                            phi_11326_ = _e387;
                            phi_11328_ = _e389;
                            phi_11830_ = type_66();
                            phi_11831_ = select(_e746, true, _e744);
                            phi_11832_ = select(select(_e423, false, _e419), false, _e744);
                        }
                        let _e988 = phi_11317_;
                        let _e990 = phi_11320_;
                        let _e992 = phi_11322_;
                        let _e994 = phi_11324_;
                        let _e996 = phi_11326_;
                        let _e998 = phi_11328_;
                        let _e1000 = phi_11830_;
                        let _e1002 = phi_11831_;
                        let _e1004 = phi_11832_;
                        phi_11316_ = _e988;
                        phi_11319_ = _e990;
                        phi_11321_ = _e992;
                        phi_11323_ = _e994;
                        phi_11325_ = _e996;
                        phi_11327_ = _e998;
                        phi_32834_ = false;
                        phi_20523_ = _e1004;
                        phi_20522_ = _e1000;
                        if !(_e1002) {
                            break;
                        }
                    }
                    let _e1007 = phi_32834_;
                    let _e1009 = phi_20523_;
                    let _e1011 = phi_20522_;
                    if _e1007 {
                        break;
                    }
                    if _e1009 {
                        let _e1013 = local_71;
                        if (_e1013.member_1 != 4294967295u) {
                            let _e1017 = global_2.member[_e1013.member_1];
                            if (_e1017.member_6.x == 0u) {
                                let _e1059 = (_e1017.member_1.x - _e1017.member.x);
                                let _e1060 = (_e1017.member_1.y - _e1017.member.y);
                                let _e1061 = (_e1017.member_1.z - _e1017.member.z);
                                let _e1066 = (_e1017.member_2.x - _e1017.member.x);
                                let _e1067 = (_e1017.member_2.y - _e1017.member.y);
                                let _e1068 = (_e1017.member_2.z - _e1017.member.z);
                                let _e1071 = ((_e1060 * _e1068) - (_e1067 * _e1061));
                                let _e1074 = ((_e1061 * _e1066) - (_e1068 * _e1059));
                                let _e1077 = ((_e1059 * _e1067) - (_e1066 * _e1060));
                                let _e1084 = (1f / sqrt((((_e1071 * _e1071) + (_e1074 * _e1074)) + (_e1077 * _e1077))));
                                let _e1088 = vec3<f32>((_e1071 * _e1084), (_e1074 * _e1084), (_e1077 * _e1084));
                                if (_e1017.member_6.w == 1u) {
                                    let _e1097 = local_73;
                                    let _e1101 = local_74;
                                    let _e1103 = ((1f - _e1097.y) - _e1101.z);
                                    let _e1124 = (((_e1017.member_3.x * _e1103) + (_e1017.member_4.x * _e1097.y)) + (_e1017.member_5.x * _e1101.z));
                                    let _e1125 = (((_e1017.member_3.y * _e1103) + (_e1017.member_4.y * _e1097.y)) + (_e1017.member_5.y * _e1101.z));
                                    let _e1126 = (((_e1017.member_3.z * _e1103) + (_e1017.member_4.z * _e1097.y)) + (_e1017.member_5.z * _e1101.z));
                                    let _e1131 = (((_e1124 * _e1124) + (_e1125 * _e1125)) + (_e1126 * _e1126));
                                    if (_e1131 > 0f) {
                                        let _e1134 = (1f / sqrt(_e1131));
                                        phi_11959_ = vec3<f32>((_e1124 * _e1134), (_e1125 * _e1134), (_e1126 * _e1134));
                                    } else {
                                        phi_11959_ = _e1088;
                                    }
                                    let _e1140 = phi_11959_;
                                    phi_11960_ = _e1140;
                                } else {
                                    phi_11960_ = _e1088;
                                }
                                let _e1142 = phi_11960_;
                                phi_11994_ = type_57(_e1088, _e1142);
                            } else {
                                if (_e1017.member_6.x == 1u) {
                                    let _e1027 = local_72;
                                    let _e1047 = vec3<f32>((((_e318.member.x + (_e318.member_1.x * _e1027.x)) - _e1017.member.x) / _e1017.member.w), (((_e318.member.y + (_e318.member_1.y * _e1027.x)) - _e1017.member.y) / _e1017.member.w), (((_e318.member.z + (_e318.member_1.z * _e1027.x)) - _e1017.member.z) / _e1017.member.w));
                                    phi_11993_ = type_57(_e1047, _e1047);
                                } else {
                                    let _e1024 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e318.member_1.y > 0f)));
                                    phi_11993_ = type_57(_e1024, _e1024);
                                }
                                let _e1050 = phi_11993_;
                                phi_11994_ = _e1050;
                            }
                            let _e1145 = phi_11994_;
                            let _e1149 = local_75;
                            let _e1154 = type_66(_e1149.member, _e1149.member_1, _e1145.member, _e1149.member_3);
                            phi_11999_ = type_66(_e1154.member, _e1154.member_1, _e1154.member_2, _e1145.member_1);
                        } else {
                            let _e7933 = local_107;
                            phi_11999_ = _e7933;
                        }
                        let _e1161 = phi_11999_;
                        phi_12000_ = _e1161;
                    } else {
                        phi_12000_ = type_66();
                    }
                    let _e1163 = phi_12000_;
                    if select(true, false, _e1009) {
                        phi_12005_ = _e1011;
                    } else {
                        phi_12005_ = _e1163;
                    }
                    let _e1166 = phi_12005_;
                    let _e1167 = (select(select(u32(_e318.member_3.w), 0u, (_e318.member_3.w < 0f)), 4294967295u, (_e318.member_3.w > 4294967000f)) + 1u);
                    if (_e1166.member_1 == 4294967295u) {
                        if _e358 {
                            phi_12164_ = 1f;
                        } else {
                            if ((_e364 & 2u) == 0u) {
                                phi_12163_ = 1f;
                            } else {
                                let _e7192 = global_1.member.member_3[1u];
                                let _e7195 = global_1.member.member_3[2u];
                                let _e7198 = ((atan2(_e318.member_1.z, _e318.member_1.x) * 0.15915494f) + 0.5f);
                                let _e7202 = select(_e318.member_1.y, -1f, (_e318.member_1.y < -1f));
                                let _e7205 = acos(select(_e7202, 1f, (_e7202 > 1f)));
                                let _e7207 = f32(_e7192);
                                let _e7208 = ((_e7198 - floor(_e7198)) * _e7207);
                                let _e7213 = select(select(u32(_e7208), 0u, (_e7208 < 0f)), 4294967295u, (_e7208 > 4294967000f));
                                let _e7214 = (_e7192 - 1u);
                                let _e7217 = f32(_e7195);
                                let _e7218 = ((_e7205 * 0.31830987f) * _e7217);
                                let _e7223 = select(select(u32(_e7218), 0u, (_e7218 < 0f)), 4294967295u, (_e7218 > 4294967000f));
                                let _e7224 = (_e7195 - 1u);
                                let _e7226 = select(_e7223, _e7224, (_e7224 < _e7223));
                                let _e7227 = sin(_e7205);
                                if (_e7227 <= 0f) {
                                    phi_12161_ = 0f;
                                } else {
                                    let _e7231 = global_1.member.member_2[3u];
                                    let _e7234 = global_1.member.member_3[0u];
                                    let _e7238 = (_e7231 + _e7226);
                                    let _e7241 = global_6.member[(_e7238 + 1u)];
                                    let _e7243 = global_6.member[_e7238];
                                    let _e7246 = ((_e7234 + (_e7226 * (_e7192 + 1u))) + select(_e7213, _e7214, (_e7214 < _e7213)));
                                    let _e7249 = global_6.member[(_e7246 + 1u)];
                                    let _e7251 = global_6.member[_e7246];
                                    phi_12161_ = ((((_e7241 - _e7243) * _e7217) * ((_e7249 - _e7251) * _e7207)) / (19.73921f * _e7227));
                                }
                                let _e7258 = phi_12161_;
                                if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_31841_ = 1f;
                                } else {
                                    let _e7260 = (_e318.member_1.w * _e318.member_1.w);
                                    let _e7262 = (_e7260 + (_e7258 * _e7258));
                                    if (_e7262 == 0f) {
                                        phi_31838_ = 0f;
                                    } else {
                                        phi_31838_ = (_e7260 / _e7262);
                                    }
                                    let _e7266 = phi_31838_;
                                    phi_31841_ = _e7266;
                                }
                                let _e7268 = phi_31841_;
                                phi_12163_ = _e7268;
                            }
                            let _e7270 = phi_12163_;
                            phi_12164_ = _e7270;
                        }
                        let _e7272 = phi_12164_;
                        if ((_e364 & 4u) == 0u) {
                            phi_13028_ = _e349;
                        } else {
                            if _e358 {
                                phi_12217_ = 1f;
                            } else {
                                let _e7283 = global_1.member.member_5;
                                let _e7287 = (_e318.member_1.x - _e7283.x);
                                let _e7288 = (_e318.member_1.y - _e7283.y);
                                let _e7289 = (_e318.member_1.z - _e7283.z);
                                let _e7296 = (sqrt((((_e7287 * _e7287) + (_e7288 * _e7288)) + (_e7289 * _e7289))) * 0.5f);
                                if (_e7296 != _e7296) {
                                    phi_31867_ = true;
                                } else {
                                    phi_31867_ = (1f <= _e7296);
                                }
                                let _e7300 = phi_31867_;
                                let _e7306 = global_1.member.member_6[3u];
                                if ((2f * asin(select(_e7296, 1f, _e7300))) >= _e7306) {
                                    phi_12215_ = 0f;
                                } else {
                                    let _e7309 = sin((_e7306 * 0.5f));
                                    phi_12215_ = (1f / ((12.566371f * _e7309) * _e7309));
                                }
                                let _e7314 = phi_12215_;
                                if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_31888_ = 1f;
                                } else {
                                    let _e7316 = (_e318.member_1.w * _e318.member_1.w);
                                    let _e7318 = (_e7316 + (_e7314 * _e7314));
                                    if (_e7318 == 0f) {
                                        phi_31885_ = 0f;
                                    } else {
                                        phi_31885_ = (_e7316 / _e7318);
                                    }
                                    let _e7322 = phi_31885_;
                                    phi_31888_ = _e7322;
                                }
                                let _e7324 = phi_31888_;
                                phi_12217_ = _e7324;
                            }
                            let _e7326 = phi_12217_;
                            let _e7328 = global_1.member.member_5;
                            let _e7332 = (_e318.member_1.x - _e7328.x);
                            let _e7333 = (_e318.member_1.y - _e7328.y);
                            let _e7334 = (_e318.member_1.z - _e7328.z);
                            let _e7341 = (sqrt((((_e7332 * _e7332) + (_e7333 * _e7333)) + (_e7334 * _e7334))) * 0.5f);
                            if (_e7341 != _e7341) {
                                phi_31899_ = true;
                            } else {
                                phi_31899_ = (1f <= _e7341);
                            }
                            let _e7345 = phi_31899_;
                            let _e7348 = (2f * asin(select(_e7341, 1f, _e7345)));
                            let _e7352 = global_1.member.member_6[3u];
                            if (_e7348 >= _e7352) {
                                phi_13014_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                let _e7356 = (1f - pow((_e7348 / _e7352), 2f));
                                if (_e7356 != _e7356) {
                                    phi_31914_ = true;
                                } else {
                                    phi_31914_ = (0f >= _e7356);
                                }
                                let _e7360 = phi_31914_;
                                let _e7363 = global_1.member.member_6;
                                let _e7369 = global_1.member.member_8[2u];
                                if (_e7369 == 4294967295u) {
                                    phi_12987_ = vec3<f32>(1f, 1f, 1f);
                                } else {
                                    let _e7372 = global_6.member[_e7369];
                                    let _e7375 = global_6.member[(_e7369 + 1u)];
                                    let _e7378 = global_6.member[(_e7369 + 2u)];
                                    let _e7381 = global_6.member[(_e7369 + 3u)];
                                    let _e7384 = global_6.member[(_e7369 + 4u)];
                                    let _e7387 = global_6.member[(_e7369 + 5u)];
                                    let _e7390 = global_6.member[(_e7369 + 6u)];
                                    let _e7393 = global_6.member[(_e7369 + 7u)];
                                    let _e7396 = global_6.member[(_e7369 + 8u)];
                                    let _e7399 = global_6.member[(_e7369 + 9u)];
                                    let _e7402 = global_6.member[(_e7369 + 13u)];
                                    let _e7405 = global_6.member[(_e7369 + 14u)];
                                    let _e7408 = global_6.member[(_e7369 + 15u)];
                                    let _e7411 = global_6.member[(_e7369 + 16u)];
                                    let _e7414 = global_6.member[(_e7369 + 18u)];
                                    let _e7417 = global_6.member[(_e7369 + 19u)];
                                    let _e7420 = global_6.member[(_e7369 + 20u)];
                                    let _e7423 = global_6.member[(_e7369 + 29u)];
                                    let _e7424 = bitcast<u32>(_e7423);
                                    let _e7437 = (((((_e318.member.x * _e318.member.x) + (_e318.member.y * _e318.member.y)) + (_e318.member.z * _e318.member.z)) - (2f * (((_e318.member.x * _e7372) + (_e318.member.y * _e7375)) + (_e318.member.z * _e7378)))) + _e7390);
                                    if (_e7437 > 0f) {
                                        let _e7446 = ((((_e318.member.x - _e7372) * _e7393) + ((_e318.member.y - _e7375) * _e7396)) + ((_e318.member.z - _e7378) * _e7399));
                                        let _e7448 = ((_e7446 * _e7446) - _e7437);
                                        if (_e7446 >= 0f) {
                                            phi_12687_ = vec3<f32>();
                                            phi_12688_ = true;
                                        } else {
                                            let _e7450 = (_e7448 <= 0f);
                                            if _e7450 {
                                                phi_12686_ = vec3<f32>();
                                            } else {
                                                let _e7453 = (-(_e7446) - sqrt(_e7448));
                                                phi_12686_ = vec3<f32>((_e318.member.x + (_e7393 * _e7453)), (_e318.member.y + (_e7396 * _e7453)), (_e318.member.z + (_e7399 * _e7453)));
                                            }
                                            let _e7462 = phi_12686_;
                                            phi_12687_ = _e7462;
                                            phi_12688_ = _e7450;
                                        }
                                        let _e7464 = phi_12687_;
                                        let _e7466 = phi_12688_;
                                        phi_12693_ = _e7464;
                                        phi_12694_ = select(true, false, _e7466);
                                    } else {
                                        phi_12693_ = vec3<f32>(_e318.member.x, _e318.member.y, _e318.member.z);
                                        phi_12694_ = true;
                                    }
                                    let _e7469 = phi_12693_;
                                    let _e7471 = phi_12694_;
                                    if _e7471 {
                                        let _e7473 = (_e7469.x - _e7372);
                                        let _e7475 = (_e7469.y - _e7375);
                                        let _e7477 = (_e7469.z - _e7378);
                                        let _e7483 = sqrt((((_e7473 * _e7473) + (_e7475 * _e7475)) + (_e7477 * _e7477)));
                                        let _e7484 = (1f / _e7483);
                                        let _e7502 = ((((((_e7469.x * _e7469.x) + (_e7469.y * _e7469.y)) + (_e7469.z * _e7469.z)) - (2f * (((_e7469.x * _e7372) + (_e7469.y * _e7375)) + (_e7469.z * _e7378)))) + _e7387) / (_e7483 + _e7381));
                                        if (_e7502 != _e7502) {
                                            phi_32038_ = true;
                                        } else {
                                            phi_32038_ = (0f >= _e7502);
                                        }
                                        let _e7506 = phi_32038_;
                                        let _e7510 = sqrt((select(_e7502, 0f, _e7506) / (_e7384 - _e7381)));
                                        if (_e7510 != _e7510) {
                                            phi_32053_ = true;
                                        } else {
                                            phi_32053_ = (1f <= _e7510);
                                        }
                                        let _e7514 = phi_32053_;
                                        let _e7516 = (select(_e7510, 1f, _e7514) * 63f);
                                        let _e7521 = ((((_e7473 * _e7484) * _e7393) + ((_e7475 * _e7484) * _e7396)) + ((_e7477 * _e7484) * _e7399));
                                        let _e7523 = select(_e7521, -1f, (_e7521 < -1f));
                                        let _e7527 = ((select(_e7523, 1f, (_e7523 > 1f)) + 1f) * 127.5f);
                                        let _e7532 = select(select(u32(_e7516), 0u, (_e7516 < 0f)), 4294967295u, (_e7516 > 4294967000f));
                                        let _e7534 = select(_e7532, 62u, (62u < _e7532));
                                        let _e7539 = select(select(u32(_e7527), 0u, (_e7527 < 0f)), 4294967295u, (_e7527 > 4294967000f));
                                        let _e7541 = select(_e7539, 254u, (254u < _e7539));
                                        let _e7543 = (_e7516 - f32(_e7534));
                                        let _e7545 = (_e7527 - f32(_e7541));
                                        let _e7549 = (_e7424 + (3u * ((_e7534 * 256u) + _e7541)));
                                        let _e7554 = (_e7424 + (3u * (((_e7534 + 1u) * 256u) + _e7541)));
                                        let _e7556 = global_6.member[_e7549];
                                        let _e7559 = global_6.member[(_e7549 + 1u)];
                                        let _e7562 = global_6.member[(_e7549 + 2u)];
                                        let _e7563 = (1f - _e7545);
                                        let _e7569 = global_6.member[(_e7549 + 3u)];
                                        let _e7572 = global_6.member[(_e7549 + 4u)];
                                        let _e7575 = global_6.member[(_e7549 + 5u)];
                                        let _e7582 = (1f - _e7543);
                                        let _e7587 = global_6.member[_e7554];
                                        let _e7590 = global_6.member[(_e7554 + 1u)];
                                        let _e7593 = global_6.member[(_e7554 + 2u)];
                                        let _e7599 = global_6.member[(_e7554 + 3u)];
                                        let _e7602 = global_6.member[(_e7554 + 4u)];
                                        let _e7605 = global_6.member[(_e7554 + 5u)];
                                        phi_12985_ = vec3<f32>(((((_e7556 * _e7563) + (_e7569 * _e7545)) * _e7582) + (((_e7587 * _e7563) + (_e7599 * _e7545)) * _e7543)), ((((_e7559 * _e7563) + (_e7572 * _e7545)) * _e7582) + (((_e7590 * _e7563) + (_e7602 * _e7545)) * _e7543)), ((((_e7562 * _e7563) + (_e7575 * _e7545)) * _e7582) + (((_e7593 * _e7563) + (_e7605 * _e7545)) * _e7543)));
                                    } else {
                                        phi_12985_ = vec3<f32>(0f, 0f, 0f);
                                    }
                                    let _e7620 = phi_12985_;
                                    let _e7627 = ((_e7411 * 1.1f) * _e7620.y);
                                    phi_12987_ = vec3<f32>(exp(-((((_e7402 * _e7620.x) + _e7627) + (_e7414 * _e7620.z)))), exp(-((((_e7405 * _e7620.x) + _e7627) + (_e7417 * _e7620.z)))), exp(-((((_e7408 * _e7620.x) + _e7627) + (_e7420 * _e7620.z)))));
                                }
                                let _e7646 = phi_12987_;
                                let _e7658 = sin((_e7352 * 0.5f));
                                let _e7661 = (((1f - (0.6f * (1f - sqrt(select(_e7356, 0f, _e7360))))) * 1.25f) / ((12.566371f * _e7658) * _e7658));
                                phi_13014_ = vec3<f32>(((_e7363.x * _e7646.x) * _e7661), ((_e7363.y * _e7646.y) * _e7661), ((_e7363.z * _e7646.z) * _e7661));
                            }
                            let _e7667 = phi_13014_;
                            phi_13028_ = vec3<f32>((_e318.member_3.x + ((_e318.member_2.x * _e7667.x) * _e7326)), (_e318.member_3.y + ((_e318.member_2.y * _e7667.y) * _e7326)), (_e318.member_3.z + ((_e318.member_2.z * _e7667.z) * _e7326)));
                        }
                        let _e7682 = phi_13028_;
                        phi_19517_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e318.member_2.x, _e318.member_2.y, _e318.member_2.z, f32(_e345)), vec4<f32>(_e7682.x, _e7682.y, _e7682.z, f32(_e1167)), vec4<u32>(_e318.member_4.x, (_e318.member_4.y & 4294967294u), 0u, 0u)), type_39(vec4<f32>(_e318.member.x, _e318.member.y, _e318.member.z, -1f), vec4<f32>(_e318.member_1.x, _e318.member_1.y, _e318.member_1.z, 0f), vec4<f32>((_e318.member_2.x * _e7272), (_e318.member_2.y * _e7272), (_e318.member_2.z * _e7272), 0f)), type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_19518_ = type_71();
                        phi_19519_ = true;
                    } else {
                        if ((_e364 & 8u) == 0u) {
                            phi_35220_ = _e1007;
                            phi_13565_ = _e339;
                            phi_13566_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        } else {
                            if (_e1166.member > 100f) {
                                let _e1180 = global_1.member.member_8[1u];
                                let _e1182 = global_6.member[_e1180];
                                let _e1185 = global_6.member[(_e1180 + 1u)];
                                let _e1188 = global_6.member[(_e1180 + 2u)];
                                let _e1191 = global_6.member[(_e1180 + 3u)];
                                let _e1194 = global_6.member[(_e1180 + 5u)];
                                let _e1197 = global_6.member[(_e1180 + 6u)];
                                let _e1200 = global_6.member[(_e1180 + 10u)];
                                let _e1203 = global_6.member[(_e1180 + 11u)];
                                let _e1206 = global_6.member[(_e1180 + 13u)];
                                let _e1209 = global_6.member[(_e1180 + 14u)];
                                let _e1212 = global_6.member[(_e1180 + 15u)];
                                let _e1215 = global_6.member[(_e1180 + 16u)];
                                let _e1218 = global_6.member[(_e1180 + 18u)];
                                let _e1221 = global_6.member[(_e1180 + 19u)];
                                let _e1224 = global_6.member[(_e1180 + 20u)];
                                let _e1232 = ((((_e318.member.x - _e1182) * _e318.member_1.x) + ((_e318.member.y - _e1185) * _e318.member_1.y)) + ((_e318.member.z - _e1188) * _e318.member_1.z));
                                let _e1244 = ((((_e318.member.x * _e318.member.x) + (_e318.member.y * _e318.member.y)) + (_e318.member.z * _e318.member.z)) - (2f * (((_e318.member.x * _e1182) + (_e318.member.y * _e1185)) + (_e318.member.z * _e1188))));
                                let _e1245 = (_e1244 + _e1197);
                                let _e1246 = (_e1232 * _e1232);
                                let _e1247 = (_e1246 - _e1245);
                                if (_e1247 < 0f) {
                                    phi_25613_ = type_54(0u, type_54().member_1);
                                    phi_25614_ = type_54();
                                    phi_25615_ = true;
                                } else {
                                    if (_e1232 < 0f) {
                                        phi_25585_ = -(sqrt(_e1247));
                                    } else {
                                        phi_25585_ = sqrt(_e1247);
                                    }
                                    let _e1254 = phi_25585_;
                                    let _e1256 = -((_e1232 + _e1254));
                                    let _e1257 = (_e1256 == 0f);
                                    if _e1257 {
                                        phi_25608_ = type_54(1u, type_53(0f, 0f));
                                        phi_25609_ = type_54();
                                    } else {
                                        let _e1258 = (_e1245 / _e1256);
                                        if (_e1256 < _e1258) {
                                            phi_25597_ = type_53(_e1256, _e1258);
                                        } else {
                                            phi_25597_ = type_53(_e1258, _e1256);
                                        }
                                        let _e1263 = phi_25597_;
                                        phi_25608_ = type_54();
                                        phi_25609_ = type_54(1u, _e1263);
                                    }
                                    let _e1266 = phi_25608_;
                                    let _e1268 = phi_25609_;
                                    phi_25613_ = _e1266;
                                    phi_25614_ = _e1268;
                                    phi_25615_ = _e1257;
                                }
                                let _e1273 = phi_25613_;
                                let _e1275 = phi_25614_;
                                let _e1277 = phi_25615_;
                                if _e1277 {
                                    phi_25619_ = _e1273;
                                } else {
                                    phi_25619_ = _e1275;
                                }
                                let _e1279 = phi_25619_;
                                if (bitcast<i32>(_e1279.member) != 0i) {
                                    phi_25506_ = type_54(0u, _e1279.member_1);
                                } else {
                                    phi_25506_ = type_54(1u, type_54().member_1);
                                }
                                let _e1289 = phi_25506_;
                                if (bitcast<i32>(_e1289.member) != 0i) {
                                    phi_25553_ = type_54(0u, type_54().member_1);
                                } else {
                                    if (_e1289.member_1.member_1 <= 0f) {
                                        phi_25549_ = type_54(0u, type_54().member_1);
                                    } else {
                                        if (_e1289.member_1.member != _e1289.member_1.member) {
                                            phi_25657_ = true;
                                        } else {
                                            phi_25657_ = (0f >= _e1289.member_1.member);
                                        }
                                        let _e1301 = phi_25657_;
                                        let _e1302 = select(_e1289.member_1.member, 0f, _e1301);
                                        let _e1303 = (_e1244 + _e1194);
                                        if (_e1303 > 0f) {
                                            let _e1305 = (_e1246 - _e1303);
                                            if (_e1305 < 0f) {
                                                phi_25765_ = type_54(0u, type_54().member_1);
                                                phi_25766_ = type_54();
                                                phi_25767_ = true;
                                            } else {
                                                if (_e1232 < 0f) {
                                                    phi_25737_ = -(sqrt(_e1305));
                                                } else {
                                                    phi_25737_ = sqrt(_e1305);
                                                }
                                                let _e1312 = phi_25737_;
                                                let _e1314 = -((_e1232 + _e1312));
                                                let _e1315 = (_e1314 == 0f);
                                                if _e1315 {
                                                    phi_25760_ = type_54(1u, type_53(0f, 0f));
                                                    phi_25761_ = type_54();
                                                } else {
                                                    let _e1316 = (_e1303 / _e1314);
                                                    if (_e1314 < _e1316) {
                                                        phi_25749_ = type_53(_e1314, _e1316);
                                                    } else {
                                                        phi_25749_ = type_53(_e1316, _e1314);
                                                    }
                                                    let _e1321 = phi_25749_;
                                                    phi_25760_ = type_54();
                                                    phi_25761_ = type_54(1u, _e1321);
                                                }
                                                let _e1324 = phi_25760_;
                                                let _e1326 = phi_25761_;
                                                phi_25765_ = _e1324;
                                                phi_25766_ = _e1326;
                                                phi_25767_ = _e1315;
                                            }
                                            let _e1331 = phi_25765_;
                                            let _e1333 = phi_25766_;
                                            let _e1335 = phi_25767_;
                                            if _e1335 {
                                                phi_25771_ = _e1331;
                                            } else {
                                                phi_25771_ = _e1333;
                                            }
                                            let _e1337 = phi_25771_;
                                            if (bitcast<i32>(_e1337.member) != 0i) {
                                                phi_25705_ = select(_e1337.member_1.member_1, _e1337.member_1.member, (_e1337.member_1.member >= 0f));
                                            } else {
                                                phi_25705_ = -1f;
                                            }
                                            let _e1348 = phi_25705_;
                                            if (_e1348 >= 0f) {
                                                if (_e1289.member_1.member_1 != _e1289.member_1.member_1) {
                                                    phi_25809_ = true;
                                                } else {
                                                    phi_25809_ = (_e1348 <= _e1289.member_1.member_1);
                                                }
                                                let _e1353 = phi_25809_;
                                                phi_25534_ = select(_e1289.member_1.member_1, _e1348, _e1353);
                                            } else {
                                                phi_25534_ = _e1289.member_1.member_1;
                                            }
                                            let _e1356 = phi_25534_;
                                            phi_25536_ = _e1356;
                                        } else {
                                            phi_25536_ = _e1289.member_1.member_1;
                                        }
                                        let _e1358 = phi_25536_;
                                        if (_e1358 <= _e1302) {
                                            phi_25545_ = type_54(0u, type_54().member_1);
                                        } else {
                                            phi_25545_ = type_54(1u, type_53(_e1302, _e1358));
                                        }
                                        let _e1366 = phi_25545_;
                                        phi_25549_ = _e1366;
                                    }
                                    let _e1371 = phi_25549_;
                                    phi_25553_ = _e1371;
                                }
                                let _e1376 = phi_25553_;
                                if (bitcast<i32>(_e1376.member) != 0i) {
                                    let _e1382 = (_e1376.member_1.member < _e1166.member);
                                    if _e1382 {
                                        if (_e1376.member_1.member_1 != _e1376.member_1.member_1) {
                                            phi_25824_ = true;
                                        } else {
                                            phi_25824_ = (_e1166.member <= _e1376.member_1.member_1);
                                        }
                                        let _e1388 = phi_25824_;
                                        let _e1389 = select(_e1376.member_1.member_1, _e1166.member, _e1388);
                                        phi_13424_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13427_ = 0u;
                                        loop {
                                            let _e1391 = phi_13424_;
                                            let _e1393 = phi_13427_;
                                            local_76 = _e1391;
                                            local_77 = _e1391;
                                            local_78 = _e1391;
                                            let _e1394 = (_e1393 < 32u);
                                            if _e1394 {
                                                let _e1395 = -(_e1232);
                                                if (_e1376.member_1.member <= _e1389) {
                                                } else {
                                                    phi_32956_ = true;
                                                    break;
                                                }
                                                let _e1398 = select(_e1395, _e1376.member_1.member, (_e1395 < _e1376.member_1.member));
                                                let _e1400 = select(_e1398, _e1389, (_e1398 > _e1389));
                                                let _e1401 = (_e1400 - _e1376.member_1.member);
                                                let _e1402 = (_e1389 - _e1400);
                                                let _e1403 = (_e1389 - _e1376.member_1.member);
                                                let _e1404 = (0.000000001f * _e1403);
                                                if (_e1401 <= _e1404) {
                                                    let _e1479 = (f32(_e1393) * 0.03125f);
                                                    let _e1482 = (f32((_e1393 + 1u)) * 0.03125f);
                                                    let _e1484 = ((_e1479 + _e1482) * 0.5f);
                                                    phi_13528_ = type_53((_e1376.member_1.member + ((_e1403 * _e1484) * _e1484)), (_e1403 * ((_e1482 * _e1482) - (_e1479 * _e1479))));
                                                } else {
                                                    if (_e1402 <= _e1404) {
                                                        let _e1462 = (f32((31u - _e1393)) * 0.03125f);
                                                        let _e1465 = (f32((32u - _e1393)) * 0.03125f);
                                                        let _e1467 = ((_e1462 + _e1465) * 0.5f);
                                                        phi_13527_ = type_53((_e1389 - ((_e1403 * _e1467) * _e1467)), (_e1403 * ((_e1465 * _e1465) - (_e1462 * _e1462))));
                                                    } else {
                                                        let _e1410 = round(((32f * _e1401) / (_e1401 + _e1402)));
                                                        let _e1415 = select(select(u32(_e1410), 0u, (_e1410 < 0f)), 4294967295u, (_e1410 > 4294967000f));
                                                        if (_e1415 < 1u) {
                                                            phi_13511_ = 1u;
                                                        } else {
                                                            phi_13511_ = select(_e1415, 31u, (_e1415 > 31u));
                                                        }
                                                        let _e1420 = phi_13511_;
                                                        if (_e1393 < _e1420) {
                                                            let _e1441 = ((_e1420 - 1u) - _e1393);
                                                            let _e1443 = f32(_e1420);
                                                            let _e1444 = (f32(_e1441) / _e1443);
                                                            let _e1447 = (f32((_e1441 + 1u)) / _e1443);
                                                            let _e1449 = ((_e1444 + _e1447) * 0.5f);
                                                            phi_13526_ = type_53((_e1400 - ((_e1401 * _e1449) * _e1449)), (_e1401 * ((_e1447 * _e1447) - (_e1444 * _e1444))));
                                                        } else {
                                                            let _e1423 = (_e1393 - _e1420);
                                                            let _e1425 = f32((32u - _e1420));
                                                            let _e1426 = (f32(_e1423) / _e1425);
                                                            let _e1429 = (f32((_e1423 + 1u)) / _e1425);
                                                            let _e1431 = ((_e1426 + _e1429) * 0.5f);
                                                            phi_13526_ = type_53((_e1400 + ((_e1402 * _e1431) * _e1431)), (_e1402 * ((_e1429 * _e1429) - (_e1426 * _e1426))));
                                                        }
                                                        let _e1459 = phi_13526_;
                                                        phi_13527_ = _e1459;
                                                    }
                                                    let _e1477 = phi_13527_;
                                                    phi_13528_ = _e1477;
                                                }
                                                let _e1494 = phi_13528_;
                                                let _e1500 = (_e318.member.x + (_e318.member_1.x * _e1494.member));
                                                let _e1501 = (_e318.member.y + (_e318.member_1.y * _e1494.member));
                                                let _e1502 = (_e318.member.z + (_e318.member_1.z * _e1494.member));
                                                let _e1516 = (_e1500 - _e1182);
                                                let _e1517 = (_e1501 - _e1185);
                                                let _e1518 = (_e1502 - _e1188);
                                                let _e1526 = ((((((_e1500 * _e1500) + (_e1501 * _e1501)) + (_e1502 * _e1502)) - (2f * (((_e1500 * _e1182) + (_e1501 * _e1185)) + (_e1502 * _e1188)))) + _e1194) / (sqrt((((_e1516 * _e1516) + (_e1517 * _e1517)) + (_e1518 * _e1518))) + _e1191));
                                                let _e1530 = (1f - (abs((_e1526 - 25000f)) * 0.00006666667f));
                                                if (_e1530 != _e1530) {
                                                    phi_26023_ = true;
                                                } else {
                                                    phi_26023_ = (0f >= _e1530);
                                                }
                                                let _e1534 = phi_26023_;
                                                let _e1536 = -(_e1526);
                                                phi_13425_ = (_e1391 + vec3<f32>((exp((_e1536 / _e1200)) * _e1494.member_1), (exp((_e1536 / _e1203)) * _e1494.member_1), (select(_e1530, 0f, _e1534) * _e1494.member_1)));
                                                phi_13428_ = (_e1393 + 1u);
                                            } else {
                                                phi_13425_ = vec3<f32>();
                                                phi_13428_ = u32();
                                            }
                                            let _e1548 = phi_13425_;
                                            let _e1550 = phi_13428_;
                                            phi_13424_ = _e1548;
                                            phi_13427_ = _e1550;
                                            phi_32956_ = _e1007;
                                            if !(_e1394) {
                                                break;
                                            }
                                        }
                                        let _e1553 = phi_32956_;
                                        if _e1553 {
                                            break;
                                        }
                                        let _e1555 = local_76;
                                        let _e1562 = local_77;
                                        let _e1564 = ((_e1215 * 1.1f) * _e1562.y);
                                        let _e1569 = local_78;
                                        phi_35224_ = _e1553;
                                        phi_13553_ = vec3<f32>(exp(-((((_e1206 * _e1555.x) + _e1564) + (_e1218 * _e1569.z)))), exp(-((((_e1209 * _e1555.x) + _e1564) + (_e1221 * _e1569.z)))), exp(-((((_e1212 * _e1555.x) + _e1564) + (_e1224 * _e1569.z)))));
                                    } else {
                                        phi_35224_ = _e1007;
                                        phi_13553_ = vec3<f32>();
                                    }
                                    let _e1585 = phi_35224_;
                                    let _e1587 = phi_13553_;
                                    phi_35223_ = _e1585;
                                    phi_13555_ = _e1587;
                                    phi_13556_ = select(true, false, _e1382);
                                } else {
                                    phi_35223_ = _e1007;
                                    phi_13555_ = vec3<f32>();
                                    phi_13556_ = true;
                                }
                                let _e1590 = phi_35223_;
                                let _e1592 = phi_13555_;
                                let _e1594 = phi_13556_;
                                phi_35221_ = _e1590;
                                phi_13563_ = (_e339 * select(_e1592, vec3<f32>(1f, 1f, 1f), vec3(_e1594)));
                                phi_13564_ = type_39(vec4<f32>(_e318.member.x, _e318.member.y, _e318.member.z, _e1166.member), vec4<f32>(_e318.member_1.x, _e318.member_1.y, _e318.member_1.z, 0f), vec4<f32>(_e318.member_2.x, _e318.member_2.y, _e318.member_2.z, 0f));
                            } else {
                                phi_35221_ = _e1007;
                                phi_13563_ = _e339;
                                phi_13564_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            }
                            let _e1599 = phi_35221_;
                            let _e1601 = phi_13563_;
                            let _e1603 = phi_13564_;
                            phi_35220_ = _e1599;
                            phi_13565_ = _e1601;
                            phi_13566_ = _e1603;
                        }
                        let _e1605 = phi_35220_;
                        let _e1607 = phi_13565_;
                        let _e1609 = phi_13566_;
                        let _e1614 = (_e318.member.x + (_e318.member_1.x * _e1166.member));
                        let _e1615 = (_e318.member.y + (_e318.member_1.y * _e1166.member));
                        let _e1616 = (_e318.member.z + (_e318.member_1.z * _e1166.member));
                        let _e1618 = global_2.member[_e1166.member_1];
                        phi_13590_ = 0i;
                        phi_13593_ = _e1618.member_6.y;
                        loop {
                            let _e1622 = phi_13590_;
                            let _e1624 = phi_13593_;
                            local_79 = _e1624;
                            if (_e1622 < 4i) {
                                let _e1627 = global_4.member[_e1624];
                                let _e1629 = (_e1627.member != 6u);
                                if _e1629 {
                                    phi_13662_ = i32();
                                    phi_13663_ = u32();
                                } else {
                                    let _e1633 = (abs(_e1614) / _e1627.member_6.z);
                                    let _e1644 = (abs(_e1616) / _e1627.member_6.w);
                                    if ((select(select(u32(_e1633), 0u, (_e1633 < 0f)), 4294967295u, (_e1633 > 4294967000f)) % 2u) == 0u) {
                                        phi_13647_ = (_e1614 > 0f);
                                    } else {
                                        phi_13647_ = (_e1614 <= 0f);
                                    }
                                    let _e1655 = phi_13647_;
                                    if ((select(select(u32(_e1644), 0u, (_e1644 < 0f)), 4294967295u, (_e1644 > 4294967000f)) % 2u) == 0u) {
                                        phi_13653_ = (_e1616 > 0f);
                                    } else {
                                        phi_13653_ = (_e1616 <= 0f);
                                    }
                                    let _e1659 = phi_13653_;
                                    if (_e1655 != _e1659) {
                                        phi_13660_ = _e1627.member_1;
                                    } else {
                                        phi_13660_ = _e1627.member_2;
                                    }
                                    let _e1664 = phi_13660_;
                                    phi_13662_ = (_e1622 + 1i);
                                    phi_13663_ = _e1664;
                                }
                                let _e1667 = phi_13662_;
                                let _e1669 = phi_13663_;
                                phi_13591_ = _e1667;
                                phi_13594_ = _e1669;
                                phi_13665_ = select(true, false, _e1629);
                            } else {
                                phi_13591_ = i32();
                                phi_13594_ = u32();
                                phi_13665_ = false;
                            }
                            let _e1672 = phi_13591_;
                            let _e1674 = phi_13594_;
                            let _e1676 = phi_13665_;
                            phi_13590_ = _e1672;
                            phi_13593_ = _e1674;
                            if !(_e1676) {
                                break;
                            }
                        }
                        let _e1679 = local_79;
                        let _e1681 = global_4.member[_e1679];
                        let _e1683 = (_e1681.member == 4u);
                        if _e1683 {
                            if _e1683 {
                                if (_e1681.member_1 == 1u) {
                                    phi_13725_ = true;
                                } else {
                                    phi_13725_ = ((((-(_e318.member_1.x) * _e1166.member_3.x) + (-(_e318.member_1.y) * _e1166.member_3.y)) + (-(_e318.member_1.z) * _e1166.member_3.z)) > 0f);
                                }
                                let _e6988 = phi_13725_;
                                if _e6988 {
                                    phi_13734_ = vec3<f32>(_e1681.member_4.x, _e1681.member_4.y, _e1681.member_4.z);
                                } else {
                                    phi_13734_ = vec3<f32>();
                                }
                                let _e6995 = phi_13734_;
                                phi_13736_ = _e6995;
                                phi_13737_ = select(true, false, _e6988);
                            } else {
                                phi_13736_ = vec3<f32>();
                                phi_13737_ = true;
                            }
                            let _e6998 = phi_13736_;
                            let _e7000 = phi_13737_;
                            let _e7002 = select(_e6998, vec3<f32>(0f, 0f, 0f), vec3(_e7000));
                            if _e358 {
                                phi_13907_ = 1f;
                            } else {
                                if ((_e364 & 1u) == 0u) {
                                    phi_13906_ = 1f;
                                } else {
                                    if (_e1618.member_6.z > 0u) {
                                        let _e7011 = global_1.member.member_2[1u];
                                        let _e7012 = (_e7011 + (_e1618.member_6.z - 1u));
                                        let _e7015 = global_6.member[(_e7012 + 1u)];
                                        let _e7017 = global_6.member[_e7012];
                                        if (_e1618.member_6.x == 1u) {
                                            let _e7094 = (_e1618.member.x - _e318.member.x);
                                            let _e7095 = (_e1618.member.y - _e318.member.y);
                                            let _e7096 = (_e1618.member.z - _e318.member.z);
                                            let _e7101 = (((_e7094 * _e7094) + (_e7095 * _e7095)) + (_e7096 * _e7096));
                                            let _e7102 = (_e1618.member.w * _e1618.member.w);
                                            if (_e7101 <= (_e7102 * 1.000001f)) {
                                                let _e7116 = (_e1614 - _e318.member.x);
                                                let _e7117 = (_e1615 - _e318.member.y);
                                                let _e7118 = (_e1616 - _e318.member.z);
                                                let _e7123 = (((_e7116 * _e7116) + (_e7117 * _e7117)) + (_e7118 * _e7118));
                                                phi_31751_ = (_e7123 / (((abs((((_e1166.member_2.x * _e7116) + (_e1166.member_2.y * _e7117)) + (_e1166.member_2.z * _e7118))) / sqrt(_e7123)) * 12.566371f) * _e7102));
                                            } else {
                                                let _e7105 = (_e7102 / _e7101);
                                                if (_e7105 != _e7105) {
                                                    phi_31770_ = true;
                                                } else {
                                                    phi_31770_ = (1f <= _e7105);
                                                }
                                                let _e7109 = phi_31770_;
                                                let _e7110 = select(_e7105, 1f, _e7109);
                                                phi_31751_ = (0.15915494f / (_e7110 / (1f + sqrt((1f - _e7110)))));
                                            }
                                            let _e7139 = phi_31751_;
                                            phi_13902_ = _e7139;
                                        } else {
                                            let _e7023 = (_e1614 - _e318.member.x);
                                            let _e7024 = (_e1615 - _e318.member.y);
                                            let _e7025 = (_e1616 - _e318.member.z);
                                            let _e7030 = (((_e7023 * _e7023) + (_e7024 * _e7024)) + (_e7025 * _e7025));
                                            let _e7041 = (abs((((_e1166.member_2.x * _e7023) + (_e1166.member_2.y * _e7024)) + (_e1166.member_2.z * _e7025))) / sqrt(_e7030));
                                            if (_e7041 <= 0f) {
                                                phi_13901_ = 0f;
                                            } else {
                                                if (_e1618.member_6.x == 0u) {
                                                    let _e7056 = (_e1618.member_1.x - _e1618.member.x);
                                                    let _e7057 = (_e1618.member_1.y - _e1618.member.y);
                                                    let _e7058 = (_e1618.member_1.z - _e1618.member.z);
                                                    let _e7063 = (_e1618.member_2.x - _e1618.member.x);
                                                    let _e7064 = (_e1618.member_2.y - _e1618.member.y);
                                                    let _e7065 = (_e1618.member_2.z - _e1618.member.z);
                                                    let _e7068 = ((_e7057 * _e7065) - (_e7064 * _e7058));
                                                    let _e7071 = ((_e7058 * _e7063) - (_e7065 * _e7056));
                                                    let _e7074 = ((_e7056 * _e7064) - (_e7063 * _e7057));
                                                    phi_13898_ = (sqrt((((_e7068 * _e7068) + (_e7071 * _e7071)) + (_e7074 * _e7074))) * 0.5f);
                                                } else {
                                                    phi_13898_ = ((12.566371f * _e1618.member.w) * _e1618.member.w);
                                                }
                                                let _e7083 = phi_13898_;
                                                phi_13901_ = (_e7030 / (_e7041 * _e7083));
                                            }
                                            let _e7087 = phi_13901_;
                                            phi_13902_ = _e7087;
                                        }
                                        let _e7141 = phi_13902_;
                                        let _e7142 = ((_e7015 - _e7017) * _e7141);
                                        if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                            phi_31791_ = 1f;
                                        } else {
                                            let _e7144 = (_e318.member_1.w * _e318.member_1.w);
                                            let _e7146 = (_e7144 + (_e7142 * _e7142));
                                            if (_e7146 == 0f) {
                                                phi_31788_ = 0f;
                                            } else {
                                                phi_31788_ = (_e7144 / _e7146);
                                            }
                                            let _e7150 = phi_31788_;
                                            phi_31791_ = _e7150;
                                        }
                                        let _e7152 = phi_31791_;
                                        phi_13905_ = _e7152;
                                    } else {
                                        phi_13905_ = 1f;
                                    }
                                    let _e7154 = phi_13905_;
                                    phi_13906_ = _e7154;
                                }
                                let _e7156 = phi_13906_;
                                phi_13907_ = _e7156;
                            }
                            let _e7158 = phi_13907_;
                            phi_19511_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e1607.x, _e1607.y, _e1607.z, f32(_e345)), vec4<f32>((_e318.member_3.x + ((_e1607.x * _e7002.x) * _e7158)), (_e318.member_3.y + ((_e1607.y * _e7002.y) * _e7158)), (_e318.member_3.z + ((_e1607.z * _e7002.z) * _e7158)), f32(_e1167)), vec4<u32>(_e318.member_4.x, (_e318.member_4.y & 4294967294u), 0u, 0u)), _e1609, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_19512_ = type_71();
                            phi_19513_ = true;
                        } else {
                            let _e1684 = -(_e318.member_1.x);
                            let _e1685 = -(_e318.member_1.y);
                            let _e1686 = -(_e318.member_1.z);
                            if ((((_e1166.member_2.x * _e1684) + (_e1166.member_2.y * _e1685)) + (_e1166.member_2.z * _e1686)) < 0f) {
                                phi_13946_ = -(_e1166.member_2);
                            } else {
                                phi_13946_ = _e1166.member_2;
                            }
                            let _e1699 = phi_13946_;
                            if ((((_e1166.member_3.x * _e1684) + (_e1166.member_3.y * _e1685)) + (_e1166.member_3.z * _e1686)) < 0f) {
                                phi_13963_ = -(_e1166.member_3);
                            } else {
                                phi_13963_ = _e1166.member_3;
                            }
                            let _e1712 = phi_13963_;
                            let _e1715 = global_1.member.member_4[2u];
                            let _e1716 = abs(_e1614);
                            let _e1717 = abs(_e1615);
                            let _e1718 = abs(_e1616);
                            let _e1720 = select(_e1717, _e1716, (_e1716 > _e1717));
                            let _e1724 = (0.00002f * (1f + select(_e1718, _e1720, (_e1720 > _e1718))));
                            if (_e1715 != _e1715) {
                                phi_26090_ = true;
                            } else {
                                phi_26090_ = (_e1724 >= _e1715);
                            }
                            let _e1728 = phi_26090_;
                            let _e1730 = (select(_e1715, _e1724, _e1728) * 2f);
                            let _e1737 = (_e1614 + (_e1699.x * _e1730));
                            let _e1738 = (_e1615 + (_e1699.y * _e1730));
                            let _e1739 = (_e1616 + (_e1699.z * _e1730));
                            let _e1740 = vec3<f32>(_e1737, _e1738, _e1739);
                            if ((_e318.member_4.y & 4u) == 0u) {
                                phi_13996_ = 0f;
                            } else {
                                let _e1745 = global_1.member.member_4[0u];
                                phi_13996_ = _e1745;
                            }
                            let _e1747 = phi_13996_;
                            let _e1750 = select(-1f, 1f, (_e1712.z >= 0f));
                            let _e1752 = (-1f / (_e1750 + _e1712.z));
                            let _e1756 = ((_e1712.x * _e1712.y) * _e1752);
                            let _e1760 = (1f + (((_e1750 * _e1712.x) * _e1712.x) * _e1752));
                            let _e1761 = (_e1750 * _e1756);
                            let _e1763 = (-(_e1750) * _e1712.x);
                            let _e1766 = (_e1750 + ((_e1712.y * _e1712.y) * _e1752));
                            let _e1767 = -(_e1712.y);
                            if (_e1681.member == 0u) {
                                phi_33871_ = vec3<f32>(0f, 0f, 0f);
                                phi_33679_ = 1.5f;
                                phi_33600_ = vec3<f32>(0f, 0f, 0f);
                                phi_33523_ = vec3<f32>(_e1681.member_4.x, _e1681.member_4.y, _e1681.member_4.z);
                                phi_33444_ = 0u;
                                phi_33362_ = 1f;
                                phi_33285_ = 0u;
                                phi_33276_ = _e318.member_4.x;
                                phi_14675_ = type_71();
                                phi_14676_ = true;
                            } else {
                                if (_e1681.member == 1u) {
                                    let _e2316 = (_e1681.member_6.y * _e1681.member_6.y);
                                    if (_e2316 != _e2316) {
                                        phi_26782_ = true;
                                    } else {
                                        phi_26782_ = (_e1747 >= _e2316);
                                    }
                                    let _e2320 = phi_26782_;
                                    let _e2321 = select(_e2316, _e1747, _e2320);
                                    let _e2322 = (_e2321 >= 0.001f);
                                    if _e2322 {
                                        phi_33928_ = vec3<f32>(_e1681.member_4.x, _e1681.member_4.y, _e1681.member_4.z);
                                        phi_33722_ = _e1681.member_6.x;
                                        phi_33575_ = vec3<f32>(0f, 0f, 0f);
                                        phi_33263_ = _e318.member_4.x;
                                        phi_14071_ = vec3<f32>();
                                        phi_14072_ = true;
                                    } else {
                                        let _e2324 = ((_e318.member_4.x * 747796405u) + 2891336453u);
                                        let _e2332 = (((_e2324 >> bitcast<u32>((((_e2324 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2324) * 277803737u);
                                        let _e2335 = ((_e2332 >> bitcast<u32>(22i)) ^ _e2332);
                                        let _e2345 = abs((((_e1684 * _e1712.x) + (_e1685 * _e1712.y)) + (_e1686 * _e1712.z)));
                                        let _e2348 = (1f / _e1681.member_6.x);
                                        let _e2352 = ((_e2348 * _e2348) * (1f - (_e2345 * _e2345)));
                                        if (_e2352 >= 1f) {
                                            phi_26822_ = 1f;
                                        } else {
                                            let _e2355 = sqrt((1f - _e2352));
                                            if (_e2345 != _e2345) {
                                                phi_26833_ = true;
                                            } else {
                                                phi_26833_ = (_e2355 <= _e2345);
                                            }
                                            let _e2359 = phi_26833_;
                                            let _e2364 = pow(((1f - _e1681.member_6.x) / (1f + _e1681.member_6.x)), 2f);
                                            phi_26822_ = (_e2364 + ((1f - _e2364) * pow((1f - select(_e2345, _e2355, _e2359)), 5f)));
                                        }
                                        let _e2371 = phi_26822_;
                                        let _e2372 = ((f32((_e2335 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e2371);
                                        if _e2372 {
                                            let _e2385 = (((_e1712.x * _e318.member_1.x) + (_e1712.y * _e318.member_1.y)) + (_e1712.z * _e318.member_1.z));
                                            phi_33576_ = vec3<f32>(0f, 0f, 0f);
                                            phi_14069_ = (_e334 - vec3<f32>(((_e1712.x * 2f) * _e2385), ((_e1712.y * 2f) * _e2385), ((_e1712.z * 2f) * _e2385)));
                                        } else {
                                            phi_33576_ = vec3<f32>(_e1681.member_4.x, _e1681.member_4.y, _e1681.member_4.z);
                                            phi_14069_ = vec3<f32>();
                                        }
                                        let _e2392 = phi_33576_;
                                        let _e2394 = phi_14069_;
                                        phi_33928_ = vec3<f32>(0f, 0f, 0f);
                                        phi_33722_ = 1.5f;
                                        phi_33575_ = _e2392;
                                        phi_33263_ = _e2335;
                                        phi_14071_ = _e2394;
                                        phi_14072_ = select(true, false, _e2372);
                                    }
                                    let _e2404 = phi_33928_;
                                    let _e2406 = phi_33722_;
                                    let _e2408 = phi_33575_;
                                    let _e2410 = phi_33263_;
                                    let _e2412 = phi_14071_;
                                    let _e2414 = phi_14072_;
                                    phi_33874_ = _e2404;
                                    phi_33682_ = _e2406;
                                    phi_33603_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33526_ = _e2408;
                                    phi_33447_ = select(0u, 1u, _e2322);
                                    phi_33365_ = select(1f, _e2321, _e2322);
                                    phi_33288_ = select(0u, 2u, _e2322);
                                    phi_33214_ = _e2410;
                                    phi_14545_ = _e1740;
                                    phi_14546_ = _e2412;
                                    phi_14547_ = vec3<f32>(1f, 1f, 1f);
                                    phi_14548_ = _e2414;
                                } else {
                                    if (_e1681.member == 3u) {
                                        phi_33217_ = _e318.member_4.x;
                                        phi_14224_ = vec3<f32>();
                                        phi_14225_ = vec3<f32>();
                                        phi_14226_ = vec3<f32>();
                                        phi_14227_ = true;
                                        phi_14228_ = false;
                                    } else {
                                        let _e1771 = (_e1681.member == 5u);
                                        if _e1771 {
                                            phi_33218_ = _e318.member_4.x;
                                            phi_14220_ = vec3<f32>();
                                            phi_14221_ = vec3<f32>();
                                            phi_14222_ = vec3<f32>();
                                            phi_14223_ = false;
                                        } else {
                                            let _e1772 = (_e1681.member == 2u);
                                            if _e1772 {
                                                let _e1777 = (((_e318.member_1.x * _e1166.member_3.x) + (_e318.member_1.y * _e1166.member_3.y)) + (_e318.member_1.z * _e1166.member_3.z));
                                                let _e1778 = (_e1777 > 0f);
                                                if _e1778 {
                                                    phi_14105_ = -(_e1166.member_3);
                                                } else {
                                                    phi_14105_ = _e1166.member_3;
                                                }
                                                let _e1781 = phi_14105_;
                                                if _e1778 {
                                                    phi_14111_ = _e1681.member_6.x;
                                                } else {
                                                    phi_14111_ = (1f / _e1681.member_6.x);
                                                }
                                                let _e1786 = phi_14111_;
                                                let _e1787 = abs(_e1777);
                                                let _e1788 = (_e1786 * _e1786);
                                                let _e1791 = (_e1788 * (1f - (_e1787 * _e1787)));
                                                if (_e1791 >= 1f) {
                                                    phi_26152_ = 1f;
                                                } else {
                                                    let _e1794 = sqrt((1f - _e1791));
                                                    if (_e1787 != _e1787) {
                                                        phi_26163_ = true;
                                                    } else {
                                                        phi_26163_ = (_e1794 <= _e1787);
                                                    }
                                                    let _e1798 = phi_26163_;
                                                    let _e1803 = pow(((1f - _e1681.member_6.x) / (1f + _e1681.member_6.x)), 2f);
                                                    phi_26152_ = (_e1803 + ((1f - _e1803) * pow((1f - select(_e1787, _e1794, _e1798)), 5f)));
                                                }
                                                let _e1810 = phi_26152_;
                                                let _e1818 = (((_e318.member_1.x * _e1781.x) + (_e318.member_1.y * _e1781.y)) + (_e318.member_1.z * _e1781.z));
                                                let _e1822 = (1f - (_e1788 * (1f - (_e1818 * _e1818))));
                                                if (_e1822 > 0f) {
                                                    let _e1825 = ((_e318.member_4.x * 747796405u) + 2891336453u);
                                                    let _e1833 = (((_e1825 >> bitcast<u32>((((_e1825 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1825) * 277803737u);
                                                    let _e1836 = ((_e1833 >> bitcast<u32>(22i)) ^ _e1833);
                                                    let _e1841 = ((f32((_e1836 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1810);
                                                    if _e1841 {
                                                        let _e1851 = sqrt(_e1822);
                                                        let _e1855 = (((_e318.member_1.x - (_e1781.x * _e1818)) * _e1786) - (_e1781.x * _e1851));
                                                        let _e1856 = (((_e318.member_1.y - (_e1781.y * _e1818)) * _e1786) - (_e1781.y * _e1851));
                                                        let _e1857 = (((_e318.member_1.z - (_e1781.z * _e1818)) * _e1786) - (_e1781.z * _e1851));
                                                        let _e1864 = (1f / sqrt((((_e1855 * _e1855) + (_e1856 * _e1856)) + (_e1857 * _e1857))));
                                                        phi_14192_ = vec3<f32>((_e1614 - (_e1781.x * _e1730)), (_e1615 - (_e1781.y * _e1730)), (_e1616 - (_e1781.z * _e1730)));
                                                        phi_14193_ = vec3<f32>((_e1855 * _e1864), (_e1856 * _e1864), (_e1857 * _e1864));
                                                        phi_14194_ = vec3<f32>(_e1681.member_4.x, _e1681.member_4.y, _e1681.member_4.z);
                                                    } else {
                                                        phi_14192_ = vec3<f32>();
                                                        phi_14193_ = vec3<f32>();
                                                        phi_14194_ = vec3<f32>();
                                                    }
                                                    let _e1882 = phi_14192_;
                                                    let _e1884 = phi_14193_;
                                                    let _e1886 = phi_14194_;
                                                    phi_33221_ = _e1836;
                                                    phi_14196_ = _e1882;
                                                    phi_14197_ = _e1884;
                                                    phi_14198_ = _e1886;
                                                    phi_14199_ = select(true, false, _e1841);
                                                } else {
                                                    phi_33221_ = _e318.member_4.x;
                                                    phi_14196_ = vec3<f32>();
                                                    phi_14197_ = vec3<f32>();
                                                    phi_14198_ = vec3<f32>();
                                                    phi_14199_ = true;
                                                }
                                                let _e1889 = phi_33221_;
                                                let _e1891 = phi_14196_;
                                                let _e1893 = phi_14197_;
                                                let _e1895 = phi_14198_;
                                                let _e1897 = phi_14199_;
                                                if _e1897 {
                                                    let _e1905 = (((_e1166.member_3.x * _e318.member_1.x) + (_e1166.member_3.y * _e318.member_1.y)) + (_e1166.member_3.z * _e318.member_1.z));
                                                    phi_14214_ = vec3<f32>((_e1614 + (_e1781.x * _e1730)), (_e1615 + (_e1781.y * _e1730)), (_e1616 + (_e1781.z * _e1730)));
                                                    phi_14215_ = (_e334 - vec3<f32>(((_e1166.member_3.x * 2f) * _e1905), ((_e1166.member_3.y * 2f) * _e1905), ((_e1166.member_3.z * 2f) * _e1905)));
                                                } else {
                                                    phi_14214_ = _e1891;
                                                    phi_14215_ = _e1893;
                                                }
                                                let _e1919 = phi_14214_;
                                                let _e1921 = phi_14215_;
                                                phi_33219_ = _e1889;
                                                phi_14217_ = _e1919;
                                                phi_14218_ = _e1921;
                                                phi_14219_ = select(_e1895, vec3<f32>(1f, 1f, 1f), vec3(_e1897));
                                            } else {
                                                phi_33219_ = _e318.member_4.x;
                                                phi_14217_ = vec3<f32>();
                                                phi_14218_ = vec3<f32>();
                                                phi_14219_ = vec3<f32>();
                                            }
                                            let _e1925 = phi_33219_;
                                            let _e1927 = phi_14217_;
                                            let _e1929 = phi_14218_;
                                            let _e1931 = phi_14219_;
                                            phi_33218_ = _e1925;
                                            phi_14220_ = _e1927;
                                            phi_14221_ = _e1929;
                                            phi_14222_ = _e1931;
                                            phi_14223_ = _e1772;
                                        }
                                        let _e1933 = phi_33218_;
                                        let _e1935 = phi_14220_;
                                        let _e1937 = phi_14221_;
                                        let _e1939 = phi_14222_;
                                        let _e1941 = phi_14223_;
                                        phi_33217_ = _e1933;
                                        phi_14224_ = _e1935;
                                        phi_14225_ = _e1937;
                                        phi_14226_ = _e1939;
                                        phi_14227_ = _e1771;
                                        phi_14228_ = _e1941;
                                    }
                                    let _e1943 = phi_33217_;
                                    let _e1945 = phi_14224_;
                                    let _e1947 = phi_14225_;
                                    let _e1949 = phi_14226_;
                                    let _e1951 = phi_14227_;
                                    let _e1953 = phi_14228_;
                                    if _e1951 {
                                        let _e1956 = (_e1681.member_6.y * _e1681.member_6.y);
                                        if (_e1956 != _e1956) {
                                            phi_26236_ = true;
                                        } else {
                                            phi_26236_ = (_e1747 >= _e1956);
                                        }
                                        let _e1960 = phi_26236_;
                                        let _e1961 = select(_e1956, _e1747, _e1960);
                                        let _e1973 = select(2u, 0u, (_e1681.member == 5u));
                                        let _e1974 = (_e1961 < 0.001f);
                                        if _e1974 {
                                            let _e1982 = (((_e1712.x * _e318.member_1.x) + (_e1712.y * _e318.member_1.y)) + (_e1712.z * _e318.member_1.z));
                                            let _e1993 = abs((((_e1684 * _e1712.x) + (_e1685 * _e1712.y)) + (_e1686 * _e1712.z)));
                                            if (_e1973 == 0u) {
                                                let _e2275 = (1f - _e1993);
                                                if (_e2275 != _e2275) {
                                                    phi_26767_ = true;
                                                } else {
                                                    phi_26767_ = (0f >= _e2275);
                                                }
                                                let _e2279 = phi_26767_;
                                                let _e2281 = pow(select(_e2275, 0f, _e2279), 5f);
                                                let _e2282 = (1f - _e2281);
                                                phi_14531_ = vec3<f32>(((_e1681.member_4.x * _e2282) + _e2281), ((_e1681.member_4.y * _e2282) + _e2281), ((_e1681.member_4.z * _e2282) + _e2281));
                                            } else {
                                                if (_e1973 == 1u) {
                                                    let _e2257 = (0.44444448f * (1f - (_e1993 * _e1993)));
                                                    if (_e2257 >= 1f) {
                                                        phi_26728_ = 1f;
                                                    } else {
                                                        let _e2260 = sqrt((1f - _e2257));
                                                        if (_e1993 != _e1993) {
                                                            phi_26739_ = true;
                                                        } else {
                                                            phi_26739_ = (_e2260 <= _e1993);
                                                        }
                                                        let _e2264 = phi_26739_;
                                                        phi_26728_ = (0.040000003f + (0.96f * pow((1f - select(_e1993, _e2260, _e2264)), 5f)));
                                                    }
                                                    let _e2271 = phi_26728_;
                                                    phi_14530_ = vec3(_e2271);
                                                } else {
                                                    let _e1997 = select(_e1993, 0f, (_e1993 < 0f));
                                                    let _e1999 = select(_e1997, 1f, (_e1997 > 1f));
                                                    let _e2001 = (1f - (_e1999 * _e1999));
                                                    let _e2008 = ((_e1681.member_4.x * _e1681.member_4.x) - (_e1681.member_5.x * _e1681.member_5.x));
                                                    let _e2009 = ((_e1681.member_4.y * _e1681.member_4.y) - (_e1681.member_5.y * _e1681.member_5.y));
                                                    let _e2010 = ((_e1681.member_4.z * _e1681.member_4.z) - (_e1681.member_5.z * _e1681.member_5.z));
                                                    let _e2012 = (_e1681.member_4.x * (_e1681.member_5.x + _e1681.member_5.x));
                                                    let _e2014 = (_e1681.member_4.y * (_e1681.member_5.y + _e1681.member_5.y));
                                                    let _e2016 = (_e1681.member_4.z * (_e1681.member_5.z + _e1681.member_5.z));
                                                    let _e2023 = ((_e2008 * _e2008) + (_e2012 * _e2012));
                                                    let _e2024 = ((_e2009 * _e2009) + (_e2014 * _e2014));
                                                    let _e2025 = ((_e2010 * _e2010) + (_e2016 * _e2016));
                                                    let _e2030 = ((_e2001 * _e2008) / _e2023);
                                                    let _e2031 = ((_e2001 * _e2009) / _e2024);
                                                    let _e2032 = ((_e2001 * _e2010) / _e2025);
                                                    let _e2033 = (1f - _e2030);
                                                    let _e2034 = (1f - _e2031);
                                                    let _e2035 = (1f - _e2032);
                                                    let _e2037 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2030, _e2031, _e2032));
                                                    let _e2038 = (_e2001 * _e2012);
                                                    let _e2039 = (_e2001 * _e2014);
                                                    let _e2040 = (_e2001 * _e2016);
                                                    let _e2045 = (vec3<f32>(_e2038, _e2039, _e2040) / vec3<f32>(_e2023, _e2024, _e2025));
                                                    let _e2058 = ((_e2037.x * _e2037.x) + (_e2045.x * _e2045.x));
                                                    let _e2059 = ((_e2037.y * _e2037.y) + (_e2045.y * _e2045.y));
                                                    let _e2060 = ((_e2037.z * _e2037.z) + (_e2045.z * _e2045.z));
                                                    if (_e2058 != _e2058) {
                                                        phi_26398_ = true;
                                                    } else {
                                                        phi_26398_ = (0f >= _e2058);
                                                    }
                                                    let _e2064 = phi_26398_;
                                                    let _e2066 = sqrt(select(_e2058, 0f, _e2064));
                                                    if (_e2059 != _e2059) {
                                                        phi_26413_ = true;
                                                    } else {
                                                        phi_26413_ = (0f >= _e2059);
                                                    }
                                                    let _e2070 = phi_26413_;
                                                    let _e2072 = sqrt(select(_e2059, 0f, _e2070));
                                                    if (_e2060 != _e2060) {
                                                        phi_26428_ = true;
                                                    } else {
                                                        phi_26428_ = (0f >= _e2060);
                                                    }
                                                    let _e2076 = phi_26428_;
                                                    let _e2078 = sqrt(select(_e2060, 0f, _e2076));
                                                    let _e2082 = ((_e2066 + _e2033) * 0.5f);
                                                    let _e2083 = ((_e2072 + _e2034) * 0.5f);
                                                    let _e2084 = ((_e2078 + _e2035) * 0.5f);
                                                    if (_e2082 != _e2082) {
                                                        phi_26455_ = true;
                                                    } else {
                                                        phi_26455_ = (0f >= _e2082);
                                                    }
                                                    let _e2088 = phi_26455_;
                                                    let _e2090 = sqrt(select(_e2082, 0f, _e2088));
                                                    if (_e2083 != _e2083) {
                                                        phi_26470_ = true;
                                                    } else {
                                                        phi_26470_ = (0f >= _e2083);
                                                    }
                                                    let _e2094 = phi_26470_;
                                                    let _e2096 = sqrt(select(_e2083, 0f, _e2094));
                                                    if (_e2084 != _e2084) {
                                                        phi_26485_ = true;
                                                    } else {
                                                        phi_26485_ = (0f >= _e2084);
                                                    }
                                                    let _e2100 = phi_26485_;
                                                    let _e2102 = sqrt(select(_e2084, 0f, _e2100));
                                                    let _e2103 = vec3<f32>(_e2090, _e2096, _e2102);
                                                    let _e2107 = ((_e2066 - _e2033) * 0.5f);
                                                    let _e2108 = ((_e2072 - _e2034) * 0.5f);
                                                    let _e2109 = ((_e2078 - _e2035) * 0.5f);
                                                    if (_e2107 != _e2107) {
                                                        phi_26512_ = true;
                                                    } else {
                                                        phi_26512_ = (0f >= _e2107);
                                                    }
                                                    let _e2113 = phi_26512_;
                                                    let _e2115 = sqrt(select(_e2107, 0f, _e2113));
                                                    if (_e2108 != _e2108) {
                                                        phi_26527_ = true;
                                                    } else {
                                                        phi_26527_ = (0f >= _e2108);
                                                    }
                                                    let _e2119 = phi_26527_;
                                                    let _e2121 = sqrt(select(_e2108, 0f, _e2119));
                                                    if (_e2109 != _e2109) {
                                                        phi_26542_ = true;
                                                    } else {
                                                        phi_26542_ = (0f >= _e2109);
                                                    }
                                                    let _e2125 = phi_26542_;
                                                    let _e2127 = sqrt(select(_e2109, 0f, _e2125));
                                                    if ((_e2038 / _e2023) < 0f) {
                                                        phi_14407_ = -(_e2115);
                                                    } else {
                                                        phi_14407_ = _e2115;
                                                    }
                                                    let _e2131 = phi_14407_;
                                                    if ((_e2039 / _e2024) < 0f) {
                                                        phi_14415_ = -(_e2121);
                                                    } else {
                                                        phi_14415_ = _e2121;
                                                    }
                                                    let _e2135 = phi_14415_;
                                                    if ((_e2040 / _e2025) < 0f) {
                                                        phi_14423_ = -(_e2127);
                                                    } else {
                                                        phi_14423_ = _e2127;
                                                    }
                                                    let _e2139 = phi_14423_;
                                                    let _e2143 = (_e1681.member_5.x * _e1999);
                                                    let _e2144 = (_e1681.member_5.y * _e1999);
                                                    let _e2145 = (_e1681.member_5.z * _e1999);
                                                    let _e2155 = vec3<f32>(((_e1681.member_4.x * _e2090) - (_e1681.member_5.x * _e2131)), ((_e1681.member_4.y * _e2096) - (_e1681.member_5.y * _e2135)), ((_e1681.member_4.z * _e2102) - (_e1681.member_5.z * _e2139)));
                                                    let _e2162 = ((_e1681.member_4.x * _e2131) + (_e1681.member_5.x * _e2090));
                                                    let _e2163 = ((_e1681.member_4.y * _e2135) + (_e1681.member_5.y * _e2096));
                                                    let _e2164 = ((_e1681.member_4.z * _e2139) + (_e1681.member_5.z * _e2102));
                                                    let _e2166 = vec3<f32>((_e1681.member_4.x * _e1999), (_e1681.member_4.y * _e1999), (_e1681.member_4.z * _e1999));
                                                    let _e2167 = (_e2166 - _e2103);
                                                    let _e2168 = (_e2143 - _e2131);
                                                    let _e2169 = (_e2144 - _e2135);
                                                    let _e2170 = (_e2145 - _e2139);
                                                    let _e2183 = (_e2166 + _e2103);
                                                    let _e2184 = (_e2143 + _e2131);
                                                    let _e2185 = (_e2144 + _e2135);
                                                    let _e2186 = (_e2145 + _e2139);
                                                    let _e2196 = ((_e2183.x * _e2183.x) + (_e2184 * _e2184));
                                                    let _e2197 = ((_e2183.y * _e2183.y) + (_e2185 * _e2185));
                                                    let _e2198 = ((_e2183.z * _e2183.z) + (_e2186 * _e2186));
                                                    let _e2208 = vec3(_e1999);
                                                    let _e2209 = (_e2208 - _e2155);
                                                    let _e2210 = -(vec3<f32>(_e2162, _e2163, _e2164));
                                                    let _e2226 = (_e2208 + _e2155);
                                                    let _e2236 = ((_e2226.x * _e2226.x) + (_e2162 * _e2162));
                                                    let _e2237 = ((_e2226.y * _e2226.y) + (_e2163 * _e2163));
                                                    let _e2238 = ((_e2226.z * _e2226.z) + (_e2164 * _e2164));
                                                    phi_14530_ = vec3<f32>((((((_e2167.x * _e2167.x) + (_e2168 * _e2168)) / select(0.000000000000000000000000000001f, _e2196, (_e2196 > 0.000000000000000000000000000001f))) + (((_e2209.x * _e2209.x) + (_e2210.x * _e2210.x)) / select(0.000000000000000000000000000001f, _e2236, (_e2236 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e2167.y * _e2167.y) + (_e2169 * _e2169)) / select(0.000000000000000000000000000001f, _e2197, (_e2197 > 0.000000000000000000000000000001f))) + (((_e2209.y * _e2209.y) + (_e2210.y * _e2210.y)) / select(0.000000000000000000000000000001f, _e2237, (_e2237 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e2167.z * _e2167.z) + (_e2170 * _e2170)) / select(0.000000000000000000000000000001f, _e2198, (_e2198 > 0.000000000000000000000000000001f))) + (((_e2209.z * _e2209.z) + (_e2210.z * _e2210.z)) / select(0.000000000000000000000000000001f, _e2238, (_e2238 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e2274 = phi_14530_;
                                                phi_14531_ = _e2274;
                                            }
                                            let _e2291 = phi_14531_;
                                            phi_14532_ = (_e334 - vec3<f32>(((_e1712.x * 2f) * _e1982), ((_e1712.y * 2f) * _e1982), ((_e1712.z * 2f) * _e1982)));
                                            phi_14533_ = _e2291;
                                        } else {
                                            phi_14532_ = vec3<f32>();
                                            phi_14533_ = vec3<f32>();
                                        }
                                        let _e2293 = phi_14532_;
                                        let _e2295 = phi_14533_;
                                        phi_33605_ = vec3<f32>(_e1681.member_5.x, _e1681.member_5.y, _e1681.member_5.z);
                                        phi_33528_ = vec3<f32>(_e1681.member_4.x, _e1681.member_4.y, _e1681.member_4.z);
                                        phi_33449_ = _e1973;
                                        phi_33367_ = _e1961;
                                        phi_14535_ = _e2293;
                                        phi_14536_ = _e2295;
                                        phi_14537_ = _e1974;
                                    } else {
                                        phi_33605_ = vec3<f32>(0f, 0f, 0f);
                                        phi_33528_ = vec3<f32>(0f, 0f, 0f);
                                        phi_33449_ = 0u;
                                        phi_33367_ = 1f;
                                        phi_14535_ = _e1947;
                                        phi_14536_ = _e1949;
                                        phi_14537_ = _e1953;
                                    }
                                    let _e2297 = phi_33605_;
                                    let _e2299 = phi_33528_;
                                    let _e2301 = phi_33449_;
                                    let _e2303 = phi_33367_;
                                    let _e2305 = phi_14535_;
                                    let _e2307 = phi_14536_;
                                    let _e2309 = phi_14537_;
                                    phi_33874_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33682_ = 1.5f;
                                    phi_33603_ = _e2297;
                                    phi_33526_ = _e2299;
                                    phi_33447_ = _e2301;
                                    phi_33365_ = _e2303;
                                    phi_33288_ = select(0u, 1u, _e1951);
                                    phi_33214_ = _e1943;
                                    phi_14545_ = select(_e1945, _e1740, vec3(_e1951));
                                    phi_14546_ = _e2305;
                                    phi_14547_ = _e2307;
                                    phi_14548_ = select(true, false, _e2309);
                                }
                                let _e2419 = phi_33874_;
                                let _e2421 = phi_33682_;
                                let _e2423 = phi_33603_;
                                let _e2425 = phi_33526_;
                                let _e2427 = phi_33447_;
                                let _e2429 = phi_33365_;
                                let _e2431 = phi_33288_;
                                let _e2433 = phi_33214_;
                                let _e2435 = phi_14545_;
                                let _e2437 = phi_14546_;
                                let _e2439 = phi_14547_;
                                let _e2441 = phi_14548_;
                                let _e2442 = select(true, false, _e2441);
                                if _e2442 {
                                    if (_e345 >= _e361) {
                                        phi_33273_ = _e2433;
                                        phi_14664_ = _e1607;
                                        phi_14665_ = type_71();
                                        phi_14666_ = true;
                                    } else {
                                        let _e2455 = global_1.member.member_4[1u];
                                        if (sqrt((((_e2439.x * _e2439.x) + (_e2439.y * _e2439.y)) + (_e2439.z * _e2439.z))) <= _e2455) {
                                            phi_33268_ = _e2433;
                                            phi_14628_ = _e1607;
                                            phi_14629_ = false;
                                        } else {
                                            let _e2459 = global_1.member.member_7[1u];
                                            if (_e2459 == 1u) {
                                                let _e2464 = global_1.member.member_4[3u];
                                                if (f32(_e345) >= _e2464) {
                                                    let _e2467 = select(_e2439.y, _e2439.x, (_e2439.x > _e2439.y));
                                                    let _e2469 = select(_e2439.z, _e2467, (_e2467 > _e2439.z));
                                                    if (_e2469 != _e2469) {
                                                        phi_26899_ = true;
                                                    } else {
                                                        phi_26899_ = (1f <= _e2469);
                                                    }
                                                    let _e2473 = phi_26899_;
                                                    let _e2474 = select(_e2469, 1f, _e2473);
                                                    let _e2476 = ((_e2433 * 747796405u) + 2891336453u);
                                                    let _e2484 = (((_e2476 >> bitcast<u32>((((_e2476 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2476) * 277803737u);
                                                    let _e2487 = ((_e2484 >> bitcast<u32>(22i)) ^ _e2484);
                                                    let _e2492 = ((f32((_e2487 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e2474);
                                                    if _e2492 {
                                                        phi_14617_ = vec3<f32>();
                                                    } else {
                                                        phi_14617_ = vec3<f32>((_e1607.x / _e2474), (_e1607.y / _e2474), (_e1607.z / _e2474));
                                                    }
                                                    let _e2501 = phi_14617_;
                                                    phi_33271_ = _e2487;
                                                    phi_14619_ = _e2501;
                                                    phi_14620_ = select(true, false, _e2492);
                                                } else {
                                                    phi_33271_ = _e2433;
                                                    phi_14619_ = _e1607;
                                                    phi_14620_ = true;
                                                }
                                                let _e2504 = phi_33271_;
                                                let _e2506 = phi_14619_;
                                                let _e2508 = phi_14620_;
                                                phi_33270_ = _e2504;
                                                phi_14621_ = _e2506;
                                                phi_14622_ = _e2508;
                                            } else {
                                                phi_33270_ = _e2433;
                                                phi_14621_ = _e1607;
                                                phi_14622_ = true;
                                            }
                                            let _e2510 = phi_33270_;
                                            let _e2512 = phi_14621_;
                                            let _e2514 = phi_14622_;
                                            phi_33268_ = _e2510;
                                            phi_14628_ = select(_e1607, _e2512, vec3(_e2514));
                                            phi_14629_ = select(false, true, _e2514);
                                        }
                                        let _e2519 = phi_33268_;
                                        let _e2521 = phi_14628_;
                                        let _e2523 = phi_14629_;
                                        if _e2523 {
                                            let _e2524 = (_e2521 * _e2439);
                                            let _e2535 = (_e2437 * (1f / sqrt((((_e2437.x * _e2437.x) + (_e2437.y * _e2437.y)) + (_e2437.z * _e2437.z)))));
                                            phi_14662_ = type_71(type_35(vec4<f32>(_e2435.x, _e2435.y, _e2435.z, _e318.member.w), vec4<f32>(_e2535.x, _e2535.y, _e2535.z, _e318.member_1.w), vec4<f32>(_e2524.x, _e2524.y, _e2524.z, f32((_e345 + 1u))), vec4<f32>(_e318.member_3.x, _e318.member_3.y, _e318.member_3.z, f32(_e1167)), vec4<u32>(_e2519, (_e318.member_4.y | 2u), 0u, 0u)), _e1609, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14662_ = type_71();
                                        }
                                        let _e2558 = phi_14662_;
                                        phi_33273_ = _e2519;
                                        phi_14664_ = _e2521;
                                        phi_14665_ = _e2558;
                                        phi_14666_ = select(true, false, _e2523);
                                    }
                                    let _e2561 = phi_33273_;
                                    let _e2563 = phi_14664_;
                                    let _e2565 = phi_14665_;
                                    let _e2567 = phi_14666_;
                                    if _e2567 {
                                        phi_14672_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e2563.x, _e2563.y, _e2563.z, f32(_e345)), vec4<f32>(_e318.member_3.x, _e318.member_3.y, _e318.member_3.z, f32(_e1167)), vec4<u32>(_e2561, (_e318.member_4.y & 4294967294u), 0u, 0u)), _e1609, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14672_ = _e2565;
                                    }
                                    let _e2580 = phi_14672_;
                                    phi_33277_ = _e2561;
                                    phi_14673_ = _e2580;
                                } else {
                                    phi_33277_ = _e2433;
                                    phi_14673_ = type_71();
                                }
                                let _e2582 = phi_33277_;
                                let _e2584 = phi_14673_;
                                phi_33871_ = _e2419;
                                phi_33679_ = _e2421;
                                phi_33600_ = _e2423;
                                phi_33523_ = _e2425;
                                phi_33444_ = _e2427;
                                phi_33362_ = _e2429;
                                phi_33285_ = _e2431;
                                phi_33276_ = _e2582;
                                phi_14675_ = _e2584;
                                phi_14676_ = select(true, false, _e2442);
                            }
                            let _e2592 = phi_33871_;
                            let _e2594 = phi_33679_;
                            let _e2596 = phi_33600_;
                            let _e2598 = phi_33523_;
                            let _e2600 = phi_33444_;
                            let _e2602 = phi_33362_;
                            let _e2604 = phi_33285_;
                            let _e2606 = phi_33276_;
                            let _e2608 = phi_14675_;
                            let _e2610 = phi_14676_;
                            if _e2610 {
                                let _e2612 = ((_e318.member_4.y | 4u) & 4294967293u);
                                let _e2615 = global_1.member.member_1[3u];
                                phi_35200_ = _e1605;
                                phi_33275_ = _e2606;
                                phi_14687_ = _e349;
                                phi_14690_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                phi_14692_ = 0u;
                                phi_14694_ = _e1167;
                                loop {
                                    let _e2617 = phi_35200_;
                                    let _e2619 = phi_33275_;
                                    let _e2621 = phi_14687_;
                                    let _e2623 = phi_14690_;
                                    let _e2625 = phi_14692_;
                                    let _e2627 = phi_14694_;
                                    local_87 = _e2627;
                                    local_88 = _e2621;
                                    local_89 = _e2621;
                                    local_90 = _e2621;
                                    local_91 = _e2623;
                                    local_92 = _e2627;
                                    local_93 = _e2621;
                                    local_94 = _e2621;
                                    local_95 = _e2621;
                                    local_96 = _e2623;
                                    local_97 = _e2627;
                                    local_98 = _e2621;
                                    local_99 = _e2621;
                                    local_100 = _e2621;
                                    local_101 = _e2623;
                                    local_102 = _e2627;
                                    local_103 = _e2621;
                                    local_104 = _e2621;
                                    local_105 = _e2621;
                                    local_106 = _e2623;
                                    let _e2629 = (_e2625 < (_e2615 + 3u));
                                    if _e2629 {
                                        if (_e2625 < _e2615) {
                                            let _e4614 = global_5.member[_e2625];
                                            let _e4619 = (_e4614.member.x - _e1737);
                                            let _e4620 = (_e4614.member.y - _e1738);
                                            let _e4621 = (_e4614.member.z - _e1739);
                                            let _e4626 = (((_e4619 * _e4619) + (_e4620 * _e4620)) + (_e4621 * _e4621));
                                            let _e4627 = sqrt(_e4626);
                                            let _e4628 = (_e4619 / _e4627);
                                            let _e4629 = (_e4620 / _e4627);
                                            let _e4630 = (_e4621 / _e4627);
                                            if (_e4614.member.w < 0.5f) {
                                                phi_14791_ = 1f;
                                            } else {
                                                let _e4654 = (((_e4614.member_2.x * -(_e4628)) + (_e4614.member_2.y * -(_e4629))) + (_e4614.member_2.z * -(_e4630)));
                                                if (_e4614.member_3.x == _e4614.member_2.w) {
                                                    phi_14790_ = select(1f, 0f, (_e4654 < _e4614.member_3.x));
                                                } else {
                                                    let _e4658 = ((_e4654 - _e4614.member_3.x) / (_e4614.member_2.w - _e4614.member_3.x));
                                                    let _e4660 = select(_e4658, 0f, (_e4658 < 0f));
                                                    let _e4662 = select(_e4660, 1f, (_e4660 > 1f));
                                                    phi_14790_ = ((_e4662 * _e4662) * (3f - (2f * _e4662)));
                                                }
                                                let _e4670 = phi_14790_;
                                                phi_14791_ = _e4670;
                                            }
                                            let _e4672 = phi_14791_;
                                            phi_35618_ = _e2619;
                                            phi_17729_ = 0f;
                                            phi_17730_ = vec3<f32>(_e4628, _e4629, _e4630);
                                            phi_17731_ = vec3<f32>(((_e4614.member_1.x * _e4672) / _e4626), ((_e4614.member_1.y * _e4672) / _e4626), ((_e4614.member_1.z * _e4672) / _e4626));
                                            phi_17732_ = _e4627;
                                        } else {
                                            if (_e2625 == _e2615) {
                                                if ((_e364 & 4u) == 0u) {
                                                    phi_35662_ = _e2619;
                                                    phi_16833_ = 0f;
                                                    phi_16834_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_16835_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e3366 = global_1.member.member_5;
                                                    let _e3374 = global_1.member.member_6[3u];
                                                    let _e3376 = sin((_e3374 * 0.5f));
                                                    let _e3378 = ((12.566371f * _e3376) * _e3376);
                                                    let _e3383 = (((_e3366.x * _e1712.x) + (_e3366.y * _e1712.y)) + (_e3366.z * _e1712.z));
                                                    if (_e3383 != _e3383) {
                                                        phi_27438_ = true;
                                                    } else {
                                                        phi_27438_ = (0f >= _e3383);
                                                    }
                                                    let _e3387 = phi_27438_;
                                                    let _e3389 = (select(_e3383, 0f, _e3387) * 0.31830987f);
                                                    if (_e2604 == 0u) {
                                                        if (_e3383 <= 0f) {
                                                            phi_28235_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_28235_ = (_e2598 * (_e3383 * 0.31830987f));
                                                        }
                                                        let _e3865 = phi_28235_;
                                                        phi_15201_ = type_56(_e3865, _e3389);
                                                        phi_15202_ = type_56();
                                                        phi_15203_ = true;
                                                    } else {
                                                        let _e3395 = (((_e1684 * _e1760) + (_e1685 * _e1761)) + (_e1686 * _e1763));
                                                        let _e3400 = (((_e1684 * _e1756) + (_e1685 * _e1766)) + (_e1686 * _e1767));
                                                        let _e3405 = (((_e1684 * _e1712.x) + (_e1685 * _e1712.y)) + (_e1686 * _e1712.z));
                                                        let _e3410 = (((_e3366.x * _e1760) + (_e3366.y * _e1761)) + (_e3366.z * _e1763));
                                                        let _e3415 = (((_e3366.x * _e1756) + (_e3366.y * _e1766)) + (_e3366.z * _e1767));
                                                        if (_e3405 <= 0f) {
                                                            phi_15165_ = type_56();
                                                            phi_15166_ = true;
                                                        } else {
                                                            let _e3417 = (_e3383 <= 0f);
                                                            if _e3417 {
                                                                phi_15164_ = type_56();
                                                            } else {
                                                                let _e3418 = (_e3395 + _e3410);
                                                                let _e3419 = (_e3400 + _e3415);
                                                                let _e3420 = (_e3405 + _e3383);
                                                                let _e3427 = (1f / sqrt((((_e3418 * _e3418) + (_e3419 * _e3419)) + (_e3420 * _e3420))));
                                                                let _e3430 = (_e3420 * _e3427);
                                                                if (_e3430 <= 0f) {
                                                                    phi_27548_ = 0f;
                                                                } else {
                                                                    let _e3432 = (_e2602 * _e2602);
                                                                    let _e3436 = ((((_e3432 - 1f) * _e3430) * _e3430) + 1f);
                                                                    phi_27548_ = (_e3432 / ((3.1415927f * _e3436) * _e3436));
                                                                }
                                                                let _e3441 = phi_27548_;
                                                                let _e3442 = (_e3405 * _e3405);
                                                                if (_e3442 >= 1f) {
                                                                    phi_27576_ = 0f;
                                                                } else {
                                                                    phi_27576_ = ((sqrt((1f + (((_e2602 * _e2602) * ((_e3395 * _e3395) + (_e3400 * _e3400))) / _e3442))) - 1f) * 0.5f);
                                                                }
                                                                let _e3455 = phi_27576_;
                                                                let _e3456 = (1f + _e3455);
                                                                let _e3457 = (_e3383 * _e3383);
                                                                if (_e3457 >= 1f) {
                                                                    phi_27604_ = 0f;
                                                                } else {
                                                                    phi_27604_ = ((sqrt((1f + (((_e2602 * _e2602) * ((_e3410 * _e3410) + (_e3415 * _e3415))) / _e3457))) - 1f) * 0.5f);
                                                                }
                                                                let _e3470 = phi_27604_;
                                                                let _e3477 = (((_e3395 * (_e3418 * _e3427)) + (_e3400 * (_e3419 * _e3427))) + (_e3405 * _e3430));
                                                                if (_e2600 == 0u) {
                                                                    let _e3772 = (1f - _e3477);
                                                                    if (_e3772 != _e3772) {
                                                                        phi_28107_ = true;
                                                                    } else {
                                                                        phi_28107_ = (0f >= _e3772);
                                                                    }
                                                                    let _e3776 = phi_28107_;
                                                                    let _e3778 = pow(select(_e3772, 0f, _e3776), 5f);
                                                                    let _e3779 = (1f - _e3778);
                                                                    phi_15152_ = vec3<f32>(((_e2598.x * _e3779) + _e3778), ((_e2598.y * _e3779) + _e3778), ((_e2598.z * _e3779) + _e3778));
                                                                } else {
                                                                    if (_e2600 == 1u) {
                                                                        let _e3745 = (1f / _e2594);
                                                                        let _e3749 = ((_e3745 * _e3745) * (1f - (_e3477 * _e3477)));
                                                                        if (_e3749 >= 1f) {
                                                                            phi_28068_ = 1f;
                                                                        } else {
                                                                            let _e3752 = sqrt((1f - _e3749));
                                                                            if (_e3477 != _e3477) {
                                                                                phi_28079_ = true;
                                                                            } else {
                                                                                phi_28079_ = (_e3752 <= _e3477);
                                                                            }
                                                                            let _e3756 = phi_28079_;
                                                                            let _e3761 = pow(((1f - _e2594) / (1f + _e2594)), 2f);
                                                                            phi_28068_ = (_e3761 + ((1f - _e3761) * pow((1f - select(_e3477, _e3752, _e3756)), 5f)));
                                                                        }
                                                                        let _e3768 = phi_28068_;
                                                                        phi_15151_ = vec3(_e3768);
                                                                    } else {
                                                                        let _e3481 = select(_e3477, 0f, (_e3477 < 0f));
                                                                        let _e3483 = select(_e3481, 1f, (_e3481 > 1f));
                                                                        let _e3485 = (1f - (_e3483 * _e3483));
                                                                        let _e3498 = ((_e2598.x * _e2598.x) - (_e2596.x * _e2596.x));
                                                                        let _e3499 = ((_e2598.y * _e2598.y) - (_e2596.y * _e2596.y));
                                                                        let _e3500 = ((_e2598.z * _e2598.z) - (_e2596.z * _e2596.z));
                                                                        let _e3502 = (_e2598.x * (_e2596.x + _e2596.x));
                                                                        let _e3504 = (_e2598.y * (_e2596.y + _e2596.y));
                                                                        let _e3506 = (_e2598.z * (_e2596.z + _e2596.z));
                                                                        let _e3513 = ((_e3498 * _e3498) + (_e3502 * _e3502));
                                                                        let _e3514 = ((_e3499 * _e3499) + (_e3504 * _e3504));
                                                                        let _e3515 = ((_e3500 * _e3500) + (_e3506 * _e3506));
                                                                        let _e3520 = ((_e3485 * _e3498) / _e3513);
                                                                        let _e3521 = ((_e3485 * _e3499) / _e3514);
                                                                        let _e3522 = ((_e3485 * _e3500) / _e3515);
                                                                        let _e3523 = (1f - _e3520);
                                                                        let _e3524 = (1f - _e3521);
                                                                        let _e3525 = (1f - _e3522);
                                                                        let _e3527 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3520, _e3521, _e3522));
                                                                        let _e3528 = (_e3485 * _e3502);
                                                                        let _e3529 = (_e3485 * _e3504);
                                                                        let _e3530 = (_e3485 * _e3506);
                                                                        let _e3535 = (vec3<f32>(_e3528, _e3529, _e3530) / vec3<f32>(_e3513, _e3514, _e3515));
                                                                        let _e3548 = ((_e3527.x * _e3527.x) + (_e3535.x * _e3535.x));
                                                                        let _e3549 = ((_e3527.y * _e3527.y) + (_e3535.y * _e3535.y));
                                                                        let _e3550 = ((_e3527.z * _e3527.z) + (_e3535.z * _e3535.z));
                                                                        if (_e3548 != _e3548) {
                                                                            phi_27738_ = true;
                                                                        } else {
                                                                            phi_27738_ = (0f >= _e3548);
                                                                        }
                                                                        let _e3554 = phi_27738_;
                                                                        let _e3556 = sqrt(select(_e3548, 0f, _e3554));
                                                                        if (_e3549 != _e3549) {
                                                                            phi_27753_ = true;
                                                                        } else {
                                                                            phi_27753_ = (0f >= _e3549);
                                                                        }
                                                                        let _e3560 = phi_27753_;
                                                                        let _e3562 = sqrt(select(_e3549, 0f, _e3560));
                                                                        if (_e3550 != _e3550) {
                                                                            phi_27768_ = true;
                                                                        } else {
                                                                            phi_27768_ = (0f >= _e3550);
                                                                        }
                                                                        let _e3566 = phi_27768_;
                                                                        let _e3568 = sqrt(select(_e3550, 0f, _e3566));
                                                                        let _e3572 = ((_e3556 + _e3523) * 0.5f);
                                                                        let _e3573 = ((_e3562 + _e3524) * 0.5f);
                                                                        let _e3574 = ((_e3568 + _e3525) * 0.5f);
                                                                        if (_e3572 != _e3572) {
                                                                            phi_27795_ = true;
                                                                        } else {
                                                                            phi_27795_ = (0f >= _e3572);
                                                                        }
                                                                        let _e3578 = phi_27795_;
                                                                        let _e3580 = sqrt(select(_e3572, 0f, _e3578));
                                                                        if (_e3573 != _e3573) {
                                                                            phi_27810_ = true;
                                                                        } else {
                                                                            phi_27810_ = (0f >= _e3573);
                                                                        }
                                                                        let _e3584 = phi_27810_;
                                                                        let _e3586 = sqrt(select(_e3573, 0f, _e3584));
                                                                        if (_e3574 != _e3574) {
                                                                            phi_27825_ = true;
                                                                        } else {
                                                                            phi_27825_ = (0f >= _e3574);
                                                                        }
                                                                        let _e3590 = phi_27825_;
                                                                        let _e3592 = sqrt(select(_e3574, 0f, _e3590));
                                                                        let _e3593 = vec3<f32>(_e3580, _e3586, _e3592);
                                                                        let _e3597 = ((_e3556 - _e3523) * 0.5f);
                                                                        let _e3598 = ((_e3562 - _e3524) * 0.5f);
                                                                        let _e3599 = ((_e3568 - _e3525) * 0.5f);
                                                                        if (_e3597 != _e3597) {
                                                                            phi_27852_ = true;
                                                                        } else {
                                                                            phi_27852_ = (0f >= _e3597);
                                                                        }
                                                                        let _e3603 = phi_27852_;
                                                                        let _e3605 = sqrt(select(_e3597, 0f, _e3603));
                                                                        if (_e3598 != _e3598) {
                                                                            phi_27867_ = true;
                                                                        } else {
                                                                            phi_27867_ = (0f >= _e3598);
                                                                        }
                                                                        let _e3609 = phi_27867_;
                                                                        let _e3611 = sqrt(select(_e3598, 0f, _e3609));
                                                                        if (_e3599 != _e3599) {
                                                                            phi_27882_ = true;
                                                                        } else {
                                                                            phi_27882_ = (0f >= _e3599);
                                                                        }
                                                                        let _e3615 = phi_27882_;
                                                                        let _e3617 = sqrt(select(_e3599, 0f, _e3615));
                                                                        if ((_e3528 / _e3513) < 0f) {
                                                                            phi_15028_ = -(_e3605);
                                                                        } else {
                                                                            phi_15028_ = _e3605;
                                                                        }
                                                                        let _e3621 = phi_15028_;
                                                                        if ((_e3529 / _e3514) < 0f) {
                                                                            phi_15036_ = -(_e3611);
                                                                        } else {
                                                                            phi_15036_ = _e3611;
                                                                        }
                                                                        let _e3625 = phi_15036_;
                                                                        if ((_e3530 / _e3515) < 0f) {
                                                                            phi_15044_ = -(_e3617);
                                                                        } else {
                                                                            phi_15044_ = _e3617;
                                                                        }
                                                                        let _e3629 = phi_15044_;
                                                                        let _e3633 = (_e2596.x * _e3483);
                                                                        let _e3634 = (_e2596.y * _e3483);
                                                                        let _e3635 = (_e2596.z * _e3483);
                                                                        let _e3645 = vec3<f32>(((_e2598.x * _e3580) - (_e2596.x * _e3621)), ((_e2598.y * _e3586) - (_e2596.y * _e3625)), ((_e2598.z * _e3592) - (_e2596.z * _e3629)));
                                                                        let _e3652 = ((_e2598.x * _e3621) + (_e2596.x * _e3580));
                                                                        let _e3653 = ((_e2598.y * _e3625) + (_e2596.y * _e3586));
                                                                        let _e3654 = ((_e2598.z * _e3629) + (_e2596.z * _e3592));
                                                                        let _e3656 = vec3<f32>((_e2598.x * _e3483), (_e2598.y * _e3483), (_e2598.z * _e3483));
                                                                        let _e3657 = (_e3656 - _e3593);
                                                                        let _e3658 = (_e3633 - _e3621);
                                                                        let _e3659 = (_e3634 - _e3625);
                                                                        let _e3660 = (_e3635 - _e3629);
                                                                        let _e3673 = (_e3656 + _e3593);
                                                                        let _e3674 = (_e3633 + _e3621);
                                                                        let _e3675 = (_e3634 + _e3625);
                                                                        let _e3676 = (_e3635 + _e3629);
                                                                        let _e3686 = ((_e3673.x * _e3673.x) + (_e3674 * _e3674));
                                                                        let _e3687 = ((_e3673.y * _e3673.y) + (_e3675 * _e3675));
                                                                        let _e3688 = ((_e3673.z * _e3673.z) + (_e3676 * _e3676));
                                                                        let _e3698 = vec3(_e3483);
                                                                        let _e3699 = (_e3698 - _e3645);
                                                                        let _e3700 = -(vec3<f32>(_e3652, _e3653, _e3654));
                                                                        let _e3716 = (_e3698 + _e3645);
                                                                        let _e3726 = ((_e3716.x * _e3716.x) + (_e3652 * _e3652));
                                                                        let _e3727 = ((_e3716.y * _e3716.y) + (_e3653 * _e3653));
                                                                        let _e3728 = ((_e3716.z * _e3716.z) + (_e3654 * _e3654));
                                                                        phi_15151_ = vec3<f32>((((((_e3657.x * _e3657.x) + (_e3658 * _e3658)) / select(0.000000000000000000000000000001f, _e3686, (_e3686 > 0.000000000000000000000000000001f))) + (((_e3699.x * _e3699.x) + (_e3700.x * _e3700.x)) / select(0.000000000000000000000000000001f, _e3726, (_e3726 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3657.y * _e3657.y) + (_e3659 * _e3659)) / select(0.000000000000000000000000000001f, _e3687, (_e3687 > 0.000000000000000000000000000001f))) + (((_e3699.y * _e3699.y) + (_e3700.y * _e3700.y)) / select(0.000000000000000000000000000001f, _e3727, (_e3727 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3657.z * _e3657.z) + (_e3660 * _e3660)) / select(0.000000000000000000000000000001f, _e3688, (_e3688 > 0.000000000000000000000000000001f))) + (((_e3699.z * _e3699.z) + (_e3700.z * _e3700.z)) / select(0.000000000000000000000000000001f, _e3728, (_e3728 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                    }
                                                                    let _e3771 = phi_15151_;
                                                                    phi_15152_ = _e3771;
                                                                }
                                                                let _e3791 = phi_15152_;
                                                                let _e3793 = (4f * _e3405);
                                                                phi_15164_ = type_56((_e3791 * (((1f / (_e3456 + _e3470)) * _e3441) / _e3793)), ((_e3441 / _e3456) / _e3793));
                                                            }
                                                            let _e3800 = phi_15164_;
                                                            phi_15165_ = _e3800;
                                                            phi_15166_ = _e3417;
                                                        }
                                                        let _e3802 = phi_15165_;
                                                        let _e3804 = phi_15166_;
                                                        if _e3804 {
                                                            phi_15171_ = type_56(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            phi_15171_ = _e3802;
                                                        }
                                                        let _e3806 = phi_15171_;
                                                        let _e3809 = (_e2604 == 1u);
                                                        if _e3809 {
                                                            phi_15199_ = _e3806;
                                                            phi_15200_ = type_56();
                                                        } else {
                                                            if (_e3405 != _e3405) {
                                                                phi_28144_ = true;
                                                            } else {
                                                                phi_28144_ = (0f >= _e3405);
                                                            }
                                                            let _e3813 = phi_28144_;
                                                            let _e3814 = select(_e3405, 0f, _e3813);
                                                            let _e3815 = (1f / _e2594);
                                                            let _e3819 = ((_e3815 * _e3815) * (1f - (_e3814 * _e3814)));
                                                            if (_e3819 >= 1f) {
                                                                phi_28163_ = 1f;
                                                            } else {
                                                                let _e3822 = sqrt((1f - _e3819));
                                                                if (_e3814 != _e3814) {
                                                                    phi_28174_ = true;
                                                                } else {
                                                                    phi_28174_ = (_e3822 <= _e3814);
                                                                }
                                                                let _e3826 = phi_28174_;
                                                                let _e3831 = pow(((1f - _e2594) / (1f + _e2594)), 2f);
                                                                phi_28163_ = (_e3831 + ((1f - _e3831) * pow((1f - select(_e3814, _e3822, _e3826)), 5f)));
                                                            }
                                                            let _e3838 = phi_28163_;
                                                            if (_e3383 <= 0f) {
                                                                phi_28213_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_28213_ = (_e2592 * (_e3383 * 0.31830987f));
                                                            }
                                                            let _e3843 = phi_28213_;
                                                            let _e3844 = (1f - _e3838);
                                                            phi_15199_ = type_56();
                                                            phi_15200_ = type_56((_e3806.member + vec3<f32>((_e3843.x * _e3844), (_e3843.y * _e3844), (_e3843.z * _e3844))), ((_e3838 * _e3806.member_1) + (_e3844 * _e3389)));
                                                        }
                                                        let _e3858 = phi_15199_;
                                                        let _e3860 = phi_15200_;
                                                        phi_15201_ = _e3858;
                                                        phi_15202_ = _e3860;
                                                        phi_15203_ = _e3809;
                                                    }
                                                    let _e3868 = phi_15201_;
                                                    let _e3870 = phi_15202_;
                                                    let _e3872 = phi_15203_;
                                                    if _e3872 {
                                                        phi_15207_ = _e3868;
                                                    } else {
                                                        phi_15207_ = _e3870;
                                                    }
                                                    let _e3874 = phi_15207_;
                                                    if ((_e3874.member_1 * _e3378) < 0.001f) {
                                                        let _e4301 = global_1.member.member_6;
                                                        let _e4307 = global_1.member.member_8[2u];
                                                        if (_e4307 == 4294967295u) {
                                                            phi_15946_ = vec3<f32>(1f, 1f, 1f);
                                                        } else {
                                                            let _e4310 = global_6.member[_e4307];
                                                            let _e4313 = global_6.member[(_e4307 + 1u)];
                                                            let _e4316 = global_6.member[(_e4307 + 2u)];
                                                            let _e4319 = global_6.member[(_e4307 + 3u)];
                                                            let _e4322 = global_6.member[(_e4307 + 4u)];
                                                            let _e4325 = global_6.member[(_e4307 + 5u)];
                                                            let _e4328 = global_6.member[(_e4307 + 6u)];
                                                            let _e4331 = global_6.member[(_e4307 + 7u)];
                                                            let _e4334 = global_6.member[(_e4307 + 8u)];
                                                            let _e4337 = global_6.member[(_e4307 + 9u)];
                                                            let _e4340 = global_6.member[(_e4307 + 13u)];
                                                            let _e4343 = global_6.member[(_e4307 + 14u)];
                                                            let _e4346 = global_6.member[(_e4307 + 15u)];
                                                            let _e4349 = global_6.member[(_e4307 + 16u)];
                                                            let _e4352 = global_6.member[(_e4307 + 18u)];
                                                            let _e4355 = global_6.member[(_e4307 + 19u)];
                                                            let _e4358 = global_6.member[(_e4307 + 20u)];
                                                            let _e4361 = global_6.member[(_e4307 + 29u)];
                                                            let _e4362 = bitcast<u32>(_e4361);
                                                            let _e4375 = (((((_e1737 * _e1737) + (_e1738 * _e1738)) + (_e1739 * _e1739)) - (2f * (((_e1737 * _e4310) + (_e1738 * _e4313)) + (_e1739 * _e4316)))) + _e4328);
                                                            if (_e4375 > 0f) {
                                                                let _e4384 = ((((_e1737 - _e4310) * _e4331) + ((_e1738 - _e4313) * _e4334)) + ((_e1739 - _e4316) * _e4337));
                                                                let _e4386 = ((_e4384 * _e4384) - _e4375);
                                                                if (_e4384 >= 0f) {
                                                                    phi_15646_ = vec3<f32>();
                                                                    phi_15647_ = true;
                                                                } else {
                                                                    let _e4388 = (_e4386 <= 0f);
                                                                    if _e4388 {
                                                                        phi_15645_ = vec3<f32>();
                                                                    } else {
                                                                        let _e4391 = (-(_e4384) - sqrt(_e4386));
                                                                        phi_15645_ = vec3<f32>((_e1737 + (_e4331 * _e4391)), (_e1738 + (_e4334 * _e4391)), (_e1739 + (_e4337 * _e4391)));
                                                                    }
                                                                    let _e4400 = phi_15645_;
                                                                    phi_15646_ = _e4400;
                                                                    phi_15647_ = _e4388;
                                                                }
                                                                let _e4402 = phi_15646_;
                                                                let _e4404 = phi_15647_;
                                                                phi_15652_ = _e4402;
                                                                phi_15653_ = select(true, false, _e4404);
                                                            } else {
                                                                phi_15652_ = _e1740;
                                                                phi_15653_ = true;
                                                            }
                                                            let _e4407 = phi_15652_;
                                                            let _e4409 = phi_15653_;
                                                            if _e4409 {
                                                                let _e4411 = (_e4407.x - _e4310);
                                                                let _e4413 = (_e4407.y - _e4313);
                                                                let _e4415 = (_e4407.z - _e4316);
                                                                let _e4421 = sqrt((((_e4411 * _e4411) + (_e4413 * _e4413)) + (_e4415 * _e4415)));
                                                                let _e4422 = (1f / _e4421);
                                                                let _e4440 = ((((((_e4407.x * _e4407.x) + (_e4407.y * _e4407.y)) + (_e4407.z * _e4407.z)) - (2f * (((_e4407.x * _e4310) + (_e4407.y * _e4313)) + (_e4407.z * _e4316)))) + _e4325) / (_e4421 + _e4319));
                                                                if (_e4440 != _e4440) {
                                                                    phi_28669_ = true;
                                                                } else {
                                                                    phi_28669_ = (0f >= _e4440);
                                                                }
                                                                let _e4444 = phi_28669_;
                                                                let _e4448 = sqrt((select(_e4440, 0f, _e4444) / (_e4322 - _e4319)));
                                                                if (_e4448 != _e4448) {
                                                                    phi_28684_ = true;
                                                                } else {
                                                                    phi_28684_ = (1f <= _e4448);
                                                                }
                                                                let _e4452 = phi_28684_;
                                                                let _e4454 = (select(_e4448, 1f, _e4452) * 63f);
                                                                let _e4459 = ((((_e4411 * _e4422) * _e4331) + ((_e4413 * _e4422) * _e4334)) + ((_e4415 * _e4422) * _e4337));
                                                                let _e4461 = select(_e4459, -1f, (_e4459 < -1f));
                                                                let _e4465 = ((select(_e4461, 1f, (_e4461 > 1f)) + 1f) * 127.5f);
                                                                let _e4470 = select(select(u32(_e4454), 0u, (_e4454 < 0f)), 4294967295u, (_e4454 > 4294967000f));
                                                                let _e4472 = select(_e4470, 62u, (62u < _e4470));
                                                                let _e4477 = select(select(u32(_e4465), 0u, (_e4465 < 0f)), 4294967295u, (_e4465 > 4294967000f));
                                                                let _e4479 = select(_e4477, 254u, (254u < _e4477));
                                                                let _e4481 = (_e4454 - f32(_e4472));
                                                                let _e4483 = (_e4465 - f32(_e4479));
                                                                let _e4487 = (_e4362 + (3u * ((_e4472 * 256u) + _e4479)));
                                                                let _e4492 = (_e4362 + (3u * (((_e4472 + 1u) * 256u) + _e4479)));
                                                                let _e4494 = global_6.member[_e4487];
                                                                let _e4497 = global_6.member[(_e4487 + 1u)];
                                                                let _e4500 = global_6.member[(_e4487 + 2u)];
                                                                let _e4501 = (1f - _e4483);
                                                                let _e4507 = global_6.member[(_e4487 + 3u)];
                                                                let _e4510 = global_6.member[(_e4487 + 4u)];
                                                                let _e4513 = global_6.member[(_e4487 + 5u)];
                                                                let _e4520 = (1f - _e4481);
                                                                let _e4525 = global_6.member[_e4492];
                                                                let _e4528 = global_6.member[(_e4492 + 1u)];
                                                                let _e4531 = global_6.member[(_e4492 + 2u)];
                                                                let _e4537 = global_6.member[(_e4492 + 3u)];
                                                                let _e4540 = global_6.member[(_e4492 + 4u)];
                                                                let _e4543 = global_6.member[(_e4492 + 5u)];
                                                                phi_15944_ = vec3<f32>(((((_e4494 * _e4501) + (_e4507 * _e4483)) * _e4520) + (((_e4525 * _e4501) + (_e4537 * _e4483)) * _e4481)), ((((_e4497 * _e4501) + (_e4510 * _e4483)) * _e4520) + (((_e4528 * _e4501) + (_e4540 * _e4483)) * _e4481)), ((((_e4500 * _e4501) + (_e4513 * _e4483)) * _e4520) + (((_e4531 * _e4501) + (_e4543 * _e4483)) * _e4481)));
                                                            } else {
                                                                phi_15944_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e4558 = phi_15944_;
                                                            let _e4565 = ((_e4349 * 1.1f) * _e4558.y);
                                                            phi_15946_ = vec3<f32>(exp(-((((_e4340 * _e4558.x) + _e4565) + (_e4352 * _e4558.z)))), exp(-((((_e4343 * _e4558.x) + _e4565) + (_e4355 * _e4558.z)))), exp(-((((_e4346 * _e4558.x) + _e4565) + (_e4358 * _e4558.z)))));
                                                        }
                                                        let _e4584 = phi_15946_;
                                                        phi_35663_ = _e2619;
                                                        phi_16830_ = 0f;
                                                        phi_16831_ = vec3<f32>(_e3366.x, _e3366.y, _e3366.z);
                                                        phi_16832_ = (vec3<f32>(_e4301.x, _e4301.y, _e4301.z) * _e4584);
                                                    } else {
                                                        let _e3879 = ((_e2619 * 747796405u) + 2891336453u);
                                                        let _e3887 = (((_e3879 >> bitcast<u32>((((_e3879 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3879) * 277803737u);
                                                        let _e3890 = ((_e3887 >> bitcast<u32>(22i)) ^ _e3887);
                                                        let _e3896 = (((f32((_e3890 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e3378) * 0.15915494f);
                                                        let _e3897 = (1f - _e3896);
                                                        let _e3899 = (_e3896 * (2f - _e3896));
                                                        if (_e3899 != _e3899) {
                                                            phi_28267_ = true;
                                                        } else {
                                                            phi_28267_ = (0f >= _e3899);
                                                        }
                                                        let _e3903 = phi_28267_;
                                                        let _e3905 = sqrt(select(_e3899, 0f, _e3903));
                                                        let _e3907 = ((_e3890 * 747796405u) + 2891336453u);
                                                        let _e3915 = (((_e3907 >> bitcast<u32>((((_e3907 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3907) * 277803737u);
                                                        let _e3918 = ((_e3915 >> bitcast<u32>(22i)) ^ _e3915);
                                                        let _e3922 = (f32((_e3918 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                        let _e3924 = select(-1f, 1f, (_e3366.z >= 0f));
                                                        let _e3926 = (-1f / (_e3924 + _e3366.z));
                                                        let _e3928 = ((_e3366.x * _e3366.y) * _e3926);
                                                        let _e3941 = (_e3905 * cos(_e3922));
                                                        let _e3946 = (_e3905 * sin(_e3922));
                                                        let _e3956 = ((((1f + (((_e3924 * _e3366.x) * _e3366.x) * _e3926)) * _e3941) + (_e3928 * _e3946)) + (_e3366.x * _e3897));
                                                        let _e3957 = ((((_e3924 * _e3928) * _e3941) + ((_e3924 + ((_e3366.y * _e3366.y) * _e3926)) * _e3946)) + (_e3366.y * _e3897));
                                                        let _e3958 = ((((-(_e3924) * _e3366.x) * _e3941) + (-(_e3366.y) * _e3946)) + (_e3366.z * _e3897));
                                                        let _e3960 = (_e3956 - _e3366.x);
                                                        let _e3961 = (_e3957 - _e3366.y);
                                                        let _e3962 = (_e3958 - _e3366.z);
                                                        let _e3969 = (sqrt((((_e3960 * _e3960) + (_e3961 * _e3961)) + (_e3962 * _e3962))) * 0.5f);
                                                        let _e3970 = (_e3969 != _e3969);
                                                        if _e3970 {
                                                            phi_28338_ = true;
                                                        } else {
                                                            phi_28338_ = (1f <= _e3969);
                                                        }
                                                        let _e3973 = phi_28338_;
                                                        if ((2f * asin(select(_e3969, 1f, _e3973))) >= _e3374) {
                                                            phi_16027_ = 0f;
                                                        } else {
                                                            phi_16027_ = (1f / _e3378);
                                                        }
                                                        let _e3980 = phi_16027_;
                                                        if (_e3980 > 0f) {
                                                            if _e3970 {
                                                                phi_28353_ = true;
                                                            } else {
                                                                phi_28353_ = (1f <= _e3969);
                                                            }
                                                            let _e3984 = phi_28353_;
                                                            let _e3987 = (2f * asin(select(_e3969, 1f, _e3984)));
                                                            if (_e3987 >= _e3374) {
                                                                phi_16828_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                let _e3991 = (1f - pow((_e3987 / _e3374), 2f));
                                                                if (_e3991 != _e3991) {
                                                                    phi_28368_ = true;
                                                                } else {
                                                                    phi_28368_ = (0f >= _e3991);
                                                                }
                                                                let _e3995 = phi_28368_;
                                                                let _e3998 = global_1.member.member_6;
                                                                let _e4004 = global_1.member.member_8[2u];
                                                                if (_e4004 == 4294967295u) {
                                                                    phi_16801_ = vec3<f32>(1f, 1f, 1f);
                                                                } else {
                                                                    let _e4007 = global_6.member[_e4004];
                                                                    let _e4010 = global_6.member[(_e4004 + 1u)];
                                                                    let _e4013 = global_6.member[(_e4004 + 2u)];
                                                                    let _e4016 = global_6.member[(_e4004 + 3u)];
                                                                    let _e4019 = global_6.member[(_e4004 + 4u)];
                                                                    let _e4022 = global_6.member[(_e4004 + 5u)];
                                                                    let _e4025 = global_6.member[(_e4004 + 6u)];
                                                                    let _e4028 = global_6.member[(_e4004 + 7u)];
                                                                    let _e4031 = global_6.member[(_e4004 + 8u)];
                                                                    let _e4034 = global_6.member[(_e4004 + 9u)];
                                                                    let _e4037 = global_6.member[(_e4004 + 13u)];
                                                                    let _e4040 = global_6.member[(_e4004 + 14u)];
                                                                    let _e4043 = global_6.member[(_e4004 + 15u)];
                                                                    let _e4046 = global_6.member[(_e4004 + 16u)];
                                                                    let _e4049 = global_6.member[(_e4004 + 18u)];
                                                                    let _e4052 = global_6.member[(_e4004 + 19u)];
                                                                    let _e4055 = global_6.member[(_e4004 + 20u)];
                                                                    let _e4058 = global_6.member[(_e4004 + 29u)];
                                                                    let _e4059 = bitcast<u32>(_e4058);
                                                                    let _e4072 = (((((_e1737 * _e1737) + (_e1738 * _e1738)) + (_e1739 * _e1739)) - (2f * (((_e1737 * _e4007) + (_e1738 * _e4010)) + (_e1739 * _e4013)))) + _e4025);
                                                                    if (_e4072 > 0f) {
                                                                        let _e4081 = ((((_e1737 - _e4007) * _e4028) + ((_e1738 - _e4010) * _e4031)) + ((_e1739 - _e4013) * _e4034));
                                                                        let _e4083 = ((_e4081 * _e4081) - _e4072);
                                                                        if (_e4081 >= 0f) {
                                                                            phi_16501_ = vec3<f32>();
                                                                            phi_16502_ = true;
                                                                        } else {
                                                                            let _e4085 = (_e4083 <= 0f);
                                                                            if _e4085 {
                                                                                phi_16500_ = vec3<f32>();
                                                                            } else {
                                                                                let _e4088 = (-(_e4081) - sqrt(_e4083));
                                                                                phi_16500_ = vec3<f32>((_e1737 + (_e4028 * _e4088)), (_e1738 + (_e4031 * _e4088)), (_e1739 + (_e4034 * _e4088)));
                                                                            }
                                                                            let _e4097 = phi_16500_;
                                                                            phi_16501_ = _e4097;
                                                                            phi_16502_ = _e4085;
                                                                        }
                                                                        let _e4099 = phi_16501_;
                                                                        let _e4101 = phi_16502_;
                                                                        phi_16507_ = _e4099;
                                                                        phi_16508_ = select(true, false, _e4101);
                                                                    } else {
                                                                        phi_16507_ = _e1740;
                                                                        phi_16508_ = true;
                                                                    }
                                                                    let _e4104 = phi_16507_;
                                                                    let _e4106 = phi_16508_;
                                                                    if _e4106 {
                                                                        let _e4108 = (_e4104.x - _e4007);
                                                                        let _e4110 = (_e4104.y - _e4010);
                                                                        let _e4112 = (_e4104.z - _e4013);
                                                                        let _e4118 = sqrt((((_e4108 * _e4108) + (_e4110 * _e4110)) + (_e4112 * _e4112)));
                                                                        let _e4119 = (1f / _e4118);
                                                                        let _e4137 = ((((((_e4104.x * _e4104.x) + (_e4104.y * _e4104.y)) + (_e4104.z * _e4104.z)) - (2f * (((_e4104.x * _e4007) + (_e4104.y * _e4010)) + (_e4104.z * _e4013)))) + _e4022) / (_e4118 + _e4016));
                                                                        if (_e4137 != _e4137) {
                                                                            phi_28492_ = true;
                                                                        } else {
                                                                            phi_28492_ = (0f >= _e4137);
                                                                        }
                                                                        let _e4141 = phi_28492_;
                                                                        let _e4145 = sqrt((select(_e4137, 0f, _e4141) / (_e4019 - _e4016)));
                                                                        if (_e4145 != _e4145) {
                                                                            phi_28507_ = true;
                                                                        } else {
                                                                            phi_28507_ = (1f <= _e4145);
                                                                        }
                                                                        let _e4149 = phi_28507_;
                                                                        let _e4151 = (select(_e4145, 1f, _e4149) * 63f);
                                                                        let _e4156 = ((((_e4108 * _e4119) * _e4028) + ((_e4110 * _e4119) * _e4031)) + ((_e4112 * _e4119) * _e4034));
                                                                        let _e4158 = select(_e4156, -1f, (_e4156 < -1f));
                                                                        let _e4162 = ((select(_e4158, 1f, (_e4158 > 1f)) + 1f) * 127.5f);
                                                                        let _e4167 = select(select(u32(_e4151), 0u, (_e4151 < 0f)), 4294967295u, (_e4151 > 4294967000f));
                                                                        let _e4169 = select(_e4167, 62u, (62u < _e4167));
                                                                        let _e4174 = select(select(u32(_e4162), 0u, (_e4162 < 0f)), 4294967295u, (_e4162 > 4294967000f));
                                                                        let _e4176 = select(_e4174, 254u, (254u < _e4174));
                                                                        let _e4178 = (_e4151 - f32(_e4169));
                                                                        let _e4180 = (_e4162 - f32(_e4176));
                                                                        let _e4184 = (_e4059 + (3u * ((_e4169 * 256u) + _e4176)));
                                                                        let _e4189 = (_e4059 + (3u * (((_e4169 + 1u) * 256u) + _e4176)));
                                                                        let _e4191 = global_6.member[_e4184];
                                                                        let _e4194 = global_6.member[(_e4184 + 1u)];
                                                                        let _e4197 = global_6.member[(_e4184 + 2u)];
                                                                        let _e4198 = (1f - _e4180);
                                                                        let _e4204 = global_6.member[(_e4184 + 3u)];
                                                                        let _e4207 = global_6.member[(_e4184 + 4u)];
                                                                        let _e4210 = global_6.member[(_e4184 + 5u)];
                                                                        let _e4217 = (1f - _e4178);
                                                                        let _e4222 = global_6.member[_e4189];
                                                                        let _e4225 = global_6.member[(_e4189 + 1u)];
                                                                        let _e4228 = global_6.member[(_e4189 + 2u)];
                                                                        let _e4234 = global_6.member[(_e4189 + 3u)];
                                                                        let _e4237 = global_6.member[(_e4189 + 4u)];
                                                                        let _e4240 = global_6.member[(_e4189 + 5u)];
                                                                        phi_16799_ = vec3<f32>(((((_e4191 * _e4198) + (_e4204 * _e4180)) * _e4217) + (((_e4222 * _e4198) + (_e4234 * _e4180)) * _e4178)), ((((_e4194 * _e4198) + (_e4207 * _e4180)) * _e4217) + (((_e4225 * _e4198) + (_e4237 * _e4180)) * _e4178)), ((((_e4197 * _e4198) + (_e4210 * _e4180)) * _e4217) + (((_e4228 * _e4198) + (_e4240 * _e4180)) * _e4178)));
                                                                    } else {
                                                                        phi_16799_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e4255 = phi_16799_;
                                                                    let _e4262 = ((_e4046 * 1.1f) * _e4255.y);
                                                                    phi_16801_ = vec3<f32>(exp(-((((_e4037 * _e4255.x) + _e4262) + (_e4049 * _e4255.z)))), exp(-((((_e4040 * _e4255.x) + _e4262) + (_e4052 * _e4255.z)))), exp(-((((_e4043 * _e4255.x) + _e4262) + (_e4055 * _e4255.z)))));
                                                                }
                                                                let _e4281 = phi_16801_;
                                                                let _e4292 = (((1f - (0.6f * (1f - sqrt(select(_e3991, 0f, _e3995))))) * 1.25f) / _e3378);
                                                                phi_16828_ = vec3<f32>(((_e3998.x * _e4281.x) * _e4292), ((_e3998.y * _e4281.y) * _e4292), ((_e3998.z * _e4281.z) * _e4292));
                                                            }
                                                            let _e4298 = phi_16828_;
                                                            phi_16829_ = _e4298;
                                                        } else {
                                                            phi_16829_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e4300 = phi_16829_;
                                                        phi_35663_ = _e3918;
                                                        phi_16830_ = _e3980;
                                                        phi_16831_ = vec3<f32>(_e3956, _e3957, _e3958);
                                                        phi_16832_ = _e4300;
                                                    }
                                                    let _e4588 = phi_35663_;
                                                    let _e4590 = phi_16830_;
                                                    let _e4592 = phi_16831_;
                                                    let _e4594 = phi_16832_;
                                                    phi_35662_ = _e4588;
                                                    phi_16833_ = _e4590;
                                                    phi_16834_ = _e4592;
                                                    phi_16835_ = _e4594;
                                                }
                                                let _e4596 = phi_35662_;
                                                let _e4598 = phi_16833_;
                                                let _e4600 = phi_16834_;
                                                let _e4602 = phi_16835_;
                                                phi_35619_ = _e4596;
                                                phi_17725_ = _e4598;
                                                phi_17726_ = _e4600;
                                                phi_17727_ = _e4602;
                                                phi_17728_ = 1000000000000000000000000000000f;
                                            } else {
                                                if (_e2625 == (_e2615 + 1u)) {
                                                    if ((_e364 & 1u) == 0u) {
                                                        phi_35628_ = _e2619;
                                                        phi_17359_ = 0f;
                                                        phi_17360_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17361_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17362_ = 1000000000000000000000000000000f;
                                                    } else {
                                                        if (_e367 > 0u) {
                                                            let _e2856 = global_1.member.member_2[1u];
                                                            let _e2858 = ((_e2619 * 747796405u) + 2891336453u);
                                                            let _e2866 = (((_e2858 >> bitcast<u32>((((_e2858 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2858) * 277803737u);
                                                            let _e2869 = ((_e2866 >> bitcast<u32>(22i)) ^ _e2866);
                                                            phi_16856_ = 0u;
                                                            phi_16859_ = _e367;
                                                            loop {
                                                                let _e2875 = phi_16856_;
                                                                let _e2877 = phi_16859_;
                                                                local_85 = _e2875;
                                                                local_86 = _e2875;
                                                                let _e2879 = ((_e2875 + 1u) < _e2877);
                                                                if _e2879 {
                                                                    let _e2881 = ((_e2875 + _e2877) / 2u);
                                                                    let _e2884 = global_6.member[(_e2856 + _e2881)];
                                                                    let _e2885 = (_e2884 <= (f32((_e2869 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                    phi_16857_ = select(_e2875, _e2881, _e2885);
                                                                    phi_16860_ = select(_e2881, _e2877, _e2885);
                                                                } else {
                                                                    phi_16857_ = u32();
                                                                    phi_16860_ = u32();
                                                                }
                                                                let _e2889 = phi_16857_;
                                                                let _e2891 = phi_16860_;
                                                                phi_16856_ = _e2889;
                                                                phi_16859_ = _e2891;
                                                                if !(_e2879) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2894 = local_85;
                                                            let _e2895 = (_e2856 + _e2894);
                                                            let _e2898 = global_6.member[(_e2895 + 1u)];
                                                            let _e2900 = global_6.member[_e2895];
                                                            let _e2904 = global_1.member.member_2[0u];
                                                            let _e2906 = local_86;
                                                            let _e2909 = global_6.member[(_e2904 + _e2906)];
                                                            let _e2912 = ((_e2869 * 747796405u) + 2891336453u);
                                                            let _e2920 = (((_e2912 >> bitcast<u32>((((_e2912 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2912) * 277803737u);
                                                            let _e2923 = ((_e2920 >> bitcast<u32>(22i)) ^ _e2920);
                                                            let _e2926 = f32((_e2923 >> bitcast<u32>(8i)));
                                                            let _e2927 = (_e2926 * 0.000000059604645f);
                                                            let _e2929 = ((_e2923 * 747796405u) + 2891336453u);
                                                            let _e2937 = (((_e2929 >> bitcast<u32>((((_e2929 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2929) * 277803737u);
                                                            let _e2940 = ((_e2937 >> bitcast<u32>(22i)) ^ _e2937);
                                                            let _e2943 = f32((_e2940 >> bitcast<u32>(8i)));
                                                            let _e2946 = global_2.member[bitcast<u32>(_e2909)];
                                                            let _e2949 = (_e2946.member_6.x == 1u);
                                                            if _e2949 {
                                                                let _e3091 = (_e2946.member.x - _e1737);
                                                                let _e3092 = (_e2946.member.y - _e1738);
                                                                let _e3093 = (_e2946.member.z - _e1739);
                                                                let _e3098 = (((_e3091 * _e3091) + (_e3092 * _e3092)) + (_e3093 * _e3093));
                                                                let _e3099 = (_e2946.member.w * _e2946.member.w);
                                                                let _e3101 = (_e3098 <= (_e3099 * 1.000001f));
                                                                if _e3101 {
                                                                    let _e3210 = (_e2926 * 0.00000037450704f);
                                                                    let _e3212 = ((_e2943 * 0.00000011920929f) - 1f);
                                                                    let _e3214 = (1f - (_e3212 * _e3212));
                                                                    if (_e3214 != _e3214) {
                                                                        phi_27335_ = true;
                                                                    } else {
                                                                        phi_27335_ = (0f >= _e3214);
                                                                    }
                                                                    let _e3218 = phi_27335_;
                                                                    let _e3220 = sqrt(select(_e3214, 0f, _e3218));
                                                                    let _e3222 = (_e3220 * cos(_e3210));
                                                                    let _e3224 = (_e3220 * sin(_e3210));
                                                                    let _e3229 = (_e2946.member.x + (_e3222 * _e2946.member.w));
                                                                    let _e3230 = (_e2946.member.y + (_e3224 * _e2946.member.w));
                                                                    let _e3231 = (_e2946.member.z + (_e3212 * _e2946.member.w));
                                                                    if _e3101 {
                                                                        let _e3244 = (_e3229 - _e1737);
                                                                        let _e3245 = (_e3230 - _e1738);
                                                                        let _e3246 = (_e3231 - _e1739);
                                                                        let _e3251 = (((_e3244 * _e3244) + (_e3245 * _e3245)) + (_e3246 * _e3246));
                                                                        phi_27404_ = (_e3251 / (((abs((((_e3222 * _e3244) + (_e3224 * _e3245)) + (_e3212 * _e3246))) / sqrt(_e3251)) * 12.566371f) * _e3099));
                                                                    } else {
                                                                        let _e3233 = (_e3099 / _e3098);
                                                                        if (_e3233 != _e3233) {
                                                                            phi_27423_ = true;
                                                                        } else {
                                                                            phi_27423_ = (1f <= _e3233);
                                                                        }
                                                                        let _e3237 = phi_27423_;
                                                                        let _e3238 = select(_e3233, 1f, _e3237);
                                                                        phi_27404_ = (0.15915494f / (_e3238 / (1f + sqrt((1f - _e3238)))));
                                                                    }
                                                                    let _e3264 = phi_27404_;
                                                                    phi_17072_ = type_74(vec3<f32>(_e3229, _e3230, _e3231), vec3<f32>(_e3222, _e3224, _e3212), _e3264);
                                                                } else {
                                                                    let _e3102 = (_e3099 / _e3098);
                                                                    let _e3103 = (_e3102 != _e3102);
                                                                    if _e3103 {
                                                                        phi_27187_ = true;
                                                                    } else {
                                                                        phi_27187_ = (1f <= _e3102);
                                                                    }
                                                                    let _e3106 = phi_27187_;
                                                                    let _e3107 = select(_e3102, 1f, _e3106);
                                                                    if _e3103 {
                                                                        phi_27210_ = true;
                                                                    } else {
                                                                        phi_27210_ = (1f <= _e3102);
                                                                    }
                                                                    let _e3111 = phi_27210_;
                                                                    let _e3112 = select(_e3102, 1f, _e3111);
                                                                    let _e3116 = (_e3112 / (1f + sqrt((1f - _e3112))));
                                                                    if (_e3107 < 0.00068523f) {
                                                                        phi_17007_ = (_e3107 * _e2927);
                                                                    } else {
                                                                        let _e3119 = (1f - (_e2927 * _e3116));
                                                                        let _e3121 = (1f - (_e3119 * _e3119));
                                                                        if (_e3121 != _e3121) {
                                                                            phi_27225_ = true;
                                                                        } else {
                                                                            phi_27225_ = (0f >= _e3121);
                                                                        }
                                                                        let _e3125 = phi_27225_;
                                                                        phi_17007_ = select(_e3121, 0f, _e3125);
                                                                    }
                                                                    let _e3129 = phi_17007_;
                                                                    let _e3130 = (1f - _e3129);
                                                                    if (_e3130 != _e3130) {
                                                                        phi_27240_ = true;
                                                                    } else {
                                                                        phi_27240_ = (0f >= _e3130);
                                                                    }
                                                                    let _e3134 = phi_27240_;
                                                                    let _e3139 = (1f - (_e3129 / _e3107));
                                                                    if (_e3139 != _e3139) {
                                                                        phi_27255_ = true;
                                                                    } else {
                                                                        phi_27255_ = (0f >= _e3139);
                                                                    }
                                                                    let _e3143 = phi_27255_;
                                                                    let _e3147 = ((_e3129 / sqrt(_e3107)) + (sqrt(select(_e3130, 0f, _e3134)) * sqrt(select(_e3139, 0f, _e3143))));
                                                                    let _e3149 = (1f - (_e3147 * _e3147));
                                                                    if (_e3149 != _e3149) {
                                                                        phi_27270_ = true;
                                                                    } else {
                                                                        phi_27270_ = (0f >= _e3149);
                                                                    }
                                                                    let _e3153 = phi_27270_;
                                                                    let _e3155 = sqrt(select(_e3149, 0f, _e3153));
                                                                    let _e3156 = (_e2943 * 0.00000037450704f);
                                                                    let _e3157 = sqrt(_e3098);
                                                                    let _e3158 = (_e3091 / _e3157);
                                                                    let _e3159 = (_e3092 / _e3157);
                                                                    let _e3160 = (_e3093 / _e3157);
                                                                    let _e3162 = select(-1f, 1f, (_e3160 >= 0f));
                                                                    let _e3164 = (-1f / (_e3162 + _e3160));
                                                                    let _e3166 = ((_e3158 * _e3159) * _e3164);
                                                                    let _e3179 = (_e3155 * cos(_e3156));
                                                                    let _e3184 = (_e3155 * sin(_e3156));
                                                                    let _e3197 = -(((((1f + (((_e3162 * _e3158) * _e3158) * _e3164)) * _e3179) + (_e3166 * _e3184)) + (_e3158 * _e3147)));
                                                                    let _e3198 = -(((((_e3162 * _e3166) * _e3179) + ((_e3162 + ((_e3159 * _e3159) * _e3164)) * _e3184)) + (_e3159 * _e3147)));
                                                                    let _e3199 = -(((((-(_e3162) * _e3158) * _e3179) + (-(_e3159) * _e3184)) + (_e3160 * _e3147)));
                                                                    phi_17072_ = type_74(vec3<f32>((_e2946.member.x + (_e3197 * _e2946.member.w)), (_e2946.member.y + (_e3198 * _e2946.member.w)), (_e2946.member.z + (_e3199 * _e2946.member.w))), vec3<f32>(_e3197, _e3198, _e3199), (0.15915494f / _e3116));
                                                                }
                                                                let _e3267 = phi_17072_;
                                                                phi_17258_ = _e3267;
                                                            } else {
                                                                let _e2950 = sqrt(_e2927);
                                                                let _e2951 = (1f - _e2950);
                                                                let _e2952 = ((_e2943 * 0.000000059604645f) * _e2950);
                                                                let _e2974 = (_e2950 - _e2952);
                                                                let _e2978 = (((_e2946.member.x * _e2951) + (_e2946.member_1.x * _e2952)) + (_e2946.member_2.x * _e2974));
                                                                let _e2979 = (((_e2946.member.y * _e2951) + (_e2946.member_1.y * _e2952)) + (_e2946.member_2.y * _e2974));
                                                                let _e2980 = (((_e2946.member.z * _e2951) + (_e2946.member_1.z * _e2952)) + (_e2946.member_2.z * _e2974));
                                                                let _e2982 = (_e2946.member_1.x - _e2946.member.x);
                                                                let _e2983 = (_e2946.member_1.y - _e2946.member.y);
                                                                let _e2984 = (_e2946.member_1.z - _e2946.member.z);
                                                                let _e2985 = (_e2946.member_2.x - _e2946.member.x);
                                                                let _e2986 = (_e2946.member_2.y - _e2946.member.y);
                                                                let _e2987 = (_e2946.member_2.z - _e2946.member.z);
                                                                let _e2990 = ((_e2983 * _e2987) - (_e2986 * _e2984));
                                                                let _e2993 = ((_e2984 * _e2985) - (_e2987 * _e2982));
                                                                let _e2996 = ((_e2982 * _e2986) - (_e2985 * _e2983));
                                                                let _e3002 = sqrt((((_e2990 * _e2990) + (_e2993 * _e2993)) + (_e2996 * _e2996)));
                                                                let _e3003 = (1f / _e3002);
                                                                let _e3004 = (_e2990 * _e3003);
                                                                let _e3005 = (_e2993 * _e3003);
                                                                let _e3006 = (_e2996 * _e3003);
                                                                if _e2949 {
                                                                    let _e3039 = (_e2946.member.x - _e1737);
                                                                    let _e3040 = (_e2946.member.y - _e1738);
                                                                    let _e3041 = (_e2946.member.z - _e1739);
                                                                    let _e3046 = (((_e3039 * _e3039) + (_e3040 * _e3040)) + (_e3041 * _e3041));
                                                                    let _e3047 = (_e2946.member.w * _e2946.member.w);
                                                                    if (_e3046 <= (_e3047 * 1.000001f)) {
                                                                        let _e3061 = (_e2978 - _e1737);
                                                                        let _e3062 = (_e2979 - _e1738);
                                                                        let _e3063 = (_e2980 - _e1739);
                                                                        let _e3068 = (((_e3061 * _e3061) + (_e3062 * _e3062)) + (_e3063 * _e3063));
                                                                        phi_27153_ = (_e3068 / (((abs((((_e3004 * _e3061) + (_e3005 * _e3062)) + (_e3006 * _e3063))) / sqrt(_e3068)) * 12.566371f) * _e3047));
                                                                    } else {
                                                                        let _e3050 = (_e3047 / _e3046);
                                                                        if (_e3050 != _e3050) {
                                                                            phi_27172_ = true;
                                                                        } else {
                                                                            phi_27172_ = (1f <= _e3050);
                                                                        }
                                                                        let _e3054 = phi_27172_;
                                                                        let _e3055 = select(_e3050, 1f, _e3054);
                                                                        phi_27153_ = (0.15915494f / (_e3055 / (1f + sqrt((1f - _e3055)))));
                                                                    }
                                                                    let _e3081 = phi_27153_;
                                                                    phi_17256_ = _e3081;
                                                                } else {
                                                                    let _e3008 = (_e2978 - _e1737);
                                                                    let _e3009 = (_e2979 - _e1738);
                                                                    let _e3010 = (_e2980 - _e1739);
                                                                    let _e3015 = (((_e3008 * _e3008) + (_e3009 * _e3009)) + (_e3010 * _e3010));
                                                                    let _e3023 = (abs((((_e3004 * _e3008) + (_e3005 * _e3009)) + (_e3006 * _e3010))) / sqrt(_e3015));
                                                                    if (_e3023 <= 0f) {
                                                                        phi_17255_ = 0f;
                                                                    } else {
                                                                        if (_e2946.member_6.x == 0u) {
                                                                            phi_17252_ = (_e3002 * 0.5f);
                                                                        } else {
                                                                            phi_17252_ = ((12.566371f * _e2946.member.w) * _e2946.member.w);
                                                                        }
                                                                        let _e3032 = phi_17252_;
                                                                        phi_17255_ = (_e3015 / (_e3023 * _e3032));
                                                                    }
                                                                    let _e3036 = phi_17255_;
                                                                    phi_17256_ = _e3036;
                                                                }
                                                                let _e3083 = phi_17256_;
                                                                phi_17258_ = type_74(vec3<f32>(_e2978, _e2979, _e2980), vec3<f32>(_e3004, _e3005, _e3006), _e3083);
                                                            }
                                                            let _e3269 = phi_17258_;
                                                            let _e3274 = (_e3269.member.x - _e1737);
                                                            let _e3276 = (_e3269.member.y - _e1738);
                                                            let _e3278 = (_e3269.member.z - _e1739);
                                                            let _e3284 = sqrt((((_e3274 * _e3274) + (_e3276 * _e3276)) + (_e3278 * _e3278)));
                                                            let _e3285 = (_e3274 / _e3284);
                                                            let _e3286 = (_e3276 / _e3284);
                                                            let _e3287 = (_e3278 / _e3284);
                                                            if (_e3269.member_2 > 0f) {
                                                                let _e3299 = global_4.member[_e2946.member_6.y];
                                                                if (_e3299.member == 4u) {
                                                                    if (_e3299.member_1 == 1u) {
                                                                        phi_17336_ = true;
                                                                    } else {
                                                                        phi_17336_ = ((((-(_e3285) * _e3269.member_1.x) + (-(_e3286) * _e3269.member_1.y)) + (-(_e3287) * _e3269.member_1.z)) > 0f);
                                                                    }
                                                                    let _e3314 = phi_17336_;
                                                                    if _e3314 {
                                                                        phi_17345_ = vec3<f32>(_e3299.member_4.x, _e3299.member_4.y, _e3299.member_4.z);
                                                                    } else {
                                                                        phi_17345_ = vec3<f32>();
                                                                    }
                                                                    let _e3321 = phi_17345_;
                                                                    phi_17347_ = _e3321;
                                                                    phi_17348_ = select(true, false, _e3314);
                                                                } else {
                                                                    phi_17347_ = vec3<f32>();
                                                                    phi_17348_ = true;
                                                                }
                                                                let _e3324 = phi_17347_;
                                                                let _e3326 = phi_17348_;
                                                                phi_17353_ = ((_e2898 - _e2900) * _e3269.member_2);
                                                                phi_17354_ = select(_e3324, vec3<f32>(0f, 0f, 0f), vec3(_e3326));
                                                            } else {
                                                                phi_17353_ = 0f;
                                                                phi_17354_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3330 = phi_17353_;
                                                            let _e3332 = phi_17354_;
                                                            phi_35629_ = _e2940;
                                                            phi_17355_ = _e3330;
                                                            phi_17356_ = vec3<f32>(_e3285, _e3286, _e3287);
                                                            phi_17357_ = _e3332;
                                                            phi_17358_ = ((_e3284 * 0.9999f) - _e1730);
                                                        } else {
                                                            phi_35629_ = _e2619;
                                                            phi_17355_ = 0f;
                                                            phi_17356_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17357_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17358_ = 1000000000000000000000000000000f;
                                                        }
                                                        let _e3334 = phi_35629_;
                                                        let _e3336 = phi_17355_;
                                                        let _e3338 = phi_17356_;
                                                        let _e3340 = phi_17357_;
                                                        let _e3342 = phi_17358_;
                                                        phi_35628_ = _e3334;
                                                        phi_17359_ = _e3336;
                                                        phi_17360_ = _e3338;
                                                        phi_17361_ = _e3340;
                                                        phi_17362_ = _e3342;
                                                    }
                                                    let _e3344 = phi_35628_;
                                                    let _e3346 = phi_17359_;
                                                    let _e3348 = phi_17360_;
                                                    let _e3350 = phi_17361_;
                                                    let _e3352 = phi_17362_;
                                                    phi_35620_ = _e3344;
                                                    phi_17721_ = _e3346;
                                                    phi_17722_ = _e3348;
                                                    phi_17723_ = _e3350;
                                                    phi_17724_ = _e3352;
                                                } else {
                                                    if ((_e364 & 2u) == 0u) {
                                                        phi_35621_ = _e2619;
                                                        phi_17718_ = 0f;
                                                        phi_17719_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17720_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2637 = ((_e2619 * 747796405u) + 2891336453u);
                                                        let _e2645 = (((_e2637 >> bitcast<u32>((((_e2637 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2637) * 277803737u);
                                                        let _e2648 = ((_e2645 >> bitcast<u32>(22i)) ^ _e2645);
                                                        let _e2652 = (f32((_e2648 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2654 = ((_e2648 * 747796405u) + 2891336453u);
                                                        let _e2662 = (((_e2654 >> bitcast<u32>((((_e2654 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2654) * 277803737u);
                                                        let _e2665 = ((_e2662 >> bitcast<u32>(22i)) ^ _e2662);
                                                        let _e2669 = (f32((_e2665 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2672 = global_1.member.member_3[1u];
                                                        let _e2675 = global_1.member.member_3[2u];
                                                        let _e2678 = global_1.member.member_2[3u];
                                                        phi_17383_ = 0u;
                                                        phi_17386_ = _e2675;
                                                        loop {
                                                            let _e2680 = phi_17383_;
                                                            let _e2682 = phi_17386_;
                                                            local_80 = _e2680;
                                                            local_83 = _e2680;
                                                            local_84 = _e2680;
                                                            let _e2684 = ((_e2680 + 1u) < _e2682);
                                                            if _e2684 {
                                                                let _e2686 = ((_e2680 + _e2682) / 2u);
                                                                let _e2689 = global_6.member[(_e2678 + _e2686)];
                                                                let _e2690 = (_e2689 <= _e2652);
                                                                phi_17384_ = select(_e2680, _e2686, _e2690);
                                                                phi_17387_ = select(_e2686, _e2682, _e2690);
                                                            } else {
                                                                phi_17384_ = u32();
                                                                phi_17387_ = u32();
                                                            }
                                                            let _e2694 = phi_17384_;
                                                            let _e2696 = phi_17387_;
                                                            phi_17383_ = _e2694;
                                                            phi_17386_ = _e2696;
                                                            if !(_e2684) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2700 = global_1.member.member_3[0u];
                                                        let _e2701 = (_e2672 + 1u);
                                                        let _e2703 = local_80;
                                                        let _e2705 = (_e2700 + (_e2703 * _e2701));
                                                        phi_17424_ = 0u;
                                                        phi_17427_ = _e2672;
                                                        loop {
                                                            let _e2707 = phi_17424_;
                                                            let _e2709 = phi_17427_;
                                                            local_81 = _e2707;
                                                            local_82 = _e2707;
                                                            let _e2711 = ((_e2707 + 1u) < _e2709);
                                                            if _e2711 {
                                                                let _e2713 = ((_e2707 + _e2709) / 2u);
                                                                let _e2716 = global_6.member[(_e2705 + _e2713)];
                                                                let _e2717 = (_e2716 <= _e2669);
                                                                phi_17425_ = select(_e2707, _e2713, _e2717);
                                                                phi_17428_ = select(_e2713, _e2709, _e2717);
                                                            } else {
                                                                phi_17425_ = u32();
                                                                phi_17428_ = u32();
                                                            }
                                                            let _e2721 = phi_17425_;
                                                            let _e2723 = phi_17428_;
                                                            phi_17424_ = _e2721;
                                                            phi_17427_ = _e2723;
                                                            if !(_e2711) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2726 = local_81;
                                                        let _e2729 = local_82;
                                                        let _e2730 = (_e2705 + _e2729);
                                                        let _e2732 = global_6.member[_e2730];
                                                        let _e2735 = global_6.member[(_e2730 + 1u)];
                                                        if (_e2735 > _e2732) {
                                                            let _e2739 = ((_e2669 - _e2732) / (_e2735 - _e2732));
                                                            let _e2741 = select(_e2739, 0f, (_e2739 < 0f));
                                                            phi_17504_ = select(_e2741, 0.99999f, (_e2741 > 0.99999f));
                                                        } else {
                                                            phi_17504_ = 0.5f;
                                                        }
                                                        let _e2745 = phi_17504_;
                                                        let _e2747 = f32(_e2672);
                                                        let _e2750 = local_83;
                                                        let _e2753 = local_84;
                                                        let _e2754 = (_e2678 + _e2753);
                                                        let _e2756 = global_6.member[_e2754];
                                                        let _e2759 = global_6.member[(_e2754 + 1u)];
                                                        if (_e2759 > _e2756) {
                                                            let _e2763 = ((_e2652 - _e2756) / (_e2759 - _e2756));
                                                            let _e2765 = select(_e2763, 0f, (_e2763 < 0f));
                                                            phi_17553_ = select(_e2765, 0.99999f, (_e2765 > 0.99999f));
                                                        } else {
                                                            phi_17553_ = 0.5f;
                                                        }
                                                        let _e2769 = phi_17553_;
                                                        let _e2771 = f32(_e2675);
                                                        let _e2774 = ((((f32(_e2726) + _e2745) / _e2747) - 0.5f) * 6.2831855f);
                                                        let _e2775 = (((f32(_e2750) + _e2769) / _e2771) * 3.1415927f);
                                                        let _e2776 = sin(_e2775);
                                                        let _e2778 = (_e2776 * cos(_e2774));
                                                        let _e2779 = cos(_e2775);
                                                        let _e2781 = (_e2776 * sin(_e2774));
                                                        let _e2784 = ((atan2(_e2781, _e2778) * 0.15915494f) + 0.5f);
                                                        let _e2788 = select(_e2779, -1f, (_e2779 < -1f));
                                                        let _e2791 = acos(select(_e2788, 1f, (_e2788 > 1f)));
                                                        let _e2793 = ((_e2784 - floor(_e2784)) * _e2747);
                                                        let _e2798 = select(select(u32(_e2793), 0u, (_e2793 < 0f)), 4294967295u, (_e2793 > 4294967000f));
                                                        let _e2799 = (_e2672 - 1u);
                                                        let _e2802 = ((_e2791 * 0.31830987f) * _e2771);
                                                        let _e2807 = select(select(u32(_e2802), 0u, (_e2802 < 0f)), 4294967295u, (_e2802 > 4294967000f));
                                                        let _e2808 = (_e2675 - 1u);
                                                        let _e2810 = select(_e2807, _e2808, (_e2808 < _e2807));
                                                        let _e2811 = sin(_e2791);
                                                        if (_e2811 <= 0f) {
                                                            phi_17709_ = 0f;
                                                        } else {
                                                            let _e2815 = (_e2678 + _e2810);
                                                            let _e2818 = global_6.member[(_e2815 + 1u)];
                                                            let _e2820 = global_6.member[_e2815];
                                                            let _e2823 = ((_e2700 + (_e2810 * _e2701)) + select(_e2798, _e2799, (_e2799 < _e2798)));
                                                            let _e2826 = global_6.member[(_e2823 + 1u)];
                                                            let _e2828 = global_6.member[_e2823];
                                                            phi_17709_ = ((((_e2818 - _e2820) * _e2771) * ((_e2826 - _e2828) * _e2747)) / (19.73921f * _e2811));
                                                        }
                                                        let _e2835 = phi_17709_;
                                                        let _e2836 = (_e2835 > 0f);
                                                        if _e2836 {
                                                            phi_17716_ = vec3<f32>(_e2778, _e2779, _e2781);
                                                        } else {
                                                            phi_17716_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2839 = phi_17716_;
                                                        phi_35621_ = _e2665;
                                                        phi_17718_ = select(0f, _e2835, _e2836);
                                                        phi_17719_ = _e2839;
                                                        phi_17720_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2836));
                                                    }
                                                    let _e2844 = phi_35621_;
                                                    let _e2846 = phi_17718_;
                                                    let _e2848 = phi_17719_;
                                                    let _e2850 = phi_17720_;
                                                    phi_35620_ = _e2844;
                                                    phi_17721_ = _e2846;
                                                    phi_17722_ = _e2848;
                                                    phi_17723_ = _e2850;
                                                    phi_17724_ = 1000000000000000000000000000000f;
                                                }
                                                let _e3354 = phi_35620_;
                                                let _e3356 = phi_17721_;
                                                let _e3358 = phi_17722_;
                                                let _e3360 = phi_17723_;
                                                let _e3362 = phi_17724_;
                                                phi_35619_ = _e3354;
                                                phi_17725_ = _e3356;
                                                phi_17726_ = _e3358;
                                                phi_17727_ = _e3360;
                                                phi_17728_ = _e3362;
                                            }
                                            let _e4604 = phi_35619_;
                                            let _e4606 = phi_17725_;
                                            let _e4608 = phi_17726_;
                                            let _e4610 = phi_17727_;
                                            let _e4612 = phi_17728_;
                                            phi_35618_ = _e4604;
                                            phi_17729_ = _e4606;
                                            phi_17730_ = _e4608;
                                            phi_17731_ = _e4610;
                                            phi_17732_ = _e4612;
                                        }
                                        let _e4681 = phi_35618_;
                                        let _e4683 = phi_17729_;
                                        let _e4685 = phi_17730_;
                                        let _e4687 = phi_17731_;
                                        let _e4689 = phi_17732_;
                                        if ((((_e4687.x * _e4687.x) + (_e4687.y * _e4687.y)) + (_e4687.z * _e4687.z)) > 0f) {
                                            let _e4706 = (((_e4685.x * _e1712.x) + (_e4685.y * _e1712.y)) + (_e4685.z * _e1712.z));
                                            if (_e4706 != _e4706) {
                                                phi_28737_ = true;
                                            } else {
                                                phi_28737_ = (0f >= _e4706);
                                            }
                                            let _e4710 = phi_28737_;
                                            let _e4712 = (select(_e4706, 0f, _e4710) * 0.31830987f);
                                            if (_e2604 == 0u) {
                                                if (_e4706 <= 0f) {
                                                    phi_29534_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_29534_ = (_e2598 * (_e4706 * 0.31830987f));
                                                }
                                                let _e5188 = phi_29534_;
                                                phi_18128_ = type_56(_e5188, _e4712);
                                                phi_18129_ = type_56();
                                                phi_18130_ = true;
                                            } else {
                                                let _e4718 = (((_e1684 * _e1760) + (_e1685 * _e1761)) + (_e1686 * _e1763));
                                                let _e4723 = (((_e1684 * _e1756) + (_e1685 * _e1766)) + (_e1686 * _e1767));
                                                let _e4728 = (((_e1684 * _e1712.x) + (_e1685 * _e1712.y)) + (_e1686 * _e1712.z));
                                                let _e4733 = (((_e4685.x * _e1760) + (_e4685.y * _e1761)) + (_e4685.z * _e1763));
                                                let _e4738 = (((_e4685.x * _e1756) + (_e4685.y * _e1766)) + (_e4685.z * _e1767));
                                                if (_e4728 <= 0f) {
                                                    phi_18092_ = type_56();
                                                    phi_18093_ = true;
                                                } else {
                                                    let _e4740 = (_e4706 <= 0f);
                                                    if _e4740 {
                                                        phi_18091_ = type_56();
                                                    } else {
                                                        let _e4741 = (_e4718 + _e4733);
                                                        let _e4742 = (_e4723 + _e4738);
                                                        let _e4743 = (_e4728 + _e4706);
                                                        let _e4750 = (1f / sqrt((((_e4741 * _e4741) + (_e4742 * _e4742)) + (_e4743 * _e4743))));
                                                        let _e4753 = (_e4743 * _e4750);
                                                        if (_e4753 <= 0f) {
                                                            phi_28847_ = 0f;
                                                        } else {
                                                            let _e4755 = (_e2602 * _e2602);
                                                            let _e4759 = ((((_e4755 - 1f) * _e4753) * _e4753) + 1f);
                                                            phi_28847_ = (_e4755 / ((3.1415927f * _e4759) * _e4759));
                                                        }
                                                        let _e4764 = phi_28847_;
                                                        let _e4765 = (_e4728 * _e4728);
                                                        if (_e4765 >= 1f) {
                                                            phi_28875_ = 0f;
                                                        } else {
                                                            phi_28875_ = ((sqrt((1f + (((_e2602 * _e2602) * ((_e4718 * _e4718) + (_e4723 * _e4723))) / _e4765))) - 1f) * 0.5f);
                                                        }
                                                        let _e4778 = phi_28875_;
                                                        let _e4779 = (1f + _e4778);
                                                        let _e4780 = (_e4706 * _e4706);
                                                        if (_e4780 >= 1f) {
                                                            phi_28903_ = 0f;
                                                        } else {
                                                            phi_28903_ = ((sqrt((1f + (((_e2602 * _e2602) * ((_e4733 * _e4733) + (_e4738 * _e4738))) / _e4780))) - 1f) * 0.5f);
                                                        }
                                                        let _e4793 = phi_28903_;
                                                        let _e4800 = (((_e4718 * (_e4741 * _e4750)) + (_e4723 * (_e4742 * _e4750))) + (_e4728 * _e4753));
                                                        if (_e2600 == 0u) {
                                                            let _e5095 = (1f - _e4800);
                                                            if (_e5095 != _e5095) {
                                                                phi_29406_ = true;
                                                            } else {
                                                                phi_29406_ = (0f >= _e5095);
                                                            }
                                                            let _e5099 = phi_29406_;
                                                            let _e5101 = pow(select(_e5095, 0f, _e5099), 5f);
                                                            let _e5102 = (1f - _e5101);
                                                            phi_18079_ = vec3<f32>(((_e2598.x * _e5102) + _e5101), ((_e2598.y * _e5102) + _e5101), ((_e2598.z * _e5102) + _e5101));
                                                        } else {
                                                            if (_e2600 == 1u) {
                                                                let _e5068 = (1f / _e2594);
                                                                let _e5072 = ((_e5068 * _e5068) * (1f - (_e4800 * _e4800)));
                                                                if (_e5072 >= 1f) {
                                                                    phi_29367_ = 1f;
                                                                } else {
                                                                    let _e5075 = sqrt((1f - _e5072));
                                                                    if (_e4800 != _e4800) {
                                                                        phi_29378_ = true;
                                                                    } else {
                                                                        phi_29378_ = (_e5075 <= _e4800);
                                                                    }
                                                                    let _e5079 = phi_29378_;
                                                                    let _e5084 = pow(((1f - _e2594) / (1f + _e2594)), 2f);
                                                                    phi_29367_ = (_e5084 + ((1f - _e5084) * pow((1f - select(_e4800, _e5075, _e5079)), 5f)));
                                                                }
                                                                let _e5091 = phi_29367_;
                                                                phi_18078_ = vec3(_e5091);
                                                            } else {
                                                                let _e4804 = select(_e4800, 0f, (_e4800 < 0f));
                                                                let _e4806 = select(_e4804, 1f, (_e4804 > 1f));
                                                                let _e4808 = (1f - (_e4806 * _e4806));
                                                                let _e4821 = ((_e2598.x * _e2598.x) - (_e2596.x * _e2596.x));
                                                                let _e4822 = ((_e2598.y * _e2598.y) - (_e2596.y * _e2596.y));
                                                                let _e4823 = ((_e2598.z * _e2598.z) - (_e2596.z * _e2596.z));
                                                                let _e4825 = (_e2598.x * (_e2596.x + _e2596.x));
                                                                let _e4827 = (_e2598.y * (_e2596.y + _e2596.y));
                                                                let _e4829 = (_e2598.z * (_e2596.z + _e2596.z));
                                                                let _e4836 = ((_e4821 * _e4821) + (_e4825 * _e4825));
                                                                let _e4837 = ((_e4822 * _e4822) + (_e4827 * _e4827));
                                                                let _e4838 = ((_e4823 * _e4823) + (_e4829 * _e4829));
                                                                let _e4843 = ((_e4808 * _e4821) / _e4836);
                                                                let _e4844 = ((_e4808 * _e4822) / _e4837);
                                                                let _e4845 = ((_e4808 * _e4823) / _e4838);
                                                                let _e4846 = (1f - _e4843);
                                                                let _e4847 = (1f - _e4844);
                                                                let _e4848 = (1f - _e4845);
                                                                let _e4850 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4843, _e4844, _e4845));
                                                                let _e4851 = (_e4808 * _e4825);
                                                                let _e4852 = (_e4808 * _e4827);
                                                                let _e4853 = (_e4808 * _e4829);
                                                                let _e4858 = (vec3<f32>(_e4851, _e4852, _e4853) / vec3<f32>(_e4836, _e4837, _e4838));
                                                                let _e4871 = ((_e4850.x * _e4850.x) + (_e4858.x * _e4858.x));
                                                                let _e4872 = ((_e4850.y * _e4850.y) + (_e4858.y * _e4858.y));
                                                                let _e4873 = ((_e4850.z * _e4850.z) + (_e4858.z * _e4858.z));
                                                                if (_e4871 != _e4871) {
                                                                    phi_29037_ = true;
                                                                } else {
                                                                    phi_29037_ = (0f >= _e4871);
                                                                }
                                                                let _e4877 = phi_29037_;
                                                                let _e4879 = sqrt(select(_e4871, 0f, _e4877));
                                                                if (_e4872 != _e4872) {
                                                                    phi_29052_ = true;
                                                                } else {
                                                                    phi_29052_ = (0f >= _e4872);
                                                                }
                                                                let _e4883 = phi_29052_;
                                                                let _e4885 = sqrt(select(_e4872, 0f, _e4883));
                                                                if (_e4873 != _e4873) {
                                                                    phi_29067_ = true;
                                                                } else {
                                                                    phi_29067_ = (0f >= _e4873);
                                                                }
                                                                let _e4889 = phi_29067_;
                                                                let _e4891 = sqrt(select(_e4873, 0f, _e4889));
                                                                let _e4895 = ((_e4879 + _e4846) * 0.5f);
                                                                let _e4896 = ((_e4885 + _e4847) * 0.5f);
                                                                let _e4897 = ((_e4891 + _e4848) * 0.5f);
                                                                if (_e4895 != _e4895) {
                                                                    phi_29094_ = true;
                                                                } else {
                                                                    phi_29094_ = (0f >= _e4895);
                                                                }
                                                                let _e4901 = phi_29094_;
                                                                let _e4903 = sqrt(select(_e4895, 0f, _e4901));
                                                                if (_e4896 != _e4896) {
                                                                    phi_29109_ = true;
                                                                } else {
                                                                    phi_29109_ = (0f >= _e4896);
                                                                }
                                                                let _e4907 = phi_29109_;
                                                                let _e4909 = sqrt(select(_e4896, 0f, _e4907));
                                                                if (_e4897 != _e4897) {
                                                                    phi_29124_ = true;
                                                                } else {
                                                                    phi_29124_ = (0f >= _e4897);
                                                                }
                                                                let _e4913 = phi_29124_;
                                                                let _e4915 = sqrt(select(_e4897, 0f, _e4913));
                                                                let _e4916 = vec3<f32>(_e4903, _e4909, _e4915);
                                                                let _e4920 = ((_e4879 - _e4846) * 0.5f);
                                                                let _e4921 = ((_e4885 - _e4847) * 0.5f);
                                                                let _e4922 = ((_e4891 - _e4848) * 0.5f);
                                                                if (_e4920 != _e4920) {
                                                                    phi_29151_ = true;
                                                                } else {
                                                                    phi_29151_ = (0f >= _e4920);
                                                                }
                                                                let _e4926 = phi_29151_;
                                                                let _e4928 = sqrt(select(_e4920, 0f, _e4926));
                                                                if (_e4921 != _e4921) {
                                                                    phi_29166_ = true;
                                                                } else {
                                                                    phi_29166_ = (0f >= _e4921);
                                                                }
                                                                let _e4932 = phi_29166_;
                                                                let _e4934 = sqrt(select(_e4921, 0f, _e4932));
                                                                if (_e4922 != _e4922) {
                                                                    phi_29181_ = true;
                                                                } else {
                                                                    phi_29181_ = (0f >= _e4922);
                                                                }
                                                                let _e4938 = phi_29181_;
                                                                let _e4940 = sqrt(select(_e4922, 0f, _e4938));
                                                                if ((_e4851 / _e4836) < 0f) {
                                                                    phi_17955_ = -(_e4928);
                                                                } else {
                                                                    phi_17955_ = _e4928;
                                                                }
                                                                let _e4944 = phi_17955_;
                                                                if ((_e4852 / _e4837) < 0f) {
                                                                    phi_17963_ = -(_e4934);
                                                                } else {
                                                                    phi_17963_ = _e4934;
                                                                }
                                                                let _e4948 = phi_17963_;
                                                                if ((_e4853 / _e4838) < 0f) {
                                                                    phi_17971_ = -(_e4940);
                                                                } else {
                                                                    phi_17971_ = _e4940;
                                                                }
                                                                let _e4952 = phi_17971_;
                                                                let _e4956 = (_e2596.x * _e4806);
                                                                let _e4957 = (_e2596.y * _e4806);
                                                                let _e4958 = (_e2596.z * _e4806);
                                                                let _e4968 = vec3<f32>(((_e2598.x * _e4903) - (_e2596.x * _e4944)), ((_e2598.y * _e4909) - (_e2596.y * _e4948)), ((_e2598.z * _e4915) - (_e2596.z * _e4952)));
                                                                let _e4975 = ((_e2598.x * _e4944) + (_e2596.x * _e4903));
                                                                let _e4976 = ((_e2598.y * _e4948) + (_e2596.y * _e4909));
                                                                let _e4977 = ((_e2598.z * _e4952) + (_e2596.z * _e4915));
                                                                let _e4979 = vec3<f32>((_e2598.x * _e4806), (_e2598.y * _e4806), (_e2598.z * _e4806));
                                                                let _e4980 = (_e4979 - _e4916);
                                                                let _e4981 = (_e4956 - _e4944);
                                                                let _e4982 = (_e4957 - _e4948);
                                                                let _e4983 = (_e4958 - _e4952);
                                                                let _e4996 = (_e4979 + _e4916);
                                                                let _e4997 = (_e4956 + _e4944);
                                                                let _e4998 = (_e4957 + _e4948);
                                                                let _e4999 = (_e4958 + _e4952);
                                                                let _e5009 = ((_e4996.x * _e4996.x) + (_e4997 * _e4997));
                                                                let _e5010 = ((_e4996.y * _e4996.y) + (_e4998 * _e4998));
                                                                let _e5011 = ((_e4996.z * _e4996.z) + (_e4999 * _e4999));
                                                                let _e5021 = vec3(_e4806);
                                                                let _e5022 = (_e5021 - _e4968);
                                                                let _e5023 = -(vec3<f32>(_e4975, _e4976, _e4977));
                                                                let _e5039 = (_e5021 + _e4968);
                                                                let _e5049 = ((_e5039.x * _e5039.x) + (_e4975 * _e4975));
                                                                let _e5050 = ((_e5039.y * _e5039.y) + (_e4976 * _e4976));
                                                                let _e5051 = ((_e5039.z * _e5039.z) + (_e4977 * _e4977));
                                                                phi_18078_ = vec3<f32>((((((_e4980.x * _e4980.x) + (_e4981 * _e4981)) / select(0.000000000000000000000000000001f, _e5009, (_e5009 > 0.000000000000000000000000000001f))) + (((_e5022.x * _e5022.x) + (_e5023.x * _e5023.x)) / select(0.000000000000000000000000000001f, _e5049, (_e5049 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4980.y * _e4980.y) + (_e4982 * _e4982)) / select(0.000000000000000000000000000001f, _e5010, (_e5010 > 0.000000000000000000000000000001f))) + (((_e5022.y * _e5022.y) + (_e5023.y * _e5023.y)) / select(0.000000000000000000000000000001f, _e5050, (_e5050 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4980.z * _e4980.z) + (_e4983 * _e4983)) / select(0.000000000000000000000000000001f, _e5011, (_e5011 > 0.000000000000000000000000000001f))) + (((_e5022.z * _e5022.z) + (_e5023.z * _e5023.z)) / select(0.000000000000000000000000000001f, _e5051, (_e5051 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                            }
                                                            let _e5094 = phi_18078_;
                                                            phi_18079_ = _e5094;
                                                        }
                                                        let _e5114 = phi_18079_;
                                                        let _e5116 = (4f * _e4728);
                                                        phi_18091_ = type_56((_e5114 * (((1f / (_e4779 + _e4793)) * _e4764) / _e5116)), ((_e4764 / _e4779) / _e5116));
                                                    }
                                                    let _e5123 = phi_18091_;
                                                    phi_18092_ = _e5123;
                                                    phi_18093_ = _e4740;
                                                }
                                                let _e5125 = phi_18092_;
                                                let _e5127 = phi_18093_;
                                                if _e5127 {
                                                    phi_18098_ = type_56(vec3<f32>(0f, 0f, 0f), 0f);
                                                } else {
                                                    phi_18098_ = _e5125;
                                                }
                                                let _e5129 = phi_18098_;
                                                let _e5132 = (_e2604 == 1u);
                                                if _e5132 {
                                                    phi_18126_ = _e5129;
                                                    phi_18127_ = type_56();
                                                } else {
                                                    if (_e4728 != _e4728) {
                                                        phi_29443_ = true;
                                                    } else {
                                                        phi_29443_ = (0f >= _e4728);
                                                    }
                                                    let _e5136 = phi_29443_;
                                                    let _e5137 = select(_e4728, 0f, _e5136);
                                                    let _e5138 = (1f / _e2594);
                                                    let _e5142 = ((_e5138 * _e5138) * (1f - (_e5137 * _e5137)));
                                                    if (_e5142 >= 1f) {
                                                        phi_29462_ = 1f;
                                                    } else {
                                                        let _e5145 = sqrt((1f - _e5142));
                                                        if (_e5137 != _e5137) {
                                                            phi_29473_ = true;
                                                        } else {
                                                            phi_29473_ = (_e5145 <= _e5137);
                                                        }
                                                        let _e5149 = phi_29473_;
                                                        let _e5154 = pow(((1f - _e2594) / (1f + _e2594)), 2f);
                                                        phi_29462_ = (_e5154 + ((1f - _e5154) * pow((1f - select(_e5137, _e5145, _e5149)), 5f)));
                                                    }
                                                    let _e5161 = phi_29462_;
                                                    if (_e4706 <= 0f) {
                                                        phi_29512_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_29512_ = (_e2592 * (_e4706 * 0.31830987f));
                                                    }
                                                    let _e5166 = phi_29512_;
                                                    let _e5167 = (1f - _e5161);
                                                    phi_18126_ = type_56();
                                                    phi_18127_ = type_56((_e5129.member + vec3<f32>((_e5166.x * _e5167), (_e5166.y * _e5167), (_e5166.z * _e5167))), ((_e5161 * _e5129.member_1) + (_e5167 * _e4712)));
                                                }
                                                let _e5181 = phi_18126_;
                                                let _e5183 = phi_18127_;
                                                phi_18128_ = _e5181;
                                                phi_18129_ = _e5183;
                                                phi_18130_ = _e5132;
                                            }
                                            let _e5191 = phi_18128_;
                                            let _e5193 = phi_18129_;
                                            let _e5195 = phi_18130_;
                                            if _e5195 {
                                                phi_18134_ = _e5191;
                                            } else {
                                                phi_18134_ = _e5193;
                                            }
                                            let _e5197 = phi_18134_;
                                            if (_e4683 > 0f) {
                                                if (_e4683 >= 300000000000000000000000000000000000000f) {
                                                    phi_29551_ = 1f;
                                                } else {
                                                    let _e5202 = (_e4683 * _e4683);
                                                    let _e5204 = (_e5202 + (_e5197.member_1 * _e5197.member_1));
                                                    if (_e5204 == 0f) {
                                                        phi_29548_ = 0f;
                                                    } else {
                                                        phi_29548_ = (_e5202 / _e5204);
                                                    }
                                                    let _e5208 = phi_29548_;
                                                    phi_29551_ = _e5208;
                                                }
                                                let _e5210 = phi_29551_;
                                                phi_18143_ = (_e5210 / _e4683);
                                            } else {
                                                phi_18143_ = 1f;
                                            }
                                            let _e5213 = phi_18143_;
                                            if ((((_e5197.member.x * _e5197.member.x) + (_e5197.member.y * _e5197.member.y)) + (_e5197.member.z * _e5197.member.z)) > 0f) {
                                                let _e5225 = (vec3<f32>(1f, 1f, 1f) / _e4685);
                                                local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                                phi_18177_ = _e377;
                                                phi_18180_ = 0u;
                                                phi_18182_ = _e374;
                                                phi_18184_ = _e370;
                                                phi_18188_ = type_66(_e4689, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                loop {
                                                    let _e5227 = phi_18177_;
                                                    let _e5229 = phi_18180_;
                                                    let _e5231 = phi_18182_;
                                                    let _e5233 = phi_18184_;
                                                    let _e5235 = phi_18188_;
                                                    local_108 = _e5235;
                                                    if (_e5233 < _e5231) {
                                                        let _e5603 = global_2.member[_e5233];
                                                        if (_e5603.member_6.x == 0u) {
                                                            let _e5709 = (_e5603.member_1.x - _e5603.member.x);
                                                            let _e5710 = (_e5603.member_1.y - _e5603.member.y);
                                                            let _e5711 = (_e5603.member_1.z - _e5603.member.z);
                                                            let _e5716 = (_e5603.member_2.x - _e5603.member.x);
                                                            let _e5717 = (_e5603.member_2.y - _e5603.member.y);
                                                            let _e5718 = (_e5603.member_2.z - _e5603.member.z);
                                                            let _e5721 = ((_e4685.y * _e5718) - (_e5717 * _e4685.z));
                                                            let _e5724 = ((_e4685.z * _e5716) - (_e5718 * _e4685.x));
                                                            let _e5727 = ((_e4685.x * _e5717) - (_e5716 * _e4685.y));
                                                            let _e5732 = (((_e5709 * _e5721) + (_e5710 * _e5724)) + (_e5711 * _e5727));
                                                            if (abs(_e5732) < 0.000000000001f) {
                                                                phi_18355_ = vec4<f32>();
                                                                phi_18356_ = true;
                                                            } else {
                                                                let _e5735 = (1f / _e5732);
                                                                let _e5736 = (_e1737 - _e5603.member.x);
                                                                let _e5737 = (_e1738 - _e5603.member.y);
                                                                let _e5738 = (_e1739 - _e5603.member.z);
                                                                let _e5744 = ((((_e5736 * _e5721) + (_e5737 * _e5724)) + (_e5738 * _e5727)) * _e5735);
                                                                if (_e5744 < 0f) {
                                                                    phi_18345_ = vec4<f32>();
                                                                    phi_18346_ = true;
                                                                    phi_18347_ = false;
                                                                } else {
                                                                    let _e5746 = (_e5744 > 1f);
                                                                    if _e5746 {
                                                                        phi_18343_ = vec4<f32>();
                                                                        phi_18344_ = false;
                                                                    } else {
                                                                        let _e5749 = ((_e5737 * _e5711) - (_e5710 * _e5738));
                                                                        let _e5752 = ((_e5738 * _e5709) - (_e5711 * _e5736));
                                                                        let _e5755 = ((_e5736 * _e5710) - (_e5709 * _e5737));
                                                                        let _e5761 = ((((_e4685.x * _e5749) + (_e4685.y * _e5752)) + (_e4685.z * _e5755)) * _e5735);
                                                                        if (_e5761 < 0f) {
                                                                            phi_18333_ = vec4<f32>();
                                                                            phi_18334_ = true;
                                                                            phi_18335_ = false;
                                                                        } else {
                                                                            let _e5764 = ((_e5744 + _e5761) > 1f);
                                                                            if _e5764 {
                                                                                phi_18331_ = vec4<f32>();
                                                                                phi_18332_ = false;
                                                                            } else {
                                                                                let _e5770 = ((((_e5716 * _e5749) + (_e5717 * _e5752)) + (_e5718 * _e5755)) * _e5735);
                                                                                if (_e5770 <= 0f) {
                                                                                    phi_18326_ = vec4<f32>();
                                                                                    phi_18327_ = true;
                                                                                } else {
                                                                                    let _e5772 = (_e5770 >= _e5235.member);
                                                                                    if _e5772 {
                                                                                        phi_18325_ = vec4<f32>();
                                                                                    } else {
                                                                                        phi_18325_ = vec4<f32>(_e5770, _e5744, _e5761, 1f);
                                                                                    }
                                                                                    let _e5775 = phi_18325_;
                                                                                    phi_18326_ = _e5775;
                                                                                    phi_18327_ = _e5772;
                                                                                }
                                                                                let _e5777 = phi_18326_;
                                                                                let _e5779 = phi_18327_;
                                                                                phi_18331_ = _e5777;
                                                                                phi_18332_ = _e5779;
                                                                            }
                                                                            let _e5781 = phi_18331_;
                                                                            let _e5783 = phi_18332_;
                                                                            phi_18333_ = _e5781;
                                                                            phi_18334_ = _e5764;
                                                                            phi_18335_ = _e5783;
                                                                        }
                                                                        let _e5785 = phi_18333_;
                                                                        let _e5787 = phi_18334_;
                                                                        let _e5789 = phi_18335_;
                                                                        phi_18343_ = _e5785;
                                                                        phi_18344_ = select(_e5789, true, _e5787);
                                                                    }
                                                                    let _e5792 = phi_18343_;
                                                                    let _e5794 = phi_18344_;
                                                                    phi_18345_ = _e5792;
                                                                    phi_18346_ = _e5746;
                                                                    phi_18347_ = _e5794;
                                                                }
                                                                let _e5796 = phi_18345_;
                                                                let _e5798 = phi_18346_;
                                                                let _e5800 = phi_18347_;
                                                                phi_18355_ = _e5796;
                                                                phi_18356_ = select(_e5800, true, _e5798);
                                                            }
                                                            let _e5803 = phi_18355_;
                                                            let _e5805 = phi_18356_;
                                                            phi_18502_ = _e5803;
                                                            phi_18503_ = _e5805;
                                                        } else {
                                                            if (_e5603.member_6.x == 1u) {
                                                                let _e5631 = (_e1737 - _e5603.member.x);
                                                                let _e5632 = (_e1738 - _e5603.member.y);
                                                                let _e5633 = (_e1739 - _e5603.member.z);
                                                                let _e5638 = (((_e5631 * _e4685.x) + (_e5632 * _e4685.y)) + (_e5633 * _e4685.z));
                                                                if (_e5638 > 0f) {
                                                                    phi_18396_ = select(true, false, ((((_e5631 * _e5631) + (_e5632 * _e5632)) + (_e5633 * _e5633)) > (_e5603.member.w * _e5603.member.w)));
                                                                } else {
                                                                    phi_18396_ = true;
                                                                }
                                                                let _e5651 = phi_18396_;
                                                                if _e5651 {
                                                                    let _e5655 = (_e5631 - (_e4685.x * _e5638));
                                                                    let _e5656 = (_e5632 - (_e4685.y * _e5638));
                                                                    let _e5657 = (_e5633 - (_e4685.z * _e5638));
                                                                    let _e5666 = ((_e5603.member.w * _e5603.member.w) - (((_e5655 * _e5655) + (_e5656 * _e5656)) + (_e5657 * _e5657)));
                                                                    if (_e5666 < 0f) {
                                                                        phi_18455_ = vec4<f32>();
                                                                        phi_18456_ = true;
                                                                    } else {
                                                                        let _e5668 = sqrt(_e5666);
                                                                        let _e5669 = -(_e5638);
                                                                        let _e5670 = (_e5669 - _e5668);
                                                                        if (_e5670 <= 0f) {
                                                                            phi_18433_ = true;
                                                                        } else {
                                                                            phi_18433_ = (_e5670 >= _e5235.member);
                                                                        }
                                                                        let _e5674 = phi_18433_;
                                                                        if _e5674 {
                                                                            phi_18439_ = (_e5669 + _e5668);
                                                                        } else {
                                                                            phi_18439_ = _e5670;
                                                                        }
                                                                        let _e5677 = phi_18439_;
                                                                        if (_e5677 <= 0f) {
                                                                            phi_18450_ = vec4<f32>();
                                                                            phi_18451_ = true;
                                                                        } else {
                                                                            let _e5679 = (_e5677 >= _e5235.member);
                                                                            if _e5679 {
                                                                                phi_18449_ = vec4<f32>();
                                                                            } else {
                                                                                phi_18449_ = vec4<f32>(_e5677, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5682 = phi_18449_;
                                                                            phi_18450_ = _e5682;
                                                                            phi_18451_ = _e5679;
                                                                        }
                                                                        let _e5684 = phi_18450_;
                                                                        let _e5686 = phi_18451_;
                                                                        phi_18455_ = _e5684;
                                                                        phi_18456_ = _e5686;
                                                                    }
                                                                    let _e5688 = phi_18455_;
                                                                    let _e5690 = phi_18456_;
                                                                    phi_18460_ = _e5688;
                                                                    phi_18461_ = _e5690;
                                                                } else {
                                                                    phi_18460_ = vec4<f32>();
                                                                    phi_18461_ = true;
                                                                }
                                                                let _e5692 = phi_18460_;
                                                                let _e5694 = phi_18461_;
                                                                phi_18492_ = _e5692;
                                                                phi_18493_ = _e5694;
                                                            } else {
                                                                if (abs(_e4685.y) < 0.000000000001f) {
                                                                    phi_18490_ = vec4<f32>();
                                                                    phi_18491_ = true;
                                                                } else {
                                                                    let _e5613 = ((_e5603.member.x - _e1738) / _e4685.y);
                                                                    if (_e5613 <= 0f) {
                                                                        phi_18485_ = vec4<f32>();
                                                                        phi_18486_ = true;
                                                                    } else {
                                                                        let _e5615 = (_e5613 >= _e5235.member);
                                                                        if _e5615 {
                                                                            phi_18484_ = vec4<f32>();
                                                                        } else {
                                                                            phi_18484_ = vec4<f32>(_e5613, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5618 = phi_18484_;
                                                                        phi_18485_ = _e5618;
                                                                        phi_18486_ = _e5615;
                                                                    }
                                                                    let _e5620 = phi_18485_;
                                                                    let _e5622 = phi_18486_;
                                                                    phi_18490_ = _e5620;
                                                                    phi_18491_ = _e5622;
                                                                }
                                                                let _e5624 = phi_18490_;
                                                                let _e5626 = phi_18491_;
                                                                phi_18492_ = _e5624;
                                                                phi_18493_ = _e5626;
                                                            }
                                                            let _e5696 = phi_18492_;
                                                            let _e5698 = phi_18493_;
                                                            phi_18502_ = _e5696;
                                                            phi_18503_ = select(true, false, select(true, false, _e5698));
                                                        }
                                                        let _e5807 = phi_18502_;
                                                        let _e5809 = phi_18503_;
                                                        let _e5812 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5807, vec4(select(true, false, _e5809)));
                                                        let _e5814 = (_e5812.w > 0f);
                                                        if _e5814 {
                                                            let _e5820 = type_66(_e5812.x, _e5235.member_1, _e5235.member_2, _e5235.member_3);
                                                            let _e5825 = type_66(_e5820.member, _e5233, _e5820.member_2, _e5820.member_3);
                                                            phi_18524_ = _e5825;
                                                            phi_18526_ = _e5825;
                                                        } else {
                                                            phi_18524_ = type_66();
                                                            phi_18526_ = _e5235;
                                                        }
                                                        let _e5827 = phi_18524_;
                                                        let _e5829 = phi_18526_;
                                                        let _e5830 = select(true, false, _e5814);
                                                        if _e5830 {
                                                            phi_18532_ = (_e5233 + 1u);
                                                        } else {
                                                            phi_18532_ = u32();
                                                        }
                                                        let _e5833 = phi_18532_;
                                                        phi_18178_ = _e5227;
                                                        phi_18181_ = _e5229;
                                                        phi_18183_ = _e5231;
                                                        phi_18185_ = _e5833;
                                                        phi_18189_ = _e5829;
                                                        phi_18708_ = _e5827;
                                                        phi_18709_ = _e5830;
                                                        phi_18710_ = false;
                                                    } else {
                                                        if (_e5227 == 4294967295u) {
                                                            let _e5238 = (_e5229 == 0u);
                                                            if _e5238 {
                                                                phi_18563_ = u32();
                                                                phi_18564_ = u32();
                                                                phi_18566_ = false;
                                                                phi_18567_ = false;
                                                            } else {
                                                                let _e5239 = (_e5229 - 1u);
                                                                let _e5240 = (_e5239 < 64u);
                                                                if _e5240 {
                                                                } else {
                                                                    phi_35128_ = true;
                                                                    phi_20525_ = bool();
                                                                    phi_20524_ = type_66();
                                                                    break;
                                                                }
                                                                let _e5242 = local_68[_e5239];
                                                                let _e5244 = (_e5242 >= _e5235.member);
                                                                if _e5244 {
                                                                    phi_18561_ = u32();
                                                                } else {
                                                                    if _e5240 {
                                                                    } else {
                                                                        phi_35128_ = true;
                                                                        phi_20525_ = bool();
                                                                        phi_20524_ = type_66();
                                                                        break;
                                                                    }
                                                                    let _e5246 = local_67[_e5239];
                                                                    phi_18561_ = _e5246;
                                                                }
                                                                let _e5248 = phi_18561_;
                                                                phi_18563_ = _e5239;
                                                                phi_18564_ = _e5248;
                                                                phi_18566_ = select(true, false, _e5244);
                                                                phi_18567_ = _e5244;
                                                            }
                                                            let _e5251 = phi_18563_;
                                                            let _e5253 = phi_18564_;
                                                            let _e5255 = phi_18566_;
                                                            let _e5257 = phi_18567_;
                                                            phi_18568_ = _e5251;
                                                            phi_18569_ = _e5253;
                                                            phi_18570_ = _e5251;
                                                            phi_18571_ = _e5255;
                                                            phi_18572_ = _e5257;
                                                            phi_18573_ = _e5238;
                                                        } else {
                                                            phi_18568_ = u32();
                                                            phi_18569_ = _e5227;
                                                            phi_18570_ = _e5229;
                                                            phi_18571_ = true;
                                                            phi_18572_ = false;
                                                            phi_18573_ = false;
                                                        }
                                                        let _e5259 = phi_18568_;
                                                        let _e5261 = phi_18569_;
                                                        let _e5263 = phi_18570_;
                                                        let _e5265 = phi_18571_;
                                                        let _e5267 = phi_18572_;
                                                        let _e5269 = phi_18573_;
                                                        if _e5265 {
                                                            let _e5271 = ((_e5261 & 2147483648u) == 0u);
                                                            if _e5271 {
                                                                let _e5279 = global_3.member[_e5261];
                                                                if (_e5279.member_1 == 4294967295u) {
                                                                    phi_18603_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e5292 = ((_e5279.member.x - _e1737) * _e5225.x);
                                                                    let _e5294 = ((_e5279.member.y - _e1738) * _e5225.y);
                                                                    let _e5296 = ((_e5279.member.z - _e1739) * _e5225.z);
                                                                    let _e5303 = ((_e5279.member_2.x - _e1737) * _e5225.x);
                                                                    let _e5304 = ((_e5279.member_2.y - _e1738) * _e5225.y);
                                                                    let _e5305 = ((_e5279.member_2.z - _e1739) * _e5225.z);
                                                                    if (_e5292 == _e5292) {
                                                                        let _e5307 = (_e5303 == _e5303);
                                                                        if _e5307 {
                                                                            let _e5308 = (_e5292 != _e5292);
                                                                            if _e5308 {
                                                                                phi_29649_ = true;
                                                                            } else {
                                                                                phi_29649_ = (_e5303 <= _e5292);
                                                                            }
                                                                            let _e5311 = phi_29649_;
                                                                            let _e5312 = select(_e5292, _e5303, _e5311);
                                                                            if _e5308 {
                                                                                phi_29679_ = true;
                                                                            } else {
                                                                                phi_29679_ = (_e5303 >= _e5292);
                                                                            }
                                                                            let _e5317 = phi_29679_;
                                                                            let _e5318 = select(_e5292, _e5303, _e5317);
                                                                            phi_29626_ = type_53(select(-300000000000000000000000000000000000000f, _e5312, (_e5312 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5318, (_e5318 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_29626_ = type_53();
                                                                        }
                                                                        let _e5323 = phi_29626_;
                                                                        phi_29629_ = _e5323;
                                                                        phi_29630_ = select(true, false, _e5307);
                                                                    } else {
                                                                        phi_29629_ = type_53();
                                                                        phi_29630_ = true;
                                                                    }
                                                                    let _e5326 = phi_29629_;
                                                                    let _e5328 = phi_29630_;
                                                                    if _e5328 {
                                                                        phi_29635_ = type_53(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_29635_ = _e5326;
                                                                    }
                                                                    let _e5330 = phi_29635_;
                                                                    if (_e5294 == _e5294) {
                                                                        let _e5334 = (_e5304 == _e5304);
                                                                        if _e5334 {
                                                                            let _e5335 = (_e5294 != _e5294);
                                                                            if _e5335 {
                                                                                phi_29736_ = true;
                                                                            } else {
                                                                                phi_29736_ = (_e5304 <= _e5294);
                                                                            }
                                                                            let _e5338 = phi_29736_;
                                                                            let _e5339 = select(_e5294, _e5304, _e5338);
                                                                            if (_e5330.member != _e5330.member) {
                                                                                phi_29751_ = true;
                                                                            } else {
                                                                                phi_29751_ = (_e5339 >= _e5330.member);
                                                                            }
                                                                            let _e5343 = phi_29751_;
                                                                            if _e5335 {
                                                                                phi_29766_ = true;
                                                                            } else {
                                                                                phi_29766_ = (_e5304 >= _e5294);
                                                                            }
                                                                            let _e5347 = phi_29766_;
                                                                            let _e5348 = select(_e5294, _e5304, _e5347);
                                                                            if (_e5330.member_1 != _e5330.member_1) {
                                                                                phi_29781_ = true;
                                                                            } else {
                                                                                phi_29781_ = (_e5348 <= _e5330.member_1);
                                                                            }
                                                                            let _e5352 = phi_29781_;
                                                                            phi_29713_ = type_53(select(_e5330.member, _e5339, _e5343), select(_e5330.member_1, _e5348, _e5352));
                                                                        } else {
                                                                            phi_29713_ = type_53();
                                                                        }
                                                                        let _e5356 = phi_29713_;
                                                                        phi_29716_ = _e5356;
                                                                        phi_29717_ = select(true, false, _e5334);
                                                                    } else {
                                                                        phi_29716_ = type_53();
                                                                        phi_29717_ = true;
                                                                    }
                                                                    let _e5359 = phi_29716_;
                                                                    let _e5361 = phi_29717_;
                                                                    if _e5361 {
                                                                        phi_29722_ = _e5330;
                                                                    } else {
                                                                        phi_29722_ = _e5359;
                                                                    }
                                                                    let _e5363 = phi_29722_;
                                                                    if (_e5296 == _e5296) {
                                                                        let _e5367 = (_e5305 == _e5305);
                                                                        if _e5367 {
                                                                            let _e5368 = (_e5296 != _e5296);
                                                                            if _e5368 {
                                                                                phi_29823_ = true;
                                                                            } else {
                                                                                phi_29823_ = (_e5305 <= _e5296);
                                                                            }
                                                                            let _e5371 = phi_29823_;
                                                                            let _e5372 = select(_e5296, _e5305, _e5371);
                                                                            if (_e5363.member != _e5363.member) {
                                                                                phi_29838_ = true;
                                                                            } else {
                                                                                phi_29838_ = (_e5372 >= _e5363.member);
                                                                            }
                                                                            let _e5376 = phi_29838_;
                                                                            if _e5368 {
                                                                                phi_29853_ = true;
                                                                            } else {
                                                                                phi_29853_ = (_e5305 >= _e5296);
                                                                            }
                                                                            let _e5380 = phi_29853_;
                                                                            let _e5381 = select(_e5296, _e5305, _e5380);
                                                                            if (_e5363.member_1 != _e5363.member_1) {
                                                                                phi_29868_ = true;
                                                                            } else {
                                                                                phi_29868_ = (_e5381 <= _e5363.member_1);
                                                                            }
                                                                            let _e5385 = phi_29868_;
                                                                            phi_29800_ = type_53(select(_e5363.member, _e5372, _e5376), select(_e5363.member_1, _e5381, _e5385));
                                                                        } else {
                                                                            phi_29800_ = type_53();
                                                                        }
                                                                        let _e5389 = phi_29800_;
                                                                        phi_29803_ = _e5389;
                                                                        phi_29804_ = select(true, false, _e5367);
                                                                    } else {
                                                                        phi_29803_ = type_53();
                                                                        phi_29804_ = true;
                                                                    }
                                                                    let _e5392 = phi_29803_;
                                                                    let _e5394 = phi_29804_;
                                                                    if _e5394 {
                                                                        phi_29809_ = _e5363;
                                                                    } else {
                                                                        phi_29809_ = _e5392;
                                                                    }
                                                                    let _e5396 = phi_29809_;
                                                                    if (_e5396.member_1 < 0f) {
                                                                        phi_29607_ = true;
                                                                    } else {
                                                                        if (_e5396.member > _e5396.member_1) {
                                                                            phi_29604_ = true;
                                                                        } else {
                                                                            phi_29604_ = (_e5396.member > _e5235.member);
                                                                        }
                                                                        let _e5403 = phi_29604_;
                                                                        phi_29607_ = _e5403;
                                                                    }
                                                                    let _e5405 = phi_29607_;
                                                                    phi_18603_ = select(_e5396.member, 300000000000000000000000000000000000000f, _e5405);
                                                                }
                                                                let _e5408 = phi_18603_;
                                                                if (_e5279.member_5 == 4294967295u) {
                                                                    phi_18613_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e5421 = ((_e5279.member_4.x - _e1737) * _e5225.x);
                                                                    let _e5423 = ((_e5279.member_4.y - _e1738) * _e5225.y);
                                                                    let _e5425 = ((_e5279.member_4.z - _e1739) * _e5225.z);
                                                                    let _e5432 = ((_e5279.member_6.x - _e1737) * _e5225.x);
                                                                    let _e5433 = ((_e5279.member_6.y - _e1738) * _e5225.y);
                                                                    let _e5434 = ((_e5279.member_6.z - _e1739) * _e5225.z);
                                                                    if (_e5421 == _e5421) {
                                                                        let _e5436 = (_e5432 == _e5432);
                                                                        if _e5436 {
                                                                            let _e5437 = (_e5421 != _e5421);
                                                                            if _e5437 {
                                                                                phi_29970_ = true;
                                                                            } else {
                                                                                phi_29970_ = (_e5432 <= _e5421);
                                                                            }
                                                                            let _e5440 = phi_29970_;
                                                                            let _e5441 = select(_e5421, _e5432, _e5440);
                                                                            if _e5437 {
                                                                                phi_30000_ = true;
                                                                            } else {
                                                                                phi_30000_ = (_e5432 >= _e5421);
                                                                            }
                                                                            let _e5446 = phi_30000_;
                                                                            let _e5447 = select(_e5421, _e5432, _e5446);
                                                                            phi_29947_ = type_53(select(-300000000000000000000000000000000000000f, _e5441, (_e5441 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5447, (_e5447 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_29947_ = type_53();
                                                                        }
                                                                        let _e5452 = phi_29947_;
                                                                        phi_29950_ = _e5452;
                                                                        phi_29951_ = select(true, false, _e5436);
                                                                    } else {
                                                                        phi_29950_ = type_53();
                                                                        phi_29951_ = true;
                                                                    }
                                                                    let _e5455 = phi_29950_;
                                                                    let _e5457 = phi_29951_;
                                                                    if _e5457 {
                                                                        phi_29956_ = type_53(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_29956_ = _e5455;
                                                                    }
                                                                    let _e5459 = phi_29956_;
                                                                    if (_e5423 == _e5423) {
                                                                        let _e5463 = (_e5433 == _e5433);
                                                                        if _e5463 {
                                                                            let _e5464 = (_e5423 != _e5423);
                                                                            if _e5464 {
                                                                                phi_30057_ = true;
                                                                            } else {
                                                                                phi_30057_ = (_e5433 <= _e5423);
                                                                            }
                                                                            let _e5467 = phi_30057_;
                                                                            let _e5468 = select(_e5423, _e5433, _e5467);
                                                                            if (_e5459.member != _e5459.member) {
                                                                                phi_30072_ = true;
                                                                            } else {
                                                                                phi_30072_ = (_e5468 >= _e5459.member);
                                                                            }
                                                                            let _e5472 = phi_30072_;
                                                                            if _e5464 {
                                                                                phi_30087_ = true;
                                                                            } else {
                                                                                phi_30087_ = (_e5433 >= _e5423);
                                                                            }
                                                                            let _e5476 = phi_30087_;
                                                                            let _e5477 = select(_e5423, _e5433, _e5476);
                                                                            if (_e5459.member_1 != _e5459.member_1) {
                                                                                phi_30102_ = true;
                                                                            } else {
                                                                                phi_30102_ = (_e5477 <= _e5459.member_1);
                                                                            }
                                                                            let _e5481 = phi_30102_;
                                                                            phi_30034_ = type_53(select(_e5459.member, _e5468, _e5472), select(_e5459.member_1, _e5477, _e5481));
                                                                        } else {
                                                                            phi_30034_ = type_53();
                                                                        }
                                                                        let _e5485 = phi_30034_;
                                                                        phi_30037_ = _e5485;
                                                                        phi_30038_ = select(true, false, _e5463);
                                                                    } else {
                                                                        phi_30037_ = type_53();
                                                                        phi_30038_ = true;
                                                                    }
                                                                    let _e5488 = phi_30037_;
                                                                    let _e5490 = phi_30038_;
                                                                    if _e5490 {
                                                                        phi_30043_ = _e5459;
                                                                    } else {
                                                                        phi_30043_ = _e5488;
                                                                    }
                                                                    let _e5492 = phi_30043_;
                                                                    if (_e5425 == _e5425) {
                                                                        let _e5496 = (_e5434 == _e5434);
                                                                        if _e5496 {
                                                                            let _e5497 = (_e5425 != _e5425);
                                                                            if _e5497 {
                                                                                phi_30144_ = true;
                                                                            } else {
                                                                                phi_30144_ = (_e5434 <= _e5425);
                                                                            }
                                                                            let _e5500 = phi_30144_;
                                                                            let _e5501 = select(_e5425, _e5434, _e5500);
                                                                            if (_e5492.member != _e5492.member) {
                                                                                phi_30159_ = true;
                                                                            } else {
                                                                                phi_30159_ = (_e5501 >= _e5492.member);
                                                                            }
                                                                            let _e5505 = phi_30159_;
                                                                            if _e5497 {
                                                                                phi_30174_ = true;
                                                                            } else {
                                                                                phi_30174_ = (_e5434 >= _e5425);
                                                                            }
                                                                            let _e5509 = phi_30174_;
                                                                            let _e5510 = select(_e5425, _e5434, _e5509);
                                                                            if (_e5492.member_1 != _e5492.member_1) {
                                                                                phi_30189_ = true;
                                                                            } else {
                                                                                phi_30189_ = (_e5510 <= _e5492.member_1);
                                                                            }
                                                                            let _e5514 = phi_30189_;
                                                                            phi_30121_ = type_53(select(_e5492.member, _e5501, _e5505), select(_e5492.member_1, _e5510, _e5514));
                                                                        } else {
                                                                            phi_30121_ = type_53();
                                                                        }
                                                                        let _e5518 = phi_30121_;
                                                                        phi_30124_ = _e5518;
                                                                        phi_30125_ = select(true, false, _e5496);
                                                                    } else {
                                                                        phi_30124_ = type_53();
                                                                        phi_30125_ = true;
                                                                    }
                                                                    let _e5521 = phi_30124_;
                                                                    let _e5523 = phi_30125_;
                                                                    if _e5523 {
                                                                        phi_30130_ = _e5492;
                                                                    } else {
                                                                        phi_30130_ = _e5521;
                                                                    }
                                                                    let _e5525 = phi_30130_;
                                                                    if (_e5525.member_1 < 0f) {
                                                                        phi_29928_ = true;
                                                                    } else {
                                                                        if (_e5525.member > _e5525.member_1) {
                                                                            phi_29925_ = true;
                                                                        } else {
                                                                            phi_29925_ = (_e5525.member > _e5235.member);
                                                                        }
                                                                        let _e5532 = phi_29925_;
                                                                        phi_29928_ = _e5532;
                                                                    }
                                                                    let _e5534 = phi_29928_;
                                                                    phi_18613_ = select(_e5525.member, 300000000000000000000000000000000000000f, _e5534);
                                                                }
                                                                let _e5537 = phi_18613_;
                                                                if (_e5279.member_3 > 0u) {
                                                                    phi_18624_ = ((2147483648u | ((_e5279.member_3 - 1u) << bitcast<u32>(28u))) | _e5279.member_1);
                                                                } else {
                                                                    phi_18624_ = _e5279.member_1;
                                                                }
                                                                let _e5546 = phi_18624_;
                                                                if (_e5279.member_7 > 0u) {
                                                                    phi_18635_ = ((2147483648u | ((_e5279.member_7 - 1u) << bitcast<u32>(28u))) | _e5279.member_5);
                                                                } else {
                                                                    phi_18635_ = _e5279.member_5;
                                                                }
                                                                let _e5555 = phi_18635_;
                                                                let _e5556 = (_e5408 <= _e5537);
                                                                let _e5560 = select(_e5408, _e5537, _e5556);
                                                                if (_e5560 < 300000000000000000000000000000000000000f) {
                                                                    let _e5562 = (_e5263 < 64u);
                                                                    if _e5562 {
                                                                        if _e5562 {
                                                                        } else {
                                                                            phi_35128_ = true;
                                                                            phi_20525_ = bool();
                                                                            phi_20524_ = type_66();
                                                                            break;
                                                                        }
                                                                        local_67[_e5263] = select(_e5546, _e5555, _e5556);
                                                                        if _e5562 {
                                                                        } else {
                                                                            phi_35128_ = true;
                                                                            phi_20525_ = bool();
                                                                            phi_20524_ = type_66();
                                                                            break;
                                                                        }
                                                                        local_68[_e5263] = _e5560;
                                                                        phi_18674_ = (_e5263 + 1u);
                                                                    } else {
                                                                        phi_18674_ = _e5263;
                                                                    }
                                                                    let _e5567 = phi_18674_;
                                                                    phi_18675_ = _e5567;
                                                                } else {
                                                                    phi_18675_ = _e5263;
                                                                }
                                                                let _e5569 = phi_18675_;
                                                                phi_18686_ = select(4294967295u, select(_e5555, _e5546, _e5556), (select(_e5537, _e5408, _e5556) < 300000000000000000000000000000000000000f));
                                                                phi_18687_ = _e5569;
                                                                phi_18688_ = u32();
                                                                phi_18689_ = u32();
                                                            } else {
                                                                let _e5272 = (_e5261 & 268435455u);
                                                                phi_18686_ = u32();
                                                                phi_18687_ = u32();
                                                                phi_18688_ = ((_e5272 + ((_e5261 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                                phi_18689_ = _e5272;
                                                            }
                                                            let _e5573 = phi_18686_;
                                                            let _e5575 = phi_18687_;
                                                            let _e5577 = phi_18688_;
                                                            let _e5579 = phi_18689_;
                                                            phi_18691_ = _e5573;
                                                            phi_18692_ = _e5575;
                                                            phi_18694_ = _e5577;
                                                            phi_18695_ = _e5579;
                                                            phi_18696_ = select(true, false, _e5271);
                                                            phi_18697_ = _e5271;
                                                        } else {
                                                            phi_18691_ = u32();
                                                            phi_18692_ = u32();
                                                            phi_18694_ = _e5231;
                                                            phi_18695_ = _e5233;
                                                            phi_18696_ = _e5267;
                                                            phi_18697_ = false;
                                                        }
                                                        let _e5582 = phi_18691_;
                                                        let _e5584 = phi_18692_;
                                                        let _e5586 = phi_18694_;
                                                        let _e5588 = phi_18695_;
                                                        let _e5590 = phi_18696_;
                                                        let _e5592 = phi_18697_;
                                                        phi_18178_ = select(_e5582, 4294967295u, _e5590);
                                                        phi_18181_ = select(_e5584, select(_e5259, _e5263, _e5265), _e5590);
                                                        phi_18183_ = select(_e5231, _e5586, _e5590);
                                                        phi_18185_ = select(_e5233, _e5588, _e5590);
                                                        phi_18189_ = _e5235;
                                                        phi_18708_ = type_66();
                                                        phi_18709_ = select(_e5592, true, _e5590);
                                                        phi_18710_ = select(select(_e5269, false, _e5265), false, _e5590);
                                                    }
                                                    let _e5835 = phi_18178_;
                                                    let _e5837 = phi_18181_;
                                                    let _e5839 = phi_18183_;
                                                    let _e5841 = phi_18185_;
                                                    let _e5843 = phi_18189_;
                                                    let _e5845 = phi_18708_;
                                                    let _e5847 = phi_18709_;
                                                    let _e5849 = phi_18710_;
                                                    phi_18177_ = _e5835;
                                                    phi_18180_ = _e5837;
                                                    phi_18182_ = _e5839;
                                                    phi_18184_ = _e5841;
                                                    phi_18188_ = _e5843;
                                                    phi_35128_ = _e2617;
                                                    phi_20525_ = _e5849;
                                                    phi_20524_ = _e5845;
                                                    if !(_e5847) {
                                                        break;
                                                    }
                                                }
                                                let _e5852 = phi_35128_;
                                                let _e5854 = phi_20525_;
                                                let _e5856 = phi_20524_;
                                                phi_35576_ = _e4681;
                                                phi_35562_ = _e5852;
                                                if _e5852 {
                                                    break;
                                                }
                                                if _e5854 {
                                                    let _e8430 = local_108;
                                                    phi_18882_ = _e8430;
                                                } else {
                                                    phi_18882_ = type_66();
                                                }
                                                let _e5858 = phi_18882_;
                                                if select(true, false, _e5854) {
                                                    phi_18887_ = _e5856;
                                                } else {
                                                    phi_18887_ = _e5858;
                                                }
                                                let _e5861 = phi_18887_;
                                                if (_e5861.member_1 != 4294967295u) {
                                                    phi_18941_ = _e2621;
                                                    phi_18942_ = _e2623;
                                                } else {
                                                    if (_e2625 == (_e2615 + 2u)) {
                                                        phi_18939_ = _e2621;
                                                        phi_18940_ = type_39(vec4<f32>(_e1737, _e1738, _e1739, -1f), vec4<f32>(_e4685.x, _e4685.y, _e4685.z, 0f), vec4<f32>(((_e1607.x * _e5197.member.x) * _e5213), ((_e1607.y * _e5197.member.y) * _e5213), ((_e1607.z * _e5197.member.z) * _e5213), 0f));
                                                    } else {
                                                        phi_18939_ = vec3<f32>((_e2621.x + (((_e1607.x * _e5197.member.x) * _e4687.x) * _e5213)), (_e2621.y + (((_e1607.y * _e5197.member.y) * _e4687.y) * _e5213)), (_e2621.z + (((_e1607.z * _e5197.member.z) * _e4687.z) * _e5213)));
                                                        phi_18940_ = _e2623;
                                                    }
                                                    let _e5899 = phi_18939_;
                                                    let _e5901 = phi_18940_;
                                                    phi_18941_ = _e5899;
                                                    phi_18942_ = _e5901;
                                                }
                                                let _e5903 = phi_18941_;
                                                let _e5905 = phi_18942_;
                                                phi_35565_ = _e5852;
                                                phi_18943_ = _e5903;
                                                phi_18944_ = _e5905;
                                                phi_18945_ = (_e2627 + 1u);
                                            } else {
                                                phi_35565_ = _e2617;
                                                phi_18943_ = _e2621;
                                                phi_18944_ = _e2623;
                                                phi_18945_ = _e2627;
                                            }
                                            let _e5907 = phi_35565_;
                                            let _e5909 = phi_18943_;
                                            let _e5911 = phi_18944_;
                                            let _e5913 = phi_18945_;
                                            phi_35564_ = _e5907;
                                            phi_18946_ = _e5909;
                                            phi_18947_ = _e5911;
                                            phi_18948_ = _e5913;
                                        } else {
                                            phi_35564_ = _e2617;
                                            phi_18946_ = _e2621;
                                            phi_18947_ = _e2623;
                                            phi_18948_ = _e2627;
                                        }
                                        let _e5915 = phi_35564_;
                                        let _e5917 = phi_18946_;
                                        let _e5919 = phi_18947_;
                                        let _e5921 = phi_18948_;
                                        phi_35856_ = _e4681;
                                        phi_35563_ = _e5915;
                                        phi_14688_ = _e5917;
                                        phi_14691_ = _e5919;
                                        phi_14693_ = (_e2625 + 1u);
                                        phi_14695_ = _e5921;
                                    } else {
                                        phi_35856_ = _e2619;
                                        phi_35563_ = _e2617;
                                        phi_14688_ = vec3<f32>();
                                        phi_14691_ = type_39();
                                        phi_14693_ = u32();
                                        phi_14695_ = u32();
                                    }
                                    let _e5924 = phi_35856_;
                                    let _e5926 = phi_35563_;
                                    let _e5928 = phi_14688_;
                                    let _e5930 = phi_14691_;
                                    let _e5932 = phi_14693_;
                                    let _e5934 = phi_14695_;
                                    phi_35200_ = _e5926;
                                    phi_33275_ = _e5924;
                                    phi_14687_ = _e5928;
                                    phi_14690_ = _e5930;
                                    phi_14692_ = _e5932;
                                    phi_14694_ = _e5934;
                                    phi_35576_ = _e5924;
                                    phi_35562_ = _e5926;
                                    if !(_e2629) {
                                        break;
                                    }
                                }
                                let _e5937 = phi_35576_;
                                let _e5939 = phi_35562_;
                                if _e5939 {
                                    break;
                                }
                                if (_e345 >= _e361) {
                                    let _e6943 = local_102;
                                    let _e6946 = local_103;
                                    let _e6949 = local_104;
                                    let _e6952 = local_105;
                                    let _e6958 = local_106;
                                    phi_19499_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e1607.x, _e1607.y, _e1607.z, f32(_e345)), vec4<f32>(_e6946.x, _e6949.y, _e6952.z, f32(_e6943)), vec4<u32>(_e5937, (_e2612 & 4294967294u), 0u, 0u)), _e1609, _e6958);
                                    phi_19500_ = type_71();
                                    phi_19501_ = true;
                                } else {
                                    let _e5942 = ((_e5937 * 747796405u) + 2891336453u);
                                    let _e5950 = (((_e5942 >> bitcast<u32>((((_e5942 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5942) * 277803737u);
                                    let _e5953 = ((_e5950 >> bitcast<u32>(22i)) ^ _e5950);
                                    let _e5956 = f32((_e5953 >> bitcast<u32>(8i)));
                                    let _e5959 = ((_e5953 * 747796405u) + 2891336453u);
                                    let _e5967 = (((_e5959 >> bitcast<u32>((((_e5959 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5959) * 277803737u);
                                    let _e5970 = ((_e5967 >> bitcast<u32>(22i)) ^ _e5967);
                                    let _e5973 = f32((_e5970 >> bitcast<u32>(8i)));
                                    let _e5975 = ((_e5970 * 747796405u) + 2891336453u);
                                    let _e5983 = (((_e5975 >> bitcast<u32>((((_e5975 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5975) * 277803737u);
                                    let _e5986 = ((_e5983 >> bitcast<u32>(22i)) ^ _e5983);
                                    let _e5991 = (_e2604 == 0u);
                                    if _e5991 {
                                        phi_30299_ = vec3<f32>();
                                        phi_30300_ = true;
                                    } else {
                                        if (_e2604 == 2u) {
                                            let _e5997 = (((_e1684 * _e1712.x) + (_e1685 * _e1712.y)) + (_e1686 * _e1712.z));
                                            if (_e5997 != _e5997) {
                                                phi_30340_ = true;
                                            } else {
                                                phi_30340_ = (0f >= _e5997);
                                            }
                                            let _e6001 = phi_30340_;
                                            let _e6002 = select(_e5997, 0f, _e6001);
                                            let _e6003 = (1f / _e2594);
                                            let _e6007 = ((_e6003 * _e6003) * (1f - (_e6002 * _e6002)));
                                            if (_e6007 >= 1f) {
                                                phi_30359_ = 1f;
                                            } else {
                                                let _e6010 = sqrt((1f - _e6007));
                                                if (_e6002 != _e6002) {
                                                    phi_30370_ = true;
                                                } else {
                                                    phi_30370_ = (_e6010 <= _e6002);
                                                }
                                                let _e6014 = phi_30370_;
                                                let _e6019 = pow(((1f - _e2594) / (1f + _e2594)), 2f);
                                                phi_30359_ = (_e6019 + ((1f - _e6019) * pow((1f - select(_e6002, _e6010, _e6014)), 5f)));
                                            }
                                            let _e6026 = phi_30359_;
                                            phi_30290_ = select(true, false, ((f32((_e5986 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6026));
                                        } else {
                                            phi_30290_ = true;
                                        }
                                        let _e6030 = phi_30290_;
                                        if _e6030 {
                                            let _e6035 = (((_e1684 * _e1760) + (_e1685 * _e1761)) + (_e1686 * _e1763));
                                            let _e6040 = (((_e1684 * _e1756) + (_e1685 * _e1766)) + (_e1686 * _e1767));
                                            let _e6045 = (((_e1684 * _e1712.x) + (_e1685 * _e1712.y)) + (_e1686 * _e1712.z));
                                            if (_e6045 <= 0f) {
                                                phi_30524_ = vec3<f32>();
                                                phi_30525_ = true;
                                            } else {
                                                let _e6048 = (_e2602 * _e6035);
                                                let _e6049 = (_e2602 * _e6040);
                                                let _e6056 = (1f / sqrt((((_e6048 * _e6048) + (_e6049 * _e6049)) + (_e6045 * _e6045))));
                                                let _e6057 = (_e6048 * _e6056);
                                                let _e6058 = (_e6049 * _e6056);
                                                let _e6059 = (_e6045 * _e6056);
                                                let _e6062 = ((_e6057 * _e6057) + (_e6058 * _e6058));
                                                if (_e6062 > 0f) {
                                                    let _e6065 = sqrt(_e6062);
                                                    phi_30426_ = vec3<f32>((-(_e6058) / _e6065), (_e6057 / _e6065), 0f);
                                                } else {
                                                    phi_30426_ = vec3<f32>(1f, 0f, 0f);
                                                }
                                                let _e6070 = phi_30426_;
                                                let _e6083 = sqrt((_e5956 * 0.000000059604645f));
                                                let _e6084 = (_e5973 * 0.00000037450704f);
                                                let _e6086 = (_e6083 * cos(_e6084));
                                                let _e6088 = (0.5f * (1f + _e6059));
                                                let _e6091 = (1f - (_e6086 * _e6086));
                                                if (_e6091 != _e6091) {
                                                    phi_30582_ = true;
                                                } else {
                                                    phi_30582_ = (0f >= _e6091);
                                                }
                                                let _e6095 = phi_30582_;
                                                let _e6102 = (((1f - _e6088) * sqrt(select(_e6091, 0f, _e6095))) + ((_e6088 * _e6083) * sin(_e6084)));
                                                let _e6113 = (_e6091 - (_e6102 * _e6102));
                                                if (_e6113 != _e6113) {
                                                    phi_30597_ = true;
                                                } else {
                                                    phi_30597_ = (0f >= _e6113);
                                                }
                                                let _e6117 = phi_30597_;
                                                let _e6119 = sqrt(select(_e6113, 0f, _e6117));
                                                let _e6125 = (((_e6070.z * _e6086) + (((_e6057 * _e6070.y) - (_e6070.x * _e6058)) * _e6102)) + (_e6059 * _e6119));
                                                let _e6126 = (_e2602 * (((_e6070.x * _e6086) + (((_e6058 * _e6070.z) - (_e6070.y * _e6059)) * _e6102)) + (_e6057 * _e6119)));
                                                let _e6127 = (_e2602 * (((_e6070.y * _e6086) + (((_e6059 * _e6070.x) - (_e6070.z * _e6057)) * _e6102)) + (_e6058 * _e6119)));
                                                if (_e6125 != _e6125) {
                                                    phi_30612_ = true;
                                                } else {
                                                    phi_30612_ = (0f >= _e6125);
                                                }
                                                let _e6131 = phi_30612_;
                                                let _e6132 = select(_e6125, 0f, _e6131);
                                                let _e6139 = (1f / sqrt((((_e6126 * _e6126) + (_e6127 * _e6127)) + (_e6132 * _e6132))));
                                                let _e6140 = (_e6126 * _e6139);
                                                let _e6141 = (_e6127 * _e6139);
                                                let _e6142 = (_e6132 * _e6139);
                                                let _e6148 = (2f * (((_e6035 * _e6140) + (_e6040 * _e6141)) + (_e6045 * _e6142)));
                                                let _e6151 = (_e6142 * _e6148);
                                                let _e6154 = (vec3<f32>((_e6140 * _e6148), (_e6141 * _e6148), _e6151) - vec3<f32>(_e6035, _e6040, _e6045));
                                                let _e6155 = ((_e6151 - _e6045) <= 0f);
                                                if _e6155 {
                                                    phi_30521_ = vec3<f32>();
                                                } else {
                                                    phi_30521_ = vec3<f32>((((_e1760 * _e6154.x) + (_e1756 * _e6154.y)) + (_e1712.x * _e6154.z)), (((_e1761 * _e6154.x) + (_e1766 * _e6154.y)) + (_e1712.y * _e6154.z)), (((_e1763 * _e6154.x) + (_e1767 * _e6154.y)) + (_e1712.z * _e6154.z)));
                                                }
                                                let _e6176 = phi_30521_;
                                                phi_30524_ = _e6176;
                                                phi_30525_ = _e6155;
                                            }
                                            let _e6178 = phi_30524_;
                                            let _e6180 = phi_30525_;
                                            phi_30295_ = select(_e6178, vec3<f32>(0f, 0f, 0f), vec3(_e6180));
                                        } else {
                                            phi_30295_ = vec3<f32>();
                                        }
                                        let _e6184 = phi_30295_;
                                        phi_30299_ = _e6184;
                                        phi_30300_ = select(true, false, _e6030);
                                    }
                                    let _e6187 = phi_30299_;
                                    let _e6189 = phi_30300_;
                                    if _e6189 {
                                        let _e6190 = (_e5956 * 0.00000037450704f);
                                        let _e6192 = ((_e5973 * 0.00000011920929f) - 1f);
                                        let _e6194 = (1f - (_e6192 * _e6192));
                                        if (_e6194 != _e6194) {
                                            phi_30712_ = true;
                                        } else {
                                            phi_30712_ = (0f >= _e6194);
                                        }
                                        let _e6198 = phi_30712_;
                                        let _e6200 = sqrt(select(_e6194, 0f, _e6198));
                                        let _e6205 = (_e1712.x + (_e6200 * cos(_e6190)));
                                        let _e6206 = (_e1712.y + (_e6200 * sin(_e6190)));
                                        let _e6207 = (_e1712.z + _e6192);
                                        let _e6213 = sqrt((((_e6205 * _e6205) + (_e6206 * _e6206)) + (_e6207 * _e6207)));
                                        if (_e6213 < 0.000001f) {
                                            phi_30686_ = _e1712;
                                        } else {
                                            let _e6215 = (1f / _e6213);
                                            phi_30686_ = vec3<f32>((_e6205 * _e6215), (_e6206 * _e6215), (_e6207 * _e6215));
                                        }
                                        let _e6221 = phi_30686_;
                                        phi_30307_ = _e6221;
                                    } else {
                                        phi_30307_ = _e6187;
                                    }
                                    let _e6223 = phi_30307_;
                                    let _e6231 = (((_e6223.x * _e1712.x) + (_e6223.y * _e1712.y)) + (_e6223.z * _e1712.z));
                                    if (_e6231 != _e6231) {
                                        phi_30727_ = true;
                                    } else {
                                        phi_30727_ = (0f >= _e6231);
                                    }
                                    let _e6235 = phi_30727_;
                                    let _e6237 = (select(_e6231, 0f, _e6235) * 0.31830987f);
                                    if _e5991 {
                                        if (_e6231 <= 0f) {
                                            phi_31524_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_31524_ = (_e2598 * (_e6231 * 0.31830987f));
                                        }
                                        let _e6712 = phi_31524_;
                                        phi_19340_ = type_56(_e6712, _e6237);
                                        phi_19341_ = type_56();
                                        phi_19342_ = true;
                                    } else {
                                        let _e6242 = (((_e1684 * _e1760) + (_e1685 * _e1761)) + (_e1686 * _e1763));
                                        let _e6247 = (((_e1684 * _e1756) + (_e1685 * _e1766)) + (_e1686 * _e1767));
                                        let _e6252 = (((_e1684 * _e1712.x) + (_e1685 * _e1712.y)) + (_e1686 * _e1712.z));
                                        let _e6257 = (((_e6223.x * _e1760) + (_e6223.y * _e1761)) + (_e6223.z * _e1763));
                                        let _e6262 = (((_e6223.x * _e1756) + (_e6223.y * _e1766)) + (_e6223.z * _e1767));
                                        if (_e6252 <= 0f) {
                                            phi_19304_ = type_56();
                                            phi_19305_ = true;
                                        } else {
                                            let _e6264 = (_e6231 <= 0f);
                                            if _e6264 {
                                                phi_19303_ = type_56();
                                            } else {
                                                let _e6265 = (_e6242 + _e6257);
                                                let _e6266 = (_e6247 + _e6262);
                                                let _e6267 = (_e6252 + _e6231);
                                                let _e6274 = (1f / sqrt((((_e6265 * _e6265) + (_e6266 * _e6266)) + (_e6267 * _e6267))));
                                                let _e6277 = (_e6267 * _e6274);
                                                if (_e6277 <= 0f) {
                                                    phi_30837_ = 0f;
                                                } else {
                                                    let _e6279 = (_e2602 * _e2602);
                                                    let _e6283 = ((((_e6279 - 1f) * _e6277) * _e6277) + 1f);
                                                    phi_30837_ = (_e6279 / ((3.1415927f * _e6283) * _e6283));
                                                }
                                                let _e6288 = phi_30837_;
                                                let _e6289 = (_e6252 * _e6252);
                                                if (_e6289 >= 1f) {
                                                    phi_30865_ = 0f;
                                                } else {
                                                    phi_30865_ = ((sqrt((1f + (((_e2602 * _e2602) * ((_e6242 * _e6242) + (_e6247 * _e6247))) / _e6289))) - 1f) * 0.5f);
                                                }
                                                let _e6302 = phi_30865_;
                                                let _e6303 = (1f + _e6302);
                                                let _e6304 = (_e6231 * _e6231);
                                                if (_e6304 >= 1f) {
                                                    phi_30893_ = 0f;
                                                } else {
                                                    phi_30893_ = ((sqrt((1f + (((_e2602 * _e2602) * ((_e6257 * _e6257) + (_e6262 * _e6262))) / _e6304))) - 1f) * 0.5f);
                                                }
                                                let _e6317 = phi_30893_;
                                                let _e6324 = (((_e6242 * (_e6265 * _e6274)) + (_e6247 * (_e6266 * _e6274))) + (_e6252 * _e6277));
                                                if (_e2600 == 0u) {
                                                    let _e6619 = (1f - _e6324);
                                                    if (_e6619 != _e6619) {
                                                        phi_31396_ = true;
                                                    } else {
                                                        phi_31396_ = (0f >= _e6619);
                                                    }
                                                    let _e6623 = phi_31396_;
                                                    let _e6625 = pow(select(_e6619, 0f, _e6623), 5f);
                                                    let _e6626 = (1f - _e6625);
                                                    phi_19291_ = vec3<f32>(((_e2598.x * _e6626) + _e6625), ((_e2598.y * _e6626) + _e6625), ((_e2598.z * _e6626) + _e6625));
                                                } else {
                                                    if (_e2600 == 1u) {
                                                        let _e6592 = (1f / _e2594);
                                                        let _e6596 = ((_e6592 * _e6592) * (1f - (_e6324 * _e6324)));
                                                        if (_e6596 >= 1f) {
                                                            phi_31357_ = 1f;
                                                        } else {
                                                            let _e6599 = sqrt((1f - _e6596));
                                                            if (_e6324 != _e6324) {
                                                                phi_31368_ = true;
                                                            } else {
                                                                phi_31368_ = (_e6599 <= _e6324);
                                                            }
                                                            let _e6603 = phi_31368_;
                                                            let _e6608 = pow(((1f - _e2594) / (1f + _e2594)), 2f);
                                                            phi_31357_ = (_e6608 + ((1f - _e6608) * pow((1f - select(_e6324, _e6599, _e6603)), 5f)));
                                                        }
                                                        let _e6615 = phi_31357_;
                                                        phi_19290_ = vec3(_e6615);
                                                    } else {
                                                        let _e6328 = select(_e6324, 0f, (_e6324 < 0f));
                                                        let _e6330 = select(_e6328, 1f, (_e6328 > 1f));
                                                        let _e6332 = (1f - (_e6330 * _e6330));
                                                        let _e6345 = ((_e2598.x * _e2598.x) - (_e2596.x * _e2596.x));
                                                        let _e6346 = ((_e2598.y * _e2598.y) - (_e2596.y * _e2596.y));
                                                        let _e6347 = ((_e2598.z * _e2598.z) - (_e2596.z * _e2596.z));
                                                        let _e6349 = (_e2598.x * (_e2596.x + _e2596.x));
                                                        let _e6351 = (_e2598.y * (_e2596.y + _e2596.y));
                                                        let _e6353 = (_e2598.z * (_e2596.z + _e2596.z));
                                                        let _e6360 = ((_e6345 * _e6345) + (_e6349 * _e6349));
                                                        let _e6361 = ((_e6346 * _e6346) + (_e6351 * _e6351));
                                                        let _e6362 = ((_e6347 * _e6347) + (_e6353 * _e6353));
                                                        let _e6367 = ((_e6332 * _e6345) / _e6360);
                                                        let _e6368 = ((_e6332 * _e6346) / _e6361);
                                                        let _e6369 = ((_e6332 * _e6347) / _e6362);
                                                        let _e6370 = (1f - _e6367);
                                                        let _e6371 = (1f - _e6368);
                                                        let _e6372 = (1f - _e6369);
                                                        let _e6374 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e6367, _e6368, _e6369));
                                                        let _e6375 = (_e6332 * _e6349);
                                                        let _e6376 = (_e6332 * _e6351);
                                                        let _e6377 = (_e6332 * _e6353);
                                                        let _e6382 = (vec3<f32>(_e6375, _e6376, _e6377) / vec3<f32>(_e6360, _e6361, _e6362));
                                                        let _e6395 = ((_e6374.x * _e6374.x) + (_e6382.x * _e6382.x));
                                                        let _e6396 = ((_e6374.y * _e6374.y) + (_e6382.y * _e6382.y));
                                                        let _e6397 = ((_e6374.z * _e6374.z) + (_e6382.z * _e6382.z));
                                                        if (_e6395 != _e6395) {
                                                            phi_31027_ = true;
                                                        } else {
                                                            phi_31027_ = (0f >= _e6395);
                                                        }
                                                        let _e6401 = phi_31027_;
                                                        let _e6403 = sqrt(select(_e6395, 0f, _e6401));
                                                        if (_e6396 != _e6396) {
                                                            phi_31042_ = true;
                                                        } else {
                                                            phi_31042_ = (0f >= _e6396);
                                                        }
                                                        let _e6407 = phi_31042_;
                                                        let _e6409 = sqrt(select(_e6396, 0f, _e6407));
                                                        if (_e6397 != _e6397) {
                                                            phi_31057_ = true;
                                                        } else {
                                                            phi_31057_ = (0f >= _e6397);
                                                        }
                                                        let _e6413 = phi_31057_;
                                                        let _e6415 = sqrt(select(_e6397, 0f, _e6413));
                                                        let _e6419 = ((_e6403 + _e6370) * 0.5f);
                                                        let _e6420 = ((_e6409 + _e6371) * 0.5f);
                                                        let _e6421 = ((_e6415 + _e6372) * 0.5f);
                                                        if (_e6419 != _e6419) {
                                                            phi_31084_ = true;
                                                        } else {
                                                            phi_31084_ = (0f >= _e6419);
                                                        }
                                                        let _e6425 = phi_31084_;
                                                        let _e6427 = sqrt(select(_e6419, 0f, _e6425));
                                                        if (_e6420 != _e6420) {
                                                            phi_31099_ = true;
                                                        } else {
                                                            phi_31099_ = (0f >= _e6420);
                                                        }
                                                        let _e6431 = phi_31099_;
                                                        let _e6433 = sqrt(select(_e6420, 0f, _e6431));
                                                        if (_e6421 != _e6421) {
                                                            phi_31114_ = true;
                                                        } else {
                                                            phi_31114_ = (0f >= _e6421);
                                                        }
                                                        let _e6437 = phi_31114_;
                                                        let _e6439 = sqrt(select(_e6421, 0f, _e6437));
                                                        let _e6440 = vec3<f32>(_e6427, _e6433, _e6439);
                                                        let _e6444 = ((_e6403 - _e6370) * 0.5f);
                                                        let _e6445 = ((_e6409 - _e6371) * 0.5f);
                                                        let _e6446 = ((_e6415 - _e6372) * 0.5f);
                                                        if (_e6444 != _e6444) {
                                                            phi_31141_ = true;
                                                        } else {
                                                            phi_31141_ = (0f >= _e6444);
                                                        }
                                                        let _e6450 = phi_31141_;
                                                        let _e6452 = sqrt(select(_e6444, 0f, _e6450));
                                                        if (_e6445 != _e6445) {
                                                            phi_31156_ = true;
                                                        } else {
                                                            phi_31156_ = (0f >= _e6445);
                                                        }
                                                        let _e6456 = phi_31156_;
                                                        let _e6458 = sqrt(select(_e6445, 0f, _e6456));
                                                        if (_e6446 != _e6446) {
                                                            phi_31171_ = true;
                                                        } else {
                                                            phi_31171_ = (0f >= _e6446);
                                                        }
                                                        let _e6462 = phi_31171_;
                                                        let _e6464 = sqrt(select(_e6446, 0f, _e6462));
                                                        if ((_e6375 / _e6360) < 0f) {
                                                            phi_19167_ = -(_e6452);
                                                        } else {
                                                            phi_19167_ = _e6452;
                                                        }
                                                        let _e6468 = phi_19167_;
                                                        if ((_e6376 / _e6361) < 0f) {
                                                            phi_19175_ = -(_e6458);
                                                        } else {
                                                            phi_19175_ = _e6458;
                                                        }
                                                        let _e6472 = phi_19175_;
                                                        if ((_e6377 / _e6362) < 0f) {
                                                            phi_19183_ = -(_e6464);
                                                        } else {
                                                            phi_19183_ = _e6464;
                                                        }
                                                        let _e6476 = phi_19183_;
                                                        let _e6480 = (_e2596.x * _e6330);
                                                        let _e6481 = (_e2596.y * _e6330);
                                                        let _e6482 = (_e2596.z * _e6330);
                                                        let _e6492 = vec3<f32>(((_e2598.x * _e6427) - (_e2596.x * _e6468)), ((_e2598.y * _e6433) - (_e2596.y * _e6472)), ((_e2598.z * _e6439) - (_e2596.z * _e6476)));
                                                        let _e6499 = ((_e2598.x * _e6468) + (_e2596.x * _e6427));
                                                        let _e6500 = ((_e2598.y * _e6472) + (_e2596.y * _e6433));
                                                        let _e6501 = ((_e2598.z * _e6476) + (_e2596.z * _e6439));
                                                        let _e6503 = vec3<f32>((_e2598.x * _e6330), (_e2598.y * _e6330), (_e2598.z * _e6330));
                                                        let _e6504 = (_e6503 - _e6440);
                                                        let _e6505 = (_e6480 - _e6468);
                                                        let _e6506 = (_e6481 - _e6472);
                                                        let _e6507 = (_e6482 - _e6476);
                                                        let _e6520 = (_e6503 + _e6440);
                                                        let _e6521 = (_e6480 + _e6468);
                                                        let _e6522 = (_e6481 + _e6472);
                                                        let _e6523 = (_e6482 + _e6476);
                                                        let _e6533 = ((_e6520.x * _e6520.x) + (_e6521 * _e6521));
                                                        let _e6534 = ((_e6520.y * _e6520.y) + (_e6522 * _e6522));
                                                        let _e6535 = ((_e6520.z * _e6520.z) + (_e6523 * _e6523));
                                                        let _e6545 = vec3(_e6330);
                                                        let _e6546 = (_e6545 - _e6492);
                                                        let _e6547 = -(vec3<f32>(_e6499, _e6500, _e6501));
                                                        let _e6563 = (_e6545 + _e6492);
                                                        let _e6573 = ((_e6563.x * _e6563.x) + (_e6499 * _e6499));
                                                        let _e6574 = ((_e6563.y * _e6563.y) + (_e6500 * _e6500));
                                                        let _e6575 = ((_e6563.z * _e6563.z) + (_e6501 * _e6501));
                                                        phi_19290_ = vec3<f32>((((((_e6504.x * _e6504.x) + (_e6505 * _e6505)) / select(0.000000000000000000000000000001f, _e6533, (_e6533 > 0.000000000000000000000000000001f))) + (((_e6546.x * _e6546.x) + (_e6547.x * _e6547.x)) / select(0.000000000000000000000000000001f, _e6573, (_e6573 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6504.y * _e6504.y) + (_e6506 * _e6506)) / select(0.000000000000000000000000000001f, _e6534, (_e6534 > 0.000000000000000000000000000001f))) + (((_e6546.y * _e6546.y) + (_e6547.y * _e6547.y)) / select(0.000000000000000000000000000001f, _e6574, (_e6574 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6504.z * _e6504.z) + (_e6507 * _e6507)) / select(0.000000000000000000000000000001f, _e6535, (_e6535 > 0.000000000000000000000000000001f))) + (((_e6546.z * _e6546.z) + (_e6547.z * _e6547.z)) / select(0.000000000000000000000000000001f, _e6575, (_e6575 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e6618 = phi_19290_;
                                                    phi_19291_ = _e6618;
                                                }
                                                let _e6638 = phi_19291_;
                                                let _e6640 = (4f * _e6252);
                                                phi_19303_ = type_56((_e6638 * (((1f / (_e6303 + _e6317)) * _e6288) / _e6640)), ((_e6288 / _e6303) / _e6640));
                                            }
                                            let _e6647 = phi_19303_;
                                            phi_19304_ = _e6647;
                                            phi_19305_ = _e6264;
                                        }
                                        let _e6649 = phi_19304_;
                                        let _e6651 = phi_19305_;
                                        if _e6651 {
                                            phi_19310_ = type_56(vec3<f32>(0f, 0f, 0f), 0f);
                                        } else {
                                            phi_19310_ = _e6649;
                                        }
                                        let _e6653 = phi_19310_;
                                        let _e6656 = (_e2604 == 1u);
                                        if _e6656 {
                                            phi_19338_ = _e6653;
                                            phi_19339_ = type_56();
                                        } else {
                                            if (_e6252 != _e6252) {
                                                phi_31433_ = true;
                                            } else {
                                                phi_31433_ = (0f >= _e6252);
                                            }
                                            let _e6660 = phi_31433_;
                                            let _e6661 = select(_e6252, 0f, _e6660);
                                            let _e6662 = (1f / _e2594);
                                            let _e6666 = ((_e6662 * _e6662) * (1f - (_e6661 * _e6661)));
                                            if (_e6666 >= 1f) {
                                                phi_31452_ = 1f;
                                            } else {
                                                let _e6669 = sqrt((1f - _e6666));
                                                if (_e6661 != _e6661) {
                                                    phi_31463_ = true;
                                                } else {
                                                    phi_31463_ = (_e6669 <= _e6661);
                                                }
                                                let _e6673 = phi_31463_;
                                                let _e6678 = pow(((1f - _e2594) / (1f + _e2594)), 2f);
                                                phi_31452_ = (_e6678 + ((1f - _e6678) * pow((1f - select(_e6661, _e6669, _e6673)), 5f)));
                                            }
                                            let _e6685 = phi_31452_;
                                            if (_e6231 <= 0f) {
                                                phi_31502_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_31502_ = (_e2592 * (_e6231 * 0.31830987f));
                                            }
                                            let _e6690 = phi_31502_;
                                            let _e6691 = (1f - _e6685);
                                            phi_19338_ = type_56();
                                            phi_19339_ = type_56((_e6653.member + vec3<f32>((_e6690.x * _e6691), (_e6690.y * _e6691), (_e6690.z * _e6691))), ((_e6685 * _e6653.member_1) + (_e6691 * _e6237)));
                                        }
                                        let _e6705 = phi_19338_;
                                        let _e6707 = phi_19339_;
                                        phi_19340_ = _e6705;
                                        phi_19341_ = _e6707;
                                        phi_19342_ = _e6656;
                                    }
                                    let _e6715 = phi_19340_;
                                    let _e6717 = phi_19341_;
                                    let _e6719 = phi_19342_;
                                    if _e6719 {
                                        phi_19346_ = _e6715;
                                    } else {
                                        phi_19346_ = _e6717;
                                    }
                                    let _e6721 = phi_19346_;
                                    if ((((_e6223.x * _e6223.x) + (_e6223.y * _e6223.y)) + (_e6223.z * _e6223.z)) == 0f) {
                                        phi_37461_ = _e5986;
                                        phi_19485_ = type_71();
                                        phi_19486_ = type_71();
                                        phi_19487_ = true;
                                        phi_19488_ = false;
                                    } else {
                                        let _e6730 = (_e6721.member_1 <= 0f);
                                        if _e6730 {
                                            phi_37462_ = _e5986;
                                            phi_19482_ = type_71();
                                            phi_19483_ = type_71();
                                            phi_19484_ = false;
                                        } else {
                                            let _e6732 = (_e6721.member.x / _e6721.member_1);
                                            let _e6734 = (_e6721.member.y / _e6721.member_1);
                                            let _e6736 = (_e6721.member.z / _e6721.member_1);
                                            let _e6745 = global_1.member.member_4[1u];
                                            if (sqrt((((_e6732 * _e6732) + (_e6734 * _e6734)) + (_e6736 * _e6736))) <= _e6745) {
                                                phi_37455_ = _e5986;
                                                phi_19433_ = _e1607;
                                                phi_19434_ = false;
                                            } else {
                                                let _e6749 = global_1.member.member_7[1u];
                                                if (_e6749 == 1u) {
                                                    let _e6754 = global_1.member.member_4[3u];
                                                    if (f32(_e345) >= _e6754) {
                                                        let _e6757 = select(_e6734, _e6732, (_e6732 > _e6734));
                                                        let _e6759 = select(_e6736, _e6757, (_e6757 > _e6736));
                                                        if (_e6759 != _e6759) {
                                                            phi_31549_ = true;
                                                        } else {
                                                            phi_31549_ = (1f <= _e6759);
                                                        }
                                                        let _e6763 = phi_31549_;
                                                        let _e6764 = select(_e6759, 1f, _e6763);
                                                        let _e6766 = ((_e5986 * 747796405u) + 2891336453u);
                                                        let _e6774 = (((_e6766 >> bitcast<u32>((((_e6766 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6766) * 277803737u);
                                                        let _e6777 = ((_e6774 >> bitcast<u32>(22i)) ^ _e6774);
                                                        let _e6782 = ((f32((_e6777 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6764);
                                                        if _e6782 {
                                                            phi_19422_ = vec3<f32>();
                                                        } else {
                                                            phi_19422_ = vec3<f32>((_e1607.x / _e6764), (_e1607.y / _e6764), (_e1607.z / _e6764));
                                                        }
                                                        let _e6791 = phi_19422_;
                                                        phi_37458_ = _e6777;
                                                        phi_19424_ = _e6791;
                                                        phi_19425_ = select(true, false, _e6782);
                                                    } else {
                                                        phi_37458_ = _e5986;
                                                        phi_19424_ = _e1607;
                                                        phi_19425_ = true;
                                                    }
                                                    let _e6794 = phi_37458_;
                                                    let _e6796 = phi_19424_;
                                                    let _e6798 = phi_19425_;
                                                    phi_37457_ = _e6794;
                                                    phi_19426_ = _e6796;
                                                    phi_19427_ = _e6798;
                                                } else {
                                                    phi_37457_ = _e5986;
                                                    phi_19426_ = _e1607;
                                                    phi_19427_ = true;
                                                }
                                                let _e6800 = phi_37457_;
                                                let _e6802 = phi_19426_;
                                                let _e6804 = phi_19427_;
                                                phi_37455_ = _e6800;
                                                phi_19433_ = select(_e1607, _e6802, vec3(_e6804));
                                                phi_19434_ = select(false, true, _e6804);
                                            }
                                            let _e6809 = phi_37455_;
                                            let _e6811 = phi_19433_;
                                            let _e6813 = phi_19434_;
                                            if _e6813 {
                                                if ((((_e6223.x * _e1699.x) + (_e6223.y * _e1699.y)) + (_e6223.z * _e1699.z)) < 0f) {
                                                    phi_19463_ = -(_e1699);
                                                } else {
                                                    phi_19463_ = _e1699;
                                                }
                                                let _e6852 = phi_19463_;
                                                let _e6869 = local_92;
                                                let _e6872 = local_93;
                                                let _e6875 = local_94;
                                                let _e6878 = local_95;
                                                let _e6884 = local_96;
                                                phi_19479_ = type_71(type_35(vec4<f32>((_e1614 + (_e6852.x * _e1730)), (_e1615 + (_e6852.y * _e1730)), (_e1616 + (_e6852.z * _e1730)), _e318.member.w), vec4<f32>(_e6223.x, _e6223.y, _e6223.z, _e6721.member_1), vec4<f32>((_e6811.x * _e6732), (_e6811.y * _e6734), (_e6811.z * _e6736), f32((_e345 + 1u))), vec4<f32>(_e6872.x, _e6875.y, _e6878.z, f32(_e6869)), vec4<u32>(_e6809, _e2612, 0u, 0u)), _e1609, _e6884);
                                                phi_19480_ = type_71();
                                            } else {
                                                let _e6821 = local_87;
                                                let _e6824 = local_88;
                                                let _e6827 = local_89;
                                                let _e6830 = local_90;
                                                let _e6836 = local_91;
                                                phi_19479_ = type_71();
                                                phi_19480_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e6811.x, _e6811.y, _e6811.z, f32(_e345)), vec4<f32>(_e6824.x, _e6827.y, _e6830.z, f32(_e6821)), vec4<u32>(_e6809, (_e2612 & 4294967294u), 0u, 0u)), _e1609, _e6836);
                                            }
                                            let _e6887 = phi_19479_;
                                            let _e6889 = phi_19480_;
                                            phi_37462_ = _e6809;
                                            phi_19482_ = _e6887;
                                            phi_19483_ = _e6889;
                                            phi_19484_ = select(true, false, _e6813);
                                        }
                                        let _e6892 = phi_37462_;
                                        let _e6894 = phi_19482_;
                                        let _e6896 = phi_19483_;
                                        let _e6898 = phi_19484_;
                                        phi_37461_ = _e6892;
                                        phi_19485_ = _e6894;
                                        phi_19486_ = _e6896;
                                        phi_19487_ = _e6730;
                                        phi_19488_ = _e6898;
                                    }
                                    let _e6900 = phi_37461_;
                                    let _e6902 = phi_19485_;
                                    let _e6904 = phi_19486_;
                                    let _e6906 = phi_19487_;
                                    let _e6908 = phi_19488_;
                                    if _e6906 {
                                        let _e6916 = local_97;
                                        let _e6919 = local_98;
                                        let _e6922 = local_99;
                                        let _e6925 = local_100;
                                        let _e6931 = local_101;
                                        phi_19494_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e1607.x, _e1607.y, _e1607.z, f32(_e345)), vec4<f32>(_e6919.x, _e6922.y, _e6925.z, f32(_e6916)), vec4<u32>(_e6900, (_e2612 & 4294967294u), 0u, 0u)), _e1609, _e6931);
                                    } else {
                                        phi_19494_ = _e6904;
                                    }
                                    let _e6934 = phi_19494_;
                                    phi_19499_ = _e6934;
                                    phi_19500_ = _e6902;
                                    phi_19501_ = select(_e6908, true, _e6906);
                                }
                                let _e6961 = phi_19499_;
                                let _e6963 = phi_19500_;
                                let _e6965 = phi_19501_;
                                phi_19505_ = _e6961;
                                phi_19506_ = _e6963;
                                phi_19507_ = _e6965;
                            } else {
                                phi_19505_ = _e2608;
                                phi_19506_ = type_71();
                                phi_19507_ = true;
                            }
                            let _e6967 = phi_19505_;
                            let _e6969 = phi_19506_;
                            let _e6971 = phi_19507_;
                            phi_19511_ = _e6967;
                            phi_19512_ = _e6969;
                            phi_19513_ = _e6971;
                        }
                        let _e7183 = phi_19511_;
                        let _e7185 = phi_19512_;
                        let _e7187 = phi_19513_;
                        phi_19517_ = _e7183;
                        phi_19518_ = _e7185;
                        phi_19519_ = _e7187;
                    }
                    let _e7695 = phi_19517_;
                    let _e7697 = phi_19518_;
                    let _e7699 = phi_19519_;
                    if _e7699 {
                        phi_19523_ = _e7695;
                    } else {
                        phi_19523_ = _e7697;
                    }
                    let _e7701 = phi_19523_;
                    phi_19527_ = vec4<f32>(0f, 0f, 0f, 0f);
                    phi_19528_ = _e7701.member_2;
                    phi_19529_ = _e7701.member_1;
                    phi_19530_ = _e7701.member;
                }
                let _e7751 = phi_19527_;
                let _e7753 = phi_19528_;
                let _e7755 = phi_19529_;
                let _e7757 = phi_19530_;
                if _e315 {
                } else {
                    break;
                }
                global_7.member[_e310.member_1] = _e7757;
                let _e7758 = (2u * _e310.member_1);
                if (_e7758 < _e290) {
                } else {
                    break;
                }
                global_8.member[_e7758] = _e7755;
                let _e7762 = (_e7758 + 1u);
                if (_e7762 < _e290) {
                } else {
                    break;
                }
                global_8.member[_e7762] = _e7753;
                if (_e7751.w > 0f) {
                    if (_e310.member_1 < arrayLength((&global_9.member))) {
                    } else {
                        break;
                    }
                    let _e7773 = global_9.member[_e310.member_1][0u];
                    global_9.member[_e310.member_1][0u] = (_e7773 + _e7751.x);
                    let _e7779 = global_9.member[_e310.member_1][1u];
                    global_9.member[_e310.member_1][1u] = (_e7779 + _e7751.y);
                    let _e7785 = global_9.member[_e310.member_1][2u];
                    global_9.member[_e310.member_1][2u] = (_e7785 + _e7751.z);
                    let _e7790 = global_9.member[_e310.member_1][3u];
                    global_9.member[_e310.member_1][3u] = (_e7790 + _e7751.w);
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(64, 1, 1) 
fn answer_sky(@builtin(global_invocation_id) param: vec3<u32>) {
    global = param;
    function_();
}

@compute @workgroup_size(64, 1, 1) 
fn bounce_paths(@builtin(global_invocation_id) param_1: vec3<u32>) {
    global = param_1;
    function_1();
}
