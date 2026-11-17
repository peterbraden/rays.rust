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

struct type_28 {
    member: array<f32>,
}

struct type_30 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
    member_4: vec4<u32>,
}

struct type_32 {
    member: array<type_30>,
}

struct type_34 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
}

struct type_36 {
    member: array<type_34>,
}

struct type_40 {
    member: u32,
    member_1: u32,
}

struct type_46 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_48 {
    member: u32,
    member_1: type_46,
}

struct type_49 {
    member: f32,
    member_1: f32,
}

struct type_50 {
    member: u32,
    member_1: type_49,
}

struct type_51 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_52 {
    member: vec3<f32>,
    member_1: f32,
}

struct type_53 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_57 {
    member: array<vec4<f32>>,
}

struct type_59 {
    member: f32,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
}

struct type_64 {
    member: type_30,
    member_1: type_34,
    member_2: type_34,
}

struct type_66 {
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
@group(0) @binding(5) 
var<storage> global_5: type_28;
@group(0) @binding(6) 
var<storage, read_write> global_6: type_32;
@group(0) @binding(8) 
var<storage, read_write> global_7: type_36;
@group(0) @binding(7) 
var<storage, read_write> global_8: type_57;

fn function_() {
    var phi_437_: type_40;
    var phi_31742_: bool;
    var phi_469_: vec3<f32>;
    var phi_472_: u32;
    var phi_20649_: f32;
    var phi_20661_: type_49;
    var phi_20672_: type_50;
    var phi_20673_: type_50;
    var phi_20677_: type_50;
    var phi_20678_: type_50;
    var phi_20679_: bool;
    var phi_20683_: type_50;
    var phi_20570_: type_50;
    var phi_20721_: bool;
    var phi_20801_: f32;
    var phi_20813_: type_49;
    var phi_20824_: type_50;
    var phi_20825_: type_50;
    var phi_20829_: type_50;
    var phi_20830_: type_50;
    var phi_20831_: bool;
    var phi_20835_: type_50;
    var phi_20769_: f32;
    var phi_20873_: bool;
    var phi_20598_: f32;
    var phi_20600_: f32;
    var phi_20609_: type_50;
    var phi_20613_: type_50;
    var phi_20617_: type_50;
    var phi_20888_: bool;
    var phi_9472_: vec3<f32>;
    var phi_9475_: vec3<f32>;
    var phi_9477_: vec3<f32>;
    var phi_9479_: vec3<f32>;
    var phi_9481_: u32;
    var phi_9565_: u32;
    var phi_9580_: type_49;
    var phi_9581_: type_49;
    var phi_9582_: type_49;
    var phi_21088_: bool;
    var phi_21215_: bool;
    var phi_21230_: bool;
    var phi_9910_: vec3<f32>;
    var phi_9983_: vec3<f32>;
    var phi_9984_: vec3<f32>;
    var phi_9985_: bool;
    var phi_9990_: vec3<f32>;
    var phi_9991_: bool;
    var phi_21354_: bool;
    var phi_21369_: bool;
    var phi_10282_: vec3<f32>;
    var phi_9473_: vec3<f32>;
    var phi_9476_: vec3<f32>;
    var phi_9478_: vec3<f32>;
    var phi_9480_: vec3<f32>;
    var phi_9482_: u32;
    var phi_31725_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_32209_: bool;
    var phi_10339_: type_53;
    var phi_32208_: bool;
    var phi_10341_: type_53;
    var phi_10342_: bool;
    var phi_10347_: type_53;
    var phi_848_: vec3<f32>;
    var phi_850_: vec3<f32>;
    var phi_851_: bool;
    var phi_31817_: bool;
    var phi_861_: vec3<f32>;
    var phi_864_: i32;
    var phi_866_: u32;
    var phi_21638_: bool;
    var phi_1288_: u32;
    var phi_1318_: u32;
    var phi_1289_: u32;
    var phi_1319_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1337_: u32;
    var phi_2642_: type_48;
    var phi_2650_: type_46;
    var phi_2652_: type_46;
    var phi_2653_: bool;
    var phi_2654_: type_46;
    var phi_2655_: bool;
    var phi_2656_: bool;
    var phi_21749_: f32;
    var phi_21761_: type_49;
    var phi_21772_: type_50;
    var phi_21773_: type_50;
    var phi_21777_: type_50;
    var phi_21778_: type_50;
    var phi_21779_: bool;
    var phi_21783_: type_50;
    var phi_21670_: type_50;
    var phi_21821_: bool;
    var phi_21901_: f32;
    var phi_21913_: type_49;
    var phi_21924_: type_50;
    var phi_21925_: type_50;
    var phi_21929_: type_50;
    var phi_21930_: type_50;
    var phi_21931_: bool;
    var phi_21935_: type_50;
    var phi_21869_: f32;
    var phi_21973_: bool;
    var phi_21698_: f32;
    var phi_21700_: f32;
    var phi_21709_: type_50;
    var phi_21713_: type_50;
    var phi_21717_: type_50;
    var phi_2674_: vec3<f32>;
    var phi_2677_: vec3<f32>;
    var phi_2679_: vec3<f32>;
    var phi_2681_: vec3<f32>;
    var phi_2683_: u32;
    var phi_2767_: u32;
    var phi_2782_: type_49;
    var phi_2783_: type_49;
    var phi_2784_: type_49;
    var phi_22172_: bool;
    var phi_22299_: bool;
    var phi_22314_: bool;
    var phi_3112_: vec3<f32>;
    var phi_22436_: f32;
    var phi_22448_: type_49;
    var phi_22459_: type_50;
    var phi_22460_: type_50;
    var phi_22464_: type_50;
    var phi_22465_: type_50;
    var phi_22466_: bool;
    var phi_22470_: type_50;
    var phi_22404_: f32;
    var phi_22546_: f32;
    var phi_22558_: type_49;
    var phi_22569_: type_50;
    var phi_22570_: type_50;
    var phi_22574_: type_50;
    var phi_22575_: type_50;
    var phi_22576_: bool;
    var phi_22580_: type_50;
    var phi_22514_: f32;
    var phi_22341_: vec3<f32>;
    var phi_22342_: u32;
    var phi_22712_: bool;
    var phi_22375_: vec3<f32>;
    var phi_22376_: u32;
    var local_11: vec3<f32>;
    var phi_22383_: type_51;
    var phi_22387_: type_51;
    var phi_3171_: vec3<f32>;
    var phi_3172_: vec3<f32>;
    var phi_2675_: vec3<f32>;
    var phi_2678_: vec3<f32>;
    var phi_2680_: vec3<f32>;
    var phi_2682_: vec3<f32>;
    var phi_2684_: u32;
    var phi_31785_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_31879_: bool;
    var phi_3192_: type_46;
    var phi_31878_: bool;
    var phi_3193_: type_46;
    var phi_31877_: bool;
    var phi_3216_: vec3<f32>;
    var phi_3217_: i32;
    var phi_3218_: u32;
    var phi_31876_: bool;
    var phi_862_: vec3<f32>;
    var phi_865_: i32;
    var phi_867_: u32;
    var phi_3219_: bool;
    var phi_31875_: bool;
    var phi_3553_: vec3<f32>;
    var phi_3556_: u32;
    var phi_22840_: f32;
    var phi_22855_: f32;
    var phi_22870_: f32;
    var phi_3627_: i32;
    var phi_3657_: i32;
    var phi_3665_: vec3<f32>;
    var phi_3668_: i32;
    var phi_3691_: vec3<f32>;
    var phi_3694_: i32;
    var phi_3692_: vec3<f32>;
    var phi_3695_: i32;
    var phi_3666_: vec3<f32>;
    var phi_3669_: i32;
    var phi_3893_: vec3<f32>;
    var phi_3894_: vec3<f32>;
    var phi_3895_: vec3<f32>;
    var phi_3554_: vec3<f32>;
    var phi_3557_: u32;
    var phi_31880_: bool;
    var local_15: vec3<f32>;
    var phi_31928_: bool;
    var phi_3906_: f32;
    var phi_3930_: f32;
    var phi_3932_: u32;
    var phi_3935_: f32;
    var phi_4038_: f32;
    var phi_4039_: f32;
    var phi_4055_: f32;
    var phi_3933_: u32;
    var phi_3936_: f32;
    var phi_4058_: u32;
    var phi_4061_: f32;
    var phi_4169_: f32;
    var phi_4059_: u32;
    var phi_4062_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_22986_: f32;
    var phi_22998_: type_49;
    var phi_23009_: type_50;
    var phi_23010_: type_50;
    var phi_23014_: type_50;
    var phi_23015_: type_50;
    var phi_23016_: bool;
    var phi_23020_: type_50;
    var phi_22954_: f32;
    var phi_4570_: bool;
    var phi_23146_: f32;
    var phi_23158_: type_49;
    var phi_23169_: type_50;
    var phi_23170_: type_50;
    var phi_23174_: type_50;
    var phi_23175_: type_50;
    var phi_23176_: bool;
    var phi_23180_: type_50;
    var phi_23067_: type_50;
    var phi_23218_: bool;
    var phi_23298_: f32;
    var phi_23310_: type_49;
    var phi_23321_: type_50;
    var phi_23322_: type_50;
    var phi_23326_: type_50;
    var phi_23327_: type_50;
    var phi_23328_: bool;
    var phi_23332_: type_50;
    var phi_23266_: f32;
    var phi_23370_: bool;
    var phi_23095_: f32;
    var phi_23097_: f32;
    var phi_23106_: type_50;
    var phi_23110_: type_50;
    var phi_23114_: type_50;
    var phi_4584_: vec3<f32>;
    var phi_4587_: u32;
    var phi_4671_: u32;
    var phi_4686_: type_49;
    var phi_4687_: type_49;
    var phi_4688_: type_49;
    var phi_23569_: bool;
    var phi_4585_: vec3<f32>;
    var phi_4588_: u32;
    var phi_31907_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_31974_: bool;
    var phi_4713_: vec3<f32>;
    var phi_31973_: bool;
    var phi_4715_: vec3<f32>;
    var phi_4717_: bool;
    var phi_31969_: bool;
    var phi_4728_: vec3<f32>;
    var phi_4729_: bool;
    var phi_31968_: bool;
    var phi_4731_: vec3<f32>;
    var phi_4732_: bool;
    var phi_4733_: bool;
    var phi_31967_: bool;
    var phi_4737_: vec3<f32>;
    var phi_4738_: vec3<f32>;
    var phi_4739_: bool;
    var phi_4740_: vec3<f32>;
    var phi_4741_: bool;
    var phi_5061_: vec3<f32>;
    var phi_5064_: u32;
    var phi_23640_: f32;
    var phi_23655_: f32;
    var phi_23670_: f32;
    var phi_5135_: i32;
    var phi_5165_: i32;
    var phi_5173_: vec3<f32>;
    var phi_5176_: i32;
    var phi_5199_: vec3<f32>;
    var phi_5202_: i32;
    var phi_5273_: vec3<f32>;
    var phi_5276_: u32;
    var phi_5396_: vec3<f32>;
    var phi_5274_: vec3<f32>;
    var phi_5277_: u32;
    var phi_5200_: vec3<f32>;
    var phi_5203_: i32;
    var phi_5174_: vec3<f32>;
    var phi_5177_: i32;
    var phi_5401_: vec3<f32>;
    var phi_5402_: vec3<f32>;
    var phi_5403_: vec3<f32>;
    var phi_5062_: vec3<f32>;
    var phi_5065_: u32;
    var phi_31962_: bool;
    var phi_32004_: bool;
    var phi_5423_: vec3<f32>;
    var phi_5636_: u32;
    var phi_5637_: bool;
    var phi_5640_: u32;
    var phi_5641_: bool;
    var phi_5642_: bool;
    var phi_5652_: f32;
    var phi_5655_: f32;
    var phi_5657_: u32;
    var phi_5659_: vec3<f32>;
    var phi_5661_: vec3<f32>;
    var phi_5663_: f32;
    var phi_5665_: f32;
    var phi_5703_: u32;
    var phi_5718_: vec3<f32>;
    var phi_5738_: vec3<f32>;
    var phi_23685_: vec3<f32>;
    var phi_5739_: u32;
    var phi_5740_: vec3<f32>;
    var phi_5741_: u32;
    var phi_5742_: vec3<f32>;
    var phi_5743_: u32;
    var phi_5745_: vec3<f32>;
    var phi_5880_: u32;
    var phi_5883_: f32;
    var phi_5885_: f32;
    var phi_5887_: f32;
    var phi_5889_: vec3<f32>;
    var phi_23740_: f32;
    var phi_23746_: f32;
    var phi_23774_: f32;
    var phi_23780_: f32;
    var phi_23808_: f32;
    var phi_23814_: f32;
    var phi_23842_: f32;
    var phi_23848_: f32;
    var phi_23876_: f32;
    var phi_23882_: f32;
    var phi_23910_: f32;
    var phi_23916_: f32;
    var phi_23944_: f32;
    var phi_23950_: f32;
    var phi_23978_: f32;
    var phi_23984_: f32;
    var phi_5881_: u32;
    var phi_5884_: f32;
    var phi_5886_: f32;
    var phi_5888_: f32;
    var phi_5890_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6306_: f32;
    var phi_6358_: vec3<f32>;
    var phi_6359_: f32;
    var phi_6360_: vec3<f32>;
    var phi_6361_: f32;
    var phi_6362_: vec3<f32>;
    var phi_6363_: f32;
    var phi_6365_: vec3<f32>;
    var phi_6366_: f32;
    var phi_6367_: f32;
    var phi_6368_: vec3<f32>;
    var phi_6369_: vec3<f32>;
    var phi_6370_: f32;
    var phi_6371_: f32;
    var phi_6372_: vec3<f32>;
    var phi_6373_: vec3<f32>;
    var phi_6374_: f32;
    var phi_6375_: f32;
    var phi_6376_: vec3<f32>;
    var phi_6377_: vec3<f32>;
    var phi_6378_: f32;
    var phi_6379_: f32;
    var phi_6380_: vec3<f32>;
    var phi_6381_: vec3<f32>;
    var phi_6383_: f32;
    var phi_6384_: f32;
    var phi_6385_: vec3<f32>;
    var phi_6386_: vec3<f32>;
    var phi_6387_: f32;
    var phi_6388_: f32;
    var phi_6389_: f32;
    var phi_6390_: f32;
    var phi_6391_: vec3<f32>;
    var phi_6392_: vec3<f32>;
    var phi_6393_: f32;
    var phi_6394_: f32;
    var phi_5653_: f32;
    var phi_5656_: f32;
    var phi_5658_: u32;
    var phi_5660_: vec3<f32>;
    var phi_5662_: vec3<f32>;
    var phi_5664_: f32;
    var phi_5666_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_24060_: bool;
    var local_29: f32;
    var phi_6426_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6505_: f32;
    var phi_6506_: vec3<f32>;
    var phi_6507_: f32;
    var phi_6509_: vec3<f32>;
    var phi_6512_: u32;
    var phi_6960_: f32;
    var phi_7140_: u32;
    var phi_7143_: f32;
    var phi_7145_: f32;
    var phi_7147_: f32;
    var phi_7149_: vec3<f32>;
    var phi_24149_: f32;
    var phi_24155_: f32;
    var phi_24183_: f32;
    var phi_24189_: f32;
    var phi_24217_: f32;
    var phi_24223_: f32;
    var phi_24251_: f32;
    var phi_24257_: f32;
    var phi_24285_: f32;
    var phi_24291_: f32;
    var phi_24319_: f32;
    var phi_24325_: f32;
    var phi_24353_: f32;
    var phi_24359_: f32;
    var phi_24387_: f32;
    var phi_24393_: f32;
    var phi_7141_: u32;
    var phi_7144_: f32;
    var phi_7146_: f32;
    var phi_7148_: f32;
    var phi_7150_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7563_: f32;
    var phi_24439_: bool;
    var phi_24454_: bool;
    var phi_7656_: vec3<f32>;
    var phi_7657_: vec3<f32>;
    var phi_7658_: vec3<f32>;
    var phi_7659_: vec3<f32>;
    var phi_6510_: vec3<f32>;
    var phi_6513_: u32;
    var phi_7678_: vec3<f32>;
    var phi_7681_: u32;
    var phi_7683_: f32;
    var phi_7984_: vec3<f32>;
    var phi_7985_: u32;
    var phi_7986_: f32;
    var phi_7987_: vec3<f32>;
    var phi_8004_: vec3<f32>;
    var phi_8006_: u32;
    var phi_8009_: vec3<f32>;
    var phi_8011_: f32;
    var phi_8013_: f32;
    var phi_8015_: f32;
    var phi_8035_: bool;
    var phi_8051_: bool;
    var phi_8053_: bool;
    var phi_8055_: bool;
    var phi_8056_: bool;
    var phi_8057_: bool;
    var phi_8058_: bool;
    var phi_8059_: bool;
    var phi_8103_: vec3<f32>;
    var phi_24473_: vec3<f32>;
    var phi_8104_: u32;
    var phi_8105_: vec3<f32>;
    var phi_8106_: u32;
    var phi_8107_: vec3<f32>;
    var phi_8242_: u32;
    var phi_8245_: f32;
    var phi_8247_: f32;
    var phi_8249_: f32;
    var phi_8251_: vec3<f32>;
    var phi_24528_: f32;
    var phi_24534_: f32;
    var phi_24562_: f32;
    var phi_24568_: f32;
    var phi_24596_: f32;
    var phi_24602_: f32;
    var phi_24630_: f32;
    var phi_24636_: f32;
    var phi_24664_: f32;
    var phi_24670_: f32;
    var phi_24698_: f32;
    var phi_24704_: f32;
    var phi_24732_: f32;
    var phi_24738_: f32;
    var phi_24766_: f32;
    var phi_24772_: f32;
    var phi_8243_: u32;
    var phi_8246_: f32;
    var phi_8248_: f32;
    var phi_8250_: f32;
    var phi_8252_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8665_: f32;
    var phi_8691_: vec3<f32>;
    var phi_8692_: f32;
    var phi_8693_: f32;
    var phi_8694_: vec3<f32>;
    var phi_8695_: f32;
    var phi_8696_: f32;
    var phi_8697_: vec3<f32>;
    var phi_8698_: f32;
    var phi_8699_: f32;
    var phi_8700_: vec3<f32>;
    var phi_8701_: f32;
    var phi_8702_: f32;
    var phi_8704_: vec3<f32>;
    var phi_8705_: f32;
    var phi_8706_: f32;
    var phi_8707_: f32;
    var phi_8007_: u32;
    var phi_8010_: vec3<f32>;
    var phi_8012_: f32;
    var phi_8014_: f32;
    var phi_8016_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_8997_: type_52;
    var local_42: f32;
    var phi_8998_: type_52;
    var local_43: f32;
    var local_44: f32;
    var phi_8999_: type_52;
    var phi_9000_: type_52;
    var phi_7679_: vec3<f32>;
    var phi_7682_: u32;
    var phi_7684_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9029_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_32212_: bool;
    var phi_9076_: vec3<f32>;
    var phi_32211_: bool;
    var phi_9077_: vec3<f32>;
    var phi_32210_: bool;
    var phi_9078_: vec3<f32>;
    var phi_32206_: bool;
    var phi_10351_: vec3<f32>;
    var phi_32205_: bool;
    var phi_10352_: vec3<f32>;
    var phi_32204_: bool;
    var phi_470_: vec3<f32>;
    var phi_473_: u32;
    var phi_32203_: bool;
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
            let _e270 = global;
            let _e280 = (_e270.x + (_e270.y * 4194240u));
            let _e284 = global_1.member.member[0u];
            let _e288 = global_1.member.member[1u];
            if (_e280 >= (_e284 * _e288)) {
                phi_437_ = type_40(0u, type_40().member_1);
            } else {
                phi_437_ = type_40(1u, _e280);
            }
            let _e296 = phi_437_;
            if (bitcast<i32>(_e296.member) != 0i) {
                let _e301 = (_e296.member_1 < arrayLength((&global_6.member)));
                if _e301 {
                } else {
                    break;
                }
                let _e306 = global_6.member[_e296.member_1].member_4[1u];
                if ((_e306 & 16u) == 0u) {
                } else {
                    phi_31742_ = false;
                    phi_469_ = vec3<f32>(0f, 0f, 0f);
                    phi_472_ = 0u;
                    loop {
                        let _e310 = phi_31742_;
                        let _e312 = phi_469_;
                        let _e314 = phi_472_;
                        local_55 = _e312;
                        local_56 = _e312;
                        local_57 = _e312;
                        let _e315 = (_e314 < 2u);
                        if _e315 {
                            let _e317 = ((2u * _e296.member_1) + _e314);
                            if (_e317 < arrayLength((&global_7.member))) {
                            } else {
                                phi_32203_ = true;
                                break;
                            }
                            let _e321 = global_7.member[_e317];
                            if ((((_e321.member_2.x * _e321.member_2.x) + (_e321.member_2.y * _e321.member_2.y)) + (_e321.member_2.z * _e321.member_2.z)) == 0f) {
                                phi_32205_ = _e310;
                                phi_10352_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                if (_e321.member.w < 0f) {
                                    let _e1276 = global_1.member.member_8[0u];
                                    let _e1279 = global_1.member.member_8[3u];
                                    if (_e1276 == 4294967295u) {
                                        let _e1281 = (_e1279 == 4294967295u);
                                        if _e1281 {
                                            let _e1284 = global_1.member.member_3[1u];
                                            let _e1287 = global_1.member.member_3[2u];
                                            let _e1290 = ((atan2(_e321.member_1.z, _e321.member_1.x) * 0.15915494f) + 0.5f);
                                            let _e1295 = (((_e1290 - floor(_e1290)) * f32(_e1284)) - 0.5f);
                                            let _e1297 = select(_e321.member_1.y, -1f, (_e321.member_1.y < -1f));
                                            let _e1304 = (((acos(select(_e1297, 1f, (_e1297 > 1f))) * 0.31830987f) * f32(_e1287)) - 0.5f);
                                            let _e1305 = (_e1287 - 1u);
                                            let _e1306 = f32(_e1305);
                                            if (0f <= _e1306) {
                                            } else {
                                                phi_32203_ = true;
                                                break;
                                            }
                                            let _e1309 = select(_e1304, 0f, (_e1304 < 0f));
                                            let _e1311 = select(_e1309, _e1306, (_e1309 > _e1306));
                                            let _e1312 = floor(_e1295);
                                            let _e1313 = floor(_e1311);
                                            let _e1314 = (_e1295 - _e1312);
                                            let _e1315 = (_e1311 - _e1313);
                                            let _e1326 = (_e1284 == 0u);
                                            if _e1326 {
                                                phi_32203_ = true;
                                                break;
                                            }
                                            let _e1327 = (bitcast<u32>((select(0i, select(select(i32(_e1312), i32(-2147483648), (_e1312 < -2147483600f)), 2147483647i, (_e1312 > 2147483500f)), (_e1312 == _e1312)) + bitcast<i32>(_e1284))) % _e1284);
                                            let _e1332 = select(select(u32(_e1313), 0u, (_e1313 < 0f)), 4294967295u, (_e1313 > 4294967000f));
                                            let _e1333 = (_e1332 + 1u);
                                            let _e1338 = global_1.member.member_2[2u];
                                            let _e1339 = (_e1332 * _e1284);
                                            if _e1326 {
                                                phi_32203_ = true;
                                                break;
                                            }
                                            let _e1340 = (_e1327 % _e1284);
                                            let _e1343 = (_e1338 + (3u * (_e1339 + _e1340)));
                                            let _e1345 = global_5.member[_e1343];
                                            let _e1348 = global_5.member[(_e1343 + 1u)];
                                            let _e1351 = global_5.member[(_e1343 + 2u)];
                                            if _e1326 {
                                                phi_32203_ = true;
                                                break;
                                            }
                                            let _e1354 = ((_e1327 + 1u) % _e1284);
                                            let _e1357 = (_e1338 + (3u * (_e1339 + _e1354)));
                                            let _e1359 = global_5.member[_e1357];
                                            let _e1362 = global_5.member[(_e1357 + 1u)];
                                            let _e1365 = global_5.member[(_e1357 + 2u)];
                                            let _e1373 = (vec3<f32>(_e1345, _e1348, _e1351) + vec3<f32>(((_e1359 - _e1345) * _e1314), ((_e1362 - _e1348) * _e1314), ((_e1365 - _e1351) * _e1314)));
                                            let _e1374 = (select(_e1333, _e1305, (_e1305 < _e1333)) * _e1284);
                                            if _e1326 {
                                                phi_32203_ = true;
                                                break;
                                            }
                                            let _e1377 = (_e1338 + (3u * (_e1374 + _e1340)));
                                            let _e1379 = global_5.member[_e1377];
                                            let _e1382 = global_5.member[(_e1377 + 1u)];
                                            let _e1385 = global_5.member[(_e1377 + 2u)];
                                            if _e1326 {
                                                phi_32203_ = true;
                                                break;
                                            }
                                            let _e1389 = (_e1338 + (3u * (_e1374 + _e1354)));
                                            let _e1391 = global_5.member[_e1389];
                                            let _e1394 = global_5.member[(_e1389 + 1u)];
                                            let _e1397 = global_5.member[(_e1389 + 2u)];
                                            let _e1405 = (vec3<f32>(_e1379, _e1382, _e1385) + vec3<f32>(((_e1391 - _e1379) * _e1314), ((_e1394 - _e1382) * _e1314), ((_e1397 - _e1385) * _e1314)));
                                            phi_848_ = (_e1373 + vec3<f32>(((_e1405.x - _e1373.x) * _e1315), ((_e1405.y - _e1373.y) * _e1315), ((_e1405.z - _e1373.z) * _e1315)));
                                        } else {
                                            phi_848_ = vec3<f32>();
                                        }
                                        let _e1421 = phi_848_;
                                        phi_850_ = _e1421;
                                        phi_851_ = select(true, false, _e1281);
                                    } else {
                                        phi_850_ = vec3<f32>();
                                        phi_851_ = true;
                                    }
                                    let _e1424 = phi_850_;
                                    let _e1426 = phi_851_;
                                    if _e1426 {
                                        let _e1427 = (_e1279 != 4294967295u);
                                        if (_e1276 != 4294967295u) {
                                            phi_31817_ = _e310;
                                            phi_861_ = vec3<f32>(0f, 0f, 0f);
                                            phi_864_ = 0i;
                                            phi_866_ = _e1276;
                                            loop {
                                                let _e1430 = phi_31817_;
                                                let _e1432 = phi_861_;
                                                let _e1434 = phi_864_;
                                                let _e1436 = phi_866_;
                                                local_17 = _e1432;
                                                local_18 = _e1432;
                                                local_19 = _e1432;
                                                local_61 = _e1432;
                                                local_62 = _e1432;
                                                if (_e1436 != 4294967295u) {
                                                    let _e1438 = (_e1434 < 2i);
                                                    if _e1438 {
                                                        let _e1440 = global_5.member[_e1436];
                                                        let _e1443 = global_5.member[(_e1436 + 1u)];
                                                        let _e1446 = global_5.member[(_e1436 + 2u)];
                                                        let _e1449 = global_5.member[(_e1436 + 3u)];
                                                        let _e1452 = global_5.member[(_e1436 + 4u)];
                                                        let _e1455 = global_5.member[(_e1436 + 5u)];
                                                        let _e1458 = global_5.member[(_e1436 + 6u)];
                                                        let _e1461 = global_5.member[(_e1436 + 7u)];
                                                        let _e1464 = global_5.member[(_e1436 + 8u)];
                                                        let _e1467 = global_5.member[(_e1436 + 9u)];
                                                        let _e1470 = global_5.member[(_e1436 + 10u)];
                                                        let _e1473 = global_5.member[(_e1436 + 11u)];
                                                        let _e1476 = global_5.member[(_e1436 + 12u)];
                                                        let _e1479 = global_5.member[(_e1436 + 13u)];
                                                        let _e1482 = global_5.member[(_e1436 + 14u)];
                                                        let _e1485 = global_5.member[(_e1436 + 15u)];
                                                        let _e1489 = global_5.member[(_e1436 + 16u)];
                                                        let _e1492 = global_5.member[(_e1436 + 17u)];
                                                        let _e1495 = global_5.member[(_e1436 + 18u)];
                                                        let _e1498 = global_5.member[(_e1436 + 19u)];
                                                        let _e1501 = global_5.member[(_e1436 + 20u)];
                                                        let _e1504 = global_5.member[(_e1436 + 30u)];
                                                        let _e1505 = bitcast<u32>(_e1504);
                                                        let _e1508 = global_5.member[(_e1436 + 31u)];
                                                        let _e1509 = bitcast<u32>(_e1508);
                                                        let _e1512 = global_5.member[(_e1436 + 32u)];
                                                        let _e1514 = (_e321.member_1.y < 0f);
                                                        if _e1514 {
                                                            phi_2654_ = type_46();
                                                            phi_2655_ = true;
                                                            phi_2656_ = false;
                                                        } else {
                                                            let _e1515 = (_e1509 == 4294967295u);
                                                            if _e1515 {
                                                                phi_2652_ = type_46();
                                                                phi_2653_ = false;
                                                            } else {
                                                                let _e1529 = (_e321.member.x - _e1440);
                                                                let _e1530 = (_e321.member.y - _e1443);
                                                                let _e1531 = (_e321.member.z - _e1446);
                                                                let _e1539 = ((((((_e321.member.x * _e321.member.x) + (_e321.member.y * _e321.member.y)) + (_e321.member.z * _e321.member.z)) - (2f * (((_e321.member.x * _e1440) + (_e321.member.y * _e1443)) + (_e321.member.z * _e1446)))) + _e1455) / (sqrt((((_e1529 * _e1529) + (_e1530 * _e1530)) + (_e1531 * _e1531))) + _e1449));
                                                                let _e1540 = (_e1436 + 21u);
                                                                let _e1543 = global_5.member[(_e1436 + 28u)];
                                                                if (_e1539 > _e1543) {
                                                                    phi_2642_ = type_48(0u, type_48().member_1);
                                                                } else {
                                                                    let _e1546 = global_5.member[_e1540];
                                                                    if (_e1539 != _e1539) {
                                                                        phi_21638_ = true;
                                                                    } else {
                                                                        phi_21638_ = (_e1546 >= _e1539);
                                                                    }
                                                                    let _e1550 = phi_21638_;
                                                                    let _e1551 = select(_e1539, _e1546, _e1550);
                                                                    phi_1288_ = 0u;
                                                                    loop {
                                                                        let _e1553 = phi_1288_;
                                                                        local_8 = _e1553;
                                                                        local_9 = _e1553;
                                                                        local_10 = _e1553;
                                                                        if (_e1553 < 8u) {
                                                                            let _e1557 = global_5.member[(_e1540 + _e1553)];
                                                                            let _e1558 = (_e1557 <= _e1551);
                                                                            if _e1558 {
                                                                                phi_1318_ = (_e1553 + 1u);
                                                                            } else {
                                                                                phi_1318_ = u32();
                                                                            }
                                                                            let _e1561 = phi_1318_;
                                                                            phi_1289_ = _e1561;
                                                                            phi_1319_ = _e1558;
                                                                        } else {
                                                                            phi_1289_ = u32();
                                                                            phi_1319_ = false;
                                                                        }
                                                                        let _e1563 = phi_1289_;
                                                                        let _e1565 = phi_1319_;
                                                                        phi_1288_ = _e1563;
                                                                        if !(_e1565) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e1568 = local_8;
                                                                    if (_e1568 < 1u) {
                                                                        phi_1337_ = 1u;
                                                                    } else {
                                                                        let _e1571 = local_9;
                                                                        let _e1574 = local_10;
                                                                        phi_1337_ = select(_e1574, 7u, (_e1571 > 7u));
                                                                    }
                                                                    let _e1577 = phi_1337_;
                                                                    let _e1578 = (_e1577 - 1u);
                                                                    let _e1581 = global_5.member[(_e1540 + _e1578)];
                                                                    let _e1584 = global_5.member[(_e1540 + _e1577)];
                                                                    let _e1587 = ((_e1551 - _e1581) / (_e1584 - _e1581));
                                                                    let _e1588 = select(_e321.member_1.y, 0f, _e1514);
                                                                    let _e1594 = (sqrt((asin(select(_e1588, 1f, (_e1588 > 1f))) * 0.63661975f)) * 127f);
                                                                    let _e1596 = (atan2(_e321.member_1.z, _e321.member_1.x) * 0.15915494f);
                                                                    let _e1599 = ((_e1596 - floor(_e1596)) * 256f);
                                                                    let _e1600 = floor(_e1599);
                                                                    let _e1606 = (select(select(u32(_e1600), 0u, (_e1600 < 0f)), 4294967295u, (_e1600 > 4294967000f)) % 256u);
                                                                    let _e1607 = floor(_e1594);
                                                                    let _e1612 = select(select(u32(_e1607), 0u, (_e1607 < 0f)), 4294967295u, (_e1607 > 4294967000f));
                                                                    let _e1614 = select(_e1612, 126u, (126u < _e1612));
                                                                    let _e1616 = ((_e1606 + 1u) % 256u);
                                                                    let _e1618 = (_e1599 - _e1600);
                                                                    let _e1620 = (_e1594 - f32(_e1614));
                                                                    let _e1622 = (_e1509 + (_e1578 * 294912u));
                                                                    let _e1623 = (_e1614 * 256u);
                                                                    let _e1625 = (9u * (_e1623 + _e1606));
                                                                    let _e1626 = (_e1622 + _e1625);
                                                                    let _e1628 = (9u * (_e1623 + _e1616));
                                                                    let _e1629 = (_e1622 + _e1628);
                                                                    let _e1630 = ((_e1614 + 1u) * 256u);
                                                                    let _e1632 = (9u * (_e1630 + _e1606));
                                                                    let _e1633 = (_e1622 + _e1632);
                                                                    let _e1635 = (9u * (_e1630 + _e1616));
                                                                    let _e1636 = (_e1622 + _e1635);
                                                                    let _e1637 = (1f - _e1618);
                                                                    let _e1638 = (1f - _e1620);
                                                                    let _e1639 = (_e1637 * _e1638);
                                                                    let _e1640 = (_e1618 * _e1638);
                                                                    let _e1641 = (_e1637 * _e1620);
                                                                    let _e1642 = (_e1618 * _e1620);
                                                                    let _e1644 = global_5.member[_e1626];
                                                                    let _e1647 = global_5.member[(_e1626 + 1u)];
                                                                    let _e1650 = global_5.member[(_e1626 + 2u)];
                                                                    let _e1655 = global_5.member[_e1629];
                                                                    let _e1658 = global_5.member[(_e1629 + 1u)];
                                                                    let _e1661 = global_5.member[(_e1629 + 2u)];
                                                                    let _e1669 = global_5.member[_e1633];
                                                                    let _e1672 = global_5.member[(_e1633 + 1u)];
                                                                    let _e1675 = global_5.member[(_e1633 + 2u)];
                                                                    let _e1683 = global_5.member[_e1636];
                                                                    let _e1686 = global_5.member[(_e1636 + 1u)];
                                                                    let _e1689 = global_5.member[(_e1636 + 2u)];
                                                                    let _e1698 = global_5.member[(_e1626 + 3u)];
                                                                    let _e1701 = global_5.member[(_e1626 + 4u)];
                                                                    let _e1704 = global_5.member[(_e1626 + 5u)];
                                                                    let _e1710 = global_5.member[(_e1629 + 3u)];
                                                                    let _e1713 = global_5.member[(_e1629 + 4u)];
                                                                    let _e1716 = global_5.member[(_e1629 + 5u)];
                                                                    let _e1725 = global_5.member[(_e1633 + 3u)];
                                                                    let _e1728 = global_5.member[(_e1633 + 4u)];
                                                                    let _e1731 = global_5.member[(_e1633 + 5u)];
                                                                    let _e1740 = global_5.member[(_e1636 + 3u)];
                                                                    let _e1743 = global_5.member[(_e1636 + 4u)];
                                                                    let _e1746 = global_5.member[(_e1636 + 5u)];
                                                                    let _e1755 = global_5.member[(_e1626 + 6u)];
                                                                    let _e1758 = global_5.member[(_e1626 + 7u)];
                                                                    let _e1761 = global_5.member[(_e1626 + 8u)];
                                                                    let _e1767 = global_5.member[(_e1629 + 6u)];
                                                                    let _e1770 = global_5.member[(_e1629 + 7u)];
                                                                    let _e1773 = global_5.member[(_e1629 + 8u)];
                                                                    let _e1782 = global_5.member[(_e1633 + 6u)];
                                                                    let _e1785 = global_5.member[(_e1633 + 7u)];
                                                                    let _e1788 = global_5.member[(_e1633 + 8u)];
                                                                    let _e1797 = global_5.member[(_e1636 + 6u)];
                                                                    let _e1800 = global_5.member[(_e1636 + 7u)];
                                                                    let _e1803 = global_5.member[(_e1636 + 8u)];
                                                                    let _e1811 = (_e1509 + (_e1577 * 294912u));
                                                                    let _e1812 = (_e1811 + _e1625);
                                                                    let _e1813 = (_e1811 + _e1628);
                                                                    let _e1814 = (_e1811 + _e1632);
                                                                    let _e1815 = (_e1811 + _e1635);
                                                                    let _e1817 = global_5.member[_e1812];
                                                                    let _e1820 = global_5.member[(_e1812 + 1u)];
                                                                    let _e1823 = global_5.member[(_e1812 + 2u)];
                                                                    let _e1828 = global_5.member[_e1813];
                                                                    let _e1831 = global_5.member[(_e1813 + 1u)];
                                                                    let _e1834 = global_5.member[(_e1813 + 2u)];
                                                                    let _e1842 = global_5.member[_e1814];
                                                                    let _e1845 = global_5.member[(_e1814 + 1u)];
                                                                    let _e1848 = global_5.member[(_e1814 + 2u)];
                                                                    let _e1856 = global_5.member[_e1815];
                                                                    let _e1859 = global_5.member[(_e1815 + 1u)];
                                                                    let _e1862 = global_5.member[(_e1815 + 2u)];
                                                                    let _e1871 = global_5.member[(_e1812 + 3u)];
                                                                    let _e1874 = global_5.member[(_e1812 + 4u)];
                                                                    let _e1877 = global_5.member[(_e1812 + 5u)];
                                                                    let _e1883 = global_5.member[(_e1813 + 3u)];
                                                                    let _e1886 = global_5.member[(_e1813 + 4u)];
                                                                    let _e1889 = global_5.member[(_e1813 + 5u)];
                                                                    let _e1898 = global_5.member[(_e1814 + 3u)];
                                                                    let _e1901 = global_5.member[(_e1814 + 4u)];
                                                                    let _e1904 = global_5.member[(_e1814 + 5u)];
                                                                    let _e1913 = global_5.member[(_e1815 + 3u)];
                                                                    let _e1916 = global_5.member[(_e1815 + 4u)];
                                                                    let _e1919 = global_5.member[(_e1815 + 5u)];
                                                                    let _e1928 = global_5.member[(_e1812 + 6u)];
                                                                    let _e1931 = global_5.member[(_e1812 + 7u)];
                                                                    let _e1934 = global_5.member[(_e1812 + 8u)];
                                                                    let _e1940 = global_5.member[(_e1813 + 6u)];
                                                                    let _e1943 = global_5.member[(_e1813 + 7u)];
                                                                    let _e1946 = global_5.member[(_e1813 + 8u)];
                                                                    let _e1955 = global_5.member[(_e1814 + 6u)];
                                                                    let _e1958 = global_5.member[(_e1814 + 7u)];
                                                                    let _e1961 = global_5.member[(_e1814 + 8u)];
                                                                    let _e1970 = global_5.member[(_e1815 + 6u)];
                                                                    let _e1973 = global_5.member[(_e1815 + 7u)];
                                                                    let _e1976 = global_5.member[(_e1815 + 8u)];
                                                                    let _e1983 = (1f - _e1587);
                                                                    phi_2642_ = type_48(1u, type_46(vec3<f32>(((((((_e1644 * _e1639) + (_e1655 * _e1640)) + (_e1669 * _e1641)) + (_e1683 * _e1642)) * _e1983) + (((((_e1817 * _e1639) + (_e1828 * _e1640)) + (_e1842 * _e1641)) + (_e1856 * _e1642)) * _e1587)), ((((((_e1647 * _e1639) + (_e1658 * _e1640)) + (_e1672 * _e1641)) + (_e1686 * _e1642)) * _e1983) + (((((_e1820 * _e1639) + (_e1831 * _e1640)) + (_e1845 * _e1641)) + (_e1859 * _e1642)) * _e1587)), ((((((_e1650 * _e1639) + (_e1661 * _e1640)) + (_e1675 * _e1641)) + (_e1689 * _e1642)) * _e1983) + (((((_e1823 * _e1639) + (_e1834 * _e1640)) + (_e1848 * _e1641)) + (_e1862 * _e1642)) * _e1587))), vec3<f32>(((((((_e1698 * _e1639) + (_e1710 * _e1640)) + (_e1725 * _e1641)) + (_e1740 * _e1642)) * _e1983) + (((((_e1871 * _e1639) + (_e1883 * _e1640)) + (_e1898 * _e1641)) + (_e1913 * _e1642)) * _e1587)), ((((((_e1701 * _e1639) + (_e1713 * _e1640)) + (_e1728 * _e1641)) + (_e1743 * _e1642)) * _e1983) + (((((_e1874 * _e1639) + (_e1886 * _e1640)) + (_e1901 * _e1641)) + (_e1916 * _e1642)) * _e1587)), ((((((_e1704 * _e1639) + (_e1716 * _e1640)) + (_e1731 * _e1641)) + (_e1746 * _e1642)) * _e1983) + (((((_e1877 * _e1639) + (_e1889 * _e1640)) + (_e1904 * _e1641)) + (_e1919 * _e1642)) * _e1587))), vec3<f32>(((((((_e1755 * _e1639) + (_e1767 * _e1640)) + (_e1782 * _e1641)) + (_e1797 * _e1642)) * _e1983) + (((((_e1928 * _e1639) + (_e1940 * _e1640)) + (_e1955 * _e1641)) + (_e1970 * _e1642)) * _e1587)), ((((((_e1758 * _e1639) + (_e1770 * _e1640)) + (_e1785 * _e1641)) + (_e1800 * _e1642)) * _e1983) + (((((_e1931 * _e1639) + (_e1943 * _e1640)) + (_e1958 * _e1641)) + (_e1973 * _e1642)) * _e1587)), ((((((_e1761 * _e1639) + (_e1773 * _e1640)) + (_e1788 * _e1641)) + (_e1803 * _e1642)) * _e1983) + (((((_e1934 * _e1639) + (_e1946 * _e1640)) + (_e1961 * _e1641)) + (_e1976 * _e1642)) * _e1587)))));
                                                                }
                                                                let _e2020 = phi_2642_;
                                                                let _e2023 = (bitcast<i32>(_e2020.member) != 0i);
                                                                if _e2023 {
                                                                    phi_2650_ = _e2020.member_1;
                                                                } else {
                                                                    phi_2650_ = type_46();
                                                                }
                                                                let _e2026 = phi_2650_;
                                                                phi_2652_ = _e2026;
                                                                phi_2653_ = select(true, false, _e2023);
                                                            }
                                                            let _e2029 = phi_2652_;
                                                            let _e2031 = phi_2653_;
                                                            phi_2654_ = _e2029;
                                                            phi_2655_ = _e1515;
                                                            phi_2656_ = _e2031;
                                                        }
                                                        let _e2033 = phi_2654_;
                                                        let _e2035 = phi_2655_;
                                                        let _e2037 = phi_2656_;
                                                        if select(_e2037, true, _e2035) {
                                                            let _e2046 = ((((_e321.member.x - _e1440) * _e321.member_1.x) + ((_e321.member.y - _e1443) * _e321.member_1.y)) + ((_e321.member.z - _e1446) * _e321.member_1.z));
                                                            let _e2058 = ((((_e321.member.x * _e321.member.x) + (_e321.member.y * _e321.member.y)) + (_e321.member.z * _e321.member.z)) - (2f * (((_e321.member.x * _e1440) + (_e321.member.y * _e1443)) + (_e321.member.z * _e1446))));
                                                            let _e2059 = (_e2058 + _e1458);
                                                            let _e2060 = (_e2046 * _e2046);
                                                            let _e2061 = (_e2060 - _e2059);
                                                            if (_e2061 < 0f) {
                                                                phi_21777_ = type_50(0u, type_50().member_1);
                                                                phi_21778_ = type_50();
                                                                phi_21779_ = true;
                                                            } else {
                                                                if (_e2046 < 0f) {
                                                                    phi_21749_ = -(sqrt(_e2061));
                                                                } else {
                                                                    phi_21749_ = sqrt(_e2061);
                                                                }
                                                                let _e2068 = phi_21749_;
                                                                let _e2070 = -((_e2046 + _e2068));
                                                                let _e2071 = (_e2070 == 0f);
                                                                if _e2071 {
                                                                    phi_21772_ = type_50(1u, type_49(0f, 0f));
                                                                    phi_21773_ = type_50();
                                                                } else {
                                                                    let _e2072 = (_e2059 / _e2070);
                                                                    if (_e2070 < _e2072) {
                                                                        phi_21761_ = type_49(_e2070, _e2072);
                                                                    } else {
                                                                        phi_21761_ = type_49(_e2072, _e2070);
                                                                    }
                                                                    let _e2077 = phi_21761_;
                                                                    phi_21772_ = type_50();
                                                                    phi_21773_ = type_50(1u, _e2077);
                                                                }
                                                                let _e2080 = phi_21772_;
                                                                let _e2082 = phi_21773_;
                                                                phi_21777_ = _e2080;
                                                                phi_21778_ = _e2082;
                                                                phi_21779_ = _e2071;
                                                            }
                                                            let _e2087 = phi_21777_;
                                                            let _e2089 = phi_21778_;
                                                            let _e2091 = phi_21779_;
                                                            if _e2091 {
                                                                phi_21783_ = _e2087;
                                                            } else {
                                                                phi_21783_ = _e2089;
                                                            }
                                                            let _e2093 = phi_21783_;
                                                            if (bitcast<i32>(_e2093.member) != 0i) {
                                                                phi_21670_ = type_50(0u, _e2093.member_1);
                                                            } else {
                                                                phi_21670_ = type_50(1u, type_50().member_1);
                                                            }
                                                            let _e2103 = phi_21670_;
                                                            if (bitcast<i32>(_e2103.member) != 0i) {
                                                                phi_21717_ = type_50(0u, type_50().member_1);
                                                            } else {
                                                                if (_e2103.member_1.member_1 <= 0f) {
                                                                    phi_21713_ = type_50(0u, type_50().member_1);
                                                                } else {
                                                                    if (_e2103.member_1.member != _e2103.member_1.member) {
                                                                        phi_21821_ = true;
                                                                    } else {
                                                                        phi_21821_ = (0f >= _e2103.member_1.member);
                                                                    }
                                                                    let _e2115 = phi_21821_;
                                                                    let _e2116 = select(_e2103.member_1.member, 0f, _e2115);
                                                                    let _e2117 = (_e2058 + _e1455);
                                                                    if (_e2117 > 0f) {
                                                                        let _e2119 = (_e2060 - _e2117);
                                                                        if (_e2119 < 0f) {
                                                                            phi_21929_ = type_50(0u, type_50().member_1);
                                                                            phi_21930_ = type_50();
                                                                            phi_21931_ = true;
                                                                        } else {
                                                                            if (_e2046 < 0f) {
                                                                                phi_21901_ = -(sqrt(_e2119));
                                                                            } else {
                                                                                phi_21901_ = sqrt(_e2119);
                                                                            }
                                                                            let _e2126 = phi_21901_;
                                                                            let _e2128 = -((_e2046 + _e2126));
                                                                            let _e2129 = (_e2128 == 0f);
                                                                            if _e2129 {
                                                                                phi_21924_ = type_50(1u, type_49(0f, 0f));
                                                                                phi_21925_ = type_50();
                                                                            } else {
                                                                                let _e2130 = (_e2117 / _e2128);
                                                                                if (_e2128 < _e2130) {
                                                                                    phi_21913_ = type_49(_e2128, _e2130);
                                                                                } else {
                                                                                    phi_21913_ = type_49(_e2130, _e2128);
                                                                                }
                                                                                let _e2135 = phi_21913_;
                                                                                phi_21924_ = type_50();
                                                                                phi_21925_ = type_50(1u, _e2135);
                                                                            }
                                                                            let _e2138 = phi_21924_;
                                                                            let _e2140 = phi_21925_;
                                                                            phi_21929_ = _e2138;
                                                                            phi_21930_ = _e2140;
                                                                            phi_21931_ = _e2129;
                                                                        }
                                                                        let _e2145 = phi_21929_;
                                                                        let _e2147 = phi_21930_;
                                                                        let _e2149 = phi_21931_;
                                                                        if _e2149 {
                                                                            phi_21935_ = _e2145;
                                                                        } else {
                                                                            phi_21935_ = _e2147;
                                                                        }
                                                                        let _e2151 = phi_21935_;
                                                                        if (bitcast<i32>(_e2151.member) != 0i) {
                                                                            phi_21869_ = select(_e2151.member_1.member_1, _e2151.member_1.member, (_e2151.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_21869_ = -1f;
                                                                        }
                                                                        let _e2162 = phi_21869_;
                                                                        if (_e2162 >= 0f) {
                                                                            if (_e2103.member_1.member_1 != _e2103.member_1.member_1) {
                                                                                phi_21973_ = true;
                                                                            } else {
                                                                                phi_21973_ = (_e2162 <= _e2103.member_1.member_1);
                                                                            }
                                                                            let _e2167 = phi_21973_;
                                                                            phi_21698_ = select(_e2103.member_1.member_1, _e2162, _e2167);
                                                                        } else {
                                                                            phi_21698_ = _e2103.member_1.member_1;
                                                                        }
                                                                        let _e2170 = phi_21698_;
                                                                        phi_21700_ = _e2170;
                                                                    } else {
                                                                        phi_21700_ = _e2103.member_1.member_1;
                                                                    }
                                                                    let _e2172 = phi_21700_;
                                                                    if (_e2172 <= _e2116) {
                                                                        phi_21709_ = type_50(0u, type_50().member_1);
                                                                    } else {
                                                                        phi_21709_ = type_50(1u, type_49(_e2116, _e2172));
                                                                    }
                                                                    let _e2180 = phi_21709_;
                                                                    phi_21713_ = _e2180;
                                                                }
                                                                let _e2185 = phi_21713_;
                                                                phi_21717_ = _e2185;
                                                            }
                                                            let _e2190 = phi_21717_;
                                                            if (bitcast<i32>(_e2190.member) != 0i) {
                                                                phi_2674_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_2677_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_2679_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_2681_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_2683_ = 0u;
                                                                loop {
                                                                    let _e2199 = phi_2674_;
                                                                    let _e2201 = phi_2677_;
                                                                    let _e2203 = phi_2679_;
                                                                    let _e2205 = phi_2681_;
                                                                    let _e2207 = phi_2683_;
                                                                    local_12 = _e2205;
                                                                    local_13 = _e2203;
                                                                    local_14 = _e2201;
                                                                    let _e2208 = (_e2207 < 32u);
                                                                    if _e2208 {
                                                                        let _e2209 = -(_e2046);
                                                                        if (_e2190.member_1.member <= _e2190.member_1.member_1) {
                                                                        } else {
                                                                            phi_31785_ = true;
                                                                            break;
                                                                        }
                                                                        let _e2212 = select(_e2209, _e2190.member_1.member, (_e2209 < _e2190.member_1.member));
                                                                        let _e2214 = select(_e2212, _e2190.member_1.member_1, (_e2212 > _e2190.member_1.member_1));
                                                                        let _e2215 = (_e2214 - _e2190.member_1.member);
                                                                        let _e2216 = (_e2190.member_1.member_1 - _e2214);
                                                                        let _e2217 = (_e2190.member_1.member_1 - _e2190.member_1.member);
                                                                        let _e2218 = (0.000000001f * _e2217);
                                                                        if (_e2215 <= _e2218) {
                                                                            let _e2293 = (f32(_e2207) * 0.03125f);
                                                                            let _e2296 = (f32((_e2207 + 1u)) * 0.03125f);
                                                                            let _e2298 = ((_e2293 + _e2296) * 0.5f);
                                                                            phi_2784_ = type_49((_e2190.member_1.member + ((_e2217 * _e2298) * _e2298)), (_e2217 * ((_e2296 * _e2296) - (_e2293 * _e2293))));
                                                                        } else {
                                                                            if (_e2216 <= _e2218) {
                                                                                let _e2276 = (f32((31u - _e2207)) * 0.03125f);
                                                                                let _e2279 = (f32((32u - _e2207)) * 0.03125f);
                                                                                let _e2281 = ((_e2276 + _e2279) * 0.5f);
                                                                                phi_2783_ = type_49((_e2190.member_1.member_1 - ((_e2217 * _e2281) * _e2281)), (_e2217 * ((_e2279 * _e2279) - (_e2276 * _e2276))));
                                                                            } else {
                                                                                let _e2224 = round(((32f * _e2215) / (_e2215 + _e2216)));
                                                                                let _e2229 = select(select(u32(_e2224), 0u, (_e2224 < 0f)), 4294967295u, (_e2224 > 4294967000f));
                                                                                if (_e2229 < 1u) {
                                                                                    phi_2767_ = 1u;
                                                                                } else {
                                                                                    phi_2767_ = select(_e2229, 31u, (_e2229 > 31u));
                                                                                }
                                                                                let _e2234 = phi_2767_;
                                                                                if (_e2207 < _e2234) {
                                                                                    let _e2255 = ((_e2234 - 1u) - _e2207);
                                                                                    let _e2257 = f32(_e2234);
                                                                                    let _e2258 = (f32(_e2255) / _e2257);
                                                                                    let _e2261 = (f32((_e2255 + 1u)) / _e2257);
                                                                                    let _e2263 = ((_e2258 + _e2261) * 0.5f);
                                                                                    phi_2782_ = type_49((_e2214 - ((_e2215 * _e2263) * _e2263)), (_e2215 * ((_e2261 * _e2261) - (_e2258 * _e2258))));
                                                                                } else {
                                                                                    let _e2237 = (_e2207 - _e2234);
                                                                                    let _e2239 = f32((32u - _e2234));
                                                                                    let _e2240 = (f32(_e2237) / _e2239);
                                                                                    let _e2243 = (f32((_e2237 + 1u)) / _e2239);
                                                                                    let _e2245 = ((_e2240 + _e2243) * 0.5f);
                                                                                    phi_2782_ = type_49((_e2214 + ((_e2216 * _e2245) * _e2245)), (_e2216 * ((_e2243 * _e2243) - (_e2240 * _e2240))));
                                                                                }
                                                                                let _e2273 = phi_2782_;
                                                                                phi_2783_ = _e2273;
                                                                            }
                                                                            let _e2291 = phi_2783_;
                                                                            phi_2784_ = _e2291;
                                                                        }
                                                                        let _e2308 = phi_2784_;
                                                                        let _e2314 = (_e321.member.x + (_e321.member_1.x * _e2308.member));
                                                                        let _e2315 = (_e321.member.y + (_e321.member_1.y * _e2308.member));
                                                                        let _e2316 = (_e321.member.z + (_e321.member_1.z * _e2308.member));
                                                                        let _e2329 = ((((_e2314 * _e2314) + (_e2315 * _e2315)) + (_e2316 * _e2316)) - (2f * (((_e2314 * _e1440) + (_e2315 * _e1443)) + (_e2316 * _e1446))));
                                                                        let _e2330 = (_e2329 + _e1455);
                                                                        let _e2331 = (_e2314 - _e1440);
                                                                        let _e2332 = (_e2315 - _e1443);
                                                                        let _e2333 = (_e2316 - _e1446);
                                                                        let _e2339 = sqrt((((_e2331 * _e2331) + (_e2332 * _e2332)) + (_e2333 * _e2333)));
                                                                        let _e2341 = (_e2330 / (_e2339 + _e1449));
                                                                        let _e2345 = (1f - (abs((_e2341 - 25000f)) * 0.00006666667f));
                                                                        if (_e2345 != _e2345) {
                                                                            phi_22172_ = true;
                                                                        } else {
                                                                            phi_22172_ = (0f >= _e2345);
                                                                        }
                                                                        let _e2349 = phi_22172_;
                                                                        let _e2351 = -(_e2341);
                                                                        let _e2354 = (exp((_e2351 / _e1470)) * _e2308.member_1);
                                                                        let _e2357 = (exp((_e2351 / _e1473)) * _e2308.member_1);
                                                                        let _e2359 = (_e2354 * 0.5f);
                                                                        let _e2360 = (_e2357 * 0.5f);
                                                                        let _e2361 = ((select(_e2345, 0f, _e2349) * _e2308.member_1) * 0.5f);
                                                                        let _e2363 = (_e2199.x + _e2359);
                                                                        let _e2365 = (_e2199.y + _e2360);
                                                                        let _e2367 = (_e2199.z + _e2361);
                                                                        let _e2372 = (_e1489 * 1.1f);
                                                                        let _e2373 = (_e2372 * _e2365);
                                                                        let _e2392 = (_e1489 * _e2357);
                                                                        if (_e1505 == 4294967295u) {
                                                                            phi_3112_ = vec3<f32>(0f, 0f, 0f);
                                                                        } else {
                                                                            let _e2400 = (1f / _e2339);
                                                                            let _e2408 = ((((_e2331 * _e2400) * _e1461) + ((_e2332 * _e2400) * _e1464)) + ((_e2333 * _e2400) * _e1467));
                                                                            if (_e2341 != _e2341) {
                                                                                phi_22299_ = true;
                                                                            } else {
                                                                                phi_22299_ = (0f >= _e2341);
                                                                            }
                                                                            let _e2412 = phi_22299_;
                                                                            let _e2416 = sqrt((select(_e2341, 0f, _e2412) / (_e1452 - _e1449)));
                                                                            if (_e2416 != _e2416) {
                                                                                phi_22314_ = true;
                                                                            } else {
                                                                                phi_22314_ = (1f <= _e2416);
                                                                            }
                                                                            let _e2420 = phi_22314_;
                                                                            let _e2422 = (select(_e2416, 1f, _e2420) * 31f);
                                                                            let _e2424 = select(_e2408, -1f, (_e2408 < -1f));
                                                                            let _e2428 = ((select(_e2424, 1f, (_e2424 > 1f)) + 1f) * 15.5f);
                                                                            let _e2433 = select(select(u32(_e2422), 0u, (_e2422 < 0f)), 4294967295u, (_e2422 > 4294967000f));
                                                                            let _e2435 = select(_e2433, 30u, (30u < _e2433));
                                                                            let _e2440 = select(select(u32(_e2428), 0u, (_e2428 < 0f)), 4294967295u, (_e2428 > 4294967000f));
                                                                            let _e2442 = select(_e2440, 30u, (30u < _e2440));
                                                                            let _e2444 = (_e2422 - f32(_e2435));
                                                                            let _e2446 = (_e2428 - f32(_e2442));
                                                                            let _e2450 = (_e1505 + (3u * ((_e2435 * 32u) + _e2442)));
                                                                            let _e2455 = (_e1505 + (3u * (((_e2435 + 1u) * 32u) + _e2442)));
                                                                            let _e2457 = global_5.member[_e2450];
                                                                            let _e2460 = global_5.member[(_e2450 + 1u)];
                                                                            let _e2463 = global_5.member[(_e2450 + 2u)];
                                                                            let _e2464 = (1f - _e2446);
                                                                            let _e2470 = global_5.member[(_e2450 + 3u)];
                                                                            let _e2473 = global_5.member[(_e2450 + 4u)];
                                                                            let _e2476 = global_5.member[(_e2450 + 5u)];
                                                                            let _e2483 = (1f - _e2444);
                                                                            let _e2488 = global_5.member[_e2455];
                                                                            let _e2491 = global_5.member[(_e2455 + 1u)];
                                                                            let _e2494 = global_5.member[(_e2455 + 2u)];
                                                                            let _e2500 = global_5.member[(_e2455 + 3u)];
                                                                            let _e2503 = global_5.member[(_e2455 + 4u)];
                                                                            let _e2506 = global_5.member[(_e2455 + 5u)];
                                                                            phi_3112_ = vec3<f32>(((((_e2457 * _e2464) + (_e2470 * _e2446)) * _e2483) + (((_e2488 * _e2464) + (_e2500 * _e2446)) * _e2444)), ((((_e2460 * _e2464) + (_e2473 * _e2446)) * _e2483) + (((_e2491 * _e2464) + (_e2503 * _e2446)) * _e2444)), ((((_e2463 * _e2464) + (_e2476 * _e2446)) * _e2483) + (((_e2494 * _e2464) + (_e2506 * _e2446)) * _e2444)));
                                                                        }
                                                                        let _e2521 = phi_3112_;
                                                                        let _e2539 = (((_e2331 * _e1461) + (_e2332 * _e1464)) + (_e2333 * _e1467));
                                                                        let _e2540 = (_e2539 * _e2539);
                                                                        let _e2541 = (_e2540 - _e2330);
                                                                        if (_e2541 < 0f) {
                                                                            phi_22464_ = type_50(0u, type_50().member_1);
                                                                            phi_22465_ = type_50();
                                                                            phi_22466_ = true;
                                                                        } else {
                                                                            if (_e2539 < 0f) {
                                                                                phi_22436_ = -(sqrt(_e2541));
                                                                            } else {
                                                                                phi_22436_ = sqrt(_e2541);
                                                                            }
                                                                            let _e2548 = phi_22436_;
                                                                            let _e2550 = -((_e2539 + _e2548));
                                                                            let _e2551 = (_e2550 == 0f);
                                                                            if _e2551 {
                                                                                phi_22459_ = type_50(1u, type_49(0f, 0f));
                                                                                phi_22460_ = type_50();
                                                                            } else {
                                                                                let _e2552 = (_e2330 / _e2550);
                                                                                if (_e2550 < _e2552) {
                                                                                    phi_22448_ = type_49(_e2550, _e2552);
                                                                                } else {
                                                                                    phi_22448_ = type_49(_e2552, _e2550);
                                                                                }
                                                                                let _e2557 = phi_22448_;
                                                                                phi_22459_ = type_50();
                                                                                phi_22460_ = type_50(1u, _e2557);
                                                                            }
                                                                            let _e2560 = phi_22459_;
                                                                            let _e2562 = phi_22460_;
                                                                            phi_22464_ = _e2560;
                                                                            phi_22465_ = _e2562;
                                                                            phi_22466_ = _e2551;
                                                                        }
                                                                        let _e2567 = phi_22464_;
                                                                        let _e2569 = phi_22465_;
                                                                        let _e2571 = phi_22466_;
                                                                        if _e2571 {
                                                                            phi_22470_ = _e2567;
                                                                        } else {
                                                                            phi_22470_ = _e2569;
                                                                        }
                                                                        let _e2573 = phi_22470_;
                                                                        if (bitcast<i32>(_e2573.member) != 0i) {
                                                                            phi_22404_ = select(_e2573.member_1.member_1, _e2573.member_1.member, (_e2573.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_22404_ = -1f;
                                                                        }
                                                                        let _e2584 = phi_22404_;
                                                                        if (_e2584 >= 0f) {
                                                                            phi_22387_ = type_51(0u, type_51().member_1);
                                                                        } else {
                                                                            let _e2586 = (_e2329 + _e1458);
                                                                            let _e2587 = (_e2540 - _e2586);
                                                                            if (_e2587 < 0f) {
                                                                                phi_22574_ = type_50(0u, type_50().member_1);
                                                                                phi_22575_ = type_50();
                                                                                phi_22576_ = true;
                                                                            } else {
                                                                                if (_e2539 < 0f) {
                                                                                    phi_22546_ = -(sqrt(_e2587));
                                                                                } else {
                                                                                    phi_22546_ = sqrt(_e2587);
                                                                                }
                                                                                let _e2594 = phi_22546_;
                                                                                let _e2596 = -((_e2539 + _e2594));
                                                                                let _e2597 = (_e2596 == 0f);
                                                                                if _e2597 {
                                                                                    phi_22569_ = type_50(1u, type_49(0f, 0f));
                                                                                    phi_22570_ = type_50();
                                                                                } else {
                                                                                    let _e2598 = (_e2586 / _e2596);
                                                                                    if (_e2596 < _e2598) {
                                                                                        phi_22558_ = type_49(_e2596, _e2598);
                                                                                    } else {
                                                                                        phi_22558_ = type_49(_e2598, _e2596);
                                                                                    }
                                                                                    let _e2603 = phi_22558_;
                                                                                    phi_22569_ = type_50();
                                                                                    phi_22570_ = type_50(1u, _e2603);
                                                                                }
                                                                                let _e2606 = phi_22569_;
                                                                                let _e2608 = phi_22570_;
                                                                                phi_22574_ = _e2606;
                                                                                phi_22575_ = _e2608;
                                                                                phi_22576_ = _e2597;
                                                                            }
                                                                            let _e2613 = phi_22574_;
                                                                            let _e2615 = phi_22575_;
                                                                            let _e2617 = phi_22576_;
                                                                            if _e2617 {
                                                                                phi_22580_ = _e2613;
                                                                            } else {
                                                                                phi_22580_ = _e2615;
                                                                            }
                                                                            let _e2619 = phi_22580_;
                                                                            if (bitcast<i32>(_e2619.member) != 0i) {
                                                                                phi_22514_ = select(_e2619.member_1.member_1, _e2619.member_1.member, (_e2619.member_1.member >= 0f));
                                                                            } else {
                                                                                phi_22514_ = -1f;
                                                                            }
                                                                            let _e2630 = phi_22514_;
                                                                            if (_e2630 < 0f) {
                                                                                phi_22383_ = type_51(0u, type_51().member_1);
                                                                            } else {
                                                                                phi_22341_ = vec3<f32>(0f, 0f, 0f);
                                                                                phi_22342_ = 0u;
                                                                                loop {
                                                                                    let _e2633 = phi_22341_;
                                                                                    let _e2635 = phi_22342_;
                                                                                    local_11 = _e2633;
                                                                                    let _e2636 = (_e2635 < 16u);
                                                                                    if _e2636 {
                                                                                        let _e2638 = (f32(_e2635) * 0.0625f);
                                                                                        let _e2639 = (_e2635 + 1u);
                                                                                        let _e2641 = (f32(_e2639) * 0.0625f);
                                                                                        let _e2643 = ((_e2638 + _e2641) * 0.5f);
                                                                                        let _e2645 = ((_e2630 * _e2643) * _e2643);
                                                                                        let _e2649 = (_e2630 * ((_e2641 * _e2641) - (_e2638 * _e2638)));
                                                                                        let _e2654 = (vec3<f32>(_e2314, _e2315, _e2316) + vec3<f32>((_e1461 * _e2645), (_e1464 * _e2645), (_e1467 * _e2645)));
                                                                                        let _e2671 = (_e2654.x - _e1440);
                                                                                        let _e2672 = (_e2654.y - _e1443);
                                                                                        let _e2673 = (_e2654.z - _e1446);
                                                                                        let _e2681 = ((((((_e2654.x * _e2654.x) + (_e2654.y * _e2654.y)) + (_e2654.z * _e2654.z)) - (2f * (((_e2654.x * _e1440) + (_e2654.y * _e1443)) + (_e2654.z * _e1446)))) + _e1455) / (sqrt((((_e2671 * _e2671) + (_e2672 * _e2672)) + (_e2673 * _e2673))) + _e1449));
                                                                                        let _e2685 = (1f - (abs((_e2681 - 25000f)) * 0.00006666667f));
                                                                                        if (_e2685 != _e2685) {
                                                                                            phi_22712_ = true;
                                                                                        } else {
                                                                                            phi_22712_ = (0f >= _e2685);
                                                                                        }
                                                                                        let _e2689 = phi_22712_;
                                                                                        let _e2691 = -(_e2681);
                                                                                        phi_22375_ = (_e2633 + vec3<f32>((exp((_e2691 / _e1470)) * _e2649), (exp((_e2691 / _e1473)) * _e2649), (select(_e2685, 0f, _e2689) * _e2649)));
                                                                                        phi_22376_ = _e2639;
                                                                                    } else {
                                                                                        phi_22375_ = vec3<f32>();
                                                                                        phi_22376_ = u32();
                                                                                    }
                                                                                    let _e2702 = phi_22375_;
                                                                                    let _e2704 = phi_22376_;
                                                                                    phi_22341_ = _e2702;
                                                                                    phi_22342_ = _e2704;
                                                                                    if !(_e2636) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2707 = local_11;
                                                                                phi_22383_ = type_51(1u, _e2707);
                                                                            }
                                                                            let _e2713 = phi_22383_;
                                                                            phi_22387_ = _e2713;
                                                                        }
                                                                        let _e2718 = phi_22387_;
                                                                        if (bitcast<i32>(_e2718.member) != 0i) {
                                                                            let _e2723 = (vec3<f32>(_e2363, _e2365, _e2367) + _e2718.member_1);
                                                                            let _e2729 = (_e2372 * _e2723.y);
                                                                            let _e2741 = exp(-((((_e1479 * _e2723.x) + _e2729) + (_e1495 * _e2723.z))));
                                                                            let _e2743 = exp(-((((_e1482 * _e2723.x) + _e2729) + (_e1498 * _e2723.z))));
                                                                            let _e2745 = exp(-((((_e1485 * _e2723.x) + _e2729) + (_e1501 * _e2723.z))));
                                                                            phi_3171_ = vec3<f32>((_e2203.x + (_e2741 * _e2357)), (_e2203.y + (_e2743 * _e2357)), (_e2203.z + (_e2745 * _e2357)));
                                                                            phi_3172_ = vec3<f32>((_e2205.x + (_e2741 * _e2354)), (_e2205.y + (_e2743 * _e2354)), (_e2205.z + (_e2745 * _e2354)));
                                                                        } else {
                                                                            phi_3171_ = _e2203;
                                                                            phi_3172_ = _e2205;
                                                                        }
                                                                        let _e2767 = phi_3171_;
                                                                        let _e2769 = phi_3172_;
                                                                        phi_2675_ = vec3<f32>((_e2363 + _e2359), (_e2365 + _e2360), (_e2367 + _e2361));
                                                                        phi_2678_ = vec3<f32>((_e2201.x + ((exp(-((((_e1479 * _e2363) + _e2373) + (_e1495 * _e2367)))) * ((_e1479 * _e2354) + _e2392)) * _e2521.x)), (_e2201.y + ((exp(-((((_e1482 * _e2363) + _e2373) + (_e1498 * _e2367)))) * ((_e1482 * _e2354) + _e2392)) * _e2521.y)), (_e2201.z + ((exp(-((((_e1485 * _e2363) + _e2373) + (_e1501 * _e2367)))) * ((_e1485 * _e2354) + _e2392)) * _e2521.z)));
                                                                        phi_2680_ = _e2767;
                                                                        phi_2682_ = _e2769;
                                                                        phi_2684_ = (_e2207 + 1u);
                                                                    } else {
                                                                        phi_2675_ = vec3<f32>();
                                                                        phi_2678_ = vec3<f32>();
                                                                        phi_2680_ = vec3<f32>();
                                                                        phi_2682_ = vec3<f32>();
                                                                        phi_2684_ = u32();
                                                                    }
                                                                    let _e2776 = phi_2675_;
                                                                    let _e2778 = phi_2678_;
                                                                    let _e2780 = phi_2680_;
                                                                    let _e2782 = phi_2682_;
                                                                    let _e2784 = phi_2684_;
                                                                    phi_2674_ = _e2776;
                                                                    phi_2677_ = _e2778;
                                                                    phi_2679_ = _e2780;
                                                                    phi_2681_ = _e2782;
                                                                    phi_2683_ = _e2784;
                                                                    phi_31785_ = _e1430;
                                                                    if !(_e2208) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e2787 = phi_31785_;
                                                                phi_31875_ = _e2787;
                                                                if _e2787 {
                                                                    break;
                                                                }
                                                                let _e2789 = local_12;
                                                                let _e2792 = local_13;
                                                                let _e2795 = local_14;
                                                                phi_31879_ = _e2787;
                                                                phi_3192_ = type_46((_e2789 * vec3<f32>(_e1479, _e1482, _e1485)), (_e2792 * _e1489), _e2795);
                                                            } else {
                                                                phi_31879_ = _e1430;
                                                                phi_3192_ = type_46(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                            }
                                                            let _e2798 = phi_31879_;
                                                            let _e2800 = phi_3192_;
                                                            phi_31878_ = _e2798;
                                                            phi_3193_ = _e2800;
                                                        } else {
                                                            phi_31878_ = _e1430;
                                                            phi_3193_ = _e2033;
                                                        }
                                                        let _e2802 = phi_31878_;
                                                        let _e2804 = phi_3193_;
                                                        let _e2812 = (((_e321.member_1.x * _e1461) + (_e321.member_1.y * _e1464)) + (_e321.member_1.z * _e1467));
                                                        let _e2814 = (1f + (_e2812 * _e2812));
                                                        let _e2815 = (0.059683103f * _e2814);
                                                        let _e2822 = (_e1492 * _e1492);
                                                        let _e2833 = ((0.119366206f * ((1f - _e2822) * _e2814)) / ((2f + _e2822) * pow(((1f + _e2822) - ((2f * _e1492) * _e2812)), 1.5f)));
                                                        phi_31877_ = _e2802;
                                                        phi_3216_ = vec3<f32>((_e1432.x + (((((_e2804.member.x * _e2815) + (_e2804.member_1.x * _e2833)) + _e2804.member_2.x) * _e1476) * 0.38317f)), (_e1432.y + (((((_e2804.member.y * _e2815) + (_e2804.member_1.y * _e2833)) + _e2804.member_2.y) * _e1476) * 0.38317f)), (_e1432.z + (((((_e2804.member.z * _e2815) + (_e2804.member_1.z * _e2833)) + _e2804.member_2.z) * _e1476) * 0.38317f)));
                                                        phi_3217_ = (_e1434 + 1i);
                                                        phi_3218_ = bitcast<u32>(_e1512);
                                                    } else {
                                                        phi_31877_ = _e1430;
                                                        phi_3216_ = vec3<f32>();
                                                        phi_3217_ = i32();
                                                        phi_3218_ = u32();
                                                    }
                                                    let _e2864 = phi_31877_;
                                                    let _e2866 = phi_3216_;
                                                    let _e2868 = phi_3217_;
                                                    let _e2870 = phi_3218_;
                                                    phi_31876_ = _e2864;
                                                    phi_862_ = _e2866;
                                                    phi_865_ = _e2868;
                                                    phi_867_ = _e2870;
                                                    phi_3219_ = _e1438;
                                                } else {
                                                    phi_31876_ = _e1430;
                                                    phi_862_ = vec3<f32>();
                                                    phi_865_ = i32();
                                                    phi_867_ = u32();
                                                    phi_3219_ = false;
                                                }
                                                let _e2872 = phi_31876_;
                                                let _e2874 = phi_862_;
                                                let _e2876 = phi_865_;
                                                let _e2878 = phi_867_;
                                                let _e2880 = phi_3219_;
                                                phi_31817_ = _e2872;
                                                phi_861_ = _e2874;
                                                phi_864_ = _e2876;
                                                phi_866_ = _e2878;
                                                phi_31875_ = _e2872;
                                                if !(_e2880) {
                                                    break;
                                                }
                                            }
                                            let _e2883 = phi_31875_;
                                            phi_32203_ = _e2883;
                                            if _e2883 {
                                                break;
                                            }
                                            if _e1427 {
                                                let _e2886 = global_5.member[(_e1279 + 10u)];
                                                let _e2889 = global_5.member[(_e1279 + 21u)];
                                                let _e2890 = bitcast<u32>(_e2889);
                                                let _e2893 = global_5.member[(_e1279 + 22u)];
                                                let _e2894 = bitcast<u32>(_e2893);
                                                let _e2897 = global_5.member[(_e1279 + 23u)];
                                                let _e2901 = global_5.member[(_e1279 + 24u)];
                                                let _e2905 = global_5.member[(_e1279 + 25u)];
                                                let _e2909 = global_5.member[(_e1279 + 26u)];
                                                let _e2912 = global_5.member[_e1279];
                                                let _e2915 = global_5.member[(_e1279 + 1u)];
                                                let _e2918 = global_5.member[(_e1279 + 2u)];
                                                let _e2921 = global_5.member[(_e1279 + 3u)];
                                                let _e2924 = global_5.member[(_e1279 + 4u)];
                                                let _e2927 = global_5.member[(_e1279 + 5u)];
                                                let _e2930 = global_5.member[(_e1279 + 6u)];
                                                let _e2933 = global_5.member[(_e1279 + 7u)];
                                                let _e2936 = global_5.member[(_e1279 + 8u)];
                                                let _e2941 = (((_e321.member_1.x * _e2921) + (_e321.member_1.y * _e2924)) + (_e321.member_1.z * _e2927));
                                                let _e2943 = select(_e2941, -1f, (_e2941 < -1f));
                                                let _e2946 = asin(select(_e2943, 1f, (_e2943 > 1f)));
                                                let _e2957 = atan2((((_e321.member_1.x * _e2930) + (_e321.member_1.y * _e2933)) + (_e321.member_1.z * _e2936)), (((_e321.member_1.x * _e2912) + (_e321.member_1.y * _e2915)) + (_e321.member_1.z * _e2918)));
                                                if (_e2890 != 4294967295u) {
                                                    let _e2960 = global_5.member[_e2890];
                                                    let _e2963 = global_5.member[(_e2890 + 1u)];
                                                    let _e2964 = bitcast<u32>(_e2963);
                                                    let _e2967 = global_5.member[(_e2890 + 4u)];
                                                    phi_3553_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_3556_ = 0u;
                                                    loop {
                                                        let _e2973 = phi_3553_;
                                                        let _e2975 = phi_3556_;
                                                        local_15 = _e2973;
                                                        let _e2976 = (_e2975 < 6u);
                                                        if _e2976 {
                                                            let _e2977 = (_e2975 / 2u);
                                                            if (_e2977 == 0u) {
                                                                phi_22840_ = _e321.member_1.x;
                                                            } else {
                                                                phi_22840_ = select(_e321.member_1.z, _e321.member_1.y, (_e2977 == 1u));
                                                            }
                                                            let _e2985 = phi_22840_;
                                                            let _e2986 = (_e2985 * select(-1f, 1f, ((_e2975 % 2u) == 0u)));
                                                            if (_e2986 > 0f) {
                                                                let _e2990 = ((_e2977 + 1u) % 3u);
                                                                if (_e2990 == 0u) {
                                                                    phi_22855_ = _e321.member_1.x;
                                                                } else {
                                                                    phi_22855_ = select(_e321.member_1.z, _e321.member_1.y, (_e2990 == 1u));
                                                                }
                                                                let _e2995 = phi_22855_;
                                                                let _e2996 = (_e2995 / _e2986);
                                                                let _e2998 = ((_e2977 + 2u) % 3u);
                                                                if (_e2998 == 0u) {
                                                                    phi_22870_ = _e321.member_1.x;
                                                                } else {
                                                                    phi_22870_ = select(_e321.member_1.z, _e321.member_1.y, (_e2998 == 1u));
                                                                }
                                                                let _e3003 = phi_22870_;
                                                                let _e3004 = (_e3003 / _e2986);
                                                                let _e3006 = (1f + (_e2960 * 12f));
                                                                if (abs(_e2996) <= _e3006) {
                                                                    if (abs(_e3004) <= _e3006) {
                                                                        let _e3012 = f32(_e2964);
                                                                        let _e3014 = floor((((_e2996 + 1f) * 0.5f) * _e3012));
                                                                        let _e3021 = select(0i, select(select(i32(_e3014), i32(-2147483648), (_e3014 < -2147483600f)), 2147483647i, (_e3014 > 2147483500f)), (_e3014 == _e3014));
                                                                        let _e3023 = (bitcast<i32>(_e2964) - 1i);
                                                                        let _e3024 = (0i <= _e3023);
                                                                        if _e3024 {
                                                                        } else {
                                                                            phi_31880_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3021 < 0i) {
                                                                            phi_3627_ = 0i;
                                                                        } else {
                                                                            phi_3627_ = select(_e3021, _e3023, (_e3021 > _e3023));
                                                                        }
                                                                        let _e3029 = phi_3627_;
                                                                        let _e3033 = floor((((_e3004 + 1f) * 0.5f) * _e3012));
                                                                        let _e3040 = select(0i, select(select(i32(_e3033), i32(-2147483648), (_e3033 < -2147483600f)), 2147483647i, (_e3033 > 2147483500f)), (_e3033 == _e3033));
                                                                        if _e3024 {
                                                                        } else {
                                                                            phi_31880_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3040 < 0i) {
                                                                            phi_3657_ = 0i;
                                                                        } else {
                                                                            phi_3657_ = select(_e3040, _e3023, (_e3040 > _e3023));
                                                                        }
                                                                        let _e3045 = phi_3657_;
                                                                        let _e3046 = (_e3029 - 1i);
                                                                        phi_3665_ = _e2973;
                                                                        phi_3668_ = select(0i, _e3046, (0i < _e3046));
                                                                        loop {
                                                                            let _e3050 = phi_3665_;
                                                                            let _e3052 = phi_3668_;
                                                                            let _e3053 = (_e3029 + 1i);
                                                                            local_59 = _e3050;
                                                                            let _e3056 = (_e3052 <= select(_e3053, _e3023, (_e3023 < _e3053)));
                                                                            if _e3056 {
                                                                                let _e3057 = (_e3045 - 1i);
                                                                                phi_3691_ = _e3050;
                                                                                phi_3694_ = select(0i, _e3057, (0i < _e3057));
                                                                                loop {
                                                                                    let _e3061 = phi_3691_;
                                                                                    let _e3063 = phi_3694_;
                                                                                    let _e3064 = (_e3045 + 1i);
                                                                                    local_58 = _e3061;
                                                                                    let _e3067 = (_e3063 <= select(_e3064, _e3023, (_e3023 < _e3064)));
                                                                                    if _e3067 {
                                                                                        let _e3076 = global_5.member[(bitcast<u32>(_e2967) + ((((_e2975 * _e2964) + bitcast<u32>(_e3052)) * _e2964) + bitcast<u32>(_e3063)))];
                                                                                        phi_3692_ = vec3<f32>((_e3061.x + _e3076), vec3<f32>().y, vec3<f32>().z);
                                                                                        phi_3695_ = (_e3063 + 1i);
                                                                                    } else {
                                                                                        phi_3692_ = vec3<f32>();
                                                                                        phi_3695_ = i32();
                                                                                    }
                                                                                    let _e3085 = phi_3692_;
                                                                                    let _e3087 = phi_3695_;
                                                                                    phi_3691_ = _e3085;
                                                                                    phi_3694_ = _e3087;
                                                                                    if !(_e3067) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e6916 = local_58;
                                                                                phi_3666_ = _e6916;
                                                                                phi_3669_ = (_e3052 + 1i);
                                                                            } else {
                                                                                phi_3666_ = vec3<f32>();
                                                                                phi_3669_ = i32();
                                                                            }
                                                                            let _e3091 = phi_3666_;
                                                                            let _e3093 = phi_3669_;
                                                                            phi_3665_ = _e3091;
                                                                            phi_3668_ = _e3093;
                                                                            if !(_e3056) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e6920 = local_59;
                                                                        phi_3893_ = _e6920;
                                                                    } else {
                                                                        phi_3893_ = _e2973;
                                                                    }
                                                                    let _e3096 = phi_3893_;
                                                                    phi_3894_ = _e3096;
                                                                } else {
                                                                    phi_3894_ = _e2973;
                                                                }
                                                                let _e3098 = phi_3894_;
                                                                phi_3895_ = _e3098;
                                                            } else {
                                                                phi_3895_ = _e2973;
                                                            }
                                                            let _e3100 = phi_3895_;
                                                            phi_3554_ = _e3100;
                                                            phi_3557_ = (_e2975 + 1u);
                                                        } else {
                                                            phi_3554_ = vec3<f32>();
                                                            phi_3557_ = u32();
                                                        }
                                                        let _e3103 = phi_3554_;
                                                        let _e3105 = phi_3557_;
                                                        phi_3553_ = _e3103;
                                                        phi_3556_ = _e3105;
                                                        phi_31880_ = _e2883;
                                                        if !(_e2976) {
                                                            break;
                                                        }
                                                    }
                                                    let _e3108 = phi_31880_;
                                                    phi_32203_ = _e3108;
                                                    if _e3108 {
                                                        break;
                                                    }
                                                    let _e3110 = local_15;
                                                    phi_31928_ = _e3108;
                                                    phi_3906_ = (_e3110.x * (1f / ((6.2831855f * _e2960) * _e2960)));
                                                } else {
                                                    phi_31928_ = _e2883;
                                                    phi_3906_ = 0f;
                                                }
                                                let _e3114 = phi_31928_;
                                                let _e3116 = phi_3906_;
                                                if (_e2894 != 4294967295u) {
                                                    let _e3133 = (_e2946 * 1.3333334f);
                                                    let _e3140 = global_5.member[_e2894];
                                                    phi_3930_ = (_e3116 + ((_e3140 * ((exp((-(abs(_e2946)) / (0.06f + (abs(_e2957) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2957 * -0.71428573f) * (_e2957 * 0.71428573f)))))) + (1.3f * exp((((_e2957 * _e2957) + (_e3133 * _e3133)) * -12.499999f))))) * 4.73f));
                                                } else {
                                                    phi_3930_ = _e3116;
                                                }
                                                let _e3146 = phi_3930_;
                                                phi_3932_ = 0u;
                                                phi_3935_ = _e3146;
                                                loop {
                                                    let _e3148 = phi_3932_;
                                                    let _e3150 = phi_3935_;
                                                    local_60 = _e3150;
                                                    let _e3151 = (_e3148 < bitcast<u32>(_e2901));
                                                    if _e3151 {
                                                        let _e3153 = (bitcast<u32>(_e2897) + (_e3148 * 18u));
                                                        let _e3156 = global_5.member[(_e3153 + 1u)];
                                                        let _e3159 = global_5.member[(_e3153 + 2u)];
                                                        let _e3162 = global_5.member[(_e3153 + 3u)];
                                                        let _e3170 = global_5.member[(_e3153 + 5u)];
                                                        if ((((_e321.member_1.x * _e3156) + (_e321.member_1.y * _e3159)) + (_e321.member_1.z * _e3162)) >= _e3170) {
                                                            let _e3173 = global_5.member[_e3153];
                                                            let _e3178 = select(select(u32(_e3173), 0u, (_e3173 < 0f)), 4294967295u, (_e3173 > 4294967000f));
                                                            if (_e3178 == 0u) {
                                                                phi_4039_ = 1.8f;
                                                            } else {
                                                                if (_e3178 == 1u) {
                                                                    phi_4038_ = 1f;
                                                                } else {
                                                                    phi_4038_ = select(0f, 4.7f, (_e3178 == 2u));
                                                                }
                                                                let _e3184 = phi_4038_;
                                                                phi_4039_ = _e3184;
                                                            }
                                                            let _e3186 = phi_4039_;
                                                            let _e3189 = global_5.member[(_e3153 + 6u)];
                                                            phi_4055_ = (_e3150 + (_e3189 * _e3186));
                                                        } else {
                                                            phi_4055_ = _e3150;
                                                        }
                                                        let _e3193 = phi_4055_;
                                                        phi_3933_ = (_e3148 + 1u);
                                                        phi_3936_ = _e3193;
                                                    } else {
                                                        phi_3933_ = u32();
                                                        phi_3936_ = f32();
                                                    }
                                                    let _e3196 = phi_3933_;
                                                    let _e3198 = phi_3936_;
                                                    phi_3932_ = _e3196;
                                                    phi_3935_ = _e3198;
                                                    if !(_e3151) {
                                                        break;
                                                    }
                                                }
                                                phi_4058_ = 0u;
                                                let _e6940 = local_60;
                                                phi_4061_ = _e6940;
                                                loop {
                                                    let _e3201 = phi_4058_;
                                                    let _e3203 = phi_4061_;
                                                    local_16 = _e3203;
                                                    let _e3204 = (_e3201 < bitcast<u32>(_e2909));
                                                    if _e3204 {
                                                        let _e3206 = (bitcast<u32>(_e2905) + (_e3201 * 29u));
                                                        let _e3209 = global_5.member[(_e3206 + 1u)];
                                                        let _e3212 = global_5.member[(_e3206 + 2u)];
                                                        let _e3215 = global_5.member[(_e3206 + 3u)];
                                                        let _e3223 = global_5.member[(_e3206 + 5u)];
                                                        if ((((_e321.member_1.x * _e3209) + (_e321.member_1.y * _e3212)) + (_e321.member_1.z * _e3215)) >= _e3223) {
                                                            let _e3226 = global_5.member[_e3206];
                                                            let _e3230 = global_5.member[(_e3206 + 6u)];
                                                            phi_4169_ = (_e3203 + (_e3230 * select(1f, 3.3808f, (_e3226 < 0.5f))));
                                                        } else {
                                                            phi_4169_ = _e3203;
                                                        }
                                                        let _e3235 = phi_4169_;
                                                        phi_4059_ = (_e3201 + 1u);
                                                        phi_4062_ = _e3235;
                                                    } else {
                                                        phi_4059_ = u32();
                                                        phi_4062_ = f32();
                                                    }
                                                    let _e3238 = phi_4059_;
                                                    let _e3240 = phi_4062_;
                                                    phi_4058_ = _e3238;
                                                    phi_4061_ = _e3240;
                                                    if !(_e3204) {
                                                        break;
                                                    }
                                                }
                                                let _e3244 = global_5.member[(_e1279 + 9u)];
                                                let _e3246 = local_16;
                                                let _e3249 = local_17;
                                                let _e3253 = local_18;
                                                let _e3258 = local_19;
                                                let _e3263 = ((_e3246 * _e3244) <= (_e2886 * (((0.2126f * _e3249.x) + (0.7152f * _e3253.y)) + (0.0722f * _e3258.z))));
                                                if _e3263 {
                                                    phi_31969_ = _e3114;
                                                    phi_4728_ = vec3<f32>();
                                                    phi_4729_ = bool();
                                                } else {
                                                    let _e3265 = global_5.member[_e1276];
                                                    let _e3268 = global_5.member[(_e1276 + 1u)];
                                                    let _e3271 = global_5.member[(_e1276 + 2u)];
                                                    let _e3274 = global_5.member[(_e1276 + 3u)];
                                                    let _e3277 = global_5.member[(_e1276 + 5u)];
                                                    let _e3280 = global_5.member[(_e1276 + 6u)];
                                                    let _e3283 = global_5.member[(_e1276 + 10u)];
                                                    let _e3286 = global_5.member[(_e1276 + 11u)];
                                                    let _e3289 = global_5.member[(_e1276 + 13u)];
                                                    let _e3292 = global_5.member[(_e1276 + 14u)];
                                                    let _e3295 = global_5.member[(_e1276 + 15u)];
                                                    let _e3298 = global_5.member[(_e1276 + 16u)];
                                                    let _e3301 = global_5.member[(_e1276 + 18u)];
                                                    let _e3304 = global_5.member[(_e1276 + 19u)];
                                                    let _e3307 = global_5.member[(_e1276 + 20u)];
                                                    let _e3319 = ((((_e321.member.x * _e321.member.x) + (_e321.member.y * _e321.member.y)) + (_e321.member.z * _e321.member.z)) - (2f * (((_e321.member.x * _e3265) + (_e321.member.y * _e3268)) + (_e321.member.z * _e3271))));
                                                    let _e3320 = (_e3319 + _e3277);
                                                    let _e3321 = (_e3320 > 0f);
                                                    if _e3321 {
                                                        let _e3329 = ((((_e321.member.x - _e3265) * _e321.member_1.x) + ((_e321.member.y - _e3268) * _e321.member_1.y)) + ((_e321.member.z - _e3271) * _e321.member_1.z));
                                                        let _e3331 = ((_e3329 * _e3329) - _e3320);
                                                        if (_e3331 < 0f) {
                                                            phi_23014_ = type_50(0u, type_50().member_1);
                                                            phi_23015_ = type_50();
                                                            phi_23016_ = true;
                                                        } else {
                                                            if (_e3329 < 0f) {
                                                                phi_22986_ = -(sqrt(_e3331));
                                                            } else {
                                                                phi_22986_ = sqrt(_e3331);
                                                            }
                                                            let _e3338 = phi_22986_;
                                                            let _e3340 = -((_e3329 + _e3338));
                                                            let _e3341 = (_e3340 == 0f);
                                                            if _e3341 {
                                                                phi_23009_ = type_50(1u, type_49(0f, 0f));
                                                                phi_23010_ = type_50();
                                                            } else {
                                                                let _e3342 = (_e3320 / _e3340);
                                                                if (_e3340 < _e3342) {
                                                                    phi_22998_ = type_49(_e3340, _e3342);
                                                                } else {
                                                                    phi_22998_ = type_49(_e3342, _e3340);
                                                                }
                                                                let _e3347 = phi_22998_;
                                                                phi_23009_ = type_50();
                                                                phi_23010_ = type_50(1u, _e3347);
                                                            }
                                                            let _e3350 = phi_23009_;
                                                            let _e3352 = phi_23010_;
                                                            phi_23014_ = _e3350;
                                                            phi_23015_ = _e3352;
                                                            phi_23016_ = _e3341;
                                                        }
                                                        let _e3357 = phi_23014_;
                                                        let _e3359 = phi_23015_;
                                                        let _e3361 = phi_23016_;
                                                        if _e3361 {
                                                            phi_23020_ = _e3357;
                                                        } else {
                                                            phi_23020_ = _e3359;
                                                        }
                                                        let _e3363 = phi_23020_;
                                                        if (bitcast<i32>(_e3363.member) != 0i) {
                                                            phi_22954_ = select(_e3363.member_1.member_1, _e3363.member_1.member, (_e3363.member_1.member >= 0f));
                                                        } else {
                                                            phi_22954_ = -1f;
                                                        }
                                                        let _e3374 = phi_22954_;
                                                        phi_4570_ = select(true, false, (_e3374 >= 0f));
                                                    } else {
                                                        phi_4570_ = true;
                                                    }
                                                    let _e3378 = phi_4570_;
                                                    if _e3378 {
                                                        let _e3386 = ((((_e321.member.x - _e3265) * _e321.member_1.x) + ((_e321.member.y - _e3268) * _e321.member_1.y)) + ((_e321.member.z - _e3271) * _e321.member_1.z));
                                                        let _e3387 = (_e3319 + _e3280);
                                                        let _e3388 = (_e3386 * _e3386);
                                                        let _e3389 = (_e3388 - _e3387);
                                                        if (_e3389 < 0f) {
                                                            phi_23174_ = type_50(0u, type_50().member_1);
                                                            phi_23175_ = type_50();
                                                            phi_23176_ = true;
                                                        } else {
                                                            if (_e3386 < 0f) {
                                                                phi_23146_ = -(sqrt(_e3389));
                                                            } else {
                                                                phi_23146_ = sqrt(_e3389);
                                                            }
                                                            let _e3396 = phi_23146_;
                                                            let _e3398 = -((_e3386 + _e3396));
                                                            let _e3399 = (_e3398 == 0f);
                                                            if _e3399 {
                                                                phi_23169_ = type_50(1u, type_49(0f, 0f));
                                                                phi_23170_ = type_50();
                                                            } else {
                                                                let _e3400 = (_e3387 / _e3398);
                                                                if (_e3398 < _e3400) {
                                                                    phi_23158_ = type_49(_e3398, _e3400);
                                                                } else {
                                                                    phi_23158_ = type_49(_e3400, _e3398);
                                                                }
                                                                let _e3405 = phi_23158_;
                                                                phi_23169_ = type_50();
                                                                phi_23170_ = type_50(1u, _e3405);
                                                            }
                                                            let _e3408 = phi_23169_;
                                                            let _e3410 = phi_23170_;
                                                            phi_23174_ = _e3408;
                                                            phi_23175_ = _e3410;
                                                            phi_23176_ = _e3399;
                                                        }
                                                        let _e3415 = phi_23174_;
                                                        let _e3417 = phi_23175_;
                                                        let _e3419 = phi_23176_;
                                                        if _e3419 {
                                                            phi_23180_ = _e3415;
                                                        } else {
                                                            phi_23180_ = _e3417;
                                                        }
                                                        let _e3421 = phi_23180_;
                                                        if (bitcast<i32>(_e3421.member) != 0i) {
                                                            phi_23067_ = type_50(0u, _e3421.member_1);
                                                        } else {
                                                            phi_23067_ = type_50(1u, type_50().member_1);
                                                        }
                                                        let _e3431 = phi_23067_;
                                                        if (bitcast<i32>(_e3431.member) != 0i) {
                                                            phi_23114_ = type_50(0u, type_50().member_1);
                                                        } else {
                                                            if (_e3431.member_1.member_1 <= 0f) {
                                                                phi_23110_ = type_50(0u, type_50().member_1);
                                                            } else {
                                                                if (_e3431.member_1.member != _e3431.member_1.member) {
                                                                    phi_23218_ = true;
                                                                } else {
                                                                    phi_23218_ = (0f >= _e3431.member_1.member);
                                                                }
                                                                let _e3443 = phi_23218_;
                                                                let _e3444 = select(_e3431.member_1.member, 0f, _e3443);
                                                                if _e3321 {
                                                                    let _e3445 = (_e3388 - _e3320);
                                                                    if (_e3445 < 0f) {
                                                                        phi_23326_ = type_50(0u, type_50().member_1);
                                                                        phi_23327_ = type_50();
                                                                        phi_23328_ = true;
                                                                    } else {
                                                                        if (_e3386 < 0f) {
                                                                            phi_23298_ = -(sqrt(_e3445));
                                                                        } else {
                                                                            phi_23298_ = sqrt(_e3445);
                                                                        }
                                                                        let _e3452 = phi_23298_;
                                                                        let _e3454 = -((_e3386 + _e3452));
                                                                        let _e3455 = (_e3454 == 0f);
                                                                        if _e3455 {
                                                                            phi_23321_ = type_50(1u, type_49(0f, 0f));
                                                                            phi_23322_ = type_50();
                                                                        } else {
                                                                            let _e3456 = (_e3320 / _e3454);
                                                                            if (_e3454 < _e3456) {
                                                                                phi_23310_ = type_49(_e3454, _e3456);
                                                                            } else {
                                                                                phi_23310_ = type_49(_e3456, _e3454);
                                                                            }
                                                                            let _e3461 = phi_23310_;
                                                                            phi_23321_ = type_50();
                                                                            phi_23322_ = type_50(1u, _e3461);
                                                                        }
                                                                        let _e3464 = phi_23321_;
                                                                        let _e3466 = phi_23322_;
                                                                        phi_23326_ = _e3464;
                                                                        phi_23327_ = _e3466;
                                                                        phi_23328_ = _e3455;
                                                                    }
                                                                    let _e3471 = phi_23326_;
                                                                    let _e3473 = phi_23327_;
                                                                    let _e3475 = phi_23328_;
                                                                    if _e3475 {
                                                                        phi_23332_ = _e3471;
                                                                    } else {
                                                                        phi_23332_ = _e3473;
                                                                    }
                                                                    let _e3477 = phi_23332_;
                                                                    if (bitcast<i32>(_e3477.member) != 0i) {
                                                                        phi_23266_ = select(_e3477.member_1.member_1, _e3477.member_1.member, (_e3477.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23266_ = -1f;
                                                                    }
                                                                    let _e3488 = phi_23266_;
                                                                    if (_e3488 >= 0f) {
                                                                        if (_e3431.member_1.member_1 != _e3431.member_1.member_1) {
                                                                            phi_23370_ = true;
                                                                        } else {
                                                                            phi_23370_ = (_e3488 <= _e3431.member_1.member_1);
                                                                        }
                                                                        let _e3493 = phi_23370_;
                                                                        phi_23095_ = select(_e3431.member_1.member_1, _e3488, _e3493);
                                                                    } else {
                                                                        phi_23095_ = _e3431.member_1.member_1;
                                                                    }
                                                                    let _e3496 = phi_23095_;
                                                                    phi_23097_ = _e3496;
                                                                } else {
                                                                    phi_23097_ = _e3431.member_1.member_1;
                                                                }
                                                                let _e3498 = phi_23097_;
                                                                if (_e3498 <= _e3444) {
                                                                    phi_23106_ = type_50(0u, type_50().member_1);
                                                                } else {
                                                                    phi_23106_ = type_50(1u, type_49(_e3444, _e3498));
                                                                }
                                                                let _e3506 = phi_23106_;
                                                                phi_23110_ = _e3506;
                                                            }
                                                            let _e3511 = phi_23110_;
                                                            phi_23114_ = _e3511;
                                                        }
                                                        let _e3516 = phi_23114_;
                                                        let _e3519 = (bitcast<i32>(_e3516.member) != 0i);
                                                        if _e3519 {
                                                            phi_4584_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_4587_ = 0u;
                                                            loop {
                                                                let _e3525 = phi_4584_;
                                                                let _e3527 = phi_4587_;
                                                                local_20 = _e3525;
                                                                local_21 = _e3525;
                                                                local_22 = _e3525;
                                                                let _e3528 = (_e3527 < 32u);
                                                                if _e3528 {
                                                                    let _e3529 = -(_e3386);
                                                                    if (_e3516.member_1.member <= _e3516.member_1.member_1) {
                                                                    } else {
                                                                        phi_31907_ = true;
                                                                        break;
                                                                    }
                                                                    let _e3532 = select(_e3529, _e3516.member_1.member, (_e3529 < _e3516.member_1.member));
                                                                    let _e3534 = select(_e3532, _e3516.member_1.member_1, (_e3532 > _e3516.member_1.member_1));
                                                                    let _e3535 = (_e3534 - _e3516.member_1.member);
                                                                    let _e3536 = (_e3516.member_1.member_1 - _e3534);
                                                                    let _e3537 = (_e3516.member_1.member_1 - _e3516.member_1.member);
                                                                    let _e3538 = (0.000000001f * _e3537);
                                                                    if (_e3535 <= _e3538) {
                                                                        let _e3613 = (f32(_e3527) * 0.03125f);
                                                                        let _e3616 = (f32((_e3527 + 1u)) * 0.03125f);
                                                                        let _e3618 = ((_e3613 + _e3616) * 0.5f);
                                                                        phi_4688_ = type_49((_e3516.member_1.member + ((_e3537 * _e3618) * _e3618)), (_e3537 * ((_e3616 * _e3616) - (_e3613 * _e3613))));
                                                                    } else {
                                                                        if (_e3536 <= _e3538) {
                                                                            let _e3596 = (f32((31u - _e3527)) * 0.03125f);
                                                                            let _e3599 = (f32((32u - _e3527)) * 0.03125f);
                                                                            let _e3601 = ((_e3596 + _e3599) * 0.5f);
                                                                            phi_4687_ = type_49((_e3516.member_1.member_1 - ((_e3537 * _e3601) * _e3601)), (_e3537 * ((_e3599 * _e3599) - (_e3596 * _e3596))));
                                                                        } else {
                                                                            let _e3544 = round(((32f * _e3535) / (_e3535 + _e3536)));
                                                                            let _e3549 = select(select(u32(_e3544), 0u, (_e3544 < 0f)), 4294967295u, (_e3544 > 4294967000f));
                                                                            if (_e3549 < 1u) {
                                                                                phi_4671_ = 1u;
                                                                            } else {
                                                                                phi_4671_ = select(_e3549, 31u, (_e3549 > 31u));
                                                                            }
                                                                            let _e3554 = phi_4671_;
                                                                            if (_e3527 < _e3554) {
                                                                                let _e3575 = ((_e3554 - 1u) - _e3527);
                                                                                let _e3577 = f32(_e3554);
                                                                                let _e3578 = (f32(_e3575) / _e3577);
                                                                                let _e3581 = (f32((_e3575 + 1u)) / _e3577);
                                                                                let _e3583 = ((_e3578 + _e3581) * 0.5f);
                                                                                phi_4686_ = type_49((_e3534 - ((_e3535 * _e3583) * _e3583)), (_e3535 * ((_e3581 * _e3581) - (_e3578 * _e3578))));
                                                                            } else {
                                                                                let _e3557 = (_e3527 - _e3554);
                                                                                let _e3559 = f32((32u - _e3554));
                                                                                let _e3560 = (f32(_e3557) / _e3559);
                                                                                let _e3563 = (f32((_e3557 + 1u)) / _e3559);
                                                                                let _e3565 = ((_e3560 + _e3563) * 0.5f);
                                                                                phi_4686_ = type_49((_e3534 + ((_e3536 * _e3565) * _e3565)), (_e3536 * ((_e3563 * _e3563) - (_e3560 * _e3560))));
                                                                            }
                                                                            let _e3593 = phi_4686_;
                                                                            phi_4687_ = _e3593;
                                                                        }
                                                                        let _e3611 = phi_4687_;
                                                                        phi_4688_ = _e3611;
                                                                    }
                                                                    let _e3628 = phi_4688_;
                                                                    let _e3634 = (_e321.member.x + (_e321.member_1.x * _e3628.member));
                                                                    let _e3635 = (_e321.member.y + (_e321.member_1.y * _e3628.member));
                                                                    let _e3636 = (_e321.member.z + (_e321.member_1.z * _e3628.member));
                                                                    let _e3650 = (_e3634 - _e3265);
                                                                    let _e3651 = (_e3635 - _e3268);
                                                                    let _e3652 = (_e3636 - _e3271);
                                                                    let _e3660 = ((((((_e3634 * _e3634) + (_e3635 * _e3635)) + (_e3636 * _e3636)) - (2f * (((_e3634 * _e3265) + (_e3635 * _e3268)) + (_e3636 * _e3271)))) + _e3277) / (sqrt((((_e3650 * _e3650) + (_e3651 * _e3651)) + (_e3652 * _e3652))) + _e3274));
                                                                    let _e3664 = (1f - (abs((_e3660 - 25000f)) * 0.00006666667f));
                                                                    if (_e3664 != _e3664) {
                                                                        phi_23569_ = true;
                                                                    } else {
                                                                        phi_23569_ = (0f >= _e3664);
                                                                    }
                                                                    let _e3668 = phi_23569_;
                                                                    let _e3670 = -(_e3660);
                                                                    phi_4585_ = (_e3525 + vec3<f32>((exp((_e3670 / _e3283)) * _e3628.member_1), (exp((_e3670 / _e3286)) * _e3628.member_1), (select(_e3664, 0f, _e3668) * _e3628.member_1)));
                                                                    phi_4588_ = (_e3527 + 1u);
                                                                } else {
                                                                    phi_4585_ = vec3<f32>();
                                                                    phi_4588_ = u32();
                                                                }
                                                                let _e3682 = phi_4585_;
                                                                let _e3684 = phi_4588_;
                                                                phi_4584_ = _e3682;
                                                                phi_4587_ = _e3684;
                                                                phi_31907_ = _e3114;
                                                                if !(_e3528) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e3687 = phi_31907_;
                                                            phi_32203_ = _e3687;
                                                            if _e3687 {
                                                                break;
                                                            }
                                                            let _e3689 = local_20;
                                                            let _e3696 = local_21;
                                                            let _e3698 = ((_e3298 * 1.1f) * _e3696.y);
                                                            let _e3703 = local_22;
                                                            phi_31974_ = _e3687;
                                                            phi_4713_ = vec3<f32>(exp(-((((_e3289 * _e3689.x) + _e3698) + (_e3301 * _e3703.z)))), exp(-((((_e3292 * _e3689.x) + _e3698) + (_e3304 * _e3703.z)))), exp(-((((_e3295 * _e3689.x) + _e3698) + (_e3307 * _e3703.z)))));
                                                        } else {
                                                            phi_31974_ = _e3114;
                                                            phi_4713_ = vec3<f32>();
                                                        }
                                                        let _e3719 = phi_31974_;
                                                        let _e3721 = phi_4713_;
                                                        phi_31973_ = _e3719;
                                                        phi_4715_ = _e3721;
                                                        phi_4717_ = select(true, false, _e3519);
                                                    } else {
                                                        phi_31973_ = _e3114;
                                                        phi_4715_ = vec3<f32>();
                                                        phi_4717_ = true;
                                                    }
                                                    let _e3724 = phi_31973_;
                                                    let _e3726 = phi_4715_;
                                                    let _e3728 = phi_4717_;
                                                    let _e3732 = select(_e3726, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3378)), vec3(_e3728));
                                                    let _e3736 = select(_e3732.y, _e3732.x, (_e3732.x > _e3732.y));
                                                    phi_31969_ = _e3724;
                                                    phi_4728_ = _e3732;
                                                    phi_4729_ = (select(_e3732.z, _e3736, (_e3736 > _e3732.z)) > 0f);
                                                }
                                                let _e3742 = phi_31969_;
                                                let _e3744 = phi_4728_;
                                                let _e3746 = phi_4729_;
                                                phi_31968_ = _e3742;
                                                phi_4731_ = _e3744;
                                                phi_4732_ = _e3746;
                                                phi_4733_ = select(true, false, _e3263);
                                            } else {
                                                phi_31968_ = _e2883;
                                                phi_4731_ = vec3<f32>(1f, 1f, 1f);
                                                phi_4732_ = _e1427;
                                                phi_4733_ = true;
                                            }
                                            let _e3749 = phi_31968_;
                                            let _e3751 = phi_4731_;
                                            let _e3753 = phi_4732_;
                                            let _e3755 = phi_4733_;
                                            phi_31967_ = _e3749;
                                            let _e7011 = local_61;
                                            phi_4737_ = _e7011;
                                            phi_4738_ = _e3751;
                                            phi_4739_ = _e3753;
                                            let _e7016 = local_62;
                                            phi_4740_ = _e7016;
                                            phi_4741_ = _e3755;
                                        } else {
                                            phi_31967_ = _e310;
                                            phi_4737_ = vec3<f32>(0f, 0f, 0f);
                                            phi_4738_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4739_ = _e1427;
                                            phi_4740_ = vec3<f32>();
                                            phi_4741_ = true;
                                        }
                                        let _e3757 = phi_31967_;
                                        let _e3759 = phi_4737_;
                                        let _e3761 = phi_4738_;
                                        let _e3763 = phi_4739_;
                                        let _e3765 = phi_4740_;
                                        let _e3767 = phi_4741_;
                                        if _e3767 {
                                            if _e3763 {
                                                let _e3768 = (_e1279 + 20u);
                                                let _e3771 = global_5.member[(_e1279 + 21u)];
                                                let _e3772 = bitcast<u32>(_e3771);
                                                let _e3775 = global_5.member[(_e1279 + 22u)];
                                                let _e3776 = bitcast<u32>(_e3775);
                                                let _e3779 = global_5.member[(_e1279 + 23u)];
                                                let _e3783 = global_5.member[(_e1279 + 24u)];
                                                let _e3787 = global_5.member[(_e1279 + 25u)];
                                                let _e3791 = global_5.member[(_e1279 + 26u)];
                                                let _e3794 = global_5.member[_e1279];
                                                let _e3797 = global_5.member[(_e1279 + 1u)];
                                                let _e3800 = global_5.member[(_e1279 + 2u)];
                                                let _e3803 = global_5.member[(_e1279 + 3u)];
                                                let _e3806 = global_5.member[(_e1279 + 4u)];
                                                let _e3809 = global_5.member[(_e1279 + 5u)];
                                                let _e3812 = global_5.member[(_e1279 + 6u)];
                                                let _e3815 = global_5.member[(_e1279 + 7u)];
                                                let _e3818 = global_5.member[(_e1279 + 8u)];
                                                let _e3823 = (((_e321.member_1.x * _e3803) + (_e321.member_1.y * _e3806)) + (_e321.member_1.z * _e3809));
                                                let _e3825 = select(_e3823, -1f, (_e3823 < -1f));
                                                let _e3828 = asin(select(_e3825, 1f, (_e3825 > 1f)));
                                                let _e3839 = atan2((((_e321.member_1.x * _e3812) + (_e321.member_1.y * _e3815)) + (_e321.member_1.z * _e3818)), (((_e321.member_1.x * _e3794) + (_e321.member_1.y * _e3797)) + (_e321.member_1.z * _e3800)));
                                                if (_e3772 != 4294967295u) {
                                                    let _e3842 = global_5.member[_e3772];
                                                    let _e3845 = global_5.member[(_e3772 + 1u)];
                                                    let _e3846 = bitcast<u32>(_e3845);
                                                    let _e3849 = global_5.member[(_e3772 + 2u)];
                                                    let _e3853 = global_5.member[(_e3772 + 3u)];
                                                    let _e3855 = (4f * _e3842);
                                                    phi_5061_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_5064_ = 0u;
                                                    loop {
                                                        let _e3860 = phi_5061_;
                                                        let _e3862 = phi_5064_;
                                                        local_66 = _e3860;
                                                        let _e3863 = (_e3862 < 6u);
                                                        if _e3863 {
                                                            let _e3864 = (_e3862 / 2u);
                                                            if (_e3864 == 0u) {
                                                                phi_23640_ = _e321.member_1.x;
                                                            } else {
                                                                phi_23640_ = select(_e321.member_1.z, _e321.member_1.y, (_e3864 == 1u));
                                                            }
                                                            let _e3872 = phi_23640_;
                                                            let _e3873 = (_e3872 * select(-1f, 1f, ((_e3862 % 2u) == 0u)));
                                                            if (_e3873 > 0f) {
                                                                let _e3877 = ((_e3864 + 1u) % 3u);
                                                                if (_e3877 == 0u) {
                                                                    phi_23655_ = _e321.member_1.x;
                                                                } else {
                                                                    phi_23655_ = select(_e321.member_1.z, _e321.member_1.y, (_e3877 == 1u));
                                                                }
                                                                let _e3882 = phi_23655_;
                                                                let _e3883 = (_e3882 / _e3873);
                                                                let _e3885 = ((_e3864 + 2u) % 3u);
                                                                if (_e3885 == 0u) {
                                                                    phi_23670_ = _e321.member_1.x;
                                                                } else {
                                                                    phi_23670_ = select(_e321.member_1.z, _e321.member_1.y, (_e3885 == 1u));
                                                                }
                                                                let _e3890 = phi_23670_;
                                                                let _e3891 = (_e3890 / _e3873);
                                                                let _e3893 = (1f + (_e3842 * 12f));
                                                                if (abs(_e3883) <= _e3893) {
                                                                    if (abs(_e3891) <= _e3893) {
                                                                        let _e3899 = f32(_e3846);
                                                                        let _e3901 = floor((((_e3883 + 1f) * 0.5f) * _e3899));
                                                                        let _e3908 = select(0i, select(select(i32(_e3901), i32(-2147483648), (_e3901 < -2147483600f)), 2147483647i, (_e3901 > 2147483500f)), (_e3901 == _e3901));
                                                                        let _e3910 = (bitcast<i32>(_e3846) - 1i);
                                                                        let _e3911 = (0i <= _e3910);
                                                                        if _e3911 {
                                                                        } else {
                                                                            phi_31962_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3908 < 0i) {
                                                                            phi_5135_ = 0i;
                                                                        } else {
                                                                            phi_5135_ = select(_e3908, _e3910, (_e3908 > _e3910));
                                                                        }
                                                                        let _e3916 = phi_5135_;
                                                                        let _e3920 = floor((((_e3891 + 1f) * 0.5f) * _e3899));
                                                                        let _e3927 = select(0i, select(select(i32(_e3920), i32(-2147483648), (_e3920 < -2147483600f)), 2147483647i, (_e3920 > 2147483500f)), (_e3920 == _e3920));
                                                                        if _e3911 {
                                                                        } else {
                                                                            phi_31962_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3927 < 0i) {
                                                                            phi_5165_ = 0i;
                                                                        } else {
                                                                            phi_5165_ = select(_e3927, _e3910, (_e3927 > _e3910));
                                                                        }
                                                                        let _e3932 = phi_5165_;
                                                                        let _e3933 = (_e3916 - 1i);
                                                                        phi_5173_ = _e3860;
                                                                        phi_5176_ = select(0i, _e3933, (0i < _e3933));
                                                                        loop {
                                                                            let _e3937 = phi_5173_;
                                                                            let _e3939 = phi_5176_;
                                                                            let _e3940 = (_e3916 + 1i);
                                                                            local_65 = _e3937;
                                                                            let _e3943 = (_e3939 <= select(_e3940, _e3910, (_e3910 < _e3940)));
                                                                            if _e3943 {
                                                                                let _e3944 = (_e3932 - 1i);
                                                                                phi_5199_ = _e3937;
                                                                                phi_5202_ = select(0i, _e3944, (0i < _e3944));
                                                                                loop {
                                                                                    let _e3948 = phi_5199_;
                                                                                    let _e3950 = phi_5202_;
                                                                                    let _e3951 = (_e3932 + 1i);
                                                                                    local_64 = _e3948;
                                                                                    let _e3954 = (_e3950 <= select(_e3951, _e3910, (_e3910 < _e3951)));
                                                                                    if _e3954 {
                                                                                        let _e3961 = (bitcast<u32>(_e3849) + ((((_e3862 * _e3846) + bitcast<u32>(_e3939)) * _e3846) + bitcast<u32>(_e3950)));
                                                                                        let _e3963 = global_5.member[_e3961];
                                                                                        let _e3967 = global_5.member[(_e3961 + 1u)];
                                                                                        phi_5273_ = _e3948;
                                                                                        phi_5276_ = bitcast<u32>(_e3963);
                                                                                        loop {
                                                                                            let _e3970 = phi_5273_;
                                                                                            let _e3972 = phi_5276_;
                                                                                            local_63 = _e3970;
                                                                                            let _e3973 = (_e3972 < bitcast<u32>(_e3967));
                                                                                            if _e3973 {
                                                                                                let _e3975 = (bitcast<u32>(_e3853) + (6u * _e3972));
                                                                                                let _e3977 = global_5.member[_e3975];
                                                                                                let _e3980 = global_5.member[(_e3975 + 1u)];
                                                                                                let _e3983 = global_5.member[(_e3975 + 2u)];
                                                                                                let _e3986 = global_5.member[(_e3975 + 3u)];
                                                                                                let _e3989 = global_5.member[(_e3975 + 4u)];
                                                                                                let _e3992 = global_5.member[(_e3975 + 5u)];
                                                                                                let _e3993 = (_e321.member_1.x - _e3977);
                                                                                                let _e3994 = (_e321.member_1.y - _e3980);
                                                                                                let _e3995 = (_e321.member_1.z - _e3983);
                                                                                                let _e4000 = (((_e3993 * _e3993) + (_e3994 * _e3994)) + (_e3995 * _e3995));
                                                                                                if (_e4000 < (_e3855 * _e3855)) {
                                                                                                    let _e4008 = ((1f / ((6.2831855f * _e3842) * _e3842)) * exp((-(_e4000) / ((2f * _e3842) * _e3842))));
                                                                                                    phi_5396_ = vec3<f32>((_e3970.x + (_e3986 * _e4008)), (_e3970.y + (_e3989 * _e4008)), (_e3970.z + (_e3992 * _e4008)));
                                                                                                } else {
                                                                                                    phi_5396_ = _e3970;
                                                                                                }
                                                                                                let _e4020 = phi_5396_;
                                                                                                phi_5274_ = _e4020;
                                                                                                phi_5277_ = (_e3972 + 1u);
                                                                                            } else {
                                                                                                phi_5274_ = vec3<f32>();
                                                                                                phi_5277_ = u32();
                                                                                            }
                                                                                            let _e4023 = phi_5274_;
                                                                                            let _e4025 = phi_5277_;
                                                                                            phi_5273_ = _e4023;
                                                                                            phi_5276_ = _e4025;
                                                                                            if !(_e3973) {
                                                                                                break;
                                                                                            }
                                                                                        }
                                                                                        let _e7036 = local_63;
                                                                                        phi_5200_ = _e7036;
                                                                                        phi_5203_ = (_e3950 + 1i);
                                                                                    } else {
                                                                                        phi_5200_ = vec3<f32>();
                                                                                        phi_5203_ = i32();
                                                                                    }
                                                                                    let _e4029 = phi_5200_;
                                                                                    let _e4031 = phi_5203_;
                                                                                    phi_5199_ = _e4029;
                                                                                    phi_5202_ = _e4031;
                                                                                    if !(_e3954) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e7040 = local_64;
                                                                                phi_5174_ = _e7040;
                                                                                phi_5177_ = (_e3939 + 1i);
                                                                            } else {
                                                                                phi_5174_ = vec3<f32>();
                                                                                phi_5177_ = i32();
                                                                            }
                                                                            let _e4035 = phi_5174_;
                                                                            let _e4037 = phi_5177_;
                                                                            phi_5173_ = _e4035;
                                                                            phi_5176_ = _e4037;
                                                                            if !(_e3943) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e7044 = local_65;
                                                                        phi_5401_ = _e7044;
                                                                    } else {
                                                                        phi_5401_ = _e3860;
                                                                    }
                                                                    let _e4040 = phi_5401_;
                                                                    phi_5402_ = _e4040;
                                                                } else {
                                                                    phi_5402_ = _e3860;
                                                                }
                                                                let _e4042 = phi_5402_;
                                                                phi_5403_ = _e4042;
                                                            } else {
                                                                phi_5403_ = _e3860;
                                                            }
                                                            let _e4044 = phi_5403_;
                                                            phi_5062_ = _e4044;
                                                            phi_5065_ = (_e3862 + 1u);
                                                        } else {
                                                            phi_5062_ = vec3<f32>();
                                                            phi_5065_ = u32();
                                                        }
                                                        let _e4047 = phi_5062_;
                                                        let _e4049 = phi_5065_;
                                                        phi_5061_ = _e4047;
                                                        phi_5064_ = _e4049;
                                                        phi_31962_ = _e3757;
                                                        if !(_e3863) {
                                                            break;
                                                        }
                                                    }
                                                    let _e4052 = phi_31962_;
                                                    phi_32203_ = _e4052;
                                                    if _e4052 {
                                                        break;
                                                    }
                                                    phi_32004_ = _e4052;
                                                    let _e7053 = local_66;
                                                    phi_5423_ = _e7053;
                                                } else {
                                                    phi_32004_ = _e3757;
                                                    phi_5423_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e4054 = phi_32004_;
                                                let _e4056 = phi_5423_;
                                                if (_e3776 != 4294967295u) {
                                                    let _e4059 = global_5.member[_e3776];
                                                    let _e4062 = global_5.member[(_e3776 + 1u)];
                                                    let _e4065 = global_5.member[(_e3776 + 2u)];
                                                    let _e4068 = global_5.member[(_e3776 + 3u)];
                                                    let _e4071 = global_5.member[(_e3776 + 4u)];
                                                    let _e4074 = global_5.member[(_e3776 + 5u)];
                                                    let _e4077 = global_5.member[(_e3776 + 6u)];
                                                    let _e4080 = global_5.member[(_e3776 + 7u)];
                                                    let _e4083 = global_5.member[(_e3776 + 8u)];
                                                    let _e4086 = global_5.member[(_e3776 + 9u)];
                                                    let _e4089 = global_5.member[(_e3776 + 10u)];
                                                    let _e4093 = global_5.member[(_e3776 + 11u)];
                                                    let _e4096 = global_5.member[(_e3776 + 12u)];
                                                    let _e4099 = global_5.member[(_e3776 + 13u)];
                                                    let _e4101 = (_e4059 > 0f);
                                                    let _e4102 = (_e4062 > 0f);
                                                    if _e4102 {
                                                        let _e4105 = (abs(_e3828) <= 0.8f);
                                                        let _e4106 = select(2u, 0u, _e4101);
                                                        phi_5636_ = _e4106;
                                                        phi_5637_ = _e4105;
                                                        phi_5640_ = _e4106;
                                                        phi_5641_ = _e4105;
                                                        phi_5642_ = select(true, false, _e4105);
                                                    } else {
                                                        phi_5636_ = u32();
                                                        phi_5637_ = bool();
                                                        phi_5640_ = select(2u, 0u, _e4101);
                                                        phi_5641_ = false;
                                                        phi_5642_ = true;
                                                    }
                                                    let _e4109 = phi_5636_;
                                                    let _e4111 = phi_5637_;
                                                    let _e4113 = phi_5640_;
                                                    let _e4115 = phi_5641_;
                                                    let _e4117 = phi_5642_;
                                                    let _e4119 = select(select(f32(), 0f, _e4102), 0f, _e4117);
                                                    phi_5652_ = _e4119;
                                                    phi_5655_ = _e4119;
                                                    phi_5657_ = select(_e4109, _e4113, _e4117);
                                                    phi_5659_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_5661_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_5663_ = 0f;
                                                    phi_5665_ = 1f;
                                                    loop {
                                                        let _e4124 = phi_5652_;
                                                        let _e4126 = phi_5655_;
                                                        let _e4128 = phi_5657_;
                                                        let _e4130 = phi_5659_;
                                                        let _e4132 = phi_5661_;
                                                        let _e4134 = phi_5663_;
                                                        let _e4136 = phi_5665_;
                                                        local_26 = _e4136;
                                                        local_27 = _e4136;
                                                        local_28 = _e4136;
                                                        local_29 = _e4136;
                                                        local_30 = _e4134;
                                                        local_31 = _e4126;
                                                        local_32 = _e4124;
                                                        let _e4137 = (_e4128 < select(11u, 2u, _e4117));
                                                        if _e4137 {
                                                            let _e4138 = (_e4128 < 2u);
                                                            if _e4138 {
                                                                let _e4194 = select(23f, 4f, (_e4128 == 0u));
                                                                phi_5743_ = 4u;
                                                                phi_5745_ = vec3<f32>((_e321.member_1.x * _e4194), (_e321.member_1.y * _e4194), (_e321.member_1.z * _e4194));
                                                            } else {
                                                                if (_e4128 == 2u) {
                                                                    phi_5741_ = 3u;
                                                                    phi_5742_ = vec3<f32>(((_e321.member_1.x * 1.5f) + 5f), ((_e321.member_1.y * 1.5f) + 5f), ((_e321.member_1.z * 1.5f) + 5f));
                                                                } else {
                                                                    let _e4140 = (_e4128 >= 7u);
                                                                    if _e4140 {
                                                                        phi_5703_ = (_e4128 - 7u);
                                                                    } else {
                                                                        phi_5703_ = (_e4128 - 3u);
                                                                    }
                                                                    let _e4144 = phi_5703_;
                                                                    if _e4140 {
                                                                        phi_5718_ = vec3<f32>(((_e321.member_1.x * 11f) + -3f), ((_e321.member_1.y * 11f) + 1f), ((_e321.member_1.z * 11f) + 7f));
                                                                    } else {
                                                                        phi_5718_ = vec3<f32>((_e321.member_1.x * 7f), (_e321.member_1.y * 7f), (_e321.member_1.z * 7f));
                                                                    }
                                                                    let _e4157 = phi_5718_;
                                                                    if (_e4144 < 3u) {
                                                                        if (_e4144 == 0u) {
                                                                            phi_23685_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                        } else {
                                                                            phi_23685_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4144 == 1u)));
                                                                        }
                                                                        let _e4176 = phi_23685_;
                                                                        phi_5739_ = 4u;
                                                                        phi_5740_ = (_e4157 + _e4176);
                                                                    } else {
                                                                        if _e4140 {
                                                                            phi_5738_ = (_e4157 + _e4130);
                                                                        } else {
                                                                            phi_5738_ = (_e4157 + vec3<f32>((_e4132.x * 1.1f), (_e4132.y * 1.1f), (_e4132.z * 1.1f)));
                                                                        }
                                                                        let _e4169 = phi_5738_;
                                                                        phi_5739_ = select(7u, 5u, _e4140);
                                                                        phi_5740_ = _e4169;
                                                                    }
                                                                    let _e4179 = phi_5739_;
                                                                    let _e4181 = phi_5740_;
                                                                    phi_5741_ = _e4179;
                                                                    phi_5742_ = _e4181;
                                                                }
                                                                let _e4190 = phi_5741_;
                                                                let _e4192 = phi_5742_;
                                                                phi_5743_ = _e4190;
                                                                phi_5745_ = _e4192;
                                                            }
                                                            let _e4200 = phi_5743_;
                                                            let _e4202 = phi_5745_;
                                                            let _e4204 = select(vec3<f32>(_e4093, _e4096, _e4099), vec3<f32>(_e4083, _e4086, _e4089), vec3(_e4138));
                                                            let _e4206 = global_5.member[_e3768];
                                                            let _e4207 = bitcast<u32>(_e4206);
                                                            let _e4210 = global_5.member[(_e1279 + 11u)];
                                                            let _e4213 = global_5.member[(_e1279 + 12u)];
                                                            let _e4216 = global_5.member[(_e1279 + 13u)];
                                                            let _e4219 = global_5.member[(_e1279 + 14u)];
                                                            let _e4222 = global_5.member[(_e1279 + 15u)];
                                                            let _e4225 = global_5.member[(_e1279 + 16u)];
                                                            let _e4228 = global_5.member[(_e1279 + 17u)];
                                                            let _e4231 = global_5.member[(_e1279 + 18u)];
                                                            let _e4234 = global_5.member[(_e1279 + 19u)];
                                                            phi_5880_ = 0u;
                                                            phi_5883_ = 0f;
                                                            phi_5885_ = 1f;
                                                            phi_5887_ = 0f;
                                                            phi_5889_ = _e4202;
                                                            loop {
                                                                let _e4236 = phi_5880_;
                                                                let _e4238 = phi_5883_;
                                                                let _e4240 = phi_5885_;
                                                                let _e4242 = phi_5887_;
                                                                let _e4244 = phi_5889_;
                                                                local_23 = _e4238;
                                                                local_24 = _e4242;
                                                                local_25 = _e4238;
                                                                let _e4245 = (_e4236 < _e4200);
                                                                if _e4245 {
                                                                    let _e4248 = (_e4244.x + _e4204.x);
                                                                    let _e4251 = (_e4244.y + _e4204.y);
                                                                    let _e4254 = (_e4244.z + _e4204.z);
                                                                    let _e4255 = floor(_e4248);
                                                                    let _e4256 = floor(_e4251);
                                                                    let _e4257 = floor(_e4254);
                                                                    let _e4266 = bitcast<u32>((select(0i, select(select(i32(_e4255), i32(-2147483648), (_e4255 < -2147483600f)), 2147483647i, (_e4255 > 2147483500f)), (_e4255 == _e4255)) & 255i));
                                                                    let _e4275 = bitcast<u32>((select(0i, select(select(i32(_e4256), i32(-2147483648), (_e4256 < -2147483600f)), 2147483647i, (_e4256 > 2147483500f)), (_e4256 == _e4256)) & 255i));
                                                                    let _e4284 = bitcast<u32>((select(0i, select(select(i32(_e4257), i32(-2147483648), (_e4257 < -2147483600f)), 2147483647i, (_e4257 > 2147483500f)), (_e4257 == _e4257)) & 255i));
                                                                    let _e4285 = (_e4248 - _e4255);
                                                                    let _e4286 = (_e4251 - _e4256);
                                                                    let _e4287 = (_e4254 - _e4257);
                                                                    let _e4294 = (((_e4285 * _e4285) * _e4285) * ((_e4285 * ((_e4285 * 6f) - 15f)) + 10f));
                                                                    let _e4301 = (((_e4286 * _e4286) * _e4286) * ((_e4286 * ((_e4286 * 6f) - 15f)) + 10f));
                                                                    let _e4311 = global_5.member[(_e4207 + _e4266)];
                                                                    let _e4317 = (select(select(u32(_e4311), 0u, (_e4311 < 0f)), 4294967295u, (_e4311 > 4294967000f)) + _e4275);
                                                                    let _e4320 = global_5.member[(_e4207 + _e4317)];
                                                                    let _e4326 = (select(select(u32(_e4320), 0u, (_e4320 < 0f)), 4294967295u, (_e4320 > 4294967000f)) + _e4284);
                                                                    let _e4330 = global_5.member[(_e4207 + (_e4317 + 1u))];
                                                                    let _e4336 = (select(select(u32(_e4330), 0u, (_e4330 < 0f)), 4294967295u, (_e4330 > 4294967000f)) + _e4284);
                                                                    let _e4340 = global_5.member[(_e4207 + (_e4266 + 1u))];
                                                                    let _e4346 = (select(select(u32(_e4340), 0u, (_e4340 < 0f)), 4294967295u, (_e4340 > 4294967000f)) + _e4275);
                                                                    let _e4349 = global_5.member[(_e4207 + _e4346)];
                                                                    let _e4355 = (select(select(u32(_e4349), 0u, (_e4349 < 0f)), 4294967295u, (_e4349 > 4294967000f)) + _e4284);
                                                                    let _e4359 = global_5.member[(_e4207 + (_e4346 + 1u))];
                                                                    let _e4365 = (select(select(u32(_e4359), 0u, (_e4359 < 0f)), 4294967295u, (_e4359 > 4294967000f)) + _e4284);
                                                                    let _e4368 = global_5.member[(_e4207 + _e4326)];
                                                                    let _e4374 = (select(select(u32(_e4368), 0u, (_e4368 < 0f)), 4294967295u, (_e4368 > 4294967000f)) % 12u);
                                                                    let _e4377 = select(-1f, 1f, ((_e4374 & 1u) == 0u));
                                                                    let _e4380 = select(-1f, 1f, ((_e4374 & 2u) == 0u));
                                                                    if (_e4374 < 4u) {
                                                                        phi_23746_ = ((_e4377 * _e4285) + (_e4380 * _e4286));
                                                                    } else {
                                                                        if (_e4374 < 8u) {
                                                                            phi_23740_ = ((_e4377 * _e4285) + (_e4380 * _e4287));
                                                                        } else {
                                                                            phi_23740_ = ((_e4377 * _e4286) + (_e4380 * _e4287));
                                                                        }
                                                                        let _e4390 = phi_23740_;
                                                                        phi_23746_ = _e4390;
                                                                    }
                                                                    let _e4395 = phi_23746_;
                                                                    let _e4398 = global_5.member[(_e4207 + _e4355)];
                                                                    let _e4404 = (_e4285 - 1f);
                                                                    let _e4405 = (select(select(u32(_e4398), 0u, (_e4398 < 0f)), 4294967295u, (_e4398 > 4294967000f)) % 12u);
                                                                    let _e4408 = select(-1f, 1f, ((_e4405 & 1u) == 0u));
                                                                    let _e4411 = select(-1f, 1f, ((_e4405 & 2u) == 0u));
                                                                    if (_e4405 < 4u) {
                                                                        phi_23780_ = ((_e4408 * _e4404) + (_e4411 * _e4286));
                                                                    } else {
                                                                        if (_e4405 < 8u) {
                                                                            phi_23774_ = ((_e4408 * _e4404) + (_e4411 * _e4287));
                                                                        } else {
                                                                            phi_23774_ = ((_e4408 * _e4286) + (_e4411 * _e4287));
                                                                        }
                                                                        let _e4421 = phi_23774_;
                                                                        phi_23780_ = _e4421;
                                                                    }
                                                                    let _e4426 = phi_23780_;
                                                                    let _e4429 = global_5.member[(_e4207 + _e4336)];
                                                                    let _e4435 = (_e4286 - 1f);
                                                                    let _e4436 = (select(select(u32(_e4429), 0u, (_e4429 < 0f)), 4294967295u, (_e4429 > 4294967000f)) % 12u);
                                                                    let _e4439 = select(-1f, 1f, ((_e4436 & 1u) == 0u));
                                                                    let _e4442 = select(-1f, 1f, ((_e4436 & 2u) == 0u));
                                                                    if (_e4436 < 4u) {
                                                                        phi_23814_ = ((_e4439 * _e4285) + (_e4442 * _e4435));
                                                                    } else {
                                                                        if (_e4436 < 8u) {
                                                                            phi_23808_ = ((_e4439 * _e4285) + (_e4442 * _e4287));
                                                                        } else {
                                                                            phi_23808_ = ((_e4439 * _e4435) + (_e4442 * _e4287));
                                                                        }
                                                                        let _e4452 = phi_23808_;
                                                                        phi_23814_ = _e4452;
                                                                    }
                                                                    let _e4457 = phi_23814_;
                                                                    let _e4460 = global_5.member[(_e4207 + _e4365)];
                                                                    let _e4466 = (select(select(u32(_e4460), 0u, (_e4460 < 0f)), 4294967295u, (_e4460 > 4294967000f)) % 12u);
                                                                    let _e4469 = select(-1f, 1f, ((_e4466 & 1u) == 0u));
                                                                    let _e4472 = select(-1f, 1f, ((_e4466 & 2u) == 0u));
                                                                    if (_e4466 < 4u) {
                                                                        phi_23848_ = ((_e4469 * _e4404) + (_e4472 * _e4435));
                                                                    } else {
                                                                        if (_e4466 < 8u) {
                                                                            phi_23842_ = ((_e4469 * _e4404) + (_e4472 * _e4287));
                                                                        } else {
                                                                            phi_23842_ = ((_e4469 * _e4435) + (_e4472 * _e4287));
                                                                        }
                                                                        let _e4482 = phi_23842_;
                                                                        phi_23848_ = _e4482;
                                                                    }
                                                                    let _e4487 = phi_23848_;
                                                                    let _e4491 = global_5.member[(_e4207 + (_e4326 + 1u))];
                                                                    let _e4497 = (_e4287 - 1f);
                                                                    let _e4498 = (select(select(u32(_e4491), 0u, (_e4491 < 0f)), 4294967295u, (_e4491 > 4294967000f)) % 12u);
                                                                    let _e4501 = select(-1f, 1f, ((_e4498 & 1u) == 0u));
                                                                    let _e4504 = select(-1f, 1f, ((_e4498 & 2u) == 0u));
                                                                    if (_e4498 < 4u) {
                                                                        phi_23882_ = ((_e4501 * _e4285) + (_e4504 * _e4286));
                                                                    } else {
                                                                        if (_e4498 < 8u) {
                                                                            phi_23876_ = ((_e4501 * _e4285) + (_e4504 * _e4497));
                                                                        } else {
                                                                            phi_23876_ = ((_e4501 * _e4286) + (_e4504 * _e4497));
                                                                        }
                                                                        let _e4514 = phi_23876_;
                                                                        phi_23882_ = _e4514;
                                                                    }
                                                                    let _e4519 = phi_23882_;
                                                                    let _e4523 = global_5.member[(_e4207 + (_e4355 + 1u))];
                                                                    let _e4529 = (select(select(u32(_e4523), 0u, (_e4523 < 0f)), 4294967295u, (_e4523 > 4294967000f)) % 12u);
                                                                    let _e4532 = select(-1f, 1f, ((_e4529 & 1u) == 0u));
                                                                    let _e4535 = select(-1f, 1f, ((_e4529 & 2u) == 0u));
                                                                    if (_e4529 < 4u) {
                                                                        phi_23916_ = ((_e4532 * _e4404) + (_e4535 * _e4286));
                                                                    } else {
                                                                        if (_e4529 < 8u) {
                                                                            phi_23910_ = ((_e4532 * _e4404) + (_e4535 * _e4497));
                                                                        } else {
                                                                            phi_23910_ = ((_e4532 * _e4286) + (_e4535 * _e4497));
                                                                        }
                                                                        let _e4545 = phi_23910_;
                                                                        phi_23916_ = _e4545;
                                                                    }
                                                                    let _e4550 = phi_23916_;
                                                                    let _e4554 = global_5.member[(_e4207 + (_e4336 + 1u))];
                                                                    let _e4560 = (select(select(u32(_e4554), 0u, (_e4554 < 0f)), 4294967295u, (_e4554 > 4294967000f)) % 12u);
                                                                    let _e4563 = select(-1f, 1f, ((_e4560 & 1u) == 0u));
                                                                    let _e4566 = select(-1f, 1f, ((_e4560 & 2u) == 0u));
                                                                    if (_e4560 < 4u) {
                                                                        phi_23950_ = ((_e4563 * _e4285) + (_e4566 * _e4435));
                                                                    } else {
                                                                        if (_e4560 < 8u) {
                                                                            phi_23944_ = ((_e4563 * _e4285) + (_e4566 * _e4497));
                                                                        } else {
                                                                            phi_23944_ = ((_e4563 * _e4435) + (_e4566 * _e4497));
                                                                        }
                                                                        let _e4576 = phi_23944_;
                                                                        phi_23950_ = _e4576;
                                                                    }
                                                                    let _e4581 = phi_23950_;
                                                                    let _e4585 = global_5.member[(_e4207 + (_e4365 + 1u))];
                                                                    let _e4591 = (select(select(u32(_e4585), 0u, (_e4585 < 0f)), 4294967295u, (_e4585 > 4294967000f)) % 12u);
                                                                    let _e4594 = select(-1f, 1f, ((_e4591 & 1u) == 0u));
                                                                    let _e4597 = select(-1f, 1f, ((_e4591 & 2u) == 0u));
                                                                    if (_e4591 < 4u) {
                                                                        phi_23984_ = ((_e4594 * _e4404) + (_e4597 * _e4435));
                                                                    } else {
                                                                        if (_e4591 < 8u) {
                                                                            phi_23978_ = ((_e4594 * _e4404) + (_e4597 * _e4497));
                                                                        } else {
                                                                            phi_23978_ = ((_e4594 * _e4435) + (_e4597 * _e4497));
                                                                        }
                                                                        let _e4607 = phi_23978_;
                                                                        phi_23984_ = _e4607;
                                                                    }
                                                                    let _e4612 = phi_23984_;
                                                                    let _e4615 = (_e4395 + (_e4294 * (_e4426 - _e4395)));
                                                                    let _e4621 = (_e4519 + (_e4294 * (_e4550 - _e4519)));
                                                                    let _e4627 = (_e4615 + (_e4301 * ((_e4457 + (_e4294 * (_e4487 - _e4457))) - _e4615)));
                                                                    phi_5881_ = (_e4236 + 1u);
                                                                    phi_5884_ = (_e4238 + _e4240);
                                                                    phi_5886_ = (_e4240 * 0.5f);
                                                                    phi_5888_ = (_e4242 + ((_e4627 + ((((_e4287 * _e4287) * _e4287) * ((_e4287 * ((_e4287 * 6f) - 15f)) + 10f)) * ((_e4621 + (_e4301 * ((_e4581 + (_e4294 * (_e4612 - _e4581))) - _e4621))) - _e4627))) * _e4240));
                                                                    phi_5890_ = vec3<f32>(((((_e4210 * _e4244.x) + (_e4213 * _e4244.y)) + (_e4216 * _e4244.z)) * 2.03f), ((((_e4219 * _e4244.x) + (_e4222 * _e4244.y)) + (_e4225 * _e4244.z)) * 2.03f), ((((_e4228 * _e4244.x) + (_e4231 * _e4244.y)) + (_e4234 * _e4244.z)) * 2.03f));
                                                                } else {
                                                                    phi_5881_ = u32();
                                                                    phi_5884_ = f32();
                                                                    phi_5886_ = f32();
                                                                    phi_5888_ = f32();
                                                                    phi_5890_ = vec3<f32>();
                                                                }
                                                                let _e4659 = phi_5881_;
                                                                let _e4661 = phi_5884_;
                                                                let _e4663 = phi_5886_;
                                                                let _e4665 = phi_5888_;
                                                                let _e4667 = phi_5890_;
                                                                phi_5880_ = _e4659;
                                                                phi_5883_ = _e4661;
                                                                phi_5885_ = _e4663;
                                                                phi_5887_ = _e4665;
                                                                phi_5889_ = _e4667;
                                                                if !(_e4245) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e4670 = local_23;
                                                            if (_e4670 > 0f) {
                                                                let _e4673 = local_24;
                                                                let _e4675 = local_25;
                                                                phi_6306_ = (_e4673 / _e4675);
                                                            } else {
                                                                phi_6306_ = 0f;
                                                            }
                                                            let _e4678 = phi_6306_;
                                                            if (_e4128 == 0u) {
                                                                phi_6389_ = _e4124;
                                                                phi_6390_ = _e4126;
                                                                phi_6391_ = _e4130;
                                                                phi_6392_ = _e4132;
                                                                phi_6393_ = _e4134;
                                                                phi_6394_ = (_e4136 + (1.5f * _e4678));
                                                            } else {
                                                                if (_e4128 == 1u) {
                                                                    phi_6383_ = _e4124;
                                                                    phi_6384_ = _e4126;
                                                                    phi_6385_ = _e4130;
                                                                    phi_6386_ = _e4132;
                                                                    phi_6387_ = _e4134;
                                                                    phi_6388_ = (_e4136 + (0.8f * _e4678));
                                                                } else {
                                                                    let _e4681 = (_e4128 == 2u);
                                                                    if _e4681 {
                                                                        phi_6378_ = _e4124;
                                                                        phi_6379_ = _e4126;
                                                                        phi_6380_ = _e4130;
                                                                        phi_6381_ = _e4132;
                                                                    } else {
                                                                        if (_e4128 == 3u) {
                                                                            phi_6374_ = _e4124;
                                                                            phi_6375_ = _e4126;
                                                                            phi_6376_ = _e4130;
                                                                            phi_6377_ = vec3<f32>(_e4678, _e4132.y, _e4132.z);
                                                                        } else {
                                                                            if (_e4128 == 4u) {
                                                                                phi_6370_ = _e4124;
                                                                                phi_6371_ = _e4126;
                                                                                phi_6372_ = _e4130;
                                                                                phi_6373_ = vec3<f32>(_e4132.x, _e4678, _e4132.z);
                                                                            } else {
                                                                                if (_e4128 == 5u) {
                                                                                    phi_6366_ = _e4124;
                                                                                    phi_6367_ = _e4126;
                                                                                    phi_6368_ = _e4130;
                                                                                    phi_6369_ = vec3<f32>(_e4132.x, _e4132.y, _e4678);
                                                                                } else {
                                                                                    let _e4685 = (_e4128 == 6u);
                                                                                    if _e4685 {
                                                                                        phi_6363_ = _e4124;
                                                                                        phi_6365_ = _e4130;
                                                                                    } else {
                                                                                        if (_e4128 == 7u) {
                                                                                            phi_6361_ = _e4124;
                                                                                            phi_6362_ = vec3<f32>(_e4678, _e4130.y, _e4130.z);
                                                                                        } else {
                                                                                            if (_e4128 == 8u) {
                                                                                                phi_6359_ = _e4124;
                                                                                                phi_6360_ = vec3<f32>(_e4130.x, _e4678, _e4130.z);
                                                                                            } else {
                                                                                                let _e4688 = (_e4128 == 9u);
                                                                                                if _e4688 {
                                                                                                    phi_6358_ = vec3<f32>(_e4130.x, _e4130.y, _e4678);
                                                                                                } else {
                                                                                                    phi_6358_ = _e4130;
                                                                                                }
                                                                                                let _e4694 = phi_6358_;
                                                                                                phi_6359_ = select(_e4678, _e4124, _e4688);
                                                                                                phi_6360_ = _e4694;
                                                                                            }
                                                                                            let _e4701 = phi_6359_;
                                                                                            let _e4703 = phi_6360_;
                                                                                            phi_6361_ = _e4701;
                                                                                            phi_6362_ = _e4703;
                                                                                        }
                                                                                        let _e4709 = phi_6361_;
                                                                                        let _e4711 = phi_6362_;
                                                                                        phi_6363_ = _e4709;
                                                                                        phi_6365_ = _e4711;
                                                                                    }
                                                                                    let _e4713 = phi_6363_;
                                                                                    let _e4715 = phi_6365_;
                                                                                    phi_6366_ = _e4713;
                                                                                    phi_6367_ = select(_e4126, _e4678, _e4685);
                                                                                    phi_6368_ = _e4715;
                                                                                    phi_6369_ = _e4132;
                                                                                }
                                                                                let _e4722 = phi_6366_;
                                                                                let _e4724 = phi_6367_;
                                                                                let _e4726 = phi_6368_;
                                                                                let _e4728 = phi_6369_;
                                                                                phi_6370_ = _e4722;
                                                                                phi_6371_ = _e4724;
                                                                                phi_6372_ = _e4726;
                                                                                phi_6373_ = _e4728;
                                                                            }
                                                                            let _e4734 = phi_6370_;
                                                                            let _e4736 = phi_6371_;
                                                                            let _e4738 = phi_6372_;
                                                                            let _e4740 = phi_6373_;
                                                                            phi_6374_ = _e4734;
                                                                            phi_6375_ = _e4736;
                                                                            phi_6376_ = _e4738;
                                                                            phi_6377_ = _e4740;
                                                                        }
                                                                        let _e4746 = phi_6374_;
                                                                        let _e4748 = phi_6375_;
                                                                        let _e4750 = phi_6376_;
                                                                        let _e4752 = phi_6377_;
                                                                        phi_6378_ = _e4746;
                                                                        phi_6379_ = _e4748;
                                                                        phi_6380_ = _e4750;
                                                                        phi_6381_ = _e4752;
                                                                    }
                                                                    let _e4754 = phi_6378_;
                                                                    let _e4756 = phi_6379_;
                                                                    let _e4758 = phi_6380_;
                                                                    let _e4760 = phi_6381_;
                                                                    phi_6383_ = _e4754;
                                                                    phi_6384_ = _e4756;
                                                                    phi_6385_ = _e4758;
                                                                    phi_6386_ = _e4760;
                                                                    phi_6387_ = select(_e4134, _e4678, _e4681);
                                                                    phi_6388_ = _e4136;
                                                                }
                                                                let _e4765 = phi_6383_;
                                                                let _e4767 = phi_6384_;
                                                                let _e4769 = phi_6385_;
                                                                let _e4771 = phi_6386_;
                                                                let _e4773 = phi_6387_;
                                                                let _e4775 = phi_6388_;
                                                                phi_6389_ = _e4765;
                                                                phi_6390_ = _e4767;
                                                                phi_6391_ = _e4769;
                                                                phi_6392_ = _e4771;
                                                                phi_6393_ = _e4773;
                                                                phi_6394_ = _e4775;
                                                            }
                                                            let _e4779 = phi_6389_;
                                                            let _e4781 = phi_6390_;
                                                            let _e4783 = phi_6391_;
                                                            let _e4785 = phi_6392_;
                                                            let _e4787 = phi_6393_;
                                                            let _e4789 = phi_6394_;
                                                            phi_5653_ = _e4779;
                                                            phi_5656_ = _e4781;
                                                            phi_5658_ = (_e4128 + 1u);
                                                            phi_5660_ = _e4783;
                                                            phi_5662_ = _e4785;
                                                            phi_5664_ = _e4787;
                                                            phi_5666_ = _e4789;
                                                        } else {
                                                            phi_5653_ = f32();
                                                            phi_5656_ = f32();
                                                            phi_5658_ = u32();
                                                            phi_5660_ = vec3<f32>();
                                                            phi_5662_ = vec3<f32>();
                                                            phi_5664_ = f32();
                                                            phi_5666_ = f32();
                                                        }
                                                        let _e4792 = phi_5653_;
                                                        let _e4794 = phi_5656_;
                                                        let _e4796 = phi_5658_;
                                                        let _e4798 = phi_5660_;
                                                        let _e4800 = phi_5662_;
                                                        let _e4802 = phi_5664_;
                                                        let _e4804 = phi_5666_;
                                                        phi_5652_ = _e4792;
                                                        phi_5655_ = _e4794;
                                                        phi_5657_ = _e4796;
                                                        phi_5659_ = _e4798;
                                                        phi_5661_ = _e4800;
                                                        phi_5663_ = _e4802;
                                                        phi_5665_ = _e4804;
                                                        if !(_e4137) {
                                                            break;
                                                        }
                                                    }
                                                    if _e4101 {
                                                        let _e4819 = (exp((-(abs(_e3828)) / (0.06f + (abs(_e3839) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3839 * -0.71428573f) * (_e3839 * 0.71428573f))))));
                                                        let _e4821 = (_e3828 * 1.3333334f);
                                                        let _e4826 = (1.3f * exp((((_e3839 * _e3839) + (_e4821 * _e4821)) * -12.499999f)));
                                                        let _e4837 = local_26;
                                                        let _e4839 = local_27;
                                                        if (_e4837 != _e4839) {
                                                            phi_24060_ = true;
                                                        } else {
                                                            let _e4842 = local_28;
                                                            phi_24060_ = (0.1f >= _e4842);
                                                        }
                                                        let _e4845 = phi_24060_;
                                                        let _e4847 = local_29;
                                                        let _e4850 = (pow(select(_e4847, 0.1f, _e4845), 1.3f) * _e4059);
                                                        phi_6426_ = vec3<f32>((_e4056.x + (((_e4065 * _e4819) + (_e4074 * _e4826)) * _e4850)), (_e4056.y + (((_e4068 * _e4819) + (_e4077 * _e4826)) * _e4850)), (_e4056.z + (((_e4071 * _e4819) + (_e4080 * _e4826)) * _e4850)));
                                                    } else {
                                                        phi_6426_ = _e4056;
                                                    }
                                                    let _e4862 = phi_6426_;
                                                    if select(_e4111, _e4115, _e4117) {
                                                        let _e4864 = local_30;
                                                        let _e4870 = ((_e3828 - (0.025f * _e4864)) / (0.045f + (abs(_e3839) * 0.0095492955f)));
                                                        let _e4875 = local_31;
                                                        let _e4876 = (_e4875 * 3.3333333f);
                                                        let _e4878 = select(_e4876, 0f, (_e4876 < 0f));
                                                        let _e4880 = select(_e4878, 1f, (_e4878 > 1f));
                                                        let _e4896 = local_32;
                                                        let _e4898 = ((_e4896 - 0.25f) * 5.0000005f);
                                                        let _e4900 = select(_e4898, 0f, (_e4898 < 0f));
                                                        let _e4902 = select(_e4900, 1f, (_e4900 > 1f));
                                                        phi_6505_ = (_e4062 * (((exp((-(_e4870) * _e4870)) * (0.15f + (3f * ((_e4880 * _e4880) * (3f - (2f * _e4880)))))) * (0.5f + (0.9f * exp(((_e3839 * -0.6666667f) * (_e3839 * 0.6666667f)))))) + (1.2f * (exp(((_e3828 * -3.3333333f) * (_e3828 * 3.3333333f))) * ((_e4902 * _e4902) * (3f - (2f * _e4902)))))));
                                                    } else {
                                                        phi_6505_ = 0f;
                                                    }
                                                    let _e4916 = phi_6505_;
                                                    phi_6506_ = _e4862;
                                                    phi_6507_ = _e4916;
                                                } else {
                                                    phi_6506_ = _e4056;
                                                    phi_6507_ = 0f;
                                                }
                                                let _e4918 = phi_6506_;
                                                let _e4920 = phi_6507_;
                                                phi_6509_ = _e4918;
                                                phi_6512_ = 0u;
                                                loop {
                                                    let _e4922 = phi_6509_;
                                                    let _e4924 = phi_6512_;
                                                    local_49 = _e4922;
                                                    local_50 = _e4922;
                                                    local_51 = _e4922;
                                                    let _e4925 = (_e4924 < bitcast<u32>(_e3791));
                                                    if _e4925 {
                                                        let _e4927 = (bitcast<u32>(_e3787) + (_e4924 * 29u));
                                                        let _e4929 = global_5.member[_e4927];
                                                        let _e4930 = (_e4929 > 0.5f);
                                                        let _e4933 = global_5.member[(_e4927 + 1u)];
                                                        let _e4936 = global_5.member[(_e4927 + 2u)];
                                                        let _e4939 = global_5.member[(_e4927 + 3u)];
                                                        let _e4942 = global_5.member[(_e4927 + 4u)];
                                                        let _e4945 = global_5.member[(_e4927 + 5u)];
                                                        let _e4948 = global_5.member[(_e4927 + 6u)];
                                                        let _e4953 = (((_e321.member_1.x * _e4933) + (_e321.member_1.y * _e4936)) + (_e321.member_1.z * _e4939));
                                                        let _e4954 = (_e4953 < _e4945);
                                                        if _e4954 {
                                                            phi_7658_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_7659_ = vec3<f32>();
                                                        } else {
                                                            let _e4957 = global_5.member[(_e4927 + 7u)];
                                                            let _e4960 = global_5.member[(_e4927 + 8u)];
                                                            let _e4963 = global_5.member[(_e4927 + 9u)];
                                                            let _e4966 = global_5.member[(_e4927 + 10u)];
                                                            let _e4969 = global_5.member[(_e4927 + 11u)];
                                                            let _e4972 = global_5.member[(_e4927 + 12u)];
                                                            let _e4975 = global_5.member[(_e4927 + 13u)];
                                                            let _e4978 = global_5.member[(_e4927 + 14u)];
                                                            let _e4981 = global_5.member[(_e4927 + 15u)];
                                                            let _e4984 = global_5.member[(_e4927 + 16u)];
                                                            let _e4987 = global_5.member[(_e4927 + 17u)];
                                                            let _e4990 = global_5.member[(_e4927 + 18u)];
                                                            let _e4993 = global_5.member[(_e4927 + 19u)];
                                                            let _e4996 = global_5.member[(_e4927 + 20u)];
                                                            let _e4999 = global_5.member[(_e4927 + 21u)];
                                                            let _e5002 = global_5.member[(_e4927 + 22u)];
                                                            let _e5005 = global_5.member[(_e4927 + 23u)];
                                                            let _e5008 = global_5.member[(_e4927 + 24u)];
                                                            let _e5011 = global_5.member[(_e4927 + 25u)];
                                                            let _e5014 = global_5.member[(_e4927 + 26u)];
                                                            let _e5017 = global_5.member[(_e4927 + 27u)];
                                                            let _e5020 = global_5.member[(_e4927 + 28u)];
                                                            let _e5027 = (((_e321.member_1.x / _e4953) - _e4933) / _e4942);
                                                            let _e5028 = (((_e321.member_1.y / _e4953) - _e4936) / _e4942);
                                                            let _e5029 = (((_e321.member_1.z / _e4953) - _e4939) / _e4942);
                                                            if _e4930 {
                                                                let _e5634 = (((_e5027 * _e4984) + (_e5028 * _e4987)) + (_e5029 * _e4990));
                                                                let _e5640 = ((((_e5027 * _e4993) + (_e5028 * _e4996)) + (_e5029 * _e4999)) / _e5002);
                                                                phi_7656_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4948 * exp((-2.5f * pow((sqrt(((_e5634 * _e5634) + (_e5640 * _e5640))) * 3.3333333f), 0.7f)))));
                                                                phi_7657_ = vec3<f32>();
                                                            } else {
                                                                let _e5034 = (((_e4957 * _e4933) + (_e4960 * _e4936)) + (_e4963 * _e4939));
                                                                if (abs(_e5034) < 0.12f) {
                                                                    phi_6960_ = select(0.12f, -0.12f, (_e5034 < 0f));
                                                                } else {
                                                                    phi_6960_ = _e5034;
                                                                }
                                                                let _e5040 = phi_6960_;
                                                                let _e5046 = ((((_e4957 * _e5027) + (_e4960 * _e5028)) + (_e4963 * _e5029)) / _e5040);
                                                                let _e5050 = (_e5027 - (_e4933 * _e5046));
                                                                let _e5051 = (_e5028 - (_e4936 * _e5046));
                                                                let _e5052 = (_e5029 - (_e4939 * _e5046));
                                                                let _e5057 = (((_e5050 * _e4966) + (_e5051 * _e4969)) + (_e5052 * _e4972));
                                                                let _e5062 = (((_e5050 * _e4975) + (_e5051 * _e4978)) + (_e5052 * _e4981));
                                                                let _e5066 = sqrt(((_e5057 * _e5057) + (_e5062 * _e5062)));
                                                                let _e5078 = abs(_e5040);
                                                                let _e5080 = (pow((0.5f + (0.5f * cos(((_e5005 * (atan2(_e5062, _e5057) - (log((_e5066 + 0.03f)) / _e5008))) + _e5011)))), 3f) * sqrt(_e5078));
                                                                let _e5085 = global_5.member[_e3768];
                                                                let _e5086 = bitcast<u32>(_e5085);
                                                                let _e5089 = global_5.member[(_e1279 + 11u)];
                                                                let _e5092 = global_5.member[(_e1279 + 12u)];
                                                                let _e5095 = global_5.member[(_e1279 + 13u)];
                                                                let _e5098 = global_5.member[(_e1279 + 14u)];
                                                                let _e5101 = global_5.member[(_e1279 + 15u)];
                                                                let _e5104 = global_5.member[(_e1279 + 16u)];
                                                                let _e5107 = global_5.member[(_e1279 + 17u)];
                                                                let _e5110 = global_5.member[(_e1279 + 18u)];
                                                                let _e5113 = global_5.member[(_e1279 + 19u)];
                                                                phi_7140_ = 0u;
                                                                phi_7143_ = 0f;
                                                                phi_7145_ = 1f;
                                                                phi_7147_ = 0f;
                                                                phi_7149_ = vec3<f32>((_e5057 * 9f), (_e5062 * 9f), 0f);
                                                                loop {
                                                                    let _e5115 = phi_7140_;
                                                                    let _e5117 = phi_7143_;
                                                                    let _e5119 = phi_7145_;
                                                                    let _e5121 = phi_7147_;
                                                                    let _e5123 = phi_7149_;
                                                                    local_33 = _e5117;
                                                                    local_34 = _e5121;
                                                                    local_35 = _e5117;
                                                                    let _e5124 = (_e5115 < 4u);
                                                                    if _e5124 {
                                                                        let _e5126 = (_e5123.x + _e5014);
                                                                        let _e5128 = (_e5123.y + _e5017);
                                                                        let _e5130 = (_e5123.z + _e5020);
                                                                        let _e5131 = floor(_e5126);
                                                                        let _e5132 = floor(_e5128);
                                                                        let _e5133 = floor(_e5130);
                                                                        let _e5142 = bitcast<u32>((select(0i, select(select(i32(_e5131), i32(-2147483648), (_e5131 < -2147483600f)), 2147483647i, (_e5131 > 2147483500f)), (_e5131 == _e5131)) & 255i));
                                                                        let _e5151 = bitcast<u32>((select(0i, select(select(i32(_e5132), i32(-2147483648), (_e5132 < -2147483600f)), 2147483647i, (_e5132 > 2147483500f)), (_e5132 == _e5132)) & 255i));
                                                                        let _e5160 = bitcast<u32>((select(0i, select(select(i32(_e5133), i32(-2147483648), (_e5133 < -2147483600f)), 2147483647i, (_e5133 > 2147483500f)), (_e5133 == _e5133)) & 255i));
                                                                        let _e5161 = (_e5126 - _e5131);
                                                                        let _e5162 = (_e5128 - _e5132);
                                                                        let _e5163 = (_e5130 - _e5133);
                                                                        let _e5170 = (((_e5161 * _e5161) * _e5161) * ((_e5161 * ((_e5161 * 6f) - 15f)) + 10f));
                                                                        let _e5177 = (((_e5162 * _e5162) * _e5162) * ((_e5162 * ((_e5162 * 6f) - 15f)) + 10f));
                                                                        let _e5187 = global_5.member[(_e5086 + _e5142)];
                                                                        let _e5193 = (select(select(u32(_e5187), 0u, (_e5187 < 0f)), 4294967295u, (_e5187 > 4294967000f)) + _e5151);
                                                                        let _e5196 = global_5.member[(_e5086 + _e5193)];
                                                                        let _e5202 = (select(select(u32(_e5196), 0u, (_e5196 < 0f)), 4294967295u, (_e5196 > 4294967000f)) + _e5160);
                                                                        let _e5206 = global_5.member[(_e5086 + (_e5193 + 1u))];
                                                                        let _e5212 = (select(select(u32(_e5206), 0u, (_e5206 < 0f)), 4294967295u, (_e5206 > 4294967000f)) + _e5160);
                                                                        let _e5216 = global_5.member[(_e5086 + (_e5142 + 1u))];
                                                                        let _e5222 = (select(select(u32(_e5216), 0u, (_e5216 < 0f)), 4294967295u, (_e5216 > 4294967000f)) + _e5151);
                                                                        let _e5225 = global_5.member[(_e5086 + _e5222)];
                                                                        let _e5231 = (select(select(u32(_e5225), 0u, (_e5225 < 0f)), 4294967295u, (_e5225 > 4294967000f)) + _e5160);
                                                                        let _e5235 = global_5.member[(_e5086 + (_e5222 + 1u))];
                                                                        let _e5241 = (select(select(u32(_e5235), 0u, (_e5235 < 0f)), 4294967295u, (_e5235 > 4294967000f)) + _e5160);
                                                                        let _e5244 = global_5.member[(_e5086 + _e5202)];
                                                                        let _e5250 = (select(select(u32(_e5244), 0u, (_e5244 < 0f)), 4294967295u, (_e5244 > 4294967000f)) % 12u);
                                                                        let _e5253 = select(-1f, 1f, ((_e5250 & 1u) == 0u));
                                                                        let _e5256 = select(-1f, 1f, ((_e5250 & 2u) == 0u));
                                                                        if (_e5250 < 4u) {
                                                                            phi_24155_ = ((_e5253 * _e5161) + (_e5256 * _e5162));
                                                                        } else {
                                                                            if (_e5250 < 8u) {
                                                                                phi_24149_ = ((_e5253 * _e5161) + (_e5256 * _e5163));
                                                                            } else {
                                                                                phi_24149_ = ((_e5253 * _e5162) + (_e5256 * _e5163));
                                                                            }
                                                                            let _e5266 = phi_24149_;
                                                                            phi_24155_ = _e5266;
                                                                        }
                                                                        let _e5271 = phi_24155_;
                                                                        let _e5274 = global_5.member[(_e5086 + _e5231)];
                                                                        let _e5280 = (_e5161 - 1f);
                                                                        let _e5281 = (select(select(u32(_e5274), 0u, (_e5274 < 0f)), 4294967295u, (_e5274 > 4294967000f)) % 12u);
                                                                        let _e5284 = select(-1f, 1f, ((_e5281 & 1u) == 0u));
                                                                        let _e5287 = select(-1f, 1f, ((_e5281 & 2u) == 0u));
                                                                        if (_e5281 < 4u) {
                                                                            phi_24189_ = ((_e5284 * _e5280) + (_e5287 * _e5162));
                                                                        } else {
                                                                            if (_e5281 < 8u) {
                                                                                phi_24183_ = ((_e5284 * _e5280) + (_e5287 * _e5163));
                                                                            } else {
                                                                                phi_24183_ = ((_e5284 * _e5162) + (_e5287 * _e5163));
                                                                            }
                                                                            let _e5297 = phi_24183_;
                                                                            phi_24189_ = _e5297;
                                                                        }
                                                                        let _e5302 = phi_24189_;
                                                                        let _e5305 = global_5.member[(_e5086 + _e5212)];
                                                                        let _e5311 = (_e5162 - 1f);
                                                                        let _e5312 = (select(select(u32(_e5305), 0u, (_e5305 < 0f)), 4294967295u, (_e5305 > 4294967000f)) % 12u);
                                                                        let _e5315 = select(-1f, 1f, ((_e5312 & 1u) == 0u));
                                                                        let _e5318 = select(-1f, 1f, ((_e5312 & 2u) == 0u));
                                                                        if (_e5312 < 4u) {
                                                                            phi_24223_ = ((_e5315 * _e5161) + (_e5318 * _e5311));
                                                                        } else {
                                                                            if (_e5312 < 8u) {
                                                                                phi_24217_ = ((_e5315 * _e5161) + (_e5318 * _e5163));
                                                                            } else {
                                                                                phi_24217_ = ((_e5315 * _e5311) + (_e5318 * _e5163));
                                                                            }
                                                                            let _e5328 = phi_24217_;
                                                                            phi_24223_ = _e5328;
                                                                        }
                                                                        let _e5333 = phi_24223_;
                                                                        let _e5336 = global_5.member[(_e5086 + _e5241)];
                                                                        let _e5342 = (select(select(u32(_e5336), 0u, (_e5336 < 0f)), 4294967295u, (_e5336 > 4294967000f)) % 12u);
                                                                        let _e5345 = select(-1f, 1f, ((_e5342 & 1u) == 0u));
                                                                        let _e5348 = select(-1f, 1f, ((_e5342 & 2u) == 0u));
                                                                        if (_e5342 < 4u) {
                                                                            phi_24257_ = ((_e5345 * _e5280) + (_e5348 * _e5311));
                                                                        } else {
                                                                            if (_e5342 < 8u) {
                                                                                phi_24251_ = ((_e5345 * _e5280) + (_e5348 * _e5163));
                                                                            } else {
                                                                                phi_24251_ = ((_e5345 * _e5311) + (_e5348 * _e5163));
                                                                            }
                                                                            let _e5358 = phi_24251_;
                                                                            phi_24257_ = _e5358;
                                                                        }
                                                                        let _e5363 = phi_24257_;
                                                                        let _e5367 = global_5.member[(_e5086 + (_e5202 + 1u))];
                                                                        let _e5373 = (_e5163 - 1f);
                                                                        let _e5374 = (select(select(u32(_e5367), 0u, (_e5367 < 0f)), 4294967295u, (_e5367 > 4294967000f)) % 12u);
                                                                        let _e5377 = select(-1f, 1f, ((_e5374 & 1u) == 0u));
                                                                        let _e5380 = select(-1f, 1f, ((_e5374 & 2u) == 0u));
                                                                        if (_e5374 < 4u) {
                                                                            phi_24291_ = ((_e5377 * _e5161) + (_e5380 * _e5162));
                                                                        } else {
                                                                            if (_e5374 < 8u) {
                                                                                phi_24285_ = ((_e5377 * _e5161) + (_e5380 * _e5373));
                                                                            } else {
                                                                                phi_24285_ = ((_e5377 * _e5162) + (_e5380 * _e5373));
                                                                            }
                                                                            let _e5390 = phi_24285_;
                                                                            phi_24291_ = _e5390;
                                                                        }
                                                                        let _e5395 = phi_24291_;
                                                                        let _e5399 = global_5.member[(_e5086 + (_e5231 + 1u))];
                                                                        let _e5405 = (select(select(u32(_e5399), 0u, (_e5399 < 0f)), 4294967295u, (_e5399 > 4294967000f)) % 12u);
                                                                        let _e5408 = select(-1f, 1f, ((_e5405 & 1u) == 0u));
                                                                        let _e5411 = select(-1f, 1f, ((_e5405 & 2u) == 0u));
                                                                        if (_e5405 < 4u) {
                                                                            phi_24325_ = ((_e5408 * _e5280) + (_e5411 * _e5162));
                                                                        } else {
                                                                            if (_e5405 < 8u) {
                                                                                phi_24319_ = ((_e5408 * _e5280) + (_e5411 * _e5373));
                                                                            } else {
                                                                                phi_24319_ = ((_e5408 * _e5162) + (_e5411 * _e5373));
                                                                            }
                                                                            let _e5421 = phi_24319_;
                                                                            phi_24325_ = _e5421;
                                                                        }
                                                                        let _e5426 = phi_24325_;
                                                                        let _e5430 = global_5.member[(_e5086 + (_e5212 + 1u))];
                                                                        let _e5436 = (select(select(u32(_e5430), 0u, (_e5430 < 0f)), 4294967295u, (_e5430 > 4294967000f)) % 12u);
                                                                        let _e5439 = select(-1f, 1f, ((_e5436 & 1u) == 0u));
                                                                        let _e5442 = select(-1f, 1f, ((_e5436 & 2u) == 0u));
                                                                        if (_e5436 < 4u) {
                                                                            phi_24359_ = ((_e5439 * _e5161) + (_e5442 * _e5311));
                                                                        } else {
                                                                            if (_e5436 < 8u) {
                                                                                phi_24353_ = ((_e5439 * _e5161) + (_e5442 * _e5373));
                                                                            } else {
                                                                                phi_24353_ = ((_e5439 * _e5311) + (_e5442 * _e5373));
                                                                            }
                                                                            let _e5452 = phi_24353_;
                                                                            phi_24359_ = _e5452;
                                                                        }
                                                                        let _e5457 = phi_24359_;
                                                                        let _e5461 = global_5.member[(_e5086 + (_e5241 + 1u))];
                                                                        let _e5467 = (select(select(u32(_e5461), 0u, (_e5461 < 0f)), 4294967295u, (_e5461 > 4294967000f)) % 12u);
                                                                        let _e5470 = select(-1f, 1f, ((_e5467 & 1u) == 0u));
                                                                        let _e5473 = select(-1f, 1f, ((_e5467 & 2u) == 0u));
                                                                        if (_e5467 < 4u) {
                                                                            phi_24393_ = ((_e5470 * _e5280) + (_e5473 * _e5311));
                                                                        } else {
                                                                            if (_e5467 < 8u) {
                                                                                phi_24387_ = ((_e5470 * _e5280) + (_e5473 * _e5373));
                                                                            } else {
                                                                                phi_24387_ = ((_e5470 * _e5311) + (_e5473 * _e5373));
                                                                            }
                                                                            let _e5483 = phi_24387_;
                                                                            phi_24393_ = _e5483;
                                                                        }
                                                                        let _e5488 = phi_24393_;
                                                                        let _e5491 = (_e5271 + (_e5170 * (_e5302 - _e5271)));
                                                                        let _e5497 = (_e5395 + (_e5170 * (_e5426 - _e5395)));
                                                                        let _e5503 = (_e5491 + (_e5177 * ((_e5333 + (_e5170 * (_e5363 - _e5333))) - _e5491)));
                                                                        phi_7141_ = (_e5115 + 1u);
                                                                        phi_7144_ = (_e5117 + _e5119);
                                                                        phi_7146_ = (_e5119 * 0.5f);
                                                                        phi_7148_ = (_e5121 + ((_e5503 + ((((_e5163 * _e5163) * _e5163) * ((_e5163 * ((_e5163 * 6f) - 15f)) + 10f)) * ((_e5497 + (_e5177 * ((_e5457 + (_e5170 * (_e5488 - _e5457))) - _e5497))) - _e5503))) * _e5119));
                                                                        phi_7150_ = vec3<f32>(((((_e5089 * _e5123.x) + (_e5092 * _e5123.y)) + (_e5095 * _e5123.z)) * 2.03f), ((((_e5098 * _e5123.x) + (_e5101 * _e5123.y)) + (_e5104 * _e5123.z)) * 2.03f), ((((_e5107 * _e5123.x) + (_e5110 * _e5123.y)) + (_e5113 * _e5123.z)) * 2.03f));
                                                                    } else {
                                                                        phi_7141_ = u32();
                                                                        phi_7144_ = f32();
                                                                        phi_7146_ = f32();
                                                                        phi_7148_ = f32();
                                                                        phi_7150_ = vec3<f32>();
                                                                    }
                                                                    let _e5535 = phi_7141_;
                                                                    let _e5537 = phi_7144_;
                                                                    let _e5539 = phi_7146_;
                                                                    let _e5541 = phi_7148_;
                                                                    let _e5543 = phi_7150_;
                                                                    phi_7140_ = _e5535;
                                                                    phi_7143_ = _e5537;
                                                                    phi_7145_ = _e5539;
                                                                    phi_7147_ = _e5541;
                                                                    phi_7149_ = _e5543;
                                                                    if !(_e5124) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e5546 = local_33;
                                                                if (_e5546 > 0f) {
                                                                    let _e5549 = local_34;
                                                                    let _e5551 = local_35;
                                                                    phi_7563_ = (_e5549 / _e5551);
                                                                } else {
                                                                    phi_7563_ = 0f;
                                                                }
                                                                let _e5554 = phi_7563_;
                                                                let _e5556 = (1f + (1.2f * _e5554));
                                                                if (_e5556 != _e5556) {
                                                                    phi_24439_ = true;
                                                                } else {
                                                                    phi_24439_ = (0f >= _e5556);
                                                                }
                                                                let _e5560 = phi_24439_;
                                                                let _e5565 = ((_e5066 - 1.15f) * -2.8571432f);
                                                                let _e5567 = select(_e5565, 0f, (_e5565 < 0f));
                                                                let _e5569 = select(_e5567, 1f, (_e5567 > 1f));
                                                                let _e5575 = (1f / _e5078);
                                                                if (_e5575 != _e5575) {
                                                                    phi_24454_ = true;
                                                                } else {
                                                                    phi_24454_ = (4f <= _e5575);
                                                                }
                                                                let _e5579 = phi_24454_;
                                                                let _e5582 = (((exp((_e5066 * -3.5714285f)) * ((_e5569 * _e5569) * (3f - (2f * _e5569)))) * select(_e5575, 4f, _e5579)) * 0.12f);
                                                                let _e5588 = sqrt((((_e5027 * _e5027) + (_e5028 * _e5028)) + (_e5029 * _e5029)));
                                                                let _e5595 = (((_e5027 * _e4993) + (_e5028 * _e4996)) + (_e5029 * _e4999));
                                                                let _e5596 = (1f - _e5078);
                                                                let _e5612 = ((_e5582 * (0.6f + (0.4f * (1f - _e5080)))) + exp((_e5588 * -16.666668f)));
                                                                let _e5618 = (((_e5582 * _e5080) * 1.8f) * select(_e5556, 0f, _e5560));
                                                                let _e5625 = (_e4948 * (1f - ((((0.75f * _e5596) * _e5596) * exp(((_e5595 * -28.571428f) * (_e5595 * 28.571428f)))) * exp((_e5588 * -1.4285715f)))));
                                                                phi_7656_ = vec3<f32>();
                                                                phi_7657_ = vec3<f32>((((1.1633643f * _e5612) + (0.86689025f * _e5618)) * _e5625), (((0.97722596f * _e5612) + (1.0155001f * _e5618)) * _e5625), (((0.74455315f * _e5612) + (1.2384146f * _e5618)) * _e5625));
                                                            }
                                                            let _e5652 = phi_7656_;
                                                            let _e5654 = phi_7657_;
                                                            phi_7658_ = _e5652;
                                                            phi_7659_ = _e5654;
                                                        }
                                                        let _e5656 = phi_7658_;
                                                        let _e5658 = phi_7659_;
                                                        phi_6510_ = (_e4922 + select(_e5658, _e5656, vec3(select(_e4930, true, _e4954))));
                                                        phi_6513_ = (_e4924 + 1u);
                                                    } else {
                                                        phi_6510_ = vec3<f32>();
                                                        phi_6513_ = u32();
                                                    }
                                                    let _e5665 = phi_6510_;
                                                    let _e5667 = phi_6513_;
                                                    phi_6509_ = _e5665;
                                                    phi_6512_ = _e5667;
                                                    if !(_e4925) {
                                                        break;
                                                    }
                                                }
                                                phi_32203_ = _e4054;
                                                if _e4054 {
                                                    break;
                                                }
                                                phi_7678_ = vec3<f32>(0f, 0f, 0f);
                                                phi_7681_ = 0u;
                                                phi_7683_ = _e4920;
                                                loop {
                                                    let _e5670 = phi_7678_;
                                                    let _e5672 = phi_7681_;
                                                    let _e5674 = phi_7683_;
                                                    local_45 = _e5674;
                                                    local_46 = _e5674;
                                                    local_47 = _e5674;
                                                    local_48 = _e5674;
                                                    local_52 = _e5670;
                                                    local_53 = _e5670;
                                                    local_54 = _e5670;
                                                    let _e5675 = (_e5672 < bitcast<u32>(_e3783));
                                                    if _e5675 {
                                                        let _e5677 = (bitcast<u32>(_e3779) + (_e5672 * 18u));
                                                        let _e5679 = global_5.member[_e5677];
                                                        let _e5684 = select(select(u32(_e5679), 0u, (_e5679 < 0f)), 4294967295u, (_e5679 > 4294967000f));
                                                        let _e5687 = global_5.member[(_e5677 + 1u)];
                                                        let _e5690 = global_5.member[(_e5677 + 2u)];
                                                        let _e5693 = global_5.member[(_e5677 + 3u)];
                                                        let _e5696 = global_5.member[(_e5677 + 4u)];
                                                        let _e5699 = global_5.member[(_e5677 + 5u)];
                                                        let _e5702 = global_5.member[(_e5677 + 6u)];
                                                        let _e5705 = global_5.member[(_e5677 + 7u)];
                                                        let _e5708 = global_5.member[(_e5677 + 8u)];
                                                        let _e5711 = global_5.member[(_e5677 + 9u)];
                                                        let _e5714 = global_5.member[(_e5677 + 10u)];
                                                        let _e5717 = global_5.member[(_e5677 + 11u)];
                                                        let _e5720 = global_5.member[(_e5677 + 12u)];
                                                        let _e5723 = global_5.member[(_e5677 + 13u)];
                                                        let _e5726 = global_5.member[(_e5677 + 14u)];
                                                        let _e5729 = global_5.member[(_e5677 + 15u)];
                                                        let _e5732 = global_5.member[(_e5677 + 16u)];
                                                        let _e5735 = global_5.member[(_e5677 + 17u)];
                                                        let _e5740 = (((_e321.member_1.x * _e5687) + (_e321.member_1.y * _e5690)) + (_e321.member_1.z * _e5693));
                                                        if (_e5740 < _e5699) {
                                                            phi_9000_ = type_52(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            let _e5745 = ((_e321.member_1.x / _e5740) - _e5687);
                                                            let _e5746 = ((_e321.member_1.y / _e5740) - _e5690);
                                                            let _e5747 = ((_e321.member_1.z / _e5740) - _e5693);
                                                            let _e5754 = ((((_e5745 * _e5708) + (_e5746 * _e5711)) + (_e5747 * _e5714)) / (_e5696 * _e5726));
                                                            let _e5760 = ((((_e5745 * _e5717) + (_e5746 * _e5720)) + (_e5747 * _e5723)) / _e5696);
                                                            let _e5764 = sqrt(((_e5754 * _e5754) + (_e5760 * _e5760)));
                                                            let _e5767 = vec3<f32>((_e5754 * 2.2f), (_e5760 * 2.2f), 0f);
                                                            let _e5768 = (_e5684 == 0u);
                                                            if _e5768 {
                                                                phi_7985_ = 6u;
                                                                phi_7986_ = 1.2f;
                                                                phi_7987_ = _e5767;
                                                            } else {
                                                                let _e5769 = (_e5684 == 1u);
                                                                if _e5769 {
                                                                    phi_7984_ = vec3<f32>((_e5754 * 1.54f), (_e5760 * 1.54f), 0f);
                                                                } else {
                                                                    phi_7984_ = _e5767;
                                                                }
                                                                let _e5774 = phi_7984_;
                                                                phi_7985_ = select(6u, 5u, _e5769);
                                                                phi_7986_ = select(1f, 0.8f, _e5769);
                                                                phi_7987_ = _e5774;
                                                            }
                                                            let _e5778 = phi_7985_;
                                                            let _e5780 = phi_7986_;
                                                            let _e5782 = phi_7987_;
                                                            if _e5768 {
                                                                phi_8004_ = vec3<f32>(((_e5754 * 3.7400002f) + -7f), ((_e5760 * 3.7400002f) + 2f), 5f);
                                                            } else {
                                                                phi_8004_ = vec3<f32>((_e5754 * 3.3000002f), (_e5760 * 3.3000002f), 0f);
                                                            }
                                                            let _e5792 = phi_8004_;
                                                            phi_8006_ = 0u;
                                                            phi_8009_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_8011_ = 0f;
                                                            phi_8013_ = 0f;
                                                            phi_8015_ = 0f;
                                                            loop {
                                                                let _e5795 = phi_8006_;
                                                                let _e5797 = phi_8009_;
                                                                let _e5799 = phi_8011_;
                                                                let _e5801 = phi_8013_;
                                                                let _e5803 = phi_8015_;
                                                                local_39 = _e5803;
                                                                local_40 = _e5801;
                                                                local_41 = _e5799;
                                                                local_42 = _e5801;
                                                                local_43 = _e5801;
                                                                local_44 = _e5799;
                                                                let _e5804 = (_e5795 < 6u);
                                                                if _e5804 {
                                                                    let _e5805 = (_e5795 == 0u);
                                                                    if _e5805 {
                                                                        phi_8057_ = true;
                                                                        phi_8058_ = false;
                                                                        phi_8059_ = false;
                                                                    } else {
                                                                        if (_e5795 <= 4u) {
                                                                            phi_8035_ = (_e5684 == 2u);
                                                                        } else {
                                                                            phi_8035_ = true;
                                                                        }
                                                                        let _e5809 = phi_8035_;
                                                                        if _e5809 {
                                                                            let _e5810 = (_e5795 == 5u);
                                                                            if _e5810 {
                                                                                if _e5768 {
                                                                                    phi_8051_ = false;
                                                                                } else {
                                                                                    phi_8051_ = (_e5684 == 2u);
                                                                                }
                                                                                let _e5813 = phi_8051_;
                                                                                phi_8053_ = _e5813;
                                                                            } else {
                                                                                phi_8053_ = false;
                                                                            }
                                                                            let _e5815 = phi_8053_;
                                                                            phi_8055_ = select(false, _e5768, _e5810);
                                                                            phi_8056_ = _e5815;
                                                                        } else {
                                                                            phi_8055_ = false;
                                                                            phi_8056_ = false;
                                                                        }
                                                                        let _e5818 = phi_8055_;
                                                                        let _e5820 = phi_8056_;
                                                                        phi_8057_ = select(true, false, _e5809);
                                                                        phi_8058_ = _e5818;
                                                                        phi_8059_ = _e5820;
                                                                    }
                                                                    let _e5823 = phi_8057_;
                                                                    let _e5825 = phi_8058_;
                                                                    let _e5827 = phi_8059_;
                                                                    if select(select(_e5827, false, _e5823), true, select(_e5825, true, _e5823)) {
                                                                        if _e5805 {
                                                                            phi_8106_ = 3u;
                                                                            phi_8107_ = vec3<f32>(((_e5754 * 1.7600001f) + 3f), ((_e5760 * 1.7600001f) + 3f), 3f);
                                                                        } else {
                                                                            if (_e5795 < 4u) {
                                                                                let _e5844 = (_e5795 - 1u);
                                                                                if (_e5844 == 0u) {
                                                                                    phi_24473_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                                } else {
                                                                                    phi_24473_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5844 == 1u)));
                                                                                }
                                                                                let _e5850 = phi_24473_;
                                                                                phi_8104_ = 4u;
                                                                                phi_8105_ = (_e5782 + _e5850);
                                                                            } else {
                                                                                let _e5832 = (_e5795 == 4u);
                                                                                if _e5832 {
                                                                                    phi_8103_ = (_e5782 + vec3<f32>((_e5797.x * _e5780), (_e5797.y * _e5780), (_e5797.z * _e5780)));
                                                                                } else {
                                                                                    phi_8103_ = _e5792;
                                                                                }
                                                                                let _e5842 = phi_8103_;
                                                                                phi_8104_ = select(select(4u, 5u, _e5768), _e5778, _e5832);
                                                                                phi_8105_ = _e5842;
                                                                            }
                                                                            let _e5853 = phi_8104_;
                                                                            let _e5855 = phi_8105_;
                                                                            phi_8106_ = _e5853;
                                                                            phi_8107_ = _e5855;
                                                                        }
                                                                        let _e5862 = phi_8106_;
                                                                        let _e5864 = phi_8107_;
                                                                        let _e5866 = global_5.member[_e3768];
                                                                        let _e5867 = bitcast<u32>(_e5866);
                                                                        let _e5870 = global_5.member[(_e1279 + 11u)];
                                                                        let _e5873 = global_5.member[(_e1279 + 12u)];
                                                                        let _e5876 = global_5.member[(_e1279 + 13u)];
                                                                        let _e5879 = global_5.member[(_e1279 + 14u)];
                                                                        let _e5882 = global_5.member[(_e1279 + 15u)];
                                                                        let _e5885 = global_5.member[(_e1279 + 16u)];
                                                                        let _e5888 = global_5.member[(_e1279 + 17u)];
                                                                        let _e5891 = global_5.member[(_e1279 + 18u)];
                                                                        let _e5894 = global_5.member[(_e1279 + 19u)];
                                                                        phi_8242_ = 0u;
                                                                        phi_8245_ = 0f;
                                                                        phi_8247_ = 1f;
                                                                        phi_8249_ = 0f;
                                                                        phi_8251_ = _e5864;
                                                                        loop {
                                                                            let _e5896 = phi_8242_;
                                                                            let _e5898 = phi_8245_;
                                                                            let _e5900 = phi_8247_;
                                                                            let _e5902 = phi_8249_;
                                                                            let _e5904 = phi_8251_;
                                                                            local_36 = _e5898;
                                                                            local_37 = _e5902;
                                                                            local_38 = _e5898;
                                                                            let _e5905 = (_e5896 < _e5862);
                                                                            if _e5905 {
                                                                                let _e5907 = (_e5904.x + _e5729);
                                                                                let _e5909 = (_e5904.y + _e5732);
                                                                                let _e5911 = (_e5904.z + _e5735);
                                                                                let _e5912 = floor(_e5907);
                                                                                let _e5913 = floor(_e5909);
                                                                                let _e5914 = floor(_e5911);
                                                                                let _e5923 = bitcast<u32>((select(0i, select(select(i32(_e5912), i32(-2147483648), (_e5912 < -2147483600f)), 2147483647i, (_e5912 > 2147483500f)), (_e5912 == _e5912)) & 255i));
                                                                                let _e5932 = bitcast<u32>((select(0i, select(select(i32(_e5913), i32(-2147483648), (_e5913 < -2147483600f)), 2147483647i, (_e5913 > 2147483500f)), (_e5913 == _e5913)) & 255i));
                                                                                let _e5941 = bitcast<u32>((select(0i, select(select(i32(_e5914), i32(-2147483648), (_e5914 < -2147483600f)), 2147483647i, (_e5914 > 2147483500f)), (_e5914 == _e5914)) & 255i));
                                                                                let _e5942 = (_e5907 - _e5912);
                                                                                let _e5943 = (_e5909 - _e5913);
                                                                                let _e5944 = (_e5911 - _e5914);
                                                                                let _e5951 = (((_e5942 * _e5942) * _e5942) * ((_e5942 * ((_e5942 * 6f) - 15f)) + 10f));
                                                                                let _e5958 = (((_e5943 * _e5943) * _e5943) * ((_e5943 * ((_e5943 * 6f) - 15f)) + 10f));
                                                                                let _e5968 = global_5.member[(_e5867 + _e5923)];
                                                                                let _e5974 = (select(select(u32(_e5968), 0u, (_e5968 < 0f)), 4294967295u, (_e5968 > 4294967000f)) + _e5932);
                                                                                let _e5977 = global_5.member[(_e5867 + _e5974)];
                                                                                let _e5983 = (select(select(u32(_e5977), 0u, (_e5977 < 0f)), 4294967295u, (_e5977 > 4294967000f)) + _e5941);
                                                                                let _e5987 = global_5.member[(_e5867 + (_e5974 + 1u))];
                                                                                let _e5993 = (select(select(u32(_e5987), 0u, (_e5987 < 0f)), 4294967295u, (_e5987 > 4294967000f)) + _e5941);
                                                                                let _e5997 = global_5.member[(_e5867 + (_e5923 + 1u))];
                                                                                let _e6003 = (select(select(u32(_e5997), 0u, (_e5997 < 0f)), 4294967295u, (_e5997 > 4294967000f)) + _e5932);
                                                                                let _e6006 = global_5.member[(_e5867 + _e6003)];
                                                                                let _e6012 = (select(select(u32(_e6006), 0u, (_e6006 < 0f)), 4294967295u, (_e6006 > 4294967000f)) + _e5941);
                                                                                let _e6016 = global_5.member[(_e5867 + (_e6003 + 1u))];
                                                                                let _e6022 = (select(select(u32(_e6016), 0u, (_e6016 < 0f)), 4294967295u, (_e6016 > 4294967000f)) + _e5941);
                                                                                let _e6025 = global_5.member[(_e5867 + _e5983)];
                                                                                let _e6031 = (select(select(u32(_e6025), 0u, (_e6025 < 0f)), 4294967295u, (_e6025 > 4294967000f)) % 12u);
                                                                                let _e6034 = select(-1f, 1f, ((_e6031 & 1u) == 0u));
                                                                                let _e6037 = select(-1f, 1f, ((_e6031 & 2u) == 0u));
                                                                                if (_e6031 < 4u) {
                                                                                    phi_24534_ = ((_e6034 * _e5942) + (_e6037 * _e5943));
                                                                                } else {
                                                                                    if (_e6031 < 8u) {
                                                                                        phi_24528_ = ((_e6034 * _e5942) + (_e6037 * _e5944));
                                                                                    } else {
                                                                                        phi_24528_ = ((_e6034 * _e5943) + (_e6037 * _e5944));
                                                                                    }
                                                                                    let _e6047 = phi_24528_;
                                                                                    phi_24534_ = _e6047;
                                                                                }
                                                                                let _e6052 = phi_24534_;
                                                                                let _e6055 = global_5.member[(_e5867 + _e6012)];
                                                                                let _e6061 = (_e5942 - 1f);
                                                                                let _e6062 = (select(select(u32(_e6055), 0u, (_e6055 < 0f)), 4294967295u, (_e6055 > 4294967000f)) % 12u);
                                                                                let _e6065 = select(-1f, 1f, ((_e6062 & 1u) == 0u));
                                                                                let _e6068 = select(-1f, 1f, ((_e6062 & 2u) == 0u));
                                                                                if (_e6062 < 4u) {
                                                                                    phi_24568_ = ((_e6065 * _e6061) + (_e6068 * _e5943));
                                                                                } else {
                                                                                    if (_e6062 < 8u) {
                                                                                        phi_24562_ = ((_e6065 * _e6061) + (_e6068 * _e5944));
                                                                                    } else {
                                                                                        phi_24562_ = ((_e6065 * _e5943) + (_e6068 * _e5944));
                                                                                    }
                                                                                    let _e6078 = phi_24562_;
                                                                                    phi_24568_ = _e6078;
                                                                                }
                                                                                let _e6083 = phi_24568_;
                                                                                let _e6086 = global_5.member[(_e5867 + _e5993)];
                                                                                let _e6092 = (_e5943 - 1f);
                                                                                let _e6093 = (select(select(u32(_e6086), 0u, (_e6086 < 0f)), 4294967295u, (_e6086 > 4294967000f)) % 12u);
                                                                                let _e6096 = select(-1f, 1f, ((_e6093 & 1u) == 0u));
                                                                                let _e6099 = select(-1f, 1f, ((_e6093 & 2u) == 0u));
                                                                                if (_e6093 < 4u) {
                                                                                    phi_24602_ = ((_e6096 * _e5942) + (_e6099 * _e6092));
                                                                                } else {
                                                                                    if (_e6093 < 8u) {
                                                                                        phi_24596_ = ((_e6096 * _e5942) + (_e6099 * _e5944));
                                                                                    } else {
                                                                                        phi_24596_ = ((_e6096 * _e6092) + (_e6099 * _e5944));
                                                                                    }
                                                                                    let _e6109 = phi_24596_;
                                                                                    phi_24602_ = _e6109;
                                                                                }
                                                                                let _e6114 = phi_24602_;
                                                                                let _e6117 = global_5.member[(_e5867 + _e6022)];
                                                                                let _e6123 = (select(select(u32(_e6117), 0u, (_e6117 < 0f)), 4294967295u, (_e6117 > 4294967000f)) % 12u);
                                                                                let _e6126 = select(-1f, 1f, ((_e6123 & 1u) == 0u));
                                                                                let _e6129 = select(-1f, 1f, ((_e6123 & 2u) == 0u));
                                                                                if (_e6123 < 4u) {
                                                                                    phi_24636_ = ((_e6126 * _e6061) + (_e6129 * _e6092));
                                                                                } else {
                                                                                    if (_e6123 < 8u) {
                                                                                        phi_24630_ = ((_e6126 * _e6061) + (_e6129 * _e5944));
                                                                                    } else {
                                                                                        phi_24630_ = ((_e6126 * _e6092) + (_e6129 * _e5944));
                                                                                    }
                                                                                    let _e6139 = phi_24630_;
                                                                                    phi_24636_ = _e6139;
                                                                                }
                                                                                let _e6144 = phi_24636_;
                                                                                let _e6148 = global_5.member[(_e5867 + (_e5983 + 1u))];
                                                                                let _e6154 = (_e5944 - 1f);
                                                                                let _e6155 = (select(select(u32(_e6148), 0u, (_e6148 < 0f)), 4294967295u, (_e6148 > 4294967000f)) % 12u);
                                                                                let _e6158 = select(-1f, 1f, ((_e6155 & 1u) == 0u));
                                                                                let _e6161 = select(-1f, 1f, ((_e6155 & 2u) == 0u));
                                                                                if (_e6155 < 4u) {
                                                                                    phi_24670_ = ((_e6158 * _e5942) + (_e6161 * _e5943));
                                                                                } else {
                                                                                    if (_e6155 < 8u) {
                                                                                        phi_24664_ = ((_e6158 * _e5942) + (_e6161 * _e6154));
                                                                                    } else {
                                                                                        phi_24664_ = ((_e6158 * _e5943) + (_e6161 * _e6154));
                                                                                    }
                                                                                    let _e6171 = phi_24664_;
                                                                                    phi_24670_ = _e6171;
                                                                                }
                                                                                let _e6176 = phi_24670_;
                                                                                let _e6180 = global_5.member[(_e5867 + (_e6012 + 1u))];
                                                                                let _e6186 = (select(select(u32(_e6180), 0u, (_e6180 < 0f)), 4294967295u, (_e6180 > 4294967000f)) % 12u);
                                                                                let _e6189 = select(-1f, 1f, ((_e6186 & 1u) == 0u));
                                                                                let _e6192 = select(-1f, 1f, ((_e6186 & 2u) == 0u));
                                                                                if (_e6186 < 4u) {
                                                                                    phi_24704_ = ((_e6189 * _e6061) + (_e6192 * _e5943));
                                                                                } else {
                                                                                    if (_e6186 < 8u) {
                                                                                        phi_24698_ = ((_e6189 * _e6061) + (_e6192 * _e6154));
                                                                                    } else {
                                                                                        phi_24698_ = ((_e6189 * _e5943) + (_e6192 * _e6154));
                                                                                    }
                                                                                    let _e6202 = phi_24698_;
                                                                                    phi_24704_ = _e6202;
                                                                                }
                                                                                let _e6207 = phi_24704_;
                                                                                let _e6211 = global_5.member[(_e5867 + (_e5993 + 1u))];
                                                                                let _e6217 = (select(select(u32(_e6211), 0u, (_e6211 < 0f)), 4294967295u, (_e6211 > 4294967000f)) % 12u);
                                                                                let _e6220 = select(-1f, 1f, ((_e6217 & 1u) == 0u));
                                                                                let _e6223 = select(-1f, 1f, ((_e6217 & 2u) == 0u));
                                                                                if (_e6217 < 4u) {
                                                                                    phi_24738_ = ((_e6220 * _e5942) + (_e6223 * _e6092));
                                                                                } else {
                                                                                    if (_e6217 < 8u) {
                                                                                        phi_24732_ = ((_e6220 * _e5942) + (_e6223 * _e6154));
                                                                                    } else {
                                                                                        phi_24732_ = ((_e6220 * _e6092) + (_e6223 * _e6154));
                                                                                    }
                                                                                    let _e6233 = phi_24732_;
                                                                                    phi_24738_ = _e6233;
                                                                                }
                                                                                let _e6238 = phi_24738_;
                                                                                let _e6242 = global_5.member[(_e5867 + (_e6022 + 1u))];
                                                                                let _e6248 = (select(select(u32(_e6242), 0u, (_e6242 < 0f)), 4294967295u, (_e6242 > 4294967000f)) % 12u);
                                                                                let _e6251 = select(-1f, 1f, ((_e6248 & 1u) == 0u));
                                                                                let _e6254 = select(-1f, 1f, ((_e6248 & 2u) == 0u));
                                                                                if (_e6248 < 4u) {
                                                                                    phi_24772_ = ((_e6251 * _e6061) + (_e6254 * _e6092));
                                                                                } else {
                                                                                    if (_e6248 < 8u) {
                                                                                        phi_24766_ = ((_e6251 * _e6061) + (_e6254 * _e6154));
                                                                                    } else {
                                                                                        phi_24766_ = ((_e6251 * _e6092) + (_e6254 * _e6154));
                                                                                    }
                                                                                    let _e6264 = phi_24766_;
                                                                                    phi_24772_ = _e6264;
                                                                                }
                                                                                let _e6269 = phi_24772_;
                                                                                let _e6272 = (_e6052 + (_e5951 * (_e6083 - _e6052)));
                                                                                let _e6278 = (_e6176 + (_e5951 * (_e6207 - _e6176)));
                                                                                let _e6284 = (_e6272 + (_e5958 * ((_e6114 + (_e5951 * (_e6144 - _e6114))) - _e6272)));
                                                                                phi_8243_ = (_e5896 + 1u);
                                                                                phi_8246_ = (_e5898 + _e5900);
                                                                                phi_8248_ = (_e5900 * 0.5f);
                                                                                phi_8250_ = (_e5902 + ((_e6284 + ((((_e5944 * _e5944) * _e5944) * ((_e5944 * ((_e5944 * 6f) - 15f)) + 10f)) * ((_e6278 + (_e5958 * ((_e6238 + (_e5951 * (_e6269 - _e6238))) - _e6278))) - _e6284))) * _e5900));
                                                                                phi_8252_ = vec3<f32>(((((_e5870 * _e5904.x) + (_e5873 * _e5904.y)) + (_e5876 * _e5904.z)) * 2.03f), ((((_e5879 * _e5904.x) + (_e5882 * _e5904.y)) + (_e5885 * _e5904.z)) * 2.03f), ((((_e5888 * _e5904.x) + (_e5891 * _e5904.y)) + (_e5894 * _e5904.z)) * 2.03f));
                                                                            } else {
                                                                                phi_8243_ = u32();
                                                                                phi_8246_ = f32();
                                                                                phi_8248_ = f32();
                                                                                phi_8250_ = f32();
                                                                                phi_8252_ = vec3<f32>();
                                                                            }
                                                                            let _e6316 = phi_8243_;
                                                                            let _e6318 = phi_8246_;
                                                                            let _e6320 = phi_8248_;
                                                                            let _e6322 = phi_8250_;
                                                                            let _e6324 = phi_8252_;
                                                                            phi_8242_ = _e6316;
                                                                            phi_8245_ = _e6318;
                                                                            phi_8247_ = _e6320;
                                                                            phi_8249_ = _e6322;
                                                                            phi_8251_ = _e6324;
                                                                            if !(_e5905) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e6327 = local_36;
                                                                        if (_e6327 > 0f) {
                                                                            let _e6330 = local_37;
                                                                            let _e6332 = local_38;
                                                                            phi_8665_ = (_e6330 / _e6332);
                                                                        } else {
                                                                            phi_8665_ = 0f;
                                                                        }
                                                                        let _e6335 = phi_8665_;
                                                                        if _e5805 {
                                                                            phi_8700_ = _e5797;
                                                                            phi_8701_ = _e5799;
                                                                            phi_8702_ = _e5801;
                                                                        } else {
                                                                            if (_e5795 == 1u) {
                                                                                phi_8697_ = vec3<f32>(_e6335, _e5797.y, _e5797.z);
                                                                                phi_8698_ = _e5799;
                                                                                phi_8699_ = _e5801;
                                                                            } else {
                                                                                if (_e5795 == 2u) {
                                                                                    phi_8694_ = vec3<f32>(_e5797.x, _e6335, _e5797.z);
                                                                                    phi_8695_ = _e5799;
                                                                                    phi_8696_ = _e5801;
                                                                                } else {
                                                                                    if (_e5795 == 3u) {
                                                                                        phi_8691_ = vec3<f32>(_e5797.x, _e5797.y, _e6335);
                                                                                        phi_8692_ = _e5799;
                                                                                        phi_8693_ = _e5801;
                                                                                    } else {
                                                                                        let _e6339 = (_e5795 == 4u);
                                                                                        phi_8691_ = _e5797;
                                                                                        phi_8692_ = select(_e6335, _e5799, _e6339);
                                                                                        phi_8693_ = select(_e5801, _e6335, _e6339);
                                                                                    }
                                                                                    let _e6347 = phi_8691_;
                                                                                    let _e6349 = phi_8692_;
                                                                                    let _e6351 = phi_8693_;
                                                                                    phi_8694_ = _e6347;
                                                                                    phi_8695_ = _e6349;
                                                                                    phi_8696_ = _e6351;
                                                                                }
                                                                                let _e6357 = phi_8694_;
                                                                                let _e6359 = phi_8695_;
                                                                                let _e6361 = phi_8696_;
                                                                                phi_8697_ = _e6357;
                                                                                phi_8698_ = _e6359;
                                                                                phi_8699_ = _e6361;
                                                                            }
                                                                            let _e6367 = phi_8697_;
                                                                            let _e6369 = phi_8698_;
                                                                            let _e6371 = phi_8699_;
                                                                            phi_8700_ = _e6367;
                                                                            phi_8701_ = _e6369;
                                                                            phi_8702_ = _e6371;
                                                                        }
                                                                        let _e6373 = phi_8700_;
                                                                        let _e6375 = phi_8701_;
                                                                        let _e6377 = phi_8702_;
                                                                        phi_8704_ = _e6373;
                                                                        phi_8705_ = _e6375;
                                                                        phi_8706_ = _e6377;
                                                                        phi_8707_ = select(_e5803, _e6335, _e5805);
                                                                    } else {
                                                                        phi_8704_ = _e5797;
                                                                        phi_8705_ = _e5799;
                                                                        phi_8706_ = _e5801;
                                                                        phi_8707_ = _e5803;
                                                                    }
                                                                    let _e6380 = phi_8704_;
                                                                    let _e6382 = phi_8705_;
                                                                    let _e6384 = phi_8706_;
                                                                    let _e6386 = phi_8707_;
                                                                    phi_8007_ = (_e5795 + 1u);
                                                                    phi_8010_ = _e6380;
                                                                    phi_8012_ = _e6382;
                                                                    phi_8014_ = _e6384;
                                                                    phi_8016_ = _e6386;
                                                                } else {
                                                                    phi_8007_ = u32();
                                                                    phi_8010_ = vec3<f32>();
                                                                    phi_8012_ = f32();
                                                                    phi_8014_ = f32();
                                                                    phi_8016_ = f32();
                                                                }
                                                                let _e6389 = phi_8007_;
                                                                let _e6391 = phi_8010_;
                                                                let _e6393 = phi_8012_;
                                                                let _e6395 = phi_8014_;
                                                                let _e6397 = phi_8016_;
                                                                phi_8006_ = _e6389;
                                                                phi_8009_ = _e6391;
                                                                phi_8011_ = _e6393;
                                                                phi_8013_ = _e6395;
                                                                phi_8015_ = _e6397;
                                                                if !(_e5804) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e6400 = local_39;
                                                            let _e6403 = (_e5764 * (1f + (0.45f * _e6400)));
                                                            if _e5768 {
                                                                let _e6506 = local_43;
                                                                let _e6508 = ((_e6506 - -0.25f) * 1.4285715f);
                                                                let _e6510 = select(_e6508, 0f, (_e6508 < 0f));
                                                                let _e6512 = select(_e6510, 1f, (_e6510 > 1f));
                                                                let _e6516 = ((_e6512 * _e6512) * (3f - (2f * _e6512)));
                                                                let _e6518 = ((_e6403 - 1.05f) * -1.1764706f);
                                                                let _e6520 = select(_e6518, 0f, (_e6518 < 0f));
                                                                let _e6522 = select(_e6520, 1f, (_e6520 > 1f));
                                                                let _e6526 = ((_e6522 * _e6522) * (3f - (2f * _e6522)));
                                                                let _e6527 = (_e6516 * _e6526);
                                                                let _e6529 = ((_e6403 - 0.65f) * -1.5384616f);
                                                                let _e6531 = select(_e6529, 0f, (_e6529 < 0f));
                                                                let _e6533 = select(_e6531, 1f, (_e6531 > 1f));
                                                                let _e6538 = (((_e6533 * _e6533) * (3f - (2f * _e6533))) * _e6516);
                                                                let _e6540 = local_44;
                                                                let _e6542 = ((_e6540 - 0.1f) * 2.857143f);
                                                                let _e6544 = select(_e6542, 0f, (_e6542 < 0f));
                                                                let _e6546 = select(_e6544, 1f, (_e6544 > 1f));
                                                                let _e6551 = (((_e6546 * _e6546) * (3f - (2f * _e6546))) * _e6526);
                                                                let _e6556 = ((0.8f * _e6538) * _e6538);
                                                                let _e6565 = (_e5702 * exp((-2f * _e6551)));
                                                                phi_8999_ = type_52(vec3<f32>((((3.1815925f * _e6527) + (0.14634217f * _e6556)) * _e6565), (((0.38179109f * _e6527) + (1.2439084f * _e6556)) * _e6565), (((0.69995034f * _e6527) + (1.0975662f * _e6556)) * _e6565)), (_e5705 * _e6551));
                                                            } else {
                                                                if (_e5684 == 1u) {
                                                                    let _e6476 = local_42;
                                                                    let _e6478 = ((_e6476 - -0.3f) * 1.25f);
                                                                    let _e6480 = select(_e6478, 0f, (_e6478 < 0f));
                                                                    let _e6482 = select(_e6480, 1f, (_e6480 > 1f));
                                                                    let _e6488 = ((_e6403 - 1f) * -1f);
                                                                    let _e6490 = select(_e6488, 0f, (_e6488 < 0f));
                                                                    let _e6492 = select(_e6490, 1f, (_e6490 > 1f));
                                                                    let _e6498 = (((_e6482 * _e6482) * (3f - (2f * _e6482))) * pow(((_e6492 * _e6492) * (3f - (2f * _e6492))), 1.5f));
                                                                    phi_8998_ = type_52((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5702 * _e6498)), (_e5705 * _e6498));
                                                                } else {
                                                                    if (_e5684 == 2u) {
                                                                        let _e6432 = local_41;
                                                                        let _e6435 = (_e5764 * (1f + (0.15f * _e6432)));
                                                                        let _e6436 = (_e6435 - 0.65f);
                                                                        let _e6440 = exp(((_e6436 * -5.5555553f) * (_e6436 * 5.5555553f)));
                                                                        let _e6442 = ((_e6435 - 0.75f) * -1.8181818f);
                                                                        let _e6444 = select(_e6442, 0f, (_e6442 < 0f));
                                                                        let _e6446 = select(_e6444, 1f, (_e6444 > 1f));
                                                                        let _e6450 = ((_e6446 * _e6446) * (3f - (2f * _e6446)));
                                                                        let _e6455 = (exp(((_e5764 * -25f) * (_e5764 * 25f))) * 3f);
                                                                        phi_8997_ = type_52(vec3<f32>(((((3.1815925f * _e6440) + (_e6450 * 0.10243952f)) + _e6455) * _e5702), ((((0.38179109f * _e6440) + (_e6450 * 0.8707359f)) + _e6455) * _e5702), ((((0.69995034f * _e6440) + (_e6450 * 0.76829636f)) + _e6455) * _e5702)), 0f);
                                                                    } else {
                                                                        let _e6407 = local_40;
                                                                        let _e6409 = ((_e6407 - -0.2f) * 1.6666666f);
                                                                        let _e6411 = select(_e6409, 0f, (_e6409 < 0f));
                                                                        let _e6413 = select(_e6411, 1f, (_e6411 > 1f));
                                                                        let _e6419 = ((_e6403 - 1f) * -1.4285715f);
                                                                        let _e6421 = select(_e6419, 0f, (_e6419 < 0f));
                                                                        let _e6423 = select(_e6421, 1f, (_e6421 > 1f));
                                                                        phi_8997_ = type_52(vec3<f32>(0f, 0f, 0f), (_e5705 * (((_e6413 * _e6413) * (3f - (2f * _e6413))) * ((_e6423 * _e6423) * (3f - (2f * _e6423))))));
                                                                    }
                                                                    let _e6474 = phi_8997_;
                                                                    phi_8998_ = _e6474;
                                                                }
                                                                let _e6504 = phi_8998_;
                                                                phi_8999_ = _e6504;
                                                            }
                                                            let _e6573 = phi_8999_;
                                                            phi_9000_ = _e6573;
                                                        }
                                                        let _e6575 = phi_9000_;
                                                        phi_7679_ = (_e5670 + _e6575.member);
                                                        phi_7682_ = (_e5672 + 1u);
                                                        phi_7684_ = (_e5674 + _e6575.member_1);
                                                    } else {
                                                        phi_7679_ = vec3<f32>();
                                                        phi_7682_ = u32();
                                                        phi_7684_ = f32();
                                                    }
                                                    let _e6582 = phi_7679_;
                                                    let _e6584 = phi_7682_;
                                                    let _e6586 = phi_7684_;
                                                    phi_7678_ = _e6582;
                                                    phi_7681_ = _e6584;
                                                    phi_7683_ = _e6586;
                                                    if !(_e5675) {
                                                        break;
                                                    }
                                                }
                                                phi_32203_ = _e4054;
                                                if _e4054 {
                                                    break;
                                                }
                                                let _e6589 = local_45;
                                                if (_e6589 <= 0f) {
                                                    phi_9029_ = vec3<f32>(1f, 1f, 1f);
                                                } else {
                                                    let _e6592 = local_46;
                                                    let _e6595 = local_47;
                                                    let _e6600 = local_48;
                                                    phi_9029_ = vec3<f32>(exp((_e6595 * -0.81f)), exp(-(_e6592)), exp((_e6600 * -1.25f)));
                                                }
                                                let _e6605 = phi_9029_;
                                                let _e6607 = local_49;
                                                let _e6612 = local_50;
                                                let _e6617 = local_51;
                                                let _e6622 = local_52;
                                                let _e6626 = local_53;
                                                let _e6630 = local_54;
                                                let _e6635 = global_5.member[(_e1279 + 9u)];
                                                phi_32212_ = _e4054;
                                                phi_9076_ = vec3<f32>((_e3759.x + ((((_e6607.x * _e6605.x) + _e6622.x) * _e6635) * _e3761.x)), (_e3759.y + ((((_e6612.y * _e6605.y) + _e6626.y) * _e6635) * _e3761.y)), (_e3759.z + ((((_e6617.z * _e6605.z) + _e6630.z) * _e6635) * _e3761.z)));
                                            } else {
                                                phi_32212_ = _e3757;
                                                phi_9076_ = _e3759;
                                            }
                                            let _e6653 = phi_32212_;
                                            let _e6655 = phi_9076_;
                                            phi_32211_ = _e6653;
                                            phi_9077_ = _e6655;
                                        } else {
                                            phi_32211_ = _e3757;
                                            phi_9077_ = _e3765;
                                        }
                                        let _e6657 = phi_32211_;
                                        let _e6659 = phi_9077_;
                                        phi_32210_ = _e6657;
                                        phi_9078_ = _e6659;
                                    } else {
                                        phi_32210_ = _e310;
                                        phi_9078_ = _e1424;
                                    }
                                    let _e6661 = phi_32210_;
                                    let _e6663 = phi_9078_;
                                    phi_32206_ = _e6661;
                                    phi_10351_ = (vec3<f32>(_e321.member_2.x, _e321.member_2.y, _e321.member_2.z) * _e6663);
                                } else {
                                    let _e337 = global_1.member.member_8[1u];
                                    let _e347 = global_5.member[_e337];
                                    let _e350 = global_5.member[(_e337 + 1u)];
                                    let _e353 = global_5.member[(_e337 + 2u)];
                                    let _e356 = global_5.member[(_e337 + 3u)];
                                    let _e359 = global_5.member[(_e337 + 4u)];
                                    let _e362 = global_5.member[(_e337 + 5u)];
                                    let _e365 = global_5.member[(_e337 + 6u)];
                                    let _e368 = global_5.member[(_e337 + 7u)];
                                    let _e371 = global_5.member[(_e337 + 8u)];
                                    let _e374 = global_5.member[(_e337 + 9u)];
                                    let _e377 = global_5.member[(_e337 + 10u)];
                                    let _e380 = global_5.member[(_e337 + 11u)];
                                    let _e383 = global_5.member[(_e337 + 12u)];
                                    let _e386 = global_5.member[(_e337 + 13u)];
                                    let _e389 = global_5.member[(_e337 + 14u)];
                                    let _e392 = global_5.member[(_e337 + 15u)];
                                    let _e396 = global_5.member[(_e337 + 16u)];
                                    let _e399 = global_5.member[(_e337 + 17u)];
                                    let _e402 = global_5.member[(_e337 + 18u)];
                                    let _e405 = global_5.member[(_e337 + 19u)];
                                    let _e408 = global_5.member[(_e337 + 20u)];
                                    let _e411 = global_5.member[(_e337 + 29u)];
                                    let _e412 = bitcast<u32>(_e411);
                                    let _e415 = global_5.member[(_e337 + 30u)];
                                    let _e416 = bitcast<u32>(_e415);
                                    let _e424 = ((((_e321.member.x - _e347) * _e321.member_1.x) + ((_e321.member.y - _e350) * _e321.member_1.y)) + ((_e321.member.z - _e353) * _e321.member_1.z));
                                    let _e436 = ((((_e321.member.x * _e321.member.x) + (_e321.member.y * _e321.member.y)) + (_e321.member.z * _e321.member.z)) - (2f * (((_e321.member.x * _e347) + (_e321.member.y * _e350)) + (_e321.member.z * _e353))));
                                    let _e437 = (_e436 + _e365);
                                    let _e438 = (_e424 * _e424);
                                    let _e439 = (_e438 - _e437);
                                    if (_e439 < 0f) {
                                        phi_20677_ = type_50(0u, type_50().member_1);
                                        phi_20678_ = type_50();
                                        phi_20679_ = true;
                                    } else {
                                        if (_e424 < 0f) {
                                            phi_20649_ = -(sqrt(_e439));
                                        } else {
                                            phi_20649_ = sqrt(_e439);
                                        }
                                        let _e446 = phi_20649_;
                                        let _e448 = -((_e424 + _e446));
                                        let _e449 = (_e448 == 0f);
                                        if _e449 {
                                            phi_20672_ = type_50(1u, type_49(0f, 0f));
                                            phi_20673_ = type_50();
                                        } else {
                                            let _e450 = (_e437 / _e448);
                                            if (_e448 < _e450) {
                                                phi_20661_ = type_49(_e448, _e450);
                                            } else {
                                                phi_20661_ = type_49(_e450, _e448);
                                            }
                                            let _e455 = phi_20661_;
                                            phi_20672_ = type_50();
                                            phi_20673_ = type_50(1u, _e455);
                                        }
                                        let _e458 = phi_20672_;
                                        let _e460 = phi_20673_;
                                        phi_20677_ = _e458;
                                        phi_20678_ = _e460;
                                        phi_20679_ = _e449;
                                    }
                                    let _e465 = phi_20677_;
                                    let _e467 = phi_20678_;
                                    let _e469 = phi_20679_;
                                    if _e469 {
                                        phi_20683_ = _e465;
                                    } else {
                                        phi_20683_ = _e467;
                                    }
                                    let _e471 = phi_20683_;
                                    if (bitcast<i32>(_e471.member) != 0i) {
                                        phi_20570_ = type_50(0u, _e471.member_1);
                                    } else {
                                        phi_20570_ = type_50(1u, type_50().member_1);
                                    }
                                    let _e481 = phi_20570_;
                                    if (bitcast<i32>(_e481.member) != 0i) {
                                        phi_20617_ = type_50(0u, type_50().member_1);
                                    } else {
                                        if (_e481.member_1.member_1 <= 0f) {
                                            phi_20613_ = type_50(0u, type_50().member_1);
                                        } else {
                                            if (_e481.member_1.member != _e481.member_1.member) {
                                                phi_20721_ = true;
                                            } else {
                                                phi_20721_ = (0f >= _e481.member_1.member);
                                            }
                                            let _e493 = phi_20721_;
                                            let _e494 = select(_e481.member_1.member, 0f, _e493);
                                            let _e495 = (_e436 + _e362);
                                            if (_e495 > 0f) {
                                                let _e497 = (_e438 - _e495);
                                                if (_e497 < 0f) {
                                                    phi_20829_ = type_50(0u, type_50().member_1);
                                                    phi_20830_ = type_50();
                                                    phi_20831_ = true;
                                                } else {
                                                    if (_e424 < 0f) {
                                                        phi_20801_ = -(sqrt(_e497));
                                                    } else {
                                                        phi_20801_ = sqrt(_e497);
                                                    }
                                                    let _e504 = phi_20801_;
                                                    let _e506 = -((_e424 + _e504));
                                                    let _e507 = (_e506 == 0f);
                                                    if _e507 {
                                                        phi_20824_ = type_50(1u, type_49(0f, 0f));
                                                        phi_20825_ = type_50();
                                                    } else {
                                                        let _e508 = (_e495 / _e506);
                                                        if (_e506 < _e508) {
                                                            phi_20813_ = type_49(_e506, _e508);
                                                        } else {
                                                            phi_20813_ = type_49(_e508, _e506);
                                                        }
                                                        let _e513 = phi_20813_;
                                                        phi_20824_ = type_50();
                                                        phi_20825_ = type_50(1u, _e513);
                                                    }
                                                    let _e516 = phi_20824_;
                                                    let _e518 = phi_20825_;
                                                    phi_20829_ = _e516;
                                                    phi_20830_ = _e518;
                                                    phi_20831_ = _e507;
                                                }
                                                let _e523 = phi_20829_;
                                                let _e525 = phi_20830_;
                                                let _e527 = phi_20831_;
                                                if _e527 {
                                                    phi_20835_ = _e523;
                                                } else {
                                                    phi_20835_ = _e525;
                                                }
                                                let _e529 = phi_20835_;
                                                if (bitcast<i32>(_e529.member) != 0i) {
                                                    phi_20769_ = select(_e529.member_1.member_1, _e529.member_1.member, (_e529.member_1.member >= 0f));
                                                } else {
                                                    phi_20769_ = -1f;
                                                }
                                                let _e540 = phi_20769_;
                                                if (_e540 >= 0f) {
                                                    if (_e481.member_1.member_1 != _e481.member_1.member_1) {
                                                        phi_20873_ = true;
                                                    } else {
                                                        phi_20873_ = (_e540 <= _e481.member_1.member_1);
                                                    }
                                                    let _e545 = phi_20873_;
                                                    phi_20598_ = select(_e481.member_1.member_1, _e540, _e545);
                                                } else {
                                                    phi_20598_ = _e481.member_1.member_1;
                                                }
                                                let _e548 = phi_20598_;
                                                phi_20600_ = _e548;
                                            } else {
                                                phi_20600_ = _e481.member_1.member_1;
                                            }
                                            let _e550 = phi_20600_;
                                            if (_e550 <= _e494) {
                                                phi_20609_ = type_50(0u, type_50().member_1);
                                            } else {
                                                phi_20609_ = type_50(1u, type_49(_e494, _e550));
                                            }
                                            let _e558 = phi_20609_;
                                            phi_20613_ = _e558;
                                        }
                                        let _e563 = phi_20613_;
                                        phi_20617_ = _e563;
                                    }
                                    let _e568 = phi_20617_;
                                    if (bitcast<i32>(_e568.member) != 0i) {
                                        let _e574 = (_e568.member_1.member < _e321.member.w);
                                        if _e574 {
                                            if (_e568.member_1.member_1 != _e568.member_1.member_1) {
                                                phi_20888_ = true;
                                            } else {
                                                phi_20888_ = (_e321.member.w <= _e568.member_1.member_1);
                                            }
                                            let _e580 = phi_20888_;
                                            let _e581 = select(_e568.member_1.member_1, _e321.member.w, _e580);
                                            phi_9472_ = vec3<f32>(0f, 0f, 0f);
                                            phi_9475_ = vec3<f32>(0f, 0f, 0f);
                                            phi_9477_ = vec3<f32>(0f, 0f, 0f);
                                            phi_9479_ = vec3<f32>(0f, 0f, 0f);
                                            phi_9481_ = 0u;
                                            loop {
                                                let _e583 = phi_9472_;
                                                let _e585 = phi_9475_;
                                                let _e587 = phi_9477_;
                                                let _e589 = phi_9479_;
                                                let _e591 = phi_9481_;
                                                local = _e587;
                                                local_1 = _e585;
                                                local_2 = _e583;
                                                local_3 = _e583;
                                                local_4 = _e583;
                                                local_5 = _e589;
                                                local_6 = _e589;
                                                local_7 = _e589;
                                                let _e592 = (_e591 < 32u);
                                                if _e592 {
                                                    let _e593 = -(_e424);
                                                    if (_e568.member_1.member <= _e581) {
                                                    } else {
                                                        phi_31725_ = true;
                                                        break;
                                                    }
                                                    let _e596 = select(_e593, _e568.member_1.member, (_e593 < _e568.member_1.member));
                                                    let _e598 = select(_e596, _e581, (_e596 > _e581));
                                                    let _e599 = (_e598 - _e568.member_1.member);
                                                    let _e600 = (_e581 - _e598);
                                                    let _e601 = (_e581 - _e568.member_1.member);
                                                    let _e602 = (0.000000001f * _e601);
                                                    if (_e599 <= _e602) {
                                                        let _e677 = (f32(_e591) * 0.03125f);
                                                        let _e680 = (f32((_e591 + 1u)) * 0.03125f);
                                                        let _e682 = ((_e677 + _e680) * 0.5f);
                                                        phi_9582_ = type_49((_e568.member_1.member + ((_e601 * _e682) * _e682)), (_e601 * ((_e680 * _e680) - (_e677 * _e677))));
                                                    } else {
                                                        if (_e600 <= _e602) {
                                                            let _e660 = (f32((31u - _e591)) * 0.03125f);
                                                            let _e663 = (f32((32u - _e591)) * 0.03125f);
                                                            let _e665 = ((_e660 + _e663) * 0.5f);
                                                            phi_9581_ = type_49((_e581 - ((_e601 * _e665) * _e665)), (_e601 * ((_e663 * _e663) - (_e660 * _e660))));
                                                        } else {
                                                            let _e608 = round(((32f * _e599) / (_e599 + _e600)));
                                                            let _e613 = select(select(u32(_e608), 0u, (_e608 < 0f)), 4294967295u, (_e608 > 4294967000f));
                                                            if (_e613 < 1u) {
                                                                phi_9565_ = 1u;
                                                            } else {
                                                                phi_9565_ = select(_e613, 31u, (_e613 > 31u));
                                                            }
                                                            let _e618 = phi_9565_;
                                                            if (_e591 < _e618) {
                                                                let _e639 = ((_e618 - 1u) - _e591);
                                                                let _e641 = f32(_e618);
                                                                let _e642 = (f32(_e639) / _e641);
                                                                let _e645 = (f32((_e639 + 1u)) / _e641);
                                                                let _e647 = ((_e642 + _e645) * 0.5f);
                                                                phi_9580_ = type_49((_e598 - ((_e599 * _e647) * _e647)), (_e599 * ((_e645 * _e645) - (_e642 * _e642))));
                                                            } else {
                                                                let _e621 = (_e591 - _e618);
                                                                let _e623 = f32((32u - _e618));
                                                                let _e624 = (f32(_e621) / _e623);
                                                                let _e627 = (f32((_e621 + 1u)) / _e623);
                                                                let _e629 = ((_e624 + _e627) * 0.5f);
                                                                phi_9580_ = type_49((_e598 + ((_e600 * _e629) * _e629)), (_e600 * ((_e627 * _e627) - (_e624 * _e624))));
                                                            }
                                                            let _e657 = phi_9580_;
                                                            phi_9581_ = _e657;
                                                        }
                                                        let _e675 = phi_9581_;
                                                        phi_9582_ = _e675;
                                                    }
                                                    let _e692 = phi_9582_;
                                                    let _e698 = (_e321.member.x + (_e321.member_1.x * _e692.member));
                                                    let _e699 = (_e321.member.y + (_e321.member_1.y * _e692.member));
                                                    let _e700 = (_e321.member.z + (_e321.member_1.z * _e692.member));
                                                    let _e713 = ((((_e698 * _e698) + (_e699 * _e699)) + (_e700 * _e700)) - (2f * (((_e698 * _e347) + (_e699 * _e350)) + (_e700 * _e353))));
                                                    let _e715 = (_e698 - _e347);
                                                    let _e716 = (_e699 - _e350);
                                                    let _e717 = (_e700 - _e353);
                                                    let _e723 = sqrt((((_e715 * _e715) + (_e716 * _e716)) + (_e717 * _e717)));
                                                    let _e725 = ((_e713 + _e362) / (_e723 + _e356));
                                                    let _e729 = (1f - (abs((_e725 - 25000f)) * 0.00006666667f));
                                                    if (_e729 != _e729) {
                                                        phi_21088_ = true;
                                                    } else {
                                                        phi_21088_ = (0f >= _e729);
                                                    }
                                                    let _e733 = phi_21088_;
                                                    let _e735 = -(_e725);
                                                    let _e738 = (exp((_e735 / _e377)) * _e692.member_1);
                                                    let _e741 = (exp((_e735 / _e380)) * _e692.member_1);
                                                    let _e743 = (_e738 * 0.5f);
                                                    let _e744 = (_e741 * 0.5f);
                                                    let _e745 = ((select(_e729, 0f, _e733) * _e692.member_1) * 0.5f);
                                                    let _e747 = (_e589.x + _e743);
                                                    let _e749 = (_e589.y + _e744);
                                                    let _e751 = (_e589.z + _e745);
                                                    let _e756 = (_e396 * 1.1f);
                                                    let _e757 = (_e756 * _e749);
                                                    let _e776 = (_e396 * _e741);
                                                    if (_e416 == 4294967295u) {
                                                        phi_9910_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e784 = (1f / _e723);
                                                        let _e792 = ((((_e715 * _e784) * _e368) + ((_e716 * _e784) * _e371)) + ((_e717 * _e784) * _e374));
                                                        if (_e725 != _e725) {
                                                            phi_21215_ = true;
                                                        } else {
                                                            phi_21215_ = (0f >= _e725);
                                                        }
                                                        let _e796 = phi_21215_;
                                                        let _e800 = sqrt((select(_e725, 0f, _e796) / (_e359 - _e356)));
                                                        if (_e800 != _e800) {
                                                            phi_21230_ = true;
                                                        } else {
                                                            phi_21230_ = (1f <= _e800);
                                                        }
                                                        let _e804 = phi_21230_;
                                                        let _e806 = (select(_e800, 1f, _e804) * 31f);
                                                        let _e808 = select(_e792, -1f, (_e792 < -1f));
                                                        let _e812 = ((select(_e808, 1f, (_e808 > 1f)) + 1f) * 15.5f);
                                                        let _e817 = select(select(u32(_e806), 0u, (_e806 < 0f)), 4294967295u, (_e806 > 4294967000f));
                                                        let _e819 = select(_e817, 30u, (30u < _e817));
                                                        let _e824 = select(select(u32(_e812), 0u, (_e812 < 0f)), 4294967295u, (_e812 > 4294967000f));
                                                        let _e826 = select(_e824, 30u, (30u < _e824));
                                                        let _e828 = (_e806 - f32(_e819));
                                                        let _e830 = (_e812 - f32(_e826));
                                                        let _e834 = (_e416 + (3u * ((_e819 * 32u) + _e826)));
                                                        let _e839 = (_e416 + (3u * (((_e819 + 1u) * 32u) + _e826)));
                                                        let _e841 = global_5.member[_e834];
                                                        let _e844 = global_5.member[(_e834 + 1u)];
                                                        let _e847 = global_5.member[(_e834 + 2u)];
                                                        let _e848 = (1f - _e830);
                                                        let _e854 = global_5.member[(_e834 + 3u)];
                                                        let _e857 = global_5.member[(_e834 + 4u)];
                                                        let _e860 = global_5.member[(_e834 + 5u)];
                                                        let _e867 = (1f - _e828);
                                                        let _e872 = global_5.member[_e839];
                                                        let _e875 = global_5.member[(_e839 + 1u)];
                                                        let _e878 = global_5.member[(_e839 + 2u)];
                                                        let _e884 = global_5.member[(_e839 + 3u)];
                                                        let _e887 = global_5.member[(_e839 + 4u)];
                                                        let _e890 = global_5.member[(_e839 + 5u)];
                                                        phi_9910_ = vec3<f32>(((((_e841 * _e848) + (_e854 * _e830)) * _e867) + (((_e872 * _e848) + (_e884 * _e830)) * _e828)), ((((_e844 * _e848) + (_e857 * _e830)) * _e867) + (((_e875 * _e848) + (_e887 * _e830)) * _e828)), ((((_e847 * _e848) + (_e860 * _e830)) * _e867) + (((_e878 * _e848) + (_e890 * _e830)) * _e828)));
                                                    }
                                                    let _e905 = phi_9910_;
                                                    let _e919 = (_e713 + _e365);
                                                    if (_e919 > 0f) {
                                                        let _e925 = (((_e715 * _e368) + (_e716 * _e371)) + (_e717 * _e374));
                                                        let _e927 = ((_e925 * _e925) - _e919);
                                                        if (_e925 >= 0f) {
                                                            phi_9984_ = vec3<f32>();
                                                            phi_9985_ = true;
                                                        } else {
                                                            let _e929 = (_e927 <= 0f);
                                                            if _e929 {
                                                                phi_9983_ = vec3<f32>();
                                                            } else {
                                                                let _e932 = (-(_e925) - sqrt(_e927));
                                                                phi_9983_ = vec3<f32>((_e698 + (_e368 * _e932)), (_e699 + (_e371 * _e932)), (_e700 + (_e374 * _e932)));
                                                            }
                                                            let _e941 = phi_9983_;
                                                            phi_9984_ = _e941;
                                                            phi_9985_ = _e929;
                                                        }
                                                        let _e943 = phi_9984_;
                                                        let _e945 = phi_9985_;
                                                        phi_9990_ = _e943;
                                                        phi_9991_ = select(true, false, _e945);
                                                    } else {
                                                        phi_9990_ = vec3<f32>(_e698, _e699, _e700);
                                                        phi_9991_ = true;
                                                    }
                                                    let _e948 = phi_9990_;
                                                    let _e950 = phi_9991_;
                                                    if _e950 {
                                                        let _e952 = (_e948.x - _e347);
                                                        let _e954 = (_e948.y - _e350);
                                                        let _e956 = (_e948.z - _e353);
                                                        let _e962 = sqrt((((_e952 * _e952) + (_e954 * _e954)) + (_e956 * _e956)));
                                                        let _e963 = (1f / _e962);
                                                        let _e981 = ((((((_e948.x * _e948.x) + (_e948.y * _e948.y)) + (_e948.z * _e948.z)) - (2f * (((_e948.x * _e347) + (_e948.y * _e350)) + (_e948.z * _e353)))) + _e362) / (_e962 + _e356));
                                                        if (_e981 != _e981) {
                                                            phi_21354_ = true;
                                                        } else {
                                                            phi_21354_ = (0f >= _e981);
                                                        }
                                                        let _e985 = phi_21354_;
                                                        let _e989 = sqrt((select(_e981, 0f, _e985) / (_e359 - _e356)));
                                                        if (_e989 != _e989) {
                                                            phi_21369_ = true;
                                                        } else {
                                                            phi_21369_ = (1f <= _e989);
                                                        }
                                                        let _e993 = phi_21369_;
                                                        let _e995 = (select(_e989, 1f, _e993) * 63f);
                                                        let _e1000 = ((((_e952 * _e963) * _e368) + ((_e954 * _e963) * _e371)) + ((_e956 * _e963) * _e374));
                                                        let _e1002 = select(_e1000, -1f, (_e1000 < -1f));
                                                        let _e1006 = ((select(_e1002, 1f, (_e1002 > 1f)) + 1f) * 127.5f);
                                                        let _e1011 = select(select(u32(_e995), 0u, (_e995 < 0f)), 4294967295u, (_e995 > 4294967000f));
                                                        let _e1013 = select(_e1011, 62u, (62u < _e1011));
                                                        let _e1018 = select(select(u32(_e1006), 0u, (_e1006 < 0f)), 4294967295u, (_e1006 > 4294967000f));
                                                        let _e1020 = select(_e1018, 254u, (254u < _e1018));
                                                        let _e1022 = (_e995 - f32(_e1013));
                                                        let _e1024 = (_e1006 - f32(_e1020));
                                                        let _e1028 = (_e412 + (3u * ((_e1013 * 256u) + _e1020)));
                                                        let _e1033 = (_e412 + (3u * (((_e1013 + 1u) * 256u) + _e1020)));
                                                        let _e1035 = global_5.member[_e1028];
                                                        let _e1038 = global_5.member[(_e1028 + 1u)];
                                                        let _e1041 = global_5.member[(_e1028 + 2u)];
                                                        let _e1042 = (1f - _e1024);
                                                        let _e1048 = global_5.member[(_e1028 + 3u)];
                                                        let _e1051 = global_5.member[(_e1028 + 4u)];
                                                        let _e1054 = global_5.member[(_e1028 + 5u)];
                                                        let _e1061 = (1f - _e1022);
                                                        let _e1066 = global_5.member[_e1033];
                                                        let _e1069 = global_5.member[(_e1033 + 1u)];
                                                        let _e1072 = global_5.member[(_e1033 + 2u)];
                                                        let _e1078 = global_5.member[(_e1033 + 3u)];
                                                        let _e1081 = global_5.member[(_e1033 + 4u)];
                                                        let _e1084 = global_5.member[(_e1033 + 5u)];
                                                        phi_10282_ = vec3<f32>(((((_e1035 * _e1042) + (_e1048 * _e1024)) * _e1061) + (((_e1066 * _e1042) + (_e1078 * _e1024)) * _e1022)), ((((_e1038 * _e1042) + (_e1051 * _e1024)) * _e1061) + (((_e1069 * _e1042) + (_e1081 * _e1024)) * _e1022)), ((((_e1041 * _e1042) + (_e1054 * _e1024)) * _e1061) + (((_e1072 * _e1042) + (_e1084 * _e1024)) * _e1022)));
                                                    } else {
                                                        phi_10282_ = vec3<f32>(0f, 0f, 0f);
                                                    }
                                                    let _e1099 = phi_10282_;
                                                    let _e1100 = (vec3<f32>(_e747, _e749, _e751) + _e1099);
                                                    let _e1106 = (_e756 * _e1100.y);
                                                    let _e1118 = exp(-((((_e386 * _e1100.x) + _e1106) + (_e402 * _e1100.z))));
                                                    let _e1120 = exp(-((((_e389 * _e1100.x) + _e1106) + (_e405 * _e1100.z))));
                                                    let _e1122 = exp(-((((_e392 * _e1100.x) + _e1106) + (_e408 * _e1100.z))));
                                                    phi_9473_ = vec3<f32>((_e583.x + ((exp(-((((_e386 * _e747) + _e757) + (_e402 * _e751)))) * ((_e386 * _e738) + _e776)) * _e905.x)), (_e583.y + ((exp(-((((_e389 * _e747) + _e757) + (_e405 * _e751)))) * ((_e389 * _e738) + _e776)) * _e905.y)), (_e583.z + ((exp(-((((_e392 * _e747) + _e757) + (_e408 * _e751)))) * ((_e392 * _e738) + _e776)) * _e905.z)));
                                                    phi_9476_ = vec3<f32>((_e585.x + (_e1118 * _e741)), (_e585.y + (_e1120 * _e741)), (_e585.z + (_e1122 * _e741)));
                                                    phi_9478_ = vec3<f32>((_e587.x + (_e1118 * _e738)), (_e587.y + (_e1120 * _e738)), (_e587.z + (_e1122 * _e738)));
                                                    phi_9480_ = vec3<f32>((_e747 + _e743), (_e749 + _e744), (_e751 + _e745));
                                                    phi_9482_ = (_e591 + 1u);
                                                } else {
                                                    phi_9473_ = vec3<f32>();
                                                    phi_9476_ = vec3<f32>();
                                                    phi_9478_ = vec3<f32>();
                                                    phi_9480_ = vec3<f32>();
                                                    phi_9482_ = u32();
                                                }
                                                let _e1149 = phi_9473_;
                                                let _e1151 = phi_9476_;
                                                let _e1153 = phi_9478_;
                                                let _e1155 = phi_9480_;
                                                let _e1157 = phi_9482_;
                                                phi_9472_ = _e1149;
                                                phi_9475_ = _e1151;
                                                phi_9477_ = _e1153;
                                                phi_9479_ = _e1155;
                                                phi_9481_ = _e1157;
                                                phi_31725_ = _e310;
                                                if !(_e592) {
                                                    break;
                                                }
                                            }
                                            let _e1160 = phi_31725_;
                                            phi_32203_ = _e1160;
                                            if _e1160 {
                                                break;
                                            }
                                            let _e1162 = local;
                                            let _e1163 = (_e1162 * vec3<f32>(_e386, _e389, _e392));
                                            let _e1165 = local_1;
                                            let _e1166 = (_e1165 * _e396);
                                            let _e1171 = (((_e321.member_1.x * _e368) + (_e321.member_1.y * _e371)) + (_e321.member_1.z * _e374));
                                            let _e1173 = (1f + (_e1171 * _e1171));
                                            let _e1174 = (0.059683103f * _e1173);
                                            let _e1181 = (_e399 * _e399);
                                            let _e1192 = ((0.119366206f * ((1f - _e1181) * _e1173)) / ((2f + _e1181) * pow(((1f + _e1181) - ((2f * _e399) * _e1171)), 1.5f)));
                                            let _e1203 = local_2;
                                            let _e1207 = local_3;
                                            let _e1211 = local_4;
                                            let _e1220 = local_5;
                                            let _e1227 = local_6;
                                            let _e1229 = ((_e396 * 1.1f) * _e1227.y);
                                            let _e1234 = local_7;
                                            phi_32209_ = _e1160;
                                            phi_10339_ = type_53(vec3<f32>(exp(-((((_e386 * _e1220.x) + _e1229) + (_e402 * _e1234.z)))), exp(-((((_e389 * _e1220.x) + _e1229) + (_e405 * _e1234.z)))), exp(-((((_e392 * _e1220.x) + _e1229) + (_e408 * _e1234.z))))), (vec3<f32>(((((_e1163.x * _e1174) + (_e1166.x * _e1192)) + _e1203.x) * _e383), ((((_e1163.y * _e1174) + (_e1166.y * _e1192)) + _e1207.y) * _e383), ((((_e1163.z * _e1174) + (_e1166.z * _e1192)) + _e1211.z) * _e383)) * 0.38317f));
                                        } else {
                                            phi_32209_ = _e310;
                                            phi_10339_ = type_53();
                                        }
                                        let _e1251 = phi_32209_;
                                        let _e1253 = phi_10339_;
                                        phi_32208_ = _e1251;
                                        phi_10341_ = _e1253;
                                        phi_10342_ = select(true, false, _e574);
                                    } else {
                                        phi_32208_ = _e310;
                                        phi_10341_ = type_53();
                                        phi_10342_ = true;
                                    }
                                    let _e1256 = phi_32208_;
                                    let _e1258 = phi_10341_;
                                    let _e1260 = phi_10342_;
                                    if _e1260 {
                                        phi_10347_ = type_53(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                    } else {
                                        phi_10347_ = _e1258;
                                    }
                                    let _e1262 = phi_10347_;
                                    phi_32206_ = _e1256;
                                    phi_10351_ = (vec3<f32>(_e321.member_2.x, _e321.member_2.y, _e321.member_2.z) * _e1262.member_1);
                                }
                                let _e6667 = phi_32206_;
                                let _e6669 = phi_10351_;
                                phi_32205_ = _e6667;
                                phi_10352_ = _e6669;
                            }
                            let _e6671 = phi_32205_;
                            let _e6673 = phi_10352_;
                            phi_32204_ = _e6671;
                            phi_470_ = (_e312 + _e6673);
                            phi_473_ = (_e314 + 1u);
                        } else {
                            phi_32204_ = _e310;
                            phi_470_ = vec3<f32>();
                            phi_473_ = u32();
                        }
                        let _e6677 = phi_32204_;
                        let _e6679 = phi_470_;
                        let _e6681 = phi_473_;
                        phi_31742_ = _e6677;
                        phi_469_ = _e6679;
                        phi_472_ = _e6681;
                        phi_32203_ = _e6677;
                        if !(_e315) {
                            break;
                        }
                    }
                    let _e6684 = phi_32203_;
                    if _e6684 {
                        break;
                    }
                    if _e301 {
                    } else {
                        break;
                    }
                    let _e6686 = local_55;
                    let _e6689 = local_56;
                    let _e6692 = local_57;
                    let _e6698 = global_6.member[_e296.member_1].member_3[0u];
                    global_6.member[_e296.member_1].member_3[0u] = (_e6698 + _e6686.x);
                    let _e6704 = global_6.member[_e296.member_1].member_3[1u];
                    global_6.member[_e296.member_1].member_3[1u] = (_e6704 + _e6689.y);
                    let _e6710 = global_6.member[_e296.member_1].member_3[2u];
                    global_6.member[_e296.member_1].member_3[2u] = (_e6710 + _e6692.z);
                    let _e6716 = global_6.member[_e296.member_1].member_3[3u];
                    global_6.member[_e296.member_1].member_3[3u] = _e6716;
                    if _e301 {
                    } else {
                        break;
                    }
                    global_6.member[_e296.member_1].member_4[1u] = (_e306 & 4294967279u);
                }
            }
            break;
        }
    }
    return;
}

fn function_1() {
    var local_67: array<u32, 100>;
    var local_68: array<u32, 100>;
    var phi_11116_: type_40;
    var phi_11317_: u32;
    var phi_11320_: u32;
    var phi_11322_: u32;
    var phi_11324_: vec4<f32>;
    var phi_11326_: type_59;
    var phi_24949_: bool;
    var phi_24979_: bool;
    var phi_24926_: type_49;
    var phi_24929_: type_49;
    var phi_24930_: bool;
    var phi_24935_: type_49;
    var phi_25036_: bool;
    var phi_25051_: bool;
    var phi_25066_: bool;
    var phi_25081_: bool;
    var phi_25013_: type_49;
    var phi_25016_: type_49;
    var phi_25017_: bool;
    var phi_25022_: type_49;
    var phi_25123_: bool;
    var phi_25138_: bool;
    var phi_25153_: bool;
    var phi_25168_: bool;
    var phi_25100_: type_49;
    var phi_25103_: type_49;
    var phi_25104_: bool;
    var phi_25109_: type_49;
    var phi_24902_: bool;
    var phi_24905_: bool;
    var phi_24906_: bool;
    var phi_11716_: u32;
    var phi_11717_: u32;
    var phi_11718_: u32;
    var phi_11719_: u32;
    var phi_11720_: u32;
    var phi_11721_: u32;
    var phi_11722_: u32;
    var phi_11723_: u32;
    var phi_11724_: u32;
    var phi_11605_: vec4<f32>;
    var phi_11606_: vec4<f32>;
    var phi_11607_: bool;
    var phi_11611_: vec4<f32>;
    var phi_11612_: bool;
    var phi_11522_: bool;
    var phi_11556_: bool;
    var phi_11562_: f32;
    var phi_11572_: vec4<f32>;
    var phi_11573_: vec4<f32>;
    var phi_11574_: bool;
    var phi_11578_: vec4<f32>;
    var phi_11579_: bool;
    var phi_11583_: vec4<f32>;
    var phi_11584_: bool;
    var phi_11613_: vec4<f32>;
    var phi_11614_: bool;
    var phi_11454_: vec4<f32>;
    var phi_11455_: vec4<f32>;
    var phi_11456_: bool;
    var phi_11460_: vec4<f32>;
    var phi_11461_: bool;
    var phi_11462_: vec4<f32>;
    var phi_11463_: bool;
    var phi_11464_: bool;
    var phi_11472_: vec4<f32>;
    var phi_11473_: bool;
    var phi_11474_: vec4<f32>;
    var phi_11475_: bool;
    var phi_11476_: bool;
    var phi_11484_: vec4<f32>;
    var phi_11485_: bool;
    var phi_11623_: vec4<f32>;
    var phi_11624_: bool;
    var phi_11645_: type_59;
    var phi_11647_: type_59;
    var phi_11318_: u32;
    var phi_11321_: u32;
    var phi_11323_: u32;
    var phi_11325_: vec4<f32>;
    var phi_11327_: type_59;
    var phi_11726_: type_59;
    var phi_11727_: bool;
    var phi_11728_: bool;
    var phi_32214_: bool;
    var phi_20547_: bool;
    var phi_20546_: type_59;
    var local_69: type_59;
    var local_70: vec4<f32>;
    var phi_11889_: type_53;
    var local_71: vec4<f32>;
    var local_72: vec4<f32>;
    var phi_11855_: vec3<f32>;
    var phi_11856_: vec3<f32>;
    var phi_11890_: type_53;
    var local_73: type_59;
    var phi_11895_: type_59;
    var phi_11896_: type_59;
    var phi_11901_: type_59;
    var phi_25287_: f32;
    var phi_25299_: type_49;
    var phi_25310_: type_50;
    var phi_25311_: type_50;
    var phi_25315_: type_50;
    var phi_25316_: type_50;
    var phi_25317_: bool;
    var phi_25321_: type_50;
    var phi_25208_: type_50;
    var phi_25359_: bool;
    var phi_25439_: f32;
    var phi_25451_: type_49;
    var phi_25462_: type_50;
    var phi_25463_: type_50;
    var phi_25467_: type_50;
    var phi_25468_: type_50;
    var phi_25469_: bool;
    var phi_25473_: type_50;
    var phi_25407_: f32;
    var phi_25511_: bool;
    var phi_25236_: f32;
    var phi_25238_: f32;
    var phi_25247_: type_50;
    var phi_25251_: type_50;
    var phi_25255_: type_50;
    var phi_25526_: bool;
    var phi_13320_: vec3<f32>;
    var phi_13323_: u32;
    var phi_13407_: u32;
    var phi_13422_: type_49;
    var phi_13423_: type_49;
    var phi_13424_: type_49;
    var phi_25725_: bool;
    var phi_13321_: vec3<f32>;
    var phi_13324_: u32;
    var phi_32287_: bool;
    var local_74: vec3<f32>;
    var local_75: vec3<f32>;
    var local_76: vec3<f32>;
    var phi_34514_: bool;
    var phi_13449_: vec3<f32>;
    var phi_34513_: bool;
    var phi_13451_: vec3<f32>;
    var phi_13452_: bool;
    var phi_34511_: bool;
    var phi_13459_: vec3<f32>;
    var phi_13460_: type_34;
    var phi_34510_: bool;
    var phi_13461_: vec3<f32>;
    var phi_13462_: type_34;
    var phi_13486_: i32;
    var phi_13489_: u32;
    var phi_13543_: bool;
    var phi_13549_: bool;
    var phi_13556_: u32;
    var phi_13558_: i32;
    var phi_13559_: u32;
    var phi_13487_: i32;
    var phi_13490_: u32;
    var phi_13561_: bool;
    var local_77: u32;
    var phi_13842_: vec3<f32>;
    var phi_13859_: vec3<f32>;
    var phi_25792_: bool;
    var phi_13892_: f32;
    var phi_14001_: vec3<f32>;
    var phi_14007_: f32;
    var phi_25865_: bool;
    var phi_25854_: f32;
    var phi_14088_: vec3<f32>;
    var phi_14089_: vec3<f32>;
    var phi_14090_: vec3<f32>;
    var phi_32503_: u32;
    var phi_14092_: vec3<f32>;
    var phi_14093_: vec3<f32>;
    var phi_14094_: vec3<f32>;
    var phi_14095_: bool;
    var phi_14110_: vec3<f32>;
    var phi_14111_: vec3<f32>;
    var phi_32501_: u32;
    var phi_14113_: vec3<f32>;
    var phi_14114_: vec3<f32>;
    var phi_14115_: vec3<f32>;
    var phi_32500_: u32;
    var phi_14116_: vec3<f32>;
    var phi_14117_: vec3<f32>;
    var phi_14118_: vec3<f32>;
    var phi_14119_: bool;
    var phi_32499_: u32;
    var phi_14120_: vec3<f32>;
    var phi_14121_: vec3<f32>;
    var phi_14122_: vec3<f32>;
    var phi_14123_: bool;
    var phi_14124_: bool;
    var phi_25938_: bool;
    var phi_26100_: bool;
    var phi_26115_: bool;
    var phi_26130_: bool;
    var phi_26157_: bool;
    var phi_26172_: bool;
    var phi_26187_: bool;
    var phi_26214_: bool;
    var phi_26229_: bool;
    var phi_26244_: bool;
    var phi_14303_: f32;
    var phi_14311_: f32;
    var phi_14319_: f32;
    var phi_26441_: bool;
    var phi_26430_: f32;
    var phi_14426_: vec3<f32>;
    var phi_26469_: bool;
    var phi_14427_: vec3<f32>;
    var phi_14428_: vec3<f32>;
    var phi_14429_: vec3<f32>;
    var phi_32887_: vec3<f32>;
    var phi_32810_: vec3<f32>;
    var phi_32731_: u32;
    var phi_32649_: f32;
    var phi_14431_: vec3<f32>;
    var phi_14432_: vec3<f32>;
    var phi_14433_: bool;
    var phi_26484_: bool;
    var phi_26535_: bool;
    var phi_26524_: f32;
    var phi_32858_: vec3<f32>;
    var phi_13965_: vec3<f32>;
    var phi_33210_: vec3<f32>;
    var phi_33004_: f32;
    var phi_32857_: vec3<f32>;
    var phi_32545_: u32;
    var phi_13967_: vec3<f32>;
    var phi_13968_: bool;
    var phi_33156_: vec3<f32>;
    var phi_32964_: f32;
    var phi_32885_: vec3<f32>;
    var phi_32808_: vec3<f32>;
    var phi_32729_: u32;
    var phi_32647_: f32;
    var phi_32570_: u32;
    var phi_32496_: u32;
    var phi_14441_: vec3<f32>;
    var phi_14442_: vec3<f32>;
    var phi_14443_: vec3<f32>;
    var phi_14444_: bool;
    var phi_26601_: bool;
    var phi_14513_: vec3<f32>;
    var phi_32553_: u32;
    var phi_14515_: vec3<f32>;
    var phi_14516_: bool;
    var phi_32552_: u32;
    var phi_14517_: vec3<f32>;
    var phi_14518_: bool;
    var phi_32550_: u32;
    var phi_14524_: vec3<f32>;
    var phi_14525_: bool;
    var phi_14558_: type_64;
    var phi_32555_: u32;
    var phi_14560_: vec3<f32>;
    var phi_14561_: type_64;
    var phi_14562_: bool;
    var phi_14568_: type_64;
    var phi_32559_: u32;
    var phi_14569_: type_64;
    var phi_33153_: vec3<f32>;
    var phi_32961_: f32;
    var phi_32882_: vec3<f32>;
    var phi_32805_: vec3<f32>;
    var phi_32726_: u32;
    var phi_32644_: f32;
    var phi_32567_: u32;
    var phi_32558_: u32;
    var phi_14571_: type_64;
    var phi_14572_: bool;
    var phi_34490_: bool;
    var phi_32557_: u32;
    var phi_14583_: vec3<f32>;
    var phi_14586_: type_34;
    var phi_14588_: u32;
    var phi_14590_: u32;
    var phi_17465_: u32;
    var phi_17468_: u32;
    var phi_17466_: u32;
    var phi_17469_: u32;
    var local_78: u32;
    var phi_17506_: u32;
    var phi_17509_: u32;
    var phi_17507_: u32;
    var phi_17510_: u32;
    var local_79: u32;
    var local_80: u32;
    var phi_17586_: f32;
    var local_81: u32;
    var local_82: u32;
    var phi_17635_: f32;
    var phi_17791_: f32;
    var phi_17798_: vec3<f32>;
    var phi_34854_: u32;
    var phi_17800_: f32;
    var phi_17801_: vec3<f32>;
    var phi_17802_: vec3<f32>;
    var phi_16938_: u32;
    var phi_16941_: u32;
    var phi_16939_: u32;
    var phi_16942_: u32;
    var local_83: u32;
    var local_84: u32;
    var phi_17334_: f32;
    var phi_17337_: f32;
    var phi_26874_: bool;
    var phi_26855_: f32;
    var phi_17338_: f32;
    var phi_26889_: bool;
    var phi_26912_: bool;
    var phi_26927_: bool;
    var phi_17089_: f32;
    var phi_26942_: bool;
    var phi_26957_: bool;
    var phi_26972_: bool;
    var phi_27037_: bool;
    var phi_27125_: bool;
    var phi_27106_: f32;
    var phi_17154_: type_66;
    var phi_17340_: type_66;
    var phi_17418_: bool;
    var phi_17427_: vec3<f32>;
    var phi_17429_: vec3<f32>;
    var phi_17430_: bool;
    var phi_17435_: f32;
    var phi_17436_: vec3<f32>;
    var phi_34862_: u32;
    var phi_17437_: f32;
    var phi_17438_: vec3<f32>;
    var phi_17439_: vec3<f32>;
    var phi_17440_: f32;
    var phi_34861_: u32;
    var phi_17441_: f32;
    var phi_17442_: vec3<f32>;
    var phi_17443_: vec3<f32>;
    var phi_17444_: f32;
    var phi_34853_: u32;
    var phi_17803_: f32;
    var phi_17804_: vec3<f32>;
    var phi_17805_: vec3<f32>;
    var phi_17806_: f32;
    var phi_27140_: bool;
    var phi_27250_: f32;
    var phi_27278_: f32;
    var phi_27306_: f32;
    var phi_27440_: bool;
    var phi_27455_: bool;
    var phi_27470_: bool;
    var phi_27497_: bool;
    var phi_27512_: bool;
    var phi_27527_: bool;
    var phi_27554_: bool;
    var phi_27569_: bool;
    var phi_27584_: bool;
    var phi_15110_: f32;
    var phi_15118_: f32;
    var phi_15126_: f32;
    var phi_27781_: bool;
    var phi_27770_: f32;
    var phi_15233_: vec3<f32>;
    var phi_27809_: bool;
    var phi_15234_: vec3<f32>;
    var phi_15246_: type_52;
    var phi_15247_: type_52;
    var phi_15248_: bool;
    var phi_15253_: type_52;
    var phi_27846_: bool;
    var phi_27876_: bool;
    var phi_27865_: f32;
    var phi_27915_: vec3<f32>;
    var phi_15281_: type_52;
    var phi_15282_: type_52;
    var phi_27937_: vec3<f32>;
    var phi_15283_: type_52;
    var phi_15284_: type_52;
    var phi_15285_: bool;
    var phi_15289_: type_52;
    var phi_27969_: bool;
    var phi_28040_: bool;
    var phi_16109_: f32;
    var phi_28055_: bool;
    var phi_28070_: bool;
    var phi_16582_: vec3<f32>;
    var phi_16583_: vec3<f32>;
    var phi_16584_: bool;
    var phi_16589_: vec3<f32>;
    var phi_16590_: bool;
    var phi_28194_: bool;
    var phi_28209_: bool;
    var phi_16881_: vec3<f32>;
    var phi_16883_: vec3<f32>;
    var phi_16910_: vec3<f32>;
    var phi_16911_: vec3<f32>;
    var phi_15727_: vec3<f32>;
    var phi_15728_: vec3<f32>;
    var phi_15729_: bool;
    var phi_15734_: vec3<f32>;
    var phi_15735_: bool;
    var phi_28371_: bool;
    var phi_28386_: bool;
    var phi_16026_: vec3<f32>;
    var phi_16028_: vec3<f32>;
    var phi_34896_: u32;
    var phi_16912_: f32;
    var phi_16913_: vec3<f32>;
    var phi_16914_: vec3<f32>;
    var phi_34895_: u32;
    var phi_16915_: f32;
    var phi_16916_: vec3<f32>;
    var phi_16917_: vec3<f32>;
    var phi_34852_: u32;
    var phi_17807_: f32;
    var phi_17808_: vec3<f32>;
    var phi_17809_: vec3<f32>;
    var phi_17810_: f32;
    var phi_14872_: f32;
    var phi_14873_: f32;
    var phi_34851_: u32;
    var phi_17811_: f32;
    var phi_17812_: vec3<f32>;
    var phi_17813_: vec3<f32>;
    var phi_17814_: f32;
    var phi_28439_: bool;
    var phi_28549_: f32;
    var phi_28577_: f32;
    var phi_28605_: f32;
    var phi_28739_: bool;
    var phi_28754_: bool;
    var phi_28769_: bool;
    var phi_28796_: bool;
    var phi_28811_: bool;
    var phi_28826_: bool;
    var phi_28853_: bool;
    var phi_28868_: bool;
    var phi_28883_: bool;
    var phi_18037_: f32;
    var phi_18045_: f32;
    var phi_18053_: f32;
    var phi_29080_: bool;
    var phi_29069_: f32;
    var phi_18160_: vec3<f32>;
    var phi_29108_: bool;
    var phi_18161_: vec3<f32>;
    var phi_18173_: type_52;
    var phi_18174_: type_52;
    var phi_18175_: bool;
    var phi_18180_: type_52;
    var phi_29145_: bool;
    var phi_29175_: bool;
    var phi_29164_: f32;
    var phi_29214_: vec3<f32>;
    var phi_18208_: type_52;
    var phi_18209_: type_52;
    var phi_29236_: vec3<f32>;
    var phi_18210_: type_52;
    var phi_18211_: type_52;
    var phi_18212_: bool;
    var phi_18216_: type_52;
    var phi_29250_: f32;
    var phi_29253_: f32;
    var phi_18225_: f32;
    var phi_18259_: u32;
    var phi_18262_: u32;
    var phi_18264_: u32;
    var phi_18268_: type_59;
    var phi_29350_: bool;
    var phi_29380_: bool;
    var phi_29327_: type_49;
    var phi_29330_: type_49;
    var phi_29331_: bool;
    var phi_29336_: type_49;
    var phi_29437_: bool;
    var phi_29452_: bool;
    var phi_29467_: bool;
    var phi_29482_: bool;
    var phi_29414_: type_49;
    var phi_29417_: type_49;
    var phi_29418_: bool;
    var phi_29423_: type_49;
    var phi_29524_: bool;
    var phi_29539_: bool;
    var phi_29554_: bool;
    var phi_29569_: bool;
    var phi_29501_: type_49;
    var phi_29504_: type_49;
    var phi_29505_: bool;
    var phi_29510_: type_49;
    var phi_29303_: bool;
    var phi_29306_: bool;
    var phi_29307_: bool;
    var phi_18675_: u32;
    var phi_18676_: u32;
    var phi_18677_: u32;
    var phi_18678_: u32;
    var phi_18679_: u32;
    var phi_18680_: u32;
    var phi_18681_: u32;
    var phi_18682_: u32;
    var phi_18683_: u32;
    var phi_18564_: vec4<f32>;
    var phi_18565_: vec4<f32>;
    var phi_18566_: bool;
    var phi_18570_: vec4<f32>;
    var phi_18571_: bool;
    var phi_18476_: bool;
    var phi_18513_: bool;
    var phi_18519_: f32;
    var phi_18529_: vec4<f32>;
    var phi_18530_: vec4<f32>;
    var phi_18531_: bool;
    var phi_18535_: vec4<f32>;
    var phi_18536_: bool;
    var phi_18540_: vec4<f32>;
    var phi_18541_: bool;
    var phi_18572_: vec4<f32>;
    var phi_18573_: bool;
    var phi_18405_: vec4<f32>;
    var phi_18406_: vec4<f32>;
    var phi_18407_: bool;
    var phi_18411_: vec4<f32>;
    var phi_18412_: bool;
    var phi_18413_: vec4<f32>;
    var phi_18414_: bool;
    var phi_18415_: bool;
    var phi_18423_: vec4<f32>;
    var phi_18424_: bool;
    var phi_18425_: vec4<f32>;
    var phi_18426_: bool;
    var phi_18427_: bool;
    var phi_18435_: vec4<f32>;
    var phi_18436_: bool;
    var phi_18582_: vec4<f32>;
    var phi_18583_: bool;
    var phi_18604_: type_59;
    var phi_18606_: type_59;
    var phi_18612_: u32;
    var phi_18260_: u32;
    var phi_18263_: u32;
    var phi_18265_: u32;
    var phi_18269_: type_59;
    var phi_18685_: type_59;
    var phi_18686_: bool;
    var phi_18687_: bool;
    var phi_34410_: bool;
    var phi_20549_: bool;
    var phi_20548_: type_59;
    var phi_18859_: type_59;
    var phi_18864_: type_59;
    var phi_18916_: vec3<f32>;
    var phi_18917_: type_34;
    var phi_18918_: vec3<f32>;
    var phi_18919_: type_34;
    var phi_34798_: bool;
    var phi_18920_: vec3<f32>;
    var phi_18921_: type_34;
    var phi_18922_: u32;
    var phi_34797_: bool;
    var phi_18923_: vec3<f32>;
    var phi_18924_: type_34;
    var phi_18925_: u32;
    var phi_35040_: u32;
    var phi_34796_: bool;
    var phi_14584_: vec3<f32>;
    var phi_14587_: type_34;
    var phi_14589_: u32;
    var phi_14591_: u32;
    var phi_34809_: u32;
    var phi_34795_: bool;
    var phi_29720_: bool;
    var phi_29750_: bool;
    var phi_29739_: f32;
    var phi_29670_: bool;
    var phi_29806_: vec3<f32>;
    var phi_29962_: bool;
    var phi_29977_: bool;
    var phi_29992_: bool;
    var phi_29901_: vec3<f32>;
    var phi_29904_: vec3<f32>;
    var phi_29905_: bool;
    var phi_29675_: vec3<f32>;
    var phi_29679_: vec3<f32>;
    var phi_29680_: bool;
    var phi_30092_: bool;
    var phi_30066_: vec3<f32>;
    var phi_29687_: vec3<f32>;
    var phi_30107_: bool;
    var phi_30217_: f32;
    var phi_30245_: f32;
    var phi_30273_: f32;
    var phi_30407_: bool;
    var phi_30422_: bool;
    var phi_30437_: bool;
    var phi_30464_: bool;
    var phi_30479_: bool;
    var phi_30494_: bool;
    var phi_30521_: bool;
    var phi_30536_: bool;
    var phi_30551_: bool;
    var phi_19144_: f32;
    var phi_19152_: f32;
    var phi_19160_: f32;
    var phi_30748_: bool;
    var phi_30737_: f32;
    var phi_19267_: vec3<f32>;
    var phi_30776_: bool;
    var phi_19268_: vec3<f32>;
    var phi_19280_: type_52;
    var phi_19281_: type_52;
    var phi_19282_: bool;
    var phi_19287_: type_52;
    var phi_30813_: bool;
    var phi_30843_: bool;
    var phi_30832_: f32;
    var phi_30882_: vec3<f32>;
    var phi_19315_: type_52;
    var phi_19316_: type_52;
    var phi_30904_: vec3<f32>;
    var phi_19317_: type_52;
    var phi_19318_: type_52;
    var phi_19319_: bool;
    var phi_19323_: type_52;
    var phi_30929_: bool;
    var phi_19399_: vec3<f32>;
    var phi_36299_: u32;
    var phi_19401_: vec3<f32>;
    var phi_19402_: bool;
    var phi_36298_: u32;
    var phi_19403_: vec3<f32>;
    var phi_19404_: bool;
    var phi_36296_: u32;
    var phi_19410_: vec3<f32>;
    var phi_19411_: bool;
    var local_85: u32;
    var local_86: vec3<f32>;
    var local_87: vec3<f32>;
    var local_88: vec3<f32>;
    var local_89: type_34;
    var phi_19440_: vec3<f32>;
    var local_90: u32;
    var local_91: vec3<f32>;
    var local_92: vec3<f32>;
    var local_93: vec3<f32>;
    var local_94: type_34;
    var phi_19456_: type_64;
    var phi_19457_: type_64;
    var phi_36303_: u32;
    var phi_19459_: type_64;
    var phi_19460_: type_64;
    var phi_19461_: bool;
    var phi_36302_: u32;
    var phi_19462_: type_64;
    var phi_19463_: type_64;
    var phi_19464_: bool;
    var phi_19465_: bool;
    var local_95: u32;
    var local_96: vec3<f32>;
    var local_97: vec3<f32>;
    var local_98: vec3<f32>;
    var local_99: type_34;
    var phi_19471_: type_64;
    var local_100: u32;
    var local_101: vec3<f32>;
    var local_102: vec3<f32>;
    var local_103: vec3<f32>;
    var local_104: type_34;
    var phi_19476_: type_64;
    var phi_19477_: type_64;
    var phi_19478_: bool;
    var phi_19482_: type_64;
    var phi_19483_: type_64;
    var phi_19484_: bool;
    var phi_13621_: bool;
    var phi_13630_: vec3<f32>;
    var phi_13632_: vec3<f32>;
    var phi_13633_: bool;
    var phi_13794_: f32;
    var phi_13797_: f32;
    var phi_31150_: bool;
    var phi_31131_: f32;
    var phi_13798_: f32;
    var phi_31168_: f32;
    var phi_31171_: f32;
    var phi_13801_: f32;
    var phi_13802_: f32;
    var phi_13803_: f32;
    var phi_19488_: type_64;
    var phi_19489_: type_64;
    var phi_19490_: bool;
    var phi_12057_: f32;
    var phi_31218_: f32;
    var phi_31221_: f32;
    var phi_12059_: f32;
    var phi_12060_: f32;
    var phi_31247_: bool;
    var phi_12111_: f32;
    var phi_31265_: f32;
    var phi_31268_: f32;
    var phi_12113_: f32;
    var phi_31279_: bool;
    var phi_31294_: bool;
    var phi_12582_: vec3<f32>;
    var phi_12583_: vec3<f32>;
    var phi_12584_: bool;
    var phi_12589_: vec3<f32>;
    var phi_12590_: bool;
    var phi_31418_: bool;
    var phi_31433_: bool;
    var phi_12881_: vec3<f32>;
    var phi_12883_: vec3<f32>;
    var phi_12910_: vec3<f32>;
    var phi_12924_: vec3<f32>;
    var phi_19494_: type_64;
    var phi_19495_: type_64;
    var phi_19496_: bool;
    var phi_19500_: type_64;
    var phi_19538_: bool;
    var phi_19545_: type_30;
    var phi_11199_: bool;
    var phi_11200_: bool;
    var phi_11201_: bool;
    var phi_11222_: vec4<f32>;
    var phi_11223_: type_30;
    var phi_19546_: vec4<f32>;
    var phi_19547_: type_34;
    var phi_19548_: type_34;
    var phi_19549_: type_30;
    var local_105: type_59;
    var local_106: type_59;

    switch bitcast<i32>(0u) {
        default: {
            let _e272 = global;
            let _e283 = arrayLength((&global_7.member));
            let _e287 = (_e272.x + (_e272.y * 4194240u));
            let _e291 = global_1.member.member[0u];
            let _e295 = global_1.member.member[1u];
            if (_e287 >= (_e291 * _e295)) {
                phi_11116_ = type_40(0u, type_40().member_1);
            } else {
                phi_11116_ = type_40(1u, _e287);
            }
            let _e303 = phi_11116_;
            if (bitcast<i32>(_e303.member) != 0i) {
                let _e308 = (_e303.member_1 < arrayLength((&global_6.member)));
                if _e308 {
                } else {
                    break;
                }
                let _e315 = global_6.member[_e303.member_1].member_4[1u];
                if ((_e315 & 9u) == 8u) {
                } else {
                    if _e308 {
                    } else {
                        break;
                    }
                    let _e318 = global_6.member[_e303.member_1];
                    if ((_e318.member_4.y & 1u) == 0u) {
                        if ((_e318.member_4.y & 8u) == 0u) {
                            let _e7363 = (_e318.member_3.x * _e318.member.w);
                            let _e7364 = (_e318.member_3.y * _e318.member.w);
                            let _e7365 = (_e318.member_3.z * _e318.member.w);
                            if (_e7363 == _e7363) {
                                let _e7368 = (_e7364 == _e7364);
                                if _e7368 {
                                    phi_11199_ = select(true, false, (_e7365 == _e7365));
                                } else {
                                    phi_11199_ = false;
                                }
                                let _e7372 = phi_11199_;
                                phi_11200_ = select(true, false, _e7368);
                                phi_11201_ = _e7372;
                            } else {
                                phi_11200_ = true;
                                phi_11201_ = false;
                            }
                            let _e7375 = phi_11200_;
                            let _e7377 = phi_11201_;
                            let _e7380 = select(vec3<f32>(_e7363, _e7364, _e7365), vec3<f32>(0f, 0f, 0f), vec3(select(_e7377, true, _e7375)));
                            phi_11222_ = vec4<f32>(_e7380.x, _e7380.y, _e7380.z, _e318.member_3.w);
                            phi_11223_ = type_30(_e318.member, _e318.member_1, _e318.member_2, _e318.member_3, vec4<u32>(_e318.member_4.x, (_e318.member_4.y | 8u), _e318.member_4.z, _e318.member_4.w));
                        } else {
                            phi_11222_ = vec4<f32>(0f, 0f, 0f, 0f);
                            phi_11223_ = _e318;
                        }
                        let _e7399 = phi_11222_;
                        let _e7401 = phi_11223_;
                        phi_19546_ = _e7399;
                        phi_19547_ = type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        phi_19548_ = type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        phi_19549_ = _e7401;
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
                        local_68 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                        let _e377 = select(0u, 1u, (_e370 > 0u));
                        phi_11317_ = _e370;
                        phi_11320_ = _e377;
                        phi_11322_ = _e374;
                        phi_11324_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_11326_ = type_59(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                        loop {
                            let _e379 = phi_11317_;
                            let _e381 = phi_11320_;
                            let _e383 = phi_11322_;
                            let _e385 = phi_11324_;
                            let _e387 = phi_11326_;
                            local_69 = _e387;
                            local_70 = _e385;
                            local_71 = _e385;
                            local_72 = _e385;
                            local_73 = _e387;
                            local_105 = _e387;
                            if (_e379 < _e383) {
                                let _e556 = global_2.member[_e379];
                                if (_e556.member_6.x == 0u) {
                                    let _e662 = (_e556.member_1.x - _e556.member.x);
                                    let _e663 = (_e556.member_1.y - _e556.member.y);
                                    let _e664 = (_e556.member_1.z - _e556.member.z);
                                    let _e669 = (_e556.member_2.x - _e556.member.x);
                                    let _e670 = (_e556.member_2.y - _e556.member.y);
                                    let _e671 = (_e556.member_2.z - _e556.member.z);
                                    let _e674 = ((_e318.member_1.y * _e671) - (_e670 * _e318.member_1.z));
                                    let _e677 = ((_e318.member_1.z * _e669) - (_e671 * _e318.member_1.x));
                                    let _e680 = ((_e318.member_1.x * _e670) - (_e669 * _e318.member_1.y));
                                    let _e685 = (((_e662 * _e674) + (_e663 * _e677)) + (_e664 * _e680));
                                    if (abs(_e685) < 0.000000000001f) {
                                        phi_11484_ = vec4<f32>();
                                        phi_11485_ = true;
                                    } else {
                                        let _e688 = (1f / _e685);
                                        let _e689 = (_e318.member.x - _e556.member.x);
                                        let _e690 = (_e318.member.y - _e556.member.y);
                                        let _e691 = (_e318.member.z - _e556.member.z);
                                        let _e697 = ((((_e689 * _e674) + (_e690 * _e677)) + (_e691 * _e680)) * _e688);
                                        if (_e697 < 0f) {
                                            phi_11474_ = vec4<f32>();
                                            phi_11475_ = true;
                                            phi_11476_ = false;
                                        } else {
                                            let _e699 = (_e697 > 1f);
                                            if _e699 {
                                                phi_11472_ = vec4<f32>();
                                                phi_11473_ = false;
                                            } else {
                                                let _e702 = ((_e690 * _e664) - (_e663 * _e691));
                                                let _e705 = ((_e691 * _e662) - (_e664 * _e689));
                                                let _e708 = ((_e689 * _e663) - (_e662 * _e690));
                                                let _e714 = ((((_e318.member_1.x * _e702) + (_e318.member_1.y * _e705)) + (_e318.member_1.z * _e708)) * _e688);
                                                if (_e714 < 0f) {
                                                    phi_11462_ = vec4<f32>();
                                                    phi_11463_ = true;
                                                    phi_11464_ = false;
                                                } else {
                                                    let _e717 = ((_e697 + _e714) > 1f);
                                                    if _e717 {
                                                        phi_11460_ = vec4<f32>();
                                                        phi_11461_ = false;
                                                    } else {
                                                        let _e723 = ((((_e669 * _e702) + (_e670 * _e705)) + (_e671 * _e708)) * _e688);
                                                        if (_e723 <= 0f) {
                                                            phi_11455_ = vec4<f32>();
                                                            phi_11456_ = true;
                                                        } else {
                                                            let _e725 = (_e723 >= _e387.member);
                                                            if _e725 {
                                                                phi_11454_ = vec4<f32>();
                                                            } else {
                                                                phi_11454_ = vec4<f32>(_e723, _e697, _e714, 1f);
                                                            }
                                                            let _e728 = phi_11454_;
                                                            phi_11455_ = _e728;
                                                            phi_11456_ = _e725;
                                                        }
                                                        let _e730 = phi_11455_;
                                                        let _e732 = phi_11456_;
                                                        phi_11460_ = _e730;
                                                        phi_11461_ = _e732;
                                                    }
                                                    let _e734 = phi_11460_;
                                                    let _e736 = phi_11461_;
                                                    phi_11462_ = _e734;
                                                    phi_11463_ = _e717;
                                                    phi_11464_ = _e736;
                                                }
                                                let _e738 = phi_11462_;
                                                let _e740 = phi_11463_;
                                                let _e742 = phi_11464_;
                                                phi_11472_ = _e738;
                                                phi_11473_ = select(_e742, true, _e740);
                                            }
                                            let _e745 = phi_11472_;
                                            let _e747 = phi_11473_;
                                            phi_11474_ = _e745;
                                            phi_11475_ = _e699;
                                            phi_11476_ = _e747;
                                        }
                                        let _e749 = phi_11474_;
                                        let _e751 = phi_11475_;
                                        let _e753 = phi_11476_;
                                        phi_11484_ = _e749;
                                        phi_11485_ = select(_e753, true, _e751);
                                    }
                                    let _e756 = phi_11484_;
                                    let _e758 = phi_11485_;
                                    phi_11623_ = _e756;
                                    phi_11624_ = _e758;
                                } else {
                                    if (_e556.member_6.x == 1u) {
                                        let _e584 = (_e318.member.x - _e556.member.x);
                                        let _e585 = (_e318.member.y - _e556.member.y);
                                        let _e586 = (_e318.member.z - _e556.member.z);
                                        let _e591 = (((_e584 * _e318.member_1.x) + (_e585 * _e318.member_1.y)) + (_e586 * _e318.member_1.z));
                                        if (_e591 > 0f) {
                                            phi_11522_ = select(true, false, ((((_e584 * _e584) + (_e585 * _e585)) + (_e586 * _e586)) > (_e556.member.w * _e556.member.w)));
                                        } else {
                                            phi_11522_ = true;
                                        }
                                        let _e604 = phi_11522_;
                                        if _e604 {
                                            let _e608 = (_e584 - (_e318.member_1.x * _e591));
                                            let _e609 = (_e585 - (_e318.member_1.y * _e591));
                                            let _e610 = (_e586 - (_e318.member_1.z * _e591));
                                            let _e619 = ((_e556.member.w * _e556.member.w) - (((_e608 * _e608) + (_e609 * _e609)) + (_e610 * _e610)));
                                            if (_e619 < 0f) {
                                                phi_11578_ = vec4<f32>();
                                                phi_11579_ = true;
                                            } else {
                                                let _e621 = sqrt(_e619);
                                                let _e622 = -(_e591);
                                                let _e623 = (_e622 - _e621);
                                                if (_e623 <= 0f) {
                                                    phi_11556_ = true;
                                                } else {
                                                    phi_11556_ = (_e623 >= _e387.member);
                                                }
                                                let _e627 = phi_11556_;
                                                if _e627 {
                                                    phi_11562_ = (_e622 + _e621);
                                                } else {
                                                    phi_11562_ = _e623;
                                                }
                                                let _e630 = phi_11562_;
                                                if (_e630 <= 0f) {
                                                    phi_11573_ = vec4<f32>();
                                                    phi_11574_ = true;
                                                } else {
                                                    let _e632 = (_e630 >= _e387.member);
                                                    if _e632 {
                                                        phi_11572_ = vec4<f32>();
                                                    } else {
                                                        phi_11572_ = vec4<f32>(_e630, 0f, 0f, 1f);
                                                    }
                                                    let _e635 = phi_11572_;
                                                    phi_11573_ = _e635;
                                                    phi_11574_ = _e632;
                                                }
                                                let _e637 = phi_11573_;
                                                let _e639 = phi_11574_;
                                                phi_11578_ = _e637;
                                                phi_11579_ = _e639;
                                            }
                                            let _e641 = phi_11578_;
                                            let _e643 = phi_11579_;
                                            phi_11583_ = _e641;
                                            phi_11584_ = _e643;
                                        } else {
                                            phi_11583_ = vec4<f32>();
                                            phi_11584_ = true;
                                        }
                                        let _e645 = phi_11583_;
                                        let _e647 = phi_11584_;
                                        phi_11613_ = _e645;
                                        phi_11614_ = _e647;
                                    } else {
                                        if (abs(_e318.member_1.y) < 0.000000000001f) {
                                            phi_11611_ = vec4<f32>();
                                            phi_11612_ = true;
                                        } else {
                                            let _e566 = ((_e556.member.x - _e318.member.y) / _e318.member_1.y);
                                            if (_e566 <= 0f) {
                                                phi_11606_ = vec4<f32>();
                                                phi_11607_ = true;
                                            } else {
                                                let _e568 = (_e566 >= _e387.member);
                                                if _e568 {
                                                    phi_11605_ = vec4<f32>();
                                                } else {
                                                    phi_11605_ = vec4<f32>(_e566, 0f, 0f, 1f);
                                                }
                                                let _e571 = phi_11605_;
                                                phi_11606_ = _e571;
                                                phi_11607_ = _e568;
                                            }
                                            let _e573 = phi_11606_;
                                            let _e575 = phi_11607_;
                                            phi_11611_ = _e573;
                                            phi_11612_ = _e575;
                                        }
                                        let _e577 = phi_11611_;
                                        let _e579 = phi_11612_;
                                        phi_11613_ = _e577;
                                        phi_11614_ = _e579;
                                    }
                                    let _e649 = phi_11613_;
                                    let _e651 = phi_11614_;
                                    phi_11623_ = _e649;
                                    phi_11624_ = select(true, false, select(true, false, _e651));
                                }
                                let _e760 = phi_11623_;
                                let _e762 = phi_11624_;
                                let _e765 = select(vec4<f32>(0f, 0f, 0f, 0f), _e760, vec4(select(true, false, _e762)));
                                let _e767 = (_e765.w > 0f);
                                if _e767 {
                                    let _e773 = type_59(_e765.x, _e387.member_1, _e387.member_2, _e387.member_3);
                                    let _e778 = type_59(_e773.member, _e379, _e773.member_2, _e773.member_3);
                                    phi_11645_ = _e778;
                                    phi_11647_ = _e778;
                                } else {
                                    phi_11645_ = type_59();
                                    phi_11647_ = _e387;
                                }
                                let _e780 = phi_11645_;
                                let _e782 = phi_11647_;
                                phi_11318_ = (_e379 + 1u);
                                phi_11321_ = _e381;
                                phi_11323_ = _e383;
                                phi_11325_ = select(_e385, _e765, vec4(_e767));
                                phi_11327_ = _e782;
                                phi_11726_ = _e780;
                                phi_11727_ = true;
                                phi_11728_ = false;
                            } else {
                                let _e389 = (_e381 == 0u);
                                if _e389 {
                                    phi_11722_ = u32();
                                    phi_11723_ = u32();
                                    phi_11724_ = u32();
                                } else {
                                    let _e390 = (_e381 - 1u);
                                    let _e391 = (_e390 < 100u);
                                    if _e391 {
                                    } else {
                                        phi_32214_ = true;
                                        phi_20547_ = bool();
                                        phi_20546_ = type_59();
                                        break;
                                    }
                                    let _e393 = local_68[_e390];
                                    let _e395 = global_3.member[_e393];
                                    let _e406 = ((_e395.member.x - _e318.member.x) * _e375.x);
                                    let _e408 = ((_e395.member.y - _e318.member.y) * _e375.y);
                                    let _e410 = ((_e395.member.z - _e318.member.z) * _e375.z);
                                    let _e417 = ((_e395.member_2.x - _e318.member.x) * _e375.x);
                                    let _e418 = ((_e395.member_2.y - _e318.member.y) * _e375.y);
                                    let _e419 = ((_e395.member_2.z - _e318.member.z) * _e375.z);
                                    if (_e406 == _e406) {
                                        let _e421 = (_e417 == _e417);
                                        if _e421 {
                                            let _e422 = (_e406 != _e406);
                                            if _e422 {
                                                phi_24949_ = true;
                                            } else {
                                                phi_24949_ = (_e417 <= _e406);
                                            }
                                            let _e425 = phi_24949_;
                                            let _e426 = select(_e406, _e417, _e425);
                                            if _e422 {
                                                phi_24979_ = true;
                                            } else {
                                                phi_24979_ = (_e417 >= _e406);
                                            }
                                            let _e431 = phi_24979_;
                                            let _e432 = select(_e406, _e417, _e431);
                                            phi_24926_ = type_49(select(-300000000000000000000000000000000000000f, _e426, (_e426 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e432, (_e432 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_24926_ = type_49();
                                        }
                                        let _e437 = phi_24926_;
                                        phi_24929_ = _e437;
                                        phi_24930_ = select(true, false, _e421);
                                    } else {
                                        phi_24929_ = type_49();
                                        phi_24930_ = true;
                                    }
                                    let _e440 = phi_24929_;
                                    let _e442 = phi_24930_;
                                    if _e442 {
                                        phi_24935_ = type_49(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_24935_ = _e440;
                                    }
                                    let _e444 = phi_24935_;
                                    if (_e408 == _e408) {
                                        let _e448 = (_e418 == _e418);
                                        if _e448 {
                                            let _e449 = (_e408 != _e408);
                                            if _e449 {
                                                phi_25036_ = true;
                                            } else {
                                                phi_25036_ = (_e418 <= _e408);
                                            }
                                            let _e452 = phi_25036_;
                                            let _e453 = select(_e408, _e418, _e452);
                                            if (_e444.member != _e444.member) {
                                                phi_25051_ = true;
                                            } else {
                                                phi_25051_ = (_e453 >= _e444.member);
                                            }
                                            let _e457 = phi_25051_;
                                            if _e449 {
                                                phi_25066_ = true;
                                            } else {
                                                phi_25066_ = (_e418 >= _e408);
                                            }
                                            let _e461 = phi_25066_;
                                            let _e462 = select(_e408, _e418, _e461);
                                            if (_e444.member_1 != _e444.member_1) {
                                                phi_25081_ = true;
                                            } else {
                                                phi_25081_ = (_e462 <= _e444.member_1);
                                            }
                                            let _e466 = phi_25081_;
                                            phi_25013_ = type_49(select(_e444.member, _e453, _e457), select(_e444.member_1, _e462, _e466));
                                        } else {
                                            phi_25013_ = type_49();
                                        }
                                        let _e470 = phi_25013_;
                                        phi_25016_ = _e470;
                                        phi_25017_ = select(true, false, _e448);
                                    } else {
                                        phi_25016_ = type_49();
                                        phi_25017_ = true;
                                    }
                                    let _e473 = phi_25016_;
                                    let _e475 = phi_25017_;
                                    if _e475 {
                                        phi_25022_ = _e444;
                                    } else {
                                        phi_25022_ = _e473;
                                    }
                                    let _e477 = phi_25022_;
                                    if (_e410 == _e410) {
                                        let _e481 = (_e419 == _e419);
                                        if _e481 {
                                            let _e482 = (_e410 != _e410);
                                            if _e482 {
                                                phi_25123_ = true;
                                            } else {
                                                phi_25123_ = (_e419 <= _e410);
                                            }
                                            let _e485 = phi_25123_;
                                            let _e486 = select(_e410, _e419, _e485);
                                            if (_e477.member != _e477.member) {
                                                phi_25138_ = true;
                                            } else {
                                                phi_25138_ = (_e486 >= _e477.member);
                                            }
                                            let _e490 = phi_25138_;
                                            if _e482 {
                                                phi_25153_ = true;
                                            } else {
                                                phi_25153_ = (_e419 >= _e410);
                                            }
                                            let _e494 = phi_25153_;
                                            let _e495 = select(_e410, _e419, _e494);
                                            if (_e477.member_1 != _e477.member_1) {
                                                phi_25168_ = true;
                                            } else {
                                                phi_25168_ = (_e495 <= _e477.member_1);
                                            }
                                            let _e499 = phi_25168_;
                                            phi_25100_ = type_49(select(_e477.member, _e486, _e490), select(_e477.member_1, _e495, _e499));
                                        } else {
                                            phi_25100_ = type_49();
                                        }
                                        let _e503 = phi_25100_;
                                        phi_25103_ = _e503;
                                        phi_25104_ = select(true, false, _e481);
                                    } else {
                                        phi_25103_ = type_49();
                                        phi_25104_ = true;
                                    }
                                    let _e506 = phi_25103_;
                                    let _e508 = phi_25104_;
                                    if _e508 {
                                        phi_25109_ = _e477;
                                    } else {
                                        phi_25109_ = _e506;
                                    }
                                    let _e510 = phi_25109_;
                                    if (_e510.member_1 < 0f) {
                                        phi_24905_ = bool();
                                        phi_24906_ = true;
                                    } else {
                                        let _e514 = (_e510.member > _e510.member_1);
                                        if _e514 {
                                            phi_24902_ = bool();
                                        } else {
                                            phi_24902_ = (_e510.member > _e387.member);
                                        }
                                        let _e517 = phi_24902_;
                                        phi_24905_ = _e517;
                                        phi_24906_ = _e514;
                                    }
                                    let _e519 = phi_24905_;
                                    let _e521 = phi_24906_;
                                    if !(select(_e519, true, _e521)) {
                                        if (_e395.member_3 > 0u) {
                                            phi_11716_ = _e395.member_1;
                                            phi_11717_ = _e390;
                                            phi_11718_ = (_e395.member_1 + _e395.member_3);
                                        } else {
                                            let _e526 = (_e381 + 1u);
                                            let _e527 = (_e526 <= 100u);
                                            if _e527 {
                                                if _e391 {
                                                } else {
                                                    phi_32214_ = true;
                                                    phi_20547_ = bool();
                                                    phi_20546_ = type_59();
                                                    break;
                                                }
                                                local_68[_e390] = _e395.member_1;
                                                if (_e381 < 100u) {
                                                } else {
                                                    phi_32214_ = true;
                                                    phi_20547_ = bool();
                                                    phi_20546_ = type_59();
                                                    break;
                                                }
                                                local_68[_e381] = (_e393 + 1u);
                                            }
                                            phi_11716_ = _e379;
                                            phi_11717_ = select(_e390, _e526, _e527);
                                            phi_11718_ = _e383;
                                        }
                                        let _e536 = phi_11716_;
                                        let _e538 = phi_11717_;
                                        let _e540 = phi_11718_;
                                        phi_11719_ = _e536;
                                        phi_11720_ = _e538;
                                        phi_11721_ = _e540;
                                    } else {
                                        phi_11719_ = _e379;
                                        phi_11720_ = _e390;
                                        phi_11721_ = _e383;
                                    }
                                    let _e542 = phi_11719_;
                                    let _e544 = phi_11720_;
                                    let _e546 = phi_11721_;
                                    phi_11722_ = _e542;
                                    phi_11723_ = _e544;
                                    phi_11724_ = _e546;
                                }
                                let _e548 = phi_11722_;
                                let _e550 = phi_11723_;
                                let _e552 = phi_11724_;
                                phi_11318_ = _e548;
                                phi_11321_ = _e550;
                                phi_11323_ = _e552;
                                phi_11325_ = _e385;
                                phi_11327_ = _e387;
                                phi_11726_ = type_59();
                                phi_11727_ = select(true, false, _e389);
                                phi_11728_ = _e389;
                            }
                            let _e787 = phi_11318_;
                            let _e789 = phi_11321_;
                            let _e791 = phi_11323_;
                            let _e793 = phi_11325_;
                            let _e795 = phi_11327_;
                            let _e797 = phi_11726_;
                            let _e799 = phi_11727_;
                            let _e801 = phi_11728_;
                            phi_11317_ = _e787;
                            phi_11320_ = _e789;
                            phi_11322_ = _e791;
                            phi_11324_ = _e793;
                            phi_11326_ = _e795;
                            phi_32214_ = false;
                            phi_20547_ = _e801;
                            phi_20546_ = _e797;
                            if !(_e799) {
                                break;
                            }
                        }
                        let _e804 = phi_32214_;
                        let _e806 = phi_20547_;
                        let _e808 = phi_20546_;
                        if _e804 {
                            break;
                        }
                        if _e806 {
                            let _e810 = local_69;
                            if (_e810.member_1 != 4294967295u) {
                                let _e814 = global_2.member[_e810.member_1];
                                if (_e814.member_6.x == 0u) {
                                    let _e856 = (_e814.member_1.x - _e814.member.x);
                                    let _e857 = (_e814.member_1.y - _e814.member.y);
                                    let _e858 = (_e814.member_1.z - _e814.member.z);
                                    let _e863 = (_e814.member_2.x - _e814.member.x);
                                    let _e864 = (_e814.member_2.y - _e814.member.y);
                                    let _e865 = (_e814.member_2.z - _e814.member.z);
                                    let _e868 = ((_e857 * _e865) - (_e864 * _e858));
                                    let _e871 = ((_e858 * _e863) - (_e865 * _e856));
                                    let _e874 = ((_e856 * _e864) - (_e863 * _e857));
                                    let _e881 = (1f / sqrt((((_e868 * _e868) + (_e871 * _e871)) + (_e874 * _e874))));
                                    let _e885 = vec3<f32>((_e868 * _e881), (_e871 * _e881), (_e874 * _e881));
                                    if (_e814.member_6.w == 1u) {
                                        let _e894 = local_71;
                                        let _e898 = local_72;
                                        let _e900 = ((1f - _e894.y) - _e898.z);
                                        let _e921 = (((_e814.member_3.x * _e900) + (_e814.member_4.x * _e894.y)) + (_e814.member_5.x * _e898.z));
                                        let _e922 = (((_e814.member_3.y * _e900) + (_e814.member_4.y * _e894.y)) + (_e814.member_5.y * _e898.z));
                                        let _e923 = (((_e814.member_3.z * _e900) + (_e814.member_4.z * _e894.y)) + (_e814.member_5.z * _e898.z));
                                        let _e928 = (((_e921 * _e921) + (_e922 * _e922)) + (_e923 * _e923));
                                        if (_e928 > 0f) {
                                            let _e931 = (1f / sqrt(_e928));
                                            phi_11855_ = vec3<f32>((_e921 * _e931), (_e922 * _e931), (_e923 * _e931));
                                        } else {
                                            phi_11855_ = _e885;
                                        }
                                        let _e937 = phi_11855_;
                                        phi_11856_ = _e937;
                                    } else {
                                        phi_11856_ = _e885;
                                    }
                                    let _e939 = phi_11856_;
                                    phi_11890_ = type_53(_e885, _e939);
                                } else {
                                    if (_e814.member_6.x == 1u) {
                                        let _e824 = local_70;
                                        let _e844 = vec3<f32>((((_e318.member.x + (_e318.member_1.x * _e824.x)) - _e814.member.x) / _e814.member.w), (((_e318.member.y + (_e318.member_1.y * _e824.x)) - _e814.member.y) / _e814.member.w), (((_e318.member.z + (_e318.member_1.z * _e824.x)) - _e814.member.z) / _e814.member.w));
                                        phi_11889_ = type_53(_e844, _e844);
                                    } else {
                                        let _e821 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e318.member_1.y > 0f)));
                                        phi_11889_ = type_53(_e821, _e821);
                                    }
                                    let _e847 = phi_11889_;
                                    phi_11890_ = _e847;
                                }
                                let _e942 = phi_11890_;
                                let _e946 = local_73;
                                let _e951 = type_59(_e946.member, _e946.member_1, _e942.member, _e946.member_3);
                                phi_11895_ = type_59(_e951.member, _e951.member_1, _e951.member_2, _e942.member_1);
                            } else {
                                let _e7546 = local_105;
                                phi_11895_ = _e7546;
                            }
                            let _e958 = phi_11895_;
                            phi_11896_ = _e958;
                        } else {
                            phi_11896_ = type_59();
                        }
                        let _e960 = phi_11896_;
                        if select(true, false, _e806) {
                            phi_11901_ = _e808;
                        } else {
                            phi_11901_ = _e960;
                        }
                        let _e963 = phi_11901_;
                        let _e964 = (select(select(u32(_e318.member_3.w), 0u, (_e318.member_3.w < 0f)), 4294967295u, (_e318.member_3.w > 4294967000f)) + 1u);
                        if (_e963.member_1 == 4294967295u) {
                            if _e358 {
                                phi_12060_ = 1f;
                            } else {
                                if ((_e364 & 2u) == 0u) {
                                    phi_12059_ = 1f;
                                } else {
                                    let _e6806 = global_1.member.member_3[1u];
                                    let _e6809 = global_1.member.member_3[2u];
                                    let _e6812 = ((atan2(_e318.member_1.z, _e318.member_1.x) * 0.15915494f) + 0.5f);
                                    let _e6816 = select(_e318.member_1.y, -1f, (_e318.member_1.y < -1f));
                                    let _e6819 = acos(select(_e6816, 1f, (_e6816 > 1f)));
                                    let _e6821 = f32(_e6806);
                                    let _e6822 = ((_e6812 - floor(_e6812)) * _e6821);
                                    let _e6827 = select(select(u32(_e6822), 0u, (_e6822 < 0f)), 4294967295u, (_e6822 > 4294967000f));
                                    let _e6828 = (_e6806 - 1u);
                                    let _e6831 = f32(_e6809);
                                    let _e6832 = ((_e6819 * 0.31830987f) * _e6831);
                                    let _e6837 = select(select(u32(_e6832), 0u, (_e6832 < 0f)), 4294967295u, (_e6832 > 4294967000f));
                                    let _e6838 = (_e6809 - 1u);
                                    let _e6840 = select(_e6837, _e6838, (_e6838 < _e6837));
                                    let _e6841 = sin(_e6819);
                                    if (_e6841 <= 0f) {
                                        phi_12057_ = 0f;
                                    } else {
                                        let _e6845 = global_1.member.member_2[3u];
                                        let _e6848 = global_1.member.member_3[0u];
                                        let _e6852 = (_e6845 + _e6840);
                                        let _e6855 = global_5.member[(_e6852 + 1u)];
                                        let _e6857 = global_5.member[_e6852];
                                        let _e6860 = ((_e6848 + (_e6840 * (_e6806 + 1u))) + select(_e6827, _e6828, (_e6828 < _e6827)));
                                        let _e6863 = global_5.member[(_e6860 + 1u)];
                                        let _e6865 = global_5.member[_e6860];
                                        phi_12057_ = ((((_e6855 - _e6857) * _e6831) * ((_e6863 - _e6865) * _e6821)) / (19.73921f * _e6841));
                                    }
                                    let _e6872 = phi_12057_;
                                    if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                        phi_31221_ = 1f;
                                    } else {
                                        let _e6874 = (_e318.member_1.w * _e318.member_1.w);
                                        let _e6876 = (_e6874 + (_e6872 * _e6872));
                                        if (_e6876 == 0f) {
                                            phi_31218_ = 0f;
                                        } else {
                                            phi_31218_ = (_e6874 / _e6876);
                                        }
                                        let _e6880 = phi_31218_;
                                        phi_31221_ = _e6880;
                                    }
                                    let _e6882 = phi_31221_;
                                    phi_12059_ = _e6882;
                                }
                                let _e6884 = phi_12059_;
                                phi_12060_ = _e6884;
                            }
                            let _e6886 = phi_12060_;
                            if ((_e364 & 4u) == 0u) {
                                phi_12924_ = _e349;
                            } else {
                                if _e358 {
                                    phi_12113_ = 1f;
                                } else {
                                    let _e6897 = global_1.member.member_5;
                                    let _e6901 = (_e318.member_1.x - _e6897.x);
                                    let _e6902 = (_e318.member_1.y - _e6897.y);
                                    let _e6903 = (_e318.member_1.z - _e6897.z);
                                    let _e6910 = (sqrt((((_e6901 * _e6901) + (_e6902 * _e6902)) + (_e6903 * _e6903))) * 0.5f);
                                    if (_e6910 != _e6910) {
                                        phi_31247_ = true;
                                    } else {
                                        phi_31247_ = (1f <= _e6910);
                                    }
                                    let _e6914 = phi_31247_;
                                    let _e6920 = global_1.member.member_6[3u];
                                    if ((2f * asin(select(_e6910, 1f, _e6914))) >= _e6920) {
                                        phi_12111_ = 0f;
                                    } else {
                                        let _e6923 = sin((_e6920 * 0.5f));
                                        phi_12111_ = (1f / ((12.566371f * _e6923) * _e6923));
                                    }
                                    let _e6928 = phi_12111_;
                                    if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                        phi_31268_ = 1f;
                                    } else {
                                        let _e6930 = (_e318.member_1.w * _e318.member_1.w);
                                        let _e6932 = (_e6930 + (_e6928 * _e6928));
                                        if (_e6932 == 0f) {
                                            phi_31265_ = 0f;
                                        } else {
                                            phi_31265_ = (_e6930 / _e6932);
                                        }
                                        let _e6936 = phi_31265_;
                                        phi_31268_ = _e6936;
                                    }
                                    let _e6938 = phi_31268_;
                                    phi_12113_ = _e6938;
                                }
                                let _e6940 = phi_12113_;
                                let _e6942 = global_1.member.member_5;
                                let _e6946 = (_e318.member_1.x - _e6942.x);
                                let _e6947 = (_e318.member_1.y - _e6942.y);
                                let _e6948 = (_e318.member_1.z - _e6942.z);
                                let _e6955 = (sqrt((((_e6946 * _e6946) + (_e6947 * _e6947)) + (_e6948 * _e6948))) * 0.5f);
                                if (_e6955 != _e6955) {
                                    phi_31279_ = true;
                                } else {
                                    phi_31279_ = (1f <= _e6955);
                                }
                                let _e6959 = phi_31279_;
                                let _e6962 = (2f * asin(select(_e6955, 1f, _e6959)));
                                let _e6966 = global_1.member.member_6[3u];
                                if (_e6962 >= _e6966) {
                                    phi_12910_ = vec3<f32>(0f, 0f, 0f);
                                } else {
                                    let _e6970 = (1f - pow((_e6962 / _e6966), 2f));
                                    if (_e6970 != _e6970) {
                                        phi_31294_ = true;
                                    } else {
                                        phi_31294_ = (0f >= _e6970);
                                    }
                                    let _e6974 = phi_31294_;
                                    let _e6977 = global_1.member.member_6;
                                    let _e6983 = global_1.member.member_8[2u];
                                    if (_e6983 == 4294967295u) {
                                        phi_12883_ = vec3<f32>(1f, 1f, 1f);
                                    } else {
                                        let _e6986 = global_5.member[_e6983];
                                        let _e6989 = global_5.member[(_e6983 + 1u)];
                                        let _e6992 = global_5.member[(_e6983 + 2u)];
                                        let _e6995 = global_5.member[(_e6983 + 3u)];
                                        let _e6998 = global_5.member[(_e6983 + 4u)];
                                        let _e7001 = global_5.member[(_e6983 + 5u)];
                                        let _e7004 = global_5.member[(_e6983 + 6u)];
                                        let _e7007 = global_5.member[(_e6983 + 7u)];
                                        let _e7010 = global_5.member[(_e6983 + 8u)];
                                        let _e7013 = global_5.member[(_e6983 + 9u)];
                                        let _e7016 = global_5.member[(_e6983 + 13u)];
                                        let _e7019 = global_5.member[(_e6983 + 14u)];
                                        let _e7022 = global_5.member[(_e6983 + 15u)];
                                        let _e7025 = global_5.member[(_e6983 + 16u)];
                                        let _e7028 = global_5.member[(_e6983 + 18u)];
                                        let _e7031 = global_5.member[(_e6983 + 19u)];
                                        let _e7034 = global_5.member[(_e6983 + 20u)];
                                        let _e7037 = global_5.member[(_e6983 + 29u)];
                                        let _e7038 = bitcast<u32>(_e7037);
                                        let _e7051 = (((((_e318.member.x * _e318.member.x) + (_e318.member.y * _e318.member.y)) + (_e318.member.z * _e318.member.z)) - (2f * (((_e318.member.x * _e6986) + (_e318.member.y * _e6989)) + (_e318.member.z * _e6992)))) + _e7004);
                                        if (_e7051 > 0f) {
                                            let _e7060 = ((((_e318.member.x - _e6986) * _e7007) + ((_e318.member.y - _e6989) * _e7010)) + ((_e318.member.z - _e6992) * _e7013));
                                            let _e7062 = ((_e7060 * _e7060) - _e7051);
                                            if (_e7060 >= 0f) {
                                                phi_12583_ = vec3<f32>();
                                                phi_12584_ = true;
                                            } else {
                                                let _e7064 = (_e7062 <= 0f);
                                                if _e7064 {
                                                    phi_12582_ = vec3<f32>();
                                                } else {
                                                    let _e7067 = (-(_e7060) - sqrt(_e7062));
                                                    phi_12582_ = vec3<f32>((_e318.member.x + (_e7007 * _e7067)), (_e318.member.y + (_e7010 * _e7067)), (_e318.member.z + (_e7013 * _e7067)));
                                                }
                                                let _e7076 = phi_12582_;
                                                phi_12583_ = _e7076;
                                                phi_12584_ = _e7064;
                                            }
                                            let _e7078 = phi_12583_;
                                            let _e7080 = phi_12584_;
                                            phi_12589_ = _e7078;
                                            phi_12590_ = select(true, false, _e7080);
                                        } else {
                                            phi_12589_ = vec3<f32>(_e318.member.x, _e318.member.y, _e318.member.z);
                                            phi_12590_ = true;
                                        }
                                        let _e7083 = phi_12589_;
                                        let _e7085 = phi_12590_;
                                        if _e7085 {
                                            let _e7087 = (_e7083.x - _e6986);
                                            let _e7089 = (_e7083.y - _e6989);
                                            let _e7091 = (_e7083.z - _e6992);
                                            let _e7097 = sqrt((((_e7087 * _e7087) + (_e7089 * _e7089)) + (_e7091 * _e7091)));
                                            let _e7098 = (1f / _e7097);
                                            let _e7116 = ((((((_e7083.x * _e7083.x) + (_e7083.y * _e7083.y)) + (_e7083.z * _e7083.z)) - (2f * (((_e7083.x * _e6986) + (_e7083.y * _e6989)) + (_e7083.z * _e6992)))) + _e7001) / (_e7097 + _e6995));
                                            if (_e7116 != _e7116) {
                                                phi_31418_ = true;
                                            } else {
                                                phi_31418_ = (0f >= _e7116);
                                            }
                                            let _e7120 = phi_31418_;
                                            let _e7124 = sqrt((select(_e7116, 0f, _e7120) / (_e6998 - _e6995)));
                                            if (_e7124 != _e7124) {
                                                phi_31433_ = true;
                                            } else {
                                                phi_31433_ = (1f <= _e7124);
                                            }
                                            let _e7128 = phi_31433_;
                                            let _e7130 = (select(_e7124, 1f, _e7128) * 63f);
                                            let _e7135 = ((((_e7087 * _e7098) * _e7007) + ((_e7089 * _e7098) * _e7010)) + ((_e7091 * _e7098) * _e7013));
                                            let _e7137 = select(_e7135, -1f, (_e7135 < -1f));
                                            let _e7141 = ((select(_e7137, 1f, (_e7137 > 1f)) + 1f) * 127.5f);
                                            let _e7146 = select(select(u32(_e7130), 0u, (_e7130 < 0f)), 4294967295u, (_e7130 > 4294967000f));
                                            let _e7148 = select(_e7146, 62u, (62u < _e7146));
                                            let _e7153 = select(select(u32(_e7141), 0u, (_e7141 < 0f)), 4294967295u, (_e7141 > 4294967000f));
                                            let _e7155 = select(_e7153, 254u, (254u < _e7153));
                                            let _e7157 = (_e7130 - f32(_e7148));
                                            let _e7159 = (_e7141 - f32(_e7155));
                                            let _e7163 = (_e7038 + (3u * ((_e7148 * 256u) + _e7155)));
                                            let _e7168 = (_e7038 + (3u * (((_e7148 + 1u) * 256u) + _e7155)));
                                            let _e7170 = global_5.member[_e7163];
                                            let _e7173 = global_5.member[(_e7163 + 1u)];
                                            let _e7176 = global_5.member[(_e7163 + 2u)];
                                            let _e7177 = (1f - _e7159);
                                            let _e7183 = global_5.member[(_e7163 + 3u)];
                                            let _e7186 = global_5.member[(_e7163 + 4u)];
                                            let _e7189 = global_5.member[(_e7163 + 5u)];
                                            let _e7196 = (1f - _e7157);
                                            let _e7201 = global_5.member[_e7168];
                                            let _e7204 = global_5.member[(_e7168 + 1u)];
                                            let _e7207 = global_5.member[(_e7168 + 2u)];
                                            let _e7213 = global_5.member[(_e7168 + 3u)];
                                            let _e7216 = global_5.member[(_e7168 + 4u)];
                                            let _e7219 = global_5.member[(_e7168 + 5u)];
                                            phi_12881_ = vec3<f32>(((((_e7170 * _e7177) + (_e7183 * _e7159)) * _e7196) + (((_e7201 * _e7177) + (_e7213 * _e7159)) * _e7157)), ((((_e7173 * _e7177) + (_e7186 * _e7159)) * _e7196) + (((_e7204 * _e7177) + (_e7216 * _e7159)) * _e7157)), ((((_e7176 * _e7177) + (_e7189 * _e7159)) * _e7196) + (((_e7207 * _e7177) + (_e7219 * _e7159)) * _e7157)));
                                        } else {
                                            phi_12881_ = vec3<f32>(0f, 0f, 0f);
                                        }
                                        let _e7234 = phi_12881_;
                                        let _e7241 = ((_e7025 * 1.1f) * _e7234.y);
                                        phi_12883_ = vec3<f32>(exp(-((((_e7016 * _e7234.x) + _e7241) + (_e7028 * _e7234.z)))), exp(-((((_e7019 * _e7234.x) + _e7241) + (_e7031 * _e7234.z)))), exp(-((((_e7022 * _e7234.x) + _e7241) + (_e7034 * _e7234.z)))));
                                    }
                                    let _e7260 = phi_12883_;
                                    let _e7272 = sin((_e6966 * 0.5f));
                                    let _e7275 = (((1f - (0.6f * (1f - sqrt(select(_e6970, 0f, _e6974))))) * 1.25f) / ((12.566371f * _e7272) * _e7272));
                                    phi_12910_ = vec3<f32>(((_e6977.x * _e7260.x) * _e7275), ((_e6977.y * _e7260.y) * _e7275), ((_e6977.z * _e7260.z) * _e7275));
                                }
                                let _e7281 = phi_12910_;
                                phi_12924_ = vec3<f32>((_e318.member_3.x + ((_e318.member_2.x * _e7281.x) * _e6940)), (_e318.member_3.y + ((_e318.member_2.y * _e7281.y) * _e6940)), (_e318.member_3.z + ((_e318.member_2.z * _e7281.z) * _e6940)));
                            }
                            let _e7296 = phi_12924_;
                            phi_19494_ = type_64(type_30(_e318.member, _e318.member_1, vec4<f32>(_e318.member_2.x, _e318.member_2.y, _e318.member_2.z, f32(_e345)), vec4<f32>(_e7296.x, _e7296.y, _e7296.z, f32(_e964)), vec4<u32>(_e318.member_4.x, (_e318.member_4.y & 4294967294u), 0u, 0u)), type_34(vec4<f32>(_e318.member.x, _e318.member.y, _e318.member.z, -1f), vec4<f32>(_e318.member_1.x, _e318.member_1.y, _e318.member_1.z, 0f), vec4<f32>((_e318.member_2.x * _e6886), (_e318.member_2.y * _e6886), (_e318.member_2.z * _e6886), 0f)), type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_19495_ = type_64();
                            phi_19496_ = true;
                        } else {
                            if ((_e364 & 8u) == 0u) {
                                phi_34510_ = _e804;
                                phi_13461_ = _e339;
                                phi_13462_ = type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            } else {
                                if (_e963.member > 100f) {
                                    let _e977 = global_1.member.member_8[1u];
                                    let _e979 = global_5.member[_e977];
                                    let _e982 = global_5.member[(_e977 + 1u)];
                                    let _e985 = global_5.member[(_e977 + 2u)];
                                    let _e988 = global_5.member[(_e977 + 3u)];
                                    let _e991 = global_5.member[(_e977 + 5u)];
                                    let _e994 = global_5.member[(_e977 + 6u)];
                                    let _e997 = global_5.member[(_e977 + 10u)];
                                    let _e1000 = global_5.member[(_e977 + 11u)];
                                    let _e1003 = global_5.member[(_e977 + 13u)];
                                    let _e1006 = global_5.member[(_e977 + 14u)];
                                    let _e1009 = global_5.member[(_e977 + 15u)];
                                    let _e1012 = global_5.member[(_e977 + 16u)];
                                    let _e1015 = global_5.member[(_e977 + 18u)];
                                    let _e1018 = global_5.member[(_e977 + 19u)];
                                    let _e1021 = global_5.member[(_e977 + 20u)];
                                    let _e1029 = ((((_e318.member.x - _e979) * _e318.member_1.x) + ((_e318.member.y - _e982) * _e318.member_1.y)) + ((_e318.member.z - _e985) * _e318.member_1.z));
                                    let _e1041 = ((((_e318.member.x * _e318.member.x) + (_e318.member.y * _e318.member.y)) + (_e318.member.z * _e318.member.z)) - (2f * (((_e318.member.x * _e979) + (_e318.member.y * _e982)) + (_e318.member.z * _e985))));
                                    let _e1042 = (_e1041 + _e994);
                                    let _e1043 = (_e1029 * _e1029);
                                    let _e1044 = (_e1043 - _e1042);
                                    if (_e1044 < 0f) {
                                        phi_25315_ = type_50(0u, type_50().member_1);
                                        phi_25316_ = type_50();
                                        phi_25317_ = true;
                                    } else {
                                        if (_e1029 < 0f) {
                                            phi_25287_ = -(sqrt(_e1044));
                                        } else {
                                            phi_25287_ = sqrt(_e1044);
                                        }
                                        let _e1051 = phi_25287_;
                                        let _e1053 = -((_e1029 + _e1051));
                                        let _e1054 = (_e1053 == 0f);
                                        if _e1054 {
                                            phi_25310_ = type_50(1u, type_49(0f, 0f));
                                            phi_25311_ = type_50();
                                        } else {
                                            let _e1055 = (_e1042 / _e1053);
                                            if (_e1053 < _e1055) {
                                                phi_25299_ = type_49(_e1053, _e1055);
                                            } else {
                                                phi_25299_ = type_49(_e1055, _e1053);
                                            }
                                            let _e1060 = phi_25299_;
                                            phi_25310_ = type_50();
                                            phi_25311_ = type_50(1u, _e1060);
                                        }
                                        let _e1063 = phi_25310_;
                                        let _e1065 = phi_25311_;
                                        phi_25315_ = _e1063;
                                        phi_25316_ = _e1065;
                                        phi_25317_ = _e1054;
                                    }
                                    let _e1070 = phi_25315_;
                                    let _e1072 = phi_25316_;
                                    let _e1074 = phi_25317_;
                                    if _e1074 {
                                        phi_25321_ = _e1070;
                                    } else {
                                        phi_25321_ = _e1072;
                                    }
                                    let _e1076 = phi_25321_;
                                    if (bitcast<i32>(_e1076.member) != 0i) {
                                        phi_25208_ = type_50(0u, _e1076.member_1);
                                    } else {
                                        phi_25208_ = type_50(1u, type_50().member_1);
                                    }
                                    let _e1086 = phi_25208_;
                                    if (bitcast<i32>(_e1086.member) != 0i) {
                                        phi_25255_ = type_50(0u, type_50().member_1);
                                    } else {
                                        if (_e1086.member_1.member_1 <= 0f) {
                                            phi_25251_ = type_50(0u, type_50().member_1);
                                        } else {
                                            if (_e1086.member_1.member != _e1086.member_1.member) {
                                                phi_25359_ = true;
                                            } else {
                                                phi_25359_ = (0f >= _e1086.member_1.member);
                                            }
                                            let _e1098 = phi_25359_;
                                            let _e1099 = select(_e1086.member_1.member, 0f, _e1098);
                                            let _e1100 = (_e1041 + _e991);
                                            if (_e1100 > 0f) {
                                                let _e1102 = (_e1043 - _e1100);
                                                if (_e1102 < 0f) {
                                                    phi_25467_ = type_50(0u, type_50().member_1);
                                                    phi_25468_ = type_50();
                                                    phi_25469_ = true;
                                                } else {
                                                    if (_e1029 < 0f) {
                                                        phi_25439_ = -(sqrt(_e1102));
                                                    } else {
                                                        phi_25439_ = sqrt(_e1102);
                                                    }
                                                    let _e1109 = phi_25439_;
                                                    let _e1111 = -((_e1029 + _e1109));
                                                    let _e1112 = (_e1111 == 0f);
                                                    if _e1112 {
                                                        phi_25462_ = type_50(1u, type_49(0f, 0f));
                                                        phi_25463_ = type_50();
                                                    } else {
                                                        let _e1113 = (_e1100 / _e1111);
                                                        if (_e1111 < _e1113) {
                                                            phi_25451_ = type_49(_e1111, _e1113);
                                                        } else {
                                                            phi_25451_ = type_49(_e1113, _e1111);
                                                        }
                                                        let _e1118 = phi_25451_;
                                                        phi_25462_ = type_50();
                                                        phi_25463_ = type_50(1u, _e1118);
                                                    }
                                                    let _e1121 = phi_25462_;
                                                    let _e1123 = phi_25463_;
                                                    phi_25467_ = _e1121;
                                                    phi_25468_ = _e1123;
                                                    phi_25469_ = _e1112;
                                                }
                                                let _e1128 = phi_25467_;
                                                let _e1130 = phi_25468_;
                                                let _e1132 = phi_25469_;
                                                if _e1132 {
                                                    phi_25473_ = _e1128;
                                                } else {
                                                    phi_25473_ = _e1130;
                                                }
                                                let _e1134 = phi_25473_;
                                                if (bitcast<i32>(_e1134.member) != 0i) {
                                                    phi_25407_ = select(_e1134.member_1.member_1, _e1134.member_1.member, (_e1134.member_1.member >= 0f));
                                                } else {
                                                    phi_25407_ = -1f;
                                                }
                                                let _e1145 = phi_25407_;
                                                if (_e1145 >= 0f) {
                                                    if (_e1086.member_1.member_1 != _e1086.member_1.member_1) {
                                                        phi_25511_ = true;
                                                    } else {
                                                        phi_25511_ = (_e1145 <= _e1086.member_1.member_1);
                                                    }
                                                    let _e1150 = phi_25511_;
                                                    phi_25236_ = select(_e1086.member_1.member_1, _e1145, _e1150);
                                                } else {
                                                    phi_25236_ = _e1086.member_1.member_1;
                                                }
                                                let _e1153 = phi_25236_;
                                                phi_25238_ = _e1153;
                                            } else {
                                                phi_25238_ = _e1086.member_1.member_1;
                                            }
                                            let _e1155 = phi_25238_;
                                            if (_e1155 <= _e1099) {
                                                phi_25247_ = type_50(0u, type_50().member_1);
                                            } else {
                                                phi_25247_ = type_50(1u, type_49(_e1099, _e1155));
                                            }
                                            let _e1163 = phi_25247_;
                                            phi_25251_ = _e1163;
                                        }
                                        let _e1168 = phi_25251_;
                                        phi_25255_ = _e1168;
                                    }
                                    let _e1173 = phi_25255_;
                                    if (bitcast<i32>(_e1173.member) != 0i) {
                                        let _e1179 = (_e1173.member_1.member < _e963.member);
                                        if _e1179 {
                                            if (_e1173.member_1.member_1 != _e1173.member_1.member_1) {
                                                phi_25526_ = true;
                                            } else {
                                                phi_25526_ = (_e963.member <= _e1173.member_1.member_1);
                                            }
                                            let _e1185 = phi_25526_;
                                            let _e1186 = select(_e1173.member_1.member_1, _e963.member, _e1185);
                                            phi_13320_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13323_ = 0u;
                                            loop {
                                                let _e1188 = phi_13320_;
                                                let _e1190 = phi_13323_;
                                                local_74 = _e1188;
                                                local_75 = _e1188;
                                                local_76 = _e1188;
                                                let _e1191 = (_e1190 < 32u);
                                                if _e1191 {
                                                    let _e1192 = -(_e1029);
                                                    if (_e1173.member_1.member <= _e1186) {
                                                    } else {
                                                        phi_32287_ = true;
                                                        break;
                                                    }
                                                    let _e1195 = select(_e1192, _e1173.member_1.member, (_e1192 < _e1173.member_1.member));
                                                    let _e1197 = select(_e1195, _e1186, (_e1195 > _e1186));
                                                    let _e1198 = (_e1197 - _e1173.member_1.member);
                                                    let _e1199 = (_e1186 - _e1197);
                                                    let _e1200 = (_e1186 - _e1173.member_1.member);
                                                    let _e1201 = (0.000000001f * _e1200);
                                                    if (_e1198 <= _e1201) {
                                                        let _e1276 = (f32(_e1190) * 0.03125f);
                                                        let _e1279 = (f32((_e1190 + 1u)) * 0.03125f);
                                                        let _e1281 = ((_e1276 + _e1279) * 0.5f);
                                                        phi_13424_ = type_49((_e1173.member_1.member + ((_e1200 * _e1281) * _e1281)), (_e1200 * ((_e1279 * _e1279) - (_e1276 * _e1276))));
                                                    } else {
                                                        if (_e1199 <= _e1201) {
                                                            let _e1259 = (f32((31u - _e1190)) * 0.03125f);
                                                            let _e1262 = (f32((32u - _e1190)) * 0.03125f);
                                                            let _e1264 = ((_e1259 + _e1262) * 0.5f);
                                                            phi_13423_ = type_49((_e1186 - ((_e1200 * _e1264) * _e1264)), (_e1200 * ((_e1262 * _e1262) - (_e1259 * _e1259))));
                                                        } else {
                                                            let _e1207 = round(((32f * _e1198) / (_e1198 + _e1199)));
                                                            let _e1212 = select(select(u32(_e1207), 0u, (_e1207 < 0f)), 4294967295u, (_e1207 > 4294967000f));
                                                            if (_e1212 < 1u) {
                                                                phi_13407_ = 1u;
                                                            } else {
                                                                phi_13407_ = select(_e1212, 31u, (_e1212 > 31u));
                                                            }
                                                            let _e1217 = phi_13407_;
                                                            if (_e1190 < _e1217) {
                                                                let _e1238 = ((_e1217 - 1u) - _e1190);
                                                                let _e1240 = f32(_e1217);
                                                                let _e1241 = (f32(_e1238) / _e1240);
                                                                let _e1244 = (f32((_e1238 + 1u)) / _e1240);
                                                                let _e1246 = ((_e1241 + _e1244) * 0.5f);
                                                                phi_13422_ = type_49((_e1197 - ((_e1198 * _e1246) * _e1246)), (_e1198 * ((_e1244 * _e1244) - (_e1241 * _e1241))));
                                                            } else {
                                                                let _e1220 = (_e1190 - _e1217);
                                                                let _e1222 = f32((32u - _e1217));
                                                                let _e1223 = (f32(_e1220) / _e1222);
                                                                let _e1226 = (f32((_e1220 + 1u)) / _e1222);
                                                                let _e1228 = ((_e1223 + _e1226) * 0.5f);
                                                                phi_13422_ = type_49((_e1197 + ((_e1199 * _e1228) * _e1228)), (_e1199 * ((_e1226 * _e1226) - (_e1223 * _e1223))));
                                                            }
                                                            let _e1256 = phi_13422_;
                                                            phi_13423_ = _e1256;
                                                        }
                                                        let _e1274 = phi_13423_;
                                                        phi_13424_ = _e1274;
                                                    }
                                                    let _e1291 = phi_13424_;
                                                    let _e1297 = (_e318.member.x + (_e318.member_1.x * _e1291.member));
                                                    let _e1298 = (_e318.member.y + (_e318.member_1.y * _e1291.member));
                                                    let _e1299 = (_e318.member.z + (_e318.member_1.z * _e1291.member));
                                                    let _e1313 = (_e1297 - _e979);
                                                    let _e1314 = (_e1298 - _e982);
                                                    let _e1315 = (_e1299 - _e985);
                                                    let _e1323 = ((((((_e1297 * _e1297) + (_e1298 * _e1298)) + (_e1299 * _e1299)) - (2f * (((_e1297 * _e979) + (_e1298 * _e982)) + (_e1299 * _e985)))) + _e991) / (sqrt((((_e1313 * _e1313) + (_e1314 * _e1314)) + (_e1315 * _e1315))) + _e988));
                                                    let _e1327 = (1f - (abs((_e1323 - 25000f)) * 0.00006666667f));
                                                    if (_e1327 != _e1327) {
                                                        phi_25725_ = true;
                                                    } else {
                                                        phi_25725_ = (0f >= _e1327);
                                                    }
                                                    let _e1331 = phi_25725_;
                                                    let _e1333 = -(_e1323);
                                                    phi_13321_ = (_e1188 + vec3<f32>((exp((_e1333 / _e997)) * _e1291.member_1), (exp((_e1333 / _e1000)) * _e1291.member_1), (select(_e1327, 0f, _e1331) * _e1291.member_1)));
                                                    phi_13324_ = (_e1190 + 1u);
                                                } else {
                                                    phi_13321_ = vec3<f32>();
                                                    phi_13324_ = u32();
                                                }
                                                let _e1345 = phi_13321_;
                                                let _e1347 = phi_13324_;
                                                phi_13320_ = _e1345;
                                                phi_13323_ = _e1347;
                                                phi_32287_ = _e804;
                                                if !(_e1191) {
                                                    break;
                                                }
                                            }
                                            let _e1350 = phi_32287_;
                                            if _e1350 {
                                                break;
                                            }
                                            let _e1352 = local_74;
                                            let _e1359 = local_75;
                                            let _e1361 = ((_e1012 * 1.1f) * _e1359.y);
                                            let _e1366 = local_76;
                                            phi_34514_ = _e1350;
                                            phi_13449_ = vec3<f32>(exp(-((((_e1003 * _e1352.x) + _e1361) + (_e1015 * _e1366.z)))), exp(-((((_e1006 * _e1352.x) + _e1361) + (_e1018 * _e1366.z)))), exp(-((((_e1009 * _e1352.x) + _e1361) + (_e1021 * _e1366.z)))));
                                        } else {
                                            phi_34514_ = _e804;
                                            phi_13449_ = vec3<f32>();
                                        }
                                        let _e1382 = phi_34514_;
                                        let _e1384 = phi_13449_;
                                        phi_34513_ = _e1382;
                                        phi_13451_ = _e1384;
                                        phi_13452_ = select(true, false, _e1179);
                                    } else {
                                        phi_34513_ = _e804;
                                        phi_13451_ = vec3<f32>();
                                        phi_13452_ = true;
                                    }
                                    let _e1387 = phi_34513_;
                                    let _e1389 = phi_13451_;
                                    let _e1391 = phi_13452_;
                                    phi_34511_ = _e1387;
                                    phi_13459_ = (_e339 * select(_e1389, vec3<f32>(1f, 1f, 1f), vec3(_e1391)));
                                    phi_13460_ = type_34(vec4<f32>(_e318.member.x, _e318.member.y, _e318.member.z, _e963.member), vec4<f32>(_e318.member_1.x, _e318.member_1.y, _e318.member_1.z, 0f), vec4<f32>(_e318.member_2.x, _e318.member_2.y, _e318.member_2.z, 0f));
                                } else {
                                    phi_34511_ = _e804;
                                    phi_13459_ = _e339;
                                    phi_13460_ = type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                }
                                let _e1396 = phi_34511_;
                                let _e1398 = phi_13459_;
                                let _e1400 = phi_13460_;
                                phi_34510_ = _e1396;
                                phi_13461_ = _e1398;
                                phi_13462_ = _e1400;
                            }
                            let _e1402 = phi_34510_;
                            let _e1404 = phi_13461_;
                            let _e1406 = phi_13462_;
                            let _e1411 = (_e318.member.x + (_e318.member_1.x * _e963.member));
                            let _e1412 = (_e318.member.y + (_e318.member_1.y * _e963.member));
                            let _e1413 = (_e318.member.z + (_e318.member_1.z * _e963.member));
                            let _e1415 = global_2.member[_e963.member_1];
                            phi_13486_ = 0i;
                            phi_13489_ = _e1415.member_6.y;
                            loop {
                                let _e1419 = phi_13486_;
                                let _e1421 = phi_13489_;
                                local_77 = _e1421;
                                if (_e1419 < 4i) {
                                    let _e1424 = global_4.member[_e1421];
                                    let _e1426 = (_e1424.member != 6u);
                                    if _e1426 {
                                        phi_13558_ = i32();
                                        phi_13559_ = u32();
                                    } else {
                                        let _e1430 = (abs(_e1411) / _e1424.member_6.z);
                                        let _e1441 = (abs(_e1413) / _e1424.member_6.w);
                                        if ((select(select(u32(_e1430), 0u, (_e1430 < 0f)), 4294967295u, (_e1430 > 4294967000f)) % 2u) == 0u) {
                                            phi_13543_ = (_e1411 > 0f);
                                        } else {
                                            phi_13543_ = (_e1411 <= 0f);
                                        }
                                        let _e1452 = phi_13543_;
                                        if ((select(select(u32(_e1441), 0u, (_e1441 < 0f)), 4294967295u, (_e1441 > 4294967000f)) % 2u) == 0u) {
                                            phi_13549_ = (_e1413 > 0f);
                                        } else {
                                            phi_13549_ = (_e1413 <= 0f);
                                        }
                                        let _e1456 = phi_13549_;
                                        if (_e1452 != _e1456) {
                                            phi_13556_ = _e1424.member_1;
                                        } else {
                                            phi_13556_ = _e1424.member_2;
                                        }
                                        let _e1461 = phi_13556_;
                                        phi_13558_ = (_e1419 + 1i);
                                        phi_13559_ = _e1461;
                                    }
                                    let _e1464 = phi_13558_;
                                    let _e1466 = phi_13559_;
                                    phi_13487_ = _e1464;
                                    phi_13490_ = _e1466;
                                    phi_13561_ = select(true, false, _e1426);
                                } else {
                                    phi_13487_ = i32();
                                    phi_13490_ = u32();
                                    phi_13561_ = false;
                                }
                                let _e1469 = phi_13487_;
                                let _e1471 = phi_13490_;
                                let _e1473 = phi_13561_;
                                phi_13486_ = _e1469;
                                phi_13489_ = _e1471;
                                if !(_e1473) {
                                    break;
                                }
                            }
                            let _e1476 = local_77;
                            let _e1478 = global_4.member[_e1476];
                            let _e1480 = (_e1478.member == 4u);
                            if _e1480 {
                                if _e1480 {
                                    if (_e1478.member_1 == 1u) {
                                        phi_13621_ = true;
                                    } else {
                                        phi_13621_ = ((((-(_e318.member_1.x) * _e963.member_3.x) + (-(_e318.member_1.y) * _e963.member_3.y)) + (-(_e318.member_1.z) * _e963.member_3.z)) > 0f);
                                    }
                                    let _e6602 = phi_13621_;
                                    if _e6602 {
                                        phi_13630_ = vec3<f32>(_e1478.member_4.x, _e1478.member_4.y, _e1478.member_4.z);
                                    } else {
                                        phi_13630_ = vec3<f32>();
                                    }
                                    let _e6609 = phi_13630_;
                                    phi_13632_ = _e6609;
                                    phi_13633_ = select(true, false, _e6602);
                                } else {
                                    phi_13632_ = vec3<f32>();
                                    phi_13633_ = true;
                                }
                                let _e6612 = phi_13632_;
                                let _e6614 = phi_13633_;
                                let _e6616 = select(_e6612, vec3<f32>(0f, 0f, 0f), vec3(_e6614));
                                if _e358 {
                                    phi_13803_ = 1f;
                                } else {
                                    if ((_e364 & 1u) == 0u) {
                                        phi_13802_ = 1f;
                                    } else {
                                        if (_e1415.member_6.z > 0u) {
                                            let _e6625 = global_1.member.member_2[1u];
                                            let _e6626 = (_e6625 + (_e1415.member_6.z - 1u));
                                            let _e6629 = global_5.member[(_e6626 + 1u)];
                                            let _e6631 = global_5.member[_e6626];
                                            if (_e1415.member_6.x == 1u) {
                                                let _e6708 = (_e1415.member.x - _e318.member.x);
                                                let _e6709 = (_e1415.member.y - _e318.member.y);
                                                let _e6710 = (_e1415.member.z - _e318.member.z);
                                                let _e6715 = (((_e6708 * _e6708) + (_e6709 * _e6709)) + (_e6710 * _e6710));
                                                let _e6716 = (_e1415.member.w * _e1415.member.w);
                                                if (_e6715 <= (_e6716 * 1.000001f)) {
                                                    let _e6730 = (_e1411 - _e318.member.x);
                                                    let _e6731 = (_e1412 - _e318.member.y);
                                                    let _e6732 = (_e1413 - _e318.member.z);
                                                    let _e6737 = (((_e6730 * _e6730) + (_e6731 * _e6731)) + (_e6732 * _e6732));
                                                    phi_31131_ = (_e6737 / (((abs((((_e963.member_2.x * _e6730) + (_e963.member_2.y * _e6731)) + (_e963.member_2.z * _e6732))) / sqrt(_e6737)) * 12.566371f) * _e6716));
                                                } else {
                                                    let _e6719 = (_e6716 / _e6715);
                                                    if (_e6719 != _e6719) {
                                                        phi_31150_ = true;
                                                    } else {
                                                        phi_31150_ = (1f <= _e6719);
                                                    }
                                                    let _e6723 = phi_31150_;
                                                    let _e6724 = select(_e6719, 1f, _e6723);
                                                    phi_31131_ = (0.15915494f / (_e6724 / (1f + sqrt((1f - _e6724)))));
                                                }
                                                let _e6753 = phi_31131_;
                                                phi_13798_ = _e6753;
                                            } else {
                                                let _e6637 = (_e1411 - _e318.member.x);
                                                let _e6638 = (_e1412 - _e318.member.y);
                                                let _e6639 = (_e1413 - _e318.member.z);
                                                let _e6644 = (((_e6637 * _e6637) + (_e6638 * _e6638)) + (_e6639 * _e6639));
                                                let _e6655 = (abs((((_e963.member_2.x * _e6637) + (_e963.member_2.y * _e6638)) + (_e963.member_2.z * _e6639))) / sqrt(_e6644));
                                                if (_e6655 <= 0f) {
                                                    phi_13797_ = 0f;
                                                } else {
                                                    if (_e1415.member_6.x == 0u) {
                                                        let _e6670 = (_e1415.member_1.x - _e1415.member.x);
                                                        let _e6671 = (_e1415.member_1.y - _e1415.member.y);
                                                        let _e6672 = (_e1415.member_1.z - _e1415.member.z);
                                                        let _e6677 = (_e1415.member_2.x - _e1415.member.x);
                                                        let _e6678 = (_e1415.member_2.y - _e1415.member.y);
                                                        let _e6679 = (_e1415.member_2.z - _e1415.member.z);
                                                        let _e6682 = ((_e6671 * _e6679) - (_e6678 * _e6672));
                                                        let _e6685 = ((_e6672 * _e6677) - (_e6679 * _e6670));
                                                        let _e6688 = ((_e6670 * _e6678) - (_e6677 * _e6671));
                                                        phi_13794_ = (sqrt((((_e6682 * _e6682) + (_e6685 * _e6685)) + (_e6688 * _e6688))) * 0.5f);
                                                    } else {
                                                        phi_13794_ = ((12.566371f * _e1415.member.w) * _e1415.member.w);
                                                    }
                                                    let _e6697 = phi_13794_;
                                                    phi_13797_ = (_e6644 / (_e6655 * _e6697));
                                                }
                                                let _e6701 = phi_13797_;
                                                phi_13798_ = _e6701;
                                            }
                                            let _e6755 = phi_13798_;
                                            let _e6756 = ((_e6629 - _e6631) * _e6755);
                                            if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                                phi_31171_ = 1f;
                                            } else {
                                                let _e6758 = (_e318.member_1.w * _e318.member_1.w);
                                                let _e6760 = (_e6758 + (_e6756 * _e6756));
                                                if (_e6760 == 0f) {
                                                    phi_31168_ = 0f;
                                                } else {
                                                    phi_31168_ = (_e6758 / _e6760);
                                                }
                                                let _e6764 = phi_31168_;
                                                phi_31171_ = _e6764;
                                            }
                                            let _e6766 = phi_31171_;
                                            phi_13801_ = _e6766;
                                        } else {
                                            phi_13801_ = 1f;
                                        }
                                        let _e6768 = phi_13801_;
                                        phi_13802_ = _e6768;
                                    }
                                    let _e6770 = phi_13802_;
                                    phi_13803_ = _e6770;
                                }
                                let _e6772 = phi_13803_;
                                phi_19488_ = type_64(type_30(_e318.member, _e318.member_1, vec4<f32>(_e1404.x, _e1404.y, _e1404.z, f32(_e345)), vec4<f32>((_e318.member_3.x + ((_e1404.x * _e6616.x) * _e6772)), (_e318.member_3.y + ((_e1404.y * _e6616.y) * _e6772)), (_e318.member_3.z + ((_e1404.z * _e6616.z) * _e6772)), f32(_e964)), vec4<u32>(_e318.member_4.x, (_e318.member_4.y & 4294967294u), 0u, 0u)), _e1406, type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                phi_19489_ = type_64();
                                phi_19490_ = true;
                            } else {
                                let _e1481 = -(_e318.member_1.x);
                                let _e1482 = -(_e318.member_1.y);
                                let _e1483 = -(_e318.member_1.z);
                                if ((((_e963.member_2.x * _e1481) + (_e963.member_2.y * _e1482)) + (_e963.member_2.z * _e1483)) < 0f) {
                                    phi_13842_ = -(_e963.member_2);
                                } else {
                                    phi_13842_ = _e963.member_2;
                                }
                                let _e1496 = phi_13842_;
                                if ((((_e963.member_3.x * _e1481) + (_e963.member_3.y * _e1482)) + (_e963.member_3.z * _e1483)) < 0f) {
                                    phi_13859_ = -(_e963.member_3);
                                } else {
                                    phi_13859_ = _e963.member_3;
                                }
                                let _e1509 = phi_13859_;
                                let _e1512 = global_1.member.member_4[2u];
                                let _e1513 = abs(_e1411);
                                let _e1514 = abs(_e1412);
                                let _e1515 = abs(_e1413);
                                let _e1517 = select(_e1514, _e1513, (_e1513 > _e1514));
                                let _e1521 = (0.00002f * (1f + select(_e1515, _e1517, (_e1517 > _e1515))));
                                if (_e1512 != _e1512) {
                                    phi_25792_ = true;
                                } else {
                                    phi_25792_ = (_e1521 >= _e1512);
                                }
                                let _e1525 = phi_25792_;
                                let _e1527 = (select(_e1512, _e1521, _e1525) * 2f);
                                let _e1534 = (_e1411 + (_e1496.x * _e1527));
                                let _e1535 = (_e1412 + (_e1496.y * _e1527));
                                let _e1536 = (_e1413 + (_e1496.z * _e1527));
                                let _e1537 = vec3<f32>(_e1534, _e1535, _e1536);
                                if ((_e318.member_4.y & 4u) == 0u) {
                                    phi_13892_ = 0f;
                                } else {
                                    let _e1542 = global_1.member.member_4[0u];
                                    phi_13892_ = _e1542;
                                }
                                let _e1544 = phi_13892_;
                                let _e1547 = select(-1f, 1f, (_e1509.z >= 0f));
                                let _e1549 = (-1f / (_e1547 + _e1509.z));
                                let _e1553 = ((_e1509.x * _e1509.y) * _e1549);
                                let _e1557 = (1f + (((_e1547 * _e1509.x) * _e1509.x) * _e1549));
                                let _e1558 = (_e1547 * _e1553);
                                let _e1560 = (-(_e1547) * _e1509.x);
                                let _e1563 = (_e1547 + ((_e1509.y * _e1509.y) * _e1549));
                                let _e1564 = -(_e1509.y);
                                if (_e1478.member == 0u) {
                                    phi_33153_ = vec3<f32>(0f, 0f, 0f);
                                    phi_32961_ = 1.5f;
                                    phi_32882_ = vec3<f32>(0f, 0f, 0f);
                                    phi_32805_ = vec3<f32>(_e1478.member_4.x, _e1478.member_4.y, _e1478.member_4.z);
                                    phi_32726_ = 0u;
                                    phi_32644_ = 1f;
                                    phi_32567_ = 0u;
                                    phi_32558_ = _e318.member_4.x;
                                    phi_14571_ = type_64();
                                    phi_14572_ = true;
                                } else {
                                    if (_e1478.member == 1u) {
                                        let _e2113 = (_e1478.member_6.y * _e1478.member_6.y);
                                        if (_e2113 != _e2113) {
                                            phi_26484_ = true;
                                        } else {
                                            phi_26484_ = (_e1544 >= _e2113);
                                        }
                                        let _e2117 = phi_26484_;
                                        let _e2118 = select(_e2113, _e1544, _e2117);
                                        let _e2119 = (_e2118 >= 0.001f);
                                        if _e2119 {
                                            phi_33210_ = vec3<f32>(_e1478.member_4.x, _e1478.member_4.y, _e1478.member_4.z);
                                            phi_33004_ = _e1478.member_6.x;
                                            phi_32857_ = vec3<f32>(0f, 0f, 0f);
                                            phi_32545_ = _e318.member_4.x;
                                            phi_13967_ = vec3<f32>();
                                            phi_13968_ = true;
                                        } else {
                                            let _e2121 = ((_e318.member_4.x * 747796405u) + 2891336453u);
                                            let _e2129 = (((_e2121 >> bitcast<u32>((((_e2121 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2121) * 277803737u);
                                            let _e2132 = ((_e2129 >> bitcast<u32>(22i)) ^ _e2129);
                                            let _e2142 = abs((((_e1481 * _e1509.x) + (_e1482 * _e1509.y)) + (_e1483 * _e1509.z)));
                                            let _e2145 = (1f / _e1478.member_6.x);
                                            let _e2149 = ((_e2145 * _e2145) * (1f - (_e2142 * _e2142)));
                                            if (_e2149 >= 1f) {
                                                phi_26524_ = 1f;
                                            } else {
                                                let _e2152 = sqrt((1f - _e2149));
                                                if (_e2142 != _e2142) {
                                                    phi_26535_ = true;
                                                } else {
                                                    phi_26535_ = (_e2152 <= _e2142);
                                                }
                                                let _e2156 = phi_26535_;
                                                let _e2161 = pow(((1f - _e1478.member_6.x) / (1f + _e1478.member_6.x)), 2f);
                                                phi_26524_ = (_e2161 + ((1f - _e2161) * pow((1f - select(_e2142, _e2152, _e2156)), 5f)));
                                            }
                                            let _e2168 = phi_26524_;
                                            let _e2169 = ((f32((_e2132 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e2168);
                                            if _e2169 {
                                                let _e2182 = (((_e1509.x * _e318.member_1.x) + (_e1509.y * _e318.member_1.y)) + (_e1509.z * _e318.member_1.z));
                                                phi_32858_ = vec3<f32>(0f, 0f, 0f);
                                                phi_13965_ = (_e334 - vec3<f32>(((_e1509.x * 2f) * _e2182), ((_e1509.y * 2f) * _e2182), ((_e1509.z * 2f) * _e2182)));
                                            } else {
                                                phi_32858_ = vec3<f32>(_e1478.member_4.x, _e1478.member_4.y, _e1478.member_4.z);
                                                phi_13965_ = vec3<f32>();
                                            }
                                            let _e2189 = phi_32858_;
                                            let _e2191 = phi_13965_;
                                            phi_33210_ = vec3<f32>(0f, 0f, 0f);
                                            phi_33004_ = 1.5f;
                                            phi_32857_ = _e2189;
                                            phi_32545_ = _e2132;
                                            phi_13967_ = _e2191;
                                            phi_13968_ = select(true, false, _e2169);
                                        }
                                        let _e2201 = phi_33210_;
                                        let _e2203 = phi_33004_;
                                        let _e2205 = phi_32857_;
                                        let _e2207 = phi_32545_;
                                        let _e2209 = phi_13967_;
                                        let _e2211 = phi_13968_;
                                        phi_33156_ = _e2201;
                                        phi_32964_ = _e2203;
                                        phi_32885_ = vec3<f32>(0f, 0f, 0f);
                                        phi_32808_ = _e2205;
                                        phi_32729_ = select(0u, 1u, _e2119);
                                        phi_32647_ = select(1f, _e2118, _e2119);
                                        phi_32570_ = select(0u, 2u, _e2119);
                                        phi_32496_ = _e2207;
                                        phi_14441_ = _e1537;
                                        phi_14442_ = _e2209;
                                        phi_14443_ = vec3<f32>(1f, 1f, 1f);
                                        phi_14444_ = _e2211;
                                    } else {
                                        if (_e1478.member == 3u) {
                                            phi_32499_ = _e318.member_4.x;
                                            phi_14120_ = vec3<f32>();
                                            phi_14121_ = vec3<f32>();
                                            phi_14122_ = vec3<f32>();
                                            phi_14123_ = true;
                                            phi_14124_ = false;
                                        } else {
                                            let _e1568 = (_e1478.member == 5u);
                                            if _e1568 {
                                                phi_32500_ = _e318.member_4.x;
                                                phi_14116_ = vec3<f32>();
                                                phi_14117_ = vec3<f32>();
                                                phi_14118_ = vec3<f32>();
                                                phi_14119_ = false;
                                            } else {
                                                let _e1569 = (_e1478.member == 2u);
                                                if _e1569 {
                                                    let _e1574 = (((_e318.member_1.x * _e963.member_3.x) + (_e318.member_1.y * _e963.member_3.y)) + (_e318.member_1.z * _e963.member_3.z));
                                                    let _e1575 = (_e1574 > 0f);
                                                    if _e1575 {
                                                        phi_14001_ = -(_e963.member_3);
                                                    } else {
                                                        phi_14001_ = _e963.member_3;
                                                    }
                                                    let _e1578 = phi_14001_;
                                                    if _e1575 {
                                                        phi_14007_ = _e1478.member_6.x;
                                                    } else {
                                                        phi_14007_ = (1f / _e1478.member_6.x);
                                                    }
                                                    let _e1583 = phi_14007_;
                                                    let _e1584 = abs(_e1574);
                                                    let _e1585 = (_e1583 * _e1583);
                                                    let _e1588 = (_e1585 * (1f - (_e1584 * _e1584)));
                                                    if (_e1588 >= 1f) {
                                                        phi_25854_ = 1f;
                                                    } else {
                                                        let _e1591 = sqrt((1f - _e1588));
                                                        if (_e1584 != _e1584) {
                                                            phi_25865_ = true;
                                                        } else {
                                                            phi_25865_ = (_e1591 <= _e1584);
                                                        }
                                                        let _e1595 = phi_25865_;
                                                        let _e1600 = pow(((1f - _e1478.member_6.x) / (1f + _e1478.member_6.x)), 2f);
                                                        phi_25854_ = (_e1600 + ((1f - _e1600) * pow((1f - select(_e1584, _e1591, _e1595)), 5f)));
                                                    }
                                                    let _e1607 = phi_25854_;
                                                    let _e1615 = (((_e318.member_1.x * _e1578.x) + (_e318.member_1.y * _e1578.y)) + (_e318.member_1.z * _e1578.z));
                                                    let _e1619 = (1f - (_e1585 * (1f - (_e1615 * _e1615))));
                                                    if (_e1619 > 0f) {
                                                        let _e1622 = ((_e318.member_4.x * 747796405u) + 2891336453u);
                                                        let _e1630 = (((_e1622 >> bitcast<u32>((((_e1622 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1622) * 277803737u);
                                                        let _e1633 = ((_e1630 >> bitcast<u32>(22i)) ^ _e1630);
                                                        let _e1638 = ((f32((_e1633 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1607);
                                                        if _e1638 {
                                                            let _e1648 = sqrt(_e1619);
                                                            let _e1652 = (((_e318.member_1.x - (_e1578.x * _e1615)) * _e1583) - (_e1578.x * _e1648));
                                                            let _e1653 = (((_e318.member_1.y - (_e1578.y * _e1615)) * _e1583) - (_e1578.y * _e1648));
                                                            let _e1654 = (((_e318.member_1.z - (_e1578.z * _e1615)) * _e1583) - (_e1578.z * _e1648));
                                                            let _e1661 = (1f / sqrt((((_e1652 * _e1652) + (_e1653 * _e1653)) + (_e1654 * _e1654))));
                                                            phi_14088_ = vec3<f32>((_e1411 - (_e1578.x * _e1527)), (_e1412 - (_e1578.y * _e1527)), (_e1413 - (_e1578.z * _e1527)));
                                                            phi_14089_ = vec3<f32>((_e1652 * _e1661), (_e1653 * _e1661), (_e1654 * _e1661));
                                                            phi_14090_ = vec3<f32>(_e1478.member_4.x, _e1478.member_4.y, _e1478.member_4.z);
                                                        } else {
                                                            phi_14088_ = vec3<f32>();
                                                            phi_14089_ = vec3<f32>();
                                                            phi_14090_ = vec3<f32>();
                                                        }
                                                        let _e1679 = phi_14088_;
                                                        let _e1681 = phi_14089_;
                                                        let _e1683 = phi_14090_;
                                                        phi_32503_ = _e1633;
                                                        phi_14092_ = _e1679;
                                                        phi_14093_ = _e1681;
                                                        phi_14094_ = _e1683;
                                                        phi_14095_ = select(true, false, _e1638);
                                                    } else {
                                                        phi_32503_ = _e318.member_4.x;
                                                        phi_14092_ = vec3<f32>();
                                                        phi_14093_ = vec3<f32>();
                                                        phi_14094_ = vec3<f32>();
                                                        phi_14095_ = true;
                                                    }
                                                    let _e1686 = phi_32503_;
                                                    let _e1688 = phi_14092_;
                                                    let _e1690 = phi_14093_;
                                                    let _e1692 = phi_14094_;
                                                    let _e1694 = phi_14095_;
                                                    if _e1694 {
                                                        let _e1702 = (((_e963.member_3.x * _e318.member_1.x) + (_e963.member_3.y * _e318.member_1.y)) + (_e963.member_3.z * _e318.member_1.z));
                                                        phi_14110_ = vec3<f32>((_e1411 + (_e1578.x * _e1527)), (_e1412 + (_e1578.y * _e1527)), (_e1413 + (_e1578.z * _e1527)));
                                                        phi_14111_ = (_e334 - vec3<f32>(((_e963.member_3.x * 2f) * _e1702), ((_e963.member_3.y * 2f) * _e1702), ((_e963.member_3.z * 2f) * _e1702)));
                                                    } else {
                                                        phi_14110_ = _e1688;
                                                        phi_14111_ = _e1690;
                                                    }
                                                    let _e1716 = phi_14110_;
                                                    let _e1718 = phi_14111_;
                                                    phi_32501_ = _e1686;
                                                    phi_14113_ = _e1716;
                                                    phi_14114_ = _e1718;
                                                    phi_14115_ = select(_e1692, vec3<f32>(1f, 1f, 1f), vec3(_e1694));
                                                } else {
                                                    phi_32501_ = _e318.member_4.x;
                                                    phi_14113_ = vec3<f32>();
                                                    phi_14114_ = vec3<f32>();
                                                    phi_14115_ = vec3<f32>();
                                                }
                                                let _e1722 = phi_32501_;
                                                let _e1724 = phi_14113_;
                                                let _e1726 = phi_14114_;
                                                let _e1728 = phi_14115_;
                                                phi_32500_ = _e1722;
                                                phi_14116_ = _e1724;
                                                phi_14117_ = _e1726;
                                                phi_14118_ = _e1728;
                                                phi_14119_ = _e1569;
                                            }
                                            let _e1730 = phi_32500_;
                                            let _e1732 = phi_14116_;
                                            let _e1734 = phi_14117_;
                                            let _e1736 = phi_14118_;
                                            let _e1738 = phi_14119_;
                                            phi_32499_ = _e1730;
                                            phi_14120_ = _e1732;
                                            phi_14121_ = _e1734;
                                            phi_14122_ = _e1736;
                                            phi_14123_ = _e1568;
                                            phi_14124_ = _e1738;
                                        }
                                        let _e1740 = phi_32499_;
                                        let _e1742 = phi_14120_;
                                        let _e1744 = phi_14121_;
                                        let _e1746 = phi_14122_;
                                        let _e1748 = phi_14123_;
                                        let _e1750 = phi_14124_;
                                        if _e1748 {
                                            let _e1753 = (_e1478.member_6.y * _e1478.member_6.y);
                                            if (_e1753 != _e1753) {
                                                phi_25938_ = true;
                                            } else {
                                                phi_25938_ = (_e1544 >= _e1753);
                                            }
                                            let _e1757 = phi_25938_;
                                            let _e1758 = select(_e1753, _e1544, _e1757);
                                            let _e1770 = select(2u, 0u, (_e1478.member == 5u));
                                            let _e1771 = (_e1758 < 0.001f);
                                            if _e1771 {
                                                let _e1779 = (((_e1509.x * _e318.member_1.x) + (_e1509.y * _e318.member_1.y)) + (_e1509.z * _e318.member_1.z));
                                                let _e1790 = abs((((_e1481 * _e1509.x) + (_e1482 * _e1509.y)) + (_e1483 * _e1509.z)));
                                                if (_e1770 == 0u) {
                                                    let _e2072 = (1f - _e1790);
                                                    if (_e2072 != _e2072) {
                                                        phi_26469_ = true;
                                                    } else {
                                                        phi_26469_ = (0f >= _e2072);
                                                    }
                                                    let _e2076 = phi_26469_;
                                                    let _e2078 = pow(select(_e2072, 0f, _e2076), 5f);
                                                    let _e2079 = (1f - _e2078);
                                                    phi_14427_ = vec3<f32>(((_e1478.member_4.x * _e2079) + _e2078), ((_e1478.member_4.y * _e2079) + _e2078), ((_e1478.member_4.z * _e2079) + _e2078));
                                                } else {
                                                    if (_e1770 == 1u) {
                                                        let _e2054 = (0.44444448f * (1f - (_e1790 * _e1790)));
                                                        if (_e2054 >= 1f) {
                                                            phi_26430_ = 1f;
                                                        } else {
                                                            let _e2057 = sqrt((1f - _e2054));
                                                            if (_e1790 != _e1790) {
                                                                phi_26441_ = true;
                                                            } else {
                                                                phi_26441_ = (_e2057 <= _e1790);
                                                            }
                                                            let _e2061 = phi_26441_;
                                                            phi_26430_ = (0.040000003f + (0.96f * pow((1f - select(_e1790, _e2057, _e2061)), 5f)));
                                                        }
                                                        let _e2068 = phi_26430_;
                                                        phi_14426_ = vec3(_e2068);
                                                    } else {
                                                        let _e1794 = select(_e1790, 0f, (_e1790 < 0f));
                                                        let _e1796 = select(_e1794, 1f, (_e1794 > 1f));
                                                        let _e1798 = (1f - (_e1796 * _e1796));
                                                        let _e1805 = ((_e1478.member_4.x * _e1478.member_4.x) - (_e1478.member_5.x * _e1478.member_5.x));
                                                        let _e1806 = ((_e1478.member_4.y * _e1478.member_4.y) - (_e1478.member_5.y * _e1478.member_5.y));
                                                        let _e1807 = ((_e1478.member_4.z * _e1478.member_4.z) - (_e1478.member_5.z * _e1478.member_5.z));
                                                        let _e1809 = (_e1478.member_4.x * (_e1478.member_5.x + _e1478.member_5.x));
                                                        let _e1811 = (_e1478.member_4.y * (_e1478.member_5.y + _e1478.member_5.y));
                                                        let _e1813 = (_e1478.member_4.z * (_e1478.member_5.z + _e1478.member_5.z));
                                                        let _e1820 = ((_e1805 * _e1805) + (_e1809 * _e1809));
                                                        let _e1821 = ((_e1806 * _e1806) + (_e1811 * _e1811));
                                                        let _e1822 = ((_e1807 * _e1807) + (_e1813 * _e1813));
                                                        let _e1827 = ((_e1798 * _e1805) / _e1820);
                                                        let _e1828 = ((_e1798 * _e1806) / _e1821);
                                                        let _e1829 = ((_e1798 * _e1807) / _e1822);
                                                        let _e1830 = (1f - _e1827);
                                                        let _e1831 = (1f - _e1828);
                                                        let _e1832 = (1f - _e1829);
                                                        let _e1834 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1827, _e1828, _e1829));
                                                        let _e1835 = (_e1798 * _e1809);
                                                        let _e1836 = (_e1798 * _e1811);
                                                        let _e1837 = (_e1798 * _e1813);
                                                        let _e1842 = (vec3<f32>(_e1835, _e1836, _e1837) / vec3<f32>(_e1820, _e1821, _e1822));
                                                        let _e1855 = ((_e1834.x * _e1834.x) + (_e1842.x * _e1842.x));
                                                        let _e1856 = ((_e1834.y * _e1834.y) + (_e1842.y * _e1842.y));
                                                        let _e1857 = ((_e1834.z * _e1834.z) + (_e1842.z * _e1842.z));
                                                        if (_e1855 != _e1855) {
                                                            phi_26100_ = true;
                                                        } else {
                                                            phi_26100_ = (0f >= _e1855);
                                                        }
                                                        let _e1861 = phi_26100_;
                                                        let _e1863 = sqrt(select(_e1855, 0f, _e1861));
                                                        if (_e1856 != _e1856) {
                                                            phi_26115_ = true;
                                                        } else {
                                                            phi_26115_ = (0f >= _e1856);
                                                        }
                                                        let _e1867 = phi_26115_;
                                                        let _e1869 = sqrt(select(_e1856, 0f, _e1867));
                                                        if (_e1857 != _e1857) {
                                                            phi_26130_ = true;
                                                        } else {
                                                            phi_26130_ = (0f >= _e1857);
                                                        }
                                                        let _e1873 = phi_26130_;
                                                        let _e1875 = sqrt(select(_e1857, 0f, _e1873));
                                                        let _e1879 = ((_e1863 + _e1830) * 0.5f);
                                                        let _e1880 = ((_e1869 + _e1831) * 0.5f);
                                                        let _e1881 = ((_e1875 + _e1832) * 0.5f);
                                                        if (_e1879 != _e1879) {
                                                            phi_26157_ = true;
                                                        } else {
                                                            phi_26157_ = (0f >= _e1879);
                                                        }
                                                        let _e1885 = phi_26157_;
                                                        let _e1887 = sqrt(select(_e1879, 0f, _e1885));
                                                        if (_e1880 != _e1880) {
                                                            phi_26172_ = true;
                                                        } else {
                                                            phi_26172_ = (0f >= _e1880);
                                                        }
                                                        let _e1891 = phi_26172_;
                                                        let _e1893 = sqrt(select(_e1880, 0f, _e1891));
                                                        if (_e1881 != _e1881) {
                                                            phi_26187_ = true;
                                                        } else {
                                                            phi_26187_ = (0f >= _e1881);
                                                        }
                                                        let _e1897 = phi_26187_;
                                                        let _e1899 = sqrt(select(_e1881, 0f, _e1897));
                                                        let _e1900 = vec3<f32>(_e1887, _e1893, _e1899);
                                                        let _e1904 = ((_e1863 - _e1830) * 0.5f);
                                                        let _e1905 = ((_e1869 - _e1831) * 0.5f);
                                                        let _e1906 = ((_e1875 - _e1832) * 0.5f);
                                                        if (_e1904 != _e1904) {
                                                            phi_26214_ = true;
                                                        } else {
                                                            phi_26214_ = (0f >= _e1904);
                                                        }
                                                        let _e1910 = phi_26214_;
                                                        let _e1912 = sqrt(select(_e1904, 0f, _e1910));
                                                        if (_e1905 != _e1905) {
                                                            phi_26229_ = true;
                                                        } else {
                                                            phi_26229_ = (0f >= _e1905);
                                                        }
                                                        let _e1916 = phi_26229_;
                                                        let _e1918 = sqrt(select(_e1905, 0f, _e1916));
                                                        if (_e1906 != _e1906) {
                                                            phi_26244_ = true;
                                                        } else {
                                                            phi_26244_ = (0f >= _e1906);
                                                        }
                                                        let _e1922 = phi_26244_;
                                                        let _e1924 = sqrt(select(_e1906, 0f, _e1922));
                                                        if ((_e1835 / _e1820) < 0f) {
                                                            phi_14303_ = -(_e1912);
                                                        } else {
                                                            phi_14303_ = _e1912;
                                                        }
                                                        let _e1928 = phi_14303_;
                                                        if ((_e1836 / _e1821) < 0f) {
                                                            phi_14311_ = -(_e1918);
                                                        } else {
                                                            phi_14311_ = _e1918;
                                                        }
                                                        let _e1932 = phi_14311_;
                                                        if ((_e1837 / _e1822) < 0f) {
                                                            phi_14319_ = -(_e1924);
                                                        } else {
                                                            phi_14319_ = _e1924;
                                                        }
                                                        let _e1936 = phi_14319_;
                                                        let _e1940 = (_e1478.member_5.x * _e1796);
                                                        let _e1941 = (_e1478.member_5.y * _e1796);
                                                        let _e1942 = (_e1478.member_5.z * _e1796);
                                                        let _e1952 = vec3<f32>(((_e1478.member_4.x * _e1887) - (_e1478.member_5.x * _e1928)), ((_e1478.member_4.y * _e1893) - (_e1478.member_5.y * _e1932)), ((_e1478.member_4.z * _e1899) - (_e1478.member_5.z * _e1936)));
                                                        let _e1959 = ((_e1478.member_4.x * _e1928) + (_e1478.member_5.x * _e1887));
                                                        let _e1960 = ((_e1478.member_4.y * _e1932) + (_e1478.member_5.y * _e1893));
                                                        let _e1961 = ((_e1478.member_4.z * _e1936) + (_e1478.member_5.z * _e1899));
                                                        let _e1963 = vec3<f32>((_e1478.member_4.x * _e1796), (_e1478.member_4.y * _e1796), (_e1478.member_4.z * _e1796));
                                                        let _e1964 = (_e1963 - _e1900);
                                                        let _e1965 = (_e1940 - _e1928);
                                                        let _e1966 = (_e1941 - _e1932);
                                                        let _e1967 = (_e1942 - _e1936);
                                                        let _e1980 = (_e1963 + _e1900);
                                                        let _e1981 = (_e1940 + _e1928);
                                                        let _e1982 = (_e1941 + _e1932);
                                                        let _e1983 = (_e1942 + _e1936);
                                                        let _e1993 = ((_e1980.x * _e1980.x) + (_e1981 * _e1981));
                                                        let _e1994 = ((_e1980.y * _e1980.y) + (_e1982 * _e1982));
                                                        let _e1995 = ((_e1980.z * _e1980.z) + (_e1983 * _e1983));
                                                        let _e2005 = vec3(_e1796);
                                                        let _e2006 = (_e2005 - _e1952);
                                                        let _e2007 = -(vec3<f32>(_e1959, _e1960, _e1961));
                                                        let _e2023 = (_e2005 + _e1952);
                                                        let _e2033 = ((_e2023.x * _e2023.x) + (_e1959 * _e1959));
                                                        let _e2034 = ((_e2023.y * _e2023.y) + (_e1960 * _e1960));
                                                        let _e2035 = ((_e2023.z * _e2023.z) + (_e1961 * _e1961));
                                                        phi_14426_ = vec3<f32>((((((_e1964.x * _e1964.x) + (_e1965 * _e1965)) / select(0.000000000000000000000000000001f, _e1993, (_e1993 > 0.000000000000000000000000000001f))) + (((_e2006.x * _e2006.x) + (_e2007.x * _e2007.x)) / select(0.000000000000000000000000000001f, _e2033, (_e2033 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1964.y * _e1964.y) + (_e1966 * _e1966)) / select(0.000000000000000000000000000001f, _e1994, (_e1994 > 0.000000000000000000000000000001f))) + (((_e2006.y * _e2006.y) + (_e2007.y * _e2007.y)) / select(0.000000000000000000000000000001f, _e2034, (_e2034 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1964.z * _e1964.z) + (_e1967 * _e1967)) / select(0.000000000000000000000000000001f, _e1995, (_e1995 > 0.000000000000000000000000000001f))) + (((_e2006.z * _e2006.z) + (_e2007.z * _e2007.z)) / select(0.000000000000000000000000000001f, _e2035, (_e2035 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e2071 = phi_14426_;
                                                    phi_14427_ = _e2071;
                                                }
                                                let _e2088 = phi_14427_;
                                                phi_14428_ = (_e334 - vec3<f32>(((_e1509.x * 2f) * _e1779), ((_e1509.y * 2f) * _e1779), ((_e1509.z * 2f) * _e1779)));
                                                phi_14429_ = _e2088;
                                            } else {
                                                phi_14428_ = vec3<f32>();
                                                phi_14429_ = vec3<f32>();
                                            }
                                            let _e2090 = phi_14428_;
                                            let _e2092 = phi_14429_;
                                            phi_32887_ = vec3<f32>(_e1478.member_5.x, _e1478.member_5.y, _e1478.member_5.z);
                                            phi_32810_ = vec3<f32>(_e1478.member_4.x, _e1478.member_4.y, _e1478.member_4.z);
                                            phi_32731_ = _e1770;
                                            phi_32649_ = _e1758;
                                            phi_14431_ = _e2090;
                                            phi_14432_ = _e2092;
                                            phi_14433_ = _e1771;
                                        } else {
                                            phi_32887_ = vec3<f32>(0f, 0f, 0f);
                                            phi_32810_ = vec3<f32>(0f, 0f, 0f);
                                            phi_32731_ = 0u;
                                            phi_32649_ = 1f;
                                            phi_14431_ = _e1744;
                                            phi_14432_ = _e1746;
                                            phi_14433_ = _e1750;
                                        }
                                        let _e2094 = phi_32887_;
                                        let _e2096 = phi_32810_;
                                        let _e2098 = phi_32731_;
                                        let _e2100 = phi_32649_;
                                        let _e2102 = phi_14431_;
                                        let _e2104 = phi_14432_;
                                        let _e2106 = phi_14433_;
                                        phi_33156_ = vec3<f32>(0f, 0f, 0f);
                                        phi_32964_ = 1.5f;
                                        phi_32885_ = _e2094;
                                        phi_32808_ = _e2096;
                                        phi_32729_ = _e2098;
                                        phi_32647_ = _e2100;
                                        phi_32570_ = select(0u, 1u, _e1748);
                                        phi_32496_ = _e1740;
                                        phi_14441_ = select(_e1742, _e1537, vec3(_e1748));
                                        phi_14442_ = _e2102;
                                        phi_14443_ = _e2104;
                                        phi_14444_ = select(true, false, _e2106);
                                    }
                                    let _e2216 = phi_33156_;
                                    let _e2218 = phi_32964_;
                                    let _e2220 = phi_32885_;
                                    let _e2222 = phi_32808_;
                                    let _e2224 = phi_32729_;
                                    let _e2226 = phi_32647_;
                                    let _e2228 = phi_32570_;
                                    let _e2230 = phi_32496_;
                                    let _e2232 = phi_14441_;
                                    let _e2234 = phi_14442_;
                                    let _e2236 = phi_14443_;
                                    let _e2238 = phi_14444_;
                                    let _e2239 = select(true, false, _e2238);
                                    if _e2239 {
                                        if (_e345 >= _e361) {
                                            phi_32555_ = _e2230;
                                            phi_14560_ = _e1404;
                                            phi_14561_ = type_64();
                                            phi_14562_ = true;
                                        } else {
                                            let _e2252 = global_1.member.member_4[1u];
                                            if (sqrt((((_e2236.x * _e2236.x) + (_e2236.y * _e2236.y)) + (_e2236.z * _e2236.z))) <= _e2252) {
                                                phi_32550_ = _e2230;
                                                phi_14524_ = _e1404;
                                                phi_14525_ = false;
                                            } else {
                                                let _e2256 = global_1.member.member_7[1u];
                                                if (_e2256 == 1u) {
                                                    let _e2261 = global_1.member.member_4[3u];
                                                    if (f32(_e345) >= _e2261) {
                                                        let _e2264 = select(_e2236.y, _e2236.x, (_e2236.x > _e2236.y));
                                                        let _e2266 = select(_e2236.z, _e2264, (_e2264 > _e2236.z));
                                                        if (_e2266 != _e2266) {
                                                            phi_26601_ = true;
                                                        } else {
                                                            phi_26601_ = (1f <= _e2266);
                                                        }
                                                        let _e2270 = phi_26601_;
                                                        let _e2271 = select(_e2266, 1f, _e2270);
                                                        let _e2273 = ((_e2230 * 747796405u) + 2891336453u);
                                                        let _e2281 = (((_e2273 >> bitcast<u32>((((_e2273 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2273) * 277803737u);
                                                        let _e2284 = ((_e2281 >> bitcast<u32>(22i)) ^ _e2281);
                                                        let _e2289 = ((f32((_e2284 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e2271);
                                                        if _e2289 {
                                                            phi_14513_ = vec3<f32>();
                                                        } else {
                                                            phi_14513_ = vec3<f32>((_e1404.x / _e2271), (_e1404.y / _e2271), (_e1404.z / _e2271));
                                                        }
                                                        let _e2298 = phi_14513_;
                                                        phi_32553_ = _e2284;
                                                        phi_14515_ = _e2298;
                                                        phi_14516_ = select(true, false, _e2289);
                                                    } else {
                                                        phi_32553_ = _e2230;
                                                        phi_14515_ = _e1404;
                                                        phi_14516_ = true;
                                                    }
                                                    let _e2301 = phi_32553_;
                                                    let _e2303 = phi_14515_;
                                                    let _e2305 = phi_14516_;
                                                    phi_32552_ = _e2301;
                                                    phi_14517_ = _e2303;
                                                    phi_14518_ = _e2305;
                                                } else {
                                                    phi_32552_ = _e2230;
                                                    phi_14517_ = _e1404;
                                                    phi_14518_ = true;
                                                }
                                                let _e2307 = phi_32552_;
                                                let _e2309 = phi_14517_;
                                                let _e2311 = phi_14518_;
                                                phi_32550_ = _e2307;
                                                phi_14524_ = select(_e1404, _e2309, vec3(_e2311));
                                                phi_14525_ = select(false, true, _e2311);
                                            }
                                            let _e2316 = phi_32550_;
                                            let _e2318 = phi_14524_;
                                            let _e2320 = phi_14525_;
                                            if _e2320 {
                                                let _e2321 = (_e2318 * _e2236);
                                                let _e2332 = (_e2234 * (1f / sqrt((((_e2234.x * _e2234.x) + (_e2234.y * _e2234.y)) + (_e2234.z * _e2234.z)))));
                                                phi_14558_ = type_64(type_30(vec4<f32>(_e2232.x, _e2232.y, _e2232.z, _e318.member.w), vec4<f32>(_e2332.x, _e2332.y, _e2332.z, _e318.member_1.w), vec4<f32>(_e2321.x, _e2321.y, _e2321.z, f32((_e345 + 1u))), vec4<f32>(_e318.member_3.x, _e318.member_3.y, _e318.member_3.z, f32(_e964)), vec4<u32>(_e2316, (_e318.member_4.y | 2u), 0u, 0u)), _e1406, type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                            } else {
                                                phi_14558_ = type_64();
                                            }
                                            let _e2355 = phi_14558_;
                                            phi_32555_ = _e2316;
                                            phi_14560_ = _e2318;
                                            phi_14561_ = _e2355;
                                            phi_14562_ = select(true, false, _e2320);
                                        }
                                        let _e2358 = phi_32555_;
                                        let _e2360 = phi_14560_;
                                        let _e2362 = phi_14561_;
                                        let _e2364 = phi_14562_;
                                        if _e2364 {
                                            phi_14568_ = type_64(type_30(_e318.member, _e318.member_1, vec4<f32>(_e2360.x, _e2360.y, _e2360.z, f32(_e345)), vec4<f32>(_e318.member_3.x, _e318.member_3.y, _e318.member_3.z, f32(_e964)), vec4<u32>(_e2358, (_e318.member_4.y & 4294967294u), 0u, 0u)), _e1406, type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14568_ = _e2362;
                                        }
                                        let _e2377 = phi_14568_;
                                        phi_32559_ = _e2358;
                                        phi_14569_ = _e2377;
                                    } else {
                                        phi_32559_ = _e2230;
                                        phi_14569_ = type_64();
                                    }
                                    let _e2379 = phi_32559_;
                                    let _e2381 = phi_14569_;
                                    phi_33153_ = _e2216;
                                    phi_32961_ = _e2218;
                                    phi_32882_ = _e2220;
                                    phi_32805_ = _e2222;
                                    phi_32726_ = _e2224;
                                    phi_32644_ = _e2226;
                                    phi_32567_ = _e2228;
                                    phi_32558_ = _e2379;
                                    phi_14571_ = _e2381;
                                    phi_14572_ = select(true, false, _e2239);
                                }
                                let _e2389 = phi_33153_;
                                let _e2391 = phi_32961_;
                                let _e2393 = phi_32882_;
                                let _e2395 = phi_32805_;
                                let _e2397 = phi_32726_;
                                let _e2399 = phi_32644_;
                                let _e2401 = phi_32567_;
                                let _e2403 = phi_32558_;
                                let _e2405 = phi_14571_;
                                let _e2407 = phi_14572_;
                                if _e2407 {
                                    let _e2409 = ((_e318.member_4.y | 4u) & 4294967293u);
                                    let _e2412 = global_1.member.member_1[3u];
                                    phi_34490_ = _e1402;
                                    phi_32557_ = _e2403;
                                    phi_14583_ = _e349;
                                    phi_14586_ = type_34(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                    phi_14588_ = 0u;
                                    phi_14590_ = _e964;
                                    loop {
                                        let _e2414 = phi_34490_;
                                        let _e2416 = phi_32557_;
                                        let _e2418 = phi_14583_;
                                        let _e2420 = phi_14586_;
                                        let _e2422 = phi_14588_;
                                        let _e2424 = phi_14590_;
                                        local_85 = _e2424;
                                        local_86 = _e2418;
                                        local_87 = _e2418;
                                        local_88 = _e2418;
                                        local_89 = _e2420;
                                        local_90 = _e2424;
                                        local_91 = _e2418;
                                        local_92 = _e2418;
                                        local_93 = _e2418;
                                        local_94 = _e2420;
                                        local_95 = _e2424;
                                        local_96 = _e2418;
                                        local_97 = _e2418;
                                        local_98 = _e2418;
                                        local_99 = _e2420;
                                        local_100 = _e2424;
                                        local_101 = _e2418;
                                        local_102 = _e2418;
                                        local_103 = _e2418;
                                        local_104 = _e2420;
                                        let _e2426 = (_e2422 < (_e2412 + 3u));
                                        if _e2426 {
                                            if (_e2422 < _e2412) {
                                                let _e4412 = global_1.member.member_7[2u];
                                                let _e4414 = (_e4412 + (16u * _e2422));
                                                let _e4416 = global_5.member[_e4414];
                                                let _e4419 = global_5.member[(_e4414 + 1u)];
                                                let _e4422 = global_5.member[(_e4414 + 2u)];
                                                let _e4425 = global_5.member[(_e4414 + 3u)];
                                                let _e4428 = global_5.member[(_e4414 + 4u)];
                                                let _e4431 = global_5.member[(_e4414 + 5u)];
                                                let _e4434 = global_5.member[(_e4414 + 6u)];
                                                let _e4437 = global_5.member[(_e4414 + 8u)];
                                                let _e4440 = global_5.member[(_e4414 + 9u)];
                                                let _e4443 = global_5.member[(_e4414 + 10u)];
                                                let _e4446 = global_5.member[(_e4414 + 11u)];
                                                let _e4449 = global_5.member[(_e4414 + 12u)];
                                                let _e4450 = (_e4416 - _e1534);
                                                let _e4451 = (_e4419 - _e1535);
                                                let _e4452 = (_e4422 - _e1536);
                                                let _e4457 = (((_e4450 * _e4450) + (_e4451 * _e4451)) + (_e4452 * _e4452));
                                                let _e4458 = sqrt(_e4457);
                                                let _e4459 = (_e4450 / _e4458);
                                                let _e4460 = (_e4451 / _e4458);
                                                let _e4461 = (_e4452 / _e4458);
                                                if (_e4425 < 0.5f) {
                                                    phi_14873_ = 1f;
                                                } else {
                                                    let _e4471 = (((_e4437 * -(_e4459)) + (_e4440 * -(_e4460))) + (_e4443 * -(_e4461)));
                                                    if (_e4449 == _e4446) {
                                                        phi_14872_ = select(1f, 0f, (_e4471 < _e4449));
                                                    } else {
                                                        let _e4475 = ((_e4471 - _e4449) / (_e4446 - _e4449));
                                                        let _e4477 = select(_e4475, 0f, (_e4475 < 0f));
                                                        let _e4479 = select(_e4477, 1f, (_e4477 > 1f));
                                                        phi_14872_ = ((_e4479 * _e4479) * (3f - (2f * _e4479)));
                                                    }
                                                    let _e4487 = phi_14872_;
                                                    phi_14873_ = _e4487;
                                                }
                                                let _e4489 = phi_14873_;
                                                phi_34851_ = _e2416;
                                                phi_17811_ = 0f;
                                                phi_17812_ = vec3<f32>(_e4459, _e4460, _e4461);
                                                phi_17813_ = vec3<f32>(((_e4428 * _e4489) / _e4457), ((_e4431 * _e4489) / _e4457), ((_e4434 * _e4489) / _e4457));
                                                phi_17814_ = _e4458;
                                            } else {
                                                if (_e2422 == _e2412) {
                                                    if ((_e364 & 4u) == 0u) {
                                                        phi_34895_ = _e2416;
                                                        phi_16915_ = 0f;
                                                        phi_16916_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_16917_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e3163 = global_1.member.member_5;
                                                        let _e3171 = global_1.member.member_6[3u];
                                                        let _e3173 = sin((_e3171 * 0.5f));
                                                        let _e3175 = ((12.566371f * _e3173) * _e3173);
                                                        let _e3180 = (((_e3163.x * _e1509.x) + (_e3163.y * _e1509.y)) + (_e3163.z * _e1509.z));
                                                        if (_e3180 != _e3180) {
                                                            phi_27140_ = true;
                                                        } else {
                                                            phi_27140_ = (0f >= _e3180);
                                                        }
                                                        let _e3184 = phi_27140_;
                                                        let _e3186 = (select(_e3180, 0f, _e3184) * 0.31830987f);
                                                        if (_e2401 == 0u) {
                                                            if (_e3180 <= 0f) {
                                                                phi_27937_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_27937_ = (_e2395 * (_e3180 * 0.31830987f));
                                                            }
                                                            let _e3662 = phi_27937_;
                                                            phi_15283_ = type_52(_e3662, _e3186);
                                                            phi_15284_ = type_52();
                                                            phi_15285_ = true;
                                                        } else {
                                                            let _e3192 = (((_e1481 * _e1557) + (_e1482 * _e1558)) + (_e1483 * _e1560));
                                                            let _e3197 = (((_e1481 * _e1553) + (_e1482 * _e1563)) + (_e1483 * _e1564));
                                                            let _e3202 = (((_e1481 * _e1509.x) + (_e1482 * _e1509.y)) + (_e1483 * _e1509.z));
                                                            let _e3207 = (((_e3163.x * _e1557) + (_e3163.y * _e1558)) + (_e3163.z * _e1560));
                                                            let _e3212 = (((_e3163.x * _e1553) + (_e3163.y * _e1563)) + (_e3163.z * _e1564));
                                                            if (_e3202 <= 0f) {
                                                                phi_15247_ = type_52();
                                                                phi_15248_ = true;
                                                            } else {
                                                                let _e3214 = (_e3180 <= 0f);
                                                                if _e3214 {
                                                                    phi_15246_ = type_52();
                                                                } else {
                                                                    let _e3215 = (_e3192 + _e3207);
                                                                    let _e3216 = (_e3197 + _e3212);
                                                                    let _e3217 = (_e3202 + _e3180);
                                                                    let _e3224 = (1f / sqrt((((_e3215 * _e3215) + (_e3216 * _e3216)) + (_e3217 * _e3217))));
                                                                    let _e3227 = (_e3217 * _e3224);
                                                                    if (_e3227 <= 0f) {
                                                                        phi_27250_ = 0f;
                                                                    } else {
                                                                        let _e3229 = (_e2399 * _e2399);
                                                                        let _e3233 = ((((_e3229 - 1f) * _e3227) * _e3227) + 1f);
                                                                        phi_27250_ = (_e3229 / ((3.1415927f * _e3233) * _e3233));
                                                                    }
                                                                    let _e3238 = phi_27250_;
                                                                    let _e3239 = (_e3202 * _e3202);
                                                                    if (_e3239 >= 1f) {
                                                                        phi_27278_ = 0f;
                                                                    } else {
                                                                        phi_27278_ = ((sqrt((1f + (((_e2399 * _e2399) * ((_e3192 * _e3192) + (_e3197 * _e3197))) / _e3239))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3252 = phi_27278_;
                                                                    let _e3253 = (1f + _e3252);
                                                                    let _e3254 = (_e3180 * _e3180);
                                                                    if (_e3254 >= 1f) {
                                                                        phi_27306_ = 0f;
                                                                    } else {
                                                                        phi_27306_ = ((sqrt((1f + (((_e2399 * _e2399) * ((_e3207 * _e3207) + (_e3212 * _e3212))) / _e3254))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3267 = phi_27306_;
                                                                    let _e3274 = (((_e3192 * (_e3215 * _e3224)) + (_e3197 * (_e3216 * _e3224))) + (_e3202 * _e3227));
                                                                    if (_e2397 == 0u) {
                                                                        let _e3569 = (1f - _e3274);
                                                                        if (_e3569 != _e3569) {
                                                                            phi_27809_ = true;
                                                                        } else {
                                                                            phi_27809_ = (0f >= _e3569);
                                                                        }
                                                                        let _e3573 = phi_27809_;
                                                                        let _e3575 = pow(select(_e3569, 0f, _e3573), 5f);
                                                                        let _e3576 = (1f - _e3575);
                                                                        phi_15234_ = vec3<f32>(((_e2395.x * _e3576) + _e3575), ((_e2395.y * _e3576) + _e3575), ((_e2395.z * _e3576) + _e3575));
                                                                    } else {
                                                                        if (_e2397 == 1u) {
                                                                            let _e3542 = (1f / _e2391);
                                                                            let _e3546 = ((_e3542 * _e3542) * (1f - (_e3274 * _e3274)));
                                                                            if (_e3546 >= 1f) {
                                                                                phi_27770_ = 1f;
                                                                            } else {
                                                                                let _e3549 = sqrt((1f - _e3546));
                                                                                if (_e3274 != _e3274) {
                                                                                    phi_27781_ = true;
                                                                                } else {
                                                                                    phi_27781_ = (_e3549 <= _e3274);
                                                                                }
                                                                                let _e3553 = phi_27781_;
                                                                                let _e3558 = pow(((1f - _e2391) / (1f + _e2391)), 2f);
                                                                                phi_27770_ = (_e3558 + ((1f - _e3558) * pow((1f - select(_e3274, _e3549, _e3553)), 5f)));
                                                                            }
                                                                            let _e3565 = phi_27770_;
                                                                            phi_15233_ = vec3(_e3565);
                                                                        } else {
                                                                            let _e3278 = select(_e3274, 0f, (_e3274 < 0f));
                                                                            let _e3280 = select(_e3278, 1f, (_e3278 > 1f));
                                                                            let _e3282 = (1f - (_e3280 * _e3280));
                                                                            let _e3295 = ((_e2395.x * _e2395.x) - (_e2393.x * _e2393.x));
                                                                            let _e3296 = ((_e2395.y * _e2395.y) - (_e2393.y * _e2393.y));
                                                                            let _e3297 = ((_e2395.z * _e2395.z) - (_e2393.z * _e2393.z));
                                                                            let _e3299 = (_e2395.x * (_e2393.x + _e2393.x));
                                                                            let _e3301 = (_e2395.y * (_e2393.y + _e2393.y));
                                                                            let _e3303 = (_e2395.z * (_e2393.z + _e2393.z));
                                                                            let _e3310 = ((_e3295 * _e3295) + (_e3299 * _e3299));
                                                                            let _e3311 = ((_e3296 * _e3296) + (_e3301 * _e3301));
                                                                            let _e3312 = ((_e3297 * _e3297) + (_e3303 * _e3303));
                                                                            let _e3317 = ((_e3282 * _e3295) / _e3310);
                                                                            let _e3318 = ((_e3282 * _e3296) / _e3311);
                                                                            let _e3319 = ((_e3282 * _e3297) / _e3312);
                                                                            let _e3320 = (1f - _e3317);
                                                                            let _e3321 = (1f - _e3318);
                                                                            let _e3322 = (1f - _e3319);
                                                                            let _e3324 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3317, _e3318, _e3319));
                                                                            let _e3325 = (_e3282 * _e3299);
                                                                            let _e3326 = (_e3282 * _e3301);
                                                                            let _e3327 = (_e3282 * _e3303);
                                                                            let _e3332 = (vec3<f32>(_e3325, _e3326, _e3327) / vec3<f32>(_e3310, _e3311, _e3312));
                                                                            let _e3345 = ((_e3324.x * _e3324.x) + (_e3332.x * _e3332.x));
                                                                            let _e3346 = ((_e3324.y * _e3324.y) + (_e3332.y * _e3332.y));
                                                                            let _e3347 = ((_e3324.z * _e3324.z) + (_e3332.z * _e3332.z));
                                                                            if (_e3345 != _e3345) {
                                                                                phi_27440_ = true;
                                                                            } else {
                                                                                phi_27440_ = (0f >= _e3345);
                                                                            }
                                                                            let _e3351 = phi_27440_;
                                                                            let _e3353 = sqrt(select(_e3345, 0f, _e3351));
                                                                            if (_e3346 != _e3346) {
                                                                                phi_27455_ = true;
                                                                            } else {
                                                                                phi_27455_ = (0f >= _e3346);
                                                                            }
                                                                            let _e3357 = phi_27455_;
                                                                            let _e3359 = sqrt(select(_e3346, 0f, _e3357));
                                                                            if (_e3347 != _e3347) {
                                                                                phi_27470_ = true;
                                                                            } else {
                                                                                phi_27470_ = (0f >= _e3347);
                                                                            }
                                                                            let _e3363 = phi_27470_;
                                                                            let _e3365 = sqrt(select(_e3347, 0f, _e3363));
                                                                            let _e3369 = ((_e3353 + _e3320) * 0.5f);
                                                                            let _e3370 = ((_e3359 + _e3321) * 0.5f);
                                                                            let _e3371 = ((_e3365 + _e3322) * 0.5f);
                                                                            if (_e3369 != _e3369) {
                                                                                phi_27497_ = true;
                                                                            } else {
                                                                                phi_27497_ = (0f >= _e3369);
                                                                            }
                                                                            let _e3375 = phi_27497_;
                                                                            let _e3377 = sqrt(select(_e3369, 0f, _e3375));
                                                                            if (_e3370 != _e3370) {
                                                                                phi_27512_ = true;
                                                                            } else {
                                                                                phi_27512_ = (0f >= _e3370);
                                                                            }
                                                                            let _e3381 = phi_27512_;
                                                                            let _e3383 = sqrt(select(_e3370, 0f, _e3381));
                                                                            if (_e3371 != _e3371) {
                                                                                phi_27527_ = true;
                                                                            } else {
                                                                                phi_27527_ = (0f >= _e3371);
                                                                            }
                                                                            let _e3387 = phi_27527_;
                                                                            let _e3389 = sqrt(select(_e3371, 0f, _e3387));
                                                                            let _e3390 = vec3<f32>(_e3377, _e3383, _e3389);
                                                                            let _e3394 = ((_e3353 - _e3320) * 0.5f);
                                                                            let _e3395 = ((_e3359 - _e3321) * 0.5f);
                                                                            let _e3396 = ((_e3365 - _e3322) * 0.5f);
                                                                            if (_e3394 != _e3394) {
                                                                                phi_27554_ = true;
                                                                            } else {
                                                                                phi_27554_ = (0f >= _e3394);
                                                                            }
                                                                            let _e3400 = phi_27554_;
                                                                            let _e3402 = sqrt(select(_e3394, 0f, _e3400));
                                                                            if (_e3395 != _e3395) {
                                                                                phi_27569_ = true;
                                                                            } else {
                                                                                phi_27569_ = (0f >= _e3395);
                                                                            }
                                                                            let _e3406 = phi_27569_;
                                                                            let _e3408 = sqrt(select(_e3395, 0f, _e3406));
                                                                            if (_e3396 != _e3396) {
                                                                                phi_27584_ = true;
                                                                            } else {
                                                                                phi_27584_ = (0f >= _e3396);
                                                                            }
                                                                            let _e3412 = phi_27584_;
                                                                            let _e3414 = sqrt(select(_e3396, 0f, _e3412));
                                                                            if ((_e3325 / _e3310) < 0f) {
                                                                                phi_15110_ = -(_e3402);
                                                                            } else {
                                                                                phi_15110_ = _e3402;
                                                                            }
                                                                            let _e3418 = phi_15110_;
                                                                            if ((_e3326 / _e3311) < 0f) {
                                                                                phi_15118_ = -(_e3408);
                                                                            } else {
                                                                                phi_15118_ = _e3408;
                                                                            }
                                                                            let _e3422 = phi_15118_;
                                                                            if ((_e3327 / _e3312) < 0f) {
                                                                                phi_15126_ = -(_e3414);
                                                                            } else {
                                                                                phi_15126_ = _e3414;
                                                                            }
                                                                            let _e3426 = phi_15126_;
                                                                            let _e3430 = (_e2393.x * _e3280);
                                                                            let _e3431 = (_e2393.y * _e3280);
                                                                            let _e3432 = (_e2393.z * _e3280);
                                                                            let _e3442 = vec3<f32>(((_e2395.x * _e3377) - (_e2393.x * _e3418)), ((_e2395.y * _e3383) - (_e2393.y * _e3422)), ((_e2395.z * _e3389) - (_e2393.z * _e3426)));
                                                                            let _e3449 = ((_e2395.x * _e3418) + (_e2393.x * _e3377));
                                                                            let _e3450 = ((_e2395.y * _e3422) + (_e2393.y * _e3383));
                                                                            let _e3451 = ((_e2395.z * _e3426) + (_e2393.z * _e3389));
                                                                            let _e3453 = vec3<f32>((_e2395.x * _e3280), (_e2395.y * _e3280), (_e2395.z * _e3280));
                                                                            let _e3454 = (_e3453 - _e3390);
                                                                            let _e3455 = (_e3430 - _e3418);
                                                                            let _e3456 = (_e3431 - _e3422);
                                                                            let _e3457 = (_e3432 - _e3426);
                                                                            let _e3470 = (_e3453 + _e3390);
                                                                            let _e3471 = (_e3430 + _e3418);
                                                                            let _e3472 = (_e3431 + _e3422);
                                                                            let _e3473 = (_e3432 + _e3426);
                                                                            let _e3483 = ((_e3470.x * _e3470.x) + (_e3471 * _e3471));
                                                                            let _e3484 = ((_e3470.y * _e3470.y) + (_e3472 * _e3472));
                                                                            let _e3485 = ((_e3470.z * _e3470.z) + (_e3473 * _e3473));
                                                                            let _e3495 = vec3(_e3280);
                                                                            let _e3496 = (_e3495 - _e3442);
                                                                            let _e3497 = -(vec3<f32>(_e3449, _e3450, _e3451));
                                                                            let _e3513 = (_e3495 + _e3442);
                                                                            let _e3523 = ((_e3513.x * _e3513.x) + (_e3449 * _e3449));
                                                                            let _e3524 = ((_e3513.y * _e3513.y) + (_e3450 * _e3450));
                                                                            let _e3525 = ((_e3513.z * _e3513.z) + (_e3451 * _e3451));
                                                                            phi_15233_ = vec3<f32>((((((_e3454.x * _e3454.x) + (_e3455 * _e3455)) / select(0.000000000000000000000000000001f, _e3483, (_e3483 > 0.000000000000000000000000000001f))) + (((_e3496.x * _e3496.x) + (_e3497.x * _e3497.x)) / select(0.000000000000000000000000000001f, _e3523, (_e3523 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3454.y * _e3454.y) + (_e3456 * _e3456)) / select(0.000000000000000000000000000001f, _e3484, (_e3484 > 0.000000000000000000000000000001f))) + (((_e3496.y * _e3496.y) + (_e3497.y * _e3497.y)) / select(0.000000000000000000000000000001f, _e3524, (_e3524 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3454.z * _e3454.z) + (_e3457 * _e3457)) / select(0.000000000000000000000000000001f, _e3485, (_e3485 > 0.000000000000000000000000000001f))) + (((_e3496.z * _e3496.z) + (_e3497.z * _e3497.z)) / select(0.000000000000000000000000000001f, _e3525, (_e3525 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                        }
                                                                        let _e3568 = phi_15233_;
                                                                        phi_15234_ = _e3568;
                                                                    }
                                                                    let _e3588 = phi_15234_;
                                                                    let _e3590 = (4f * _e3202);
                                                                    phi_15246_ = type_52((_e3588 * (((1f / (_e3253 + _e3267)) * _e3238) / _e3590)), ((_e3238 / _e3253) / _e3590));
                                                                }
                                                                let _e3597 = phi_15246_;
                                                                phi_15247_ = _e3597;
                                                                phi_15248_ = _e3214;
                                                            }
                                                            let _e3599 = phi_15247_;
                                                            let _e3601 = phi_15248_;
                                                            if _e3601 {
                                                                phi_15253_ = type_52(vec3<f32>(0f, 0f, 0f), 0f);
                                                            } else {
                                                                phi_15253_ = _e3599;
                                                            }
                                                            let _e3603 = phi_15253_;
                                                            let _e3606 = (_e2401 == 1u);
                                                            if _e3606 {
                                                                phi_15281_ = _e3603;
                                                                phi_15282_ = type_52();
                                                            } else {
                                                                if (_e3202 != _e3202) {
                                                                    phi_27846_ = true;
                                                                } else {
                                                                    phi_27846_ = (0f >= _e3202);
                                                                }
                                                                let _e3610 = phi_27846_;
                                                                let _e3611 = select(_e3202, 0f, _e3610);
                                                                let _e3612 = (1f / _e2391);
                                                                let _e3616 = ((_e3612 * _e3612) * (1f - (_e3611 * _e3611)));
                                                                if (_e3616 >= 1f) {
                                                                    phi_27865_ = 1f;
                                                                } else {
                                                                    let _e3619 = sqrt((1f - _e3616));
                                                                    if (_e3611 != _e3611) {
                                                                        phi_27876_ = true;
                                                                    } else {
                                                                        phi_27876_ = (_e3619 <= _e3611);
                                                                    }
                                                                    let _e3623 = phi_27876_;
                                                                    let _e3628 = pow(((1f - _e2391) / (1f + _e2391)), 2f);
                                                                    phi_27865_ = (_e3628 + ((1f - _e3628) * pow((1f - select(_e3611, _e3619, _e3623)), 5f)));
                                                                }
                                                                let _e3635 = phi_27865_;
                                                                if (_e3180 <= 0f) {
                                                                    phi_27915_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    phi_27915_ = (_e2389 * (_e3180 * 0.31830987f));
                                                                }
                                                                let _e3640 = phi_27915_;
                                                                let _e3641 = (1f - _e3635);
                                                                phi_15281_ = type_52();
                                                                phi_15282_ = type_52((_e3603.member + vec3<f32>((_e3640.x * _e3641), (_e3640.y * _e3641), (_e3640.z * _e3641))), ((_e3635 * _e3603.member_1) + (_e3641 * _e3186)));
                                                            }
                                                            let _e3655 = phi_15281_;
                                                            let _e3657 = phi_15282_;
                                                            phi_15283_ = _e3655;
                                                            phi_15284_ = _e3657;
                                                            phi_15285_ = _e3606;
                                                        }
                                                        let _e3665 = phi_15283_;
                                                        let _e3667 = phi_15284_;
                                                        let _e3669 = phi_15285_;
                                                        if _e3669 {
                                                            phi_15289_ = _e3665;
                                                        } else {
                                                            phi_15289_ = _e3667;
                                                        }
                                                        let _e3671 = phi_15289_;
                                                        if ((_e3671.member_1 * _e3175) < 0.001f) {
                                                            let _e4098 = global_1.member.member_6;
                                                            let _e4104 = global_1.member.member_8[2u];
                                                            if (_e4104 == 4294967295u) {
                                                                phi_16028_ = vec3<f32>(1f, 1f, 1f);
                                                            } else {
                                                                let _e4107 = global_5.member[_e4104];
                                                                let _e4110 = global_5.member[(_e4104 + 1u)];
                                                                let _e4113 = global_5.member[(_e4104 + 2u)];
                                                                let _e4116 = global_5.member[(_e4104 + 3u)];
                                                                let _e4119 = global_5.member[(_e4104 + 4u)];
                                                                let _e4122 = global_5.member[(_e4104 + 5u)];
                                                                let _e4125 = global_5.member[(_e4104 + 6u)];
                                                                let _e4128 = global_5.member[(_e4104 + 7u)];
                                                                let _e4131 = global_5.member[(_e4104 + 8u)];
                                                                let _e4134 = global_5.member[(_e4104 + 9u)];
                                                                let _e4137 = global_5.member[(_e4104 + 13u)];
                                                                let _e4140 = global_5.member[(_e4104 + 14u)];
                                                                let _e4143 = global_5.member[(_e4104 + 15u)];
                                                                let _e4146 = global_5.member[(_e4104 + 16u)];
                                                                let _e4149 = global_5.member[(_e4104 + 18u)];
                                                                let _e4152 = global_5.member[(_e4104 + 19u)];
                                                                let _e4155 = global_5.member[(_e4104 + 20u)];
                                                                let _e4158 = global_5.member[(_e4104 + 29u)];
                                                                let _e4159 = bitcast<u32>(_e4158);
                                                                let _e4172 = (((((_e1534 * _e1534) + (_e1535 * _e1535)) + (_e1536 * _e1536)) - (2f * (((_e1534 * _e4107) + (_e1535 * _e4110)) + (_e1536 * _e4113)))) + _e4125);
                                                                if (_e4172 > 0f) {
                                                                    let _e4181 = ((((_e1534 - _e4107) * _e4128) + ((_e1535 - _e4110) * _e4131)) + ((_e1536 - _e4113) * _e4134));
                                                                    let _e4183 = ((_e4181 * _e4181) - _e4172);
                                                                    if (_e4181 >= 0f) {
                                                                        phi_15728_ = vec3<f32>();
                                                                        phi_15729_ = true;
                                                                    } else {
                                                                        let _e4185 = (_e4183 <= 0f);
                                                                        if _e4185 {
                                                                            phi_15727_ = vec3<f32>();
                                                                        } else {
                                                                            let _e4188 = (-(_e4181) - sqrt(_e4183));
                                                                            phi_15727_ = vec3<f32>((_e1534 + (_e4128 * _e4188)), (_e1535 + (_e4131 * _e4188)), (_e1536 + (_e4134 * _e4188)));
                                                                        }
                                                                        let _e4197 = phi_15727_;
                                                                        phi_15728_ = _e4197;
                                                                        phi_15729_ = _e4185;
                                                                    }
                                                                    let _e4199 = phi_15728_;
                                                                    let _e4201 = phi_15729_;
                                                                    phi_15734_ = _e4199;
                                                                    phi_15735_ = select(true, false, _e4201);
                                                                } else {
                                                                    phi_15734_ = _e1537;
                                                                    phi_15735_ = true;
                                                                }
                                                                let _e4204 = phi_15734_;
                                                                let _e4206 = phi_15735_;
                                                                if _e4206 {
                                                                    let _e4208 = (_e4204.x - _e4107);
                                                                    let _e4210 = (_e4204.y - _e4110);
                                                                    let _e4212 = (_e4204.z - _e4113);
                                                                    let _e4218 = sqrt((((_e4208 * _e4208) + (_e4210 * _e4210)) + (_e4212 * _e4212)));
                                                                    let _e4219 = (1f / _e4218);
                                                                    let _e4237 = ((((((_e4204.x * _e4204.x) + (_e4204.y * _e4204.y)) + (_e4204.z * _e4204.z)) - (2f * (((_e4204.x * _e4107) + (_e4204.y * _e4110)) + (_e4204.z * _e4113)))) + _e4122) / (_e4218 + _e4116));
                                                                    if (_e4237 != _e4237) {
                                                                        phi_28371_ = true;
                                                                    } else {
                                                                        phi_28371_ = (0f >= _e4237);
                                                                    }
                                                                    let _e4241 = phi_28371_;
                                                                    let _e4245 = sqrt((select(_e4237, 0f, _e4241) / (_e4119 - _e4116)));
                                                                    if (_e4245 != _e4245) {
                                                                        phi_28386_ = true;
                                                                    } else {
                                                                        phi_28386_ = (1f <= _e4245);
                                                                    }
                                                                    let _e4249 = phi_28386_;
                                                                    let _e4251 = (select(_e4245, 1f, _e4249) * 63f);
                                                                    let _e4256 = ((((_e4208 * _e4219) * _e4128) + ((_e4210 * _e4219) * _e4131)) + ((_e4212 * _e4219) * _e4134));
                                                                    let _e4258 = select(_e4256, -1f, (_e4256 < -1f));
                                                                    let _e4262 = ((select(_e4258, 1f, (_e4258 > 1f)) + 1f) * 127.5f);
                                                                    let _e4267 = select(select(u32(_e4251), 0u, (_e4251 < 0f)), 4294967295u, (_e4251 > 4294967000f));
                                                                    let _e4269 = select(_e4267, 62u, (62u < _e4267));
                                                                    let _e4274 = select(select(u32(_e4262), 0u, (_e4262 < 0f)), 4294967295u, (_e4262 > 4294967000f));
                                                                    let _e4276 = select(_e4274, 254u, (254u < _e4274));
                                                                    let _e4278 = (_e4251 - f32(_e4269));
                                                                    let _e4280 = (_e4262 - f32(_e4276));
                                                                    let _e4284 = (_e4159 + (3u * ((_e4269 * 256u) + _e4276)));
                                                                    let _e4289 = (_e4159 + (3u * (((_e4269 + 1u) * 256u) + _e4276)));
                                                                    let _e4291 = global_5.member[_e4284];
                                                                    let _e4294 = global_5.member[(_e4284 + 1u)];
                                                                    let _e4297 = global_5.member[(_e4284 + 2u)];
                                                                    let _e4298 = (1f - _e4280);
                                                                    let _e4304 = global_5.member[(_e4284 + 3u)];
                                                                    let _e4307 = global_5.member[(_e4284 + 4u)];
                                                                    let _e4310 = global_5.member[(_e4284 + 5u)];
                                                                    let _e4317 = (1f - _e4278);
                                                                    let _e4322 = global_5.member[_e4289];
                                                                    let _e4325 = global_5.member[(_e4289 + 1u)];
                                                                    let _e4328 = global_5.member[(_e4289 + 2u)];
                                                                    let _e4334 = global_5.member[(_e4289 + 3u)];
                                                                    let _e4337 = global_5.member[(_e4289 + 4u)];
                                                                    let _e4340 = global_5.member[(_e4289 + 5u)];
                                                                    phi_16026_ = vec3<f32>(((((_e4291 * _e4298) + (_e4304 * _e4280)) * _e4317) + (((_e4322 * _e4298) + (_e4334 * _e4280)) * _e4278)), ((((_e4294 * _e4298) + (_e4307 * _e4280)) * _e4317) + (((_e4325 * _e4298) + (_e4337 * _e4280)) * _e4278)), ((((_e4297 * _e4298) + (_e4310 * _e4280)) * _e4317) + (((_e4328 * _e4298) + (_e4340 * _e4280)) * _e4278)));
                                                                } else {
                                                                    phi_16026_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e4355 = phi_16026_;
                                                                let _e4362 = ((_e4146 * 1.1f) * _e4355.y);
                                                                phi_16028_ = vec3<f32>(exp(-((((_e4137 * _e4355.x) + _e4362) + (_e4149 * _e4355.z)))), exp(-((((_e4140 * _e4355.x) + _e4362) + (_e4152 * _e4355.z)))), exp(-((((_e4143 * _e4355.x) + _e4362) + (_e4155 * _e4355.z)))));
                                                            }
                                                            let _e4381 = phi_16028_;
                                                            phi_34896_ = _e2416;
                                                            phi_16912_ = 0f;
                                                            phi_16913_ = vec3<f32>(_e3163.x, _e3163.y, _e3163.z);
                                                            phi_16914_ = (vec3<f32>(_e4098.x, _e4098.y, _e4098.z) * _e4381);
                                                        } else {
                                                            let _e3676 = ((_e2416 * 747796405u) + 2891336453u);
                                                            let _e3684 = (((_e3676 >> bitcast<u32>((((_e3676 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3676) * 277803737u);
                                                            let _e3687 = ((_e3684 >> bitcast<u32>(22i)) ^ _e3684);
                                                            let _e3693 = (((f32((_e3687 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e3175) * 0.15915494f);
                                                            let _e3694 = (1f - _e3693);
                                                            let _e3696 = (_e3693 * (2f - _e3693));
                                                            if (_e3696 != _e3696) {
                                                                phi_27969_ = true;
                                                            } else {
                                                                phi_27969_ = (0f >= _e3696);
                                                            }
                                                            let _e3700 = phi_27969_;
                                                            let _e3702 = sqrt(select(_e3696, 0f, _e3700));
                                                            let _e3704 = ((_e3687 * 747796405u) + 2891336453u);
                                                            let _e3712 = (((_e3704 >> bitcast<u32>((((_e3704 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3704) * 277803737u);
                                                            let _e3715 = ((_e3712 >> bitcast<u32>(22i)) ^ _e3712);
                                                            let _e3719 = (f32((_e3715 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                            let _e3721 = select(-1f, 1f, (_e3163.z >= 0f));
                                                            let _e3723 = (-1f / (_e3721 + _e3163.z));
                                                            let _e3725 = ((_e3163.x * _e3163.y) * _e3723);
                                                            let _e3738 = (_e3702 * cos(_e3719));
                                                            let _e3743 = (_e3702 * sin(_e3719));
                                                            let _e3753 = ((((1f + (((_e3721 * _e3163.x) * _e3163.x) * _e3723)) * _e3738) + (_e3725 * _e3743)) + (_e3163.x * _e3694));
                                                            let _e3754 = ((((_e3721 * _e3725) * _e3738) + ((_e3721 + ((_e3163.y * _e3163.y) * _e3723)) * _e3743)) + (_e3163.y * _e3694));
                                                            let _e3755 = ((((-(_e3721) * _e3163.x) * _e3738) + (-(_e3163.y) * _e3743)) + (_e3163.z * _e3694));
                                                            let _e3757 = (_e3753 - _e3163.x);
                                                            let _e3758 = (_e3754 - _e3163.y);
                                                            let _e3759 = (_e3755 - _e3163.z);
                                                            let _e3766 = (sqrt((((_e3757 * _e3757) + (_e3758 * _e3758)) + (_e3759 * _e3759))) * 0.5f);
                                                            let _e3767 = (_e3766 != _e3766);
                                                            if _e3767 {
                                                                phi_28040_ = true;
                                                            } else {
                                                                phi_28040_ = (1f <= _e3766);
                                                            }
                                                            let _e3770 = phi_28040_;
                                                            if ((2f * asin(select(_e3766, 1f, _e3770))) >= _e3171) {
                                                                phi_16109_ = 0f;
                                                            } else {
                                                                phi_16109_ = (1f / _e3175);
                                                            }
                                                            let _e3777 = phi_16109_;
                                                            if (_e3777 > 0f) {
                                                                if _e3767 {
                                                                    phi_28055_ = true;
                                                                } else {
                                                                    phi_28055_ = (1f <= _e3766);
                                                                }
                                                                let _e3781 = phi_28055_;
                                                                let _e3784 = (2f * asin(select(_e3766, 1f, _e3781)));
                                                                if (_e3784 >= _e3171) {
                                                                    phi_16910_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    let _e3788 = (1f - pow((_e3784 / _e3171), 2f));
                                                                    if (_e3788 != _e3788) {
                                                                        phi_28070_ = true;
                                                                    } else {
                                                                        phi_28070_ = (0f >= _e3788);
                                                                    }
                                                                    let _e3792 = phi_28070_;
                                                                    let _e3795 = global_1.member.member_6;
                                                                    let _e3801 = global_1.member.member_8[2u];
                                                                    if (_e3801 == 4294967295u) {
                                                                        phi_16883_ = vec3<f32>(1f, 1f, 1f);
                                                                    } else {
                                                                        let _e3804 = global_5.member[_e3801];
                                                                        let _e3807 = global_5.member[(_e3801 + 1u)];
                                                                        let _e3810 = global_5.member[(_e3801 + 2u)];
                                                                        let _e3813 = global_5.member[(_e3801 + 3u)];
                                                                        let _e3816 = global_5.member[(_e3801 + 4u)];
                                                                        let _e3819 = global_5.member[(_e3801 + 5u)];
                                                                        let _e3822 = global_5.member[(_e3801 + 6u)];
                                                                        let _e3825 = global_5.member[(_e3801 + 7u)];
                                                                        let _e3828 = global_5.member[(_e3801 + 8u)];
                                                                        let _e3831 = global_5.member[(_e3801 + 9u)];
                                                                        let _e3834 = global_5.member[(_e3801 + 13u)];
                                                                        let _e3837 = global_5.member[(_e3801 + 14u)];
                                                                        let _e3840 = global_5.member[(_e3801 + 15u)];
                                                                        let _e3843 = global_5.member[(_e3801 + 16u)];
                                                                        let _e3846 = global_5.member[(_e3801 + 18u)];
                                                                        let _e3849 = global_5.member[(_e3801 + 19u)];
                                                                        let _e3852 = global_5.member[(_e3801 + 20u)];
                                                                        let _e3855 = global_5.member[(_e3801 + 29u)];
                                                                        let _e3856 = bitcast<u32>(_e3855);
                                                                        let _e3869 = (((((_e1534 * _e1534) + (_e1535 * _e1535)) + (_e1536 * _e1536)) - (2f * (((_e1534 * _e3804) + (_e1535 * _e3807)) + (_e1536 * _e3810)))) + _e3822);
                                                                        if (_e3869 > 0f) {
                                                                            let _e3878 = ((((_e1534 - _e3804) * _e3825) + ((_e1535 - _e3807) * _e3828)) + ((_e1536 - _e3810) * _e3831));
                                                                            let _e3880 = ((_e3878 * _e3878) - _e3869);
                                                                            if (_e3878 >= 0f) {
                                                                                phi_16583_ = vec3<f32>();
                                                                                phi_16584_ = true;
                                                                            } else {
                                                                                let _e3882 = (_e3880 <= 0f);
                                                                                if _e3882 {
                                                                                    phi_16582_ = vec3<f32>();
                                                                                } else {
                                                                                    let _e3885 = (-(_e3878) - sqrt(_e3880));
                                                                                    phi_16582_ = vec3<f32>((_e1534 + (_e3825 * _e3885)), (_e1535 + (_e3828 * _e3885)), (_e1536 + (_e3831 * _e3885)));
                                                                                }
                                                                                let _e3894 = phi_16582_;
                                                                                phi_16583_ = _e3894;
                                                                                phi_16584_ = _e3882;
                                                                            }
                                                                            let _e3896 = phi_16583_;
                                                                            let _e3898 = phi_16584_;
                                                                            phi_16589_ = _e3896;
                                                                            phi_16590_ = select(true, false, _e3898);
                                                                        } else {
                                                                            phi_16589_ = _e1537;
                                                                            phi_16590_ = true;
                                                                        }
                                                                        let _e3901 = phi_16589_;
                                                                        let _e3903 = phi_16590_;
                                                                        if _e3903 {
                                                                            let _e3905 = (_e3901.x - _e3804);
                                                                            let _e3907 = (_e3901.y - _e3807);
                                                                            let _e3909 = (_e3901.z - _e3810);
                                                                            let _e3915 = sqrt((((_e3905 * _e3905) + (_e3907 * _e3907)) + (_e3909 * _e3909)));
                                                                            let _e3916 = (1f / _e3915);
                                                                            let _e3934 = ((((((_e3901.x * _e3901.x) + (_e3901.y * _e3901.y)) + (_e3901.z * _e3901.z)) - (2f * (((_e3901.x * _e3804) + (_e3901.y * _e3807)) + (_e3901.z * _e3810)))) + _e3819) / (_e3915 + _e3813));
                                                                            if (_e3934 != _e3934) {
                                                                                phi_28194_ = true;
                                                                            } else {
                                                                                phi_28194_ = (0f >= _e3934);
                                                                            }
                                                                            let _e3938 = phi_28194_;
                                                                            let _e3942 = sqrt((select(_e3934, 0f, _e3938) / (_e3816 - _e3813)));
                                                                            if (_e3942 != _e3942) {
                                                                                phi_28209_ = true;
                                                                            } else {
                                                                                phi_28209_ = (1f <= _e3942);
                                                                            }
                                                                            let _e3946 = phi_28209_;
                                                                            let _e3948 = (select(_e3942, 1f, _e3946) * 63f);
                                                                            let _e3953 = ((((_e3905 * _e3916) * _e3825) + ((_e3907 * _e3916) * _e3828)) + ((_e3909 * _e3916) * _e3831));
                                                                            let _e3955 = select(_e3953, -1f, (_e3953 < -1f));
                                                                            let _e3959 = ((select(_e3955, 1f, (_e3955 > 1f)) + 1f) * 127.5f);
                                                                            let _e3964 = select(select(u32(_e3948), 0u, (_e3948 < 0f)), 4294967295u, (_e3948 > 4294967000f));
                                                                            let _e3966 = select(_e3964, 62u, (62u < _e3964));
                                                                            let _e3971 = select(select(u32(_e3959), 0u, (_e3959 < 0f)), 4294967295u, (_e3959 > 4294967000f));
                                                                            let _e3973 = select(_e3971, 254u, (254u < _e3971));
                                                                            let _e3975 = (_e3948 - f32(_e3966));
                                                                            let _e3977 = (_e3959 - f32(_e3973));
                                                                            let _e3981 = (_e3856 + (3u * ((_e3966 * 256u) + _e3973)));
                                                                            let _e3986 = (_e3856 + (3u * (((_e3966 + 1u) * 256u) + _e3973)));
                                                                            let _e3988 = global_5.member[_e3981];
                                                                            let _e3991 = global_5.member[(_e3981 + 1u)];
                                                                            let _e3994 = global_5.member[(_e3981 + 2u)];
                                                                            let _e3995 = (1f - _e3977);
                                                                            let _e4001 = global_5.member[(_e3981 + 3u)];
                                                                            let _e4004 = global_5.member[(_e3981 + 4u)];
                                                                            let _e4007 = global_5.member[(_e3981 + 5u)];
                                                                            let _e4014 = (1f - _e3975);
                                                                            let _e4019 = global_5.member[_e3986];
                                                                            let _e4022 = global_5.member[(_e3986 + 1u)];
                                                                            let _e4025 = global_5.member[(_e3986 + 2u)];
                                                                            let _e4031 = global_5.member[(_e3986 + 3u)];
                                                                            let _e4034 = global_5.member[(_e3986 + 4u)];
                                                                            let _e4037 = global_5.member[(_e3986 + 5u)];
                                                                            phi_16881_ = vec3<f32>(((((_e3988 * _e3995) + (_e4001 * _e3977)) * _e4014) + (((_e4019 * _e3995) + (_e4031 * _e3977)) * _e3975)), ((((_e3991 * _e3995) + (_e4004 * _e3977)) * _e4014) + (((_e4022 * _e3995) + (_e4034 * _e3977)) * _e3975)), ((((_e3994 * _e3995) + (_e4007 * _e3977)) * _e4014) + (((_e4025 * _e3995) + (_e4037 * _e3977)) * _e3975)));
                                                                        } else {
                                                                            phi_16881_ = vec3<f32>(0f, 0f, 0f);
                                                                        }
                                                                        let _e4052 = phi_16881_;
                                                                        let _e4059 = ((_e3843 * 1.1f) * _e4052.y);
                                                                        phi_16883_ = vec3<f32>(exp(-((((_e3834 * _e4052.x) + _e4059) + (_e3846 * _e4052.z)))), exp(-((((_e3837 * _e4052.x) + _e4059) + (_e3849 * _e4052.z)))), exp(-((((_e3840 * _e4052.x) + _e4059) + (_e3852 * _e4052.z)))));
                                                                    }
                                                                    let _e4078 = phi_16883_;
                                                                    let _e4089 = (((1f - (0.6f * (1f - sqrt(select(_e3788, 0f, _e3792))))) * 1.25f) / _e3175);
                                                                    phi_16910_ = vec3<f32>(((_e3795.x * _e4078.x) * _e4089), ((_e3795.y * _e4078.y) * _e4089), ((_e3795.z * _e4078.z) * _e4089));
                                                                }
                                                                let _e4095 = phi_16910_;
                                                                phi_16911_ = _e4095;
                                                            } else {
                                                                phi_16911_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e4097 = phi_16911_;
                                                            phi_34896_ = _e3715;
                                                            phi_16912_ = _e3777;
                                                            phi_16913_ = vec3<f32>(_e3753, _e3754, _e3755);
                                                            phi_16914_ = _e4097;
                                                        }
                                                        let _e4385 = phi_34896_;
                                                        let _e4387 = phi_16912_;
                                                        let _e4389 = phi_16913_;
                                                        let _e4391 = phi_16914_;
                                                        phi_34895_ = _e4385;
                                                        phi_16915_ = _e4387;
                                                        phi_16916_ = _e4389;
                                                        phi_16917_ = _e4391;
                                                    }
                                                    let _e4393 = phi_34895_;
                                                    let _e4395 = phi_16915_;
                                                    let _e4397 = phi_16916_;
                                                    let _e4399 = phi_16917_;
                                                    phi_34852_ = _e4393;
                                                    phi_17807_ = _e4395;
                                                    phi_17808_ = _e4397;
                                                    phi_17809_ = _e4399;
                                                    phi_17810_ = 1000000000000000000000000000000f;
                                                } else {
                                                    if (_e2422 == (_e2412 + 1u)) {
                                                        if ((_e364 & 1u) == 0u) {
                                                            phi_34861_ = _e2416;
                                                            phi_17441_ = 0f;
                                                            phi_17442_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17443_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17444_ = 1000000000000000000000000000000f;
                                                        } else {
                                                            if (_e367 > 0u) {
                                                                let _e2653 = global_1.member.member_2[1u];
                                                                let _e2655 = ((_e2416 * 747796405u) + 2891336453u);
                                                                let _e2663 = (((_e2655 >> bitcast<u32>((((_e2655 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2655) * 277803737u);
                                                                let _e2666 = ((_e2663 >> bitcast<u32>(22i)) ^ _e2663);
                                                                phi_16938_ = 0u;
                                                                phi_16941_ = _e367;
                                                                loop {
                                                                    let _e2672 = phi_16938_;
                                                                    let _e2674 = phi_16941_;
                                                                    local_83 = _e2672;
                                                                    local_84 = _e2672;
                                                                    let _e2676 = ((_e2672 + 1u) < _e2674);
                                                                    if _e2676 {
                                                                        let _e2678 = ((_e2672 + _e2674) / 2u);
                                                                        let _e2681 = global_5.member[(_e2653 + _e2678)];
                                                                        let _e2682 = (_e2681 <= (f32((_e2666 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                        phi_16939_ = select(_e2672, _e2678, _e2682);
                                                                        phi_16942_ = select(_e2678, _e2674, _e2682);
                                                                    } else {
                                                                        phi_16939_ = u32();
                                                                        phi_16942_ = u32();
                                                                    }
                                                                    let _e2686 = phi_16939_;
                                                                    let _e2688 = phi_16942_;
                                                                    phi_16938_ = _e2686;
                                                                    phi_16941_ = _e2688;
                                                                    if !(_e2676) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e2691 = local_83;
                                                                let _e2692 = (_e2653 + _e2691);
                                                                let _e2695 = global_5.member[(_e2692 + 1u)];
                                                                let _e2697 = global_5.member[_e2692];
                                                                let _e2701 = global_1.member.member_2[0u];
                                                                let _e2703 = local_84;
                                                                let _e2706 = global_5.member[(_e2701 + _e2703)];
                                                                let _e2709 = ((_e2666 * 747796405u) + 2891336453u);
                                                                let _e2717 = (((_e2709 >> bitcast<u32>((((_e2709 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2709) * 277803737u);
                                                                let _e2720 = ((_e2717 >> bitcast<u32>(22i)) ^ _e2717);
                                                                let _e2723 = f32((_e2720 >> bitcast<u32>(8i)));
                                                                let _e2724 = (_e2723 * 0.000000059604645f);
                                                                let _e2726 = ((_e2720 * 747796405u) + 2891336453u);
                                                                let _e2734 = (((_e2726 >> bitcast<u32>((((_e2726 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2726) * 277803737u);
                                                                let _e2737 = ((_e2734 >> bitcast<u32>(22i)) ^ _e2734);
                                                                let _e2740 = f32((_e2737 >> bitcast<u32>(8i)));
                                                                let _e2743 = global_2.member[bitcast<u32>(_e2706)];
                                                                let _e2746 = (_e2743.member_6.x == 1u);
                                                                if _e2746 {
                                                                    let _e2888 = (_e2743.member.x - _e1534);
                                                                    let _e2889 = (_e2743.member.y - _e1535);
                                                                    let _e2890 = (_e2743.member.z - _e1536);
                                                                    let _e2895 = (((_e2888 * _e2888) + (_e2889 * _e2889)) + (_e2890 * _e2890));
                                                                    let _e2896 = (_e2743.member.w * _e2743.member.w);
                                                                    let _e2898 = (_e2895 <= (_e2896 * 1.000001f));
                                                                    if _e2898 {
                                                                        let _e3007 = (_e2723 * 0.00000037450704f);
                                                                        let _e3009 = ((_e2740 * 0.00000011920929f) - 1f);
                                                                        let _e3011 = (1f - (_e3009 * _e3009));
                                                                        if (_e3011 != _e3011) {
                                                                            phi_27037_ = true;
                                                                        } else {
                                                                            phi_27037_ = (0f >= _e3011);
                                                                        }
                                                                        let _e3015 = phi_27037_;
                                                                        let _e3017 = sqrt(select(_e3011, 0f, _e3015));
                                                                        let _e3019 = (_e3017 * cos(_e3007));
                                                                        let _e3021 = (_e3017 * sin(_e3007));
                                                                        let _e3026 = (_e2743.member.x + (_e3019 * _e2743.member.w));
                                                                        let _e3027 = (_e2743.member.y + (_e3021 * _e2743.member.w));
                                                                        let _e3028 = (_e2743.member.z + (_e3009 * _e2743.member.w));
                                                                        if _e2898 {
                                                                            let _e3041 = (_e3026 - _e1534);
                                                                            let _e3042 = (_e3027 - _e1535);
                                                                            let _e3043 = (_e3028 - _e1536);
                                                                            let _e3048 = (((_e3041 * _e3041) + (_e3042 * _e3042)) + (_e3043 * _e3043));
                                                                            phi_27106_ = (_e3048 / (((abs((((_e3019 * _e3041) + (_e3021 * _e3042)) + (_e3009 * _e3043))) / sqrt(_e3048)) * 12.566371f) * _e2896));
                                                                        } else {
                                                                            let _e3030 = (_e2896 / _e2895);
                                                                            if (_e3030 != _e3030) {
                                                                                phi_27125_ = true;
                                                                            } else {
                                                                                phi_27125_ = (1f <= _e3030);
                                                                            }
                                                                            let _e3034 = phi_27125_;
                                                                            let _e3035 = select(_e3030, 1f, _e3034);
                                                                            phi_27106_ = (0.15915494f / (_e3035 / (1f + sqrt((1f - _e3035)))));
                                                                        }
                                                                        let _e3061 = phi_27106_;
                                                                        phi_17154_ = type_66(vec3<f32>(_e3026, _e3027, _e3028), vec3<f32>(_e3019, _e3021, _e3009), _e3061);
                                                                    } else {
                                                                        let _e2899 = (_e2896 / _e2895);
                                                                        let _e2900 = (_e2899 != _e2899);
                                                                        if _e2900 {
                                                                            phi_26889_ = true;
                                                                        } else {
                                                                            phi_26889_ = (1f <= _e2899);
                                                                        }
                                                                        let _e2903 = phi_26889_;
                                                                        let _e2904 = select(_e2899, 1f, _e2903);
                                                                        if _e2900 {
                                                                            phi_26912_ = true;
                                                                        } else {
                                                                            phi_26912_ = (1f <= _e2899);
                                                                        }
                                                                        let _e2908 = phi_26912_;
                                                                        let _e2909 = select(_e2899, 1f, _e2908);
                                                                        let _e2913 = (_e2909 / (1f + sqrt((1f - _e2909))));
                                                                        if (_e2904 < 0.00068523f) {
                                                                            phi_17089_ = (_e2904 * _e2724);
                                                                        } else {
                                                                            let _e2916 = (1f - (_e2724 * _e2913));
                                                                            let _e2918 = (1f - (_e2916 * _e2916));
                                                                            if (_e2918 != _e2918) {
                                                                                phi_26927_ = true;
                                                                            } else {
                                                                                phi_26927_ = (0f >= _e2918);
                                                                            }
                                                                            let _e2922 = phi_26927_;
                                                                            phi_17089_ = select(_e2918, 0f, _e2922);
                                                                        }
                                                                        let _e2926 = phi_17089_;
                                                                        let _e2927 = (1f - _e2926);
                                                                        if (_e2927 != _e2927) {
                                                                            phi_26942_ = true;
                                                                        } else {
                                                                            phi_26942_ = (0f >= _e2927);
                                                                        }
                                                                        let _e2931 = phi_26942_;
                                                                        let _e2936 = (1f - (_e2926 / _e2904));
                                                                        if (_e2936 != _e2936) {
                                                                            phi_26957_ = true;
                                                                        } else {
                                                                            phi_26957_ = (0f >= _e2936);
                                                                        }
                                                                        let _e2940 = phi_26957_;
                                                                        let _e2944 = ((_e2926 / sqrt(_e2904)) + (sqrt(select(_e2927, 0f, _e2931)) * sqrt(select(_e2936, 0f, _e2940))));
                                                                        let _e2946 = (1f - (_e2944 * _e2944));
                                                                        if (_e2946 != _e2946) {
                                                                            phi_26972_ = true;
                                                                        } else {
                                                                            phi_26972_ = (0f >= _e2946);
                                                                        }
                                                                        let _e2950 = phi_26972_;
                                                                        let _e2952 = sqrt(select(_e2946, 0f, _e2950));
                                                                        let _e2953 = (_e2740 * 0.00000037450704f);
                                                                        let _e2954 = sqrt(_e2895);
                                                                        let _e2955 = (_e2888 / _e2954);
                                                                        let _e2956 = (_e2889 / _e2954);
                                                                        let _e2957 = (_e2890 / _e2954);
                                                                        let _e2959 = select(-1f, 1f, (_e2957 >= 0f));
                                                                        let _e2961 = (-1f / (_e2959 + _e2957));
                                                                        let _e2963 = ((_e2955 * _e2956) * _e2961);
                                                                        let _e2976 = (_e2952 * cos(_e2953));
                                                                        let _e2981 = (_e2952 * sin(_e2953));
                                                                        let _e2994 = -(((((1f + (((_e2959 * _e2955) * _e2955) * _e2961)) * _e2976) + (_e2963 * _e2981)) + (_e2955 * _e2944)));
                                                                        let _e2995 = -(((((_e2959 * _e2963) * _e2976) + ((_e2959 + ((_e2956 * _e2956) * _e2961)) * _e2981)) + (_e2956 * _e2944)));
                                                                        let _e2996 = -(((((-(_e2959) * _e2955) * _e2976) + (-(_e2956) * _e2981)) + (_e2957 * _e2944)));
                                                                        phi_17154_ = type_66(vec3<f32>((_e2743.member.x + (_e2994 * _e2743.member.w)), (_e2743.member.y + (_e2995 * _e2743.member.w)), (_e2743.member.z + (_e2996 * _e2743.member.w))), vec3<f32>(_e2994, _e2995, _e2996), (0.15915494f / _e2913));
                                                                    }
                                                                    let _e3064 = phi_17154_;
                                                                    phi_17340_ = _e3064;
                                                                } else {
                                                                    let _e2747 = sqrt(_e2724);
                                                                    let _e2748 = (1f - _e2747);
                                                                    let _e2749 = ((_e2740 * 0.000000059604645f) * _e2747);
                                                                    let _e2771 = (_e2747 - _e2749);
                                                                    let _e2775 = (((_e2743.member.x * _e2748) + (_e2743.member_1.x * _e2749)) + (_e2743.member_2.x * _e2771));
                                                                    let _e2776 = (((_e2743.member.y * _e2748) + (_e2743.member_1.y * _e2749)) + (_e2743.member_2.y * _e2771));
                                                                    let _e2777 = (((_e2743.member.z * _e2748) + (_e2743.member_1.z * _e2749)) + (_e2743.member_2.z * _e2771));
                                                                    let _e2779 = (_e2743.member_1.x - _e2743.member.x);
                                                                    let _e2780 = (_e2743.member_1.y - _e2743.member.y);
                                                                    let _e2781 = (_e2743.member_1.z - _e2743.member.z);
                                                                    let _e2782 = (_e2743.member_2.x - _e2743.member.x);
                                                                    let _e2783 = (_e2743.member_2.y - _e2743.member.y);
                                                                    let _e2784 = (_e2743.member_2.z - _e2743.member.z);
                                                                    let _e2787 = ((_e2780 * _e2784) - (_e2783 * _e2781));
                                                                    let _e2790 = ((_e2781 * _e2782) - (_e2784 * _e2779));
                                                                    let _e2793 = ((_e2779 * _e2783) - (_e2782 * _e2780));
                                                                    let _e2799 = sqrt((((_e2787 * _e2787) + (_e2790 * _e2790)) + (_e2793 * _e2793)));
                                                                    let _e2800 = (1f / _e2799);
                                                                    let _e2801 = (_e2787 * _e2800);
                                                                    let _e2802 = (_e2790 * _e2800);
                                                                    let _e2803 = (_e2793 * _e2800);
                                                                    if _e2746 {
                                                                        let _e2836 = (_e2743.member.x - _e1534);
                                                                        let _e2837 = (_e2743.member.y - _e1535);
                                                                        let _e2838 = (_e2743.member.z - _e1536);
                                                                        let _e2843 = (((_e2836 * _e2836) + (_e2837 * _e2837)) + (_e2838 * _e2838));
                                                                        let _e2844 = (_e2743.member.w * _e2743.member.w);
                                                                        if (_e2843 <= (_e2844 * 1.000001f)) {
                                                                            let _e2858 = (_e2775 - _e1534);
                                                                            let _e2859 = (_e2776 - _e1535);
                                                                            let _e2860 = (_e2777 - _e1536);
                                                                            let _e2865 = (((_e2858 * _e2858) + (_e2859 * _e2859)) + (_e2860 * _e2860));
                                                                            phi_26855_ = (_e2865 / (((abs((((_e2801 * _e2858) + (_e2802 * _e2859)) + (_e2803 * _e2860))) / sqrt(_e2865)) * 12.566371f) * _e2844));
                                                                        } else {
                                                                            let _e2847 = (_e2844 / _e2843);
                                                                            if (_e2847 != _e2847) {
                                                                                phi_26874_ = true;
                                                                            } else {
                                                                                phi_26874_ = (1f <= _e2847);
                                                                            }
                                                                            let _e2851 = phi_26874_;
                                                                            let _e2852 = select(_e2847, 1f, _e2851);
                                                                            phi_26855_ = (0.15915494f / (_e2852 / (1f + sqrt((1f - _e2852)))));
                                                                        }
                                                                        let _e2878 = phi_26855_;
                                                                        phi_17338_ = _e2878;
                                                                    } else {
                                                                        let _e2805 = (_e2775 - _e1534);
                                                                        let _e2806 = (_e2776 - _e1535);
                                                                        let _e2807 = (_e2777 - _e1536);
                                                                        let _e2812 = (((_e2805 * _e2805) + (_e2806 * _e2806)) + (_e2807 * _e2807));
                                                                        let _e2820 = (abs((((_e2801 * _e2805) + (_e2802 * _e2806)) + (_e2803 * _e2807))) / sqrt(_e2812));
                                                                        if (_e2820 <= 0f) {
                                                                            phi_17337_ = 0f;
                                                                        } else {
                                                                            if (_e2743.member_6.x == 0u) {
                                                                                phi_17334_ = (_e2799 * 0.5f);
                                                                            } else {
                                                                                phi_17334_ = ((12.566371f * _e2743.member.w) * _e2743.member.w);
                                                                            }
                                                                            let _e2829 = phi_17334_;
                                                                            phi_17337_ = (_e2812 / (_e2820 * _e2829));
                                                                        }
                                                                        let _e2833 = phi_17337_;
                                                                        phi_17338_ = _e2833;
                                                                    }
                                                                    let _e2880 = phi_17338_;
                                                                    phi_17340_ = type_66(vec3<f32>(_e2775, _e2776, _e2777), vec3<f32>(_e2801, _e2802, _e2803), _e2880);
                                                                }
                                                                let _e3066 = phi_17340_;
                                                                let _e3071 = (_e3066.member.x - _e1534);
                                                                let _e3073 = (_e3066.member.y - _e1535);
                                                                let _e3075 = (_e3066.member.z - _e1536);
                                                                let _e3081 = sqrt((((_e3071 * _e3071) + (_e3073 * _e3073)) + (_e3075 * _e3075)));
                                                                let _e3082 = (_e3071 / _e3081);
                                                                let _e3083 = (_e3073 / _e3081);
                                                                let _e3084 = (_e3075 / _e3081);
                                                                if (_e3066.member_2 > 0f) {
                                                                    let _e3096 = global_4.member[_e2743.member_6.y];
                                                                    if (_e3096.member == 4u) {
                                                                        if (_e3096.member_1 == 1u) {
                                                                            phi_17418_ = true;
                                                                        } else {
                                                                            phi_17418_ = ((((-(_e3082) * _e3066.member_1.x) + (-(_e3083) * _e3066.member_1.y)) + (-(_e3084) * _e3066.member_1.z)) > 0f);
                                                                        }
                                                                        let _e3111 = phi_17418_;
                                                                        if _e3111 {
                                                                            phi_17427_ = vec3<f32>(_e3096.member_4.x, _e3096.member_4.y, _e3096.member_4.z);
                                                                        } else {
                                                                            phi_17427_ = vec3<f32>();
                                                                        }
                                                                        let _e3118 = phi_17427_;
                                                                        phi_17429_ = _e3118;
                                                                        phi_17430_ = select(true, false, _e3111);
                                                                    } else {
                                                                        phi_17429_ = vec3<f32>();
                                                                        phi_17430_ = true;
                                                                    }
                                                                    let _e3121 = phi_17429_;
                                                                    let _e3123 = phi_17430_;
                                                                    phi_17435_ = ((_e2695 - _e2697) * _e3066.member_2);
                                                                    phi_17436_ = select(_e3121, vec3<f32>(0f, 0f, 0f), vec3(_e3123));
                                                                } else {
                                                                    phi_17435_ = 0f;
                                                                    phi_17436_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e3127 = phi_17435_;
                                                                let _e3129 = phi_17436_;
                                                                phi_34862_ = _e2737;
                                                                phi_17437_ = _e3127;
                                                                phi_17438_ = vec3<f32>(_e3082, _e3083, _e3084);
                                                                phi_17439_ = _e3129;
                                                                phi_17440_ = ((_e3081 * 0.9999f) - _e1527);
                                                            } else {
                                                                phi_34862_ = _e2416;
                                                                phi_17437_ = 0f;
                                                                phi_17438_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_17439_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_17440_ = 1000000000000000000000000000000f;
                                                            }
                                                            let _e3131 = phi_34862_;
                                                            let _e3133 = phi_17437_;
                                                            let _e3135 = phi_17438_;
                                                            let _e3137 = phi_17439_;
                                                            let _e3139 = phi_17440_;
                                                            phi_34861_ = _e3131;
                                                            phi_17441_ = _e3133;
                                                            phi_17442_ = _e3135;
                                                            phi_17443_ = _e3137;
                                                            phi_17444_ = _e3139;
                                                        }
                                                        let _e3141 = phi_34861_;
                                                        let _e3143 = phi_17441_;
                                                        let _e3145 = phi_17442_;
                                                        let _e3147 = phi_17443_;
                                                        let _e3149 = phi_17444_;
                                                        phi_34853_ = _e3141;
                                                        phi_17803_ = _e3143;
                                                        phi_17804_ = _e3145;
                                                        phi_17805_ = _e3147;
                                                        phi_17806_ = _e3149;
                                                    } else {
                                                        if ((_e364 & 2u) == 0u) {
                                                            phi_34854_ = _e2416;
                                                            phi_17800_ = 0f;
                                                            phi_17801_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17802_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e2434 = ((_e2416 * 747796405u) + 2891336453u);
                                                            let _e2442 = (((_e2434 >> bitcast<u32>((((_e2434 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2434) * 277803737u);
                                                            let _e2445 = ((_e2442 >> bitcast<u32>(22i)) ^ _e2442);
                                                            let _e2449 = (f32((_e2445 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2451 = ((_e2445 * 747796405u) + 2891336453u);
                                                            let _e2459 = (((_e2451 >> bitcast<u32>((((_e2451 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2451) * 277803737u);
                                                            let _e2462 = ((_e2459 >> bitcast<u32>(22i)) ^ _e2459);
                                                            let _e2466 = (f32((_e2462 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2469 = global_1.member.member_3[1u];
                                                            let _e2472 = global_1.member.member_3[2u];
                                                            let _e2475 = global_1.member.member_2[3u];
                                                            phi_17465_ = 0u;
                                                            phi_17468_ = _e2472;
                                                            loop {
                                                                let _e2477 = phi_17465_;
                                                                let _e2479 = phi_17468_;
                                                                local_78 = _e2477;
                                                                local_81 = _e2477;
                                                                local_82 = _e2477;
                                                                let _e2481 = ((_e2477 + 1u) < _e2479);
                                                                if _e2481 {
                                                                    let _e2483 = ((_e2477 + _e2479) / 2u);
                                                                    let _e2486 = global_5.member[(_e2475 + _e2483)];
                                                                    let _e2487 = (_e2486 <= _e2449);
                                                                    phi_17466_ = select(_e2477, _e2483, _e2487);
                                                                    phi_17469_ = select(_e2483, _e2479, _e2487);
                                                                } else {
                                                                    phi_17466_ = u32();
                                                                    phi_17469_ = u32();
                                                                }
                                                                let _e2491 = phi_17466_;
                                                                let _e2493 = phi_17469_;
                                                                phi_17465_ = _e2491;
                                                                phi_17468_ = _e2493;
                                                                if !(_e2481) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2497 = global_1.member.member_3[0u];
                                                            let _e2498 = (_e2469 + 1u);
                                                            let _e2500 = local_78;
                                                            let _e2502 = (_e2497 + (_e2500 * _e2498));
                                                            phi_17506_ = 0u;
                                                            phi_17509_ = _e2469;
                                                            loop {
                                                                let _e2504 = phi_17506_;
                                                                let _e2506 = phi_17509_;
                                                                local_79 = _e2504;
                                                                local_80 = _e2504;
                                                                let _e2508 = ((_e2504 + 1u) < _e2506);
                                                                if _e2508 {
                                                                    let _e2510 = ((_e2504 + _e2506) / 2u);
                                                                    let _e2513 = global_5.member[(_e2502 + _e2510)];
                                                                    let _e2514 = (_e2513 <= _e2466);
                                                                    phi_17507_ = select(_e2504, _e2510, _e2514);
                                                                    phi_17510_ = select(_e2510, _e2506, _e2514);
                                                                } else {
                                                                    phi_17507_ = u32();
                                                                    phi_17510_ = u32();
                                                                }
                                                                let _e2518 = phi_17507_;
                                                                let _e2520 = phi_17510_;
                                                                phi_17506_ = _e2518;
                                                                phi_17509_ = _e2520;
                                                                if !(_e2508) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2523 = local_79;
                                                            let _e2526 = local_80;
                                                            let _e2527 = (_e2502 + _e2526);
                                                            let _e2529 = global_5.member[_e2527];
                                                            let _e2532 = global_5.member[(_e2527 + 1u)];
                                                            if (_e2532 > _e2529) {
                                                                let _e2536 = ((_e2466 - _e2529) / (_e2532 - _e2529));
                                                                let _e2538 = select(_e2536, 0f, (_e2536 < 0f));
                                                                phi_17586_ = select(_e2538, 0.99999f, (_e2538 > 0.99999f));
                                                            } else {
                                                                phi_17586_ = 0.5f;
                                                            }
                                                            let _e2542 = phi_17586_;
                                                            let _e2544 = f32(_e2469);
                                                            let _e2547 = local_81;
                                                            let _e2550 = local_82;
                                                            let _e2551 = (_e2475 + _e2550);
                                                            let _e2553 = global_5.member[_e2551];
                                                            let _e2556 = global_5.member[(_e2551 + 1u)];
                                                            if (_e2556 > _e2553) {
                                                                let _e2560 = ((_e2449 - _e2553) / (_e2556 - _e2553));
                                                                let _e2562 = select(_e2560, 0f, (_e2560 < 0f));
                                                                phi_17635_ = select(_e2562, 0.99999f, (_e2562 > 0.99999f));
                                                            } else {
                                                                phi_17635_ = 0.5f;
                                                            }
                                                            let _e2566 = phi_17635_;
                                                            let _e2568 = f32(_e2472);
                                                            let _e2571 = ((((f32(_e2523) + _e2542) / _e2544) - 0.5f) * 6.2831855f);
                                                            let _e2572 = (((f32(_e2547) + _e2566) / _e2568) * 3.1415927f);
                                                            let _e2573 = sin(_e2572);
                                                            let _e2575 = (_e2573 * cos(_e2571));
                                                            let _e2576 = cos(_e2572);
                                                            let _e2578 = (_e2573 * sin(_e2571));
                                                            let _e2581 = ((atan2(_e2578, _e2575) * 0.15915494f) + 0.5f);
                                                            let _e2585 = select(_e2576, -1f, (_e2576 < -1f));
                                                            let _e2588 = acos(select(_e2585, 1f, (_e2585 > 1f)));
                                                            let _e2590 = ((_e2581 - floor(_e2581)) * _e2544);
                                                            let _e2595 = select(select(u32(_e2590), 0u, (_e2590 < 0f)), 4294967295u, (_e2590 > 4294967000f));
                                                            let _e2596 = (_e2469 - 1u);
                                                            let _e2599 = ((_e2588 * 0.31830987f) * _e2568);
                                                            let _e2604 = select(select(u32(_e2599), 0u, (_e2599 < 0f)), 4294967295u, (_e2599 > 4294967000f));
                                                            let _e2605 = (_e2472 - 1u);
                                                            let _e2607 = select(_e2604, _e2605, (_e2605 < _e2604));
                                                            let _e2608 = sin(_e2588);
                                                            if (_e2608 <= 0f) {
                                                                phi_17791_ = 0f;
                                                            } else {
                                                                let _e2612 = (_e2475 + _e2607);
                                                                let _e2615 = global_5.member[(_e2612 + 1u)];
                                                                let _e2617 = global_5.member[_e2612];
                                                                let _e2620 = ((_e2497 + (_e2607 * _e2498)) + select(_e2595, _e2596, (_e2596 < _e2595)));
                                                                let _e2623 = global_5.member[(_e2620 + 1u)];
                                                                let _e2625 = global_5.member[_e2620];
                                                                phi_17791_ = ((((_e2615 - _e2617) * _e2568) * ((_e2623 - _e2625) * _e2544)) / (19.73921f * _e2608));
                                                            }
                                                            let _e2632 = phi_17791_;
                                                            let _e2633 = (_e2632 > 0f);
                                                            if _e2633 {
                                                                phi_17798_ = vec3<f32>(_e2575, _e2576, _e2578);
                                                            } else {
                                                                phi_17798_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2636 = phi_17798_;
                                                            phi_34854_ = _e2462;
                                                            phi_17800_ = select(0f, _e2632, _e2633);
                                                            phi_17801_ = _e2636;
                                                            phi_17802_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2633));
                                                        }
                                                        let _e2641 = phi_34854_;
                                                        let _e2643 = phi_17800_;
                                                        let _e2645 = phi_17801_;
                                                        let _e2647 = phi_17802_;
                                                        phi_34853_ = _e2641;
                                                        phi_17803_ = _e2643;
                                                        phi_17804_ = _e2645;
                                                        phi_17805_ = _e2647;
                                                        phi_17806_ = 1000000000000000000000000000000f;
                                                    }
                                                    let _e3151 = phi_34853_;
                                                    let _e3153 = phi_17803_;
                                                    let _e3155 = phi_17804_;
                                                    let _e3157 = phi_17805_;
                                                    let _e3159 = phi_17806_;
                                                    phi_34852_ = _e3151;
                                                    phi_17807_ = _e3153;
                                                    phi_17808_ = _e3155;
                                                    phi_17809_ = _e3157;
                                                    phi_17810_ = _e3159;
                                                }
                                                let _e4401 = phi_34852_;
                                                let _e4403 = phi_17807_;
                                                let _e4405 = phi_17808_;
                                                let _e4407 = phi_17809_;
                                                let _e4409 = phi_17810_;
                                                phi_34851_ = _e4401;
                                                phi_17811_ = _e4403;
                                                phi_17812_ = _e4405;
                                                phi_17813_ = _e4407;
                                                phi_17814_ = _e4409;
                                            }
                                            let _e4498 = phi_34851_;
                                            let _e4500 = phi_17811_;
                                            let _e4502 = phi_17812_;
                                            let _e4504 = phi_17813_;
                                            let _e4506 = phi_17814_;
                                            if ((((_e4504.x * _e4504.x) + (_e4504.y * _e4504.y)) + (_e4504.z * _e4504.z)) > 0f) {
                                                let _e4523 = (((_e4502.x * _e1509.x) + (_e4502.y * _e1509.y)) + (_e4502.z * _e1509.z));
                                                if (_e4523 != _e4523) {
                                                    phi_28439_ = true;
                                                } else {
                                                    phi_28439_ = (0f >= _e4523);
                                                }
                                                let _e4527 = phi_28439_;
                                                let _e4529 = (select(_e4523, 0f, _e4527) * 0.31830987f);
                                                if (_e2401 == 0u) {
                                                    if (_e4523 <= 0f) {
                                                        phi_29236_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_29236_ = (_e2395 * (_e4523 * 0.31830987f));
                                                    }
                                                    let _e5005 = phi_29236_;
                                                    phi_18210_ = type_52(_e5005, _e4529);
                                                    phi_18211_ = type_52();
                                                    phi_18212_ = true;
                                                } else {
                                                    let _e4535 = (((_e1481 * _e1557) + (_e1482 * _e1558)) + (_e1483 * _e1560));
                                                    let _e4540 = (((_e1481 * _e1553) + (_e1482 * _e1563)) + (_e1483 * _e1564));
                                                    let _e4545 = (((_e1481 * _e1509.x) + (_e1482 * _e1509.y)) + (_e1483 * _e1509.z));
                                                    let _e4550 = (((_e4502.x * _e1557) + (_e4502.y * _e1558)) + (_e4502.z * _e1560));
                                                    let _e4555 = (((_e4502.x * _e1553) + (_e4502.y * _e1563)) + (_e4502.z * _e1564));
                                                    if (_e4545 <= 0f) {
                                                        phi_18174_ = type_52();
                                                        phi_18175_ = true;
                                                    } else {
                                                        let _e4557 = (_e4523 <= 0f);
                                                        if _e4557 {
                                                            phi_18173_ = type_52();
                                                        } else {
                                                            let _e4558 = (_e4535 + _e4550);
                                                            let _e4559 = (_e4540 + _e4555);
                                                            let _e4560 = (_e4545 + _e4523);
                                                            let _e4567 = (1f / sqrt((((_e4558 * _e4558) + (_e4559 * _e4559)) + (_e4560 * _e4560))));
                                                            let _e4570 = (_e4560 * _e4567);
                                                            if (_e4570 <= 0f) {
                                                                phi_28549_ = 0f;
                                                            } else {
                                                                let _e4572 = (_e2399 * _e2399);
                                                                let _e4576 = ((((_e4572 - 1f) * _e4570) * _e4570) + 1f);
                                                                phi_28549_ = (_e4572 / ((3.1415927f * _e4576) * _e4576));
                                                            }
                                                            let _e4581 = phi_28549_;
                                                            let _e4582 = (_e4545 * _e4545);
                                                            if (_e4582 >= 1f) {
                                                                phi_28577_ = 0f;
                                                            } else {
                                                                phi_28577_ = ((sqrt((1f + (((_e2399 * _e2399) * ((_e4535 * _e4535) + (_e4540 * _e4540))) / _e4582))) - 1f) * 0.5f);
                                                            }
                                                            let _e4595 = phi_28577_;
                                                            let _e4596 = (1f + _e4595);
                                                            let _e4597 = (_e4523 * _e4523);
                                                            if (_e4597 >= 1f) {
                                                                phi_28605_ = 0f;
                                                            } else {
                                                                phi_28605_ = ((sqrt((1f + (((_e2399 * _e2399) * ((_e4550 * _e4550) + (_e4555 * _e4555))) / _e4597))) - 1f) * 0.5f);
                                                            }
                                                            let _e4610 = phi_28605_;
                                                            let _e4617 = (((_e4535 * (_e4558 * _e4567)) + (_e4540 * (_e4559 * _e4567))) + (_e4545 * _e4570));
                                                            if (_e2397 == 0u) {
                                                                let _e4912 = (1f - _e4617);
                                                                if (_e4912 != _e4912) {
                                                                    phi_29108_ = true;
                                                                } else {
                                                                    phi_29108_ = (0f >= _e4912);
                                                                }
                                                                let _e4916 = phi_29108_;
                                                                let _e4918 = pow(select(_e4912, 0f, _e4916), 5f);
                                                                let _e4919 = (1f - _e4918);
                                                                phi_18161_ = vec3<f32>(((_e2395.x * _e4919) + _e4918), ((_e2395.y * _e4919) + _e4918), ((_e2395.z * _e4919) + _e4918));
                                                            } else {
                                                                if (_e2397 == 1u) {
                                                                    let _e4885 = (1f / _e2391);
                                                                    let _e4889 = ((_e4885 * _e4885) * (1f - (_e4617 * _e4617)));
                                                                    if (_e4889 >= 1f) {
                                                                        phi_29069_ = 1f;
                                                                    } else {
                                                                        let _e4892 = sqrt((1f - _e4889));
                                                                        if (_e4617 != _e4617) {
                                                                            phi_29080_ = true;
                                                                        } else {
                                                                            phi_29080_ = (_e4892 <= _e4617);
                                                                        }
                                                                        let _e4896 = phi_29080_;
                                                                        let _e4901 = pow(((1f - _e2391) / (1f + _e2391)), 2f);
                                                                        phi_29069_ = (_e4901 + ((1f - _e4901) * pow((1f - select(_e4617, _e4892, _e4896)), 5f)));
                                                                    }
                                                                    let _e4908 = phi_29069_;
                                                                    phi_18160_ = vec3(_e4908);
                                                                } else {
                                                                    let _e4621 = select(_e4617, 0f, (_e4617 < 0f));
                                                                    let _e4623 = select(_e4621, 1f, (_e4621 > 1f));
                                                                    let _e4625 = (1f - (_e4623 * _e4623));
                                                                    let _e4638 = ((_e2395.x * _e2395.x) - (_e2393.x * _e2393.x));
                                                                    let _e4639 = ((_e2395.y * _e2395.y) - (_e2393.y * _e2393.y));
                                                                    let _e4640 = ((_e2395.z * _e2395.z) - (_e2393.z * _e2393.z));
                                                                    let _e4642 = (_e2395.x * (_e2393.x + _e2393.x));
                                                                    let _e4644 = (_e2395.y * (_e2393.y + _e2393.y));
                                                                    let _e4646 = (_e2395.z * (_e2393.z + _e2393.z));
                                                                    let _e4653 = ((_e4638 * _e4638) + (_e4642 * _e4642));
                                                                    let _e4654 = ((_e4639 * _e4639) + (_e4644 * _e4644));
                                                                    let _e4655 = ((_e4640 * _e4640) + (_e4646 * _e4646));
                                                                    let _e4660 = ((_e4625 * _e4638) / _e4653);
                                                                    let _e4661 = ((_e4625 * _e4639) / _e4654);
                                                                    let _e4662 = ((_e4625 * _e4640) / _e4655);
                                                                    let _e4663 = (1f - _e4660);
                                                                    let _e4664 = (1f - _e4661);
                                                                    let _e4665 = (1f - _e4662);
                                                                    let _e4667 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4660, _e4661, _e4662));
                                                                    let _e4668 = (_e4625 * _e4642);
                                                                    let _e4669 = (_e4625 * _e4644);
                                                                    let _e4670 = (_e4625 * _e4646);
                                                                    let _e4675 = (vec3<f32>(_e4668, _e4669, _e4670) / vec3<f32>(_e4653, _e4654, _e4655));
                                                                    let _e4688 = ((_e4667.x * _e4667.x) + (_e4675.x * _e4675.x));
                                                                    let _e4689 = ((_e4667.y * _e4667.y) + (_e4675.y * _e4675.y));
                                                                    let _e4690 = ((_e4667.z * _e4667.z) + (_e4675.z * _e4675.z));
                                                                    if (_e4688 != _e4688) {
                                                                        phi_28739_ = true;
                                                                    } else {
                                                                        phi_28739_ = (0f >= _e4688);
                                                                    }
                                                                    let _e4694 = phi_28739_;
                                                                    let _e4696 = sqrt(select(_e4688, 0f, _e4694));
                                                                    if (_e4689 != _e4689) {
                                                                        phi_28754_ = true;
                                                                    } else {
                                                                        phi_28754_ = (0f >= _e4689);
                                                                    }
                                                                    let _e4700 = phi_28754_;
                                                                    let _e4702 = sqrt(select(_e4689, 0f, _e4700));
                                                                    if (_e4690 != _e4690) {
                                                                        phi_28769_ = true;
                                                                    } else {
                                                                        phi_28769_ = (0f >= _e4690);
                                                                    }
                                                                    let _e4706 = phi_28769_;
                                                                    let _e4708 = sqrt(select(_e4690, 0f, _e4706));
                                                                    let _e4712 = ((_e4696 + _e4663) * 0.5f);
                                                                    let _e4713 = ((_e4702 + _e4664) * 0.5f);
                                                                    let _e4714 = ((_e4708 + _e4665) * 0.5f);
                                                                    if (_e4712 != _e4712) {
                                                                        phi_28796_ = true;
                                                                    } else {
                                                                        phi_28796_ = (0f >= _e4712);
                                                                    }
                                                                    let _e4718 = phi_28796_;
                                                                    let _e4720 = sqrt(select(_e4712, 0f, _e4718));
                                                                    if (_e4713 != _e4713) {
                                                                        phi_28811_ = true;
                                                                    } else {
                                                                        phi_28811_ = (0f >= _e4713);
                                                                    }
                                                                    let _e4724 = phi_28811_;
                                                                    let _e4726 = sqrt(select(_e4713, 0f, _e4724));
                                                                    if (_e4714 != _e4714) {
                                                                        phi_28826_ = true;
                                                                    } else {
                                                                        phi_28826_ = (0f >= _e4714);
                                                                    }
                                                                    let _e4730 = phi_28826_;
                                                                    let _e4732 = sqrt(select(_e4714, 0f, _e4730));
                                                                    let _e4733 = vec3<f32>(_e4720, _e4726, _e4732);
                                                                    let _e4737 = ((_e4696 - _e4663) * 0.5f);
                                                                    let _e4738 = ((_e4702 - _e4664) * 0.5f);
                                                                    let _e4739 = ((_e4708 - _e4665) * 0.5f);
                                                                    if (_e4737 != _e4737) {
                                                                        phi_28853_ = true;
                                                                    } else {
                                                                        phi_28853_ = (0f >= _e4737);
                                                                    }
                                                                    let _e4743 = phi_28853_;
                                                                    let _e4745 = sqrt(select(_e4737, 0f, _e4743));
                                                                    if (_e4738 != _e4738) {
                                                                        phi_28868_ = true;
                                                                    } else {
                                                                        phi_28868_ = (0f >= _e4738);
                                                                    }
                                                                    let _e4749 = phi_28868_;
                                                                    let _e4751 = sqrt(select(_e4738, 0f, _e4749));
                                                                    if (_e4739 != _e4739) {
                                                                        phi_28883_ = true;
                                                                    } else {
                                                                        phi_28883_ = (0f >= _e4739);
                                                                    }
                                                                    let _e4755 = phi_28883_;
                                                                    let _e4757 = sqrt(select(_e4739, 0f, _e4755));
                                                                    if ((_e4668 / _e4653) < 0f) {
                                                                        phi_18037_ = -(_e4745);
                                                                    } else {
                                                                        phi_18037_ = _e4745;
                                                                    }
                                                                    let _e4761 = phi_18037_;
                                                                    if ((_e4669 / _e4654) < 0f) {
                                                                        phi_18045_ = -(_e4751);
                                                                    } else {
                                                                        phi_18045_ = _e4751;
                                                                    }
                                                                    let _e4765 = phi_18045_;
                                                                    if ((_e4670 / _e4655) < 0f) {
                                                                        phi_18053_ = -(_e4757);
                                                                    } else {
                                                                        phi_18053_ = _e4757;
                                                                    }
                                                                    let _e4769 = phi_18053_;
                                                                    let _e4773 = (_e2393.x * _e4623);
                                                                    let _e4774 = (_e2393.y * _e4623);
                                                                    let _e4775 = (_e2393.z * _e4623);
                                                                    let _e4785 = vec3<f32>(((_e2395.x * _e4720) - (_e2393.x * _e4761)), ((_e2395.y * _e4726) - (_e2393.y * _e4765)), ((_e2395.z * _e4732) - (_e2393.z * _e4769)));
                                                                    let _e4792 = ((_e2395.x * _e4761) + (_e2393.x * _e4720));
                                                                    let _e4793 = ((_e2395.y * _e4765) + (_e2393.y * _e4726));
                                                                    let _e4794 = ((_e2395.z * _e4769) + (_e2393.z * _e4732));
                                                                    let _e4796 = vec3<f32>((_e2395.x * _e4623), (_e2395.y * _e4623), (_e2395.z * _e4623));
                                                                    let _e4797 = (_e4796 - _e4733);
                                                                    let _e4798 = (_e4773 - _e4761);
                                                                    let _e4799 = (_e4774 - _e4765);
                                                                    let _e4800 = (_e4775 - _e4769);
                                                                    let _e4813 = (_e4796 + _e4733);
                                                                    let _e4814 = (_e4773 + _e4761);
                                                                    let _e4815 = (_e4774 + _e4765);
                                                                    let _e4816 = (_e4775 + _e4769);
                                                                    let _e4826 = ((_e4813.x * _e4813.x) + (_e4814 * _e4814));
                                                                    let _e4827 = ((_e4813.y * _e4813.y) + (_e4815 * _e4815));
                                                                    let _e4828 = ((_e4813.z * _e4813.z) + (_e4816 * _e4816));
                                                                    let _e4838 = vec3(_e4623);
                                                                    let _e4839 = (_e4838 - _e4785);
                                                                    let _e4840 = -(vec3<f32>(_e4792, _e4793, _e4794));
                                                                    let _e4856 = (_e4838 + _e4785);
                                                                    let _e4866 = ((_e4856.x * _e4856.x) + (_e4792 * _e4792));
                                                                    let _e4867 = ((_e4856.y * _e4856.y) + (_e4793 * _e4793));
                                                                    let _e4868 = ((_e4856.z * _e4856.z) + (_e4794 * _e4794));
                                                                    phi_18160_ = vec3<f32>((((((_e4797.x * _e4797.x) + (_e4798 * _e4798)) / select(0.000000000000000000000000000001f, _e4826, (_e4826 > 0.000000000000000000000000000001f))) + (((_e4839.x * _e4839.x) + (_e4840.x * _e4840.x)) / select(0.000000000000000000000000000001f, _e4866, (_e4866 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4797.y * _e4797.y) + (_e4799 * _e4799)) / select(0.000000000000000000000000000001f, _e4827, (_e4827 > 0.000000000000000000000000000001f))) + (((_e4839.y * _e4839.y) + (_e4840.y * _e4840.y)) / select(0.000000000000000000000000000001f, _e4867, (_e4867 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4797.z * _e4797.z) + (_e4800 * _e4800)) / select(0.000000000000000000000000000001f, _e4828, (_e4828 > 0.000000000000000000000000000001f))) + (((_e4839.z * _e4839.z) + (_e4840.z * _e4840.z)) / select(0.000000000000000000000000000001f, _e4868, (_e4868 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                }
                                                                let _e4911 = phi_18160_;
                                                                phi_18161_ = _e4911;
                                                            }
                                                            let _e4931 = phi_18161_;
                                                            let _e4933 = (4f * _e4545);
                                                            phi_18173_ = type_52((_e4931 * (((1f / (_e4596 + _e4610)) * _e4581) / _e4933)), ((_e4581 / _e4596) / _e4933));
                                                        }
                                                        let _e4940 = phi_18173_;
                                                        phi_18174_ = _e4940;
                                                        phi_18175_ = _e4557;
                                                    }
                                                    let _e4942 = phi_18174_;
                                                    let _e4944 = phi_18175_;
                                                    if _e4944 {
                                                        phi_18180_ = type_52(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        phi_18180_ = _e4942;
                                                    }
                                                    let _e4946 = phi_18180_;
                                                    let _e4949 = (_e2401 == 1u);
                                                    if _e4949 {
                                                        phi_18208_ = _e4946;
                                                        phi_18209_ = type_52();
                                                    } else {
                                                        if (_e4545 != _e4545) {
                                                            phi_29145_ = true;
                                                        } else {
                                                            phi_29145_ = (0f >= _e4545);
                                                        }
                                                        let _e4953 = phi_29145_;
                                                        let _e4954 = select(_e4545, 0f, _e4953);
                                                        let _e4955 = (1f / _e2391);
                                                        let _e4959 = ((_e4955 * _e4955) * (1f - (_e4954 * _e4954)));
                                                        if (_e4959 >= 1f) {
                                                            phi_29164_ = 1f;
                                                        } else {
                                                            let _e4962 = sqrt((1f - _e4959));
                                                            if (_e4954 != _e4954) {
                                                                phi_29175_ = true;
                                                            } else {
                                                                phi_29175_ = (_e4962 <= _e4954);
                                                            }
                                                            let _e4966 = phi_29175_;
                                                            let _e4971 = pow(((1f - _e2391) / (1f + _e2391)), 2f);
                                                            phi_29164_ = (_e4971 + ((1f - _e4971) * pow((1f - select(_e4954, _e4962, _e4966)), 5f)));
                                                        }
                                                        let _e4978 = phi_29164_;
                                                        if (_e4523 <= 0f) {
                                                            phi_29214_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_29214_ = (_e2389 * (_e4523 * 0.31830987f));
                                                        }
                                                        let _e4983 = phi_29214_;
                                                        let _e4984 = (1f - _e4978);
                                                        phi_18208_ = type_52();
                                                        phi_18209_ = type_52((_e4946.member + vec3<f32>((_e4983.x * _e4984), (_e4983.y * _e4984), (_e4983.z * _e4984))), ((_e4978 * _e4946.member_1) + (_e4984 * _e4529)));
                                                    }
                                                    let _e4998 = phi_18208_;
                                                    let _e5000 = phi_18209_;
                                                    phi_18210_ = _e4998;
                                                    phi_18211_ = _e5000;
                                                    phi_18212_ = _e4949;
                                                }
                                                let _e5008 = phi_18210_;
                                                let _e5010 = phi_18211_;
                                                let _e5012 = phi_18212_;
                                                if _e5012 {
                                                    phi_18216_ = _e5008;
                                                } else {
                                                    phi_18216_ = _e5010;
                                                }
                                                let _e5014 = phi_18216_;
                                                if (_e4500 > 0f) {
                                                    if (_e4500 >= 300000000000000000000000000000000000000f) {
                                                        phi_29253_ = 1f;
                                                    } else {
                                                        let _e5019 = (_e4500 * _e4500);
                                                        let _e5021 = (_e5019 + (_e5014.member_1 * _e5014.member_1));
                                                        if (_e5021 == 0f) {
                                                            phi_29250_ = 0f;
                                                        } else {
                                                            phi_29250_ = (_e5019 / _e5021);
                                                        }
                                                        let _e5025 = phi_29250_;
                                                        phi_29253_ = _e5025;
                                                    }
                                                    let _e5027 = phi_29253_;
                                                    phi_18225_ = (_e5027 / _e4500);
                                                } else {
                                                    phi_18225_ = 1f;
                                                }
                                                let _e5030 = phi_18225_;
                                                if ((((_e5014.member.x * _e5014.member.x) + (_e5014.member.y * _e5014.member.y)) + (_e5014.member.z * _e5014.member.z)) > 0f) {
                                                    let _e5042 = (vec3<f32>(1f, 1f, 1f) / _e4502);
                                                    local_67 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                    phi_18259_ = _e370;
                                                    phi_18262_ = _e377;
                                                    phi_18264_ = _e374;
                                                    phi_18268_ = type_59(_e4506, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                    loop {
                                                        let _e5044 = phi_18259_;
                                                        let _e5046 = phi_18262_;
                                                        let _e5048 = phi_18264_;
                                                        let _e5050 = phi_18268_;
                                                        local_106 = _e5050;
                                                        if (_e5044 < _e5048) {
                                                            let _e5219 = global_2.member[_e5044];
                                                            if (_e5219.member_6.x == 0u) {
                                                                let _e5325 = (_e5219.member_1.x - _e5219.member.x);
                                                                let _e5326 = (_e5219.member_1.y - _e5219.member.y);
                                                                let _e5327 = (_e5219.member_1.z - _e5219.member.z);
                                                                let _e5332 = (_e5219.member_2.x - _e5219.member.x);
                                                                let _e5333 = (_e5219.member_2.y - _e5219.member.y);
                                                                let _e5334 = (_e5219.member_2.z - _e5219.member.z);
                                                                let _e5337 = ((_e4502.y * _e5334) - (_e5333 * _e4502.z));
                                                                let _e5340 = ((_e4502.z * _e5332) - (_e5334 * _e4502.x));
                                                                let _e5343 = ((_e4502.x * _e5333) - (_e5332 * _e4502.y));
                                                                let _e5348 = (((_e5325 * _e5337) + (_e5326 * _e5340)) + (_e5327 * _e5343));
                                                                if (abs(_e5348) < 0.000000000001f) {
                                                                    phi_18435_ = vec4<f32>();
                                                                    phi_18436_ = true;
                                                                } else {
                                                                    let _e5351 = (1f / _e5348);
                                                                    let _e5352 = (_e1534 - _e5219.member.x);
                                                                    let _e5353 = (_e1535 - _e5219.member.y);
                                                                    let _e5354 = (_e1536 - _e5219.member.z);
                                                                    let _e5360 = ((((_e5352 * _e5337) + (_e5353 * _e5340)) + (_e5354 * _e5343)) * _e5351);
                                                                    if (_e5360 < 0f) {
                                                                        phi_18425_ = vec4<f32>();
                                                                        phi_18426_ = true;
                                                                        phi_18427_ = false;
                                                                    } else {
                                                                        let _e5362 = (_e5360 > 1f);
                                                                        if _e5362 {
                                                                            phi_18423_ = vec4<f32>();
                                                                            phi_18424_ = false;
                                                                        } else {
                                                                            let _e5365 = ((_e5353 * _e5327) - (_e5326 * _e5354));
                                                                            let _e5368 = ((_e5354 * _e5325) - (_e5327 * _e5352));
                                                                            let _e5371 = ((_e5352 * _e5326) - (_e5325 * _e5353));
                                                                            let _e5377 = ((((_e4502.x * _e5365) + (_e4502.y * _e5368)) + (_e4502.z * _e5371)) * _e5351);
                                                                            if (_e5377 < 0f) {
                                                                                phi_18413_ = vec4<f32>();
                                                                                phi_18414_ = true;
                                                                                phi_18415_ = false;
                                                                            } else {
                                                                                let _e5380 = ((_e5360 + _e5377) > 1f);
                                                                                if _e5380 {
                                                                                    phi_18411_ = vec4<f32>();
                                                                                    phi_18412_ = false;
                                                                                } else {
                                                                                    let _e5386 = ((((_e5332 * _e5365) + (_e5333 * _e5368)) + (_e5334 * _e5371)) * _e5351);
                                                                                    if (_e5386 <= 0f) {
                                                                                        phi_18406_ = vec4<f32>();
                                                                                        phi_18407_ = true;
                                                                                    } else {
                                                                                        let _e5388 = (_e5386 >= _e5050.member);
                                                                                        if _e5388 {
                                                                                            phi_18405_ = vec4<f32>();
                                                                                        } else {
                                                                                            phi_18405_ = vec4<f32>(_e5386, _e5360, _e5377, 1f);
                                                                                        }
                                                                                        let _e5391 = phi_18405_;
                                                                                        phi_18406_ = _e5391;
                                                                                        phi_18407_ = _e5388;
                                                                                    }
                                                                                    let _e5393 = phi_18406_;
                                                                                    let _e5395 = phi_18407_;
                                                                                    phi_18411_ = _e5393;
                                                                                    phi_18412_ = _e5395;
                                                                                }
                                                                                let _e5397 = phi_18411_;
                                                                                let _e5399 = phi_18412_;
                                                                                phi_18413_ = _e5397;
                                                                                phi_18414_ = _e5380;
                                                                                phi_18415_ = _e5399;
                                                                            }
                                                                            let _e5401 = phi_18413_;
                                                                            let _e5403 = phi_18414_;
                                                                            let _e5405 = phi_18415_;
                                                                            phi_18423_ = _e5401;
                                                                            phi_18424_ = select(_e5405, true, _e5403);
                                                                        }
                                                                        let _e5408 = phi_18423_;
                                                                        let _e5410 = phi_18424_;
                                                                        phi_18425_ = _e5408;
                                                                        phi_18426_ = _e5362;
                                                                        phi_18427_ = _e5410;
                                                                    }
                                                                    let _e5412 = phi_18425_;
                                                                    let _e5414 = phi_18426_;
                                                                    let _e5416 = phi_18427_;
                                                                    phi_18435_ = _e5412;
                                                                    phi_18436_ = select(_e5416, true, _e5414);
                                                                }
                                                                let _e5419 = phi_18435_;
                                                                let _e5421 = phi_18436_;
                                                                phi_18582_ = _e5419;
                                                                phi_18583_ = _e5421;
                                                            } else {
                                                                if (_e5219.member_6.x == 1u) {
                                                                    let _e5247 = (_e1534 - _e5219.member.x);
                                                                    let _e5248 = (_e1535 - _e5219.member.y);
                                                                    let _e5249 = (_e1536 - _e5219.member.z);
                                                                    let _e5254 = (((_e5247 * _e4502.x) + (_e5248 * _e4502.y)) + (_e5249 * _e4502.z));
                                                                    if (_e5254 > 0f) {
                                                                        phi_18476_ = select(true, false, ((((_e5247 * _e5247) + (_e5248 * _e5248)) + (_e5249 * _e5249)) > (_e5219.member.w * _e5219.member.w)));
                                                                    } else {
                                                                        phi_18476_ = true;
                                                                    }
                                                                    let _e5267 = phi_18476_;
                                                                    if _e5267 {
                                                                        let _e5271 = (_e5247 - (_e4502.x * _e5254));
                                                                        let _e5272 = (_e5248 - (_e4502.y * _e5254));
                                                                        let _e5273 = (_e5249 - (_e4502.z * _e5254));
                                                                        let _e5282 = ((_e5219.member.w * _e5219.member.w) - (((_e5271 * _e5271) + (_e5272 * _e5272)) + (_e5273 * _e5273)));
                                                                        if (_e5282 < 0f) {
                                                                            phi_18535_ = vec4<f32>();
                                                                            phi_18536_ = true;
                                                                        } else {
                                                                            let _e5284 = sqrt(_e5282);
                                                                            let _e5285 = -(_e5254);
                                                                            let _e5286 = (_e5285 - _e5284);
                                                                            if (_e5286 <= 0f) {
                                                                                phi_18513_ = true;
                                                                            } else {
                                                                                phi_18513_ = (_e5286 >= _e5050.member);
                                                                            }
                                                                            let _e5290 = phi_18513_;
                                                                            if _e5290 {
                                                                                phi_18519_ = (_e5285 + _e5284);
                                                                            } else {
                                                                                phi_18519_ = _e5286;
                                                                            }
                                                                            let _e5293 = phi_18519_;
                                                                            if (_e5293 <= 0f) {
                                                                                phi_18530_ = vec4<f32>();
                                                                                phi_18531_ = true;
                                                                            } else {
                                                                                let _e5295 = (_e5293 >= _e5050.member);
                                                                                if _e5295 {
                                                                                    phi_18529_ = vec4<f32>();
                                                                                } else {
                                                                                    phi_18529_ = vec4<f32>(_e5293, 0f, 0f, 1f);
                                                                                }
                                                                                let _e5298 = phi_18529_;
                                                                                phi_18530_ = _e5298;
                                                                                phi_18531_ = _e5295;
                                                                            }
                                                                            let _e5300 = phi_18530_;
                                                                            let _e5302 = phi_18531_;
                                                                            phi_18535_ = _e5300;
                                                                            phi_18536_ = _e5302;
                                                                        }
                                                                        let _e5304 = phi_18535_;
                                                                        let _e5306 = phi_18536_;
                                                                        phi_18540_ = _e5304;
                                                                        phi_18541_ = _e5306;
                                                                    } else {
                                                                        phi_18540_ = vec4<f32>();
                                                                        phi_18541_ = true;
                                                                    }
                                                                    let _e5308 = phi_18540_;
                                                                    let _e5310 = phi_18541_;
                                                                    phi_18572_ = _e5308;
                                                                    phi_18573_ = _e5310;
                                                                } else {
                                                                    if (abs(_e4502.y) < 0.000000000001f) {
                                                                        phi_18570_ = vec4<f32>();
                                                                        phi_18571_ = true;
                                                                    } else {
                                                                        let _e5229 = ((_e5219.member.x - _e1535) / _e4502.y);
                                                                        if (_e5229 <= 0f) {
                                                                            phi_18565_ = vec4<f32>();
                                                                            phi_18566_ = true;
                                                                        } else {
                                                                            let _e5231 = (_e5229 >= _e5050.member);
                                                                            if _e5231 {
                                                                                phi_18564_ = vec4<f32>();
                                                                            } else {
                                                                                phi_18564_ = vec4<f32>(_e5229, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5234 = phi_18564_;
                                                                            phi_18565_ = _e5234;
                                                                            phi_18566_ = _e5231;
                                                                        }
                                                                        let _e5236 = phi_18565_;
                                                                        let _e5238 = phi_18566_;
                                                                        phi_18570_ = _e5236;
                                                                        phi_18571_ = _e5238;
                                                                    }
                                                                    let _e5240 = phi_18570_;
                                                                    let _e5242 = phi_18571_;
                                                                    phi_18572_ = _e5240;
                                                                    phi_18573_ = _e5242;
                                                                }
                                                                let _e5312 = phi_18572_;
                                                                let _e5314 = phi_18573_;
                                                                phi_18582_ = _e5312;
                                                                phi_18583_ = select(true, false, select(true, false, _e5314));
                                                            }
                                                            let _e5423 = phi_18582_;
                                                            let _e5425 = phi_18583_;
                                                            let _e5428 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5423, vec4(select(true, false, _e5425)));
                                                            let _e5430 = (_e5428.w > 0f);
                                                            if _e5430 {
                                                                let _e5436 = type_59(_e5428.x, _e5050.member_1, _e5050.member_2, _e5050.member_3);
                                                                let _e5441 = type_59(_e5436.member, _e5044, _e5436.member_2, _e5436.member_3);
                                                                phi_18604_ = _e5441;
                                                                phi_18606_ = _e5441;
                                                            } else {
                                                                phi_18604_ = type_59();
                                                                phi_18606_ = _e5050;
                                                            }
                                                            let _e5443 = phi_18604_;
                                                            let _e5445 = phi_18606_;
                                                            let _e5446 = select(true, false, _e5430);
                                                            if _e5446 {
                                                                phi_18612_ = (_e5044 + 1u);
                                                            } else {
                                                                phi_18612_ = u32();
                                                            }
                                                            let _e5449 = phi_18612_;
                                                            phi_18260_ = _e5449;
                                                            phi_18263_ = _e5046;
                                                            phi_18265_ = _e5048;
                                                            phi_18269_ = _e5445;
                                                            phi_18685_ = _e5443;
                                                            phi_18686_ = _e5446;
                                                            phi_18687_ = false;
                                                        } else {
                                                            let _e5052 = (_e5046 == 0u);
                                                            if _e5052 {
                                                                phi_18681_ = u32();
                                                                phi_18682_ = u32();
                                                                phi_18683_ = u32();
                                                            } else {
                                                                let _e5053 = (_e5046 - 1u);
                                                                let _e5054 = (_e5053 < 100u);
                                                                if _e5054 {
                                                                } else {
                                                                    phi_34410_ = true;
                                                                    phi_20549_ = bool();
                                                                    phi_20548_ = type_59();
                                                                    break;
                                                                }
                                                                let _e5056 = local_67[_e5053];
                                                                let _e5058 = global_3.member[_e5056];
                                                                let _e5069 = ((_e5058.member.x - _e1534) * _e5042.x);
                                                                let _e5071 = ((_e5058.member.y - _e1535) * _e5042.y);
                                                                let _e5073 = ((_e5058.member.z - _e1536) * _e5042.z);
                                                                let _e5080 = ((_e5058.member_2.x - _e1534) * _e5042.x);
                                                                let _e5081 = ((_e5058.member_2.y - _e1535) * _e5042.y);
                                                                let _e5082 = ((_e5058.member_2.z - _e1536) * _e5042.z);
                                                                if (_e5069 == _e5069) {
                                                                    let _e5084 = (_e5080 == _e5080);
                                                                    if _e5084 {
                                                                        let _e5085 = (_e5069 != _e5069);
                                                                        if _e5085 {
                                                                            phi_29350_ = true;
                                                                        } else {
                                                                            phi_29350_ = (_e5080 <= _e5069);
                                                                        }
                                                                        let _e5088 = phi_29350_;
                                                                        let _e5089 = select(_e5069, _e5080, _e5088);
                                                                        if _e5085 {
                                                                            phi_29380_ = true;
                                                                        } else {
                                                                            phi_29380_ = (_e5080 >= _e5069);
                                                                        }
                                                                        let _e5094 = phi_29380_;
                                                                        let _e5095 = select(_e5069, _e5080, _e5094);
                                                                        phi_29327_ = type_49(select(-300000000000000000000000000000000000000f, _e5089, (_e5089 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5095, (_e5095 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_29327_ = type_49();
                                                                    }
                                                                    let _e5100 = phi_29327_;
                                                                    phi_29330_ = _e5100;
                                                                    phi_29331_ = select(true, false, _e5084);
                                                                } else {
                                                                    phi_29330_ = type_49();
                                                                    phi_29331_ = true;
                                                                }
                                                                let _e5103 = phi_29330_;
                                                                let _e5105 = phi_29331_;
                                                                if _e5105 {
                                                                    phi_29336_ = type_49(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_29336_ = _e5103;
                                                                }
                                                                let _e5107 = phi_29336_;
                                                                if (_e5071 == _e5071) {
                                                                    let _e5111 = (_e5081 == _e5081);
                                                                    if _e5111 {
                                                                        let _e5112 = (_e5071 != _e5071);
                                                                        if _e5112 {
                                                                            phi_29437_ = true;
                                                                        } else {
                                                                            phi_29437_ = (_e5081 <= _e5071);
                                                                        }
                                                                        let _e5115 = phi_29437_;
                                                                        let _e5116 = select(_e5071, _e5081, _e5115);
                                                                        if (_e5107.member != _e5107.member) {
                                                                            phi_29452_ = true;
                                                                        } else {
                                                                            phi_29452_ = (_e5116 >= _e5107.member);
                                                                        }
                                                                        let _e5120 = phi_29452_;
                                                                        if _e5112 {
                                                                            phi_29467_ = true;
                                                                        } else {
                                                                            phi_29467_ = (_e5081 >= _e5071);
                                                                        }
                                                                        let _e5124 = phi_29467_;
                                                                        let _e5125 = select(_e5071, _e5081, _e5124);
                                                                        if (_e5107.member_1 != _e5107.member_1) {
                                                                            phi_29482_ = true;
                                                                        } else {
                                                                            phi_29482_ = (_e5125 <= _e5107.member_1);
                                                                        }
                                                                        let _e5129 = phi_29482_;
                                                                        phi_29414_ = type_49(select(_e5107.member, _e5116, _e5120), select(_e5107.member_1, _e5125, _e5129));
                                                                    } else {
                                                                        phi_29414_ = type_49();
                                                                    }
                                                                    let _e5133 = phi_29414_;
                                                                    phi_29417_ = _e5133;
                                                                    phi_29418_ = select(true, false, _e5111);
                                                                } else {
                                                                    phi_29417_ = type_49();
                                                                    phi_29418_ = true;
                                                                }
                                                                let _e5136 = phi_29417_;
                                                                let _e5138 = phi_29418_;
                                                                if _e5138 {
                                                                    phi_29423_ = _e5107;
                                                                } else {
                                                                    phi_29423_ = _e5136;
                                                                }
                                                                let _e5140 = phi_29423_;
                                                                if (_e5073 == _e5073) {
                                                                    let _e5144 = (_e5082 == _e5082);
                                                                    if _e5144 {
                                                                        let _e5145 = (_e5073 != _e5073);
                                                                        if _e5145 {
                                                                            phi_29524_ = true;
                                                                        } else {
                                                                            phi_29524_ = (_e5082 <= _e5073);
                                                                        }
                                                                        let _e5148 = phi_29524_;
                                                                        let _e5149 = select(_e5073, _e5082, _e5148);
                                                                        if (_e5140.member != _e5140.member) {
                                                                            phi_29539_ = true;
                                                                        } else {
                                                                            phi_29539_ = (_e5149 >= _e5140.member);
                                                                        }
                                                                        let _e5153 = phi_29539_;
                                                                        if _e5145 {
                                                                            phi_29554_ = true;
                                                                        } else {
                                                                            phi_29554_ = (_e5082 >= _e5073);
                                                                        }
                                                                        let _e5157 = phi_29554_;
                                                                        let _e5158 = select(_e5073, _e5082, _e5157);
                                                                        if (_e5140.member_1 != _e5140.member_1) {
                                                                            phi_29569_ = true;
                                                                        } else {
                                                                            phi_29569_ = (_e5158 <= _e5140.member_1);
                                                                        }
                                                                        let _e5162 = phi_29569_;
                                                                        phi_29501_ = type_49(select(_e5140.member, _e5149, _e5153), select(_e5140.member_1, _e5158, _e5162));
                                                                    } else {
                                                                        phi_29501_ = type_49();
                                                                    }
                                                                    let _e5166 = phi_29501_;
                                                                    phi_29504_ = _e5166;
                                                                    phi_29505_ = select(true, false, _e5144);
                                                                } else {
                                                                    phi_29504_ = type_49();
                                                                    phi_29505_ = true;
                                                                }
                                                                let _e5169 = phi_29504_;
                                                                let _e5171 = phi_29505_;
                                                                if _e5171 {
                                                                    phi_29510_ = _e5140;
                                                                } else {
                                                                    phi_29510_ = _e5169;
                                                                }
                                                                let _e5173 = phi_29510_;
                                                                if (_e5173.member_1 < 0f) {
                                                                    phi_29306_ = bool();
                                                                    phi_29307_ = true;
                                                                } else {
                                                                    let _e5177 = (_e5173.member > _e5173.member_1);
                                                                    if _e5177 {
                                                                        phi_29303_ = bool();
                                                                    } else {
                                                                        phi_29303_ = (_e5173.member > _e5050.member);
                                                                    }
                                                                    let _e5180 = phi_29303_;
                                                                    phi_29306_ = _e5180;
                                                                    phi_29307_ = _e5177;
                                                                }
                                                                let _e5182 = phi_29306_;
                                                                let _e5184 = phi_29307_;
                                                                if !(select(_e5182, true, _e5184)) {
                                                                    if (_e5058.member_3 > 0u) {
                                                                        phi_18675_ = _e5058.member_1;
                                                                        phi_18676_ = _e5053;
                                                                        phi_18677_ = (_e5058.member_1 + _e5058.member_3);
                                                                    } else {
                                                                        let _e5189 = (_e5046 + 1u);
                                                                        let _e5190 = (_e5189 <= 100u);
                                                                        if _e5190 {
                                                                            if _e5054 {
                                                                            } else {
                                                                                phi_34410_ = true;
                                                                                phi_20549_ = bool();
                                                                                phi_20548_ = type_59();
                                                                                break;
                                                                            }
                                                                            local_67[_e5053] = _e5058.member_1;
                                                                            if (_e5046 < 100u) {
                                                                            } else {
                                                                                phi_34410_ = true;
                                                                                phi_20549_ = bool();
                                                                                phi_20548_ = type_59();
                                                                                break;
                                                                            }
                                                                            local_67[_e5046] = (_e5056 + 1u);
                                                                        }
                                                                        phi_18675_ = _e5044;
                                                                        phi_18676_ = select(_e5053, _e5189, _e5190);
                                                                        phi_18677_ = _e5048;
                                                                    }
                                                                    let _e5199 = phi_18675_;
                                                                    let _e5201 = phi_18676_;
                                                                    let _e5203 = phi_18677_;
                                                                    phi_18678_ = _e5199;
                                                                    phi_18679_ = _e5201;
                                                                    phi_18680_ = _e5203;
                                                                } else {
                                                                    phi_18678_ = _e5044;
                                                                    phi_18679_ = _e5053;
                                                                    phi_18680_ = _e5048;
                                                                }
                                                                let _e5205 = phi_18678_;
                                                                let _e5207 = phi_18679_;
                                                                let _e5209 = phi_18680_;
                                                                phi_18681_ = _e5205;
                                                                phi_18682_ = _e5207;
                                                                phi_18683_ = _e5209;
                                                            }
                                                            let _e5211 = phi_18681_;
                                                            let _e5213 = phi_18682_;
                                                            let _e5215 = phi_18683_;
                                                            phi_18260_ = _e5211;
                                                            phi_18263_ = _e5213;
                                                            phi_18265_ = _e5215;
                                                            phi_18269_ = _e5050;
                                                            phi_18685_ = type_59();
                                                            phi_18686_ = select(true, false, _e5052);
                                                            phi_18687_ = _e5052;
                                                        }
                                                        let _e5451 = phi_18260_;
                                                        let _e5453 = phi_18263_;
                                                        let _e5455 = phi_18265_;
                                                        let _e5457 = phi_18269_;
                                                        let _e5459 = phi_18685_;
                                                        let _e5461 = phi_18686_;
                                                        let _e5463 = phi_18687_;
                                                        phi_18259_ = _e5451;
                                                        phi_18262_ = _e5453;
                                                        phi_18264_ = _e5455;
                                                        phi_18268_ = _e5457;
                                                        phi_34410_ = _e2414;
                                                        phi_20549_ = _e5463;
                                                        phi_20548_ = _e5459;
                                                        if !(_e5461) {
                                                            break;
                                                        }
                                                    }
                                                    let _e5466 = phi_34410_;
                                                    let _e5468 = phi_20549_;
                                                    let _e5470 = phi_20548_;
                                                    phi_34809_ = _e4498;
                                                    phi_34795_ = _e5466;
                                                    if _e5466 {
                                                        break;
                                                    }
                                                    if _e5468 {
                                                        let _e8000 = local_106;
                                                        phi_18859_ = _e8000;
                                                    } else {
                                                        phi_18859_ = type_59();
                                                    }
                                                    let _e5472 = phi_18859_;
                                                    if select(true, false, _e5468) {
                                                        phi_18864_ = _e5470;
                                                    } else {
                                                        phi_18864_ = _e5472;
                                                    }
                                                    let _e5475 = phi_18864_;
                                                    if (_e5475.member_1 != 4294967295u) {
                                                        phi_18918_ = _e2418;
                                                        phi_18919_ = _e2420;
                                                    } else {
                                                        if (_e2422 == (_e2412 + 2u)) {
                                                            phi_18916_ = _e2418;
                                                            phi_18917_ = type_34(vec4<f32>(_e1534, _e1535, _e1536, -1f), vec4<f32>(_e4502.x, _e4502.y, _e4502.z, 0f), vec4<f32>(((_e1404.x * _e5014.member.x) * _e5030), ((_e1404.y * _e5014.member.y) * _e5030), ((_e1404.z * _e5014.member.z) * _e5030), 0f));
                                                        } else {
                                                            phi_18916_ = vec3<f32>((_e2418.x + (((_e1404.x * _e5014.member.x) * _e4504.x) * _e5030)), (_e2418.y + (((_e1404.y * _e5014.member.y) * _e4504.y) * _e5030)), (_e2418.z + (((_e1404.z * _e5014.member.z) * _e4504.z) * _e5030)));
                                                            phi_18917_ = _e2420;
                                                        }
                                                        let _e5513 = phi_18916_;
                                                        let _e5515 = phi_18917_;
                                                        phi_18918_ = _e5513;
                                                        phi_18919_ = _e5515;
                                                    }
                                                    let _e5517 = phi_18918_;
                                                    let _e5519 = phi_18919_;
                                                    phi_34798_ = _e5466;
                                                    phi_18920_ = _e5517;
                                                    phi_18921_ = _e5519;
                                                    phi_18922_ = (_e2424 + 1u);
                                                } else {
                                                    phi_34798_ = _e2414;
                                                    phi_18920_ = _e2418;
                                                    phi_18921_ = _e2420;
                                                    phi_18922_ = _e2424;
                                                }
                                                let _e5521 = phi_34798_;
                                                let _e5523 = phi_18920_;
                                                let _e5525 = phi_18921_;
                                                let _e5527 = phi_18922_;
                                                phi_34797_ = _e5521;
                                                phi_18923_ = _e5523;
                                                phi_18924_ = _e5525;
                                                phi_18925_ = _e5527;
                                            } else {
                                                phi_34797_ = _e2414;
                                                phi_18923_ = _e2418;
                                                phi_18924_ = _e2420;
                                                phi_18925_ = _e2424;
                                            }
                                            let _e5529 = phi_34797_;
                                            let _e5531 = phi_18923_;
                                            let _e5533 = phi_18924_;
                                            let _e5535 = phi_18925_;
                                            phi_35040_ = _e4498;
                                            phi_34796_ = _e5529;
                                            phi_14584_ = _e5531;
                                            phi_14587_ = _e5533;
                                            phi_14589_ = (_e2422 + 1u);
                                            phi_14591_ = _e5535;
                                        } else {
                                            phi_35040_ = _e2416;
                                            phi_34796_ = _e2414;
                                            phi_14584_ = vec3<f32>();
                                            phi_14587_ = type_34();
                                            phi_14589_ = u32();
                                            phi_14591_ = u32();
                                        }
                                        let _e5538 = phi_35040_;
                                        let _e5540 = phi_34796_;
                                        let _e5542 = phi_14584_;
                                        let _e5544 = phi_14587_;
                                        let _e5546 = phi_14589_;
                                        let _e5548 = phi_14591_;
                                        phi_34490_ = _e5540;
                                        phi_32557_ = _e5538;
                                        phi_14583_ = _e5542;
                                        phi_14586_ = _e5544;
                                        phi_14588_ = _e5546;
                                        phi_14590_ = _e5548;
                                        phi_34809_ = _e5538;
                                        phi_34795_ = _e5540;
                                        if !(_e2426) {
                                            break;
                                        }
                                    }
                                    let _e5551 = phi_34809_;
                                    let _e5553 = phi_34795_;
                                    if _e5553 {
                                        break;
                                    }
                                    if (_e345 >= _e361) {
                                        let _e6557 = local_100;
                                        let _e6560 = local_101;
                                        let _e6563 = local_102;
                                        let _e6566 = local_103;
                                        let _e6572 = local_104;
                                        phi_19476_ = type_64(type_30(_e318.member, _e318.member_1, vec4<f32>(_e1404.x, _e1404.y, _e1404.z, f32(_e345)), vec4<f32>(_e6560.x, _e6563.y, _e6566.z, f32(_e6557)), vec4<u32>(_e5551, (_e2409 & 4294967294u), 0u, 0u)), _e1406, _e6572);
                                        phi_19477_ = type_64();
                                        phi_19478_ = true;
                                    } else {
                                        let _e5556 = ((_e5551 * 747796405u) + 2891336453u);
                                        let _e5564 = (((_e5556 >> bitcast<u32>((((_e5556 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5556) * 277803737u);
                                        let _e5567 = ((_e5564 >> bitcast<u32>(22i)) ^ _e5564);
                                        let _e5570 = f32((_e5567 >> bitcast<u32>(8i)));
                                        let _e5573 = ((_e5567 * 747796405u) + 2891336453u);
                                        let _e5581 = (((_e5573 >> bitcast<u32>((((_e5573 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5573) * 277803737u);
                                        let _e5584 = ((_e5581 >> bitcast<u32>(22i)) ^ _e5581);
                                        let _e5587 = f32((_e5584 >> bitcast<u32>(8i)));
                                        let _e5589 = ((_e5584 * 747796405u) + 2891336453u);
                                        let _e5597 = (((_e5589 >> bitcast<u32>((((_e5589 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5589) * 277803737u);
                                        let _e5600 = ((_e5597 >> bitcast<u32>(22i)) ^ _e5597);
                                        let _e5605 = (_e2401 == 0u);
                                        if _e5605 {
                                            phi_29679_ = vec3<f32>();
                                            phi_29680_ = true;
                                        } else {
                                            if (_e2401 == 2u) {
                                                let _e5611 = (((_e1481 * _e1509.x) + (_e1482 * _e1509.y)) + (_e1483 * _e1509.z));
                                                if (_e5611 != _e5611) {
                                                    phi_29720_ = true;
                                                } else {
                                                    phi_29720_ = (0f >= _e5611);
                                                }
                                                let _e5615 = phi_29720_;
                                                let _e5616 = select(_e5611, 0f, _e5615);
                                                let _e5617 = (1f / _e2391);
                                                let _e5621 = ((_e5617 * _e5617) * (1f - (_e5616 * _e5616)));
                                                if (_e5621 >= 1f) {
                                                    phi_29739_ = 1f;
                                                } else {
                                                    let _e5624 = sqrt((1f - _e5621));
                                                    if (_e5616 != _e5616) {
                                                        phi_29750_ = true;
                                                    } else {
                                                        phi_29750_ = (_e5624 <= _e5616);
                                                    }
                                                    let _e5628 = phi_29750_;
                                                    let _e5633 = pow(((1f - _e2391) / (1f + _e2391)), 2f);
                                                    phi_29739_ = (_e5633 + ((1f - _e5633) * pow((1f - select(_e5616, _e5624, _e5628)), 5f)));
                                                }
                                                let _e5640 = phi_29739_;
                                                phi_29670_ = select(true, false, ((f32((_e5600 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5640));
                                            } else {
                                                phi_29670_ = true;
                                            }
                                            let _e5644 = phi_29670_;
                                            if _e5644 {
                                                let _e5649 = (((_e1481 * _e1557) + (_e1482 * _e1558)) + (_e1483 * _e1560));
                                                let _e5654 = (((_e1481 * _e1553) + (_e1482 * _e1563)) + (_e1483 * _e1564));
                                                let _e5659 = (((_e1481 * _e1509.x) + (_e1482 * _e1509.y)) + (_e1483 * _e1509.z));
                                                if (_e5659 <= 0f) {
                                                    phi_29904_ = vec3<f32>();
                                                    phi_29905_ = true;
                                                } else {
                                                    let _e5662 = (_e2399 * _e5649);
                                                    let _e5663 = (_e2399 * _e5654);
                                                    let _e5670 = (1f / sqrt((((_e5662 * _e5662) + (_e5663 * _e5663)) + (_e5659 * _e5659))));
                                                    let _e5671 = (_e5662 * _e5670);
                                                    let _e5672 = (_e5663 * _e5670);
                                                    let _e5673 = (_e5659 * _e5670);
                                                    let _e5676 = ((_e5671 * _e5671) + (_e5672 * _e5672));
                                                    if (_e5676 > 0f) {
                                                        let _e5679 = sqrt(_e5676);
                                                        phi_29806_ = vec3<f32>((-(_e5672) / _e5679), (_e5671 / _e5679), 0f);
                                                    } else {
                                                        phi_29806_ = vec3<f32>(1f, 0f, 0f);
                                                    }
                                                    let _e5684 = phi_29806_;
                                                    let _e5697 = sqrt((_e5570 * 0.000000059604645f));
                                                    let _e5698 = (_e5587 * 0.00000037450704f);
                                                    let _e5700 = (_e5697 * cos(_e5698));
                                                    let _e5702 = (0.5f * (1f + _e5673));
                                                    let _e5705 = (1f - (_e5700 * _e5700));
                                                    if (_e5705 != _e5705) {
                                                        phi_29962_ = true;
                                                    } else {
                                                        phi_29962_ = (0f >= _e5705);
                                                    }
                                                    let _e5709 = phi_29962_;
                                                    let _e5716 = (((1f - _e5702) * sqrt(select(_e5705, 0f, _e5709))) + ((_e5702 * _e5697) * sin(_e5698)));
                                                    let _e5727 = (_e5705 - (_e5716 * _e5716));
                                                    if (_e5727 != _e5727) {
                                                        phi_29977_ = true;
                                                    } else {
                                                        phi_29977_ = (0f >= _e5727);
                                                    }
                                                    let _e5731 = phi_29977_;
                                                    let _e5733 = sqrt(select(_e5727, 0f, _e5731));
                                                    let _e5739 = (((_e5684.z * _e5700) + (((_e5671 * _e5684.y) - (_e5684.x * _e5672)) * _e5716)) + (_e5673 * _e5733));
                                                    let _e5740 = (_e2399 * (((_e5684.x * _e5700) + (((_e5672 * _e5684.z) - (_e5684.y * _e5673)) * _e5716)) + (_e5671 * _e5733)));
                                                    let _e5741 = (_e2399 * (((_e5684.y * _e5700) + (((_e5673 * _e5684.x) - (_e5684.z * _e5671)) * _e5716)) + (_e5672 * _e5733)));
                                                    if (_e5739 != _e5739) {
                                                        phi_29992_ = true;
                                                    } else {
                                                        phi_29992_ = (0f >= _e5739);
                                                    }
                                                    let _e5745 = phi_29992_;
                                                    let _e5746 = select(_e5739, 0f, _e5745);
                                                    let _e5753 = (1f / sqrt((((_e5740 * _e5740) + (_e5741 * _e5741)) + (_e5746 * _e5746))));
                                                    let _e5754 = (_e5740 * _e5753);
                                                    let _e5755 = (_e5741 * _e5753);
                                                    let _e5756 = (_e5746 * _e5753);
                                                    let _e5762 = (2f * (((_e5649 * _e5754) + (_e5654 * _e5755)) + (_e5659 * _e5756)));
                                                    let _e5765 = (_e5756 * _e5762);
                                                    let _e5768 = (vec3<f32>((_e5754 * _e5762), (_e5755 * _e5762), _e5765) - vec3<f32>(_e5649, _e5654, _e5659));
                                                    let _e5769 = ((_e5765 - _e5659) <= 0f);
                                                    if _e5769 {
                                                        phi_29901_ = vec3<f32>();
                                                    } else {
                                                        phi_29901_ = vec3<f32>((((_e1557 * _e5768.x) + (_e1553 * _e5768.y)) + (_e1509.x * _e5768.z)), (((_e1558 * _e5768.x) + (_e1563 * _e5768.y)) + (_e1509.y * _e5768.z)), (((_e1560 * _e5768.x) + (_e1564 * _e5768.y)) + (_e1509.z * _e5768.z)));
                                                    }
                                                    let _e5790 = phi_29901_;
                                                    phi_29904_ = _e5790;
                                                    phi_29905_ = _e5769;
                                                }
                                                let _e5792 = phi_29904_;
                                                let _e5794 = phi_29905_;
                                                phi_29675_ = select(_e5792, vec3<f32>(0f, 0f, 0f), vec3(_e5794));
                                            } else {
                                                phi_29675_ = vec3<f32>();
                                            }
                                            let _e5798 = phi_29675_;
                                            phi_29679_ = _e5798;
                                            phi_29680_ = select(true, false, _e5644);
                                        }
                                        let _e5801 = phi_29679_;
                                        let _e5803 = phi_29680_;
                                        if _e5803 {
                                            let _e5804 = (_e5570 * 0.00000037450704f);
                                            let _e5806 = ((_e5587 * 0.00000011920929f) - 1f);
                                            let _e5808 = (1f - (_e5806 * _e5806));
                                            if (_e5808 != _e5808) {
                                                phi_30092_ = true;
                                            } else {
                                                phi_30092_ = (0f >= _e5808);
                                            }
                                            let _e5812 = phi_30092_;
                                            let _e5814 = sqrt(select(_e5808, 0f, _e5812));
                                            let _e5819 = (_e1509.x + (_e5814 * cos(_e5804)));
                                            let _e5820 = (_e1509.y + (_e5814 * sin(_e5804)));
                                            let _e5821 = (_e1509.z + _e5806);
                                            let _e5827 = sqrt((((_e5819 * _e5819) + (_e5820 * _e5820)) + (_e5821 * _e5821)));
                                            if (_e5827 < 0.000001f) {
                                                phi_30066_ = _e1509;
                                            } else {
                                                let _e5829 = (1f / _e5827);
                                                phi_30066_ = vec3<f32>((_e5819 * _e5829), (_e5820 * _e5829), (_e5821 * _e5829));
                                            }
                                            let _e5835 = phi_30066_;
                                            phi_29687_ = _e5835;
                                        } else {
                                            phi_29687_ = _e5801;
                                        }
                                        let _e5837 = phi_29687_;
                                        let _e5845 = (((_e5837.x * _e1509.x) + (_e5837.y * _e1509.y)) + (_e5837.z * _e1509.z));
                                        if (_e5845 != _e5845) {
                                            phi_30107_ = true;
                                        } else {
                                            phi_30107_ = (0f >= _e5845);
                                        }
                                        let _e5849 = phi_30107_;
                                        let _e5851 = (select(_e5845, 0f, _e5849) * 0.31830987f);
                                        if _e5605 {
                                            if (_e5845 <= 0f) {
                                                phi_30904_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_30904_ = (_e2395 * (_e5845 * 0.31830987f));
                                            }
                                            let _e6326 = phi_30904_;
                                            phi_19317_ = type_52(_e6326, _e5851);
                                            phi_19318_ = type_52();
                                            phi_19319_ = true;
                                        } else {
                                            let _e5856 = (((_e1481 * _e1557) + (_e1482 * _e1558)) + (_e1483 * _e1560));
                                            let _e5861 = (((_e1481 * _e1553) + (_e1482 * _e1563)) + (_e1483 * _e1564));
                                            let _e5866 = (((_e1481 * _e1509.x) + (_e1482 * _e1509.y)) + (_e1483 * _e1509.z));
                                            let _e5871 = (((_e5837.x * _e1557) + (_e5837.y * _e1558)) + (_e5837.z * _e1560));
                                            let _e5876 = (((_e5837.x * _e1553) + (_e5837.y * _e1563)) + (_e5837.z * _e1564));
                                            if (_e5866 <= 0f) {
                                                phi_19281_ = type_52();
                                                phi_19282_ = true;
                                            } else {
                                                let _e5878 = (_e5845 <= 0f);
                                                if _e5878 {
                                                    phi_19280_ = type_52();
                                                } else {
                                                    let _e5879 = (_e5856 + _e5871);
                                                    let _e5880 = (_e5861 + _e5876);
                                                    let _e5881 = (_e5866 + _e5845);
                                                    let _e5888 = (1f / sqrt((((_e5879 * _e5879) + (_e5880 * _e5880)) + (_e5881 * _e5881))));
                                                    let _e5891 = (_e5881 * _e5888);
                                                    if (_e5891 <= 0f) {
                                                        phi_30217_ = 0f;
                                                    } else {
                                                        let _e5893 = (_e2399 * _e2399);
                                                        let _e5897 = ((((_e5893 - 1f) * _e5891) * _e5891) + 1f);
                                                        phi_30217_ = (_e5893 / ((3.1415927f * _e5897) * _e5897));
                                                    }
                                                    let _e5902 = phi_30217_;
                                                    let _e5903 = (_e5866 * _e5866);
                                                    if (_e5903 >= 1f) {
                                                        phi_30245_ = 0f;
                                                    } else {
                                                        phi_30245_ = ((sqrt((1f + (((_e2399 * _e2399) * ((_e5856 * _e5856) + (_e5861 * _e5861))) / _e5903))) - 1f) * 0.5f);
                                                    }
                                                    let _e5916 = phi_30245_;
                                                    let _e5917 = (1f + _e5916);
                                                    let _e5918 = (_e5845 * _e5845);
                                                    if (_e5918 >= 1f) {
                                                        phi_30273_ = 0f;
                                                    } else {
                                                        phi_30273_ = ((sqrt((1f + (((_e2399 * _e2399) * ((_e5871 * _e5871) + (_e5876 * _e5876))) / _e5918))) - 1f) * 0.5f);
                                                    }
                                                    let _e5931 = phi_30273_;
                                                    let _e5938 = (((_e5856 * (_e5879 * _e5888)) + (_e5861 * (_e5880 * _e5888))) + (_e5866 * _e5891));
                                                    if (_e2397 == 0u) {
                                                        let _e6233 = (1f - _e5938);
                                                        if (_e6233 != _e6233) {
                                                            phi_30776_ = true;
                                                        } else {
                                                            phi_30776_ = (0f >= _e6233);
                                                        }
                                                        let _e6237 = phi_30776_;
                                                        let _e6239 = pow(select(_e6233, 0f, _e6237), 5f);
                                                        let _e6240 = (1f - _e6239);
                                                        phi_19268_ = vec3<f32>(((_e2395.x * _e6240) + _e6239), ((_e2395.y * _e6240) + _e6239), ((_e2395.z * _e6240) + _e6239));
                                                    } else {
                                                        if (_e2397 == 1u) {
                                                            let _e6206 = (1f / _e2391);
                                                            let _e6210 = ((_e6206 * _e6206) * (1f - (_e5938 * _e5938)));
                                                            if (_e6210 >= 1f) {
                                                                phi_30737_ = 1f;
                                                            } else {
                                                                let _e6213 = sqrt((1f - _e6210));
                                                                if (_e5938 != _e5938) {
                                                                    phi_30748_ = true;
                                                                } else {
                                                                    phi_30748_ = (_e6213 <= _e5938);
                                                                }
                                                                let _e6217 = phi_30748_;
                                                                let _e6222 = pow(((1f - _e2391) / (1f + _e2391)), 2f);
                                                                phi_30737_ = (_e6222 + ((1f - _e6222) * pow((1f - select(_e5938, _e6213, _e6217)), 5f)));
                                                            }
                                                            let _e6229 = phi_30737_;
                                                            phi_19267_ = vec3(_e6229);
                                                        } else {
                                                            let _e5942 = select(_e5938, 0f, (_e5938 < 0f));
                                                            let _e5944 = select(_e5942, 1f, (_e5942 > 1f));
                                                            let _e5946 = (1f - (_e5944 * _e5944));
                                                            let _e5959 = ((_e2395.x * _e2395.x) - (_e2393.x * _e2393.x));
                                                            let _e5960 = ((_e2395.y * _e2395.y) - (_e2393.y * _e2393.y));
                                                            let _e5961 = ((_e2395.z * _e2395.z) - (_e2393.z * _e2393.z));
                                                            let _e5963 = (_e2395.x * (_e2393.x + _e2393.x));
                                                            let _e5965 = (_e2395.y * (_e2393.y + _e2393.y));
                                                            let _e5967 = (_e2395.z * (_e2393.z + _e2393.z));
                                                            let _e5974 = ((_e5959 * _e5959) + (_e5963 * _e5963));
                                                            let _e5975 = ((_e5960 * _e5960) + (_e5965 * _e5965));
                                                            let _e5976 = ((_e5961 * _e5961) + (_e5967 * _e5967));
                                                            let _e5981 = ((_e5946 * _e5959) / _e5974);
                                                            let _e5982 = ((_e5946 * _e5960) / _e5975);
                                                            let _e5983 = ((_e5946 * _e5961) / _e5976);
                                                            let _e5984 = (1f - _e5981);
                                                            let _e5985 = (1f - _e5982);
                                                            let _e5986 = (1f - _e5983);
                                                            let _e5988 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5981, _e5982, _e5983));
                                                            let _e5989 = (_e5946 * _e5963);
                                                            let _e5990 = (_e5946 * _e5965);
                                                            let _e5991 = (_e5946 * _e5967);
                                                            let _e5996 = (vec3<f32>(_e5989, _e5990, _e5991) / vec3<f32>(_e5974, _e5975, _e5976));
                                                            let _e6009 = ((_e5988.x * _e5988.x) + (_e5996.x * _e5996.x));
                                                            let _e6010 = ((_e5988.y * _e5988.y) + (_e5996.y * _e5996.y));
                                                            let _e6011 = ((_e5988.z * _e5988.z) + (_e5996.z * _e5996.z));
                                                            if (_e6009 != _e6009) {
                                                                phi_30407_ = true;
                                                            } else {
                                                                phi_30407_ = (0f >= _e6009);
                                                            }
                                                            let _e6015 = phi_30407_;
                                                            let _e6017 = sqrt(select(_e6009, 0f, _e6015));
                                                            if (_e6010 != _e6010) {
                                                                phi_30422_ = true;
                                                            } else {
                                                                phi_30422_ = (0f >= _e6010);
                                                            }
                                                            let _e6021 = phi_30422_;
                                                            let _e6023 = sqrt(select(_e6010, 0f, _e6021));
                                                            if (_e6011 != _e6011) {
                                                                phi_30437_ = true;
                                                            } else {
                                                                phi_30437_ = (0f >= _e6011);
                                                            }
                                                            let _e6027 = phi_30437_;
                                                            let _e6029 = sqrt(select(_e6011, 0f, _e6027));
                                                            let _e6033 = ((_e6017 + _e5984) * 0.5f);
                                                            let _e6034 = ((_e6023 + _e5985) * 0.5f);
                                                            let _e6035 = ((_e6029 + _e5986) * 0.5f);
                                                            if (_e6033 != _e6033) {
                                                                phi_30464_ = true;
                                                            } else {
                                                                phi_30464_ = (0f >= _e6033);
                                                            }
                                                            let _e6039 = phi_30464_;
                                                            let _e6041 = sqrt(select(_e6033, 0f, _e6039));
                                                            if (_e6034 != _e6034) {
                                                                phi_30479_ = true;
                                                            } else {
                                                                phi_30479_ = (0f >= _e6034);
                                                            }
                                                            let _e6045 = phi_30479_;
                                                            let _e6047 = sqrt(select(_e6034, 0f, _e6045));
                                                            if (_e6035 != _e6035) {
                                                                phi_30494_ = true;
                                                            } else {
                                                                phi_30494_ = (0f >= _e6035);
                                                            }
                                                            let _e6051 = phi_30494_;
                                                            let _e6053 = sqrt(select(_e6035, 0f, _e6051));
                                                            let _e6054 = vec3<f32>(_e6041, _e6047, _e6053);
                                                            let _e6058 = ((_e6017 - _e5984) * 0.5f);
                                                            let _e6059 = ((_e6023 - _e5985) * 0.5f);
                                                            let _e6060 = ((_e6029 - _e5986) * 0.5f);
                                                            if (_e6058 != _e6058) {
                                                                phi_30521_ = true;
                                                            } else {
                                                                phi_30521_ = (0f >= _e6058);
                                                            }
                                                            let _e6064 = phi_30521_;
                                                            let _e6066 = sqrt(select(_e6058, 0f, _e6064));
                                                            if (_e6059 != _e6059) {
                                                                phi_30536_ = true;
                                                            } else {
                                                                phi_30536_ = (0f >= _e6059);
                                                            }
                                                            let _e6070 = phi_30536_;
                                                            let _e6072 = sqrt(select(_e6059, 0f, _e6070));
                                                            if (_e6060 != _e6060) {
                                                                phi_30551_ = true;
                                                            } else {
                                                                phi_30551_ = (0f >= _e6060);
                                                            }
                                                            let _e6076 = phi_30551_;
                                                            let _e6078 = sqrt(select(_e6060, 0f, _e6076));
                                                            if ((_e5989 / _e5974) < 0f) {
                                                                phi_19144_ = -(_e6066);
                                                            } else {
                                                                phi_19144_ = _e6066;
                                                            }
                                                            let _e6082 = phi_19144_;
                                                            if ((_e5990 / _e5975) < 0f) {
                                                                phi_19152_ = -(_e6072);
                                                            } else {
                                                                phi_19152_ = _e6072;
                                                            }
                                                            let _e6086 = phi_19152_;
                                                            if ((_e5991 / _e5976) < 0f) {
                                                                phi_19160_ = -(_e6078);
                                                            } else {
                                                                phi_19160_ = _e6078;
                                                            }
                                                            let _e6090 = phi_19160_;
                                                            let _e6094 = (_e2393.x * _e5944);
                                                            let _e6095 = (_e2393.y * _e5944);
                                                            let _e6096 = (_e2393.z * _e5944);
                                                            let _e6106 = vec3<f32>(((_e2395.x * _e6041) - (_e2393.x * _e6082)), ((_e2395.y * _e6047) - (_e2393.y * _e6086)), ((_e2395.z * _e6053) - (_e2393.z * _e6090)));
                                                            let _e6113 = ((_e2395.x * _e6082) + (_e2393.x * _e6041));
                                                            let _e6114 = ((_e2395.y * _e6086) + (_e2393.y * _e6047));
                                                            let _e6115 = ((_e2395.z * _e6090) + (_e2393.z * _e6053));
                                                            let _e6117 = vec3<f32>((_e2395.x * _e5944), (_e2395.y * _e5944), (_e2395.z * _e5944));
                                                            let _e6118 = (_e6117 - _e6054);
                                                            let _e6119 = (_e6094 - _e6082);
                                                            let _e6120 = (_e6095 - _e6086);
                                                            let _e6121 = (_e6096 - _e6090);
                                                            let _e6134 = (_e6117 + _e6054);
                                                            let _e6135 = (_e6094 + _e6082);
                                                            let _e6136 = (_e6095 + _e6086);
                                                            let _e6137 = (_e6096 + _e6090);
                                                            let _e6147 = ((_e6134.x * _e6134.x) + (_e6135 * _e6135));
                                                            let _e6148 = ((_e6134.y * _e6134.y) + (_e6136 * _e6136));
                                                            let _e6149 = ((_e6134.z * _e6134.z) + (_e6137 * _e6137));
                                                            let _e6159 = vec3(_e5944);
                                                            let _e6160 = (_e6159 - _e6106);
                                                            let _e6161 = -(vec3<f32>(_e6113, _e6114, _e6115));
                                                            let _e6177 = (_e6159 + _e6106);
                                                            let _e6187 = ((_e6177.x * _e6177.x) + (_e6113 * _e6113));
                                                            let _e6188 = ((_e6177.y * _e6177.y) + (_e6114 * _e6114));
                                                            let _e6189 = ((_e6177.z * _e6177.z) + (_e6115 * _e6115));
                                                            phi_19267_ = vec3<f32>((((((_e6118.x * _e6118.x) + (_e6119 * _e6119)) / select(0.000000000000000000000000000001f, _e6147, (_e6147 > 0.000000000000000000000000000001f))) + (((_e6160.x * _e6160.x) + (_e6161.x * _e6161.x)) / select(0.000000000000000000000000000001f, _e6187, (_e6187 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6118.y * _e6118.y) + (_e6120 * _e6120)) / select(0.000000000000000000000000000001f, _e6148, (_e6148 > 0.000000000000000000000000000001f))) + (((_e6160.y * _e6160.y) + (_e6161.y * _e6161.y)) / select(0.000000000000000000000000000001f, _e6188, (_e6188 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6118.z * _e6118.z) + (_e6121 * _e6121)) / select(0.000000000000000000000000000001f, _e6149, (_e6149 > 0.000000000000000000000000000001f))) + (((_e6160.z * _e6160.z) + (_e6161.z * _e6161.z)) / select(0.000000000000000000000000000001f, _e6189, (_e6189 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e6232 = phi_19267_;
                                                        phi_19268_ = _e6232;
                                                    }
                                                    let _e6252 = phi_19268_;
                                                    let _e6254 = (4f * _e5866);
                                                    phi_19280_ = type_52((_e6252 * (((1f / (_e5917 + _e5931)) * _e5902) / _e6254)), ((_e5902 / _e5917) / _e6254));
                                                }
                                                let _e6261 = phi_19280_;
                                                phi_19281_ = _e6261;
                                                phi_19282_ = _e5878;
                                            }
                                            let _e6263 = phi_19281_;
                                            let _e6265 = phi_19282_;
                                            if _e6265 {
                                                phi_19287_ = type_52(vec3<f32>(0f, 0f, 0f), 0f);
                                            } else {
                                                phi_19287_ = _e6263;
                                            }
                                            let _e6267 = phi_19287_;
                                            let _e6270 = (_e2401 == 1u);
                                            if _e6270 {
                                                phi_19315_ = _e6267;
                                                phi_19316_ = type_52();
                                            } else {
                                                if (_e5866 != _e5866) {
                                                    phi_30813_ = true;
                                                } else {
                                                    phi_30813_ = (0f >= _e5866);
                                                }
                                                let _e6274 = phi_30813_;
                                                let _e6275 = select(_e5866, 0f, _e6274);
                                                let _e6276 = (1f / _e2391);
                                                let _e6280 = ((_e6276 * _e6276) * (1f - (_e6275 * _e6275)));
                                                if (_e6280 >= 1f) {
                                                    phi_30832_ = 1f;
                                                } else {
                                                    let _e6283 = sqrt((1f - _e6280));
                                                    if (_e6275 != _e6275) {
                                                        phi_30843_ = true;
                                                    } else {
                                                        phi_30843_ = (_e6283 <= _e6275);
                                                    }
                                                    let _e6287 = phi_30843_;
                                                    let _e6292 = pow(((1f - _e2391) / (1f + _e2391)), 2f);
                                                    phi_30832_ = (_e6292 + ((1f - _e6292) * pow((1f - select(_e6275, _e6283, _e6287)), 5f)));
                                                }
                                                let _e6299 = phi_30832_;
                                                if (_e5845 <= 0f) {
                                                    phi_30882_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_30882_ = (_e2389 * (_e5845 * 0.31830987f));
                                                }
                                                let _e6304 = phi_30882_;
                                                let _e6305 = (1f - _e6299);
                                                phi_19315_ = type_52();
                                                phi_19316_ = type_52((_e6267.member + vec3<f32>((_e6304.x * _e6305), (_e6304.y * _e6305), (_e6304.z * _e6305))), ((_e6299 * _e6267.member_1) + (_e6305 * _e5851)));
                                            }
                                            let _e6319 = phi_19315_;
                                            let _e6321 = phi_19316_;
                                            phi_19317_ = _e6319;
                                            phi_19318_ = _e6321;
                                            phi_19319_ = _e6270;
                                        }
                                        let _e6329 = phi_19317_;
                                        let _e6331 = phi_19318_;
                                        let _e6333 = phi_19319_;
                                        if _e6333 {
                                            phi_19323_ = _e6329;
                                        } else {
                                            phi_19323_ = _e6331;
                                        }
                                        let _e6335 = phi_19323_;
                                        if ((((_e5837.x * _e5837.x) + (_e5837.y * _e5837.y)) + (_e5837.z * _e5837.z)) == 0f) {
                                            phi_36302_ = _e5600;
                                            phi_19462_ = type_64();
                                            phi_19463_ = type_64();
                                            phi_19464_ = true;
                                            phi_19465_ = false;
                                        } else {
                                            let _e6344 = (_e6335.member_1 <= 0f);
                                            if _e6344 {
                                                phi_36303_ = _e5600;
                                                phi_19459_ = type_64();
                                                phi_19460_ = type_64();
                                                phi_19461_ = false;
                                            } else {
                                                let _e6346 = (_e6335.member.x / _e6335.member_1);
                                                let _e6348 = (_e6335.member.y / _e6335.member_1);
                                                let _e6350 = (_e6335.member.z / _e6335.member_1);
                                                let _e6359 = global_1.member.member_4[1u];
                                                if (sqrt((((_e6346 * _e6346) + (_e6348 * _e6348)) + (_e6350 * _e6350))) <= _e6359) {
                                                    phi_36296_ = _e5600;
                                                    phi_19410_ = _e1404;
                                                    phi_19411_ = false;
                                                } else {
                                                    let _e6363 = global_1.member.member_7[1u];
                                                    if (_e6363 == 1u) {
                                                        let _e6368 = global_1.member.member_4[3u];
                                                        if (f32(_e345) >= _e6368) {
                                                            let _e6371 = select(_e6348, _e6346, (_e6346 > _e6348));
                                                            let _e6373 = select(_e6350, _e6371, (_e6371 > _e6350));
                                                            if (_e6373 != _e6373) {
                                                                phi_30929_ = true;
                                                            } else {
                                                                phi_30929_ = (1f <= _e6373);
                                                            }
                                                            let _e6377 = phi_30929_;
                                                            let _e6378 = select(_e6373, 1f, _e6377);
                                                            let _e6380 = ((_e5600 * 747796405u) + 2891336453u);
                                                            let _e6388 = (((_e6380 >> bitcast<u32>((((_e6380 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6380) * 277803737u);
                                                            let _e6391 = ((_e6388 >> bitcast<u32>(22i)) ^ _e6388);
                                                            let _e6396 = ((f32((_e6391 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6378);
                                                            if _e6396 {
                                                                phi_19399_ = vec3<f32>();
                                                            } else {
                                                                phi_19399_ = vec3<f32>((_e1404.x / _e6378), (_e1404.y / _e6378), (_e1404.z / _e6378));
                                                            }
                                                            let _e6405 = phi_19399_;
                                                            phi_36299_ = _e6391;
                                                            phi_19401_ = _e6405;
                                                            phi_19402_ = select(true, false, _e6396);
                                                        } else {
                                                            phi_36299_ = _e5600;
                                                            phi_19401_ = _e1404;
                                                            phi_19402_ = true;
                                                        }
                                                        let _e6408 = phi_36299_;
                                                        let _e6410 = phi_19401_;
                                                        let _e6412 = phi_19402_;
                                                        phi_36298_ = _e6408;
                                                        phi_19403_ = _e6410;
                                                        phi_19404_ = _e6412;
                                                    } else {
                                                        phi_36298_ = _e5600;
                                                        phi_19403_ = _e1404;
                                                        phi_19404_ = true;
                                                    }
                                                    let _e6414 = phi_36298_;
                                                    let _e6416 = phi_19403_;
                                                    let _e6418 = phi_19404_;
                                                    phi_36296_ = _e6414;
                                                    phi_19410_ = select(_e1404, _e6416, vec3(_e6418));
                                                    phi_19411_ = select(false, true, _e6418);
                                                }
                                                let _e6423 = phi_36296_;
                                                let _e6425 = phi_19410_;
                                                let _e6427 = phi_19411_;
                                                if _e6427 {
                                                    if ((((_e5837.x * _e1496.x) + (_e5837.y * _e1496.y)) + (_e5837.z * _e1496.z)) < 0f) {
                                                        phi_19440_ = -(_e1496);
                                                    } else {
                                                        phi_19440_ = _e1496;
                                                    }
                                                    let _e6466 = phi_19440_;
                                                    let _e6483 = local_90;
                                                    let _e6486 = local_91;
                                                    let _e6489 = local_92;
                                                    let _e6492 = local_93;
                                                    let _e6498 = local_94;
                                                    phi_19456_ = type_64(type_30(vec4<f32>((_e1411 + (_e6466.x * _e1527)), (_e1412 + (_e6466.y * _e1527)), (_e1413 + (_e6466.z * _e1527)), _e318.member.w), vec4<f32>(_e5837.x, _e5837.y, _e5837.z, _e6335.member_1), vec4<f32>((_e6425.x * _e6346), (_e6425.y * _e6348), (_e6425.z * _e6350), f32((_e345 + 1u))), vec4<f32>(_e6486.x, _e6489.y, _e6492.z, f32(_e6483)), vec4<u32>(_e6423, _e2409, 0u, 0u)), _e1406, _e6498);
                                                    phi_19457_ = type_64();
                                                } else {
                                                    let _e6435 = local_85;
                                                    let _e6438 = local_86;
                                                    let _e6441 = local_87;
                                                    let _e6444 = local_88;
                                                    let _e6450 = local_89;
                                                    phi_19456_ = type_64();
                                                    phi_19457_ = type_64(type_30(_e318.member, _e318.member_1, vec4<f32>(_e6425.x, _e6425.y, _e6425.z, f32(_e345)), vec4<f32>(_e6438.x, _e6441.y, _e6444.z, f32(_e6435)), vec4<u32>(_e6423, (_e2409 & 4294967294u), 0u, 0u)), _e1406, _e6450);
                                                }
                                                let _e6501 = phi_19456_;
                                                let _e6503 = phi_19457_;
                                                phi_36303_ = _e6423;
                                                phi_19459_ = _e6501;
                                                phi_19460_ = _e6503;
                                                phi_19461_ = select(true, false, _e6427);
                                            }
                                            let _e6506 = phi_36303_;
                                            let _e6508 = phi_19459_;
                                            let _e6510 = phi_19460_;
                                            let _e6512 = phi_19461_;
                                            phi_36302_ = _e6506;
                                            phi_19462_ = _e6508;
                                            phi_19463_ = _e6510;
                                            phi_19464_ = _e6344;
                                            phi_19465_ = _e6512;
                                        }
                                        let _e6514 = phi_36302_;
                                        let _e6516 = phi_19462_;
                                        let _e6518 = phi_19463_;
                                        let _e6520 = phi_19464_;
                                        let _e6522 = phi_19465_;
                                        if _e6520 {
                                            let _e6530 = local_95;
                                            let _e6533 = local_96;
                                            let _e6536 = local_97;
                                            let _e6539 = local_98;
                                            let _e6545 = local_99;
                                            phi_19471_ = type_64(type_30(_e318.member, _e318.member_1, vec4<f32>(_e1404.x, _e1404.y, _e1404.z, f32(_e345)), vec4<f32>(_e6533.x, _e6536.y, _e6539.z, f32(_e6530)), vec4<u32>(_e6514, (_e2409 & 4294967294u), 0u, 0u)), _e1406, _e6545);
                                        } else {
                                            phi_19471_ = _e6518;
                                        }
                                        let _e6548 = phi_19471_;
                                        phi_19476_ = _e6548;
                                        phi_19477_ = _e6516;
                                        phi_19478_ = select(_e6522, true, _e6520);
                                    }
                                    let _e6575 = phi_19476_;
                                    let _e6577 = phi_19477_;
                                    let _e6579 = phi_19478_;
                                    phi_19482_ = _e6575;
                                    phi_19483_ = _e6577;
                                    phi_19484_ = _e6579;
                                } else {
                                    phi_19482_ = _e2405;
                                    phi_19483_ = type_64();
                                    phi_19484_ = true;
                                }
                                let _e6581 = phi_19482_;
                                let _e6583 = phi_19483_;
                                let _e6585 = phi_19484_;
                                phi_19488_ = _e6581;
                                phi_19489_ = _e6583;
                                phi_19490_ = _e6585;
                            }
                            let _e6797 = phi_19488_;
                            let _e6799 = phi_19489_;
                            let _e6801 = phi_19490_;
                            phi_19494_ = _e6797;
                            phi_19495_ = _e6799;
                            phi_19496_ = _e6801;
                        }
                        let _e7309 = phi_19494_;
                        let _e7311 = phi_19495_;
                        let _e7313 = phi_19496_;
                        if _e7313 {
                            phi_19500_ = _e7309;
                        } else {
                            phi_19500_ = _e7311;
                        }
                        let _e7315 = phi_19500_;
                        if ((((_e7315.member_1.member_2.x * _e7315.member_1.member_2.x) + (_e7315.member_1.member_2.y * _e7315.member_1.member_2.y)) + (_e7315.member_1.member_2.z * _e7315.member_1.member_2.z)) > 0f) {
                            phi_19538_ = true;
                        } else {
                            phi_19538_ = ((((_e7315.member_2.member_2.x * _e7315.member_2.member_2.x) + (_e7315.member_2.member_2.y * _e7315.member_2.member_2.y)) + (_e7315.member_2.member_2.z * _e7315.member_2.member_2.z)) > 0f);
                        }
                        let _e7340 = phi_19538_;
                        if _e7340 {
                            phi_19545_ = type_30(_e7315.member.member, _e7315.member.member_1, _e7315.member.member_2, _e7315.member.member_3, vec4<u32>(_e7315.member.member_4.x, (_e7315.member.member_4.y | 16u), _e7315.member.member_4.z, _e7315.member.member_4.w));
                        } else {
                            phi_19545_ = _e7315.member;
                        }
                        let _e7356 = phi_19545_;
                        phi_19546_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_19547_ = _e7315.member_2;
                        phi_19548_ = _e7315.member_1;
                        phi_19549_ = _e7356;
                    }
                    let _e7403 = phi_19546_;
                    let _e7405 = phi_19547_;
                    let _e7407 = phi_19548_;
                    let _e7409 = phi_19549_;
                    if _e308 {
                    } else {
                        break;
                    }
                    global_6.member[_e303.member_1] = _e7409;
                    if ((_e7409.member_4.y & 16u) == 0u) {
                    } else {
                        let _e7414 = (2u * _e303.member_1);
                        if (_e7414 < _e283) {
                        } else {
                            break;
                        }
                        global_7.member[_e7414] = _e7407;
                        let _e7418 = (_e7414 + 1u);
                        if (_e7418 < _e283) {
                        } else {
                            break;
                        }
                        global_7.member[_e7418] = _e7405;
                    }
                    if (_e7403.w > 0f) {
                        if (_e303.member_1 < arrayLength((&global_8.member))) {
                        } else {
                            break;
                        }
                        let _e7429 = global_8.member[_e303.member_1][0u];
                        global_8.member[_e303.member_1][0u] = (_e7429 + _e7403.x);
                        let _e7435 = global_8.member[_e303.member_1][1u];
                        global_8.member[_e303.member_1][1u] = (_e7435 + _e7403.y);
                        let _e7441 = global_8.member[_e303.member_1][2u];
                        global_8.member[_e303.member_1][2u] = (_e7441 + _e7403.z);
                        let _e7446 = global_8.member[_e303.member_1][3u];
                        global_8.member[_e303.member_1][3u] = (_e7446 + _e7403.w);
                    }
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
