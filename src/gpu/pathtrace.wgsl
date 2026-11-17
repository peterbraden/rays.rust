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

struct type_51 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_53 {
    member: u32,
    member_1: type_51,
}

struct type_54 {
    member: f32,
    member_1: f32,
}

struct type_55 {
    member: u32,
    member_1: type_54,
}

struct type_56 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_57 {
    member: vec3<f32>,
    member_1: f32,
}

struct type_58 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_62 {
    member: array<vec4<f32>>,
}

struct type_64 {
    member: f32,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
}

struct type_69 {
    member: type_35,
    member_1: type_39,
    member_2: type_39,
}

struct type_72 {
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
var<storage, read_write> global_9: type_62;

fn function_() {
    var phi_447_: type_45;
    var phi_31572_: bool;
    var phi_481_: vec3<f32>;
    var phi_484_: u32;
    var phi_20479_: f32;
    var phi_20491_: type_54;
    var phi_20502_: type_55;
    var phi_20503_: type_55;
    var phi_20507_: type_55;
    var phi_20508_: type_55;
    var phi_20509_: bool;
    var phi_20513_: type_55;
    var phi_20400_: type_55;
    var phi_20551_: bool;
    var phi_20631_: f32;
    var phi_20643_: type_54;
    var phi_20654_: type_55;
    var phi_20655_: type_55;
    var phi_20659_: type_55;
    var phi_20660_: type_55;
    var phi_20661_: bool;
    var phi_20665_: type_55;
    var phi_20599_: f32;
    var phi_20703_: bool;
    var phi_20428_: f32;
    var phi_20430_: f32;
    var phi_20439_: type_55;
    var phi_20443_: type_55;
    var phi_20447_: type_55;
    var phi_20718_: bool;
    var phi_9484_: vec3<f32>;
    var phi_9487_: vec3<f32>;
    var phi_9489_: vec3<f32>;
    var phi_9491_: vec3<f32>;
    var phi_9493_: u32;
    var phi_9577_: u32;
    var phi_9592_: type_54;
    var phi_9593_: type_54;
    var phi_9594_: type_54;
    var phi_20918_: bool;
    var phi_21045_: bool;
    var phi_21060_: bool;
    var phi_9922_: vec3<f32>;
    var phi_9995_: vec3<f32>;
    var phi_9996_: vec3<f32>;
    var phi_9997_: bool;
    var phi_10002_: vec3<f32>;
    var phi_10003_: bool;
    var phi_21184_: bool;
    var phi_21199_: bool;
    var phi_10294_: vec3<f32>;
    var phi_9485_: vec3<f32>;
    var phi_9488_: vec3<f32>;
    var phi_9490_: vec3<f32>;
    var phi_9492_: vec3<f32>;
    var phi_9494_: u32;
    var phi_31555_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_32039_: bool;
    var phi_10351_: type_58;
    var phi_32038_: bool;
    var phi_10353_: type_58;
    var phi_10354_: bool;
    var phi_10359_: type_58;
    var phi_860_: vec3<f32>;
    var phi_862_: vec3<f32>;
    var phi_863_: bool;
    var phi_31647_: bool;
    var phi_873_: vec3<f32>;
    var phi_876_: i32;
    var phi_878_: u32;
    var phi_21468_: bool;
    var phi_1300_: u32;
    var phi_1330_: u32;
    var phi_1301_: u32;
    var phi_1331_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1349_: u32;
    var phi_2654_: type_53;
    var phi_2662_: type_51;
    var phi_2664_: type_51;
    var phi_2665_: bool;
    var phi_2666_: type_51;
    var phi_2667_: bool;
    var phi_2668_: bool;
    var phi_21579_: f32;
    var phi_21591_: type_54;
    var phi_21602_: type_55;
    var phi_21603_: type_55;
    var phi_21607_: type_55;
    var phi_21608_: type_55;
    var phi_21609_: bool;
    var phi_21613_: type_55;
    var phi_21500_: type_55;
    var phi_21651_: bool;
    var phi_21731_: f32;
    var phi_21743_: type_54;
    var phi_21754_: type_55;
    var phi_21755_: type_55;
    var phi_21759_: type_55;
    var phi_21760_: type_55;
    var phi_21761_: bool;
    var phi_21765_: type_55;
    var phi_21699_: f32;
    var phi_21803_: bool;
    var phi_21528_: f32;
    var phi_21530_: f32;
    var phi_21539_: type_55;
    var phi_21543_: type_55;
    var phi_21547_: type_55;
    var phi_2686_: vec3<f32>;
    var phi_2689_: vec3<f32>;
    var phi_2691_: vec3<f32>;
    var phi_2693_: vec3<f32>;
    var phi_2695_: u32;
    var phi_2779_: u32;
    var phi_2794_: type_54;
    var phi_2795_: type_54;
    var phi_2796_: type_54;
    var phi_22002_: bool;
    var phi_22129_: bool;
    var phi_22144_: bool;
    var phi_3124_: vec3<f32>;
    var phi_22266_: f32;
    var phi_22278_: type_54;
    var phi_22289_: type_55;
    var phi_22290_: type_55;
    var phi_22294_: type_55;
    var phi_22295_: type_55;
    var phi_22296_: bool;
    var phi_22300_: type_55;
    var phi_22234_: f32;
    var phi_22376_: f32;
    var phi_22388_: type_54;
    var phi_22399_: type_55;
    var phi_22400_: type_55;
    var phi_22404_: type_55;
    var phi_22405_: type_55;
    var phi_22406_: bool;
    var phi_22410_: type_55;
    var phi_22344_: f32;
    var phi_22171_: vec3<f32>;
    var phi_22172_: u32;
    var phi_22542_: bool;
    var phi_22205_: vec3<f32>;
    var phi_22206_: u32;
    var local_11: vec3<f32>;
    var phi_22213_: type_56;
    var phi_22217_: type_56;
    var phi_3183_: vec3<f32>;
    var phi_3184_: vec3<f32>;
    var phi_2687_: vec3<f32>;
    var phi_2690_: vec3<f32>;
    var phi_2692_: vec3<f32>;
    var phi_2694_: vec3<f32>;
    var phi_2696_: u32;
    var phi_31615_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_31709_: bool;
    var phi_3204_: type_51;
    var phi_31708_: bool;
    var phi_3205_: type_51;
    var phi_31707_: bool;
    var phi_3228_: vec3<f32>;
    var phi_3229_: i32;
    var phi_3230_: u32;
    var phi_31706_: bool;
    var phi_874_: vec3<f32>;
    var phi_877_: i32;
    var phi_879_: u32;
    var phi_3231_: bool;
    var phi_31705_: bool;
    var phi_3565_: vec3<f32>;
    var phi_3568_: u32;
    var phi_22670_: f32;
    var phi_22685_: f32;
    var phi_22700_: f32;
    var phi_3639_: i32;
    var phi_3669_: i32;
    var phi_3677_: vec3<f32>;
    var phi_3680_: i32;
    var phi_3703_: vec3<f32>;
    var phi_3706_: i32;
    var phi_3704_: vec3<f32>;
    var phi_3707_: i32;
    var phi_3678_: vec3<f32>;
    var phi_3681_: i32;
    var phi_3905_: vec3<f32>;
    var phi_3906_: vec3<f32>;
    var phi_3907_: vec3<f32>;
    var phi_3566_: vec3<f32>;
    var phi_3569_: u32;
    var phi_31710_: bool;
    var local_15: vec3<f32>;
    var phi_31758_: bool;
    var phi_3918_: f32;
    var phi_3942_: f32;
    var phi_3944_: u32;
    var phi_3947_: f32;
    var phi_4050_: f32;
    var phi_4051_: f32;
    var phi_4067_: f32;
    var phi_3945_: u32;
    var phi_3948_: f32;
    var phi_4070_: u32;
    var phi_4073_: f32;
    var phi_4181_: f32;
    var phi_4071_: u32;
    var phi_4074_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_22816_: f32;
    var phi_22828_: type_54;
    var phi_22839_: type_55;
    var phi_22840_: type_55;
    var phi_22844_: type_55;
    var phi_22845_: type_55;
    var phi_22846_: bool;
    var phi_22850_: type_55;
    var phi_22784_: f32;
    var phi_4582_: bool;
    var phi_22976_: f32;
    var phi_22988_: type_54;
    var phi_22999_: type_55;
    var phi_23000_: type_55;
    var phi_23004_: type_55;
    var phi_23005_: type_55;
    var phi_23006_: bool;
    var phi_23010_: type_55;
    var phi_22897_: type_55;
    var phi_23048_: bool;
    var phi_23128_: f32;
    var phi_23140_: type_54;
    var phi_23151_: type_55;
    var phi_23152_: type_55;
    var phi_23156_: type_55;
    var phi_23157_: type_55;
    var phi_23158_: bool;
    var phi_23162_: type_55;
    var phi_23096_: f32;
    var phi_23200_: bool;
    var phi_22925_: f32;
    var phi_22927_: f32;
    var phi_22936_: type_55;
    var phi_22940_: type_55;
    var phi_22944_: type_55;
    var phi_4596_: vec3<f32>;
    var phi_4599_: u32;
    var phi_4683_: u32;
    var phi_4698_: type_54;
    var phi_4699_: type_54;
    var phi_4700_: type_54;
    var phi_23399_: bool;
    var phi_4597_: vec3<f32>;
    var phi_4600_: u32;
    var phi_31737_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_31804_: bool;
    var phi_4725_: vec3<f32>;
    var phi_31803_: bool;
    var phi_4727_: vec3<f32>;
    var phi_4729_: bool;
    var phi_31799_: bool;
    var phi_4740_: vec3<f32>;
    var phi_4741_: bool;
    var phi_31798_: bool;
    var phi_4743_: vec3<f32>;
    var phi_4744_: bool;
    var phi_4745_: bool;
    var phi_31797_: bool;
    var phi_4749_: vec3<f32>;
    var phi_4750_: vec3<f32>;
    var phi_4751_: bool;
    var phi_4752_: vec3<f32>;
    var phi_4753_: bool;
    var phi_5073_: vec3<f32>;
    var phi_5076_: u32;
    var phi_23470_: f32;
    var phi_23485_: f32;
    var phi_23500_: f32;
    var phi_5147_: i32;
    var phi_5177_: i32;
    var phi_5185_: vec3<f32>;
    var phi_5188_: i32;
    var phi_5211_: vec3<f32>;
    var phi_5214_: i32;
    var phi_5285_: vec3<f32>;
    var phi_5288_: u32;
    var phi_5408_: vec3<f32>;
    var phi_5286_: vec3<f32>;
    var phi_5289_: u32;
    var phi_5212_: vec3<f32>;
    var phi_5215_: i32;
    var phi_5186_: vec3<f32>;
    var phi_5189_: i32;
    var phi_5413_: vec3<f32>;
    var phi_5414_: vec3<f32>;
    var phi_5415_: vec3<f32>;
    var phi_5074_: vec3<f32>;
    var phi_5077_: u32;
    var phi_31792_: bool;
    var phi_31834_: bool;
    var phi_5435_: vec3<f32>;
    var phi_5648_: u32;
    var phi_5649_: bool;
    var phi_5652_: u32;
    var phi_5653_: bool;
    var phi_5654_: bool;
    var phi_5664_: f32;
    var phi_5667_: f32;
    var phi_5669_: u32;
    var phi_5671_: vec3<f32>;
    var phi_5673_: vec3<f32>;
    var phi_5675_: f32;
    var phi_5677_: f32;
    var phi_5715_: u32;
    var phi_5730_: vec3<f32>;
    var phi_5750_: vec3<f32>;
    var phi_23515_: vec3<f32>;
    var phi_5751_: u32;
    var phi_5752_: vec3<f32>;
    var phi_5753_: u32;
    var phi_5754_: vec3<f32>;
    var phi_5755_: u32;
    var phi_5757_: vec3<f32>;
    var phi_5892_: u32;
    var phi_5895_: f32;
    var phi_5897_: f32;
    var phi_5899_: f32;
    var phi_5901_: vec3<f32>;
    var phi_23570_: f32;
    var phi_23576_: f32;
    var phi_23604_: f32;
    var phi_23610_: f32;
    var phi_23638_: f32;
    var phi_23644_: f32;
    var phi_23672_: f32;
    var phi_23678_: f32;
    var phi_23706_: f32;
    var phi_23712_: f32;
    var phi_23740_: f32;
    var phi_23746_: f32;
    var phi_23774_: f32;
    var phi_23780_: f32;
    var phi_23808_: f32;
    var phi_23814_: f32;
    var phi_5893_: u32;
    var phi_5896_: f32;
    var phi_5898_: f32;
    var phi_5900_: f32;
    var phi_5902_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6318_: f32;
    var phi_6370_: vec3<f32>;
    var phi_6371_: f32;
    var phi_6372_: vec3<f32>;
    var phi_6373_: f32;
    var phi_6374_: vec3<f32>;
    var phi_6375_: f32;
    var phi_6377_: vec3<f32>;
    var phi_6378_: f32;
    var phi_6379_: f32;
    var phi_6380_: vec3<f32>;
    var phi_6381_: vec3<f32>;
    var phi_6382_: f32;
    var phi_6383_: f32;
    var phi_6384_: vec3<f32>;
    var phi_6385_: vec3<f32>;
    var phi_6386_: f32;
    var phi_6387_: f32;
    var phi_6388_: vec3<f32>;
    var phi_6389_: vec3<f32>;
    var phi_6390_: f32;
    var phi_6391_: f32;
    var phi_6392_: vec3<f32>;
    var phi_6393_: vec3<f32>;
    var phi_6395_: f32;
    var phi_6396_: f32;
    var phi_6397_: vec3<f32>;
    var phi_6398_: vec3<f32>;
    var phi_6399_: f32;
    var phi_6400_: f32;
    var phi_6401_: f32;
    var phi_6402_: f32;
    var phi_6403_: vec3<f32>;
    var phi_6404_: vec3<f32>;
    var phi_6405_: f32;
    var phi_6406_: f32;
    var phi_5665_: f32;
    var phi_5668_: f32;
    var phi_5670_: u32;
    var phi_5672_: vec3<f32>;
    var phi_5674_: vec3<f32>;
    var phi_5676_: f32;
    var phi_5678_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_23890_: bool;
    var local_29: f32;
    var phi_6438_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6517_: f32;
    var phi_6518_: vec3<f32>;
    var phi_6519_: f32;
    var phi_6521_: vec3<f32>;
    var phi_6524_: u32;
    var phi_6972_: f32;
    var phi_7152_: u32;
    var phi_7155_: f32;
    var phi_7157_: f32;
    var phi_7159_: f32;
    var phi_7161_: vec3<f32>;
    var phi_23979_: f32;
    var phi_23985_: f32;
    var phi_24013_: f32;
    var phi_24019_: f32;
    var phi_24047_: f32;
    var phi_24053_: f32;
    var phi_24081_: f32;
    var phi_24087_: f32;
    var phi_24115_: f32;
    var phi_24121_: f32;
    var phi_24149_: f32;
    var phi_24155_: f32;
    var phi_24183_: f32;
    var phi_24189_: f32;
    var phi_24217_: f32;
    var phi_24223_: f32;
    var phi_7153_: u32;
    var phi_7156_: f32;
    var phi_7158_: f32;
    var phi_7160_: f32;
    var phi_7162_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7575_: f32;
    var phi_24269_: bool;
    var phi_24284_: bool;
    var phi_7668_: vec3<f32>;
    var phi_7669_: vec3<f32>;
    var phi_7670_: vec3<f32>;
    var phi_7671_: vec3<f32>;
    var phi_6522_: vec3<f32>;
    var phi_6525_: u32;
    var phi_7690_: vec3<f32>;
    var phi_7693_: u32;
    var phi_7695_: f32;
    var phi_7996_: vec3<f32>;
    var phi_7997_: u32;
    var phi_7998_: f32;
    var phi_7999_: vec3<f32>;
    var phi_8016_: vec3<f32>;
    var phi_8018_: u32;
    var phi_8021_: vec3<f32>;
    var phi_8023_: f32;
    var phi_8025_: f32;
    var phi_8027_: f32;
    var phi_8047_: bool;
    var phi_8063_: bool;
    var phi_8065_: bool;
    var phi_8067_: bool;
    var phi_8068_: bool;
    var phi_8069_: bool;
    var phi_8070_: bool;
    var phi_8071_: bool;
    var phi_8115_: vec3<f32>;
    var phi_24303_: vec3<f32>;
    var phi_8116_: u32;
    var phi_8117_: vec3<f32>;
    var phi_8118_: u32;
    var phi_8119_: vec3<f32>;
    var phi_8254_: u32;
    var phi_8257_: f32;
    var phi_8259_: f32;
    var phi_8261_: f32;
    var phi_8263_: vec3<f32>;
    var phi_24358_: f32;
    var phi_24364_: f32;
    var phi_24392_: f32;
    var phi_24398_: f32;
    var phi_24426_: f32;
    var phi_24432_: f32;
    var phi_24460_: f32;
    var phi_24466_: f32;
    var phi_24494_: f32;
    var phi_24500_: f32;
    var phi_24528_: f32;
    var phi_24534_: f32;
    var phi_24562_: f32;
    var phi_24568_: f32;
    var phi_24596_: f32;
    var phi_24602_: f32;
    var phi_8255_: u32;
    var phi_8258_: f32;
    var phi_8260_: f32;
    var phi_8262_: f32;
    var phi_8264_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8677_: f32;
    var phi_8703_: vec3<f32>;
    var phi_8704_: f32;
    var phi_8705_: f32;
    var phi_8706_: vec3<f32>;
    var phi_8707_: f32;
    var phi_8708_: f32;
    var phi_8709_: vec3<f32>;
    var phi_8710_: f32;
    var phi_8711_: f32;
    var phi_8712_: vec3<f32>;
    var phi_8713_: f32;
    var phi_8714_: f32;
    var phi_8716_: vec3<f32>;
    var phi_8717_: f32;
    var phi_8718_: f32;
    var phi_8719_: f32;
    var phi_8019_: u32;
    var phi_8022_: vec3<f32>;
    var phi_8024_: f32;
    var phi_8026_: f32;
    var phi_8028_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_9009_: type_57;
    var local_42: f32;
    var phi_9010_: type_57;
    var local_43: f32;
    var local_44: f32;
    var phi_9011_: type_57;
    var phi_9012_: type_57;
    var phi_7691_: vec3<f32>;
    var phi_7694_: u32;
    var phi_7696_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9041_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_32042_: bool;
    var phi_9088_: vec3<f32>;
    var phi_32041_: bool;
    var phi_9089_: vec3<f32>;
    var phi_32040_: bool;
    var phi_9090_: vec3<f32>;
    var phi_32036_: bool;
    var phi_10363_: vec3<f32>;
    var phi_32035_: bool;
    var phi_10364_: vec3<f32>;
    var phi_32034_: bool;
    var phi_482_: vec3<f32>;
    var phi_485_: u32;
    var phi_32033_: bool;
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
            let _e271 = global;
            let _e281 = (_e271.x + (_e271.y * 4194240u));
            let _e285 = global_1.member.member[0u];
            let _e289 = global_1.member.member[1u];
            if (_e281 >= (_e285 * _e289)) {
                phi_447_ = type_45(0u, type_45().member_1);
            } else {
                phi_447_ = type_45(1u, _e281);
            }
            let _e297 = phi_447_;
            if (bitcast<i32>(_e297.member) != 0i) {
                let _e302 = (_e297.member_1 < arrayLength((&global_7.member)));
                if _e302 {
                } else {
                    break;
                }
                let _e307 = global_7.member[_e297.member_1].member_4[1u];
                if ((_e307 & 16u) == 0u) {
                } else {
                    phi_31572_ = false;
                    phi_481_ = vec3<f32>(0f, 0f, 0f);
                    phi_484_ = 0u;
                    loop {
                        let _e311 = phi_31572_;
                        let _e313 = phi_481_;
                        let _e315 = phi_484_;
                        local_55 = _e313;
                        local_56 = _e313;
                        local_57 = _e313;
                        let _e316 = (_e315 < 2u);
                        if _e316 {
                            let _e318 = ((2u * _e297.member_1) + _e315);
                            if (_e318 < arrayLength((&global_8.member))) {
                            } else {
                                phi_32033_ = true;
                                break;
                            }
                            let _e322 = global_8.member[_e318];
                            if ((((_e322.member_2.x * _e322.member_2.x) + (_e322.member_2.y * _e322.member_2.y)) + (_e322.member_2.z * _e322.member_2.z)) == 0f) {
                                phi_32035_ = _e311;
                                phi_10364_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                if (_e322.member.w < 0f) {
                                    let _e1277 = global_1.member.member_8[0u];
                                    let _e1280 = global_1.member.member_8[3u];
                                    if (_e1277 == 4294967295u) {
                                        let _e1282 = (_e1280 == 4294967295u);
                                        if _e1282 {
                                            let _e1285 = global_1.member.member_3[1u];
                                            let _e1288 = global_1.member.member_3[2u];
                                            let _e1291 = ((atan2(_e322.member_1.z, _e322.member_1.x) * 0.15915494f) + 0.5f);
                                            let _e1296 = (((_e1291 - floor(_e1291)) * f32(_e1285)) - 0.5f);
                                            let _e1298 = select(_e322.member_1.y, -1f, (_e322.member_1.y < -1f));
                                            let _e1305 = (((acos(select(_e1298, 1f, (_e1298 > 1f))) * 0.31830987f) * f32(_e1288)) - 0.5f);
                                            let _e1306 = (_e1288 - 1u);
                                            let _e1307 = f32(_e1306);
                                            if (0f <= _e1307) {
                                            } else {
                                                phi_32033_ = true;
                                                break;
                                            }
                                            let _e1310 = select(_e1305, 0f, (_e1305 < 0f));
                                            let _e1312 = select(_e1310, _e1307, (_e1310 > _e1307));
                                            let _e1313 = floor(_e1296);
                                            let _e1314 = floor(_e1312);
                                            let _e1315 = (_e1296 - _e1313);
                                            let _e1316 = (_e1312 - _e1314);
                                            let _e1327 = (_e1285 == 0u);
                                            if _e1327 {
                                                phi_32033_ = true;
                                                break;
                                            }
                                            let _e1328 = (bitcast<u32>((select(0i, select(select(i32(_e1313), i32(-2147483648), (_e1313 < -2147483600f)), 2147483647i, (_e1313 > 2147483500f)), (_e1313 == _e1313)) + bitcast<i32>(_e1285))) % _e1285);
                                            let _e1333 = select(select(u32(_e1314), 0u, (_e1314 < 0f)), 4294967295u, (_e1314 > 4294967000f));
                                            let _e1334 = (_e1333 + 1u);
                                            let _e1339 = global_1.member.member_2[2u];
                                            let _e1340 = (_e1333 * _e1285);
                                            if _e1327 {
                                                phi_32033_ = true;
                                                break;
                                            }
                                            let _e1341 = (_e1328 % _e1285);
                                            let _e1344 = (_e1339 + (3u * (_e1340 + _e1341)));
                                            let _e1346 = global_6.member[_e1344];
                                            let _e1349 = global_6.member[(_e1344 + 1u)];
                                            let _e1352 = global_6.member[(_e1344 + 2u)];
                                            if _e1327 {
                                                phi_32033_ = true;
                                                break;
                                            }
                                            let _e1355 = ((_e1328 + 1u) % _e1285);
                                            let _e1358 = (_e1339 + (3u * (_e1340 + _e1355)));
                                            let _e1360 = global_6.member[_e1358];
                                            let _e1363 = global_6.member[(_e1358 + 1u)];
                                            let _e1366 = global_6.member[(_e1358 + 2u)];
                                            let _e1374 = (vec3<f32>(_e1346, _e1349, _e1352) + vec3<f32>(((_e1360 - _e1346) * _e1315), ((_e1363 - _e1349) * _e1315), ((_e1366 - _e1352) * _e1315)));
                                            let _e1375 = (select(_e1334, _e1306, (_e1306 < _e1334)) * _e1285);
                                            if _e1327 {
                                                phi_32033_ = true;
                                                break;
                                            }
                                            let _e1378 = (_e1339 + (3u * (_e1375 + _e1341)));
                                            let _e1380 = global_6.member[_e1378];
                                            let _e1383 = global_6.member[(_e1378 + 1u)];
                                            let _e1386 = global_6.member[(_e1378 + 2u)];
                                            if _e1327 {
                                                phi_32033_ = true;
                                                break;
                                            }
                                            let _e1390 = (_e1339 + (3u * (_e1375 + _e1355)));
                                            let _e1392 = global_6.member[_e1390];
                                            let _e1395 = global_6.member[(_e1390 + 1u)];
                                            let _e1398 = global_6.member[(_e1390 + 2u)];
                                            let _e1406 = (vec3<f32>(_e1380, _e1383, _e1386) + vec3<f32>(((_e1392 - _e1380) * _e1315), ((_e1395 - _e1383) * _e1315), ((_e1398 - _e1386) * _e1315)));
                                            phi_860_ = (_e1374 + vec3<f32>(((_e1406.x - _e1374.x) * _e1316), ((_e1406.y - _e1374.y) * _e1316), ((_e1406.z - _e1374.z) * _e1316)));
                                        } else {
                                            phi_860_ = vec3<f32>();
                                        }
                                        let _e1422 = phi_860_;
                                        phi_862_ = _e1422;
                                        phi_863_ = select(true, false, _e1282);
                                    } else {
                                        phi_862_ = vec3<f32>();
                                        phi_863_ = true;
                                    }
                                    let _e1425 = phi_862_;
                                    let _e1427 = phi_863_;
                                    if _e1427 {
                                        let _e1428 = (_e1280 != 4294967295u);
                                        if (_e1277 != 4294967295u) {
                                            phi_31647_ = _e311;
                                            phi_873_ = vec3<f32>(0f, 0f, 0f);
                                            phi_876_ = 0i;
                                            phi_878_ = _e1277;
                                            loop {
                                                let _e1431 = phi_31647_;
                                                let _e1433 = phi_873_;
                                                let _e1435 = phi_876_;
                                                let _e1437 = phi_878_;
                                                local_17 = _e1433;
                                                local_18 = _e1433;
                                                local_19 = _e1433;
                                                local_61 = _e1433;
                                                local_62 = _e1433;
                                                if (_e1437 != 4294967295u) {
                                                    let _e1439 = (_e1435 < 2i);
                                                    if _e1439 {
                                                        let _e1441 = global_6.member[_e1437];
                                                        let _e1444 = global_6.member[(_e1437 + 1u)];
                                                        let _e1447 = global_6.member[(_e1437 + 2u)];
                                                        let _e1450 = global_6.member[(_e1437 + 3u)];
                                                        let _e1453 = global_6.member[(_e1437 + 4u)];
                                                        let _e1456 = global_6.member[(_e1437 + 5u)];
                                                        let _e1459 = global_6.member[(_e1437 + 6u)];
                                                        let _e1462 = global_6.member[(_e1437 + 7u)];
                                                        let _e1465 = global_6.member[(_e1437 + 8u)];
                                                        let _e1468 = global_6.member[(_e1437 + 9u)];
                                                        let _e1471 = global_6.member[(_e1437 + 10u)];
                                                        let _e1474 = global_6.member[(_e1437 + 11u)];
                                                        let _e1477 = global_6.member[(_e1437 + 12u)];
                                                        let _e1480 = global_6.member[(_e1437 + 13u)];
                                                        let _e1483 = global_6.member[(_e1437 + 14u)];
                                                        let _e1486 = global_6.member[(_e1437 + 15u)];
                                                        let _e1490 = global_6.member[(_e1437 + 16u)];
                                                        let _e1493 = global_6.member[(_e1437 + 17u)];
                                                        let _e1496 = global_6.member[(_e1437 + 18u)];
                                                        let _e1499 = global_6.member[(_e1437 + 19u)];
                                                        let _e1502 = global_6.member[(_e1437 + 20u)];
                                                        let _e1505 = global_6.member[(_e1437 + 30u)];
                                                        let _e1506 = bitcast<u32>(_e1505);
                                                        let _e1509 = global_6.member[(_e1437 + 31u)];
                                                        let _e1510 = bitcast<u32>(_e1509);
                                                        let _e1513 = global_6.member[(_e1437 + 32u)];
                                                        let _e1515 = (_e322.member_1.y < 0f);
                                                        if _e1515 {
                                                            phi_2666_ = type_51();
                                                            phi_2667_ = true;
                                                            phi_2668_ = false;
                                                        } else {
                                                            let _e1516 = (_e1510 == 4294967295u);
                                                            if _e1516 {
                                                                phi_2664_ = type_51();
                                                                phi_2665_ = false;
                                                            } else {
                                                                let _e1530 = (_e322.member.x - _e1441);
                                                                let _e1531 = (_e322.member.y - _e1444);
                                                                let _e1532 = (_e322.member.z - _e1447);
                                                                let _e1540 = ((((((_e322.member.x * _e322.member.x) + (_e322.member.y * _e322.member.y)) + (_e322.member.z * _e322.member.z)) - (2f * (((_e322.member.x * _e1441) + (_e322.member.y * _e1444)) + (_e322.member.z * _e1447)))) + _e1456) / (sqrt((((_e1530 * _e1530) + (_e1531 * _e1531)) + (_e1532 * _e1532))) + _e1450));
                                                                let _e1541 = (_e1437 + 21u);
                                                                let _e1544 = global_6.member[(_e1437 + 28u)];
                                                                if (_e1540 > _e1544) {
                                                                    phi_2654_ = type_53(0u, type_53().member_1);
                                                                } else {
                                                                    let _e1547 = global_6.member[_e1541];
                                                                    if (_e1540 != _e1540) {
                                                                        phi_21468_ = true;
                                                                    } else {
                                                                        phi_21468_ = (_e1547 >= _e1540);
                                                                    }
                                                                    let _e1551 = phi_21468_;
                                                                    let _e1552 = select(_e1540, _e1547, _e1551);
                                                                    phi_1300_ = 0u;
                                                                    loop {
                                                                        let _e1554 = phi_1300_;
                                                                        local_8 = _e1554;
                                                                        local_9 = _e1554;
                                                                        local_10 = _e1554;
                                                                        if (_e1554 < 8u) {
                                                                            let _e1558 = global_6.member[(_e1541 + _e1554)];
                                                                            let _e1559 = (_e1558 <= _e1552);
                                                                            if _e1559 {
                                                                                phi_1330_ = (_e1554 + 1u);
                                                                            } else {
                                                                                phi_1330_ = u32();
                                                                            }
                                                                            let _e1562 = phi_1330_;
                                                                            phi_1301_ = _e1562;
                                                                            phi_1331_ = _e1559;
                                                                        } else {
                                                                            phi_1301_ = u32();
                                                                            phi_1331_ = false;
                                                                        }
                                                                        let _e1564 = phi_1301_;
                                                                        let _e1566 = phi_1331_;
                                                                        phi_1300_ = _e1564;
                                                                        if !(_e1566) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e1569 = local_8;
                                                                    if (_e1569 < 1u) {
                                                                        phi_1349_ = 1u;
                                                                    } else {
                                                                        let _e1572 = local_9;
                                                                        let _e1575 = local_10;
                                                                        phi_1349_ = select(_e1575, 7u, (_e1572 > 7u));
                                                                    }
                                                                    let _e1578 = phi_1349_;
                                                                    let _e1579 = (_e1578 - 1u);
                                                                    let _e1582 = global_6.member[(_e1541 + _e1579)];
                                                                    let _e1585 = global_6.member[(_e1541 + _e1578)];
                                                                    let _e1588 = ((_e1552 - _e1582) / (_e1585 - _e1582));
                                                                    let _e1589 = select(_e322.member_1.y, 0f, _e1515);
                                                                    let _e1595 = (sqrt((asin(select(_e1589, 1f, (_e1589 > 1f))) * 0.63661975f)) * 127f);
                                                                    let _e1597 = (atan2(_e322.member_1.z, _e322.member_1.x) * 0.15915494f);
                                                                    let _e1600 = ((_e1597 - floor(_e1597)) * 256f);
                                                                    let _e1601 = floor(_e1600);
                                                                    let _e1607 = (select(select(u32(_e1601), 0u, (_e1601 < 0f)), 4294967295u, (_e1601 > 4294967000f)) % 256u);
                                                                    let _e1608 = floor(_e1595);
                                                                    let _e1613 = select(select(u32(_e1608), 0u, (_e1608 < 0f)), 4294967295u, (_e1608 > 4294967000f));
                                                                    let _e1615 = select(_e1613, 126u, (126u < _e1613));
                                                                    let _e1617 = ((_e1607 + 1u) % 256u);
                                                                    let _e1619 = (_e1600 - _e1601);
                                                                    let _e1621 = (_e1595 - f32(_e1615));
                                                                    let _e1623 = (_e1510 + (_e1579 * 294912u));
                                                                    let _e1624 = (_e1615 * 256u);
                                                                    let _e1626 = (9u * (_e1624 + _e1607));
                                                                    let _e1627 = (_e1623 + _e1626);
                                                                    let _e1629 = (9u * (_e1624 + _e1617));
                                                                    let _e1630 = (_e1623 + _e1629);
                                                                    let _e1631 = ((_e1615 + 1u) * 256u);
                                                                    let _e1633 = (9u * (_e1631 + _e1607));
                                                                    let _e1634 = (_e1623 + _e1633);
                                                                    let _e1636 = (9u * (_e1631 + _e1617));
                                                                    let _e1637 = (_e1623 + _e1636);
                                                                    let _e1638 = (1f - _e1619);
                                                                    let _e1639 = (1f - _e1621);
                                                                    let _e1640 = (_e1638 * _e1639);
                                                                    let _e1641 = (_e1619 * _e1639);
                                                                    let _e1642 = (_e1638 * _e1621);
                                                                    let _e1643 = (_e1619 * _e1621);
                                                                    let _e1645 = global_6.member[_e1627];
                                                                    let _e1648 = global_6.member[(_e1627 + 1u)];
                                                                    let _e1651 = global_6.member[(_e1627 + 2u)];
                                                                    let _e1656 = global_6.member[_e1630];
                                                                    let _e1659 = global_6.member[(_e1630 + 1u)];
                                                                    let _e1662 = global_6.member[(_e1630 + 2u)];
                                                                    let _e1670 = global_6.member[_e1634];
                                                                    let _e1673 = global_6.member[(_e1634 + 1u)];
                                                                    let _e1676 = global_6.member[(_e1634 + 2u)];
                                                                    let _e1684 = global_6.member[_e1637];
                                                                    let _e1687 = global_6.member[(_e1637 + 1u)];
                                                                    let _e1690 = global_6.member[(_e1637 + 2u)];
                                                                    let _e1699 = global_6.member[(_e1627 + 3u)];
                                                                    let _e1702 = global_6.member[(_e1627 + 4u)];
                                                                    let _e1705 = global_6.member[(_e1627 + 5u)];
                                                                    let _e1711 = global_6.member[(_e1630 + 3u)];
                                                                    let _e1714 = global_6.member[(_e1630 + 4u)];
                                                                    let _e1717 = global_6.member[(_e1630 + 5u)];
                                                                    let _e1726 = global_6.member[(_e1634 + 3u)];
                                                                    let _e1729 = global_6.member[(_e1634 + 4u)];
                                                                    let _e1732 = global_6.member[(_e1634 + 5u)];
                                                                    let _e1741 = global_6.member[(_e1637 + 3u)];
                                                                    let _e1744 = global_6.member[(_e1637 + 4u)];
                                                                    let _e1747 = global_6.member[(_e1637 + 5u)];
                                                                    let _e1756 = global_6.member[(_e1627 + 6u)];
                                                                    let _e1759 = global_6.member[(_e1627 + 7u)];
                                                                    let _e1762 = global_6.member[(_e1627 + 8u)];
                                                                    let _e1768 = global_6.member[(_e1630 + 6u)];
                                                                    let _e1771 = global_6.member[(_e1630 + 7u)];
                                                                    let _e1774 = global_6.member[(_e1630 + 8u)];
                                                                    let _e1783 = global_6.member[(_e1634 + 6u)];
                                                                    let _e1786 = global_6.member[(_e1634 + 7u)];
                                                                    let _e1789 = global_6.member[(_e1634 + 8u)];
                                                                    let _e1798 = global_6.member[(_e1637 + 6u)];
                                                                    let _e1801 = global_6.member[(_e1637 + 7u)];
                                                                    let _e1804 = global_6.member[(_e1637 + 8u)];
                                                                    let _e1812 = (_e1510 + (_e1578 * 294912u));
                                                                    let _e1813 = (_e1812 + _e1626);
                                                                    let _e1814 = (_e1812 + _e1629);
                                                                    let _e1815 = (_e1812 + _e1633);
                                                                    let _e1816 = (_e1812 + _e1636);
                                                                    let _e1818 = global_6.member[_e1813];
                                                                    let _e1821 = global_6.member[(_e1813 + 1u)];
                                                                    let _e1824 = global_6.member[(_e1813 + 2u)];
                                                                    let _e1829 = global_6.member[_e1814];
                                                                    let _e1832 = global_6.member[(_e1814 + 1u)];
                                                                    let _e1835 = global_6.member[(_e1814 + 2u)];
                                                                    let _e1843 = global_6.member[_e1815];
                                                                    let _e1846 = global_6.member[(_e1815 + 1u)];
                                                                    let _e1849 = global_6.member[(_e1815 + 2u)];
                                                                    let _e1857 = global_6.member[_e1816];
                                                                    let _e1860 = global_6.member[(_e1816 + 1u)];
                                                                    let _e1863 = global_6.member[(_e1816 + 2u)];
                                                                    let _e1872 = global_6.member[(_e1813 + 3u)];
                                                                    let _e1875 = global_6.member[(_e1813 + 4u)];
                                                                    let _e1878 = global_6.member[(_e1813 + 5u)];
                                                                    let _e1884 = global_6.member[(_e1814 + 3u)];
                                                                    let _e1887 = global_6.member[(_e1814 + 4u)];
                                                                    let _e1890 = global_6.member[(_e1814 + 5u)];
                                                                    let _e1899 = global_6.member[(_e1815 + 3u)];
                                                                    let _e1902 = global_6.member[(_e1815 + 4u)];
                                                                    let _e1905 = global_6.member[(_e1815 + 5u)];
                                                                    let _e1914 = global_6.member[(_e1816 + 3u)];
                                                                    let _e1917 = global_6.member[(_e1816 + 4u)];
                                                                    let _e1920 = global_6.member[(_e1816 + 5u)];
                                                                    let _e1929 = global_6.member[(_e1813 + 6u)];
                                                                    let _e1932 = global_6.member[(_e1813 + 7u)];
                                                                    let _e1935 = global_6.member[(_e1813 + 8u)];
                                                                    let _e1941 = global_6.member[(_e1814 + 6u)];
                                                                    let _e1944 = global_6.member[(_e1814 + 7u)];
                                                                    let _e1947 = global_6.member[(_e1814 + 8u)];
                                                                    let _e1956 = global_6.member[(_e1815 + 6u)];
                                                                    let _e1959 = global_6.member[(_e1815 + 7u)];
                                                                    let _e1962 = global_6.member[(_e1815 + 8u)];
                                                                    let _e1971 = global_6.member[(_e1816 + 6u)];
                                                                    let _e1974 = global_6.member[(_e1816 + 7u)];
                                                                    let _e1977 = global_6.member[(_e1816 + 8u)];
                                                                    let _e1984 = (1f - _e1588);
                                                                    phi_2654_ = type_53(1u, type_51(vec3<f32>(((((((_e1645 * _e1640) + (_e1656 * _e1641)) + (_e1670 * _e1642)) + (_e1684 * _e1643)) * _e1984) + (((((_e1818 * _e1640) + (_e1829 * _e1641)) + (_e1843 * _e1642)) + (_e1857 * _e1643)) * _e1588)), ((((((_e1648 * _e1640) + (_e1659 * _e1641)) + (_e1673 * _e1642)) + (_e1687 * _e1643)) * _e1984) + (((((_e1821 * _e1640) + (_e1832 * _e1641)) + (_e1846 * _e1642)) + (_e1860 * _e1643)) * _e1588)), ((((((_e1651 * _e1640) + (_e1662 * _e1641)) + (_e1676 * _e1642)) + (_e1690 * _e1643)) * _e1984) + (((((_e1824 * _e1640) + (_e1835 * _e1641)) + (_e1849 * _e1642)) + (_e1863 * _e1643)) * _e1588))), vec3<f32>(((((((_e1699 * _e1640) + (_e1711 * _e1641)) + (_e1726 * _e1642)) + (_e1741 * _e1643)) * _e1984) + (((((_e1872 * _e1640) + (_e1884 * _e1641)) + (_e1899 * _e1642)) + (_e1914 * _e1643)) * _e1588)), ((((((_e1702 * _e1640) + (_e1714 * _e1641)) + (_e1729 * _e1642)) + (_e1744 * _e1643)) * _e1984) + (((((_e1875 * _e1640) + (_e1887 * _e1641)) + (_e1902 * _e1642)) + (_e1917 * _e1643)) * _e1588)), ((((((_e1705 * _e1640) + (_e1717 * _e1641)) + (_e1732 * _e1642)) + (_e1747 * _e1643)) * _e1984) + (((((_e1878 * _e1640) + (_e1890 * _e1641)) + (_e1905 * _e1642)) + (_e1920 * _e1643)) * _e1588))), vec3<f32>(((((((_e1756 * _e1640) + (_e1768 * _e1641)) + (_e1783 * _e1642)) + (_e1798 * _e1643)) * _e1984) + (((((_e1929 * _e1640) + (_e1941 * _e1641)) + (_e1956 * _e1642)) + (_e1971 * _e1643)) * _e1588)), ((((((_e1759 * _e1640) + (_e1771 * _e1641)) + (_e1786 * _e1642)) + (_e1801 * _e1643)) * _e1984) + (((((_e1932 * _e1640) + (_e1944 * _e1641)) + (_e1959 * _e1642)) + (_e1974 * _e1643)) * _e1588)), ((((((_e1762 * _e1640) + (_e1774 * _e1641)) + (_e1789 * _e1642)) + (_e1804 * _e1643)) * _e1984) + (((((_e1935 * _e1640) + (_e1947 * _e1641)) + (_e1962 * _e1642)) + (_e1977 * _e1643)) * _e1588)))));
                                                                }
                                                                let _e2021 = phi_2654_;
                                                                let _e2024 = (bitcast<i32>(_e2021.member) != 0i);
                                                                if _e2024 {
                                                                    phi_2662_ = _e2021.member_1;
                                                                } else {
                                                                    phi_2662_ = type_51();
                                                                }
                                                                let _e2027 = phi_2662_;
                                                                phi_2664_ = _e2027;
                                                                phi_2665_ = select(true, false, _e2024);
                                                            }
                                                            let _e2030 = phi_2664_;
                                                            let _e2032 = phi_2665_;
                                                            phi_2666_ = _e2030;
                                                            phi_2667_ = _e1516;
                                                            phi_2668_ = _e2032;
                                                        }
                                                        let _e2034 = phi_2666_;
                                                        let _e2036 = phi_2667_;
                                                        let _e2038 = phi_2668_;
                                                        if select(_e2038, true, _e2036) {
                                                            let _e2047 = ((((_e322.member.x - _e1441) * _e322.member_1.x) + ((_e322.member.y - _e1444) * _e322.member_1.y)) + ((_e322.member.z - _e1447) * _e322.member_1.z));
                                                            let _e2059 = ((((_e322.member.x * _e322.member.x) + (_e322.member.y * _e322.member.y)) + (_e322.member.z * _e322.member.z)) - (2f * (((_e322.member.x * _e1441) + (_e322.member.y * _e1444)) + (_e322.member.z * _e1447))));
                                                            let _e2060 = (_e2059 + _e1459);
                                                            let _e2061 = (_e2047 * _e2047);
                                                            let _e2062 = (_e2061 - _e2060);
                                                            if (_e2062 < 0f) {
                                                                phi_21607_ = type_55(0u, type_55().member_1);
                                                                phi_21608_ = type_55();
                                                                phi_21609_ = true;
                                                            } else {
                                                                if (_e2047 < 0f) {
                                                                    phi_21579_ = -(sqrt(_e2062));
                                                                } else {
                                                                    phi_21579_ = sqrt(_e2062);
                                                                }
                                                                let _e2069 = phi_21579_;
                                                                let _e2071 = -((_e2047 + _e2069));
                                                                let _e2072 = (_e2071 == 0f);
                                                                if _e2072 {
                                                                    phi_21602_ = type_55(1u, type_54(0f, 0f));
                                                                    phi_21603_ = type_55();
                                                                } else {
                                                                    let _e2073 = (_e2060 / _e2071);
                                                                    if (_e2071 < _e2073) {
                                                                        phi_21591_ = type_54(_e2071, _e2073);
                                                                    } else {
                                                                        phi_21591_ = type_54(_e2073, _e2071);
                                                                    }
                                                                    let _e2078 = phi_21591_;
                                                                    phi_21602_ = type_55();
                                                                    phi_21603_ = type_55(1u, _e2078);
                                                                }
                                                                let _e2081 = phi_21602_;
                                                                let _e2083 = phi_21603_;
                                                                phi_21607_ = _e2081;
                                                                phi_21608_ = _e2083;
                                                                phi_21609_ = _e2072;
                                                            }
                                                            let _e2088 = phi_21607_;
                                                            let _e2090 = phi_21608_;
                                                            let _e2092 = phi_21609_;
                                                            if _e2092 {
                                                                phi_21613_ = _e2088;
                                                            } else {
                                                                phi_21613_ = _e2090;
                                                            }
                                                            let _e2094 = phi_21613_;
                                                            if (bitcast<i32>(_e2094.member) != 0i) {
                                                                phi_21500_ = type_55(0u, _e2094.member_1);
                                                            } else {
                                                                phi_21500_ = type_55(1u, type_55().member_1);
                                                            }
                                                            let _e2104 = phi_21500_;
                                                            if (bitcast<i32>(_e2104.member) != 0i) {
                                                                phi_21547_ = type_55(0u, type_55().member_1);
                                                            } else {
                                                                if (_e2104.member_1.member_1 <= 0f) {
                                                                    phi_21543_ = type_55(0u, type_55().member_1);
                                                                } else {
                                                                    if (_e2104.member_1.member != _e2104.member_1.member) {
                                                                        phi_21651_ = true;
                                                                    } else {
                                                                        phi_21651_ = (0f >= _e2104.member_1.member);
                                                                    }
                                                                    let _e2116 = phi_21651_;
                                                                    let _e2117 = select(_e2104.member_1.member, 0f, _e2116);
                                                                    let _e2118 = (_e2059 + _e1456);
                                                                    if (_e2118 > 0f) {
                                                                        let _e2120 = (_e2061 - _e2118);
                                                                        if (_e2120 < 0f) {
                                                                            phi_21759_ = type_55(0u, type_55().member_1);
                                                                            phi_21760_ = type_55();
                                                                            phi_21761_ = true;
                                                                        } else {
                                                                            if (_e2047 < 0f) {
                                                                                phi_21731_ = -(sqrt(_e2120));
                                                                            } else {
                                                                                phi_21731_ = sqrt(_e2120);
                                                                            }
                                                                            let _e2127 = phi_21731_;
                                                                            let _e2129 = -((_e2047 + _e2127));
                                                                            let _e2130 = (_e2129 == 0f);
                                                                            if _e2130 {
                                                                                phi_21754_ = type_55(1u, type_54(0f, 0f));
                                                                                phi_21755_ = type_55();
                                                                            } else {
                                                                                let _e2131 = (_e2118 / _e2129);
                                                                                if (_e2129 < _e2131) {
                                                                                    phi_21743_ = type_54(_e2129, _e2131);
                                                                                } else {
                                                                                    phi_21743_ = type_54(_e2131, _e2129);
                                                                                }
                                                                                let _e2136 = phi_21743_;
                                                                                phi_21754_ = type_55();
                                                                                phi_21755_ = type_55(1u, _e2136);
                                                                            }
                                                                            let _e2139 = phi_21754_;
                                                                            let _e2141 = phi_21755_;
                                                                            phi_21759_ = _e2139;
                                                                            phi_21760_ = _e2141;
                                                                            phi_21761_ = _e2130;
                                                                        }
                                                                        let _e2146 = phi_21759_;
                                                                        let _e2148 = phi_21760_;
                                                                        let _e2150 = phi_21761_;
                                                                        if _e2150 {
                                                                            phi_21765_ = _e2146;
                                                                        } else {
                                                                            phi_21765_ = _e2148;
                                                                        }
                                                                        let _e2152 = phi_21765_;
                                                                        if (bitcast<i32>(_e2152.member) != 0i) {
                                                                            phi_21699_ = select(_e2152.member_1.member_1, _e2152.member_1.member, (_e2152.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_21699_ = -1f;
                                                                        }
                                                                        let _e2163 = phi_21699_;
                                                                        if (_e2163 >= 0f) {
                                                                            if (_e2104.member_1.member_1 != _e2104.member_1.member_1) {
                                                                                phi_21803_ = true;
                                                                            } else {
                                                                                phi_21803_ = (_e2163 <= _e2104.member_1.member_1);
                                                                            }
                                                                            let _e2168 = phi_21803_;
                                                                            phi_21528_ = select(_e2104.member_1.member_1, _e2163, _e2168);
                                                                        } else {
                                                                            phi_21528_ = _e2104.member_1.member_1;
                                                                        }
                                                                        let _e2171 = phi_21528_;
                                                                        phi_21530_ = _e2171;
                                                                    } else {
                                                                        phi_21530_ = _e2104.member_1.member_1;
                                                                    }
                                                                    let _e2173 = phi_21530_;
                                                                    if (_e2173 <= _e2117) {
                                                                        phi_21539_ = type_55(0u, type_55().member_1);
                                                                    } else {
                                                                        phi_21539_ = type_55(1u, type_54(_e2117, _e2173));
                                                                    }
                                                                    let _e2181 = phi_21539_;
                                                                    phi_21543_ = _e2181;
                                                                }
                                                                let _e2186 = phi_21543_;
                                                                phi_21547_ = _e2186;
                                                            }
                                                            let _e2191 = phi_21547_;
                                                            if (bitcast<i32>(_e2191.member) != 0i) {
                                                                phi_2686_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_2689_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_2691_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_2693_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_2695_ = 0u;
                                                                loop {
                                                                    let _e2200 = phi_2686_;
                                                                    let _e2202 = phi_2689_;
                                                                    let _e2204 = phi_2691_;
                                                                    let _e2206 = phi_2693_;
                                                                    let _e2208 = phi_2695_;
                                                                    local_12 = _e2206;
                                                                    local_13 = _e2204;
                                                                    local_14 = _e2202;
                                                                    let _e2209 = (_e2208 < 32u);
                                                                    if _e2209 {
                                                                        let _e2210 = -(_e2047);
                                                                        if (_e2191.member_1.member <= _e2191.member_1.member_1) {
                                                                        } else {
                                                                            phi_31615_ = true;
                                                                            break;
                                                                        }
                                                                        let _e2213 = select(_e2210, _e2191.member_1.member, (_e2210 < _e2191.member_1.member));
                                                                        let _e2215 = select(_e2213, _e2191.member_1.member_1, (_e2213 > _e2191.member_1.member_1));
                                                                        let _e2216 = (_e2215 - _e2191.member_1.member);
                                                                        let _e2217 = (_e2191.member_1.member_1 - _e2215);
                                                                        let _e2218 = (_e2191.member_1.member_1 - _e2191.member_1.member);
                                                                        let _e2219 = (0.000000001f * _e2218);
                                                                        if (_e2216 <= _e2219) {
                                                                            let _e2294 = (f32(_e2208) * 0.03125f);
                                                                            let _e2297 = (f32((_e2208 + 1u)) * 0.03125f);
                                                                            let _e2299 = ((_e2294 + _e2297) * 0.5f);
                                                                            phi_2796_ = type_54((_e2191.member_1.member + ((_e2218 * _e2299) * _e2299)), (_e2218 * ((_e2297 * _e2297) - (_e2294 * _e2294))));
                                                                        } else {
                                                                            if (_e2217 <= _e2219) {
                                                                                let _e2277 = (f32((31u - _e2208)) * 0.03125f);
                                                                                let _e2280 = (f32((32u - _e2208)) * 0.03125f);
                                                                                let _e2282 = ((_e2277 + _e2280) * 0.5f);
                                                                                phi_2795_ = type_54((_e2191.member_1.member_1 - ((_e2218 * _e2282) * _e2282)), (_e2218 * ((_e2280 * _e2280) - (_e2277 * _e2277))));
                                                                            } else {
                                                                                let _e2225 = round(((32f * _e2216) / (_e2216 + _e2217)));
                                                                                let _e2230 = select(select(u32(_e2225), 0u, (_e2225 < 0f)), 4294967295u, (_e2225 > 4294967000f));
                                                                                if (_e2230 < 1u) {
                                                                                    phi_2779_ = 1u;
                                                                                } else {
                                                                                    phi_2779_ = select(_e2230, 31u, (_e2230 > 31u));
                                                                                }
                                                                                let _e2235 = phi_2779_;
                                                                                if (_e2208 < _e2235) {
                                                                                    let _e2256 = ((_e2235 - 1u) - _e2208);
                                                                                    let _e2258 = f32(_e2235);
                                                                                    let _e2259 = (f32(_e2256) / _e2258);
                                                                                    let _e2262 = (f32((_e2256 + 1u)) / _e2258);
                                                                                    let _e2264 = ((_e2259 + _e2262) * 0.5f);
                                                                                    phi_2794_ = type_54((_e2215 - ((_e2216 * _e2264) * _e2264)), (_e2216 * ((_e2262 * _e2262) - (_e2259 * _e2259))));
                                                                                } else {
                                                                                    let _e2238 = (_e2208 - _e2235);
                                                                                    let _e2240 = f32((32u - _e2235));
                                                                                    let _e2241 = (f32(_e2238) / _e2240);
                                                                                    let _e2244 = (f32((_e2238 + 1u)) / _e2240);
                                                                                    let _e2246 = ((_e2241 + _e2244) * 0.5f);
                                                                                    phi_2794_ = type_54((_e2215 + ((_e2217 * _e2246) * _e2246)), (_e2217 * ((_e2244 * _e2244) - (_e2241 * _e2241))));
                                                                                }
                                                                                let _e2274 = phi_2794_;
                                                                                phi_2795_ = _e2274;
                                                                            }
                                                                            let _e2292 = phi_2795_;
                                                                            phi_2796_ = _e2292;
                                                                        }
                                                                        let _e2309 = phi_2796_;
                                                                        let _e2315 = (_e322.member.x + (_e322.member_1.x * _e2309.member));
                                                                        let _e2316 = (_e322.member.y + (_e322.member_1.y * _e2309.member));
                                                                        let _e2317 = (_e322.member.z + (_e322.member_1.z * _e2309.member));
                                                                        let _e2330 = ((((_e2315 * _e2315) + (_e2316 * _e2316)) + (_e2317 * _e2317)) - (2f * (((_e2315 * _e1441) + (_e2316 * _e1444)) + (_e2317 * _e1447))));
                                                                        let _e2331 = (_e2330 + _e1456);
                                                                        let _e2332 = (_e2315 - _e1441);
                                                                        let _e2333 = (_e2316 - _e1444);
                                                                        let _e2334 = (_e2317 - _e1447);
                                                                        let _e2340 = sqrt((((_e2332 * _e2332) + (_e2333 * _e2333)) + (_e2334 * _e2334)));
                                                                        let _e2342 = (_e2331 / (_e2340 + _e1450));
                                                                        let _e2346 = (1f - (abs((_e2342 - 25000f)) * 0.00006666667f));
                                                                        if (_e2346 != _e2346) {
                                                                            phi_22002_ = true;
                                                                        } else {
                                                                            phi_22002_ = (0f >= _e2346);
                                                                        }
                                                                        let _e2350 = phi_22002_;
                                                                        let _e2352 = -(_e2342);
                                                                        let _e2355 = (exp((_e2352 / _e1471)) * _e2309.member_1);
                                                                        let _e2358 = (exp((_e2352 / _e1474)) * _e2309.member_1);
                                                                        let _e2360 = (_e2355 * 0.5f);
                                                                        let _e2361 = (_e2358 * 0.5f);
                                                                        let _e2362 = ((select(_e2346, 0f, _e2350) * _e2309.member_1) * 0.5f);
                                                                        let _e2364 = (_e2200.x + _e2360);
                                                                        let _e2366 = (_e2200.y + _e2361);
                                                                        let _e2368 = (_e2200.z + _e2362);
                                                                        let _e2373 = (_e1490 * 1.1f);
                                                                        let _e2374 = (_e2373 * _e2366);
                                                                        let _e2393 = (_e1490 * _e2358);
                                                                        if (_e1506 == 4294967295u) {
                                                                            phi_3124_ = vec3<f32>(0f, 0f, 0f);
                                                                        } else {
                                                                            let _e2401 = (1f / _e2340);
                                                                            let _e2409 = ((((_e2332 * _e2401) * _e1462) + ((_e2333 * _e2401) * _e1465)) + ((_e2334 * _e2401) * _e1468));
                                                                            if (_e2342 != _e2342) {
                                                                                phi_22129_ = true;
                                                                            } else {
                                                                                phi_22129_ = (0f >= _e2342);
                                                                            }
                                                                            let _e2413 = phi_22129_;
                                                                            let _e2417 = sqrt((select(_e2342, 0f, _e2413) / (_e1453 - _e1450)));
                                                                            if (_e2417 != _e2417) {
                                                                                phi_22144_ = true;
                                                                            } else {
                                                                                phi_22144_ = (1f <= _e2417);
                                                                            }
                                                                            let _e2421 = phi_22144_;
                                                                            let _e2423 = (select(_e2417, 1f, _e2421) * 31f);
                                                                            let _e2425 = select(_e2409, -1f, (_e2409 < -1f));
                                                                            let _e2429 = ((select(_e2425, 1f, (_e2425 > 1f)) + 1f) * 15.5f);
                                                                            let _e2434 = select(select(u32(_e2423), 0u, (_e2423 < 0f)), 4294967295u, (_e2423 > 4294967000f));
                                                                            let _e2436 = select(_e2434, 30u, (30u < _e2434));
                                                                            let _e2441 = select(select(u32(_e2429), 0u, (_e2429 < 0f)), 4294967295u, (_e2429 > 4294967000f));
                                                                            let _e2443 = select(_e2441, 30u, (30u < _e2441));
                                                                            let _e2445 = (_e2423 - f32(_e2436));
                                                                            let _e2447 = (_e2429 - f32(_e2443));
                                                                            let _e2451 = (_e1506 + (3u * ((_e2436 * 32u) + _e2443)));
                                                                            let _e2456 = (_e1506 + (3u * (((_e2436 + 1u) * 32u) + _e2443)));
                                                                            let _e2458 = global_6.member[_e2451];
                                                                            let _e2461 = global_6.member[(_e2451 + 1u)];
                                                                            let _e2464 = global_6.member[(_e2451 + 2u)];
                                                                            let _e2465 = (1f - _e2447);
                                                                            let _e2471 = global_6.member[(_e2451 + 3u)];
                                                                            let _e2474 = global_6.member[(_e2451 + 4u)];
                                                                            let _e2477 = global_6.member[(_e2451 + 5u)];
                                                                            let _e2484 = (1f - _e2445);
                                                                            let _e2489 = global_6.member[_e2456];
                                                                            let _e2492 = global_6.member[(_e2456 + 1u)];
                                                                            let _e2495 = global_6.member[(_e2456 + 2u)];
                                                                            let _e2501 = global_6.member[(_e2456 + 3u)];
                                                                            let _e2504 = global_6.member[(_e2456 + 4u)];
                                                                            let _e2507 = global_6.member[(_e2456 + 5u)];
                                                                            phi_3124_ = vec3<f32>(((((_e2458 * _e2465) + (_e2471 * _e2447)) * _e2484) + (((_e2489 * _e2465) + (_e2501 * _e2447)) * _e2445)), ((((_e2461 * _e2465) + (_e2474 * _e2447)) * _e2484) + (((_e2492 * _e2465) + (_e2504 * _e2447)) * _e2445)), ((((_e2464 * _e2465) + (_e2477 * _e2447)) * _e2484) + (((_e2495 * _e2465) + (_e2507 * _e2447)) * _e2445)));
                                                                        }
                                                                        let _e2522 = phi_3124_;
                                                                        let _e2540 = (((_e2332 * _e1462) + (_e2333 * _e1465)) + (_e2334 * _e1468));
                                                                        let _e2541 = (_e2540 * _e2540);
                                                                        let _e2542 = (_e2541 - _e2331);
                                                                        if (_e2542 < 0f) {
                                                                            phi_22294_ = type_55(0u, type_55().member_1);
                                                                            phi_22295_ = type_55();
                                                                            phi_22296_ = true;
                                                                        } else {
                                                                            if (_e2540 < 0f) {
                                                                                phi_22266_ = -(sqrt(_e2542));
                                                                            } else {
                                                                                phi_22266_ = sqrt(_e2542);
                                                                            }
                                                                            let _e2549 = phi_22266_;
                                                                            let _e2551 = -((_e2540 + _e2549));
                                                                            let _e2552 = (_e2551 == 0f);
                                                                            if _e2552 {
                                                                                phi_22289_ = type_55(1u, type_54(0f, 0f));
                                                                                phi_22290_ = type_55();
                                                                            } else {
                                                                                let _e2553 = (_e2331 / _e2551);
                                                                                if (_e2551 < _e2553) {
                                                                                    phi_22278_ = type_54(_e2551, _e2553);
                                                                                } else {
                                                                                    phi_22278_ = type_54(_e2553, _e2551);
                                                                                }
                                                                                let _e2558 = phi_22278_;
                                                                                phi_22289_ = type_55();
                                                                                phi_22290_ = type_55(1u, _e2558);
                                                                            }
                                                                            let _e2561 = phi_22289_;
                                                                            let _e2563 = phi_22290_;
                                                                            phi_22294_ = _e2561;
                                                                            phi_22295_ = _e2563;
                                                                            phi_22296_ = _e2552;
                                                                        }
                                                                        let _e2568 = phi_22294_;
                                                                        let _e2570 = phi_22295_;
                                                                        let _e2572 = phi_22296_;
                                                                        if _e2572 {
                                                                            phi_22300_ = _e2568;
                                                                        } else {
                                                                            phi_22300_ = _e2570;
                                                                        }
                                                                        let _e2574 = phi_22300_;
                                                                        if (bitcast<i32>(_e2574.member) != 0i) {
                                                                            phi_22234_ = select(_e2574.member_1.member_1, _e2574.member_1.member, (_e2574.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_22234_ = -1f;
                                                                        }
                                                                        let _e2585 = phi_22234_;
                                                                        if (_e2585 >= 0f) {
                                                                            phi_22217_ = type_56(0u, type_56().member_1);
                                                                        } else {
                                                                            let _e2587 = (_e2330 + _e1459);
                                                                            let _e2588 = (_e2541 - _e2587);
                                                                            if (_e2588 < 0f) {
                                                                                phi_22404_ = type_55(0u, type_55().member_1);
                                                                                phi_22405_ = type_55();
                                                                                phi_22406_ = true;
                                                                            } else {
                                                                                if (_e2540 < 0f) {
                                                                                    phi_22376_ = -(sqrt(_e2588));
                                                                                } else {
                                                                                    phi_22376_ = sqrt(_e2588);
                                                                                }
                                                                                let _e2595 = phi_22376_;
                                                                                let _e2597 = -((_e2540 + _e2595));
                                                                                let _e2598 = (_e2597 == 0f);
                                                                                if _e2598 {
                                                                                    phi_22399_ = type_55(1u, type_54(0f, 0f));
                                                                                    phi_22400_ = type_55();
                                                                                } else {
                                                                                    let _e2599 = (_e2587 / _e2597);
                                                                                    if (_e2597 < _e2599) {
                                                                                        phi_22388_ = type_54(_e2597, _e2599);
                                                                                    } else {
                                                                                        phi_22388_ = type_54(_e2599, _e2597);
                                                                                    }
                                                                                    let _e2604 = phi_22388_;
                                                                                    phi_22399_ = type_55();
                                                                                    phi_22400_ = type_55(1u, _e2604);
                                                                                }
                                                                                let _e2607 = phi_22399_;
                                                                                let _e2609 = phi_22400_;
                                                                                phi_22404_ = _e2607;
                                                                                phi_22405_ = _e2609;
                                                                                phi_22406_ = _e2598;
                                                                            }
                                                                            let _e2614 = phi_22404_;
                                                                            let _e2616 = phi_22405_;
                                                                            let _e2618 = phi_22406_;
                                                                            if _e2618 {
                                                                                phi_22410_ = _e2614;
                                                                            } else {
                                                                                phi_22410_ = _e2616;
                                                                            }
                                                                            let _e2620 = phi_22410_;
                                                                            if (bitcast<i32>(_e2620.member) != 0i) {
                                                                                phi_22344_ = select(_e2620.member_1.member_1, _e2620.member_1.member, (_e2620.member_1.member >= 0f));
                                                                            } else {
                                                                                phi_22344_ = -1f;
                                                                            }
                                                                            let _e2631 = phi_22344_;
                                                                            if (_e2631 < 0f) {
                                                                                phi_22213_ = type_56(0u, type_56().member_1);
                                                                            } else {
                                                                                phi_22171_ = vec3<f32>(0f, 0f, 0f);
                                                                                phi_22172_ = 0u;
                                                                                loop {
                                                                                    let _e2634 = phi_22171_;
                                                                                    let _e2636 = phi_22172_;
                                                                                    local_11 = _e2634;
                                                                                    let _e2637 = (_e2636 < 16u);
                                                                                    if _e2637 {
                                                                                        let _e2639 = (f32(_e2636) * 0.0625f);
                                                                                        let _e2640 = (_e2636 + 1u);
                                                                                        let _e2642 = (f32(_e2640) * 0.0625f);
                                                                                        let _e2644 = ((_e2639 + _e2642) * 0.5f);
                                                                                        let _e2646 = ((_e2631 * _e2644) * _e2644);
                                                                                        let _e2650 = (_e2631 * ((_e2642 * _e2642) - (_e2639 * _e2639)));
                                                                                        let _e2655 = (vec3<f32>(_e2315, _e2316, _e2317) + vec3<f32>((_e1462 * _e2646), (_e1465 * _e2646), (_e1468 * _e2646)));
                                                                                        let _e2672 = (_e2655.x - _e1441);
                                                                                        let _e2673 = (_e2655.y - _e1444);
                                                                                        let _e2674 = (_e2655.z - _e1447);
                                                                                        let _e2682 = ((((((_e2655.x * _e2655.x) + (_e2655.y * _e2655.y)) + (_e2655.z * _e2655.z)) - (2f * (((_e2655.x * _e1441) + (_e2655.y * _e1444)) + (_e2655.z * _e1447)))) + _e1456) / (sqrt((((_e2672 * _e2672) + (_e2673 * _e2673)) + (_e2674 * _e2674))) + _e1450));
                                                                                        let _e2686 = (1f - (abs((_e2682 - 25000f)) * 0.00006666667f));
                                                                                        if (_e2686 != _e2686) {
                                                                                            phi_22542_ = true;
                                                                                        } else {
                                                                                            phi_22542_ = (0f >= _e2686);
                                                                                        }
                                                                                        let _e2690 = phi_22542_;
                                                                                        let _e2692 = -(_e2682);
                                                                                        phi_22205_ = (_e2634 + vec3<f32>((exp((_e2692 / _e1471)) * _e2650), (exp((_e2692 / _e1474)) * _e2650), (select(_e2686, 0f, _e2690) * _e2650)));
                                                                                        phi_22206_ = _e2640;
                                                                                    } else {
                                                                                        phi_22205_ = vec3<f32>();
                                                                                        phi_22206_ = u32();
                                                                                    }
                                                                                    let _e2703 = phi_22205_;
                                                                                    let _e2705 = phi_22206_;
                                                                                    phi_22171_ = _e2703;
                                                                                    phi_22172_ = _e2705;
                                                                                    if !(_e2637) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2708 = local_11;
                                                                                phi_22213_ = type_56(1u, _e2708);
                                                                            }
                                                                            let _e2714 = phi_22213_;
                                                                            phi_22217_ = _e2714;
                                                                        }
                                                                        let _e2719 = phi_22217_;
                                                                        if (bitcast<i32>(_e2719.member) != 0i) {
                                                                            let _e2724 = (vec3<f32>(_e2364, _e2366, _e2368) + _e2719.member_1);
                                                                            let _e2730 = (_e2373 * _e2724.y);
                                                                            let _e2742 = exp(-((((_e1480 * _e2724.x) + _e2730) + (_e1496 * _e2724.z))));
                                                                            let _e2744 = exp(-((((_e1483 * _e2724.x) + _e2730) + (_e1499 * _e2724.z))));
                                                                            let _e2746 = exp(-((((_e1486 * _e2724.x) + _e2730) + (_e1502 * _e2724.z))));
                                                                            phi_3183_ = vec3<f32>((_e2204.x + (_e2742 * _e2358)), (_e2204.y + (_e2744 * _e2358)), (_e2204.z + (_e2746 * _e2358)));
                                                                            phi_3184_ = vec3<f32>((_e2206.x + (_e2742 * _e2355)), (_e2206.y + (_e2744 * _e2355)), (_e2206.z + (_e2746 * _e2355)));
                                                                        } else {
                                                                            phi_3183_ = _e2204;
                                                                            phi_3184_ = _e2206;
                                                                        }
                                                                        let _e2768 = phi_3183_;
                                                                        let _e2770 = phi_3184_;
                                                                        phi_2687_ = vec3<f32>((_e2364 + _e2360), (_e2366 + _e2361), (_e2368 + _e2362));
                                                                        phi_2690_ = vec3<f32>((_e2202.x + ((exp(-((((_e1480 * _e2364) + _e2374) + (_e1496 * _e2368)))) * ((_e1480 * _e2355) + _e2393)) * _e2522.x)), (_e2202.y + ((exp(-((((_e1483 * _e2364) + _e2374) + (_e1499 * _e2368)))) * ((_e1483 * _e2355) + _e2393)) * _e2522.y)), (_e2202.z + ((exp(-((((_e1486 * _e2364) + _e2374) + (_e1502 * _e2368)))) * ((_e1486 * _e2355) + _e2393)) * _e2522.z)));
                                                                        phi_2692_ = _e2768;
                                                                        phi_2694_ = _e2770;
                                                                        phi_2696_ = (_e2208 + 1u);
                                                                    } else {
                                                                        phi_2687_ = vec3<f32>();
                                                                        phi_2690_ = vec3<f32>();
                                                                        phi_2692_ = vec3<f32>();
                                                                        phi_2694_ = vec3<f32>();
                                                                        phi_2696_ = u32();
                                                                    }
                                                                    let _e2777 = phi_2687_;
                                                                    let _e2779 = phi_2690_;
                                                                    let _e2781 = phi_2692_;
                                                                    let _e2783 = phi_2694_;
                                                                    let _e2785 = phi_2696_;
                                                                    phi_2686_ = _e2777;
                                                                    phi_2689_ = _e2779;
                                                                    phi_2691_ = _e2781;
                                                                    phi_2693_ = _e2783;
                                                                    phi_2695_ = _e2785;
                                                                    phi_31615_ = _e1431;
                                                                    if !(_e2209) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e2788 = phi_31615_;
                                                                phi_31705_ = _e2788;
                                                                if _e2788 {
                                                                    break;
                                                                }
                                                                let _e2790 = local_12;
                                                                let _e2793 = local_13;
                                                                let _e2796 = local_14;
                                                                phi_31709_ = _e2788;
                                                                phi_3204_ = type_51((_e2790 * vec3<f32>(_e1480, _e1483, _e1486)), (_e2793 * _e1490), _e2796);
                                                            } else {
                                                                phi_31709_ = _e1431;
                                                                phi_3204_ = type_51(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                            }
                                                            let _e2799 = phi_31709_;
                                                            let _e2801 = phi_3204_;
                                                            phi_31708_ = _e2799;
                                                            phi_3205_ = _e2801;
                                                        } else {
                                                            phi_31708_ = _e1431;
                                                            phi_3205_ = _e2034;
                                                        }
                                                        let _e2803 = phi_31708_;
                                                        let _e2805 = phi_3205_;
                                                        let _e2813 = (((_e322.member_1.x * _e1462) + (_e322.member_1.y * _e1465)) + (_e322.member_1.z * _e1468));
                                                        let _e2815 = (1f + (_e2813 * _e2813));
                                                        let _e2816 = (0.059683103f * _e2815);
                                                        let _e2823 = (_e1493 * _e1493);
                                                        let _e2834 = ((0.119366206f * ((1f - _e2823) * _e2815)) / ((2f + _e2823) * pow(((1f + _e2823) - ((2f * _e1493) * _e2813)), 1.5f)));
                                                        phi_31707_ = _e2803;
                                                        phi_3228_ = vec3<f32>((_e1433.x + (((((_e2805.member.x * _e2816) + (_e2805.member_1.x * _e2834)) + _e2805.member_2.x) * _e1477) * 0.38317f)), (_e1433.y + (((((_e2805.member.y * _e2816) + (_e2805.member_1.y * _e2834)) + _e2805.member_2.y) * _e1477) * 0.38317f)), (_e1433.z + (((((_e2805.member.z * _e2816) + (_e2805.member_1.z * _e2834)) + _e2805.member_2.z) * _e1477) * 0.38317f)));
                                                        phi_3229_ = (_e1435 + 1i);
                                                        phi_3230_ = bitcast<u32>(_e1513);
                                                    } else {
                                                        phi_31707_ = _e1431;
                                                        phi_3228_ = vec3<f32>();
                                                        phi_3229_ = i32();
                                                        phi_3230_ = u32();
                                                    }
                                                    let _e2865 = phi_31707_;
                                                    let _e2867 = phi_3228_;
                                                    let _e2869 = phi_3229_;
                                                    let _e2871 = phi_3230_;
                                                    phi_31706_ = _e2865;
                                                    phi_874_ = _e2867;
                                                    phi_877_ = _e2869;
                                                    phi_879_ = _e2871;
                                                    phi_3231_ = _e1439;
                                                } else {
                                                    phi_31706_ = _e1431;
                                                    phi_874_ = vec3<f32>();
                                                    phi_877_ = i32();
                                                    phi_879_ = u32();
                                                    phi_3231_ = false;
                                                }
                                                let _e2873 = phi_31706_;
                                                let _e2875 = phi_874_;
                                                let _e2877 = phi_877_;
                                                let _e2879 = phi_879_;
                                                let _e2881 = phi_3231_;
                                                phi_31647_ = _e2873;
                                                phi_873_ = _e2875;
                                                phi_876_ = _e2877;
                                                phi_878_ = _e2879;
                                                phi_31705_ = _e2873;
                                                if !(_e2881) {
                                                    break;
                                                }
                                            }
                                            let _e2884 = phi_31705_;
                                            phi_32033_ = _e2884;
                                            if _e2884 {
                                                break;
                                            }
                                            if _e1428 {
                                                let _e2887 = global_6.member[(_e1280 + 10u)];
                                                let _e2890 = global_6.member[(_e1280 + 21u)];
                                                let _e2891 = bitcast<u32>(_e2890);
                                                let _e2894 = global_6.member[(_e1280 + 22u)];
                                                let _e2895 = bitcast<u32>(_e2894);
                                                let _e2898 = global_6.member[(_e1280 + 23u)];
                                                let _e2902 = global_6.member[(_e1280 + 24u)];
                                                let _e2906 = global_6.member[(_e1280 + 25u)];
                                                let _e2910 = global_6.member[(_e1280 + 26u)];
                                                let _e2913 = global_6.member[_e1280];
                                                let _e2916 = global_6.member[(_e1280 + 1u)];
                                                let _e2919 = global_6.member[(_e1280 + 2u)];
                                                let _e2922 = global_6.member[(_e1280 + 3u)];
                                                let _e2925 = global_6.member[(_e1280 + 4u)];
                                                let _e2928 = global_6.member[(_e1280 + 5u)];
                                                let _e2931 = global_6.member[(_e1280 + 6u)];
                                                let _e2934 = global_6.member[(_e1280 + 7u)];
                                                let _e2937 = global_6.member[(_e1280 + 8u)];
                                                let _e2942 = (((_e322.member_1.x * _e2922) + (_e322.member_1.y * _e2925)) + (_e322.member_1.z * _e2928));
                                                let _e2944 = select(_e2942, -1f, (_e2942 < -1f));
                                                let _e2947 = asin(select(_e2944, 1f, (_e2944 > 1f)));
                                                let _e2958 = atan2((((_e322.member_1.x * _e2931) + (_e322.member_1.y * _e2934)) + (_e322.member_1.z * _e2937)), (((_e322.member_1.x * _e2913) + (_e322.member_1.y * _e2916)) + (_e322.member_1.z * _e2919)));
                                                if (_e2891 != 4294967295u) {
                                                    let _e2961 = global_6.member[_e2891];
                                                    let _e2964 = global_6.member[(_e2891 + 1u)];
                                                    let _e2965 = bitcast<u32>(_e2964);
                                                    let _e2968 = global_6.member[(_e2891 + 4u)];
                                                    phi_3565_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_3568_ = 0u;
                                                    loop {
                                                        let _e2974 = phi_3565_;
                                                        let _e2976 = phi_3568_;
                                                        local_15 = _e2974;
                                                        let _e2977 = (_e2976 < 6u);
                                                        if _e2977 {
                                                            let _e2978 = (_e2976 / 2u);
                                                            if (_e2978 == 0u) {
                                                                phi_22670_ = _e322.member_1.x;
                                                            } else {
                                                                phi_22670_ = select(_e322.member_1.z, _e322.member_1.y, (_e2978 == 1u));
                                                            }
                                                            let _e2986 = phi_22670_;
                                                            let _e2987 = (_e2986 * select(-1f, 1f, ((_e2976 % 2u) == 0u)));
                                                            if (_e2987 > 0f) {
                                                                let _e2991 = ((_e2978 + 1u) % 3u);
                                                                if (_e2991 == 0u) {
                                                                    phi_22685_ = _e322.member_1.x;
                                                                } else {
                                                                    phi_22685_ = select(_e322.member_1.z, _e322.member_1.y, (_e2991 == 1u));
                                                                }
                                                                let _e2996 = phi_22685_;
                                                                let _e2997 = (_e2996 / _e2987);
                                                                let _e2999 = ((_e2978 + 2u) % 3u);
                                                                if (_e2999 == 0u) {
                                                                    phi_22700_ = _e322.member_1.x;
                                                                } else {
                                                                    phi_22700_ = select(_e322.member_1.z, _e322.member_1.y, (_e2999 == 1u));
                                                                }
                                                                let _e3004 = phi_22700_;
                                                                let _e3005 = (_e3004 / _e2987);
                                                                let _e3007 = (1f + (_e2961 * 12f));
                                                                if (abs(_e2997) <= _e3007) {
                                                                    if (abs(_e3005) <= _e3007) {
                                                                        let _e3013 = f32(_e2965);
                                                                        let _e3015 = floor((((_e2997 + 1f) * 0.5f) * _e3013));
                                                                        let _e3022 = select(0i, select(select(i32(_e3015), i32(-2147483648), (_e3015 < -2147483600f)), 2147483647i, (_e3015 > 2147483500f)), (_e3015 == _e3015));
                                                                        let _e3024 = (bitcast<i32>(_e2965) - 1i);
                                                                        let _e3025 = (0i <= _e3024);
                                                                        if _e3025 {
                                                                        } else {
                                                                            phi_31710_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3022 < 0i) {
                                                                            phi_3639_ = 0i;
                                                                        } else {
                                                                            phi_3639_ = select(_e3022, _e3024, (_e3022 > _e3024));
                                                                        }
                                                                        let _e3030 = phi_3639_;
                                                                        let _e3034 = floor((((_e3005 + 1f) * 0.5f) * _e3013));
                                                                        let _e3041 = select(0i, select(select(i32(_e3034), i32(-2147483648), (_e3034 < -2147483600f)), 2147483647i, (_e3034 > 2147483500f)), (_e3034 == _e3034));
                                                                        if _e3025 {
                                                                        } else {
                                                                            phi_31710_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3041 < 0i) {
                                                                            phi_3669_ = 0i;
                                                                        } else {
                                                                            phi_3669_ = select(_e3041, _e3024, (_e3041 > _e3024));
                                                                        }
                                                                        let _e3046 = phi_3669_;
                                                                        let _e3047 = (_e3030 - 1i);
                                                                        phi_3677_ = _e2974;
                                                                        phi_3680_ = select(0i, _e3047, (0i < _e3047));
                                                                        loop {
                                                                            let _e3051 = phi_3677_;
                                                                            let _e3053 = phi_3680_;
                                                                            let _e3054 = (_e3030 + 1i);
                                                                            local_59 = _e3051;
                                                                            let _e3057 = (_e3053 <= select(_e3054, _e3024, (_e3024 < _e3054)));
                                                                            if _e3057 {
                                                                                let _e3058 = (_e3046 - 1i);
                                                                                phi_3703_ = _e3051;
                                                                                phi_3706_ = select(0i, _e3058, (0i < _e3058));
                                                                                loop {
                                                                                    let _e3062 = phi_3703_;
                                                                                    let _e3064 = phi_3706_;
                                                                                    let _e3065 = (_e3046 + 1i);
                                                                                    local_58 = _e3062;
                                                                                    let _e3068 = (_e3064 <= select(_e3065, _e3024, (_e3024 < _e3065)));
                                                                                    if _e3068 {
                                                                                        let _e3077 = global_6.member[(bitcast<u32>(_e2968) + ((((_e2976 * _e2965) + bitcast<u32>(_e3053)) * _e2965) + bitcast<u32>(_e3064)))];
                                                                                        phi_3704_ = vec3<f32>((_e3062.x + _e3077), vec3<f32>().y, vec3<f32>().z);
                                                                                        phi_3707_ = (_e3064 + 1i);
                                                                                    } else {
                                                                                        phi_3704_ = vec3<f32>();
                                                                                        phi_3707_ = i32();
                                                                                    }
                                                                                    let _e3086 = phi_3704_;
                                                                                    let _e3088 = phi_3707_;
                                                                                    phi_3703_ = _e3086;
                                                                                    phi_3706_ = _e3088;
                                                                                    if !(_e3068) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e6917 = local_58;
                                                                                phi_3678_ = _e6917;
                                                                                phi_3681_ = (_e3053 + 1i);
                                                                            } else {
                                                                                phi_3678_ = vec3<f32>();
                                                                                phi_3681_ = i32();
                                                                            }
                                                                            let _e3092 = phi_3678_;
                                                                            let _e3094 = phi_3681_;
                                                                            phi_3677_ = _e3092;
                                                                            phi_3680_ = _e3094;
                                                                            if !(_e3057) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e6921 = local_59;
                                                                        phi_3905_ = _e6921;
                                                                    } else {
                                                                        phi_3905_ = _e2974;
                                                                    }
                                                                    let _e3097 = phi_3905_;
                                                                    phi_3906_ = _e3097;
                                                                } else {
                                                                    phi_3906_ = _e2974;
                                                                }
                                                                let _e3099 = phi_3906_;
                                                                phi_3907_ = _e3099;
                                                            } else {
                                                                phi_3907_ = _e2974;
                                                            }
                                                            let _e3101 = phi_3907_;
                                                            phi_3566_ = _e3101;
                                                            phi_3569_ = (_e2976 + 1u);
                                                        } else {
                                                            phi_3566_ = vec3<f32>();
                                                            phi_3569_ = u32();
                                                        }
                                                        let _e3104 = phi_3566_;
                                                        let _e3106 = phi_3569_;
                                                        phi_3565_ = _e3104;
                                                        phi_3568_ = _e3106;
                                                        phi_31710_ = _e2884;
                                                        if !(_e2977) {
                                                            break;
                                                        }
                                                    }
                                                    let _e3109 = phi_31710_;
                                                    phi_32033_ = _e3109;
                                                    if _e3109 {
                                                        break;
                                                    }
                                                    let _e3111 = local_15;
                                                    phi_31758_ = _e3109;
                                                    phi_3918_ = (_e3111.x * (1f / ((6.2831855f * _e2961) * _e2961)));
                                                } else {
                                                    phi_31758_ = _e2884;
                                                    phi_3918_ = 0f;
                                                }
                                                let _e3115 = phi_31758_;
                                                let _e3117 = phi_3918_;
                                                if (_e2895 != 4294967295u) {
                                                    let _e3134 = (_e2947 * 1.3333334f);
                                                    let _e3141 = global_6.member[_e2895];
                                                    phi_3942_ = (_e3117 + ((_e3141 * ((exp((-(abs(_e2947)) / (0.06f + (abs(_e2958) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2958 * -0.71428573f) * (_e2958 * 0.71428573f)))))) + (1.3f * exp((((_e2958 * _e2958) + (_e3134 * _e3134)) * -12.499999f))))) * 4.73f));
                                                } else {
                                                    phi_3942_ = _e3117;
                                                }
                                                let _e3147 = phi_3942_;
                                                phi_3944_ = 0u;
                                                phi_3947_ = _e3147;
                                                loop {
                                                    let _e3149 = phi_3944_;
                                                    let _e3151 = phi_3947_;
                                                    local_60 = _e3151;
                                                    let _e3152 = (_e3149 < bitcast<u32>(_e2902));
                                                    if _e3152 {
                                                        let _e3154 = (bitcast<u32>(_e2898) + (_e3149 * 18u));
                                                        let _e3157 = global_6.member[(_e3154 + 1u)];
                                                        let _e3160 = global_6.member[(_e3154 + 2u)];
                                                        let _e3163 = global_6.member[(_e3154 + 3u)];
                                                        let _e3171 = global_6.member[(_e3154 + 5u)];
                                                        if ((((_e322.member_1.x * _e3157) + (_e322.member_1.y * _e3160)) + (_e322.member_1.z * _e3163)) >= _e3171) {
                                                            let _e3174 = global_6.member[_e3154];
                                                            let _e3179 = select(select(u32(_e3174), 0u, (_e3174 < 0f)), 4294967295u, (_e3174 > 4294967000f));
                                                            if (_e3179 == 0u) {
                                                                phi_4051_ = 1.8f;
                                                            } else {
                                                                if (_e3179 == 1u) {
                                                                    phi_4050_ = 1f;
                                                                } else {
                                                                    phi_4050_ = select(0f, 4.7f, (_e3179 == 2u));
                                                                }
                                                                let _e3185 = phi_4050_;
                                                                phi_4051_ = _e3185;
                                                            }
                                                            let _e3187 = phi_4051_;
                                                            let _e3190 = global_6.member[(_e3154 + 6u)];
                                                            phi_4067_ = (_e3151 + (_e3190 * _e3187));
                                                        } else {
                                                            phi_4067_ = _e3151;
                                                        }
                                                        let _e3194 = phi_4067_;
                                                        phi_3945_ = (_e3149 + 1u);
                                                        phi_3948_ = _e3194;
                                                    } else {
                                                        phi_3945_ = u32();
                                                        phi_3948_ = f32();
                                                    }
                                                    let _e3197 = phi_3945_;
                                                    let _e3199 = phi_3948_;
                                                    phi_3944_ = _e3197;
                                                    phi_3947_ = _e3199;
                                                    if !(_e3152) {
                                                        break;
                                                    }
                                                }
                                                phi_4070_ = 0u;
                                                let _e6941 = local_60;
                                                phi_4073_ = _e6941;
                                                loop {
                                                    let _e3202 = phi_4070_;
                                                    let _e3204 = phi_4073_;
                                                    local_16 = _e3204;
                                                    let _e3205 = (_e3202 < bitcast<u32>(_e2910));
                                                    if _e3205 {
                                                        let _e3207 = (bitcast<u32>(_e2906) + (_e3202 * 29u));
                                                        let _e3210 = global_6.member[(_e3207 + 1u)];
                                                        let _e3213 = global_6.member[(_e3207 + 2u)];
                                                        let _e3216 = global_6.member[(_e3207 + 3u)];
                                                        let _e3224 = global_6.member[(_e3207 + 5u)];
                                                        if ((((_e322.member_1.x * _e3210) + (_e322.member_1.y * _e3213)) + (_e322.member_1.z * _e3216)) >= _e3224) {
                                                            let _e3227 = global_6.member[_e3207];
                                                            let _e3231 = global_6.member[(_e3207 + 6u)];
                                                            phi_4181_ = (_e3204 + (_e3231 * select(1f, 3.3808f, (_e3227 < 0.5f))));
                                                        } else {
                                                            phi_4181_ = _e3204;
                                                        }
                                                        let _e3236 = phi_4181_;
                                                        phi_4071_ = (_e3202 + 1u);
                                                        phi_4074_ = _e3236;
                                                    } else {
                                                        phi_4071_ = u32();
                                                        phi_4074_ = f32();
                                                    }
                                                    let _e3239 = phi_4071_;
                                                    let _e3241 = phi_4074_;
                                                    phi_4070_ = _e3239;
                                                    phi_4073_ = _e3241;
                                                    if !(_e3205) {
                                                        break;
                                                    }
                                                }
                                                let _e3245 = global_6.member[(_e1280 + 9u)];
                                                let _e3247 = local_16;
                                                let _e3250 = local_17;
                                                let _e3254 = local_18;
                                                let _e3259 = local_19;
                                                let _e3264 = ((_e3247 * _e3245) <= (_e2887 * (((0.2126f * _e3250.x) + (0.7152f * _e3254.y)) + (0.0722f * _e3259.z))));
                                                if _e3264 {
                                                    phi_31799_ = _e3115;
                                                    phi_4740_ = vec3<f32>();
                                                    phi_4741_ = bool();
                                                } else {
                                                    let _e3266 = global_6.member[_e1277];
                                                    let _e3269 = global_6.member[(_e1277 + 1u)];
                                                    let _e3272 = global_6.member[(_e1277 + 2u)];
                                                    let _e3275 = global_6.member[(_e1277 + 3u)];
                                                    let _e3278 = global_6.member[(_e1277 + 5u)];
                                                    let _e3281 = global_6.member[(_e1277 + 6u)];
                                                    let _e3284 = global_6.member[(_e1277 + 10u)];
                                                    let _e3287 = global_6.member[(_e1277 + 11u)];
                                                    let _e3290 = global_6.member[(_e1277 + 13u)];
                                                    let _e3293 = global_6.member[(_e1277 + 14u)];
                                                    let _e3296 = global_6.member[(_e1277 + 15u)];
                                                    let _e3299 = global_6.member[(_e1277 + 16u)];
                                                    let _e3302 = global_6.member[(_e1277 + 18u)];
                                                    let _e3305 = global_6.member[(_e1277 + 19u)];
                                                    let _e3308 = global_6.member[(_e1277 + 20u)];
                                                    let _e3320 = ((((_e322.member.x * _e322.member.x) + (_e322.member.y * _e322.member.y)) + (_e322.member.z * _e322.member.z)) - (2f * (((_e322.member.x * _e3266) + (_e322.member.y * _e3269)) + (_e322.member.z * _e3272))));
                                                    let _e3321 = (_e3320 + _e3278);
                                                    let _e3322 = (_e3321 > 0f);
                                                    if _e3322 {
                                                        let _e3330 = ((((_e322.member.x - _e3266) * _e322.member_1.x) + ((_e322.member.y - _e3269) * _e322.member_1.y)) + ((_e322.member.z - _e3272) * _e322.member_1.z));
                                                        let _e3332 = ((_e3330 * _e3330) - _e3321);
                                                        if (_e3332 < 0f) {
                                                            phi_22844_ = type_55(0u, type_55().member_1);
                                                            phi_22845_ = type_55();
                                                            phi_22846_ = true;
                                                        } else {
                                                            if (_e3330 < 0f) {
                                                                phi_22816_ = -(sqrt(_e3332));
                                                            } else {
                                                                phi_22816_ = sqrt(_e3332);
                                                            }
                                                            let _e3339 = phi_22816_;
                                                            let _e3341 = -((_e3330 + _e3339));
                                                            let _e3342 = (_e3341 == 0f);
                                                            if _e3342 {
                                                                phi_22839_ = type_55(1u, type_54(0f, 0f));
                                                                phi_22840_ = type_55();
                                                            } else {
                                                                let _e3343 = (_e3321 / _e3341);
                                                                if (_e3341 < _e3343) {
                                                                    phi_22828_ = type_54(_e3341, _e3343);
                                                                } else {
                                                                    phi_22828_ = type_54(_e3343, _e3341);
                                                                }
                                                                let _e3348 = phi_22828_;
                                                                phi_22839_ = type_55();
                                                                phi_22840_ = type_55(1u, _e3348);
                                                            }
                                                            let _e3351 = phi_22839_;
                                                            let _e3353 = phi_22840_;
                                                            phi_22844_ = _e3351;
                                                            phi_22845_ = _e3353;
                                                            phi_22846_ = _e3342;
                                                        }
                                                        let _e3358 = phi_22844_;
                                                        let _e3360 = phi_22845_;
                                                        let _e3362 = phi_22846_;
                                                        if _e3362 {
                                                            phi_22850_ = _e3358;
                                                        } else {
                                                            phi_22850_ = _e3360;
                                                        }
                                                        let _e3364 = phi_22850_;
                                                        if (bitcast<i32>(_e3364.member) != 0i) {
                                                            phi_22784_ = select(_e3364.member_1.member_1, _e3364.member_1.member, (_e3364.member_1.member >= 0f));
                                                        } else {
                                                            phi_22784_ = -1f;
                                                        }
                                                        let _e3375 = phi_22784_;
                                                        phi_4582_ = select(true, false, (_e3375 >= 0f));
                                                    } else {
                                                        phi_4582_ = true;
                                                    }
                                                    let _e3379 = phi_4582_;
                                                    if _e3379 {
                                                        let _e3387 = ((((_e322.member.x - _e3266) * _e322.member_1.x) + ((_e322.member.y - _e3269) * _e322.member_1.y)) + ((_e322.member.z - _e3272) * _e322.member_1.z));
                                                        let _e3388 = (_e3320 + _e3281);
                                                        let _e3389 = (_e3387 * _e3387);
                                                        let _e3390 = (_e3389 - _e3388);
                                                        if (_e3390 < 0f) {
                                                            phi_23004_ = type_55(0u, type_55().member_1);
                                                            phi_23005_ = type_55();
                                                            phi_23006_ = true;
                                                        } else {
                                                            if (_e3387 < 0f) {
                                                                phi_22976_ = -(sqrt(_e3390));
                                                            } else {
                                                                phi_22976_ = sqrt(_e3390);
                                                            }
                                                            let _e3397 = phi_22976_;
                                                            let _e3399 = -((_e3387 + _e3397));
                                                            let _e3400 = (_e3399 == 0f);
                                                            if _e3400 {
                                                                phi_22999_ = type_55(1u, type_54(0f, 0f));
                                                                phi_23000_ = type_55();
                                                            } else {
                                                                let _e3401 = (_e3388 / _e3399);
                                                                if (_e3399 < _e3401) {
                                                                    phi_22988_ = type_54(_e3399, _e3401);
                                                                } else {
                                                                    phi_22988_ = type_54(_e3401, _e3399);
                                                                }
                                                                let _e3406 = phi_22988_;
                                                                phi_22999_ = type_55();
                                                                phi_23000_ = type_55(1u, _e3406);
                                                            }
                                                            let _e3409 = phi_22999_;
                                                            let _e3411 = phi_23000_;
                                                            phi_23004_ = _e3409;
                                                            phi_23005_ = _e3411;
                                                            phi_23006_ = _e3400;
                                                        }
                                                        let _e3416 = phi_23004_;
                                                        let _e3418 = phi_23005_;
                                                        let _e3420 = phi_23006_;
                                                        if _e3420 {
                                                            phi_23010_ = _e3416;
                                                        } else {
                                                            phi_23010_ = _e3418;
                                                        }
                                                        let _e3422 = phi_23010_;
                                                        if (bitcast<i32>(_e3422.member) != 0i) {
                                                            phi_22897_ = type_55(0u, _e3422.member_1);
                                                        } else {
                                                            phi_22897_ = type_55(1u, type_55().member_1);
                                                        }
                                                        let _e3432 = phi_22897_;
                                                        if (bitcast<i32>(_e3432.member) != 0i) {
                                                            phi_22944_ = type_55(0u, type_55().member_1);
                                                        } else {
                                                            if (_e3432.member_1.member_1 <= 0f) {
                                                                phi_22940_ = type_55(0u, type_55().member_1);
                                                            } else {
                                                                if (_e3432.member_1.member != _e3432.member_1.member) {
                                                                    phi_23048_ = true;
                                                                } else {
                                                                    phi_23048_ = (0f >= _e3432.member_1.member);
                                                                }
                                                                let _e3444 = phi_23048_;
                                                                let _e3445 = select(_e3432.member_1.member, 0f, _e3444);
                                                                if _e3322 {
                                                                    let _e3446 = (_e3389 - _e3321);
                                                                    if (_e3446 < 0f) {
                                                                        phi_23156_ = type_55(0u, type_55().member_1);
                                                                        phi_23157_ = type_55();
                                                                        phi_23158_ = true;
                                                                    } else {
                                                                        if (_e3387 < 0f) {
                                                                            phi_23128_ = -(sqrt(_e3446));
                                                                        } else {
                                                                            phi_23128_ = sqrt(_e3446);
                                                                        }
                                                                        let _e3453 = phi_23128_;
                                                                        let _e3455 = -((_e3387 + _e3453));
                                                                        let _e3456 = (_e3455 == 0f);
                                                                        if _e3456 {
                                                                            phi_23151_ = type_55(1u, type_54(0f, 0f));
                                                                            phi_23152_ = type_55();
                                                                        } else {
                                                                            let _e3457 = (_e3321 / _e3455);
                                                                            if (_e3455 < _e3457) {
                                                                                phi_23140_ = type_54(_e3455, _e3457);
                                                                            } else {
                                                                                phi_23140_ = type_54(_e3457, _e3455);
                                                                            }
                                                                            let _e3462 = phi_23140_;
                                                                            phi_23151_ = type_55();
                                                                            phi_23152_ = type_55(1u, _e3462);
                                                                        }
                                                                        let _e3465 = phi_23151_;
                                                                        let _e3467 = phi_23152_;
                                                                        phi_23156_ = _e3465;
                                                                        phi_23157_ = _e3467;
                                                                        phi_23158_ = _e3456;
                                                                    }
                                                                    let _e3472 = phi_23156_;
                                                                    let _e3474 = phi_23157_;
                                                                    let _e3476 = phi_23158_;
                                                                    if _e3476 {
                                                                        phi_23162_ = _e3472;
                                                                    } else {
                                                                        phi_23162_ = _e3474;
                                                                    }
                                                                    let _e3478 = phi_23162_;
                                                                    if (bitcast<i32>(_e3478.member) != 0i) {
                                                                        phi_23096_ = select(_e3478.member_1.member_1, _e3478.member_1.member, (_e3478.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23096_ = -1f;
                                                                    }
                                                                    let _e3489 = phi_23096_;
                                                                    if (_e3489 >= 0f) {
                                                                        if (_e3432.member_1.member_1 != _e3432.member_1.member_1) {
                                                                            phi_23200_ = true;
                                                                        } else {
                                                                            phi_23200_ = (_e3489 <= _e3432.member_1.member_1);
                                                                        }
                                                                        let _e3494 = phi_23200_;
                                                                        phi_22925_ = select(_e3432.member_1.member_1, _e3489, _e3494);
                                                                    } else {
                                                                        phi_22925_ = _e3432.member_1.member_1;
                                                                    }
                                                                    let _e3497 = phi_22925_;
                                                                    phi_22927_ = _e3497;
                                                                } else {
                                                                    phi_22927_ = _e3432.member_1.member_1;
                                                                }
                                                                let _e3499 = phi_22927_;
                                                                if (_e3499 <= _e3445) {
                                                                    phi_22936_ = type_55(0u, type_55().member_1);
                                                                } else {
                                                                    phi_22936_ = type_55(1u, type_54(_e3445, _e3499));
                                                                }
                                                                let _e3507 = phi_22936_;
                                                                phi_22940_ = _e3507;
                                                            }
                                                            let _e3512 = phi_22940_;
                                                            phi_22944_ = _e3512;
                                                        }
                                                        let _e3517 = phi_22944_;
                                                        let _e3520 = (bitcast<i32>(_e3517.member) != 0i);
                                                        if _e3520 {
                                                            phi_4596_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_4599_ = 0u;
                                                            loop {
                                                                let _e3526 = phi_4596_;
                                                                let _e3528 = phi_4599_;
                                                                local_20 = _e3526;
                                                                local_21 = _e3526;
                                                                local_22 = _e3526;
                                                                let _e3529 = (_e3528 < 32u);
                                                                if _e3529 {
                                                                    let _e3530 = -(_e3387);
                                                                    if (_e3517.member_1.member <= _e3517.member_1.member_1) {
                                                                    } else {
                                                                        phi_31737_ = true;
                                                                        break;
                                                                    }
                                                                    let _e3533 = select(_e3530, _e3517.member_1.member, (_e3530 < _e3517.member_1.member));
                                                                    let _e3535 = select(_e3533, _e3517.member_1.member_1, (_e3533 > _e3517.member_1.member_1));
                                                                    let _e3536 = (_e3535 - _e3517.member_1.member);
                                                                    let _e3537 = (_e3517.member_1.member_1 - _e3535);
                                                                    let _e3538 = (_e3517.member_1.member_1 - _e3517.member_1.member);
                                                                    let _e3539 = (0.000000001f * _e3538);
                                                                    if (_e3536 <= _e3539) {
                                                                        let _e3614 = (f32(_e3528) * 0.03125f);
                                                                        let _e3617 = (f32((_e3528 + 1u)) * 0.03125f);
                                                                        let _e3619 = ((_e3614 + _e3617) * 0.5f);
                                                                        phi_4700_ = type_54((_e3517.member_1.member + ((_e3538 * _e3619) * _e3619)), (_e3538 * ((_e3617 * _e3617) - (_e3614 * _e3614))));
                                                                    } else {
                                                                        if (_e3537 <= _e3539) {
                                                                            let _e3597 = (f32((31u - _e3528)) * 0.03125f);
                                                                            let _e3600 = (f32((32u - _e3528)) * 0.03125f);
                                                                            let _e3602 = ((_e3597 + _e3600) * 0.5f);
                                                                            phi_4699_ = type_54((_e3517.member_1.member_1 - ((_e3538 * _e3602) * _e3602)), (_e3538 * ((_e3600 * _e3600) - (_e3597 * _e3597))));
                                                                        } else {
                                                                            let _e3545 = round(((32f * _e3536) / (_e3536 + _e3537)));
                                                                            let _e3550 = select(select(u32(_e3545), 0u, (_e3545 < 0f)), 4294967295u, (_e3545 > 4294967000f));
                                                                            if (_e3550 < 1u) {
                                                                                phi_4683_ = 1u;
                                                                            } else {
                                                                                phi_4683_ = select(_e3550, 31u, (_e3550 > 31u));
                                                                            }
                                                                            let _e3555 = phi_4683_;
                                                                            if (_e3528 < _e3555) {
                                                                                let _e3576 = ((_e3555 - 1u) - _e3528);
                                                                                let _e3578 = f32(_e3555);
                                                                                let _e3579 = (f32(_e3576) / _e3578);
                                                                                let _e3582 = (f32((_e3576 + 1u)) / _e3578);
                                                                                let _e3584 = ((_e3579 + _e3582) * 0.5f);
                                                                                phi_4698_ = type_54((_e3535 - ((_e3536 * _e3584) * _e3584)), (_e3536 * ((_e3582 * _e3582) - (_e3579 * _e3579))));
                                                                            } else {
                                                                                let _e3558 = (_e3528 - _e3555);
                                                                                let _e3560 = f32((32u - _e3555));
                                                                                let _e3561 = (f32(_e3558) / _e3560);
                                                                                let _e3564 = (f32((_e3558 + 1u)) / _e3560);
                                                                                let _e3566 = ((_e3561 + _e3564) * 0.5f);
                                                                                phi_4698_ = type_54((_e3535 + ((_e3537 * _e3566) * _e3566)), (_e3537 * ((_e3564 * _e3564) - (_e3561 * _e3561))));
                                                                            }
                                                                            let _e3594 = phi_4698_;
                                                                            phi_4699_ = _e3594;
                                                                        }
                                                                        let _e3612 = phi_4699_;
                                                                        phi_4700_ = _e3612;
                                                                    }
                                                                    let _e3629 = phi_4700_;
                                                                    let _e3635 = (_e322.member.x + (_e322.member_1.x * _e3629.member));
                                                                    let _e3636 = (_e322.member.y + (_e322.member_1.y * _e3629.member));
                                                                    let _e3637 = (_e322.member.z + (_e322.member_1.z * _e3629.member));
                                                                    let _e3651 = (_e3635 - _e3266);
                                                                    let _e3652 = (_e3636 - _e3269);
                                                                    let _e3653 = (_e3637 - _e3272);
                                                                    let _e3661 = ((((((_e3635 * _e3635) + (_e3636 * _e3636)) + (_e3637 * _e3637)) - (2f * (((_e3635 * _e3266) + (_e3636 * _e3269)) + (_e3637 * _e3272)))) + _e3278) / (sqrt((((_e3651 * _e3651) + (_e3652 * _e3652)) + (_e3653 * _e3653))) + _e3275));
                                                                    let _e3665 = (1f - (abs((_e3661 - 25000f)) * 0.00006666667f));
                                                                    if (_e3665 != _e3665) {
                                                                        phi_23399_ = true;
                                                                    } else {
                                                                        phi_23399_ = (0f >= _e3665);
                                                                    }
                                                                    let _e3669 = phi_23399_;
                                                                    let _e3671 = -(_e3661);
                                                                    phi_4597_ = (_e3526 + vec3<f32>((exp((_e3671 / _e3284)) * _e3629.member_1), (exp((_e3671 / _e3287)) * _e3629.member_1), (select(_e3665, 0f, _e3669) * _e3629.member_1)));
                                                                    phi_4600_ = (_e3528 + 1u);
                                                                } else {
                                                                    phi_4597_ = vec3<f32>();
                                                                    phi_4600_ = u32();
                                                                }
                                                                let _e3683 = phi_4597_;
                                                                let _e3685 = phi_4600_;
                                                                phi_4596_ = _e3683;
                                                                phi_4599_ = _e3685;
                                                                phi_31737_ = _e3115;
                                                                if !(_e3529) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e3688 = phi_31737_;
                                                            phi_32033_ = _e3688;
                                                            if _e3688 {
                                                                break;
                                                            }
                                                            let _e3690 = local_20;
                                                            let _e3697 = local_21;
                                                            let _e3699 = ((_e3299 * 1.1f) * _e3697.y);
                                                            let _e3704 = local_22;
                                                            phi_31804_ = _e3688;
                                                            phi_4725_ = vec3<f32>(exp(-((((_e3290 * _e3690.x) + _e3699) + (_e3302 * _e3704.z)))), exp(-((((_e3293 * _e3690.x) + _e3699) + (_e3305 * _e3704.z)))), exp(-((((_e3296 * _e3690.x) + _e3699) + (_e3308 * _e3704.z)))));
                                                        } else {
                                                            phi_31804_ = _e3115;
                                                            phi_4725_ = vec3<f32>();
                                                        }
                                                        let _e3720 = phi_31804_;
                                                        let _e3722 = phi_4725_;
                                                        phi_31803_ = _e3720;
                                                        phi_4727_ = _e3722;
                                                        phi_4729_ = select(true, false, _e3520);
                                                    } else {
                                                        phi_31803_ = _e3115;
                                                        phi_4727_ = vec3<f32>();
                                                        phi_4729_ = true;
                                                    }
                                                    let _e3725 = phi_31803_;
                                                    let _e3727 = phi_4727_;
                                                    let _e3729 = phi_4729_;
                                                    let _e3733 = select(_e3727, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3379)), vec3(_e3729));
                                                    let _e3737 = select(_e3733.y, _e3733.x, (_e3733.x > _e3733.y));
                                                    phi_31799_ = _e3725;
                                                    phi_4740_ = _e3733;
                                                    phi_4741_ = (select(_e3733.z, _e3737, (_e3737 > _e3733.z)) > 0f);
                                                }
                                                let _e3743 = phi_31799_;
                                                let _e3745 = phi_4740_;
                                                let _e3747 = phi_4741_;
                                                phi_31798_ = _e3743;
                                                phi_4743_ = _e3745;
                                                phi_4744_ = _e3747;
                                                phi_4745_ = select(true, false, _e3264);
                                            } else {
                                                phi_31798_ = _e2884;
                                                phi_4743_ = vec3<f32>(1f, 1f, 1f);
                                                phi_4744_ = _e1428;
                                                phi_4745_ = true;
                                            }
                                            let _e3750 = phi_31798_;
                                            let _e3752 = phi_4743_;
                                            let _e3754 = phi_4744_;
                                            let _e3756 = phi_4745_;
                                            phi_31797_ = _e3750;
                                            let _e7012 = local_61;
                                            phi_4749_ = _e7012;
                                            phi_4750_ = _e3752;
                                            phi_4751_ = _e3754;
                                            let _e7017 = local_62;
                                            phi_4752_ = _e7017;
                                            phi_4753_ = _e3756;
                                        } else {
                                            phi_31797_ = _e311;
                                            phi_4749_ = vec3<f32>(0f, 0f, 0f);
                                            phi_4750_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4751_ = _e1428;
                                            phi_4752_ = vec3<f32>();
                                            phi_4753_ = true;
                                        }
                                        let _e3758 = phi_31797_;
                                        let _e3760 = phi_4749_;
                                        let _e3762 = phi_4750_;
                                        let _e3764 = phi_4751_;
                                        let _e3766 = phi_4752_;
                                        let _e3768 = phi_4753_;
                                        if _e3768 {
                                            if _e3764 {
                                                let _e3769 = (_e1280 + 20u);
                                                let _e3772 = global_6.member[(_e1280 + 21u)];
                                                let _e3773 = bitcast<u32>(_e3772);
                                                let _e3776 = global_6.member[(_e1280 + 22u)];
                                                let _e3777 = bitcast<u32>(_e3776);
                                                let _e3780 = global_6.member[(_e1280 + 23u)];
                                                let _e3784 = global_6.member[(_e1280 + 24u)];
                                                let _e3788 = global_6.member[(_e1280 + 25u)];
                                                let _e3792 = global_6.member[(_e1280 + 26u)];
                                                let _e3795 = global_6.member[_e1280];
                                                let _e3798 = global_6.member[(_e1280 + 1u)];
                                                let _e3801 = global_6.member[(_e1280 + 2u)];
                                                let _e3804 = global_6.member[(_e1280 + 3u)];
                                                let _e3807 = global_6.member[(_e1280 + 4u)];
                                                let _e3810 = global_6.member[(_e1280 + 5u)];
                                                let _e3813 = global_6.member[(_e1280 + 6u)];
                                                let _e3816 = global_6.member[(_e1280 + 7u)];
                                                let _e3819 = global_6.member[(_e1280 + 8u)];
                                                let _e3824 = (((_e322.member_1.x * _e3804) + (_e322.member_1.y * _e3807)) + (_e322.member_1.z * _e3810));
                                                let _e3826 = select(_e3824, -1f, (_e3824 < -1f));
                                                let _e3829 = asin(select(_e3826, 1f, (_e3826 > 1f)));
                                                let _e3840 = atan2((((_e322.member_1.x * _e3813) + (_e322.member_1.y * _e3816)) + (_e322.member_1.z * _e3819)), (((_e322.member_1.x * _e3795) + (_e322.member_1.y * _e3798)) + (_e322.member_1.z * _e3801)));
                                                if (_e3773 != 4294967295u) {
                                                    let _e3843 = global_6.member[_e3773];
                                                    let _e3846 = global_6.member[(_e3773 + 1u)];
                                                    let _e3847 = bitcast<u32>(_e3846);
                                                    let _e3850 = global_6.member[(_e3773 + 2u)];
                                                    let _e3854 = global_6.member[(_e3773 + 3u)];
                                                    let _e3856 = (4f * _e3843);
                                                    phi_5073_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_5076_ = 0u;
                                                    loop {
                                                        let _e3861 = phi_5073_;
                                                        let _e3863 = phi_5076_;
                                                        local_66 = _e3861;
                                                        let _e3864 = (_e3863 < 6u);
                                                        if _e3864 {
                                                            let _e3865 = (_e3863 / 2u);
                                                            if (_e3865 == 0u) {
                                                                phi_23470_ = _e322.member_1.x;
                                                            } else {
                                                                phi_23470_ = select(_e322.member_1.z, _e322.member_1.y, (_e3865 == 1u));
                                                            }
                                                            let _e3873 = phi_23470_;
                                                            let _e3874 = (_e3873 * select(-1f, 1f, ((_e3863 % 2u) == 0u)));
                                                            if (_e3874 > 0f) {
                                                                let _e3878 = ((_e3865 + 1u) % 3u);
                                                                if (_e3878 == 0u) {
                                                                    phi_23485_ = _e322.member_1.x;
                                                                } else {
                                                                    phi_23485_ = select(_e322.member_1.z, _e322.member_1.y, (_e3878 == 1u));
                                                                }
                                                                let _e3883 = phi_23485_;
                                                                let _e3884 = (_e3883 / _e3874);
                                                                let _e3886 = ((_e3865 + 2u) % 3u);
                                                                if (_e3886 == 0u) {
                                                                    phi_23500_ = _e322.member_1.x;
                                                                } else {
                                                                    phi_23500_ = select(_e322.member_1.z, _e322.member_1.y, (_e3886 == 1u));
                                                                }
                                                                let _e3891 = phi_23500_;
                                                                let _e3892 = (_e3891 / _e3874);
                                                                let _e3894 = (1f + (_e3843 * 12f));
                                                                if (abs(_e3884) <= _e3894) {
                                                                    if (abs(_e3892) <= _e3894) {
                                                                        let _e3900 = f32(_e3847);
                                                                        let _e3902 = floor((((_e3884 + 1f) * 0.5f) * _e3900));
                                                                        let _e3909 = select(0i, select(select(i32(_e3902), i32(-2147483648), (_e3902 < -2147483600f)), 2147483647i, (_e3902 > 2147483500f)), (_e3902 == _e3902));
                                                                        let _e3911 = (bitcast<i32>(_e3847) - 1i);
                                                                        let _e3912 = (0i <= _e3911);
                                                                        if _e3912 {
                                                                        } else {
                                                                            phi_31792_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3909 < 0i) {
                                                                            phi_5147_ = 0i;
                                                                        } else {
                                                                            phi_5147_ = select(_e3909, _e3911, (_e3909 > _e3911));
                                                                        }
                                                                        let _e3917 = phi_5147_;
                                                                        let _e3921 = floor((((_e3892 + 1f) * 0.5f) * _e3900));
                                                                        let _e3928 = select(0i, select(select(i32(_e3921), i32(-2147483648), (_e3921 < -2147483600f)), 2147483647i, (_e3921 > 2147483500f)), (_e3921 == _e3921));
                                                                        if _e3912 {
                                                                        } else {
                                                                            phi_31792_ = true;
                                                                            break;
                                                                        }
                                                                        if (_e3928 < 0i) {
                                                                            phi_5177_ = 0i;
                                                                        } else {
                                                                            phi_5177_ = select(_e3928, _e3911, (_e3928 > _e3911));
                                                                        }
                                                                        let _e3933 = phi_5177_;
                                                                        let _e3934 = (_e3917 - 1i);
                                                                        phi_5185_ = _e3861;
                                                                        phi_5188_ = select(0i, _e3934, (0i < _e3934));
                                                                        loop {
                                                                            let _e3938 = phi_5185_;
                                                                            let _e3940 = phi_5188_;
                                                                            let _e3941 = (_e3917 + 1i);
                                                                            local_65 = _e3938;
                                                                            let _e3944 = (_e3940 <= select(_e3941, _e3911, (_e3911 < _e3941)));
                                                                            if _e3944 {
                                                                                let _e3945 = (_e3933 - 1i);
                                                                                phi_5211_ = _e3938;
                                                                                phi_5214_ = select(0i, _e3945, (0i < _e3945));
                                                                                loop {
                                                                                    let _e3949 = phi_5211_;
                                                                                    let _e3951 = phi_5214_;
                                                                                    let _e3952 = (_e3933 + 1i);
                                                                                    local_64 = _e3949;
                                                                                    let _e3955 = (_e3951 <= select(_e3952, _e3911, (_e3911 < _e3952)));
                                                                                    if _e3955 {
                                                                                        let _e3962 = (bitcast<u32>(_e3850) + ((((_e3863 * _e3847) + bitcast<u32>(_e3940)) * _e3847) + bitcast<u32>(_e3951)));
                                                                                        let _e3964 = global_6.member[_e3962];
                                                                                        let _e3968 = global_6.member[(_e3962 + 1u)];
                                                                                        phi_5285_ = _e3949;
                                                                                        phi_5288_ = bitcast<u32>(_e3964);
                                                                                        loop {
                                                                                            let _e3971 = phi_5285_;
                                                                                            let _e3973 = phi_5288_;
                                                                                            local_63 = _e3971;
                                                                                            let _e3974 = (_e3973 < bitcast<u32>(_e3968));
                                                                                            if _e3974 {
                                                                                                let _e3976 = (bitcast<u32>(_e3854) + (6u * _e3973));
                                                                                                let _e3978 = global_6.member[_e3976];
                                                                                                let _e3981 = global_6.member[(_e3976 + 1u)];
                                                                                                let _e3984 = global_6.member[(_e3976 + 2u)];
                                                                                                let _e3987 = global_6.member[(_e3976 + 3u)];
                                                                                                let _e3990 = global_6.member[(_e3976 + 4u)];
                                                                                                let _e3993 = global_6.member[(_e3976 + 5u)];
                                                                                                let _e3994 = (_e322.member_1.x - _e3978);
                                                                                                let _e3995 = (_e322.member_1.y - _e3981);
                                                                                                let _e3996 = (_e322.member_1.z - _e3984);
                                                                                                let _e4001 = (((_e3994 * _e3994) + (_e3995 * _e3995)) + (_e3996 * _e3996));
                                                                                                if (_e4001 < (_e3856 * _e3856)) {
                                                                                                    let _e4009 = ((1f / ((6.2831855f * _e3843) * _e3843)) * exp((-(_e4001) / ((2f * _e3843) * _e3843))));
                                                                                                    phi_5408_ = vec3<f32>((_e3971.x + (_e3987 * _e4009)), (_e3971.y + (_e3990 * _e4009)), (_e3971.z + (_e3993 * _e4009)));
                                                                                                } else {
                                                                                                    phi_5408_ = _e3971;
                                                                                                }
                                                                                                let _e4021 = phi_5408_;
                                                                                                phi_5286_ = _e4021;
                                                                                                phi_5289_ = (_e3973 + 1u);
                                                                                            } else {
                                                                                                phi_5286_ = vec3<f32>();
                                                                                                phi_5289_ = u32();
                                                                                            }
                                                                                            let _e4024 = phi_5286_;
                                                                                            let _e4026 = phi_5289_;
                                                                                            phi_5285_ = _e4024;
                                                                                            phi_5288_ = _e4026;
                                                                                            if !(_e3974) {
                                                                                                break;
                                                                                            }
                                                                                        }
                                                                                        let _e7037 = local_63;
                                                                                        phi_5212_ = _e7037;
                                                                                        phi_5215_ = (_e3951 + 1i);
                                                                                    } else {
                                                                                        phi_5212_ = vec3<f32>();
                                                                                        phi_5215_ = i32();
                                                                                    }
                                                                                    let _e4030 = phi_5212_;
                                                                                    let _e4032 = phi_5215_;
                                                                                    phi_5211_ = _e4030;
                                                                                    phi_5214_ = _e4032;
                                                                                    if !(_e3955) {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e7041 = local_64;
                                                                                phi_5186_ = _e7041;
                                                                                phi_5189_ = (_e3940 + 1i);
                                                                            } else {
                                                                                phi_5186_ = vec3<f32>();
                                                                                phi_5189_ = i32();
                                                                            }
                                                                            let _e4036 = phi_5186_;
                                                                            let _e4038 = phi_5189_;
                                                                            phi_5185_ = _e4036;
                                                                            phi_5188_ = _e4038;
                                                                            if !(_e3944) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e7045 = local_65;
                                                                        phi_5413_ = _e7045;
                                                                    } else {
                                                                        phi_5413_ = _e3861;
                                                                    }
                                                                    let _e4041 = phi_5413_;
                                                                    phi_5414_ = _e4041;
                                                                } else {
                                                                    phi_5414_ = _e3861;
                                                                }
                                                                let _e4043 = phi_5414_;
                                                                phi_5415_ = _e4043;
                                                            } else {
                                                                phi_5415_ = _e3861;
                                                            }
                                                            let _e4045 = phi_5415_;
                                                            phi_5074_ = _e4045;
                                                            phi_5077_ = (_e3863 + 1u);
                                                        } else {
                                                            phi_5074_ = vec3<f32>();
                                                            phi_5077_ = u32();
                                                        }
                                                        let _e4048 = phi_5074_;
                                                        let _e4050 = phi_5077_;
                                                        phi_5073_ = _e4048;
                                                        phi_5076_ = _e4050;
                                                        phi_31792_ = _e3758;
                                                        if !(_e3864) {
                                                            break;
                                                        }
                                                    }
                                                    let _e4053 = phi_31792_;
                                                    phi_32033_ = _e4053;
                                                    if _e4053 {
                                                        break;
                                                    }
                                                    phi_31834_ = _e4053;
                                                    let _e7054 = local_66;
                                                    phi_5435_ = _e7054;
                                                } else {
                                                    phi_31834_ = _e3758;
                                                    phi_5435_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e4055 = phi_31834_;
                                                let _e4057 = phi_5435_;
                                                if (_e3777 != 4294967295u) {
                                                    let _e4060 = global_6.member[_e3777];
                                                    let _e4063 = global_6.member[(_e3777 + 1u)];
                                                    let _e4066 = global_6.member[(_e3777 + 2u)];
                                                    let _e4069 = global_6.member[(_e3777 + 3u)];
                                                    let _e4072 = global_6.member[(_e3777 + 4u)];
                                                    let _e4075 = global_6.member[(_e3777 + 5u)];
                                                    let _e4078 = global_6.member[(_e3777 + 6u)];
                                                    let _e4081 = global_6.member[(_e3777 + 7u)];
                                                    let _e4084 = global_6.member[(_e3777 + 8u)];
                                                    let _e4087 = global_6.member[(_e3777 + 9u)];
                                                    let _e4090 = global_6.member[(_e3777 + 10u)];
                                                    let _e4094 = global_6.member[(_e3777 + 11u)];
                                                    let _e4097 = global_6.member[(_e3777 + 12u)];
                                                    let _e4100 = global_6.member[(_e3777 + 13u)];
                                                    let _e4102 = (_e4060 > 0f);
                                                    let _e4103 = (_e4063 > 0f);
                                                    if _e4103 {
                                                        let _e4106 = (abs(_e3829) <= 0.8f);
                                                        let _e4107 = select(2u, 0u, _e4102);
                                                        phi_5648_ = _e4107;
                                                        phi_5649_ = _e4106;
                                                        phi_5652_ = _e4107;
                                                        phi_5653_ = _e4106;
                                                        phi_5654_ = select(true, false, _e4106);
                                                    } else {
                                                        phi_5648_ = u32();
                                                        phi_5649_ = bool();
                                                        phi_5652_ = select(2u, 0u, _e4102);
                                                        phi_5653_ = false;
                                                        phi_5654_ = true;
                                                    }
                                                    let _e4110 = phi_5648_;
                                                    let _e4112 = phi_5649_;
                                                    let _e4114 = phi_5652_;
                                                    let _e4116 = phi_5653_;
                                                    let _e4118 = phi_5654_;
                                                    let _e4120 = select(select(f32(), 0f, _e4103), 0f, _e4118);
                                                    phi_5664_ = _e4120;
                                                    phi_5667_ = _e4120;
                                                    phi_5669_ = select(_e4110, _e4114, _e4118);
                                                    phi_5671_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_5673_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_5675_ = 0f;
                                                    phi_5677_ = 1f;
                                                    loop {
                                                        let _e4125 = phi_5664_;
                                                        let _e4127 = phi_5667_;
                                                        let _e4129 = phi_5669_;
                                                        let _e4131 = phi_5671_;
                                                        let _e4133 = phi_5673_;
                                                        let _e4135 = phi_5675_;
                                                        let _e4137 = phi_5677_;
                                                        local_26 = _e4137;
                                                        local_27 = _e4137;
                                                        local_28 = _e4137;
                                                        local_29 = _e4137;
                                                        local_30 = _e4135;
                                                        local_31 = _e4127;
                                                        local_32 = _e4125;
                                                        let _e4138 = (_e4129 < select(11u, 2u, _e4118));
                                                        if _e4138 {
                                                            let _e4139 = (_e4129 < 2u);
                                                            if _e4139 {
                                                                let _e4195 = select(23f, 4f, (_e4129 == 0u));
                                                                phi_5755_ = 4u;
                                                                phi_5757_ = vec3<f32>((_e322.member_1.x * _e4195), (_e322.member_1.y * _e4195), (_e322.member_1.z * _e4195));
                                                            } else {
                                                                if (_e4129 == 2u) {
                                                                    phi_5753_ = 3u;
                                                                    phi_5754_ = vec3<f32>(((_e322.member_1.x * 1.5f) + 5f), ((_e322.member_1.y * 1.5f) + 5f), ((_e322.member_1.z * 1.5f) + 5f));
                                                                } else {
                                                                    let _e4141 = (_e4129 >= 7u);
                                                                    if _e4141 {
                                                                        phi_5715_ = (_e4129 - 7u);
                                                                    } else {
                                                                        phi_5715_ = (_e4129 - 3u);
                                                                    }
                                                                    let _e4145 = phi_5715_;
                                                                    if _e4141 {
                                                                        phi_5730_ = vec3<f32>(((_e322.member_1.x * 11f) + -3f), ((_e322.member_1.y * 11f) + 1f), ((_e322.member_1.z * 11f) + 7f));
                                                                    } else {
                                                                        phi_5730_ = vec3<f32>((_e322.member_1.x * 7f), (_e322.member_1.y * 7f), (_e322.member_1.z * 7f));
                                                                    }
                                                                    let _e4158 = phi_5730_;
                                                                    if (_e4145 < 3u) {
                                                                        if (_e4145 == 0u) {
                                                                            phi_23515_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                        } else {
                                                                            phi_23515_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4145 == 1u)));
                                                                        }
                                                                        let _e4177 = phi_23515_;
                                                                        phi_5751_ = 4u;
                                                                        phi_5752_ = (_e4158 + _e4177);
                                                                    } else {
                                                                        if _e4141 {
                                                                            phi_5750_ = (_e4158 + _e4131);
                                                                        } else {
                                                                            phi_5750_ = (_e4158 + vec3<f32>((_e4133.x * 1.1f), (_e4133.y * 1.1f), (_e4133.z * 1.1f)));
                                                                        }
                                                                        let _e4170 = phi_5750_;
                                                                        phi_5751_ = select(7u, 5u, _e4141);
                                                                        phi_5752_ = _e4170;
                                                                    }
                                                                    let _e4180 = phi_5751_;
                                                                    let _e4182 = phi_5752_;
                                                                    phi_5753_ = _e4180;
                                                                    phi_5754_ = _e4182;
                                                                }
                                                                let _e4191 = phi_5753_;
                                                                let _e4193 = phi_5754_;
                                                                phi_5755_ = _e4191;
                                                                phi_5757_ = _e4193;
                                                            }
                                                            let _e4201 = phi_5755_;
                                                            let _e4203 = phi_5757_;
                                                            let _e4205 = select(vec3<f32>(_e4094, _e4097, _e4100), vec3<f32>(_e4084, _e4087, _e4090), vec3(_e4139));
                                                            let _e4207 = global_6.member[_e3769];
                                                            let _e4208 = bitcast<u32>(_e4207);
                                                            let _e4211 = global_6.member[(_e1280 + 11u)];
                                                            let _e4214 = global_6.member[(_e1280 + 12u)];
                                                            let _e4217 = global_6.member[(_e1280 + 13u)];
                                                            let _e4220 = global_6.member[(_e1280 + 14u)];
                                                            let _e4223 = global_6.member[(_e1280 + 15u)];
                                                            let _e4226 = global_6.member[(_e1280 + 16u)];
                                                            let _e4229 = global_6.member[(_e1280 + 17u)];
                                                            let _e4232 = global_6.member[(_e1280 + 18u)];
                                                            let _e4235 = global_6.member[(_e1280 + 19u)];
                                                            phi_5892_ = 0u;
                                                            phi_5895_ = 0f;
                                                            phi_5897_ = 1f;
                                                            phi_5899_ = 0f;
                                                            phi_5901_ = _e4203;
                                                            loop {
                                                                let _e4237 = phi_5892_;
                                                                let _e4239 = phi_5895_;
                                                                let _e4241 = phi_5897_;
                                                                let _e4243 = phi_5899_;
                                                                let _e4245 = phi_5901_;
                                                                local_23 = _e4239;
                                                                local_24 = _e4243;
                                                                local_25 = _e4239;
                                                                let _e4246 = (_e4237 < _e4201);
                                                                if _e4246 {
                                                                    let _e4249 = (_e4245.x + _e4205.x);
                                                                    let _e4252 = (_e4245.y + _e4205.y);
                                                                    let _e4255 = (_e4245.z + _e4205.z);
                                                                    let _e4256 = floor(_e4249);
                                                                    let _e4257 = floor(_e4252);
                                                                    let _e4258 = floor(_e4255);
                                                                    let _e4267 = bitcast<u32>((select(0i, select(select(i32(_e4256), i32(-2147483648), (_e4256 < -2147483600f)), 2147483647i, (_e4256 > 2147483500f)), (_e4256 == _e4256)) & 255i));
                                                                    let _e4276 = bitcast<u32>((select(0i, select(select(i32(_e4257), i32(-2147483648), (_e4257 < -2147483600f)), 2147483647i, (_e4257 > 2147483500f)), (_e4257 == _e4257)) & 255i));
                                                                    let _e4285 = bitcast<u32>((select(0i, select(select(i32(_e4258), i32(-2147483648), (_e4258 < -2147483600f)), 2147483647i, (_e4258 > 2147483500f)), (_e4258 == _e4258)) & 255i));
                                                                    let _e4286 = (_e4249 - _e4256);
                                                                    let _e4287 = (_e4252 - _e4257);
                                                                    let _e4288 = (_e4255 - _e4258);
                                                                    let _e4295 = (((_e4286 * _e4286) * _e4286) * ((_e4286 * ((_e4286 * 6f) - 15f)) + 10f));
                                                                    let _e4302 = (((_e4287 * _e4287) * _e4287) * ((_e4287 * ((_e4287 * 6f) - 15f)) + 10f));
                                                                    let _e4312 = global_6.member[(_e4208 + _e4267)];
                                                                    let _e4318 = (select(select(u32(_e4312), 0u, (_e4312 < 0f)), 4294967295u, (_e4312 > 4294967000f)) + _e4276);
                                                                    let _e4321 = global_6.member[(_e4208 + _e4318)];
                                                                    let _e4327 = (select(select(u32(_e4321), 0u, (_e4321 < 0f)), 4294967295u, (_e4321 > 4294967000f)) + _e4285);
                                                                    let _e4331 = global_6.member[(_e4208 + (_e4318 + 1u))];
                                                                    let _e4337 = (select(select(u32(_e4331), 0u, (_e4331 < 0f)), 4294967295u, (_e4331 > 4294967000f)) + _e4285);
                                                                    let _e4341 = global_6.member[(_e4208 + (_e4267 + 1u))];
                                                                    let _e4347 = (select(select(u32(_e4341), 0u, (_e4341 < 0f)), 4294967295u, (_e4341 > 4294967000f)) + _e4276);
                                                                    let _e4350 = global_6.member[(_e4208 + _e4347)];
                                                                    let _e4356 = (select(select(u32(_e4350), 0u, (_e4350 < 0f)), 4294967295u, (_e4350 > 4294967000f)) + _e4285);
                                                                    let _e4360 = global_6.member[(_e4208 + (_e4347 + 1u))];
                                                                    let _e4366 = (select(select(u32(_e4360), 0u, (_e4360 < 0f)), 4294967295u, (_e4360 > 4294967000f)) + _e4285);
                                                                    let _e4369 = global_6.member[(_e4208 + _e4327)];
                                                                    let _e4375 = (select(select(u32(_e4369), 0u, (_e4369 < 0f)), 4294967295u, (_e4369 > 4294967000f)) % 12u);
                                                                    let _e4378 = select(-1f, 1f, ((_e4375 & 1u) == 0u));
                                                                    let _e4381 = select(-1f, 1f, ((_e4375 & 2u) == 0u));
                                                                    if (_e4375 < 4u) {
                                                                        phi_23576_ = ((_e4378 * _e4286) + (_e4381 * _e4287));
                                                                    } else {
                                                                        if (_e4375 < 8u) {
                                                                            phi_23570_ = ((_e4378 * _e4286) + (_e4381 * _e4288));
                                                                        } else {
                                                                            phi_23570_ = ((_e4378 * _e4287) + (_e4381 * _e4288));
                                                                        }
                                                                        let _e4391 = phi_23570_;
                                                                        phi_23576_ = _e4391;
                                                                    }
                                                                    let _e4396 = phi_23576_;
                                                                    let _e4399 = global_6.member[(_e4208 + _e4356)];
                                                                    let _e4405 = (_e4286 - 1f);
                                                                    let _e4406 = (select(select(u32(_e4399), 0u, (_e4399 < 0f)), 4294967295u, (_e4399 > 4294967000f)) % 12u);
                                                                    let _e4409 = select(-1f, 1f, ((_e4406 & 1u) == 0u));
                                                                    let _e4412 = select(-1f, 1f, ((_e4406 & 2u) == 0u));
                                                                    if (_e4406 < 4u) {
                                                                        phi_23610_ = ((_e4409 * _e4405) + (_e4412 * _e4287));
                                                                    } else {
                                                                        if (_e4406 < 8u) {
                                                                            phi_23604_ = ((_e4409 * _e4405) + (_e4412 * _e4288));
                                                                        } else {
                                                                            phi_23604_ = ((_e4409 * _e4287) + (_e4412 * _e4288));
                                                                        }
                                                                        let _e4422 = phi_23604_;
                                                                        phi_23610_ = _e4422;
                                                                    }
                                                                    let _e4427 = phi_23610_;
                                                                    let _e4430 = global_6.member[(_e4208 + _e4337)];
                                                                    let _e4436 = (_e4287 - 1f);
                                                                    let _e4437 = (select(select(u32(_e4430), 0u, (_e4430 < 0f)), 4294967295u, (_e4430 > 4294967000f)) % 12u);
                                                                    let _e4440 = select(-1f, 1f, ((_e4437 & 1u) == 0u));
                                                                    let _e4443 = select(-1f, 1f, ((_e4437 & 2u) == 0u));
                                                                    if (_e4437 < 4u) {
                                                                        phi_23644_ = ((_e4440 * _e4286) + (_e4443 * _e4436));
                                                                    } else {
                                                                        if (_e4437 < 8u) {
                                                                            phi_23638_ = ((_e4440 * _e4286) + (_e4443 * _e4288));
                                                                        } else {
                                                                            phi_23638_ = ((_e4440 * _e4436) + (_e4443 * _e4288));
                                                                        }
                                                                        let _e4453 = phi_23638_;
                                                                        phi_23644_ = _e4453;
                                                                    }
                                                                    let _e4458 = phi_23644_;
                                                                    let _e4461 = global_6.member[(_e4208 + _e4366)];
                                                                    let _e4467 = (select(select(u32(_e4461), 0u, (_e4461 < 0f)), 4294967295u, (_e4461 > 4294967000f)) % 12u);
                                                                    let _e4470 = select(-1f, 1f, ((_e4467 & 1u) == 0u));
                                                                    let _e4473 = select(-1f, 1f, ((_e4467 & 2u) == 0u));
                                                                    if (_e4467 < 4u) {
                                                                        phi_23678_ = ((_e4470 * _e4405) + (_e4473 * _e4436));
                                                                    } else {
                                                                        if (_e4467 < 8u) {
                                                                            phi_23672_ = ((_e4470 * _e4405) + (_e4473 * _e4288));
                                                                        } else {
                                                                            phi_23672_ = ((_e4470 * _e4436) + (_e4473 * _e4288));
                                                                        }
                                                                        let _e4483 = phi_23672_;
                                                                        phi_23678_ = _e4483;
                                                                    }
                                                                    let _e4488 = phi_23678_;
                                                                    let _e4492 = global_6.member[(_e4208 + (_e4327 + 1u))];
                                                                    let _e4498 = (_e4288 - 1f);
                                                                    let _e4499 = (select(select(u32(_e4492), 0u, (_e4492 < 0f)), 4294967295u, (_e4492 > 4294967000f)) % 12u);
                                                                    let _e4502 = select(-1f, 1f, ((_e4499 & 1u) == 0u));
                                                                    let _e4505 = select(-1f, 1f, ((_e4499 & 2u) == 0u));
                                                                    if (_e4499 < 4u) {
                                                                        phi_23712_ = ((_e4502 * _e4286) + (_e4505 * _e4287));
                                                                    } else {
                                                                        if (_e4499 < 8u) {
                                                                            phi_23706_ = ((_e4502 * _e4286) + (_e4505 * _e4498));
                                                                        } else {
                                                                            phi_23706_ = ((_e4502 * _e4287) + (_e4505 * _e4498));
                                                                        }
                                                                        let _e4515 = phi_23706_;
                                                                        phi_23712_ = _e4515;
                                                                    }
                                                                    let _e4520 = phi_23712_;
                                                                    let _e4524 = global_6.member[(_e4208 + (_e4356 + 1u))];
                                                                    let _e4530 = (select(select(u32(_e4524), 0u, (_e4524 < 0f)), 4294967295u, (_e4524 > 4294967000f)) % 12u);
                                                                    let _e4533 = select(-1f, 1f, ((_e4530 & 1u) == 0u));
                                                                    let _e4536 = select(-1f, 1f, ((_e4530 & 2u) == 0u));
                                                                    if (_e4530 < 4u) {
                                                                        phi_23746_ = ((_e4533 * _e4405) + (_e4536 * _e4287));
                                                                    } else {
                                                                        if (_e4530 < 8u) {
                                                                            phi_23740_ = ((_e4533 * _e4405) + (_e4536 * _e4498));
                                                                        } else {
                                                                            phi_23740_ = ((_e4533 * _e4287) + (_e4536 * _e4498));
                                                                        }
                                                                        let _e4546 = phi_23740_;
                                                                        phi_23746_ = _e4546;
                                                                    }
                                                                    let _e4551 = phi_23746_;
                                                                    let _e4555 = global_6.member[(_e4208 + (_e4337 + 1u))];
                                                                    let _e4561 = (select(select(u32(_e4555), 0u, (_e4555 < 0f)), 4294967295u, (_e4555 > 4294967000f)) % 12u);
                                                                    let _e4564 = select(-1f, 1f, ((_e4561 & 1u) == 0u));
                                                                    let _e4567 = select(-1f, 1f, ((_e4561 & 2u) == 0u));
                                                                    if (_e4561 < 4u) {
                                                                        phi_23780_ = ((_e4564 * _e4286) + (_e4567 * _e4436));
                                                                    } else {
                                                                        if (_e4561 < 8u) {
                                                                            phi_23774_ = ((_e4564 * _e4286) + (_e4567 * _e4498));
                                                                        } else {
                                                                            phi_23774_ = ((_e4564 * _e4436) + (_e4567 * _e4498));
                                                                        }
                                                                        let _e4577 = phi_23774_;
                                                                        phi_23780_ = _e4577;
                                                                    }
                                                                    let _e4582 = phi_23780_;
                                                                    let _e4586 = global_6.member[(_e4208 + (_e4366 + 1u))];
                                                                    let _e4592 = (select(select(u32(_e4586), 0u, (_e4586 < 0f)), 4294967295u, (_e4586 > 4294967000f)) % 12u);
                                                                    let _e4595 = select(-1f, 1f, ((_e4592 & 1u) == 0u));
                                                                    let _e4598 = select(-1f, 1f, ((_e4592 & 2u) == 0u));
                                                                    if (_e4592 < 4u) {
                                                                        phi_23814_ = ((_e4595 * _e4405) + (_e4598 * _e4436));
                                                                    } else {
                                                                        if (_e4592 < 8u) {
                                                                            phi_23808_ = ((_e4595 * _e4405) + (_e4598 * _e4498));
                                                                        } else {
                                                                            phi_23808_ = ((_e4595 * _e4436) + (_e4598 * _e4498));
                                                                        }
                                                                        let _e4608 = phi_23808_;
                                                                        phi_23814_ = _e4608;
                                                                    }
                                                                    let _e4613 = phi_23814_;
                                                                    let _e4616 = (_e4396 + (_e4295 * (_e4427 - _e4396)));
                                                                    let _e4622 = (_e4520 + (_e4295 * (_e4551 - _e4520)));
                                                                    let _e4628 = (_e4616 + (_e4302 * ((_e4458 + (_e4295 * (_e4488 - _e4458))) - _e4616)));
                                                                    phi_5893_ = (_e4237 + 1u);
                                                                    phi_5896_ = (_e4239 + _e4241);
                                                                    phi_5898_ = (_e4241 * 0.5f);
                                                                    phi_5900_ = (_e4243 + ((_e4628 + ((((_e4288 * _e4288) * _e4288) * ((_e4288 * ((_e4288 * 6f) - 15f)) + 10f)) * ((_e4622 + (_e4302 * ((_e4582 + (_e4295 * (_e4613 - _e4582))) - _e4622))) - _e4628))) * _e4241));
                                                                    phi_5902_ = vec3<f32>(((((_e4211 * _e4245.x) + (_e4214 * _e4245.y)) + (_e4217 * _e4245.z)) * 2.03f), ((((_e4220 * _e4245.x) + (_e4223 * _e4245.y)) + (_e4226 * _e4245.z)) * 2.03f), ((((_e4229 * _e4245.x) + (_e4232 * _e4245.y)) + (_e4235 * _e4245.z)) * 2.03f));
                                                                } else {
                                                                    phi_5893_ = u32();
                                                                    phi_5896_ = f32();
                                                                    phi_5898_ = f32();
                                                                    phi_5900_ = f32();
                                                                    phi_5902_ = vec3<f32>();
                                                                }
                                                                let _e4660 = phi_5893_;
                                                                let _e4662 = phi_5896_;
                                                                let _e4664 = phi_5898_;
                                                                let _e4666 = phi_5900_;
                                                                let _e4668 = phi_5902_;
                                                                phi_5892_ = _e4660;
                                                                phi_5895_ = _e4662;
                                                                phi_5897_ = _e4664;
                                                                phi_5899_ = _e4666;
                                                                phi_5901_ = _e4668;
                                                                if !(_e4246) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e4671 = local_23;
                                                            if (_e4671 > 0f) {
                                                                let _e4674 = local_24;
                                                                let _e4676 = local_25;
                                                                phi_6318_ = (_e4674 / _e4676);
                                                            } else {
                                                                phi_6318_ = 0f;
                                                            }
                                                            let _e4679 = phi_6318_;
                                                            if (_e4129 == 0u) {
                                                                phi_6401_ = _e4125;
                                                                phi_6402_ = _e4127;
                                                                phi_6403_ = _e4131;
                                                                phi_6404_ = _e4133;
                                                                phi_6405_ = _e4135;
                                                                phi_6406_ = (_e4137 + (1.5f * _e4679));
                                                            } else {
                                                                if (_e4129 == 1u) {
                                                                    phi_6395_ = _e4125;
                                                                    phi_6396_ = _e4127;
                                                                    phi_6397_ = _e4131;
                                                                    phi_6398_ = _e4133;
                                                                    phi_6399_ = _e4135;
                                                                    phi_6400_ = (_e4137 + (0.8f * _e4679));
                                                                } else {
                                                                    let _e4682 = (_e4129 == 2u);
                                                                    if _e4682 {
                                                                        phi_6390_ = _e4125;
                                                                        phi_6391_ = _e4127;
                                                                        phi_6392_ = _e4131;
                                                                        phi_6393_ = _e4133;
                                                                    } else {
                                                                        if (_e4129 == 3u) {
                                                                            phi_6386_ = _e4125;
                                                                            phi_6387_ = _e4127;
                                                                            phi_6388_ = _e4131;
                                                                            phi_6389_ = vec3<f32>(_e4679, _e4133.y, _e4133.z);
                                                                        } else {
                                                                            if (_e4129 == 4u) {
                                                                                phi_6382_ = _e4125;
                                                                                phi_6383_ = _e4127;
                                                                                phi_6384_ = _e4131;
                                                                                phi_6385_ = vec3<f32>(_e4133.x, _e4679, _e4133.z);
                                                                            } else {
                                                                                if (_e4129 == 5u) {
                                                                                    phi_6378_ = _e4125;
                                                                                    phi_6379_ = _e4127;
                                                                                    phi_6380_ = _e4131;
                                                                                    phi_6381_ = vec3<f32>(_e4133.x, _e4133.y, _e4679);
                                                                                } else {
                                                                                    let _e4686 = (_e4129 == 6u);
                                                                                    if _e4686 {
                                                                                        phi_6375_ = _e4125;
                                                                                        phi_6377_ = _e4131;
                                                                                    } else {
                                                                                        if (_e4129 == 7u) {
                                                                                            phi_6373_ = _e4125;
                                                                                            phi_6374_ = vec3<f32>(_e4679, _e4131.y, _e4131.z);
                                                                                        } else {
                                                                                            if (_e4129 == 8u) {
                                                                                                phi_6371_ = _e4125;
                                                                                                phi_6372_ = vec3<f32>(_e4131.x, _e4679, _e4131.z);
                                                                                            } else {
                                                                                                let _e4689 = (_e4129 == 9u);
                                                                                                if _e4689 {
                                                                                                    phi_6370_ = vec3<f32>(_e4131.x, _e4131.y, _e4679);
                                                                                                } else {
                                                                                                    phi_6370_ = _e4131;
                                                                                                }
                                                                                                let _e4695 = phi_6370_;
                                                                                                phi_6371_ = select(_e4679, _e4125, _e4689);
                                                                                                phi_6372_ = _e4695;
                                                                                            }
                                                                                            let _e4702 = phi_6371_;
                                                                                            let _e4704 = phi_6372_;
                                                                                            phi_6373_ = _e4702;
                                                                                            phi_6374_ = _e4704;
                                                                                        }
                                                                                        let _e4710 = phi_6373_;
                                                                                        let _e4712 = phi_6374_;
                                                                                        phi_6375_ = _e4710;
                                                                                        phi_6377_ = _e4712;
                                                                                    }
                                                                                    let _e4714 = phi_6375_;
                                                                                    let _e4716 = phi_6377_;
                                                                                    phi_6378_ = _e4714;
                                                                                    phi_6379_ = select(_e4127, _e4679, _e4686);
                                                                                    phi_6380_ = _e4716;
                                                                                    phi_6381_ = _e4133;
                                                                                }
                                                                                let _e4723 = phi_6378_;
                                                                                let _e4725 = phi_6379_;
                                                                                let _e4727 = phi_6380_;
                                                                                let _e4729 = phi_6381_;
                                                                                phi_6382_ = _e4723;
                                                                                phi_6383_ = _e4725;
                                                                                phi_6384_ = _e4727;
                                                                                phi_6385_ = _e4729;
                                                                            }
                                                                            let _e4735 = phi_6382_;
                                                                            let _e4737 = phi_6383_;
                                                                            let _e4739 = phi_6384_;
                                                                            let _e4741 = phi_6385_;
                                                                            phi_6386_ = _e4735;
                                                                            phi_6387_ = _e4737;
                                                                            phi_6388_ = _e4739;
                                                                            phi_6389_ = _e4741;
                                                                        }
                                                                        let _e4747 = phi_6386_;
                                                                        let _e4749 = phi_6387_;
                                                                        let _e4751 = phi_6388_;
                                                                        let _e4753 = phi_6389_;
                                                                        phi_6390_ = _e4747;
                                                                        phi_6391_ = _e4749;
                                                                        phi_6392_ = _e4751;
                                                                        phi_6393_ = _e4753;
                                                                    }
                                                                    let _e4755 = phi_6390_;
                                                                    let _e4757 = phi_6391_;
                                                                    let _e4759 = phi_6392_;
                                                                    let _e4761 = phi_6393_;
                                                                    phi_6395_ = _e4755;
                                                                    phi_6396_ = _e4757;
                                                                    phi_6397_ = _e4759;
                                                                    phi_6398_ = _e4761;
                                                                    phi_6399_ = select(_e4135, _e4679, _e4682);
                                                                    phi_6400_ = _e4137;
                                                                }
                                                                let _e4766 = phi_6395_;
                                                                let _e4768 = phi_6396_;
                                                                let _e4770 = phi_6397_;
                                                                let _e4772 = phi_6398_;
                                                                let _e4774 = phi_6399_;
                                                                let _e4776 = phi_6400_;
                                                                phi_6401_ = _e4766;
                                                                phi_6402_ = _e4768;
                                                                phi_6403_ = _e4770;
                                                                phi_6404_ = _e4772;
                                                                phi_6405_ = _e4774;
                                                                phi_6406_ = _e4776;
                                                            }
                                                            let _e4780 = phi_6401_;
                                                            let _e4782 = phi_6402_;
                                                            let _e4784 = phi_6403_;
                                                            let _e4786 = phi_6404_;
                                                            let _e4788 = phi_6405_;
                                                            let _e4790 = phi_6406_;
                                                            phi_5665_ = _e4780;
                                                            phi_5668_ = _e4782;
                                                            phi_5670_ = (_e4129 + 1u);
                                                            phi_5672_ = _e4784;
                                                            phi_5674_ = _e4786;
                                                            phi_5676_ = _e4788;
                                                            phi_5678_ = _e4790;
                                                        } else {
                                                            phi_5665_ = f32();
                                                            phi_5668_ = f32();
                                                            phi_5670_ = u32();
                                                            phi_5672_ = vec3<f32>();
                                                            phi_5674_ = vec3<f32>();
                                                            phi_5676_ = f32();
                                                            phi_5678_ = f32();
                                                        }
                                                        let _e4793 = phi_5665_;
                                                        let _e4795 = phi_5668_;
                                                        let _e4797 = phi_5670_;
                                                        let _e4799 = phi_5672_;
                                                        let _e4801 = phi_5674_;
                                                        let _e4803 = phi_5676_;
                                                        let _e4805 = phi_5678_;
                                                        phi_5664_ = _e4793;
                                                        phi_5667_ = _e4795;
                                                        phi_5669_ = _e4797;
                                                        phi_5671_ = _e4799;
                                                        phi_5673_ = _e4801;
                                                        phi_5675_ = _e4803;
                                                        phi_5677_ = _e4805;
                                                        if !(_e4138) {
                                                            break;
                                                        }
                                                    }
                                                    if _e4102 {
                                                        let _e4820 = (exp((-(abs(_e3829)) / (0.06f + (abs(_e3840) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3840 * -0.71428573f) * (_e3840 * 0.71428573f))))));
                                                        let _e4822 = (_e3829 * 1.3333334f);
                                                        let _e4827 = (1.3f * exp((((_e3840 * _e3840) + (_e4822 * _e4822)) * -12.499999f)));
                                                        let _e4838 = local_26;
                                                        let _e4840 = local_27;
                                                        if (_e4838 != _e4840) {
                                                            phi_23890_ = true;
                                                        } else {
                                                            let _e4843 = local_28;
                                                            phi_23890_ = (0.1f >= _e4843);
                                                        }
                                                        let _e4846 = phi_23890_;
                                                        let _e4848 = local_29;
                                                        let _e4851 = (pow(select(_e4848, 0.1f, _e4846), 1.3f) * _e4060);
                                                        phi_6438_ = vec3<f32>((_e4057.x + (((_e4066 * _e4820) + (_e4075 * _e4827)) * _e4851)), (_e4057.y + (((_e4069 * _e4820) + (_e4078 * _e4827)) * _e4851)), (_e4057.z + (((_e4072 * _e4820) + (_e4081 * _e4827)) * _e4851)));
                                                    } else {
                                                        phi_6438_ = _e4057;
                                                    }
                                                    let _e4863 = phi_6438_;
                                                    if select(_e4112, _e4116, _e4118) {
                                                        let _e4865 = local_30;
                                                        let _e4871 = ((_e3829 - (0.025f * _e4865)) / (0.045f + (abs(_e3840) * 0.0095492955f)));
                                                        let _e4876 = local_31;
                                                        let _e4877 = (_e4876 * 3.3333333f);
                                                        let _e4879 = select(_e4877, 0f, (_e4877 < 0f));
                                                        let _e4881 = select(_e4879, 1f, (_e4879 > 1f));
                                                        let _e4897 = local_32;
                                                        let _e4899 = ((_e4897 - 0.25f) * 5.0000005f);
                                                        let _e4901 = select(_e4899, 0f, (_e4899 < 0f));
                                                        let _e4903 = select(_e4901, 1f, (_e4901 > 1f));
                                                        phi_6517_ = (_e4063 * (((exp((-(_e4871) * _e4871)) * (0.15f + (3f * ((_e4881 * _e4881) * (3f - (2f * _e4881)))))) * (0.5f + (0.9f * exp(((_e3840 * -0.6666667f) * (_e3840 * 0.6666667f)))))) + (1.2f * (exp(((_e3829 * -3.3333333f) * (_e3829 * 3.3333333f))) * ((_e4903 * _e4903) * (3f - (2f * _e4903)))))));
                                                    } else {
                                                        phi_6517_ = 0f;
                                                    }
                                                    let _e4917 = phi_6517_;
                                                    phi_6518_ = _e4863;
                                                    phi_6519_ = _e4917;
                                                } else {
                                                    phi_6518_ = _e4057;
                                                    phi_6519_ = 0f;
                                                }
                                                let _e4919 = phi_6518_;
                                                let _e4921 = phi_6519_;
                                                phi_6521_ = _e4919;
                                                phi_6524_ = 0u;
                                                loop {
                                                    let _e4923 = phi_6521_;
                                                    let _e4925 = phi_6524_;
                                                    local_49 = _e4923;
                                                    local_50 = _e4923;
                                                    local_51 = _e4923;
                                                    let _e4926 = (_e4925 < bitcast<u32>(_e3792));
                                                    if _e4926 {
                                                        let _e4928 = (bitcast<u32>(_e3788) + (_e4925 * 29u));
                                                        let _e4930 = global_6.member[_e4928];
                                                        let _e4931 = (_e4930 > 0.5f);
                                                        let _e4934 = global_6.member[(_e4928 + 1u)];
                                                        let _e4937 = global_6.member[(_e4928 + 2u)];
                                                        let _e4940 = global_6.member[(_e4928 + 3u)];
                                                        let _e4943 = global_6.member[(_e4928 + 4u)];
                                                        let _e4946 = global_6.member[(_e4928 + 5u)];
                                                        let _e4949 = global_6.member[(_e4928 + 6u)];
                                                        let _e4954 = (((_e322.member_1.x * _e4934) + (_e322.member_1.y * _e4937)) + (_e322.member_1.z * _e4940));
                                                        let _e4955 = (_e4954 < _e4946);
                                                        if _e4955 {
                                                            phi_7670_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_7671_ = vec3<f32>();
                                                        } else {
                                                            let _e4958 = global_6.member[(_e4928 + 7u)];
                                                            let _e4961 = global_6.member[(_e4928 + 8u)];
                                                            let _e4964 = global_6.member[(_e4928 + 9u)];
                                                            let _e4967 = global_6.member[(_e4928 + 10u)];
                                                            let _e4970 = global_6.member[(_e4928 + 11u)];
                                                            let _e4973 = global_6.member[(_e4928 + 12u)];
                                                            let _e4976 = global_6.member[(_e4928 + 13u)];
                                                            let _e4979 = global_6.member[(_e4928 + 14u)];
                                                            let _e4982 = global_6.member[(_e4928 + 15u)];
                                                            let _e4985 = global_6.member[(_e4928 + 16u)];
                                                            let _e4988 = global_6.member[(_e4928 + 17u)];
                                                            let _e4991 = global_6.member[(_e4928 + 18u)];
                                                            let _e4994 = global_6.member[(_e4928 + 19u)];
                                                            let _e4997 = global_6.member[(_e4928 + 20u)];
                                                            let _e5000 = global_6.member[(_e4928 + 21u)];
                                                            let _e5003 = global_6.member[(_e4928 + 22u)];
                                                            let _e5006 = global_6.member[(_e4928 + 23u)];
                                                            let _e5009 = global_6.member[(_e4928 + 24u)];
                                                            let _e5012 = global_6.member[(_e4928 + 25u)];
                                                            let _e5015 = global_6.member[(_e4928 + 26u)];
                                                            let _e5018 = global_6.member[(_e4928 + 27u)];
                                                            let _e5021 = global_6.member[(_e4928 + 28u)];
                                                            let _e5028 = (((_e322.member_1.x / _e4954) - _e4934) / _e4943);
                                                            let _e5029 = (((_e322.member_1.y / _e4954) - _e4937) / _e4943);
                                                            let _e5030 = (((_e322.member_1.z / _e4954) - _e4940) / _e4943);
                                                            if _e4931 {
                                                                let _e5635 = (((_e5028 * _e4985) + (_e5029 * _e4988)) + (_e5030 * _e4991));
                                                                let _e5641 = ((((_e5028 * _e4994) + (_e5029 * _e4997)) + (_e5030 * _e5000)) / _e5003);
                                                                phi_7668_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4949 * exp((-2.5f * pow((sqrt(((_e5635 * _e5635) + (_e5641 * _e5641))) * 3.3333333f), 0.7f)))));
                                                                phi_7669_ = vec3<f32>();
                                                            } else {
                                                                let _e5035 = (((_e4958 * _e4934) + (_e4961 * _e4937)) + (_e4964 * _e4940));
                                                                if (abs(_e5035) < 0.12f) {
                                                                    phi_6972_ = select(0.12f, -0.12f, (_e5035 < 0f));
                                                                } else {
                                                                    phi_6972_ = _e5035;
                                                                }
                                                                let _e5041 = phi_6972_;
                                                                let _e5047 = ((((_e4958 * _e5028) + (_e4961 * _e5029)) + (_e4964 * _e5030)) / _e5041);
                                                                let _e5051 = (_e5028 - (_e4934 * _e5047));
                                                                let _e5052 = (_e5029 - (_e4937 * _e5047));
                                                                let _e5053 = (_e5030 - (_e4940 * _e5047));
                                                                let _e5058 = (((_e5051 * _e4967) + (_e5052 * _e4970)) + (_e5053 * _e4973));
                                                                let _e5063 = (((_e5051 * _e4976) + (_e5052 * _e4979)) + (_e5053 * _e4982));
                                                                let _e5067 = sqrt(((_e5058 * _e5058) + (_e5063 * _e5063)));
                                                                let _e5079 = abs(_e5041);
                                                                let _e5081 = (pow((0.5f + (0.5f * cos(((_e5006 * (atan2(_e5063, _e5058) - (log((_e5067 + 0.03f)) / _e5009))) + _e5012)))), 3f) * sqrt(_e5079));
                                                                let _e5086 = global_6.member[_e3769];
                                                                let _e5087 = bitcast<u32>(_e5086);
                                                                let _e5090 = global_6.member[(_e1280 + 11u)];
                                                                let _e5093 = global_6.member[(_e1280 + 12u)];
                                                                let _e5096 = global_6.member[(_e1280 + 13u)];
                                                                let _e5099 = global_6.member[(_e1280 + 14u)];
                                                                let _e5102 = global_6.member[(_e1280 + 15u)];
                                                                let _e5105 = global_6.member[(_e1280 + 16u)];
                                                                let _e5108 = global_6.member[(_e1280 + 17u)];
                                                                let _e5111 = global_6.member[(_e1280 + 18u)];
                                                                let _e5114 = global_6.member[(_e1280 + 19u)];
                                                                phi_7152_ = 0u;
                                                                phi_7155_ = 0f;
                                                                phi_7157_ = 1f;
                                                                phi_7159_ = 0f;
                                                                phi_7161_ = vec3<f32>((_e5058 * 9f), (_e5063 * 9f), 0f);
                                                                loop {
                                                                    let _e5116 = phi_7152_;
                                                                    let _e5118 = phi_7155_;
                                                                    let _e5120 = phi_7157_;
                                                                    let _e5122 = phi_7159_;
                                                                    let _e5124 = phi_7161_;
                                                                    local_33 = _e5118;
                                                                    local_34 = _e5122;
                                                                    local_35 = _e5118;
                                                                    let _e5125 = (_e5116 < 4u);
                                                                    if _e5125 {
                                                                        let _e5127 = (_e5124.x + _e5015);
                                                                        let _e5129 = (_e5124.y + _e5018);
                                                                        let _e5131 = (_e5124.z + _e5021);
                                                                        let _e5132 = floor(_e5127);
                                                                        let _e5133 = floor(_e5129);
                                                                        let _e5134 = floor(_e5131);
                                                                        let _e5143 = bitcast<u32>((select(0i, select(select(i32(_e5132), i32(-2147483648), (_e5132 < -2147483600f)), 2147483647i, (_e5132 > 2147483500f)), (_e5132 == _e5132)) & 255i));
                                                                        let _e5152 = bitcast<u32>((select(0i, select(select(i32(_e5133), i32(-2147483648), (_e5133 < -2147483600f)), 2147483647i, (_e5133 > 2147483500f)), (_e5133 == _e5133)) & 255i));
                                                                        let _e5161 = bitcast<u32>((select(0i, select(select(i32(_e5134), i32(-2147483648), (_e5134 < -2147483600f)), 2147483647i, (_e5134 > 2147483500f)), (_e5134 == _e5134)) & 255i));
                                                                        let _e5162 = (_e5127 - _e5132);
                                                                        let _e5163 = (_e5129 - _e5133);
                                                                        let _e5164 = (_e5131 - _e5134);
                                                                        let _e5171 = (((_e5162 * _e5162) * _e5162) * ((_e5162 * ((_e5162 * 6f) - 15f)) + 10f));
                                                                        let _e5178 = (((_e5163 * _e5163) * _e5163) * ((_e5163 * ((_e5163 * 6f) - 15f)) + 10f));
                                                                        let _e5188 = global_6.member[(_e5087 + _e5143)];
                                                                        let _e5194 = (select(select(u32(_e5188), 0u, (_e5188 < 0f)), 4294967295u, (_e5188 > 4294967000f)) + _e5152);
                                                                        let _e5197 = global_6.member[(_e5087 + _e5194)];
                                                                        let _e5203 = (select(select(u32(_e5197), 0u, (_e5197 < 0f)), 4294967295u, (_e5197 > 4294967000f)) + _e5161);
                                                                        let _e5207 = global_6.member[(_e5087 + (_e5194 + 1u))];
                                                                        let _e5213 = (select(select(u32(_e5207), 0u, (_e5207 < 0f)), 4294967295u, (_e5207 > 4294967000f)) + _e5161);
                                                                        let _e5217 = global_6.member[(_e5087 + (_e5143 + 1u))];
                                                                        let _e5223 = (select(select(u32(_e5217), 0u, (_e5217 < 0f)), 4294967295u, (_e5217 > 4294967000f)) + _e5152);
                                                                        let _e5226 = global_6.member[(_e5087 + _e5223)];
                                                                        let _e5232 = (select(select(u32(_e5226), 0u, (_e5226 < 0f)), 4294967295u, (_e5226 > 4294967000f)) + _e5161);
                                                                        let _e5236 = global_6.member[(_e5087 + (_e5223 + 1u))];
                                                                        let _e5242 = (select(select(u32(_e5236), 0u, (_e5236 < 0f)), 4294967295u, (_e5236 > 4294967000f)) + _e5161);
                                                                        let _e5245 = global_6.member[(_e5087 + _e5203)];
                                                                        let _e5251 = (select(select(u32(_e5245), 0u, (_e5245 < 0f)), 4294967295u, (_e5245 > 4294967000f)) % 12u);
                                                                        let _e5254 = select(-1f, 1f, ((_e5251 & 1u) == 0u));
                                                                        let _e5257 = select(-1f, 1f, ((_e5251 & 2u) == 0u));
                                                                        if (_e5251 < 4u) {
                                                                            phi_23985_ = ((_e5254 * _e5162) + (_e5257 * _e5163));
                                                                        } else {
                                                                            if (_e5251 < 8u) {
                                                                                phi_23979_ = ((_e5254 * _e5162) + (_e5257 * _e5164));
                                                                            } else {
                                                                                phi_23979_ = ((_e5254 * _e5163) + (_e5257 * _e5164));
                                                                            }
                                                                            let _e5267 = phi_23979_;
                                                                            phi_23985_ = _e5267;
                                                                        }
                                                                        let _e5272 = phi_23985_;
                                                                        let _e5275 = global_6.member[(_e5087 + _e5232)];
                                                                        let _e5281 = (_e5162 - 1f);
                                                                        let _e5282 = (select(select(u32(_e5275), 0u, (_e5275 < 0f)), 4294967295u, (_e5275 > 4294967000f)) % 12u);
                                                                        let _e5285 = select(-1f, 1f, ((_e5282 & 1u) == 0u));
                                                                        let _e5288 = select(-1f, 1f, ((_e5282 & 2u) == 0u));
                                                                        if (_e5282 < 4u) {
                                                                            phi_24019_ = ((_e5285 * _e5281) + (_e5288 * _e5163));
                                                                        } else {
                                                                            if (_e5282 < 8u) {
                                                                                phi_24013_ = ((_e5285 * _e5281) + (_e5288 * _e5164));
                                                                            } else {
                                                                                phi_24013_ = ((_e5285 * _e5163) + (_e5288 * _e5164));
                                                                            }
                                                                            let _e5298 = phi_24013_;
                                                                            phi_24019_ = _e5298;
                                                                        }
                                                                        let _e5303 = phi_24019_;
                                                                        let _e5306 = global_6.member[(_e5087 + _e5213)];
                                                                        let _e5312 = (_e5163 - 1f);
                                                                        let _e5313 = (select(select(u32(_e5306), 0u, (_e5306 < 0f)), 4294967295u, (_e5306 > 4294967000f)) % 12u);
                                                                        let _e5316 = select(-1f, 1f, ((_e5313 & 1u) == 0u));
                                                                        let _e5319 = select(-1f, 1f, ((_e5313 & 2u) == 0u));
                                                                        if (_e5313 < 4u) {
                                                                            phi_24053_ = ((_e5316 * _e5162) + (_e5319 * _e5312));
                                                                        } else {
                                                                            if (_e5313 < 8u) {
                                                                                phi_24047_ = ((_e5316 * _e5162) + (_e5319 * _e5164));
                                                                            } else {
                                                                                phi_24047_ = ((_e5316 * _e5312) + (_e5319 * _e5164));
                                                                            }
                                                                            let _e5329 = phi_24047_;
                                                                            phi_24053_ = _e5329;
                                                                        }
                                                                        let _e5334 = phi_24053_;
                                                                        let _e5337 = global_6.member[(_e5087 + _e5242)];
                                                                        let _e5343 = (select(select(u32(_e5337), 0u, (_e5337 < 0f)), 4294967295u, (_e5337 > 4294967000f)) % 12u);
                                                                        let _e5346 = select(-1f, 1f, ((_e5343 & 1u) == 0u));
                                                                        let _e5349 = select(-1f, 1f, ((_e5343 & 2u) == 0u));
                                                                        if (_e5343 < 4u) {
                                                                            phi_24087_ = ((_e5346 * _e5281) + (_e5349 * _e5312));
                                                                        } else {
                                                                            if (_e5343 < 8u) {
                                                                                phi_24081_ = ((_e5346 * _e5281) + (_e5349 * _e5164));
                                                                            } else {
                                                                                phi_24081_ = ((_e5346 * _e5312) + (_e5349 * _e5164));
                                                                            }
                                                                            let _e5359 = phi_24081_;
                                                                            phi_24087_ = _e5359;
                                                                        }
                                                                        let _e5364 = phi_24087_;
                                                                        let _e5368 = global_6.member[(_e5087 + (_e5203 + 1u))];
                                                                        let _e5374 = (_e5164 - 1f);
                                                                        let _e5375 = (select(select(u32(_e5368), 0u, (_e5368 < 0f)), 4294967295u, (_e5368 > 4294967000f)) % 12u);
                                                                        let _e5378 = select(-1f, 1f, ((_e5375 & 1u) == 0u));
                                                                        let _e5381 = select(-1f, 1f, ((_e5375 & 2u) == 0u));
                                                                        if (_e5375 < 4u) {
                                                                            phi_24121_ = ((_e5378 * _e5162) + (_e5381 * _e5163));
                                                                        } else {
                                                                            if (_e5375 < 8u) {
                                                                                phi_24115_ = ((_e5378 * _e5162) + (_e5381 * _e5374));
                                                                            } else {
                                                                                phi_24115_ = ((_e5378 * _e5163) + (_e5381 * _e5374));
                                                                            }
                                                                            let _e5391 = phi_24115_;
                                                                            phi_24121_ = _e5391;
                                                                        }
                                                                        let _e5396 = phi_24121_;
                                                                        let _e5400 = global_6.member[(_e5087 + (_e5232 + 1u))];
                                                                        let _e5406 = (select(select(u32(_e5400), 0u, (_e5400 < 0f)), 4294967295u, (_e5400 > 4294967000f)) % 12u);
                                                                        let _e5409 = select(-1f, 1f, ((_e5406 & 1u) == 0u));
                                                                        let _e5412 = select(-1f, 1f, ((_e5406 & 2u) == 0u));
                                                                        if (_e5406 < 4u) {
                                                                            phi_24155_ = ((_e5409 * _e5281) + (_e5412 * _e5163));
                                                                        } else {
                                                                            if (_e5406 < 8u) {
                                                                                phi_24149_ = ((_e5409 * _e5281) + (_e5412 * _e5374));
                                                                            } else {
                                                                                phi_24149_ = ((_e5409 * _e5163) + (_e5412 * _e5374));
                                                                            }
                                                                            let _e5422 = phi_24149_;
                                                                            phi_24155_ = _e5422;
                                                                        }
                                                                        let _e5427 = phi_24155_;
                                                                        let _e5431 = global_6.member[(_e5087 + (_e5213 + 1u))];
                                                                        let _e5437 = (select(select(u32(_e5431), 0u, (_e5431 < 0f)), 4294967295u, (_e5431 > 4294967000f)) % 12u);
                                                                        let _e5440 = select(-1f, 1f, ((_e5437 & 1u) == 0u));
                                                                        let _e5443 = select(-1f, 1f, ((_e5437 & 2u) == 0u));
                                                                        if (_e5437 < 4u) {
                                                                            phi_24189_ = ((_e5440 * _e5162) + (_e5443 * _e5312));
                                                                        } else {
                                                                            if (_e5437 < 8u) {
                                                                                phi_24183_ = ((_e5440 * _e5162) + (_e5443 * _e5374));
                                                                            } else {
                                                                                phi_24183_ = ((_e5440 * _e5312) + (_e5443 * _e5374));
                                                                            }
                                                                            let _e5453 = phi_24183_;
                                                                            phi_24189_ = _e5453;
                                                                        }
                                                                        let _e5458 = phi_24189_;
                                                                        let _e5462 = global_6.member[(_e5087 + (_e5242 + 1u))];
                                                                        let _e5468 = (select(select(u32(_e5462), 0u, (_e5462 < 0f)), 4294967295u, (_e5462 > 4294967000f)) % 12u);
                                                                        let _e5471 = select(-1f, 1f, ((_e5468 & 1u) == 0u));
                                                                        let _e5474 = select(-1f, 1f, ((_e5468 & 2u) == 0u));
                                                                        if (_e5468 < 4u) {
                                                                            phi_24223_ = ((_e5471 * _e5281) + (_e5474 * _e5312));
                                                                        } else {
                                                                            if (_e5468 < 8u) {
                                                                                phi_24217_ = ((_e5471 * _e5281) + (_e5474 * _e5374));
                                                                            } else {
                                                                                phi_24217_ = ((_e5471 * _e5312) + (_e5474 * _e5374));
                                                                            }
                                                                            let _e5484 = phi_24217_;
                                                                            phi_24223_ = _e5484;
                                                                        }
                                                                        let _e5489 = phi_24223_;
                                                                        let _e5492 = (_e5272 + (_e5171 * (_e5303 - _e5272)));
                                                                        let _e5498 = (_e5396 + (_e5171 * (_e5427 - _e5396)));
                                                                        let _e5504 = (_e5492 + (_e5178 * ((_e5334 + (_e5171 * (_e5364 - _e5334))) - _e5492)));
                                                                        phi_7153_ = (_e5116 + 1u);
                                                                        phi_7156_ = (_e5118 + _e5120);
                                                                        phi_7158_ = (_e5120 * 0.5f);
                                                                        phi_7160_ = (_e5122 + ((_e5504 + ((((_e5164 * _e5164) * _e5164) * ((_e5164 * ((_e5164 * 6f) - 15f)) + 10f)) * ((_e5498 + (_e5178 * ((_e5458 + (_e5171 * (_e5489 - _e5458))) - _e5498))) - _e5504))) * _e5120));
                                                                        phi_7162_ = vec3<f32>(((((_e5090 * _e5124.x) + (_e5093 * _e5124.y)) + (_e5096 * _e5124.z)) * 2.03f), ((((_e5099 * _e5124.x) + (_e5102 * _e5124.y)) + (_e5105 * _e5124.z)) * 2.03f), ((((_e5108 * _e5124.x) + (_e5111 * _e5124.y)) + (_e5114 * _e5124.z)) * 2.03f));
                                                                    } else {
                                                                        phi_7153_ = u32();
                                                                        phi_7156_ = f32();
                                                                        phi_7158_ = f32();
                                                                        phi_7160_ = f32();
                                                                        phi_7162_ = vec3<f32>();
                                                                    }
                                                                    let _e5536 = phi_7153_;
                                                                    let _e5538 = phi_7156_;
                                                                    let _e5540 = phi_7158_;
                                                                    let _e5542 = phi_7160_;
                                                                    let _e5544 = phi_7162_;
                                                                    phi_7152_ = _e5536;
                                                                    phi_7155_ = _e5538;
                                                                    phi_7157_ = _e5540;
                                                                    phi_7159_ = _e5542;
                                                                    phi_7161_ = _e5544;
                                                                    if !(_e5125) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e5547 = local_33;
                                                                if (_e5547 > 0f) {
                                                                    let _e5550 = local_34;
                                                                    let _e5552 = local_35;
                                                                    phi_7575_ = (_e5550 / _e5552);
                                                                } else {
                                                                    phi_7575_ = 0f;
                                                                }
                                                                let _e5555 = phi_7575_;
                                                                let _e5557 = (1f + (1.2f * _e5555));
                                                                if (_e5557 != _e5557) {
                                                                    phi_24269_ = true;
                                                                } else {
                                                                    phi_24269_ = (0f >= _e5557);
                                                                }
                                                                let _e5561 = phi_24269_;
                                                                let _e5566 = ((_e5067 - 1.15f) * -2.8571432f);
                                                                let _e5568 = select(_e5566, 0f, (_e5566 < 0f));
                                                                let _e5570 = select(_e5568, 1f, (_e5568 > 1f));
                                                                let _e5576 = (1f / _e5079);
                                                                if (_e5576 != _e5576) {
                                                                    phi_24284_ = true;
                                                                } else {
                                                                    phi_24284_ = (4f <= _e5576);
                                                                }
                                                                let _e5580 = phi_24284_;
                                                                let _e5583 = (((exp((_e5067 * -3.5714285f)) * ((_e5570 * _e5570) * (3f - (2f * _e5570)))) * select(_e5576, 4f, _e5580)) * 0.12f);
                                                                let _e5589 = sqrt((((_e5028 * _e5028) + (_e5029 * _e5029)) + (_e5030 * _e5030)));
                                                                let _e5596 = (((_e5028 * _e4994) + (_e5029 * _e4997)) + (_e5030 * _e5000));
                                                                let _e5597 = (1f - _e5079);
                                                                let _e5613 = ((_e5583 * (0.6f + (0.4f * (1f - _e5081)))) + exp((_e5589 * -16.666668f)));
                                                                let _e5619 = (((_e5583 * _e5081) * 1.8f) * select(_e5557, 0f, _e5561));
                                                                let _e5626 = (_e4949 * (1f - ((((0.75f * _e5597) * _e5597) * exp(((_e5596 * -28.571428f) * (_e5596 * 28.571428f)))) * exp((_e5589 * -1.4285715f)))));
                                                                phi_7668_ = vec3<f32>();
                                                                phi_7669_ = vec3<f32>((((1.1633643f * _e5613) + (0.86689025f * _e5619)) * _e5626), (((0.97722596f * _e5613) + (1.0155001f * _e5619)) * _e5626), (((0.74455315f * _e5613) + (1.2384146f * _e5619)) * _e5626));
                                                            }
                                                            let _e5653 = phi_7668_;
                                                            let _e5655 = phi_7669_;
                                                            phi_7670_ = _e5653;
                                                            phi_7671_ = _e5655;
                                                        }
                                                        let _e5657 = phi_7670_;
                                                        let _e5659 = phi_7671_;
                                                        phi_6522_ = (_e4923 + select(_e5659, _e5657, vec3(select(_e4931, true, _e4955))));
                                                        phi_6525_ = (_e4925 + 1u);
                                                    } else {
                                                        phi_6522_ = vec3<f32>();
                                                        phi_6525_ = u32();
                                                    }
                                                    let _e5666 = phi_6522_;
                                                    let _e5668 = phi_6525_;
                                                    phi_6521_ = _e5666;
                                                    phi_6524_ = _e5668;
                                                    if !(_e4926) {
                                                        break;
                                                    }
                                                }
                                                phi_32033_ = _e4055;
                                                if _e4055 {
                                                    break;
                                                }
                                                phi_7690_ = vec3<f32>(0f, 0f, 0f);
                                                phi_7693_ = 0u;
                                                phi_7695_ = _e4921;
                                                loop {
                                                    let _e5671 = phi_7690_;
                                                    let _e5673 = phi_7693_;
                                                    let _e5675 = phi_7695_;
                                                    local_45 = _e5675;
                                                    local_46 = _e5675;
                                                    local_47 = _e5675;
                                                    local_48 = _e5675;
                                                    local_52 = _e5671;
                                                    local_53 = _e5671;
                                                    local_54 = _e5671;
                                                    let _e5676 = (_e5673 < bitcast<u32>(_e3784));
                                                    if _e5676 {
                                                        let _e5678 = (bitcast<u32>(_e3780) + (_e5673 * 18u));
                                                        let _e5680 = global_6.member[_e5678];
                                                        let _e5685 = select(select(u32(_e5680), 0u, (_e5680 < 0f)), 4294967295u, (_e5680 > 4294967000f));
                                                        let _e5688 = global_6.member[(_e5678 + 1u)];
                                                        let _e5691 = global_6.member[(_e5678 + 2u)];
                                                        let _e5694 = global_6.member[(_e5678 + 3u)];
                                                        let _e5697 = global_6.member[(_e5678 + 4u)];
                                                        let _e5700 = global_6.member[(_e5678 + 5u)];
                                                        let _e5703 = global_6.member[(_e5678 + 6u)];
                                                        let _e5706 = global_6.member[(_e5678 + 7u)];
                                                        let _e5709 = global_6.member[(_e5678 + 8u)];
                                                        let _e5712 = global_6.member[(_e5678 + 9u)];
                                                        let _e5715 = global_6.member[(_e5678 + 10u)];
                                                        let _e5718 = global_6.member[(_e5678 + 11u)];
                                                        let _e5721 = global_6.member[(_e5678 + 12u)];
                                                        let _e5724 = global_6.member[(_e5678 + 13u)];
                                                        let _e5727 = global_6.member[(_e5678 + 14u)];
                                                        let _e5730 = global_6.member[(_e5678 + 15u)];
                                                        let _e5733 = global_6.member[(_e5678 + 16u)];
                                                        let _e5736 = global_6.member[(_e5678 + 17u)];
                                                        let _e5741 = (((_e322.member_1.x * _e5688) + (_e322.member_1.y * _e5691)) + (_e322.member_1.z * _e5694));
                                                        if (_e5741 < _e5700) {
                                                            phi_9012_ = type_57(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            let _e5746 = ((_e322.member_1.x / _e5741) - _e5688);
                                                            let _e5747 = ((_e322.member_1.y / _e5741) - _e5691);
                                                            let _e5748 = ((_e322.member_1.z / _e5741) - _e5694);
                                                            let _e5755 = ((((_e5746 * _e5709) + (_e5747 * _e5712)) + (_e5748 * _e5715)) / (_e5697 * _e5727));
                                                            let _e5761 = ((((_e5746 * _e5718) + (_e5747 * _e5721)) + (_e5748 * _e5724)) / _e5697);
                                                            let _e5765 = sqrt(((_e5755 * _e5755) + (_e5761 * _e5761)));
                                                            let _e5768 = vec3<f32>((_e5755 * 2.2f), (_e5761 * 2.2f), 0f);
                                                            let _e5769 = (_e5685 == 0u);
                                                            if _e5769 {
                                                                phi_7997_ = 6u;
                                                                phi_7998_ = 1.2f;
                                                                phi_7999_ = _e5768;
                                                            } else {
                                                                let _e5770 = (_e5685 == 1u);
                                                                if _e5770 {
                                                                    phi_7996_ = vec3<f32>((_e5755 * 1.54f), (_e5761 * 1.54f), 0f);
                                                                } else {
                                                                    phi_7996_ = _e5768;
                                                                }
                                                                let _e5775 = phi_7996_;
                                                                phi_7997_ = select(6u, 5u, _e5770);
                                                                phi_7998_ = select(1f, 0.8f, _e5770);
                                                                phi_7999_ = _e5775;
                                                            }
                                                            let _e5779 = phi_7997_;
                                                            let _e5781 = phi_7998_;
                                                            let _e5783 = phi_7999_;
                                                            if _e5769 {
                                                                phi_8016_ = vec3<f32>(((_e5755 * 3.7400002f) + -7f), ((_e5761 * 3.7400002f) + 2f), 5f);
                                                            } else {
                                                                phi_8016_ = vec3<f32>((_e5755 * 3.3000002f), (_e5761 * 3.3000002f), 0f);
                                                            }
                                                            let _e5793 = phi_8016_;
                                                            phi_8018_ = 0u;
                                                            phi_8021_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_8023_ = 0f;
                                                            phi_8025_ = 0f;
                                                            phi_8027_ = 0f;
                                                            loop {
                                                                let _e5796 = phi_8018_;
                                                                let _e5798 = phi_8021_;
                                                                let _e5800 = phi_8023_;
                                                                let _e5802 = phi_8025_;
                                                                let _e5804 = phi_8027_;
                                                                local_39 = _e5804;
                                                                local_40 = _e5802;
                                                                local_41 = _e5800;
                                                                local_42 = _e5802;
                                                                local_43 = _e5802;
                                                                local_44 = _e5800;
                                                                let _e5805 = (_e5796 < 6u);
                                                                if _e5805 {
                                                                    let _e5806 = (_e5796 == 0u);
                                                                    if _e5806 {
                                                                        phi_8069_ = true;
                                                                        phi_8070_ = false;
                                                                        phi_8071_ = false;
                                                                    } else {
                                                                        if (_e5796 <= 4u) {
                                                                            phi_8047_ = (_e5685 == 2u);
                                                                        } else {
                                                                            phi_8047_ = true;
                                                                        }
                                                                        let _e5810 = phi_8047_;
                                                                        if _e5810 {
                                                                            let _e5811 = (_e5796 == 5u);
                                                                            if _e5811 {
                                                                                if _e5769 {
                                                                                    phi_8063_ = false;
                                                                                } else {
                                                                                    phi_8063_ = (_e5685 == 2u);
                                                                                }
                                                                                let _e5814 = phi_8063_;
                                                                                phi_8065_ = _e5814;
                                                                            } else {
                                                                                phi_8065_ = false;
                                                                            }
                                                                            let _e5816 = phi_8065_;
                                                                            phi_8067_ = select(false, _e5769, _e5811);
                                                                            phi_8068_ = _e5816;
                                                                        } else {
                                                                            phi_8067_ = false;
                                                                            phi_8068_ = false;
                                                                        }
                                                                        let _e5819 = phi_8067_;
                                                                        let _e5821 = phi_8068_;
                                                                        phi_8069_ = select(true, false, _e5810);
                                                                        phi_8070_ = _e5819;
                                                                        phi_8071_ = _e5821;
                                                                    }
                                                                    let _e5824 = phi_8069_;
                                                                    let _e5826 = phi_8070_;
                                                                    let _e5828 = phi_8071_;
                                                                    if select(select(_e5828, false, _e5824), true, select(_e5826, true, _e5824)) {
                                                                        if _e5806 {
                                                                            phi_8118_ = 3u;
                                                                            phi_8119_ = vec3<f32>(((_e5755 * 1.7600001f) + 3f), ((_e5761 * 1.7600001f) + 3f), 3f);
                                                                        } else {
                                                                            if (_e5796 < 4u) {
                                                                                let _e5845 = (_e5796 - 1u);
                                                                                if (_e5845 == 0u) {
                                                                                    phi_24303_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                                } else {
                                                                                    phi_24303_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5845 == 1u)));
                                                                                }
                                                                                let _e5851 = phi_24303_;
                                                                                phi_8116_ = 4u;
                                                                                phi_8117_ = (_e5783 + _e5851);
                                                                            } else {
                                                                                let _e5833 = (_e5796 == 4u);
                                                                                if _e5833 {
                                                                                    phi_8115_ = (_e5783 + vec3<f32>((_e5798.x * _e5781), (_e5798.y * _e5781), (_e5798.z * _e5781)));
                                                                                } else {
                                                                                    phi_8115_ = _e5793;
                                                                                }
                                                                                let _e5843 = phi_8115_;
                                                                                phi_8116_ = select(select(4u, 5u, _e5769), _e5779, _e5833);
                                                                                phi_8117_ = _e5843;
                                                                            }
                                                                            let _e5854 = phi_8116_;
                                                                            let _e5856 = phi_8117_;
                                                                            phi_8118_ = _e5854;
                                                                            phi_8119_ = _e5856;
                                                                        }
                                                                        let _e5863 = phi_8118_;
                                                                        let _e5865 = phi_8119_;
                                                                        let _e5867 = global_6.member[_e3769];
                                                                        let _e5868 = bitcast<u32>(_e5867);
                                                                        let _e5871 = global_6.member[(_e1280 + 11u)];
                                                                        let _e5874 = global_6.member[(_e1280 + 12u)];
                                                                        let _e5877 = global_6.member[(_e1280 + 13u)];
                                                                        let _e5880 = global_6.member[(_e1280 + 14u)];
                                                                        let _e5883 = global_6.member[(_e1280 + 15u)];
                                                                        let _e5886 = global_6.member[(_e1280 + 16u)];
                                                                        let _e5889 = global_6.member[(_e1280 + 17u)];
                                                                        let _e5892 = global_6.member[(_e1280 + 18u)];
                                                                        let _e5895 = global_6.member[(_e1280 + 19u)];
                                                                        phi_8254_ = 0u;
                                                                        phi_8257_ = 0f;
                                                                        phi_8259_ = 1f;
                                                                        phi_8261_ = 0f;
                                                                        phi_8263_ = _e5865;
                                                                        loop {
                                                                            let _e5897 = phi_8254_;
                                                                            let _e5899 = phi_8257_;
                                                                            let _e5901 = phi_8259_;
                                                                            let _e5903 = phi_8261_;
                                                                            let _e5905 = phi_8263_;
                                                                            local_36 = _e5899;
                                                                            local_37 = _e5903;
                                                                            local_38 = _e5899;
                                                                            let _e5906 = (_e5897 < _e5863);
                                                                            if _e5906 {
                                                                                let _e5908 = (_e5905.x + _e5730);
                                                                                let _e5910 = (_e5905.y + _e5733);
                                                                                let _e5912 = (_e5905.z + _e5736);
                                                                                let _e5913 = floor(_e5908);
                                                                                let _e5914 = floor(_e5910);
                                                                                let _e5915 = floor(_e5912);
                                                                                let _e5924 = bitcast<u32>((select(0i, select(select(i32(_e5913), i32(-2147483648), (_e5913 < -2147483600f)), 2147483647i, (_e5913 > 2147483500f)), (_e5913 == _e5913)) & 255i));
                                                                                let _e5933 = bitcast<u32>((select(0i, select(select(i32(_e5914), i32(-2147483648), (_e5914 < -2147483600f)), 2147483647i, (_e5914 > 2147483500f)), (_e5914 == _e5914)) & 255i));
                                                                                let _e5942 = bitcast<u32>((select(0i, select(select(i32(_e5915), i32(-2147483648), (_e5915 < -2147483600f)), 2147483647i, (_e5915 > 2147483500f)), (_e5915 == _e5915)) & 255i));
                                                                                let _e5943 = (_e5908 - _e5913);
                                                                                let _e5944 = (_e5910 - _e5914);
                                                                                let _e5945 = (_e5912 - _e5915);
                                                                                let _e5952 = (((_e5943 * _e5943) * _e5943) * ((_e5943 * ((_e5943 * 6f) - 15f)) + 10f));
                                                                                let _e5959 = (((_e5944 * _e5944) * _e5944) * ((_e5944 * ((_e5944 * 6f) - 15f)) + 10f));
                                                                                let _e5969 = global_6.member[(_e5868 + _e5924)];
                                                                                let _e5975 = (select(select(u32(_e5969), 0u, (_e5969 < 0f)), 4294967295u, (_e5969 > 4294967000f)) + _e5933);
                                                                                let _e5978 = global_6.member[(_e5868 + _e5975)];
                                                                                let _e5984 = (select(select(u32(_e5978), 0u, (_e5978 < 0f)), 4294967295u, (_e5978 > 4294967000f)) + _e5942);
                                                                                let _e5988 = global_6.member[(_e5868 + (_e5975 + 1u))];
                                                                                let _e5994 = (select(select(u32(_e5988), 0u, (_e5988 < 0f)), 4294967295u, (_e5988 > 4294967000f)) + _e5942);
                                                                                let _e5998 = global_6.member[(_e5868 + (_e5924 + 1u))];
                                                                                let _e6004 = (select(select(u32(_e5998), 0u, (_e5998 < 0f)), 4294967295u, (_e5998 > 4294967000f)) + _e5933);
                                                                                let _e6007 = global_6.member[(_e5868 + _e6004)];
                                                                                let _e6013 = (select(select(u32(_e6007), 0u, (_e6007 < 0f)), 4294967295u, (_e6007 > 4294967000f)) + _e5942);
                                                                                let _e6017 = global_6.member[(_e5868 + (_e6004 + 1u))];
                                                                                let _e6023 = (select(select(u32(_e6017), 0u, (_e6017 < 0f)), 4294967295u, (_e6017 > 4294967000f)) + _e5942);
                                                                                let _e6026 = global_6.member[(_e5868 + _e5984)];
                                                                                let _e6032 = (select(select(u32(_e6026), 0u, (_e6026 < 0f)), 4294967295u, (_e6026 > 4294967000f)) % 12u);
                                                                                let _e6035 = select(-1f, 1f, ((_e6032 & 1u) == 0u));
                                                                                let _e6038 = select(-1f, 1f, ((_e6032 & 2u) == 0u));
                                                                                if (_e6032 < 4u) {
                                                                                    phi_24364_ = ((_e6035 * _e5943) + (_e6038 * _e5944));
                                                                                } else {
                                                                                    if (_e6032 < 8u) {
                                                                                        phi_24358_ = ((_e6035 * _e5943) + (_e6038 * _e5945));
                                                                                    } else {
                                                                                        phi_24358_ = ((_e6035 * _e5944) + (_e6038 * _e5945));
                                                                                    }
                                                                                    let _e6048 = phi_24358_;
                                                                                    phi_24364_ = _e6048;
                                                                                }
                                                                                let _e6053 = phi_24364_;
                                                                                let _e6056 = global_6.member[(_e5868 + _e6013)];
                                                                                let _e6062 = (_e5943 - 1f);
                                                                                let _e6063 = (select(select(u32(_e6056), 0u, (_e6056 < 0f)), 4294967295u, (_e6056 > 4294967000f)) % 12u);
                                                                                let _e6066 = select(-1f, 1f, ((_e6063 & 1u) == 0u));
                                                                                let _e6069 = select(-1f, 1f, ((_e6063 & 2u) == 0u));
                                                                                if (_e6063 < 4u) {
                                                                                    phi_24398_ = ((_e6066 * _e6062) + (_e6069 * _e5944));
                                                                                } else {
                                                                                    if (_e6063 < 8u) {
                                                                                        phi_24392_ = ((_e6066 * _e6062) + (_e6069 * _e5945));
                                                                                    } else {
                                                                                        phi_24392_ = ((_e6066 * _e5944) + (_e6069 * _e5945));
                                                                                    }
                                                                                    let _e6079 = phi_24392_;
                                                                                    phi_24398_ = _e6079;
                                                                                }
                                                                                let _e6084 = phi_24398_;
                                                                                let _e6087 = global_6.member[(_e5868 + _e5994)];
                                                                                let _e6093 = (_e5944 - 1f);
                                                                                let _e6094 = (select(select(u32(_e6087), 0u, (_e6087 < 0f)), 4294967295u, (_e6087 > 4294967000f)) % 12u);
                                                                                let _e6097 = select(-1f, 1f, ((_e6094 & 1u) == 0u));
                                                                                let _e6100 = select(-1f, 1f, ((_e6094 & 2u) == 0u));
                                                                                if (_e6094 < 4u) {
                                                                                    phi_24432_ = ((_e6097 * _e5943) + (_e6100 * _e6093));
                                                                                } else {
                                                                                    if (_e6094 < 8u) {
                                                                                        phi_24426_ = ((_e6097 * _e5943) + (_e6100 * _e5945));
                                                                                    } else {
                                                                                        phi_24426_ = ((_e6097 * _e6093) + (_e6100 * _e5945));
                                                                                    }
                                                                                    let _e6110 = phi_24426_;
                                                                                    phi_24432_ = _e6110;
                                                                                }
                                                                                let _e6115 = phi_24432_;
                                                                                let _e6118 = global_6.member[(_e5868 + _e6023)];
                                                                                let _e6124 = (select(select(u32(_e6118), 0u, (_e6118 < 0f)), 4294967295u, (_e6118 > 4294967000f)) % 12u);
                                                                                let _e6127 = select(-1f, 1f, ((_e6124 & 1u) == 0u));
                                                                                let _e6130 = select(-1f, 1f, ((_e6124 & 2u) == 0u));
                                                                                if (_e6124 < 4u) {
                                                                                    phi_24466_ = ((_e6127 * _e6062) + (_e6130 * _e6093));
                                                                                } else {
                                                                                    if (_e6124 < 8u) {
                                                                                        phi_24460_ = ((_e6127 * _e6062) + (_e6130 * _e5945));
                                                                                    } else {
                                                                                        phi_24460_ = ((_e6127 * _e6093) + (_e6130 * _e5945));
                                                                                    }
                                                                                    let _e6140 = phi_24460_;
                                                                                    phi_24466_ = _e6140;
                                                                                }
                                                                                let _e6145 = phi_24466_;
                                                                                let _e6149 = global_6.member[(_e5868 + (_e5984 + 1u))];
                                                                                let _e6155 = (_e5945 - 1f);
                                                                                let _e6156 = (select(select(u32(_e6149), 0u, (_e6149 < 0f)), 4294967295u, (_e6149 > 4294967000f)) % 12u);
                                                                                let _e6159 = select(-1f, 1f, ((_e6156 & 1u) == 0u));
                                                                                let _e6162 = select(-1f, 1f, ((_e6156 & 2u) == 0u));
                                                                                if (_e6156 < 4u) {
                                                                                    phi_24500_ = ((_e6159 * _e5943) + (_e6162 * _e5944));
                                                                                } else {
                                                                                    if (_e6156 < 8u) {
                                                                                        phi_24494_ = ((_e6159 * _e5943) + (_e6162 * _e6155));
                                                                                    } else {
                                                                                        phi_24494_ = ((_e6159 * _e5944) + (_e6162 * _e6155));
                                                                                    }
                                                                                    let _e6172 = phi_24494_;
                                                                                    phi_24500_ = _e6172;
                                                                                }
                                                                                let _e6177 = phi_24500_;
                                                                                let _e6181 = global_6.member[(_e5868 + (_e6013 + 1u))];
                                                                                let _e6187 = (select(select(u32(_e6181), 0u, (_e6181 < 0f)), 4294967295u, (_e6181 > 4294967000f)) % 12u);
                                                                                let _e6190 = select(-1f, 1f, ((_e6187 & 1u) == 0u));
                                                                                let _e6193 = select(-1f, 1f, ((_e6187 & 2u) == 0u));
                                                                                if (_e6187 < 4u) {
                                                                                    phi_24534_ = ((_e6190 * _e6062) + (_e6193 * _e5944));
                                                                                } else {
                                                                                    if (_e6187 < 8u) {
                                                                                        phi_24528_ = ((_e6190 * _e6062) + (_e6193 * _e6155));
                                                                                    } else {
                                                                                        phi_24528_ = ((_e6190 * _e5944) + (_e6193 * _e6155));
                                                                                    }
                                                                                    let _e6203 = phi_24528_;
                                                                                    phi_24534_ = _e6203;
                                                                                }
                                                                                let _e6208 = phi_24534_;
                                                                                let _e6212 = global_6.member[(_e5868 + (_e5994 + 1u))];
                                                                                let _e6218 = (select(select(u32(_e6212), 0u, (_e6212 < 0f)), 4294967295u, (_e6212 > 4294967000f)) % 12u);
                                                                                let _e6221 = select(-1f, 1f, ((_e6218 & 1u) == 0u));
                                                                                let _e6224 = select(-1f, 1f, ((_e6218 & 2u) == 0u));
                                                                                if (_e6218 < 4u) {
                                                                                    phi_24568_ = ((_e6221 * _e5943) + (_e6224 * _e6093));
                                                                                } else {
                                                                                    if (_e6218 < 8u) {
                                                                                        phi_24562_ = ((_e6221 * _e5943) + (_e6224 * _e6155));
                                                                                    } else {
                                                                                        phi_24562_ = ((_e6221 * _e6093) + (_e6224 * _e6155));
                                                                                    }
                                                                                    let _e6234 = phi_24562_;
                                                                                    phi_24568_ = _e6234;
                                                                                }
                                                                                let _e6239 = phi_24568_;
                                                                                let _e6243 = global_6.member[(_e5868 + (_e6023 + 1u))];
                                                                                let _e6249 = (select(select(u32(_e6243), 0u, (_e6243 < 0f)), 4294967295u, (_e6243 > 4294967000f)) % 12u);
                                                                                let _e6252 = select(-1f, 1f, ((_e6249 & 1u) == 0u));
                                                                                let _e6255 = select(-1f, 1f, ((_e6249 & 2u) == 0u));
                                                                                if (_e6249 < 4u) {
                                                                                    phi_24602_ = ((_e6252 * _e6062) + (_e6255 * _e6093));
                                                                                } else {
                                                                                    if (_e6249 < 8u) {
                                                                                        phi_24596_ = ((_e6252 * _e6062) + (_e6255 * _e6155));
                                                                                    } else {
                                                                                        phi_24596_ = ((_e6252 * _e6093) + (_e6255 * _e6155));
                                                                                    }
                                                                                    let _e6265 = phi_24596_;
                                                                                    phi_24602_ = _e6265;
                                                                                }
                                                                                let _e6270 = phi_24602_;
                                                                                let _e6273 = (_e6053 + (_e5952 * (_e6084 - _e6053)));
                                                                                let _e6279 = (_e6177 + (_e5952 * (_e6208 - _e6177)));
                                                                                let _e6285 = (_e6273 + (_e5959 * ((_e6115 + (_e5952 * (_e6145 - _e6115))) - _e6273)));
                                                                                phi_8255_ = (_e5897 + 1u);
                                                                                phi_8258_ = (_e5899 + _e5901);
                                                                                phi_8260_ = (_e5901 * 0.5f);
                                                                                phi_8262_ = (_e5903 + ((_e6285 + ((((_e5945 * _e5945) * _e5945) * ((_e5945 * ((_e5945 * 6f) - 15f)) + 10f)) * ((_e6279 + (_e5959 * ((_e6239 + (_e5952 * (_e6270 - _e6239))) - _e6279))) - _e6285))) * _e5901));
                                                                                phi_8264_ = vec3<f32>(((((_e5871 * _e5905.x) + (_e5874 * _e5905.y)) + (_e5877 * _e5905.z)) * 2.03f), ((((_e5880 * _e5905.x) + (_e5883 * _e5905.y)) + (_e5886 * _e5905.z)) * 2.03f), ((((_e5889 * _e5905.x) + (_e5892 * _e5905.y)) + (_e5895 * _e5905.z)) * 2.03f));
                                                                            } else {
                                                                                phi_8255_ = u32();
                                                                                phi_8258_ = f32();
                                                                                phi_8260_ = f32();
                                                                                phi_8262_ = f32();
                                                                                phi_8264_ = vec3<f32>();
                                                                            }
                                                                            let _e6317 = phi_8255_;
                                                                            let _e6319 = phi_8258_;
                                                                            let _e6321 = phi_8260_;
                                                                            let _e6323 = phi_8262_;
                                                                            let _e6325 = phi_8264_;
                                                                            phi_8254_ = _e6317;
                                                                            phi_8257_ = _e6319;
                                                                            phi_8259_ = _e6321;
                                                                            phi_8261_ = _e6323;
                                                                            phi_8263_ = _e6325;
                                                                            if !(_e5906) {
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e6328 = local_36;
                                                                        if (_e6328 > 0f) {
                                                                            let _e6331 = local_37;
                                                                            let _e6333 = local_38;
                                                                            phi_8677_ = (_e6331 / _e6333);
                                                                        } else {
                                                                            phi_8677_ = 0f;
                                                                        }
                                                                        let _e6336 = phi_8677_;
                                                                        if _e5806 {
                                                                            phi_8712_ = _e5798;
                                                                            phi_8713_ = _e5800;
                                                                            phi_8714_ = _e5802;
                                                                        } else {
                                                                            if (_e5796 == 1u) {
                                                                                phi_8709_ = vec3<f32>(_e6336, _e5798.y, _e5798.z);
                                                                                phi_8710_ = _e5800;
                                                                                phi_8711_ = _e5802;
                                                                            } else {
                                                                                if (_e5796 == 2u) {
                                                                                    phi_8706_ = vec3<f32>(_e5798.x, _e6336, _e5798.z);
                                                                                    phi_8707_ = _e5800;
                                                                                    phi_8708_ = _e5802;
                                                                                } else {
                                                                                    if (_e5796 == 3u) {
                                                                                        phi_8703_ = vec3<f32>(_e5798.x, _e5798.y, _e6336);
                                                                                        phi_8704_ = _e5800;
                                                                                        phi_8705_ = _e5802;
                                                                                    } else {
                                                                                        let _e6340 = (_e5796 == 4u);
                                                                                        phi_8703_ = _e5798;
                                                                                        phi_8704_ = select(_e6336, _e5800, _e6340);
                                                                                        phi_8705_ = select(_e5802, _e6336, _e6340);
                                                                                    }
                                                                                    let _e6348 = phi_8703_;
                                                                                    let _e6350 = phi_8704_;
                                                                                    let _e6352 = phi_8705_;
                                                                                    phi_8706_ = _e6348;
                                                                                    phi_8707_ = _e6350;
                                                                                    phi_8708_ = _e6352;
                                                                                }
                                                                                let _e6358 = phi_8706_;
                                                                                let _e6360 = phi_8707_;
                                                                                let _e6362 = phi_8708_;
                                                                                phi_8709_ = _e6358;
                                                                                phi_8710_ = _e6360;
                                                                                phi_8711_ = _e6362;
                                                                            }
                                                                            let _e6368 = phi_8709_;
                                                                            let _e6370 = phi_8710_;
                                                                            let _e6372 = phi_8711_;
                                                                            phi_8712_ = _e6368;
                                                                            phi_8713_ = _e6370;
                                                                            phi_8714_ = _e6372;
                                                                        }
                                                                        let _e6374 = phi_8712_;
                                                                        let _e6376 = phi_8713_;
                                                                        let _e6378 = phi_8714_;
                                                                        phi_8716_ = _e6374;
                                                                        phi_8717_ = _e6376;
                                                                        phi_8718_ = _e6378;
                                                                        phi_8719_ = select(_e5804, _e6336, _e5806);
                                                                    } else {
                                                                        phi_8716_ = _e5798;
                                                                        phi_8717_ = _e5800;
                                                                        phi_8718_ = _e5802;
                                                                        phi_8719_ = _e5804;
                                                                    }
                                                                    let _e6381 = phi_8716_;
                                                                    let _e6383 = phi_8717_;
                                                                    let _e6385 = phi_8718_;
                                                                    let _e6387 = phi_8719_;
                                                                    phi_8019_ = (_e5796 + 1u);
                                                                    phi_8022_ = _e6381;
                                                                    phi_8024_ = _e6383;
                                                                    phi_8026_ = _e6385;
                                                                    phi_8028_ = _e6387;
                                                                } else {
                                                                    phi_8019_ = u32();
                                                                    phi_8022_ = vec3<f32>();
                                                                    phi_8024_ = f32();
                                                                    phi_8026_ = f32();
                                                                    phi_8028_ = f32();
                                                                }
                                                                let _e6390 = phi_8019_;
                                                                let _e6392 = phi_8022_;
                                                                let _e6394 = phi_8024_;
                                                                let _e6396 = phi_8026_;
                                                                let _e6398 = phi_8028_;
                                                                phi_8018_ = _e6390;
                                                                phi_8021_ = _e6392;
                                                                phi_8023_ = _e6394;
                                                                phi_8025_ = _e6396;
                                                                phi_8027_ = _e6398;
                                                                if !(_e5805) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e6401 = local_39;
                                                            let _e6404 = (_e5765 * (1f + (0.45f * _e6401)));
                                                            if _e5769 {
                                                                let _e6507 = local_43;
                                                                let _e6509 = ((_e6507 - -0.25f) * 1.4285715f);
                                                                let _e6511 = select(_e6509, 0f, (_e6509 < 0f));
                                                                let _e6513 = select(_e6511, 1f, (_e6511 > 1f));
                                                                let _e6517 = ((_e6513 * _e6513) * (3f - (2f * _e6513)));
                                                                let _e6519 = ((_e6404 - 1.05f) * -1.1764706f);
                                                                let _e6521 = select(_e6519, 0f, (_e6519 < 0f));
                                                                let _e6523 = select(_e6521, 1f, (_e6521 > 1f));
                                                                let _e6527 = ((_e6523 * _e6523) * (3f - (2f * _e6523)));
                                                                let _e6528 = (_e6517 * _e6527);
                                                                let _e6530 = ((_e6404 - 0.65f) * -1.5384616f);
                                                                let _e6532 = select(_e6530, 0f, (_e6530 < 0f));
                                                                let _e6534 = select(_e6532, 1f, (_e6532 > 1f));
                                                                let _e6539 = (((_e6534 * _e6534) * (3f - (2f * _e6534))) * _e6517);
                                                                let _e6541 = local_44;
                                                                let _e6543 = ((_e6541 - 0.1f) * 2.857143f);
                                                                let _e6545 = select(_e6543, 0f, (_e6543 < 0f));
                                                                let _e6547 = select(_e6545, 1f, (_e6545 > 1f));
                                                                let _e6552 = (((_e6547 * _e6547) * (3f - (2f * _e6547))) * _e6527);
                                                                let _e6557 = ((0.8f * _e6539) * _e6539);
                                                                let _e6566 = (_e5703 * exp((-2f * _e6552)));
                                                                phi_9011_ = type_57(vec3<f32>((((3.1815925f * _e6528) + (0.14634217f * _e6557)) * _e6566), (((0.38179109f * _e6528) + (1.2439084f * _e6557)) * _e6566), (((0.69995034f * _e6528) + (1.0975662f * _e6557)) * _e6566)), (_e5706 * _e6552));
                                                            } else {
                                                                if (_e5685 == 1u) {
                                                                    let _e6477 = local_42;
                                                                    let _e6479 = ((_e6477 - -0.3f) * 1.25f);
                                                                    let _e6481 = select(_e6479, 0f, (_e6479 < 0f));
                                                                    let _e6483 = select(_e6481, 1f, (_e6481 > 1f));
                                                                    let _e6489 = ((_e6404 - 1f) * -1f);
                                                                    let _e6491 = select(_e6489, 0f, (_e6489 < 0f));
                                                                    let _e6493 = select(_e6491, 1f, (_e6491 > 1f));
                                                                    let _e6499 = (((_e6483 * _e6483) * (3f - (2f * _e6483))) * pow(((_e6493 * _e6493) * (3f - (2f * _e6493))), 1.5f));
                                                                    phi_9010_ = type_57((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5703 * _e6499)), (_e5706 * _e6499));
                                                                } else {
                                                                    if (_e5685 == 2u) {
                                                                        let _e6433 = local_41;
                                                                        let _e6436 = (_e5765 * (1f + (0.15f * _e6433)));
                                                                        let _e6437 = (_e6436 - 0.65f);
                                                                        let _e6441 = exp(((_e6437 * -5.5555553f) * (_e6437 * 5.5555553f)));
                                                                        let _e6443 = ((_e6436 - 0.75f) * -1.8181818f);
                                                                        let _e6445 = select(_e6443, 0f, (_e6443 < 0f));
                                                                        let _e6447 = select(_e6445, 1f, (_e6445 > 1f));
                                                                        let _e6451 = ((_e6447 * _e6447) * (3f - (2f * _e6447)));
                                                                        let _e6456 = (exp(((_e5765 * -25f) * (_e5765 * 25f))) * 3f);
                                                                        phi_9009_ = type_57(vec3<f32>(((((3.1815925f * _e6441) + (_e6451 * 0.10243952f)) + _e6456) * _e5703), ((((0.38179109f * _e6441) + (_e6451 * 0.8707359f)) + _e6456) * _e5703), ((((0.69995034f * _e6441) + (_e6451 * 0.76829636f)) + _e6456) * _e5703)), 0f);
                                                                    } else {
                                                                        let _e6408 = local_40;
                                                                        let _e6410 = ((_e6408 - -0.2f) * 1.6666666f);
                                                                        let _e6412 = select(_e6410, 0f, (_e6410 < 0f));
                                                                        let _e6414 = select(_e6412, 1f, (_e6412 > 1f));
                                                                        let _e6420 = ((_e6404 - 1f) * -1.4285715f);
                                                                        let _e6422 = select(_e6420, 0f, (_e6420 < 0f));
                                                                        let _e6424 = select(_e6422, 1f, (_e6422 > 1f));
                                                                        phi_9009_ = type_57(vec3<f32>(0f, 0f, 0f), (_e5706 * (((_e6414 * _e6414) * (3f - (2f * _e6414))) * ((_e6424 * _e6424) * (3f - (2f * _e6424))))));
                                                                    }
                                                                    let _e6475 = phi_9009_;
                                                                    phi_9010_ = _e6475;
                                                                }
                                                                let _e6505 = phi_9010_;
                                                                phi_9011_ = _e6505;
                                                            }
                                                            let _e6574 = phi_9011_;
                                                            phi_9012_ = _e6574;
                                                        }
                                                        let _e6576 = phi_9012_;
                                                        phi_7691_ = (_e5671 + _e6576.member);
                                                        phi_7694_ = (_e5673 + 1u);
                                                        phi_7696_ = (_e5675 + _e6576.member_1);
                                                    } else {
                                                        phi_7691_ = vec3<f32>();
                                                        phi_7694_ = u32();
                                                        phi_7696_ = f32();
                                                    }
                                                    let _e6583 = phi_7691_;
                                                    let _e6585 = phi_7694_;
                                                    let _e6587 = phi_7696_;
                                                    phi_7690_ = _e6583;
                                                    phi_7693_ = _e6585;
                                                    phi_7695_ = _e6587;
                                                    if !(_e5676) {
                                                        break;
                                                    }
                                                }
                                                phi_32033_ = _e4055;
                                                if _e4055 {
                                                    break;
                                                }
                                                let _e6590 = local_45;
                                                if (_e6590 <= 0f) {
                                                    phi_9041_ = vec3<f32>(1f, 1f, 1f);
                                                } else {
                                                    let _e6593 = local_46;
                                                    let _e6596 = local_47;
                                                    let _e6601 = local_48;
                                                    phi_9041_ = vec3<f32>(exp((_e6596 * -0.81f)), exp(-(_e6593)), exp((_e6601 * -1.25f)));
                                                }
                                                let _e6606 = phi_9041_;
                                                let _e6608 = local_49;
                                                let _e6613 = local_50;
                                                let _e6618 = local_51;
                                                let _e6623 = local_52;
                                                let _e6627 = local_53;
                                                let _e6631 = local_54;
                                                let _e6636 = global_6.member[(_e1280 + 9u)];
                                                phi_32042_ = _e4055;
                                                phi_9088_ = vec3<f32>((_e3760.x + ((((_e6608.x * _e6606.x) + _e6623.x) * _e6636) * _e3762.x)), (_e3760.y + ((((_e6613.y * _e6606.y) + _e6627.y) * _e6636) * _e3762.y)), (_e3760.z + ((((_e6618.z * _e6606.z) + _e6631.z) * _e6636) * _e3762.z)));
                                            } else {
                                                phi_32042_ = _e3758;
                                                phi_9088_ = _e3760;
                                            }
                                            let _e6654 = phi_32042_;
                                            let _e6656 = phi_9088_;
                                            phi_32041_ = _e6654;
                                            phi_9089_ = _e6656;
                                        } else {
                                            phi_32041_ = _e3758;
                                            phi_9089_ = _e3766;
                                        }
                                        let _e6658 = phi_32041_;
                                        let _e6660 = phi_9089_;
                                        phi_32040_ = _e6658;
                                        phi_9090_ = _e6660;
                                    } else {
                                        phi_32040_ = _e311;
                                        phi_9090_ = _e1425;
                                    }
                                    let _e6662 = phi_32040_;
                                    let _e6664 = phi_9090_;
                                    phi_32036_ = _e6662;
                                    phi_10363_ = (vec3<f32>(_e322.member_2.x, _e322.member_2.y, _e322.member_2.z) * _e6664);
                                } else {
                                    let _e338 = global_1.member.member_8[1u];
                                    let _e348 = global_6.member[_e338];
                                    let _e351 = global_6.member[(_e338 + 1u)];
                                    let _e354 = global_6.member[(_e338 + 2u)];
                                    let _e357 = global_6.member[(_e338 + 3u)];
                                    let _e360 = global_6.member[(_e338 + 4u)];
                                    let _e363 = global_6.member[(_e338 + 5u)];
                                    let _e366 = global_6.member[(_e338 + 6u)];
                                    let _e369 = global_6.member[(_e338 + 7u)];
                                    let _e372 = global_6.member[(_e338 + 8u)];
                                    let _e375 = global_6.member[(_e338 + 9u)];
                                    let _e378 = global_6.member[(_e338 + 10u)];
                                    let _e381 = global_6.member[(_e338 + 11u)];
                                    let _e384 = global_6.member[(_e338 + 12u)];
                                    let _e387 = global_6.member[(_e338 + 13u)];
                                    let _e390 = global_6.member[(_e338 + 14u)];
                                    let _e393 = global_6.member[(_e338 + 15u)];
                                    let _e397 = global_6.member[(_e338 + 16u)];
                                    let _e400 = global_6.member[(_e338 + 17u)];
                                    let _e403 = global_6.member[(_e338 + 18u)];
                                    let _e406 = global_6.member[(_e338 + 19u)];
                                    let _e409 = global_6.member[(_e338 + 20u)];
                                    let _e412 = global_6.member[(_e338 + 29u)];
                                    let _e413 = bitcast<u32>(_e412);
                                    let _e416 = global_6.member[(_e338 + 30u)];
                                    let _e417 = bitcast<u32>(_e416);
                                    let _e425 = ((((_e322.member.x - _e348) * _e322.member_1.x) + ((_e322.member.y - _e351) * _e322.member_1.y)) + ((_e322.member.z - _e354) * _e322.member_1.z));
                                    let _e437 = ((((_e322.member.x * _e322.member.x) + (_e322.member.y * _e322.member.y)) + (_e322.member.z * _e322.member.z)) - (2f * (((_e322.member.x * _e348) + (_e322.member.y * _e351)) + (_e322.member.z * _e354))));
                                    let _e438 = (_e437 + _e366);
                                    let _e439 = (_e425 * _e425);
                                    let _e440 = (_e439 - _e438);
                                    if (_e440 < 0f) {
                                        phi_20507_ = type_55(0u, type_55().member_1);
                                        phi_20508_ = type_55();
                                        phi_20509_ = true;
                                    } else {
                                        if (_e425 < 0f) {
                                            phi_20479_ = -(sqrt(_e440));
                                        } else {
                                            phi_20479_ = sqrt(_e440);
                                        }
                                        let _e447 = phi_20479_;
                                        let _e449 = -((_e425 + _e447));
                                        let _e450 = (_e449 == 0f);
                                        if _e450 {
                                            phi_20502_ = type_55(1u, type_54(0f, 0f));
                                            phi_20503_ = type_55();
                                        } else {
                                            let _e451 = (_e438 / _e449);
                                            if (_e449 < _e451) {
                                                phi_20491_ = type_54(_e449, _e451);
                                            } else {
                                                phi_20491_ = type_54(_e451, _e449);
                                            }
                                            let _e456 = phi_20491_;
                                            phi_20502_ = type_55();
                                            phi_20503_ = type_55(1u, _e456);
                                        }
                                        let _e459 = phi_20502_;
                                        let _e461 = phi_20503_;
                                        phi_20507_ = _e459;
                                        phi_20508_ = _e461;
                                        phi_20509_ = _e450;
                                    }
                                    let _e466 = phi_20507_;
                                    let _e468 = phi_20508_;
                                    let _e470 = phi_20509_;
                                    if _e470 {
                                        phi_20513_ = _e466;
                                    } else {
                                        phi_20513_ = _e468;
                                    }
                                    let _e472 = phi_20513_;
                                    if (bitcast<i32>(_e472.member) != 0i) {
                                        phi_20400_ = type_55(0u, _e472.member_1);
                                    } else {
                                        phi_20400_ = type_55(1u, type_55().member_1);
                                    }
                                    let _e482 = phi_20400_;
                                    if (bitcast<i32>(_e482.member) != 0i) {
                                        phi_20447_ = type_55(0u, type_55().member_1);
                                    } else {
                                        if (_e482.member_1.member_1 <= 0f) {
                                            phi_20443_ = type_55(0u, type_55().member_1);
                                        } else {
                                            if (_e482.member_1.member != _e482.member_1.member) {
                                                phi_20551_ = true;
                                            } else {
                                                phi_20551_ = (0f >= _e482.member_1.member);
                                            }
                                            let _e494 = phi_20551_;
                                            let _e495 = select(_e482.member_1.member, 0f, _e494);
                                            let _e496 = (_e437 + _e363);
                                            if (_e496 > 0f) {
                                                let _e498 = (_e439 - _e496);
                                                if (_e498 < 0f) {
                                                    phi_20659_ = type_55(0u, type_55().member_1);
                                                    phi_20660_ = type_55();
                                                    phi_20661_ = true;
                                                } else {
                                                    if (_e425 < 0f) {
                                                        phi_20631_ = -(sqrt(_e498));
                                                    } else {
                                                        phi_20631_ = sqrt(_e498);
                                                    }
                                                    let _e505 = phi_20631_;
                                                    let _e507 = -((_e425 + _e505));
                                                    let _e508 = (_e507 == 0f);
                                                    if _e508 {
                                                        phi_20654_ = type_55(1u, type_54(0f, 0f));
                                                        phi_20655_ = type_55();
                                                    } else {
                                                        let _e509 = (_e496 / _e507);
                                                        if (_e507 < _e509) {
                                                            phi_20643_ = type_54(_e507, _e509);
                                                        } else {
                                                            phi_20643_ = type_54(_e509, _e507);
                                                        }
                                                        let _e514 = phi_20643_;
                                                        phi_20654_ = type_55();
                                                        phi_20655_ = type_55(1u, _e514);
                                                    }
                                                    let _e517 = phi_20654_;
                                                    let _e519 = phi_20655_;
                                                    phi_20659_ = _e517;
                                                    phi_20660_ = _e519;
                                                    phi_20661_ = _e508;
                                                }
                                                let _e524 = phi_20659_;
                                                let _e526 = phi_20660_;
                                                let _e528 = phi_20661_;
                                                if _e528 {
                                                    phi_20665_ = _e524;
                                                } else {
                                                    phi_20665_ = _e526;
                                                }
                                                let _e530 = phi_20665_;
                                                if (bitcast<i32>(_e530.member) != 0i) {
                                                    phi_20599_ = select(_e530.member_1.member_1, _e530.member_1.member, (_e530.member_1.member >= 0f));
                                                } else {
                                                    phi_20599_ = -1f;
                                                }
                                                let _e541 = phi_20599_;
                                                if (_e541 >= 0f) {
                                                    if (_e482.member_1.member_1 != _e482.member_1.member_1) {
                                                        phi_20703_ = true;
                                                    } else {
                                                        phi_20703_ = (_e541 <= _e482.member_1.member_1);
                                                    }
                                                    let _e546 = phi_20703_;
                                                    phi_20428_ = select(_e482.member_1.member_1, _e541, _e546);
                                                } else {
                                                    phi_20428_ = _e482.member_1.member_1;
                                                }
                                                let _e549 = phi_20428_;
                                                phi_20430_ = _e549;
                                            } else {
                                                phi_20430_ = _e482.member_1.member_1;
                                            }
                                            let _e551 = phi_20430_;
                                            if (_e551 <= _e495) {
                                                phi_20439_ = type_55(0u, type_55().member_1);
                                            } else {
                                                phi_20439_ = type_55(1u, type_54(_e495, _e551));
                                            }
                                            let _e559 = phi_20439_;
                                            phi_20443_ = _e559;
                                        }
                                        let _e564 = phi_20443_;
                                        phi_20447_ = _e564;
                                    }
                                    let _e569 = phi_20447_;
                                    if (bitcast<i32>(_e569.member) != 0i) {
                                        let _e575 = (_e569.member_1.member < _e322.member.w);
                                        if _e575 {
                                            if (_e569.member_1.member_1 != _e569.member_1.member_1) {
                                                phi_20718_ = true;
                                            } else {
                                                phi_20718_ = (_e322.member.w <= _e569.member_1.member_1);
                                            }
                                            let _e581 = phi_20718_;
                                            let _e582 = select(_e569.member_1.member_1, _e322.member.w, _e581);
                                            phi_9484_ = vec3<f32>(0f, 0f, 0f);
                                            phi_9487_ = vec3<f32>(0f, 0f, 0f);
                                            phi_9489_ = vec3<f32>(0f, 0f, 0f);
                                            phi_9491_ = vec3<f32>(0f, 0f, 0f);
                                            phi_9493_ = 0u;
                                            loop {
                                                let _e584 = phi_9484_;
                                                let _e586 = phi_9487_;
                                                let _e588 = phi_9489_;
                                                let _e590 = phi_9491_;
                                                let _e592 = phi_9493_;
                                                local = _e588;
                                                local_1 = _e586;
                                                local_2 = _e584;
                                                local_3 = _e584;
                                                local_4 = _e584;
                                                local_5 = _e590;
                                                local_6 = _e590;
                                                local_7 = _e590;
                                                let _e593 = (_e592 < 32u);
                                                if _e593 {
                                                    let _e594 = -(_e425);
                                                    if (_e569.member_1.member <= _e582) {
                                                    } else {
                                                        phi_31555_ = true;
                                                        break;
                                                    }
                                                    let _e597 = select(_e594, _e569.member_1.member, (_e594 < _e569.member_1.member));
                                                    let _e599 = select(_e597, _e582, (_e597 > _e582));
                                                    let _e600 = (_e599 - _e569.member_1.member);
                                                    let _e601 = (_e582 - _e599);
                                                    let _e602 = (_e582 - _e569.member_1.member);
                                                    let _e603 = (0.000000001f * _e602);
                                                    if (_e600 <= _e603) {
                                                        let _e678 = (f32(_e592) * 0.03125f);
                                                        let _e681 = (f32((_e592 + 1u)) * 0.03125f);
                                                        let _e683 = ((_e678 + _e681) * 0.5f);
                                                        phi_9594_ = type_54((_e569.member_1.member + ((_e602 * _e683) * _e683)), (_e602 * ((_e681 * _e681) - (_e678 * _e678))));
                                                    } else {
                                                        if (_e601 <= _e603) {
                                                            let _e661 = (f32((31u - _e592)) * 0.03125f);
                                                            let _e664 = (f32((32u - _e592)) * 0.03125f);
                                                            let _e666 = ((_e661 + _e664) * 0.5f);
                                                            phi_9593_ = type_54((_e582 - ((_e602 * _e666) * _e666)), (_e602 * ((_e664 * _e664) - (_e661 * _e661))));
                                                        } else {
                                                            let _e609 = round(((32f * _e600) / (_e600 + _e601)));
                                                            let _e614 = select(select(u32(_e609), 0u, (_e609 < 0f)), 4294967295u, (_e609 > 4294967000f));
                                                            if (_e614 < 1u) {
                                                                phi_9577_ = 1u;
                                                            } else {
                                                                phi_9577_ = select(_e614, 31u, (_e614 > 31u));
                                                            }
                                                            let _e619 = phi_9577_;
                                                            if (_e592 < _e619) {
                                                                let _e640 = ((_e619 - 1u) - _e592);
                                                                let _e642 = f32(_e619);
                                                                let _e643 = (f32(_e640) / _e642);
                                                                let _e646 = (f32((_e640 + 1u)) / _e642);
                                                                let _e648 = ((_e643 + _e646) * 0.5f);
                                                                phi_9592_ = type_54((_e599 - ((_e600 * _e648) * _e648)), (_e600 * ((_e646 * _e646) - (_e643 * _e643))));
                                                            } else {
                                                                let _e622 = (_e592 - _e619);
                                                                let _e624 = f32((32u - _e619));
                                                                let _e625 = (f32(_e622) / _e624);
                                                                let _e628 = (f32((_e622 + 1u)) / _e624);
                                                                let _e630 = ((_e625 + _e628) * 0.5f);
                                                                phi_9592_ = type_54((_e599 + ((_e601 * _e630) * _e630)), (_e601 * ((_e628 * _e628) - (_e625 * _e625))));
                                                            }
                                                            let _e658 = phi_9592_;
                                                            phi_9593_ = _e658;
                                                        }
                                                        let _e676 = phi_9593_;
                                                        phi_9594_ = _e676;
                                                    }
                                                    let _e693 = phi_9594_;
                                                    let _e699 = (_e322.member.x + (_e322.member_1.x * _e693.member));
                                                    let _e700 = (_e322.member.y + (_e322.member_1.y * _e693.member));
                                                    let _e701 = (_e322.member.z + (_e322.member_1.z * _e693.member));
                                                    let _e714 = ((((_e699 * _e699) + (_e700 * _e700)) + (_e701 * _e701)) - (2f * (((_e699 * _e348) + (_e700 * _e351)) + (_e701 * _e354))));
                                                    let _e716 = (_e699 - _e348);
                                                    let _e717 = (_e700 - _e351);
                                                    let _e718 = (_e701 - _e354);
                                                    let _e724 = sqrt((((_e716 * _e716) + (_e717 * _e717)) + (_e718 * _e718)));
                                                    let _e726 = ((_e714 + _e363) / (_e724 + _e357));
                                                    let _e730 = (1f - (abs((_e726 - 25000f)) * 0.00006666667f));
                                                    if (_e730 != _e730) {
                                                        phi_20918_ = true;
                                                    } else {
                                                        phi_20918_ = (0f >= _e730);
                                                    }
                                                    let _e734 = phi_20918_;
                                                    let _e736 = -(_e726);
                                                    let _e739 = (exp((_e736 / _e378)) * _e693.member_1);
                                                    let _e742 = (exp((_e736 / _e381)) * _e693.member_1);
                                                    let _e744 = (_e739 * 0.5f);
                                                    let _e745 = (_e742 * 0.5f);
                                                    let _e746 = ((select(_e730, 0f, _e734) * _e693.member_1) * 0.5f);
                                                    let _e748 = (_e590.x + _e744);
                                                    let _e750 = (_e590.y + _e745);
                                                    let _e752 = (_e590.z + _e746);
                                                    let _e757 = (_e397 * 1.1f);
                                                    let _e758 = (_e757 * _e750);
                                                    let _e777 = (_e397 * _e742);
                                                    if (_e417 == 4294967295u) {
                                                        phi_9922_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e785 = (1f / _e724);
                                                        let _e793 = ((((_e716 * _e785) * _e369) + ((_e717 * _e785) * _e372)) + ((_e718 * _e785) * _e375));
                                                        if (_e726 != _e726) {
                                                            phi_21045_ = true;
                                                        } else {
                                                            phi_21045_ = (0f >= _e726);
                                                        }
                                                        let _e797 = phi_21045_;
                                                        let _e801 = sqrt((select(_e726, 0f, _e797) / (_e360 - _e357)));
                                                        if (_e801 != _e801) {
                                                            phi_21060_ = true;
                                                        } else {
                                                            phi_21060_ = (1f <= _e801);
                                                        }
                                                        let _e805 = phi_21060_;
                                                        let _e807 = (select(_e801, 1f, _e805) * 31f);
                                                        let _e809 = select(_e793, -1f, (_e793 < -1f));
                                                        let _e813 = ((select(_e809, 1f, (_e809 > 1f)) + 1f) * 15.5f);
                                                        let _e818 = select(select(u32(_e807), 0u, (_e807 < 0f)), 4294967295u, (_e807 > 4294967000f));
                                                        let _e820 = select(_e818, 30u, (30u < _e818));
                                                        let _e825 = select(select(u32(_e813), 0u, (_e813 < 0f)), 4294967295u, (_e813 > 4294967000f));
                                                        let _e827 = select(_e825, 30u, (30u < _e825));
                                                        let _e829 = (_e807 - f32(_e820));
                                                        let _e831 = (_e813 - f32(_e827));
                                                        let _e835 = (_e417 + (3u * ((_e820 * 32u) + _e827)));
                                                        let _e840 = (_e417 + (3u * (((_e820 + 1u) * 32u) + _e827)));
                                                        let _e842 = global_6.member[_e835];
                                                        let _e845 = global_6.member[(_e835 + 1u)];
                                                        let _e848 = global_6.member[(_e835 + 2u)];
                                                        let _e849 = (1f - _e831);
                                                        let _e855 = global_6.member[(_e835 + 3u)];
                                                        let _e858 = global_6.member[(_e835 + 4u)];
                                                        let _e861 = global_6.member[(_e835 + 5u)];
                                                        let _e868 = (1f - _e829);
                                                        let _e873 = global_6.member[_e840];
                                                        let _e876 = global_6.member[(_e840 + 1u)];
                                                        let _e879 = global_6.member[(_e840 + 2u)];
                                                        let _e885 = global_6.member[(_e840 + 3u)];
                                                        let _e888 = global_6.member[(_e840 + 4u)];
                                                        let _e891 = global_6.member[(_e840 + 5u)];
                                                        phi_9922_ = vec3<f32>(((((_e842 * _e849) + (_e855 * _e831)) * _e868) + (((_e873 * _e849) + (_e885 * _e831)) * _e829)), ((((_e845 * _e849) + (_e858 * _e831)) * _e868) + (((_e876 * _e849) + (_e888 * _e831)) * _e829)), ((((_e848 * _e849) + (_e861 * _e831)) * _e868) + (((_e879 * _e849) + (_e891 * _e831)) * _e829)));
                                                    }
                                                    let _e906 = phi_9922_;
                                                    let _e920 = (_e714 + _e366);
                                                    if (_e920 > 0f) {
                                                        let _e926 = (((_e716 * _e369) + (_e717 * _e372)) + (_e718 * _e375));
                                                        let _e928 = ((_e926 * _e926) - _e920);
                                                        if (_e926 >= 0f) {
                                                            phi_9996_ = vec3<f32>();
                                                            phi_9997_ = true;
                                                        } else {
                                                            let _e930 = (_e928 <= 0f);
                                                            if _e930 {
                                                                phi_9995_ = vec3<f32>();
                                                            } else {
                                                                let _e933 = (-(_e926) - sqrt(_e928));
                                                                phi_9995_ = vec3<f32>((_e699 + (_e369 * _e933)), (_e700 + (_e372 * _e933)), (_e701 + (_e375 * _e933)));
                                                            }
                                                            let _e942 = phi_9995_;
                                                            phi_9996_ = _e942;
                                                            phi_9997_ = _e930;
                                                        }
                                                        let _e944 = phi_9996_;
                                                        let _e946 = phi_9997_;
                                                        phi_10002_ = _e944;
                                                        phi_10003_ = select(true, false, _e946);
                                                    } else {
                                                        phi_10002_ = vec3<f32>(_e699, _e700, _e701);
                                                        phi_10003_ = true;
                                                    }
                                                    let _e949 = phi_10002_;
                                                    let _e951 = phi_10003_;
                                                    if _e951 {
                                                        let _e953 = (_e949.x - _e348);
                                                        let _e955 = (_e949.y - _e351);
                                                        let _e957 = (_e949.z - _e354);
                                                        let _e963 = sqrt((((_e953 * _e953) + (_e955 * _e955)) + (_e957 * _e957)));
                                                        let _e964 = (1f / _e963);
                                                        let _e982 = ((((((_e949.x * _e949.x) + (_e949.y * _e949.y)) + (_e949.z * _e949.z)) - (2f * (((_e949.x * _e348) + (_e949.y * _e351)) + (_e949.z * _e354)))) + _e363) / (_e963 + _e357));
                                                        if (_e982 != _e982) {
                                                            phi_21184_ = true;
                                                        } else {
                                                            phi_21184_ = (0f >= _e982);
                                                        }
                                                        let _e986 = phi_21184_;
                                                        let _e990 = sqrt((select(_e982, 0f, _e986) / (_e360 - _e357)));
                                                        if (_e990 != _e990) {
                                                            phi_21199_ = true;
                                                        } else {
                                                            phi_21199_ = (1f <= _e990);
                                                        }
                                                        let _e994 = phi_21199_;
                                                        let _e996 = (select(_e990, 1f, _e994) * 63f);
                                                        let _e1001 = ((((_e953 * _e964) * _e369) + ((_e955 * _e964) * _e372)) + ((_e957 * _e964) * _e375));
                                                        let _e1003 = select(_e1001, -1f, (_e1001 < -1f));
                                                        let _e1007 = ((select(_e1003, 1f, (_e1003 > 1f)) + 1f) * 127.5f);
                                                        let _e1012 = select(select(u32(_e996), 0u, (_e996 < 0f)), 4294967295u, (_e996 > 4294967000f));
                                                        let _e1014 = select(_e1012, 62u, (62u < _e1012));
                                                        let _e1019 = select(select(u32(_e1007), 0u, (_e1007 < 0f)), 4294967295u, (_e1007 > 4294967000f));
                                                        let _e1021 = select(_e1019, 254u, (254u < _e1019));
                                                        let _e1023 = (_e996 - f32(_e1014));
                                                        let _e1025 = (_e1007 - f32(_e1021));
                                                        let _e1029 = (_e413 + (3u * ((_e1014 * 256u) + _e1021)));
                                                        let _e1034 = (_e413 + (3u * (((_e1014 + 1u) * 256u) + _e1021)));
                                                        let _e1036 = global_6.member[_e1029];
                                                        let _e1039 = global_6.member[(_e1029 + 1u)];
                                                        let _e1042 = global_6.member[(_e1029 + 2u)];
                                                        let _e1043 = (1f - _e1025);
                                                        let _e1049 = global_6.member[(_e1029 + 3u)];
                                                        let _e1052 = global_6.member[(_e1029 + 4u)];
                                                        let _e1055 = global_6.member[(_e1029 + 5u)];
                                                        let _e1062 = (1f - _e1023);
                                                        let _e1067 = global_6.member[_e1034];
                                                        let _e1070 = global_6.member[(_e1034 + 1u)];
                                                        let _e1073 = global_6.member[(_e1034 + 2u)];
                                                        let _e1079 = global_6.member[(_e1034 + 3u)];
                                                        let _e1082 = global_6.member[(_e1034 + 4u)];
                                                        let _e1085 = global_6.member[(_e1034 + 5u)];
                                                        phi_10294_ = vec3<f32>(((((_e1036 * _e1043) + (_e1049 * _e1025)) * _e1062) + (((_e1067 * _e1043) + (_e1079 * _e1025)) * _e1023)), ((((_e1039 * _e1043) + (_e1052 * _e1025)) * _e1062) + (((_e1070 * _e1043) + (_e1082 * _e1025)) * _e1023)), ((((_e1042 * _e1043) + (_e1055 * _e1025)) * _e1062) + (((_e1073 * _e1043) + (_e1085 * _e1025)) * _e1023)));
                                                    } else {
                                                        phi_10294_ = vec3<f32>(0f, 0f, 0f);
                                                    }
                                                    let _e1100 = phi_10294_;
                                                    let _e1101 = (vec3<f32>(_e748, _e750, _e752) + _e1100);
                                                    let _e1107 = (_e757 * _e1101.y);
                                                    let _e1119 = exp(-((((_e387 * _e1101.x) + _e1107) + (_e403 * _e1101.z))));
                                                    let _e1121 = exp(-((((_e390 * _e1101.x) + _e1107) + (_e406 * _e1101.z))));
                                                    let _e1123 = exp(-((((_e393 * _e1101.x) + _e1107) + (_e409 * _e1101.z))));
                                                    phi_9485_ = vec3<f32>((_e584.x + ((exp(-((((_e387 * _e748) + _e758) + (_e403 * _e752)))) * ((_e387 * _e739) + _e777)) * _e906.x)), (_e584.y + ((exp(-((((_e390 * _e748) + _e758) + (_e406 * _e752)))) * ((_e390 * _e739) + _e777)) * _e906.y)), (_e584.z + ((exp(-((((_e393 * _e748) + _e758) + (_e409 * _e752)))) * ((_e393 * _e739) + _e777)) * _e906.z)));
                                                    phi_9488_ = vec3<f32>((_e586.x + (_e1119 * _e742)), (_e586.y + (_e1121 * _e742)), (_e586.z + (_e1123 * _e742)));
                                                    phi_9490_ = vec3<f32>((_e588.x + (_e1119 * _e739)), (_e588.y + (_e1121 * _e739)), (_e588.z + (_e1123 * _e739)));
                                                    phi_9492_ = vec3<f32>((_e748 + _e744), (_e750 + _e745), (_e752 + _e746));
                                                    phi_9494_ = (_e592 + 1u);
                                                } else {
                                                    phi_9485_ = vec3<f32>();
                                                    phi_9488_ = vec3<f32>();
                                                    phi_9490_ = vec3<f32>();
                                                    phi_9492_ = vec3<f32>();
                                                    phi_9494_ = u32();
                                                }
                                                let _e1150 = phi_9485_;
                                                let _e1152 = phi_9488_;
                                                let _e1154 = phi_9490_;
                                                let _e1156 = phi_9492_;
                                                let _e1158 = phi_9494_;
                                                phi_9484_ = _e1150;
                                                phi_9487_ = _e1152;
                                                phi_9489_ = _e1154;
                                                phi_9491_ = _e1156;
                                                phi_9493_ = _e1158;
                                                phi_31555_ = _e311;
                                                if !(_e593) {
                                                    break;
                                                }
                                            }
                                            let _e1161 = phi_31555_;
                                            phi_32033_ = _e1161;
                                            if _e1161 {
                                                break;
                                            }
                                            let _e1163 = local;
                                            let _e1164 = (_e1163 * vec3<f32>(_e387, _e390, _e393));
                                            let _e1166 = local_1;
                                            let _e1167 = (_e1166 * _e397);
                                            let _e1172 = (((_e322.member_1.x * _e369) + (_e322.member_1.y * _e372)) + (_e322.member_1.z * _e375));
                                            let _e1174 = (1f + (_e1172 * _e1172));
                                            let _e1175 = (0.059683103f * _e1174);
                                            let _e1182 = (_e400 * _e400);
                                            let _e1193 = ((0.119366206f * ((1f - _e1182) * _e1174)) / ((2f + _e1182) * pow(((1f + _e1182) - ((2f * _e400) * _e1172)), 1.5f)));
                                            let _e1204 = local_2;
                                            let _e1208 = local_3;
                                            let _e1212 = local_4;
                                            let _e1221 = local_5;
                                            let _e1228 = local_6;
                                            let _e1230 = ((_e397 * 1.1f) * _e1228.y);
                                            let _e1235 = local_7;
                                            phi_32039_ = _e1161;
                                            phi_10351_ = type_58(vec3<f32>(exp(-((((_e387 * _e1221.x) + _e1230) + (_e403 * _e1235.z)))), exp(-((((_e390 * _e1221.x) + _e1230) + (_e406 * _e1235.z)))), exp(-((((_e393 * _e1221.x) + _e1230) + (_e409 * _e1235.z))))), (vec3<f32>(((((_e1164.x * _e1175) + (_e1167.x * _e1193)) + _e1204.x) * _e384), ((((_e1164.y * _e1175) + (_e1167.y * _e1193)) + _e1208.y) * _e384), ((((_e1164.z * _e1175) + (_e1167.z * _e1193)) + _e1212.z) * _e384)) * 0.38317f));
                                        } else {
                                            phi_32039_ = _e311;
                                            phi_10351_ = type_58();
                                        }
                                        let _e1252 = phi_32039_;
                                        let _e1254 = phi_10351_;
                                        phi_32038_ = _e1252;
                                        phi_10353_ = _e1254;
                                        phi_10354_ = select(true, false, _e575);
                                    } else {
                                        phi_32038_ = _e311;
                                        phi_10353_ = type_58();
                                        phi_10354_ = true;
                                    }
                                    let _e1257 = phi_32038_;
                                    let _e1259 = phi_10353_;
                                    let _e1261 = phi_10354_;
                                    if _e1261 {
                                        phi_10359_ = type_58(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                    } else {
                                        phi_10359_ = _e1259;
                                    }
                                    let _e1263 = phi_10359_;
                                    phi_32036_ = _e1257;
                                    phi_10363_ = (vec3<f32>(_e322.member_2.x, _e322.member_2.y, _e322.member_2.z) * _e1263.member_1);
                                }
                                let _e6668 = phi_32036_;
                                let _e6670 = phi_10363_;
                                phi_32035_ = _e6668;
                                phi_10364_ = _e6670;
                            }
                            let _e6672 = phi_32035_;
                            let _e6674 = phi_10364_;
                            phi_32034_ = _e6672;
                            phi_482_ = (_e313 + _e6674);
                            phi_485_ = (_e315 + 1u);
                        } else {
                            phi_32034_ = _e311;
                            phi_482_ = vec3<f32>();
                            phi_485_ = u32();
                        }
                        let _e6678 = phi_32034_;
                        let _e6680 = phi_482_;
                        let _e6682 = phi_485_;
                        phi_31572_ = _e6678;
                        phi_481_ = _e6680;
                        phi_484_ = _e6682;
                        phi_32033_ = _e6678;
                        if !(_e316) {
                            break;
                        }
                    }
                    let _e6685 = phi_32033_;
                    if _e6685 {
                        break;
                    }
                    if _e302 {
                    } else {
                        break;
                    }
                    let _e6687 = local_55;
                    let _e6690 = local_56;
                    let _e6693 = local_57;
                    let _e6699 = global_7.member[_e297.member_1].member_3[0u];
                    global_7.member[_e297.member_1].member_3[0u] = (_e6699 + _e6687.x);
                    let _e6705 = global_7.member[_e297.member_1].member_3[1u];
                    global_7.member[_e297.member_1].member_3[1u] = (_e6705 + _e6690.y);
                    let _e6711 = global_7.member[_e297.member_1].member_3[2u];
                    global_7.member[_e297.member_1].member_3[2u] = (_e6711 + _e6693.z);
                    let _e6717 = global_7.member[_e297.member_1].member_3[3u];
                    global_7.member[_e297.member_1].member_3[3u] = _e6717;
                    if _e302 {
                    } else {
                        break;
                    }
                    global_7.member[_e297.member_1].member_4[1u] = (_e307 & 4294967279u);
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
    var phi_11130_: type_45;
    var phi_11333_: u32;
    var phi_11336_: u32;
    var phi_11338_: u32;
    var phi_11340_: vec4<f32>;
    var phi_11342_: type_64;
    var phi_24779_: bool;
    var phi_24809_: bool;
    var phi_24756_: type_54;
    var phi_24759_: type_54;
    var phi_24760_: bool;
    var phi_24765_: type_54;
    var phi_24866_: bool;
    var phi_24881_: bool;
    var phi_24896_: bool;
    var phi_24911_: bool;
    var phi_24843_: type_54;
    var phi_24846_: type_54;
    var phi_24847_: bool;
    var phi_24852_: type_54;
    var phi_24953_: bool;
    var phi_24968_: bool;
    var phi_24983_: bool;
    var phi_24998_: bool;
    var phi_24930_: type_54;
    var phi_24933_: type_54;
    var phi_24934_: bool;
    var phi_24939_: type_54;
    var phi_24732_: bool;
    var phi_24735_: bool;
    var phi_24736_: bool;
    var phi_11732_: u32;
    var phi_11733_: u32;
    var phi_11734_: u32;
    var phi_11735_: u32;
    var phi_11736_: u32;
    var phi_11737_: u32;
    var phi_11738_: u32;
    var phi_11739_: u32;
    var phi_11740_: u32;
    var phi_11621_: vec4<f32>;
    var phi_11622_: vec4<f32>;
    var phi_11623_: bool;
    var phi_11627_: vec4<f32>;
    var phi_11628_: bool;
    var phi_11538_: bool;
    var phi_11572_: bool;
    var phi_11578_: f32;
    var phi_11588_: vec4<f32>;
    var phi_11589_: vec4<f32>;
    var phi_11590_: bool;
    var phi_11594_: vec4<f32>;
    var phi_11595_: bool;
    var phi_11599_: vec4<f32>;
    var phi_11600_: bool;
    var phi_11629_: vec4<f32>;
    var phi_11630_: bool;
    var phi_11470_: vec4<f32>;
    var phi_11471_: vec4<f32>;
    var phi_11472_: bool;
    var phi_11476_: vec4<f32>;
    var phi_11477_: bool;
    var phi_11478_: vec4<f32>;
    var phi_11479_: bool;
    var phi_11480_: bool;
    var phi_11488_: vec4<f32>;
    var phi_11489_: bool;
    var phi_11490_: vec4<f32>;
    var phi_11491_: bool;
    var phi_11492_: bool;
    var phi_11500_: vec4<f32>;
    var phi_11501_: bool;
    var phi_11639_: vec4<f32>;
    var phi_11640_: bool;
    var phi_11661_: type_64;
    var phi_11663_: type_64;
    var phi_11334_: u32;
    var phi_11337_: u32;
    var phi_11339_: u32;
    var phi_11341_: vec4<f32>;
    var phi_11343_: type_64;
    var phi_11742_: type_64;
    var phi_11743_: bool;
    var phi_11744_: bool;
    var phi_32044_: bool;
    var phi_20377_: bool;
    var phi_20376_: type_64;
    var local_69: type_64;
    var local_70: vec4<f32>;
    var phi_11905_: type_58;
    var local_71: vec4<f32>;
    var local_72: vec4<f32>;
    var phi_11871_: vec3<f32>;
    var phi_11872_: vec3<f32>;
    var phi_11906_: type_58;
    var local_73: type_64;
    var phi_11911_: type_64;
    var phi_11912_: type_64;
    var phi_11917_: type_64;
    var phi_25117_: f32;
    var phi_25129_: type_54;
    var phi_25140_: type_55;
    var phi_25141_: type_55;
    var phi_25145_: type_55;
    var phi_25146_: type_55;
    var phi_25147_: bool;
    var phi_25151_: type_55;
    var phi_25038_: type_55;
    var phi_25189_: bool;
    var phi_25269_: f32;
    var phi_25281_: type_54;
    var phi_25292_: type_55;
    var phi_25293_: type_55;
    var phi_25297_: type_55;
    var phi_25298_: type_55;
    var phi_25299_: bool;
    var phi_25303_: type_55;
    var phi_25237_: f32;
    var phi_25341_: bool;
    var phi_25066_: f32;
    var phi_25068_: f32;
    var phi_25077_: type_55;
    var phi_25081_: type_55;
    var phi_25085_: type_55;
    var phi_25356_: bool;
    var phi_13336_: vec3<f32>;
    var phi_13339_: u32;
    var phi_13423_: u32;
    var phi_13438_: type_54;
    var phi_13439_: type_54;
    var phi_13440_: type_54;
    var phi_25555_: bool;
    var phi_13337_: vec3<f32>;
    var phi_13340_: u32;
    var phi_32117_: bool;
    var local_74: vec3<f32>;
    var local_75: vec3<f32>;
    var local_76: vec3<f32>;
    var phi_34344_: bool;
    var phi_13465_: vec3<f32>;
    var phi_34343_: bool;
    var phi_13467_: vec3<f32>;
    var phi_13468_: bool;
    var phi_34341_: bool;
    var phi_13475_: vec3<f32>;
    var phi_13476_: type_39;
    var phi_34340_: bool;
    var phi_13477_: vec3<f32>;
    var phi_13478_: type_39;
    var phi_13502_: i32;
    var phi_13505_: u32;
    var phi_13559_: bool;
    var phi_13565_: bool;
    var phi_13572_: u32;
    var phi_13574_: i32;
    var phi_13575_: u32;
    var phi_13503_: i32;
    var phi_13506_: u32;
    var phi_13577_: bool;
    var local_77: u32;
    var phi_13858_: vec3<f32>;
    var phi_13875_: vec3<f32>;
    var phi_25622_: bool;
    var phi_13908_: f32;
    var phi_14017_: vec3<f32>;
    var phi_14023_: f32;
    var phi_25695_: bool;
    var phi_25684_: f32;
    var phi_14104_: vec3<f32>;
    var phi_14105_: vec3<f32>;
    var phi_14106_: vec3<f32>;
    var phi_32333_: u32;
    var phi_14108_: vec3<f32>;
    var phi_14109_: vec3<f32>;
    var phi_14110_: vec3<f32>;
    var phi_14111_: bool;
    var phi_14126_: vec3<f32>;
    var phi_14127_: vec3<f32>;
    var phi_32331_: u32;
    var phi_14129_: vec3<f32>;
    var phi_14130_: vec3<f32>;
    var phi_14131_: vec3<f32>;
    var phi_32330_: u32;
    var phi_14132_: vec3<f32>;
    var phi_14133_: vec3<f32>;
    var phi_14134_: vec3<f32>;
    var phi_14135_: bool;
    var phi_32329_: u32;
    var phi_14136_: vec3<f32>;
    var phi_14137_: vec3<f32>;
    var phi_14138_: vec3<f32>;
    var phi_14139_: bool;
    var phi_14140_: bool;
    var phi_25768_: bool;
    var phi_25930_: bool;
    var phi_25945_: bool;
    var phi_25960_: bool;
    var phi_25987_: bool;
    var phi_26002_: bool;
    var phi_26017_: bool;
    var phi_26044_: bool;
    var phi_26059_: bool;
    var phi_26074_: bool;
    var phi_14319_: f32;
    var phi_14327_: f32;
    var phi_14335_: f32;
    var phi_26271_: bool;
    var phi_26260_: f32;
    var phi_14442_: vec3<f32>;
    var phi_26299_: bool;
    var phi_14443_: vec3<f32>;
    var phi_14444_: vec3<f32>;
    var phi_14445_: vec3<f32>;
    var phi_32717_: vec3<f32>;
    var phi_32640_: vec3<f32>;
    var phi_32561_: u32;
    var phi_32479_: f32;
    var phi_14447_: vec3<f32>;
    var phi_14448_: vec3<f32>;
    var phi_14449_: bool;
    var phi_26314_: bool;
    var phi_26365_: bool;
    var phi_26354_: f32;
    var phi_32688_: vec3<f32>;
    var phi_13981_: vec3<f32>;
    var phi_33040_: vec3<f32>;
    var phi_32834_: f32;
    var phi_32687_: vec3<f32>;
    var phi_32375_: u32;
    var phi_13983_: vec3<f32>;
    var phi_13984_: bool;
    var phi_32986_: vec3<f32>;
    var phi_32794_: f32;
    var phi_32715_: vec3<f32>;
    var phi_32638_: vec3<f32>;
    var phi_32559_: u32;
    var phi_32477_: f32;
    var phi_32400_: u32;
    var phi_32326_: u32;
    var phi_14457_: vec3<f32>;
    var phi_14458_: vec3<f32>;
    var phi_14459_: vec3<f32>;
    var phi_14460_: bool;
    var phi_26431_: bool;
    var phi_14529_: vec3<f32>;
    var phi_32383_: u32;
    var phi_14531_: vec3<f32>;
    var phi_14532_: bool;
    var phi_32382_: u32;
    var phi_14533_: vec3<f32>;
    var phi_14534_: bool;
    var phi_32380_: u32;
    var phi_14540_: vec3<f32>;
    var phi_14541_: bool;
    var phi_14574_: type_69;
    var phi_32385_: u32;
    var phi_14576_: vec3<f32>;
    var phi_14577_: type_69;
    var phi_14578_: bool;
    var phi_14584_: type_69;
    var phi_32389_: u32;
    var phi_14585_: type_69;
    var phi_32983_: vec3<f32>;
    var phi_32791_: f32;
    var phi_32712_: vec3<f32>;
    var phi_32635_: vec3<f32>;
    var phi_32556_: u32;
    var phi_32474_: f32;
    var phi_32397_: u32;
    var phi_32388_: u32;
    var phi_14587_: type_69;
    var phi_14588_: bool;
    var phi_34320_: bool;
    var phi_32387_: u32;
    var phi_14599_: vec3<f32>;
    var phi_14602_: type_39;
    var phi_14604_: u32;
    var phi_14606_: u32;
    var phi_17295_: u32;
    var phi_17298_: u32;
    var phi_17296_: u32;
    var phi_17299_: u32;
    var local_78: u32;
    var phi_17336_: u32;
    var phi_17339_: u32;
    var phi_17337_: u32;
    var phi_17340_: u32;
    var local_79: u32;
    var local_80: u32;
    var phi_17416_: f32;
    var local_81: u32;
    var local_82: u32;
    var phi_17465_: f32;
    var phi_17621_: f32;
    var phi_17628_: vec3<f32>;
    var phi_34684_: u32;
    var phi_17630_: f32;
    var phi_17631_: vec3<f32>;
    var phi_17632_: vec3<f32>;
    var phi_16768_: u32;
    var phi_16771_: u32;
    var phi_16769_: u32;
    var phi_16772_: u32;
    var local_83: u32;
    var local_84: u32;
    var phi_17164_: f32;
    var phi_17167_: f32;
    var phi_26704_: bool;
    var phi_26685_: f32;
    var phi_17168_: f32;
    var phi_26719_: bool;
    var phi_26742_: bool;
    var phi_26757_: bool;
    var phi_16919_: f32;
    var phi_26772_: bool;
    var phi_26787_: bool;
    var phi_26802_: bool;
    var phi_26867_: bool;
    var phi_26955_: bool;
    var phi_26936_: f32;
    var phi_16984_: type_72;
    var phi_17170_: type_72;
    var phi_17248_: bool;
    var phi_17257_: vec3<f32>;
    var phi_17259_: vec3<f32>;
    var phi_17260_: bool;
    var phi_17265_: f32;
    var phi_17266_: vec3<f32>;
    var phi_34692_: u32;
    var phi_17267_: f32;
    var phi_17268_: vec3<f32>;
    var phi_17269_: vec3<f32>;
    var phi_17270_: f32;
    var phi_34691_: u32;
    var phi_17271_: f32;
    var phi_17272_: vec3<f32>;
    var phi_17273_: vec3<f32>;
    var phi_17274_: f32;
    var phi_34683_: u32;
    var phi_17633_: f32;
    var phi_17634_: vec3<f32>;
    var phi_17635_: vec3<f32>;
    var phi_17636_: f32;
    var phi_26970_: bool;
    var phi_27080_: f32;
    var phi_27108_: f32;
    var phi_27136_: f32;
    var phi_27270_: bool;
    var phi_27285_: bool;
    var phi_27300_: bool;
    var phi_27327_: bool;
    var phi_27342_: bool;
    var phi_27357_: bool;
    var phi_27384_: bool;
    var phi_27399_: bool;
    var phi_27414_: bool;
    var phi_14940_: f32;
    var phi_14948_: f32;
    var phi_14956_: f32;
    var phi_27611_: bool;
    var phi_27600_: f32;
    var phi_15063_: vec3<f32>;
    var phi_27639_: bool;
    var phi_15064_: vec3<f32>;
    var phi_15076_: type_57;
    var phi_15077_: type_57;
    var phi_15078_: bool;
    var phi_15083_: type_57;
    var phi_27676_: bool;
    var phi_27706_: bool;
    var phi_27695_: f32;
    var phi_27745_: vec3<f32>;
    var phi_15111_: type_57;
    var phi_15112_: type_57;
    var phi_27767_: vec3<f32>;
    var phi_15113_: type_57;
    var phi_15114_: type_57;
    var phi_15115_: bool;
    var phi_15119_: type_57;
    var phi_27799_: bool;
    var phi_27870_: bool;
    var phi_15939_: f32;
    var phi_27885_: bool;
    var phi_27900_: bool;
    var phi_16412_: vec3<f32>;
    var phi_16413_: vec3<f32>;
    var phi_16414_: bool;
    var phi_16419_: vec3<f32>;
    var phi_16420_: bool;
    var phi_28024_: bool;
    var phi_28039_: bool;
    var phi_16711_: vec3<f32>;
    var phi_16713_: vec3<f32>;
    var phi_16740_: vec3<f32>;
    var phi_16741_: vec3<f32>;
    var phi_15557_: vec3<f32>;
    var phi_15558_: vec3<f32>;
    var phi_15559_: bool;
    var phi_15564_: vec3<f32>;
    var phi_15565_: bool;
    var phi_28201_: bool;
    var phi_28216_: bool;
    var phi_15856_: vec3<f32>;
    var phi_15858_: vec3<f32>;
    var phi_34726_: u32;
    var phi_16742_: f32;
    var phi_16743_: vec3<f32>;
    var phi_16744_: vec3<f32>;
    var phi_34725_: u32;
    var phi_16745_: f32;
    var phi_16746_: vec3<f32>;
    var phi_16747_: vec3<f32>;
    var phi_34682_: u32;
    var phi_17637_: f32;
    var phi_17638_: vec3<f32>;
    var phi_17639_: vec3<f32>;
    var phi_17640_: f32;
    var phi_14702_: f32;
    var phi_14703_: f32;
    var phi_34681_: u32;
    var phi_17641_: f32;
    var phi_17642_: vec3<f32>;
    var phi_17643_: vec3<f32>;
    var phi_17644_: f32;
    var phi_28269_: bool;
    var phi_28379_: f32;
    var phi_28407_: f32;
    var phi_28435_: f32;
    var phi_28569_: bool;
    var phi_28584_: bool;
    var phi_28599_: bool;
    var phi_28626_: bool;
    var phi_28641_: bool;
    var phi_28656_: bool;
    var phi_28683_: bool;
    var phi_28698_: bool;
    var phi_28713_: bool;
    var phi_17867_: f32;
    var phi_17875_: f32;
    var phi_17883_: f32;
    var phi_28910_: bool;
    var phi_28899_: f32;
    var phi_17990_: vec3<f32>;
    var phi_28938_: bool;
    var phi_17991_: vec3<f32>;
    var phi_18003_: type_57;
    var phi_18004_: type_57;
    var phi_18005_: bool;
    var phi_18010_: type_57;
    var phi_28975_: bool;
    var phi_29005_: bool;
    var phi_28994_: f32;
    var phi_29044_: vec3<f32>;
    var phi_18038_: type_57;
    var phi_18039_: type_57;
    var phi_29066_: vec3<f32>;
    var phi_18040_: type_57;
    var phi_18041_: type_57;
    var phi_18042_: bool;
    var phi_18046_: type_57;
    var phi_29080_: f32;
    var phi_29083_: f32;
    var phi_18055_: f32;
    var phi_18089_: u32;
    var phi_18092_: u32;
    var phi_18094_: u32;
    var phi_18098_: type_64;
    var phi_29180_: bool;
    var phi_29210_: bool;
    var phi_29157_: type_54;
    var phi_29160_: type_54;
    var phi_29161_: bool;
    var phi_29166_: type_54;
    var phi_29267_: bool;
    var phi_29282_: bool;
    var phi_29297_: bool;
    var phi_29312_: bool;
    var phi_29244_: type_54;
    var phi_29247_: type_54;
    var phi_29248_: bool;
    var phi_29253_: type_54;
    var phi_29354_: bool;
    var phi_29369_: bool;
    var phi_29384_: bool;
    var phi_29399_: bool;
    var phi_29331_: type_54;
    var phi_29334_: type_54;
    var phi_29335_: bool;
    var phi_29340_: type_54;
    var phi_29133_: bool;
    var phi_29136_: bool;
    var phi_29137_: bool;
    var phi_18505_: u32;
    var phi_18506_: u32;
    var phi_18507_: u32;
    var phi_18508_: u32;
    var phi_18509_: u32;
    var phi_18510_: u32;
    var phi_18511_: u32;
    var phi_18512_: u32;
    var phi_18513_: u32;
    var phi_18394_: vec4<f32>;
    var phi_18395_: vec4<f32>;
    var phi_18396_: bool;
    var phi_18400_: vec4<f32>;
    var phi_18401_: bool;
    var phi_18306_: bool;
    var phi_18343_: bool;
    var phi_18349_: f32;
    var phi_18359_: vec4<f32>;
    var phi_18360_: vec4<f32>;
    var phi_18361_: bool;
    var phi_18365_: vec4<f32>;
    var phi_18366_: bool;
    var phi_18370_: vec4<f32>;
    var phi_18371_: bool;
    var phi_18402_: vec4<f32>;
    var phi_18403_: bool;
    var phi_18235_: vec4<f32>;
    var phi_18236_: vec4<f32>;
    var phi_18237_: bool;
    var phi_18241_: vec4<f32>;
    var phi_18242_: bool;
    var phi_18243_: vec4<f32>;
    var phi_18244_: bool;
    var phi_18245_: bool;
    var phi_18253_: vec4<f32>;
    var phi_18254_: bool;
    var phi_18255_: vec4<f32>;
    var phi_18256_: bool;
    var phi_18257_: bool;
    var phi_18265_: vec4<f32>;
    var phi_18266_: bool;
    var phi_18412_: vec4<f32>;
    var phi_18413_: bool;
    var phi_18434_: type_64;
    var phi_18436_: type_64;
    var phi_18442_: u32;
    var phi_18090_: u32;
    var phi_18093_: u32;
    var phi_18095_: u32;
    var phi_18099_: type_64;
    var phi_18515_: type_64;
    var phi_18516_: bool;
    var phi_18517_: bool;
    var phi_34240_: bool;
    var phi_20379_: bool;
    var phi_20378_: type_64;
    var phi_18689_: type_64;
    var phi_18694_: type_64;
    var phi_18746_: vec3<f32>;
    var phi_18747_: type_39;
    var phi_18748_: vec3<f32>;
    var phi_18749_: type_39;
    var phi_34628_: bool;
    var phi_18750_: vec3<f32>;
    var phi_18751_: type_39;
    var phi_18752_: u32;
    var phi_34627_: bool;
    var phi_18753_: vec3<f32>;
    var phi_18754_: type_39;
    var phi_18755_: u32;
    var phi_34870_: u32;
    var phi_34626_: bool;
    var phi_14600_: vec3<f32>;
    var phi_14603_: type_39;
    var phi_14605_: u32;
    var phi_14607_: u32;
    var phi_34639_: u32;
    var phi_34625_: bool;
    var phi_29550_: bool;
    var phi_29580_: bool;
    var phi_29569_: f32;
    var phi_29500_: bool;
    var phi_29636_: vec3<f32>;
    var phi_29792_: bool;
    var phi_29807_: bool;
    var phi_29822_: bool;
    var phi_29731_: vec3<f32>;
    var phi_29734_: vec3<f32>;
    var phi_29735_: bool;
    var phi_29505_: vec3<f32>;
    var phi_29509_: vec3<f32>;
    var phi_29510_: bool;
    var phi_29922_: bool;
    var phi_29896_: vec3<f32>;
    var phi_29517_: vec3<f32>;
    var phi_29937_: bool;
    var phi_30047_: f32;
    var phi_30075_: f32;
    var phi_30103_: f32;
    var phi_30237_: bool;
    var phi_30252_: bool;
    var phi_30267_: bool;
    var phi_30294_: bool;
    var phi_30309_: bool;
    var phi_30324_: bool;
    var phi_30351_: bool;
    var phi_30366_: bool;
    var phi_30381_: bool;
    var phi_18974_: f32;
    var phi_18982_: f32;
    var phi_18990_: f32;
    var phi_30578_: bool;
    var phi_30567_: f32;
    var phi_19097_: vec3<f32>;
    var phi_30606_: bool;
    var phi_19098_: vec3<f32>;
    var phi_19110_: type_57;
    var phi_19111_: type_57;
    var phi_19112_: bool;
    var phi_19117_: type_57;
    var phi_30643_: bool;
    var phi_30673_: bool;
    var phi_30662_: f32;
    var phi_30712_: vec3<f32>;
    var phi_19145_: type_57;
    var phi_19146_: type_57;
    var phi_30734_: vec3<f32>;
    var phi_19147_: type_57;
    var phi_19148_: type_57;
    var phi_19149_: bool;
    var phi_19153_: type_57;
    var phi_30759_: bool;
    var phi_19229_: vec3<f32>;
    var phi_36129_: u32;
    var phi_19231_: vec3<f32>;
    var phi_19232_: bool;
    var phi_36128_: u32;
    var phi_19233_: vec3<f32>;
    var phi_19234_: bool;
    var phi_36126_: u32;
    var phi_19240_: vec3<f32>;
    var phi_19241_: bool;
    var local_85: u32;
    var local_86: vec3<f32>;
    var local_87: vec3<f32>;
    var local_88: vec3<f32>;
    var local_89: type_39;
    var phi_19270_: vec3<f32>;
    var local_90: u32;
    var local_91: vec3<f32>;
    var local_92: vec3<f32>;
    var local_93: vec3<f32>;
    var local_94: type_39;
    var phi_19286_: type_69;
    var phi_19287_: type_69;
    var phi_36133_: u32;
    var phi_19289_: type_69;
    var phi_19290_: type_69;
    var phi_19291_: bool;
    var phi_36132_: u32;
    var phi_19292_: type_69;
    var phi_19293_: type_69;
    var phi_19294_: bool;
    var phi_19295_: bool;
    var local_95: u32;
    var local_96: vec3<f32>;
    var local_97: vec3<f32>;
    var local_98: vec3<f32>;
    var local_99: type_39;
    var phi_19301_: type_69;
    var local_100: u32;
    var local_101: vec3<f32>;
    var local_102: vec3<f32>;
    var local_103: vec3<f32>;
    var local_104: type_39;
    var phi_19306_: type_69;
    var phi_19307_: type_69;
    var phi_19308_: bool;
    var phi_19312_: type_69;
    var phi_19313_: type_69;
    var phi_19314_: bool;
    var phi_13637_: bool;
    var phi_13646_: vec3<f32>;
    var phi_13648_: vec3<f32>;
    var phi_13649_: bool;
    var phi_13810_: f32;
    var phi_13813_: f32;
    var phi_30980_: bool;
    var phi_30961_: f32;
    var phi_13814_: f32;
    var phi_30998_: f32;
    var phi_31001_: f32;
    var phi_13817_: f32;
    var phi_13818_: f32;
    var phi_13819_: f32;
    var phi_19318_: type_69;
    var phi_19319_: type_69;
    var phi_19320_: bool;
    var phi_12073_: f32;
    var phi_31048_: f32;
    var phi_31051_: f32;
    var phi_12075_: f32;
    var phi_12076_: f32;
    var phi_31077_: bool;
    var phi_12127_: f32;
    var phi_31095_: f32;
    var phi_31098_: f32;
    var phi_12129_: f32;
    var phi_31109_: bool;
    var phi_31124_: bool;
    var phi_12598_: vec3<f32>;
    var phi_12599_: vec3<f32>;
    var phi_12600_: bool;
    var phi_12605_: vec3<f32>;
    var phi_12606_: bool;
    var phi_31248_: bool;
    var phi_31263_: bool;
    var phi_12897_: vec3<f32>;
    var phi_12899_: vec3<f32>;
    var phi_12926_: vec3<f32>;
    var phi_12940_: vec3<f32>;
    var phi_19324_: type_69;
    var phi_19325_: type_69;
    var phi_19326_: bool;
    var phi_19330_: type_69;
    var phi_19368_: bool;
    var phi_19375_: type_35;
    var phi_11213_: bool;
    var phi_11214_: bool;
    var phi_11215_: bool;
    var phi_11236_: vec4<f32>;
    var phi_11237_: type_35;
    var phi_19376_: vec4<f32>;
    var phi_19377_: type_39;
    var phi_19378_: type_39;
    var phi_19379_: type_35;
    var local_105: type_64;
    var local_106: type_64;

    switch bitcast<i32>(0u) {
        default: {
            let _e273 = global;
            let _e285 = arrayLength((&global_8.member));
            let _e289 = (_e273.x + (_e273.y * 4194240u));
            let _e293 = global_1.member.member[0u];
            let _e297 = global_1.member.member[1u];
            if (_e289 >= (_e293 * _e297)) {
                phi_11130_ = type_45(0u, type_45().member_1);
            } else {
                phi_11130_ = type_45(1u, _e289);
            }
            let _e305 = phi_11130_;
            if (bitcast<i32>(_e305.member) != 0i) {
                let _e310 = (_e305.member_1 < arrayLength((&global_7.member)));
                if _e310 {
                } else {
                    break;
                }
                let _e317 = global_7.member[_e305.member_1].member_4[1u];
                if ((_e317 & 9u) == 8u) {
                } else {
                    if _e310 {
                    } else {
                        break;
                    }
                    let _e320 = global_7.member[_e305.member_1];
                    if ((_e320.member_4.y & 1u) == 0u) {
                        if ((_e320.member_4.y & 8u) == 0u) {
                            let _e7345 = (_e320.member_3.x * _e320.member.w);
                            let _e7346 = (_e320.member_3.y * _e320.member.w);
                            let _e7347 = (_e320.member_3.z * _e320.member.w);
                            if (_e7345 == _e7345) {
                                let _e7350 = (_e7346 == _e7346);
                                if _e7350 {
                                    phi_11213_ = select(true, false, (_e7347 == _e7347));
                                } else {
                                    phi_11213_ = false;
                                }
                                let _e7354 = phi_11213_;
                                phi_11214_ = select(true, false, _e7350);
                                phi_11215_ = _e7354;
                            } else {
                                phi_11214_ = true;
                                phi_11215_ = false;
                            }
                            let _e7357 = phi_11214_;
                            let _e7359 = phi_11215_;
                            let _e7362 = select(vec3<f32>(_e7345, _e7346, _e7347), vec3<f32>(0f, 0f, 0f), vec3(select(_e7359, true, _e7357)));
                            phi_11236_ = vec4<f32>(_e7362.x, _e7362.y, _e7362.z, _e320.member_3.w);
                            phi_11237_ = type_35(_e320.member, _e320.member_1, _e320.member_2, _e320.member_3, vec4<u32>(_e320.member_4.x, (_e320.member_4.y | 8u), _e320.member_4.z, _e320.member_4.w));
                        } else {
                            phi_11236_ = vec4<f32>(0f, 0f, 0f, 0f);
                            phi_11237_ = _e320;
                        }
                        let _e7381 = phi_11236_;
                        let _e7383 = phi_11237_;
                        phi_19376_ = _e7381;
                        phi_19377_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        phi_19378_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        phi_19379_ = _e7383;
                    } else {
                        let _e336 = vec3<f32>(_e320.member_1.x, _e320.member_1.y, _e320.member_1.z);
                        let _e341 = vec3<f32>(_e320.member_2.x, _e320.member_2.y, _e320.member_2.z);
                        let _e347 = select(select(u32(_e320.member_2.w), 0u, (_e320.member_2.w < 0f)), 4294967295u, (_e320.member_2.w > 4294967000f));
                        let _e351 = vec3<f32>(_e320.member_3.x, _e320.member_3.y, _e320.member_3.z);
                        let _e360 = ((_e320.member_4.y & 2u) != 0u);
                        let _e363 = global_1.member.member[3u];
                        let _e366 = global_1.member.member_3[3u];
                        let _e369 = global_1.member.member_1[2u];
                        let _e372 = global_1.member.member_1[0u];
                        let _e375 = global_1.member.member_1[1u];
                        let _e376 = (_e372 + _e375);
                        let _e377 = (vec3<f32>(1f, 1f, 1f) / _e336);
                        local_68 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                        let _e379 = select(0u, 1u, (_e372 > 0u));
                        phi_11333_ = _e372;
                        phi_11336_ = _e379;
                        phi_11338_ = _e376;
                        phi_11340_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_11342_ = type_64(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                        loop {
                            let _e381 = phi_11333_;
                            let _e383 = phi_11336_;
                            let _e385 = phi_11338_;
                            let _e387 = phi_11340_;
                            let _e389 = phi_11342_;
                            local_69 = _e389;
                            local_70 = _e387;
                            local_71 = _e387;
                            local_72 = _e387;
                            local_73 = _e389;
                            local_105 = _e389;
                            if (_e381 < _e385) {
                                let _e558 = global_2.member[_e381];
                                if (_e558.member_6.x == 0u) {
                                    let _e664 = (_e558.member_1.x - _e558.member.x);
                                    let _e665 = (_e558.member_1.y - _e558.member.y);
                                    let _e666 = (_e558.member_1.z - _e558.member.z);
                                    let _e671 = (_e558.member_2.x - _e558.member.x);
                                    let _e672 = (_e558.member_2.y - _e558.member.y);
                                    let _e673 = (_e558.member_2.z - _e558.member.z);
                                    let _e676 = ((_e320.member_1.y * _e673) - (_e672 * _e320.member_1.z));
                                    let _e679 = ((_e320.member_1.z * _e671) - (_e673 * _e320.member_1.x));
                                    let _e682 = ((_e320.member_1.x * _e672) - (_e671 * _e320.member_1.y));
                                    let _e687 = (((_e664 * _e676) + (_e665 * _e679)) + (_e666 * _e682));
                                    if (abs(_e687) < 0.000000000001f) {
                                        phi_11500_ = vec4<f32>();
                                        phi_11501_ = true;
                                    } else {
                                        let _e690 = (1f / _e687);
                                        let _e691 = (_e320.member.x - _e558.member.x);
                                        let _e692 = (_e320.member.y - _e558.member.y);
                                        let _e693 = (_e320.member.z - _e558.member.z);
                                        let _e699 = ((((_e691 * _e676) + (_e692 * _e679)) + (_e693 * _e682)) * _e690);
                                        if (_e699 < 0f) {
                                            phi_11490_ = vec4<f32>();
                                            phi_11491_ = true;
                                            phi_11492_ = false;
                                        } else {
                                            let _e701 = (_e699 > 1f);
                                            if _e701 {
                                                phi_11488_ = vec4<f32>();
                                                phi_11489_ = false;
                                            } else {
                                                let _e704 = ((_e692 * _e666) - (_e665 * _e693));
                                                let _e707 = ((_e693 * _e664) - (_e666 * _e691));
                                                let _e710 = ((_e691 * _e665) - (_e664 * _e692));
                                                let _e716 = ((((_e320.member_1.x * _e704) + (_e320.member_1.y * _e707)) + (_e320.member_1.z * _e710)) * _e690);
                                                if (_e716 < 0f) {
                                                    phi_11478_ = vec4<f32>();
                                                    phi_11479_ = true;
                                                    phi_11480_ = false;
                                                } else {
                                                    let _e719 = ((_e699 + _e716) > 1f);
                                                    if _e719 {
                                                        phi_11476_ = vec4<f32>();
                                                        phi_11477_ = false;
                                                    } else {
                                                        let _e725 = ((((_e671 * _e704) + (_e672 * _e707)) + (_e673 * _e710)) * _e690);
                                                        if (_e725 <= 0f) {
                                                            phi_11471_ = vec4<f32>();
                                                            phi_11472_ = true;
                                                        } else {
                                                            let _e727 = (_e725 >= _e389.member);
                                                            if _e727 {
                                                                phi_11470_ = vec4<f32>();
                                                            } else {
                                                                phi_11470_ = vec4<f32>(_e725, _e699, _e716, 1f);
                                                            }
                                                            let _e730 = phi_11470_;
                                                            phi_11471_ = _e730;
                                                            phi_11472_ = _e727;
                                                        }
                                                        let _e732 = phi_11471_;
                                                        let _e734 = phi_11472_;
                                                        phi_11476_ = _e732;
                                                        phi_11477_ = _e734;
                                                    }
                                                    let _e736 = phi_11476_;
                                                    let _e738 = phi_11477_;
                                                    phi_11478_ = _e736;
                                                    phi_11479_ = _e719;
                                                    phi_11480_ = _e738;
                                                }
                                                let _e740 = phi_11478_;
                                                let _e742 = phi_11479_;
                                                let _e744 = phi_11480_;
                                                phi_11488_ = _e740;
                                                phi_11489_ = select(_e744, true, _e742);
                                            }
                                            let _e747 = phi_11488_;
                                            let _e749 = phi_11489_;
                                            phi_11490_ = _e747;
                                            phi_11491_ = _e701;
                                            phi_11492_ = _e749;
                                        }
                                        let _e751 = phi_11490_;
                                        let _e753 = phi_11491_;
                                        let _e755 = phi_11492_;
                                        phi_11500_ = _e751;
                                        phi_11501_ = select(_e755, true, _e753);
                                    }
                                    let _e758 = phi_11500_;
                                    let _e760 = phi_11501_;
                                    phi_11639_ = _e758;
                                    phi_11640_ = _e760;
                                } else {
                                    if (_e558.member_6.x == 1u) {
                                        let _e586 = (_e320.member.x - _e558.member.x);
                                        let _e587 = (_e320.member.y - _e558.member.y);
                                        let _e588 = (_e320.member.z - _e558.member.z);
                                        let _e593 = (((_e586 * _e320.member_1.x) + (_e587 * _e320.member_1.y)) + (_e588 * _e320.member_1.z));
                                        if (_e593 > 0f) {
                                            phi_11538_ = select(true, false, ((((_e586 * _e586) + (_e587 * _e587)) + (_e588 * _e588)) > (_e558.member.w * _e558.member.w)));
                                        } else {
                                            phi_11538_ = true;
                                        }
                                        let _e606 = phi_11538_;
                                        if _e606 {
                                            let _e610 = (_e586 - (_e320.member_1.x * _e593));
                                            let _e611 = (_e587 - (_e320.member_1.y * _e593));
                                            let _e612 = (_e588 - (_e320.member_1.z * _e593));
                                            let _e621 = ((_e558.member.w * _e558.member.w) - (((_e610 * _e610) + (_e611 * _e611)) + (_e612 * _e612)));
                                            if (_e621 < 0f) {
                                                phi_11594_ = vec4<f32>();
                                                phi_11595_ = true;
                                            } else {
                                                let _e623 = sqrt(_e621);
                                                let _e624 = -(_e593);
                                                let _e625 = (_e624 - _e623);
                                                if (_e625 <= 0f) {
                                                    phi_11572_ = true;
                                                } else {
                                                    phi_11572_ = (_e625 >= _e389.member);
                                                }
                                                let _e629 = phi_11572_;
                                                if _e629 {
                                                    phi_11578_ = (_e624 + _e623);
                                                } else {
                                                    phi_11578_ = _e625;
                                                }
                                                let _e632 = phi_11578_;
                                                if (_e632 <= 0f) {
                                                    phi_11589_ = vec4<f32>();
                                                    phi_11590_ = true;
                                                } else {
                                                    let _e634 = (_e632 >= _e389.member);
                                                    if _e634 {
                                                        phi_11588_ = vec4<f32>();
                                                    } else {
                                                        phi_11588_ = vec4<f32>(_e632, 0f, 0f, 1f);
                                                    }
                                                    let _e637 = phi_11588_;
                                                    phi_11589_ = _e637;
                                                    phi_11590_ = _e634;
                                                }
                                                let _e639 = phi_11589_;
                                                let _e641 = phi_11590_;
                                                phi_11594_ = _e639;
                                                phi_11595_ = _e641;
                                            }
                                            let _e643 = phi_11594_;
                                            let _e645 = phi_11595_;
                                            phi_11599_ = _e643;
                                            phi_11600_ = _e645;
                                        } else {
                                            phi_11599_ = vec4<f32>();
                                            phi_11600_ = true;
                                        }
                                        let _e647 = phi_11599_;
                                        let _e649 = phi_11600_;
                                        phi_11629_ = _e647;
                                        phi_11630_ = _e649;
                                    } else {
                                        if (abs(_e320.member_1.y) < 0.000000000001f) {
                                            phi_11627_ = vec4<f32>();
                                            phi_11628_ = true;
                                        } else {
                                            let _e568 = ((_e558.member.x - _e320.member.y) / _e320.member_1.y);
                                            if (_e568 <= 0f) {
                                                phi_11622_ = vec4<f32>();
                                                phi_11623_ = true;
                                            } else {
                                                let _e570 = (_e568 >= _e389.member);
                                                if _e570 {
                                                    phi_11621_ = vec4<f32>();
                                                } else {
                                                    phi_11621_ = vec4<f32>(_e568, 0f, 0f, 1f);
                                                }
                                                let _e573 = phi_11621_;
                                                phi_11622_ = _e573;
                                                phi_11623_ = _e570;
                                            }
                                            let _e575 = phi_11622_;
                                            let _e577 = phi_11623_;
                                            phi_11627_ = _e575;
                                            phi_11628_ = _e577;
                                        }
                                        let _e579 = phi_11627_;
                                        let _e581 = phi_11628_;
                                        phi_11629_ = _e579;
                                        phi_11630_ = _e581;
                                    }
                                    let _e651 = phi_11629_;
                                    let _e653 = phi_11630_;
                                    phi_11639_ = _e651;
                                    phi_11640_ = select(true, false, select(true, false, _e653));
                                }
                                let _e762 = phi_11639_;
                                let _e764 = phi_11640_;
                                let _e767 = select(vec4<f32>(0f, 0f, 0f, 0f), _e762, vec4(select(true, false, _e764)));
                                let _e769 = (_e767.w > 0f);
                                if _e769 {
                                    let _e775 = type_64(_e767.x, _e389.member_1, _e389.member_2, _e389.member_3);
                                    let _e780 = type_64(_e775.member, _e381, _e775.member_2, _e775.member_3);
                                    phi_11661_ = _e780;
                                    phi_11663_ = _e780;
                                } else {
                                    phi_11661_ = type_64();
                                    phi_11663_ = _e389;
                                }
                                let _e782 = phi_11661_;
                                let _e784 = phi_11663_;
                                phi_11334_ = (_e381 + 1u);
                                phi_11337_ = _e383;
                                phi_11339_ = _e385;
                                phi_11341_ = select(_e387, _e767, vec4(_e769));
                                phi_11343_ = _e784;
                                phi_11742_ = _e782;
                                phi_11743_ = true;
                                phi_11744_ = false;
                            } else {
                                let _e391 = (_e383 == 0u);
                                if _e391 {
                                    phi_11738_ = u32();
                                    phi_11739_ = u32();
                                    phi_11740_ = u32();
                                } else {
                                    let _e392 = (_e383 - 1u);
                                    let _e393 = (_e392 < 100u);
                                    if _e393 {
                                    } else {
                                        phi_32044_ = true;
                                        phi_20377_ = bool();
                                        phi_20376_ = type_64();
                                        break;
                                    }
                                    let _e395 = local_68[_e392];
                                    let _e397 = global_3.member[_e395];
                                    let _e408 = ((_e397.member.x - _e320.member.x) * _e377.x);
                                    let _e410 = ((_e397.member.y - _e320.member.y) * _e377.y);
                                    let _e412 = ((_e397.member.z - _e320.member.z) * _e377.z);
                                    let _e419 = ((_e397.member_2.x - _e320.member.x) * _e377.x);
                                    let _e420 = ((_e397.member_2.y - _e320.member.y) * _e377.y);
                                    let _e421 = ((_e397.member_2.z - _e320.member.z) * _e377.z);
                                    if (_e408 == _e408) {
                                        let _e423 = (_e419 == _e419);
                                        if _e423 {
                                            let _e424 = (_e408 != _e408);
                                            if _e424 {
                                                phi_24779_ = true;
                                            } else {
                                                phi_24779_ = (_e419 <= _e408);
                                            }
                                            let _e427 = phi_24779_;
                                            let _e428 = select(_e408, _e419, _e427);
                                            if _e424 {
                                                phi_24809_ = true;
                                            } else {
                                                phi_24809_ = (_e419 >= _e408);
                                            }
                                            let _e433 = phi_24809_;
                                            let _e434 = select(_e408, _e419, _e433);
                                            phi_24756_ = type_54(select(-300000000000000000000000000000000000000f, _e428, (_e428 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e434, (_e434 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_24756_ = type_54();
                                        }
                                        let _e439 = phi_24756_;
                                        phi_24759_ = _e439;
                                        phi_24760_ = select(true, false, _e423);
                                    } else {
                                        phi_24759_ = type_54();
                                        phi_24760_ = true;
                                    }
                                    let _e442 = phi_24759_;
                                    let _e444 = phi_24760_;
                                    if _e444 {
                                        phi_24765_ = type_54(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_24765_ = _e442;
                                    }
                                    let _e446 = phi_24765_;
                                    if (_e410 == _e410) {
                                        let _e450 = (_e420 == _e420);
                                        if _e450 {
                                            let _e451 = (_e410 != _e410);
                                            if _e451 {
                                                phi_24866_ = true;
                                            } else {
                                                phi_24866_ = (_e420 <= _e410);
                                            }
                                            let _e454 = phi_24866_;
                                            let _e455 = select(_e410, _e420, _e454);
                                            if (_e446.member != _e446.member) {
                                                phi_24881_ = true;
                                            } else {
                                                phi_24881_ = (_e455 >= _e446.member);
                                            }
                                            let _e459 = phi_24881_;
                                            if _e451 {
                                                phi_24896_ = true;
                                            } else {
                                                phi_24896_ = (_e420 >= _e410);
                                            }
                                            let _e463 = phi_24896_;
                                            let _e464 = select(_e410, _e420, _e463);
                                            if (_e446.member_1 != _e446.member_1) {
                                                phi_24911_ = true;
                                            } else {
                                                phi_24911_ = (_e464 <= _e446.member_1);
                                            }
                                            let _e468 = phi_24911_;
                                            phi_24843_ = type_54(select(_e446.member, _e455, _e459), select(_e446.member_1, _e464, _e468));
                                        } else {
                                            phi_24843_ = type_54();
                                        }
                                        let _e472 = phi_24843_;
                                        phi_24846_ = _e472;
                                        phi_24847_ = select(true, false, _e450);
                                    } else {
                                        phi_24846_ = type_54();
                                        phi_24847_ = true;
                                    }
                                    let _e475 = phi_24846_;
                                    let _e477 = phi_24847_;
                                    if _e477 {
                                        phi_24852_ = _e446;
                                    } else {
                                        phi_24852_ = _e475;
                                    }
                                    let _e479 = phi_24852_;
                                    if (_e412 == _e412) {
                                        let _e483 = (_e421 == _e421);
                                        if _e483 {
                                            let _e484 = (_e412 != _e412);
                                            if _e484 {
                                                phi_24953_ = true;
                                            } else {
                                                phi_24953_ = (_e421 <= _e412);
                                            }
                                            let _e487 = phi_24953_;
                                            let _e488 = select(_e412, _e421, _e487);
                                            if (_e479.member != _e479.member) {
                                                phi_24968_ = true;
                                            } else {
                                                phi_24968_ = (_e488 >= _e479.member);
                                            }
                                            let _e492 = phi_24968_;
                                            if _e484 {
                                                phi_24983_ = true;
                                            } else {
                                                phi_24983_ = (_e421 >= _e412);
                                            }
                                            let _e496 = phi_24983_;
                                            let _e497 = select(_e412, _e421, _e496);
                                            if (_e479.member_1 != _e479.member_1) {
                                                phi_24998_ = true;
                                            } else {
                                                phi_24998_ = (_e497 <= _e479.member_1);
                                            }
                                            let _e501 = phi_24998_;
                                            phi_24930_ = type_54(select(_e479.member, _e488, _e492), select(_e479.member_1, _e497, _e501));
                                        } else {
                                            phi_24930_ = type_54();
                                        }
                                        let _e505 = phi_24930_;
                                        phi_24933_ = _e505;
                                        phi_24934_ = select(true, false, _e483);
                                    } else {
                                        phi_24933_ = type_54();
                                        phi_24934_ = true;
                                    }
                                    let _e508 = phi_24933_;
                                    let _e510 = phi_24934_;
                                    if _e510 {
                                        phi_24939_ = _e479;
                                    } else {
                                        phi_24939_ = _e508;
                                    }
                                    let _e512 = phi_24939_;
                                    if (_e512.member_1 < 0f) {
                                        phi_24735_ = bool();
                                        phi_24736_ = true;
                                    } else {
                                        let _e516 = (_e512.member > _e512.member_1);
                                        if _e516 {
                                            phi_24732_ = bool();
                                        } else {
                                            phi_24732_ = (_e512.member > _e389.member);
                                        }
                                        let _e519 = phi_24732_;
                                        phi_24735_ = _e519;
                                        phi_24736_ = _e516;
                                    }
                                    let _e521 = phi_24735_;
                                    let _e523 = phi_24736_;
                                    if !(select(_e521, true, _e523)) {
                                        if (_e397.member_3 > 0u) {
                                            phi_11732_ = _e397.member_1;
                                            phi_11733_ = _e392;
                                            phi_11734_ = (_e397.member_1 + _e397.member_3);
                                        } else {
                                            let _e528 = (_e383 + 1u);
                                            let _e529 = (_e528 <= 100u);
                                            if _e529 {
                                                if _e393 {
                                                } else {
                                                    phi_32044_ = true;
                                                    phi_20377_ = bool();
                                                    phi_20376_ = type_64();
                                                    break;
                                                }
                                                local_68[_e392] = _e397.member_1;
                                                if (_e383 < 100u) {
                                                } else {
                                                    phi_32044_ = true;
                                                    phi_20377_ = bool();
                                                    phi_20376_ = type_64();
                                                    break;
                                                }
                                                local_68[_e383] = (_e395 + 1u);
                                            }
                                            phi_11732_ = _e381;
                                            phi_11733_ = select(_e392, _e528, _e529);
                                            phi_11734_ = _e385;
                                        }
                                        let _e538 = phi_11732_;
                                        let _e540 = phi_11733_;
                                        let _e542 = phi_11734_;
                                        phi_11735_ = _e538;
                                        phi_11736_ = _e540;
                                        phi_11737_ = _e542;
                                    } else {
                                        phi_11735_ = _e381;
                                        phi_11736_ = _e392;
                                        phi_11737_ = _e385;
                                    }
                                    let _e544 = phi_11735_;
                                    let _e546 = phi_11736_;
                                    let _e548 = phi_11737_;
                                    phi_11738_ = _e544;
                                    phi_11739_ = _e546;
                                    phi_11740_ = _e548;
                                }
                                let _e550 = phi_11738_;
                                let _e552 = phi_11739_;
                                let _e554 = phi_11740_;
                                phi_11334_ = _e550;
                                phi_11337_ = _e552;
                                phi_11339_ = _e554;
                                phi_11341_ = _e387;
                                phi_11343_ = _e389;
                                phi_11742_ = type_64();
                                phi_11743_ = select(true, false, _e391);
                                phi_11744_ = _e391;
                            }
                            let _e789 = phi_11334_;
                            let _e791 = phi_11337_;
                            let _e793 = phi_11339_;
                            let _e795 = phi_11341_;
                            let _e797 = phi_11343_;
                            let _e799 = phi_11742_;
                            let _e801 = phi_11743_;
                            let _e803 = phi_11744_;
                            phi_11333_ = _e789;
                            phi_11336_ = _e791;
                            phi_11338_ = _e793;
                            phi_11340_ = _e795;
                            phi_11342_ = _e797;
                            phi_32044_ = false;
                            phi_20377_ = _e803;
                            phi_20376_ = _e799;
                            if !(_e801) {
                                break;
                            }
                        }
                        let _e806 = phi_32044_;
                        let _e808 = phi_20377_;
                        let _e810 = phi_20376_;
                        if _e806 {
                            break;
                        }
                        if _e808 {
                            let _e812 = local_69;
                            if (_e812.member_1 != 4294967295u) {
                                let _e816 = global_2.member[_e812.member_1];
                                if (_e816.member_6.x == 0u) {
                                    let _e858 = (_e816.member_1.x - _e816.member.x);
                                    let _e859 = (_e816.member_1.y - _e816.member.y);
                                    let _e860 = (_e816.member_1.z - _e816.member.z);
                                    let _e865 = (_e816.member_2.x - _e816.member.x);
                                    let _e866 = (_e816.member_2.y - _e816.member.y);
                                    let _e867 = (_e816.member_2.z - _e816.member.z);
                                    let _e870 = ((_e859 * _e867) - (_e866 * _e860));
                                    let _e873 = ((_e860 * _e865) - (_e867 * _e858));
                                    let _e876 = ((_e858 * _e866) - (_e865 * _e859));
                                    let _e883 = (1f / sqrt((((_e870 * _e870) + (_e873 * _e873)) + (_e876 * _e876))));
                                    let _e887 = vec3<f32>((_e870 * _e883), (_e873 * _e883), (_e876 * _e883));
                                    if (_e816.member_6.w == 1u) {
                                        let _e896 = local_71;
                                        let _e900 = local_72;
                                        let _e902 = ((1f - _e896.y) - _e900.z);
                                        let _e923 = (((_e816.member_3.x * _e902) + (_e816.member_4.x * _e896.y)) + (_e816.member_5.x * _e900.z));
                                        let _e924 = (((_e816.member_3.y * _e902) + (_e816.member_4.y * _e896.y)) + (_e816.member_5.y * _e900.z));
                                        let _e925 = (((_e816.member_3.z * _e902) + (_e816.member_4.z * _e896.y)) + (_e816.member_5.z * _e900.z));
                                        let _e930 = (((_e923 * _e923) + (_e924 * _e924)) + (_e925 * _e925));
                                        if (_e930 > 0f) {
                                            let _e933 = (1f / sqrt(_e930));
                                            phi_11871_ = vec3<f32>((_e923 * _e933), (_e924 * _e933), (_e925 * _e933));
                                        } else {
                                            phi_11871_ = _e887;
                                        }
                                        let _e939 = phi_11871_;
                                        phi_11872_ = _e939;
                                    } else {
                                        phi_11872_ = _e887;
                                    }
                                    let _e941 = phi_11872_;
                                    phi_11906_ = type_58(_e887, _e941);
                                } else {
                                    if (_e816.member_6.x == 1u) {
                                        let _e826 = local_70;
                                        let _e846 = vec3<f32>((((_e320.member.x + (_e320.member_1.x * _e826.x)) - _e816.member.x) / _e816.member.w), (((_e320.member.y + (_e320.member_1.y * _e826.x)) - _e816.member.y) / _e816.member.w), (((_e320.member.z + (_e320.member_1.z * _e826.x)) - _e816.member.z) / _e816.member.w));
                                        phi_11905_ = type_58(_e846, _e846);
                                    } else {
                                        let _e823 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e320.member_1.y > 0f)));
                                        phi_11905_ = type_58(_e823, _e823);
                                    }
                                    let _e849 = phi_11905_;
                                    phi_11906_ = _e849;
                                }
                                let _e944 = phi_11906_;
                                let _e948 = local_73;
                                let _e953 = type_64(_e948.member, _e948.member_1, _e944.member, _e948.member_3);
                                phi_11911_ = type_64(_e953.member, _e953.member_1, _e953.member_2, _e944.member_1);
                            } else {
                                let _e7528 = local_105;
                                phi_11911_ = _e7528;
                            }
                            let _e960 = phi_11911_;
                            phi_11912_ = _e960;
                        } else {
                            phi_11912_ = type_64();
                        }
                        let _e962 = phi_11912_;
                        if select(true, false, _e808) {
                            phi_11917_ = _e810;
                        } else {
                            phi_11917_ = _e962;
                        }
                        let _e965 = phi_11917_;
                        let _e966 = (select(select(u32(_e320.member_3.w), 0u, (_e320.member_3.w < 0f)), 4294967295u, (_e320.member_3.w > 4294967000f)) + 1u);
                        if (_e965.member_1 == 4294967295u) {
                            if _e360 {
                                phi_12076_ = 1f;
                            } else {
                                if ((_e366 & 2u) == 0u) {
                                    phi_12075_ = 1f;
                                } else {
                                    let _e6788 = global_1.member.member_3[1u];
                                    let _e6791 = global_1.member.member_3[2u];
                                    let _e6794 = ((atan2(_e320.member_1.z, _e320.member_1.x) * 0.15915494f) + 0.5f);
                                    let _e6798 = select(_e320.member_1.y, -1f, (_e320.member_1.y < -1f));
                                    let _e6801 = acos(select(_e6798, 1f, (_e6798 > 1f)));
                                    let _e6803 = f32(_e6788);
                                    let _e6804 = ((_e6794 - floor(_e6794)) * _e6803);
                                    let _e6809 = select(select(u32(_e6804), 0u, (_e6804 < 0f)), 4294967295u, (_e6804 > 4294967000f));
                                    let _e6810 = (_e6788 - 1u);
                                    let _e6813 = f32(_e6791);
                                    let _e6814 = ((_e6801 * 0.31830987f) * _e6813);
                                    let _e6819 = select(select(u32(_e6814), 0u, (_e6814 < 0f)), 4294967295u, (_e6814 > 4294967000f));
                                    let _e6820 = (_e6791 - 1u);
                                    let _e6822 = select(_e6819, _e6820, (_e6820 < _e6819));
                                    let _e6823 = sin(_e6801);
                                    if (_e6823 <= 0f) {
                                        phi_12073_ = 0f;
                                    } else {
                                        let _e6827 = global_1.member.member_2[3u];
                                        let _e6830 = global_1.member.member_3[0u];
                                        let _e6834 = (_e6827 + _e6822);
                                        let _e6837 = global_6.member[(_e6834 + 1u)];
                                        let _e6839 = global_6.member[_e6834];
                                        let _e6842 = ((_e6830 + (_e6822 * (_e6788 + 1u))) + select(_e6809, _e6810, (_e6810 < _e6809)));
                                        let _e6845 = global_6.member[(_e6842 + 1u)];
                                        let _e6847 = global_6.member[_e6842];
                                        phi_12073_ = ((((_e6837 - _e6839) * _e6813) * ((_e6845 - _e6847) * _e6803)) / (19.73921f * _e6823));
                                    }
                                    let _e6854 = phi_12073_;
                                    if (_e320.member_1.w >= 300000000000000000000000000000000000000f) {
                                        phi_31051_ = 1f;
                                    } else {
                                        let _e6856 = (_e320.member_1.w * _e320.member_1.w);
                                        let _e6858 = (_e6856 + (_e6854 * _e6854));
                                        if (_e6858 == 0f) {
                                            phi_31048_ = 0f;
                                        } else {
                                            phi_31048_ = (_e6856 / _e6858);
                                        }
                                        let _e6862 = phi_31048_;
                                        phi_31051_ = _e6862;
                                    }
                                    let _e6864 = phi_31051_;
                                    phi_12075_ = _e6864;
                                }
                                let _e6866 = phi_12075_;
                                phi_12076_ = _e6866;
                            }
                            let _e6868 = phi_12076_;
                            if ((_e366 & 4u) == 0u) {
                                phi_12940_ = _e351;
                            } else {
                                if _e360 {
                                    phi_12129_ = 1f;
                                } else {
                                    let _e6879 = global_1.member.member_5;
                                    let _e6883 = (_e320.member_1.x - _e6879.x);
                                    let _e6884 = (_e320.member_1.y - _e6879.y);
                                    let _e6885 = (_e320.member_1.z - _e6879.z);
                                    let _e6892 = (sqrt((((_e6883 * _e6883) + (_e6884 * _e6884)) + (_e6885 * _e6885))) * 0.5f);
                                    if (_e6892 != _e6892) {
                                        phi_31077_ = true;
                                    } else {
                                        phi_31077_ = (1f <= _e6892);
                                    }
                                    let _e6896 = phi_31077_;
                                    let _e6902 = global_1.member.member_6[3u];
                                    if ((2f * asin(select(_e6892, 1f, _e6896))) >= _e6902) {
                                        phi_12127_ = 0f;
                                    } else {
                                        let _e6905 = sin((_e6902 * 0.5f));
                                        phi_12127_ = (1f / ((12.566371f * _e6905) * _e6905));
                                    }
                                    let _e6910 = phi_12127_;
                                    if (_e320.member_1.w >= 300000000000000000000000000000000000000f) {
                                        phi_31098_ = 1f;
                                    } else {
                                        let _e6912 = (_e320.member_1.w * _e320.member_1.w);
                                        let _e6914 = (_e6912 + (_e6910 * _e6910));
                                        if (_e6914 == 0f) {
                                            phi_31095_ = 0f;
                                        } else {
                                            phi_31095_ = (_e6912 / _e6914);
                                        }
                                        let _e6918 = phi_31095_;
                                        phi_31098_ = _e6918;
                                    }
                                    let _e6920 = phi_31098_;
                                    phi_12129_ = _e6920;
                                }
                                let _e6922 = phi_12129_;
                                let _e6924 = global_1.member.member_5;
                                let _e6928 = (_e320.member_1.x - _e6924.x);
                                let _e6929 = (_e320.member_1.y - _e6924.y);
                                let _e6930 = (_e320.member_1.z - _e6924.z);
                                let _e6937 = (sqrt((((_e6928 * _e6928) + (_e6929 * _e6929)) + (_e6930 * _e6930))) * 0.5f);
                                if (_e6937 != _e6937) {
                                    phi_31109_ = true;
                                } else {
                                    phi_31109_ = (1f <= _e6937);
                                }
                                let _e6941 = phi_31109_;
                                let _e6944 = (2f * asin(select(_e6937, 1f, _e6941)));
                                let _e6948 = global_1.member.member_6[3u];
                                if (_e6944 >= _e6948) {
                                    phi_12926_ = vec3<f32>(0f, 0f, 0f);
                                } else {
                                    let _e6952 = (1f - pow((_e6944 / _e6948), 2f));
                                    if (_e6952 != _e6952) {
                                        phi_31124_ = true;
                                    } else {
                                        phi_31124_ = (0f >= _e6952);
                                    }
                                    let _e6956 = phi_31124_;
                                    let _e6959 = global_1.member.member_6;
                                    let _e6965 = global_1.member.member_8[2u];
                                    if (_e6965 == 4294967295u) {
                                        phi_12899_ = vec3<f32>(1f, 1f, 1f);
                                    } else {
                                        let _e6968 = global_6.member[_e6965];
                                        let _e6971 = global_6.member[(_e6965 + 1u)];
                                        let _e6974 = global_6.member[(_e6965 + 2u)];
                                        let _e6977 = global_6.member[(_e6965 + 3u)];
                                        let _e6980 = global_6.member[(_e6965 + 4u)];
                                        let _e6983 = global_6.member[(_e6965 + 5u)];
                                        let _e6986 = global_6.member[(_e6965 + 6u)];
                                        let _e6989 = global_6.member[(_e6965 + 7u)];
                                        let _e6992 = global_6.member[(_e6965 + 8u)];
                                        let _e6995 = global_6.member[(_e6965 + 9u)];
                                        let _e6998 = global_6.member[(_e6965 + 13u)];
                                        let _e7001 = global_6.member[(_e6965 + 14u)];
                                        let _e7004 = global_6.member[(_e6965 + 15u)];
                                        let _e7007 = global_6.member[(_e6965 + 16u)];
                                        let _e7010 = global_6.member[(_e6965 + 18u)];
                                        let _e7013 = global_6.member[(_e6965 + 19u)];
                                        let _e7016 = global_6.member[(_e6965 + 20u)];
                                        let _e7019 = global_6.member[(_e6965 + 29u)];
                                        let _e7020 = bitcast<u32>(_e7019);
                                        let _e7033 = (((((_e320.member.x * _e320.member.x) + (_e320.member.y * _e320.member.y)) + (_e320.member.z * _e320.member.z)) - (2f * (((_e320.member.x * _e6968) + (_e320.member.y * _e6971)) + (_e320.member.z * _e6974)))) + _e6986);
                                        if (_e7033 > 0f) {
                                            let _e7042 = ((((_e320.member.x - _e6968) * _e6989) + ((_e320.member.y - _e6971) * _e6992)) + ((_e320.member.z - _e6974) * _e6995));
                                            let _e7044 = ((_e7042 * _e7042) - _e7033);
                                            if (_e7042 >= 0f) {
                                                phi_12599_ = vec3<f32>();
                                                phi_12600_ = true;
                                            } else {
                                                let _e7046 = (_e7044 <= 0f);
                                                if _e7046 {
                                                    phi_12598_ = vec3<f32>();
                                                } else {
                                                    let _e7049 = (-(_e7042) - sqrt(_e7044));
                                                    phi_12598_ = vec3<f32>((_e320.member.x + (_e6989 * _e7049)), (_e320.member.y + (_e6992 * _e7049)), (_e320.member.z + (_e6995 * _e7049)));
                                                }
                                                let _e7058 = phi_12598_;
                                                phi_12599_ = _e7058;
                                                phi_12600_ = _e7046;
                                            }
                                            let _e7060 = phi_12599_;
                                            let _e7062 = phi_12600_;
                                            phi_12605_ = _e7060;
                                            phi_12606_ = select(true, false, _e7062);
                                        } else {
                                            phi_12605_ = vec3<f32>(_e320.member.x, _e320.member.y, _e320.member.z);
                                            phi_12606_ = true;
                                        }
                                        let _e7065 = phi_12605_;
                                        let _e7067 = phi_12606_;
                                        if _e7067 {
                                            let _e7069 = (_e7065.x - _e6968);
                                            let _e7071 = (_e7065.y - _e6971);
                                            let _e7073 = (_e7065.z - _e6974);
                                            let _e7079 = sqrt((((_e7069 * _e7069) + (_e7071 * _e7071)) + (_e7073 * _e7073)));
                                            let _e7080 = (1f / _e7079);
                                            let _e7098 = ((((((_e7065.x * _e7065.x) + (_e7065.y * _e7065.y)) + (_e7065.z * _e7065.z)) - (2f * (((_e7065.x * _e6968) + (_e7065.y * _e6971)) + (_e7065.z * _e6974)))) + _e6983) / (_e7079 + _e6977));
                                            if (_e7098 != _e7098) {
                                                phi_31248_ = true;
                                            } else {
                                                phi_31248_ = (0f >= _e7098);
                                            }
                                            let _e7102 = phi_31248_;
                                            let _e7106 = sqrt((select(_e7098, 0f, _e7102) / (_e6980 - _e6977)));
                                            if (_e7106 != _e7106) {
                                                phi_31263_ = true;
                                            } else {
                                                phi_31263_ = (1f <= _e7106);
                                            }
                                            let _e7110 = phi_31263_;
                                            let _e7112 = (select(_e7106, 1f, _e7110) * 63f);
                                            let _e7117 = ((((_e7069 * _e7080) * _e6989) + ((_e7071 * _e7080) * _e6992)) + ((_e7073 * _e7080) * _e6995));
                                            let _e7119 = select(_e7117, -1f, (_e7117 < -1f));
                                            let _e7123 = ((select(_e7119, 1f, (_e7119 > 1f)) + 1f) * 127.5f);
                                            let _e7128 = select(select(u32(_e7112), 0u, (_e7112 < 0f)), 4294967295u, (_e7112 > 4294967000f));
                                            let _e7130 = select(_e7128, 62u, (62u < _e7128));
                                            let _e7135 = select(select(u32(_e7123), 0u, (_e7123 < 0f)), 4294967295u, (_e7123 > 4294967000f));
                                            let _e7137 = select(_e7135, 254u, (254u < _e7135));
                                            let _e7139 = (_e7112 - f32(_e7130));
                                            let _e7141 = (_e7123 - f32(_e7137));
                                            let _e7145 = (_e7020 + (3u * ((_e7130 * 256u) + _e7137)));
                                            let _e7150 = (_e7020 + (3u * (((_e7130 + 1u) * 256u) + _e7137)));
                                            let _e7152 = global_6.member[_e7145];
                                            let _e7155 = global_6.member[(_e7145 + 1u)];
                                            let _e7158 = global_6.member[(_e7145 + 2u)];
                                            let _e7159 = (1f - _e7141);
                                            let _e7165 = global_6.member[(_e7145 + 3u)];
                                            let _e7168 = global_6.member[(_e7145 + 4u)];
                                            let _e7171 = global_6.member[(_e7145 + 5u)];
                                            let _e7178 = (1f - _e7139);
                                            let _e7183 = global_6.member[_e7150];
                                            let _e7186 = global_6.member[(_e7150 + 1u)];
                                            let _e7189 = global_6.member[(_e7150 + 2u)];
                                            let _e7195 = global_6.member[(_e7150 + 3u)];
                                            let _e7198 = global_6.member[(_e7150 + 4u)];
                                            let _e7201 = global_6.member[(_e7150 + 5u)];
                                            phi_12897_ = vec3<f32>(((((_e7152 * _e7159) + (_e7165 * _e7141)) * _e7178) + (((_e7183 * _e7159) + (_e7195 * _e7141)) * _e7139)), ((((_e7155 * _e7159) + (_e7168 * _e7141)) * _e7178) + (((_e7186 * _e7159) + (_e7198 * _e7141)) * _e7139)), ((((_e7158 * _e7159) + (_e7171 * _e7141)) * _e7178) + (((_e7189 * _e7159) + (_e7201 * _e7141)) * _e7139)));
                                        } else {
                                            phi_12897_ = vec3<f32>(0f, 0f, 0f);
                                        }
                                        let _e7216 = phi_12897_;
                                        let _e7223 = ((_e7007 * 1.1f) * _e7216.y);
                                        phi_12899_ = vec3<f32>(exp(-((((_e6998 * _e7216.x) + _e7223) + (_e7010 * _e7216.z)))), exp(-((((_e7001 * _e7216.x) + _e7223) + (_e7013 * _e7216.z)))), exp(-((((_e7004 * _e7216.x) + _e7223) + (_e7016 * _e7216.z)))));
                                    }
                                    let _e7242 = phi_12899_;
                                    let _e7254 = sin((_e6948 * 0.5f));
                                    let _e7257 = (((1f - (0.6f * (1f - sqrt(select(_e6952, 0f, _e6956))))) * 1.25f) / ((12.566371f * _e7254) * _e7254));
                                    phi_12926_ = vec3<f32>(((_e6959.x * _e7242.x) * _e7257), ((_e6959.y * _e7242.y) * _e7257), ((_e6959.z * _e7242.z) * _e7257));
                                }
                                let _e7263 = phi_12926_;
                                phi_12940_ = vec3<f32>((_e320.member_3.x + ((_e320.member_2.x * _e7263.x) * _e6922)), (_e320.member_3.y + ((_e320.member_2.y * _e7263.y) * _e6922)), (_e320.member_3.z + ((_e320.member_2.z * _e7263.z) * _e6922)));
                            }
                            let _e7278 = phi_12940_;
                            phi_19324_ = type_69(type_35(_e320.member, _e320.member_1, vec4<f32>(_e320.member_2.x, _e320.member_2.y, _e320.member_2.z, f32(_e347)), vec4<f32>(_e7278.x, _e7278.y, _e7278.z, f32(_e966)), vec4<u32>(_e320.member_4.x, (_e320.member_4.y & 4294967294u), 0u, 0u)), type_39(vec4<f32>(_e320.member.x, _e320.member.y, _e320.member.z, -1f), vec4<f32>(_e320.member_1.x, _e320.member_1.y, _e320.member_1.z, 0f), vec4<f32>((_e320.member_2.x * _e6868), (_e320.member_2.y * _e6868), (_e320.member_2.z * _e6868), 0f)), type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_19325_ = type_69();
                            phi_19326_ = true;
                        } else {
                            if ((_e366 & 8u) == 0u) {
                                phi_34340_ = _e806;
                                phi_13477_ = _e341;
                                phi_13478_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            } else {
                                if (_e965.member > 100f) {
                                    let _e979 = global_1.member.member_8[1u];
                                    let _e981 = global_6.member[_e979];
                                    let _e984 = global_6.member[(_e979 + 1u)];
                                    let _e987 = global_6.member[(_e979 + 2u)];
                                    let _e990 = global_6.member[(_e979 + 3u)];
                                    let _e993 = global_6.member[(_e979 + 5u)];
                                    let _e996 = global_6.member[(_e979 + 6u)];
                                    let _e999 = global_6.member[(_e979 + 10u)];
                                    let _e1002 = global_6.member[(_e979 + 11u)];
                                    let _e1005 = global_6.member[(_e979 + 13u)];
                                    let _e1008 = global_6.member[(_e979 + 14u)];
                                    let _e1011 = global_6.member[(_e979 + 15u)];
                                    let _e1014 = global_6.member[(_e979 + 16u)];
                                    let _e1017 = global_6.member[(_e979 + 18u)];
                                    let _e1020 = global_6.member[(_e979 + 19u)];
                                    let _e1023 = global_6.member[(_e979 + 20u)];
                                    let _e1031 = ((((_e320.member.x - _e981) * _e320.member_1.x) + ((_e320.member.y - _e984) * _e320.member_1.y)) + ((_e320.member.z - _e987) * _e320.member_1.z));
                                    let _e1043 = ((((_e320.member.x * _e320.member.x) + (_e320.member.y * _e320.member.y)) + (_e320.member.z * _e320.member.z)) - (2f * (((_e320.member.x * _e981) + (_e320.member.y * _e984)) + (_e320.member.z * _e987))));
                                    let _e1044 = (_e1043 + _e996);
                                    let _e1045 = (_e1031 * _e1031);
                                    let _e1046 = (_e1045 - _e1044);
                                    if (_e1046 < 0f) {
                                        phi_25145_ = type_55(0u, type_55().member_1);
                                        phi_25146_ = type_55();
                                        phi_25147_ = true;
                                    } else {
                                        if (_e1031 < 0f) {
                                            phi_25117_ = -(sqrt(_e1046));
                                        } else {
                                            phi_25117_ = sqrt(_e1046);
                                        }
                                        let _e1053 = phi_25117_;
                                        let _e1055 = -((_e1031 + _e1053));
                                        let _e1056 = (_e1055 == 0f);
                                        if _e1056 {
                                            phi_25140_ = type_55(1u, type_54(0f, 0f));
                                            phi_25141_ = type_55();
                                        } else {
                                            let _e1057 = (_e1044 / _e1055);
                                            if (_e1055 < _e1057) {
                                                phi_25129_ = type_54(_e1055, _e1057);
                                            } else {
                                                phi_25129_ = type_54(_e1057, _e1055);
                                            }
                                            let _e1062 = phi_25129_;
                                            phi_25140_ = type_55();
                                            phi_25141_ = type_55(1u, _e1062);
                                        }
                                        let _e1065 = phi_25140_;
                                        let _e1067 = phi_25141_;
                                        phi_25145_ = _e1065;
                                        phi_25146_ = _e1067;
                                        phi_25147_ = _e1056;
                                    }
                                    let _e1072 = phi_25145_;
                                    let _e1074 = phi_25146_;
                                    let _e1076 = phi_25147_;
                                    if _e1076 {
                                        phi_25151_ = _e1072;
                                    } else {
                                        phi_25151_ = _e1074;
                                    }
                                    let _e1078 = phi_25151_;
                                    if (bitcast<i32>(_e1078.member) != 0i) {
                                        phi_25038_ = type_55(0u, _e1078.member_1);
                                    } else {
                                        phi_25038_ = type_55(1u, type_55().member_1);
                                    }
                                    let _e1088 = phi_25038_;
                                    if (bitcast<i32>(_e1088.member) != 0i) {
                                        phi_25085_ = type_55(0u, type_55().member_1);
                                    } else {
                                        if (_e1088.member_1.member_1 <= 0f) {
                                            phi_25081_ = type_55(0u, type_55().member_1);
                                        } else {
                                            if (_e1088.member_1.member != _e1088.member_1.member) {
                                                phi_25189_ = true;
                                            } else {
                                                phi_25189_ = (0f >= _e1088.member_1.member);
                                            }
                                            let _e1100 = phi_25189_;
                                            let _e1101 = select(_e1088.member_1.member, 0f, _e1100);
                                            let _e1102 = (_e1043 + _e993);
                                            if (_e1102 > 0f) {
                                                let _e1104 = (_e1045 - _e1102);
                                                if (_e1104 < 0f) {
                                                    phi_25297_ = type_55(0u, type_55().member_1);
                                                    phi_25298_ = type_55();
                                                    phi_25299_ = true;
                                                } else {
                                                    if (_e1031 < 0f) {
                                                        phi_25269_ = -(sqrt(_e1104));
                                                    } else {
                                                        phi_25269_ = sqrt(_e1104);
                                                    }
                                                    let _e1111 = phi_25269_;
                                                    let _e1113 = -((_e1031 + _e1111));
                                                    let _e1114 = (_e1113 == 0f);
                                                    if _e1114 {
                                                        phi_25292_ = type_55(1u, type_54(0f, 0f));
                                                        phi_25293_ = type_55();
                                                    } else {
                                                        let _e1115 = (_e1102 / _e1113);
                                                        if (_e1113 < _e1115) {
                                                            phi_25281_ = type_54(_e1113, _e1115);
                                                        } else {
                                                            phi_25281_ = type_54(_e1115, _e1113);
                                                        }
                                                        let _e1120 = phi_25281_;
                                                        phi_25292_ = type_55();
                                                        phi_25293_ = type_55(1u, _e1120);
                                                    }
                                                    let _e1123 = phi_25292_;
                                                    let _e1125 = phi_25293_;
                                                    phi_25297_ = _e1123;
                                                    phi_25298_ = _e1125;
                                                    phi_25299_ = _e1114;
                                                }
                                                let _e1130 = phi_25297_;
                                                let _e1132 = phi_25298_;
                                                let _e1134 = phi_25299_;
                                                if _e1134 {
                                                    phi_25303_ = _e1130;
                                                } else {
                                                    phi_25303_ = _e1132;
                                                }
                                                let _e1136 = phi_25303_;
                                                if (bitcast<i32>(_e1136.member) != 0i) {
                                                    phi_25237_ = select(_e1136.member_1.member_1, _e1136.member_1.member, (_e1136.member_1.member >= 0f));
                                                } else {
                                                    phi_25237_ = -1f;
                                                }
                                                let _e1147 = phi_25237_;
                                                if (_e1147 >= 0f) {
                                                    if (_e1088.member_1.member_1 != _e1088.member_1.member_1) {
                                                        phi_25341_ = true;
                                                    } else {
                                                        phi_25341_ = (_e1147 <= _e1088.member_1.member_1);
                                                    }
                                                    let _e1152 = phi_25341_;
                                                    phi_25066_ = select(_e1088.member_1.member_1, _e1147, _e1152);
                                                } else {
                                                    phi_25066_ = _e1088.member_1.member_1;
                                                }
                                                let _e1155 = phi_25066_;
                                                phi_25068_ = _e1155;
                                            } else {
                                                phi_25068_ = _e1088.member_1.member_1;
                                            }
                                            let _e1157 = phi_25068_;
                                            if (_e1157 <= _e1101) {
                                                phi_25077_ = type_55(0u, type_55().member_1);
                                            } else {
                                                phi_25077_ = type_55(1u, type_54(_e1101, _e1157));
                                            }
                                            let _e1165 = phi_25077_;
                                            phi_25081_ = _e1165;
                                        }
                                        let _e1170 = phi_25081_;
                                        phi_25085_ = _e1170;
                                    }
                                    let _e1175 = phi_25085_;
                                    if (bitcast<i32>(_e1175.member) != 0i) {
                                        let _e1181 = (_e1175.member_1.member < _e965.member);
                                        if _e1181 {
                                            if (_e1175.member_1.member_1 != _e1175.member_1.member_1) {
                                                phi_25356_ = true;
                                            } else {
                                                phi_25356_ = (_e965.member <= _e1175.member_1.member_1);
                                            }
                                            let _e1187 = phi_25356_;
                                            let _e1188 = select(_e1175.member_1.member_1, _e965.member, _e1187);
                                            phi_13336_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13339_ = 0u;
                                            loop {
                                                let _e1190 = phi_13336_;
                                                let _e1192 = phi_13339_;
                                                local_74 = _e1190;
                                                local_75 = _e1190;
                                                local_76 = _e1190;
                                                let _e1193 = (_e1192 < 32u);
                                                if _e1193 {
                                                    let _e1194 = -(_e1031);
                                                    if (_e1175.member_1.member <= _e1188) {
                                                    } else {
                                                        phi_32117_ = true;
                                                        break;
                                                    }
                                                    let _e1197 = select(_e1194, _e1175.member_1.member, (_e1194 < _e1175.member_1.member));
                                                    let _e1199 = select(_e1197, _e1188, (_e1197 > _e1188));
                                                    let _e1200 = (_e1199 - _e1175.member_1.member);
                                                    let _e1201 = (_e1188 - _e1199);
                                                    let _e1202 = (_e1188 - _e1175.member_1.member);
                                                    let _e1203 = (0.000000001f * _e1202);
                                                    if (_e1200 <= _e1203) {
                                                        let _e1278 = (f32(_e1192) * 0.03125f);
                                                        let _e1281 = (f32((_e1192 + 1u)) * 0.03125f);
                                                        let _e1283 = ((_e1278 + _e1281) * 0.5f);
                                                        phi_13440_ = type_54((_e1175.member_1.member + ((_e1202 * _e1283) * _e1283)), (_e1202 * ((_e1281 * _e1281) - (_e1278 * _e1278))));
                                                    } else {
                                                        if (_e1201 <= _e1203) {
                                                            let _e1261 = (f32((31u - _e1192)) * 0.03125f);
                                                            let _e1264 = (f32((32u - _e1192)) * 0.03125f);
                                                            let _e1266 = ((_e1261 + _e1264) * 0.5f);
                                                            phi_13439_ = type_54((_e1188 - ((_e1202 * _e1266) * _e1266)), (_e1202 * ((_e1264 * _e1264) - (_e1261 * _e1261))));
                                                        } else {
                                                            let _e1209 = round(((32f * _e1200) / (_e1200 + _e1201)));
                                                            let _e1214 = select(select(u32(_e1209), 0u, (_e1209 < 0f)), 4294967295u, (_e1209 > 4294967000f));
                                                            if (_e1214 < 1u) {
                                                                phi_13423_ = 1u;
                                                            } else {
                                                                phi_13423_ = select(_e1214, 31u, (_e1214 > 31u));
                                                            }
                                                            let _e1219 = phi_13423_;
                                                            if (_e1192 < _e1219) {
                                                                let _e1240 = ((_e1219 - 1u) - _e1192);
                                                                let _e1242 = f32(_e1219);
                                                                let _e1243 = (f32(_e1240) / _e1242);
                                                                let _e1246 = (f32((_e1240 + 1u)) / _e1242);
                                                                let _e1248 = ((_e1243 + _e1246) * 0.5f);
                                                                phi_13438_ = type_54((_e1199 - ((_e1200 * _e1248) * _e1248)), (_e1200 * ((_e1246 * _e1246) - (_e1243 * _e1243))));
                                                            } else {
                                                                let _e1222 = (_e1192 - _e1219);
                                                                let _e1224 = f32((32u - _e1219));
                                                                let _e1225 = (f32(_e1222) / _e1224);
                                                                let _e1228 = (f32((_e1222 + 1u)) / _e1224);
                                                                let _e1230 = ((_e1225 + _e1228) * 0.5f);
                                                                phi_13438_ = type_54((_e1199 + ((_e1201 * _e1230) * _e1230)), (_e1201 * ((_e1228 * _e1228) - (_e1225 * _e1225))));
                                                            }
                                                            let _e1258 = phi_13438_;
                                                            phi_13439_ = _e1258;
                                                        }
                                                        let _e1276 = phi_13439_;
                                                        phi_13440_ = _e1276;
                                                    }
                                                    let _e1293 = phi_13440_;
                                                    let _e1299 = (_e320.member.x + (_e320.member_1.x * _e1293.member));
                                                    let _e1300 = (_e320.member.y + (_e320.member_1.y * _e1293.member));
                                                    let _e1301 = (_e320.member.z + (_e320.member_1.z * _e1293.member));
                                                    let _e1315 = (_e1299 - _e981);
                                                    let _e1316 = (_e1300 - _e984);
                                                    let _e1317 = (_e1301 - _e987);
                                                    let _e1325 = ((((((_e1299 * _e1299) + (_e1300 * _e1300)) + (_e1301 * _e1301)) - (2f * (((_e1299 * _e981) + (_e1300 * _e984)) + (_e1301 * _e987)))) + _e993) / (sqrt((((_e1315 * _e1315) + (_e1316 * _e1316)) + (_e1317 * _e1317))) + _e990));
                                                    let _e1329 = (1f - (abs((_e1325 - 25000f)) * 0.00006666667f));
                                                    if (_e1329 != _e1329) {
                                                        phi_25555_ = true;
                                                    } else {
                                                        phi_25555_ = (0f >= _e1329);
                                                    }
                                                    let _e1333 = phi_25555_;
                                                    let _e1335 = -(_e1325);
                                                    phi_13337_ = (_e1190 + vec3<f32>((exp((_e1335 / _e999)) * _e1293.member_1), (exp((_e1335 / _e1002)) * _e1293.member_1), (select(_e1329, 0f, _e1333) * _e1293.member_1)));
                                                    phi_13340_ = (_e1192 + 1u);
                                                } else {
                                                    phi_13337_ = vec3<f32>();
                                                    phi_13340_ = u32();
                                                }
                                                let _e1347 = phi_13337_;
                                                let _e1349 = phi_13340_;
                                                phi_13336_ = _e1347;
                                                phi_13339_ = _e1349;
                                                phi_32117_ = _e806;
                                                if !(_e1193) {
                                                    break;
                                                }
                                            }
                                            let _e1352 = phi_32117_;
                                            if _e1352 {
                                                break;
                                            }
                                            let _e1354 = local_74;
                                            let _e1361 = local_75;
                                            let _e1363 = ((_e1014 * 1.1f) * _e1361.y);
                                            let _e1368 = local_76;
                                            phi_34344_ = _e1352;
                                            phi_13465_ = vec3<f32>(exp(-((((_e1005 * _e1354.x) + _e1363) + (_e1017 * _e1368.z)))), exp(-((((_e1008 * _e1354.x) + _e1363) + (_e1020 * _e1368.z)))), exp(-((((_e1011 * _e1354.x) + _e1363) + (_e1023 * _e1368.z)))));
                                        } else {
                                            phi_34344_ = _e806;
                                            phi_13465_ = vec3<f32>();
                                        }
                                        let _e1384 = phi_34344_;
                                        let _e1386 = phi_13465_;
                                        phi_34343_ = _e1384;
                                        phi_13467_ = _e1386;
                                        phi_13468_ = select(true, false, _e1181);
                                    } else {
                                        phi_34343_ = _e806;
                                        phi_13467_ = vec3<f32>();
                                        phi_13468_ = true;
                                    }
                                    let _e1389 = phi_34343_;
                                    let _e1391 = phi_13467_;
                                    let _e1393 = phi_13468_;
                                    phi_34341_ = _e1389;
                                    phi_13475_ = (_e341 * select(_e1391, vec3<f32>(1f, 1f, 1f), vec3(_e1393)));
                                    phi_13476_ = type_39(vec4<f32>(_e320.member.x, _e320.member.y, _e320.member.z, _e965.member), vec4<f32>(_e320.member_1.x, _e320.member_1.y, _e320.member_1.z, 0f), vec4<f32>(_e320.member_2.x, _e320.member_2.y, _e320.member_2.z, 0f));
                                } else {
                                    phi_34341_ = _e806;
                                    phi_13475_ = _e341;
                                    phi_13476_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                }
                                let _e1398 = phi_34341_;
                                let _e1400 = phi_13475_;
                                let _e1402 = phi_13476_;
                                phi_34340_ = _e1398;
                                phi_13477_ = _e1400;
                                phi_13478_ = _e1402;
                            }
                            let _e1404 = phi_34340_;
                            let _e1406 = phi_13477_;
                            let _e1408 = phi_13478_;
                            let _e1413 = (_e320.member.x + (_e320.member_1.x * _e965.member));
                            let _e1414 = (_e320.member.y + (_e320.member_1.y * _e965.member));
                            let _e1415 = (_e320.member.z + (_e320.member_1.z * _e965.member));
                            let _e1417 = global_2.member[_e965.member_1];
                            phi_13502_ = 0i;
                            phi_13505_ = _e1417.member_6.y;
                            loop {
                                let _e1421 = phi_13502_;
                                let _e1423 = phi_13505_;
                                local_77 = _e1423;
                                if (_e1421 < 4i) {
                                    let _e1426 = global_4.member[_e1423];
                                    let _e1428 = (_e1426.member != 6u);
                                    if _e1428 {
                                        phi_13574_ = i32();
                                        phi_13575_ = u32();
                                    } else {
                                        let _e1432 = (abs(_e1413) / _e1426.member_6.z);
                                        let _e1443 = (abs(_e1415) / _e1426.member_6.w);
                                        if ((select(select(u32(_e1432), 0u, (_e1432 < 0f)), 4294967295u, (_e1432 > 4294967000f)) % 2u) == 0u) {
                                            phi_13559_ = (_e1413 > 0f);
                                        } else {
                                            phi_13559_ = (_e1413 <= 0f);
                                        }
                                        let _e1454 = phi_13559_;
                                        if ((select(select(u32(_e1443), 0u, (_e1443 < 0f)), 4294967295u, (_e1443 > 4294967000f)) % 2u) == 0u) {
                                            phi_13565_ = (_e1415 > 0f);
                                        } else {
                                            phi_13565_ = (_e1415 <= 0f);
                                        }
                                        let _e1458 = phi_13565_;
                                        if (_e1454 != _e1458) {
                                            phi_13572_ = _e1426.member_1;
                                        } else {
                                            phi_13572_ = _e1426.member_2;
                                        }
                                        let _e1463 = phi_13572_;
                                        phi_13574_ = (_e1421 + 1i);
                                        phi_13575_ = _e1463;
                                    }
                                    let _e1466 = phi_13574_;
                                    let _e1468 = phi_13575_;
                                    phi_13503_ = _e1466;
                                    phi_13506_ = _e1468;
                                    phi_13577_ = select(true, false, _e1428);
                                } else {
                                    phi_13503_ = i32();
                                    phi_13506_ = u32();
                                    phi_13577_ = false;
                                }
                                let _e1471 = phi_13503_;
                                let _e1473 = phi_13506_;
                                let _e1475 = phi_13577_;
                                phi_13502_ = _e1471;
                                phi_13505_ = _e1473;
                                if !(_e1475) {
                                    break;
                                }
                            }
                            let _e1478 = local_77;
                            let _e1480 = global_4.member[_e1478];
                            let _e1482 = (_e1480.member == 4u);
                            if _e1482 {
                                if _e1482 {
                                    if (_e1480.member_1 == 1u) {
                                        phi_13637_ = true;
                                    } else {
                                        phi_13637_ = ((((-(_e320.member_1.x) * _e965.member_3.x) + (-(_e320.member_1.y) * _e965.member_3.y)) + (-(_e320.member_1.z) * _e965.member_3.z)) > 0f);
                                    }
                                    let _e6584 = phi_13637_;
                                    if _e6584 {
                                        phi_13646_ = vec3<f32>(_e1480.member_4.x, _e1480.member_4.y, _e1480.member_4.z);
                                    } else {
                                        phi_13646_ = vec3<f32>();
                                    }
                                    let _e6591 = phi_13646_;
                                    phi_13648_ = _e6591;
                                    phi_13649_ = select(true, false, _e6584);
                                } else {
                                    phi_13648_ = vec3<f32>();
                                    phi_13649_ = true;
                                }
                                let _e6594 = phi_13648_;
                                let _e6596 = phi_13649_;
                                let _e6598 = select(_e6594, vec3<f32>(0f, 0f, 0f), vec3(_e6596));
                                if _e360 {
                                    phi_13819_ = 1f;
                                } else {
                                    if ((_e366 & 1u) == 0u) {
                                        phi_13818_ = 1f;
                                    } else {
                                        if (_e1417.member_6.z > 0u) {
                                            let _e6607 = global_1.member.member_2[1u];
                                            let _e6608 = (_e6607 + (_e1417.member_6.z - 1u));
                                            let _e6611 = global_6.member[(_e6608 + 1u)];
                                            let _e6613 = global_6.member[_e6608];
                                            if (_e1417.member_6.x == 1u) {
                                                let _e6690 = (_e1417.member.x - _e320.member.x);
                                                let _e6691 = (_e1417.member.y - _e320.member.y);
                                                let _e6692 = (_e1417.member.z - _e320.member.z);
                                                let _e6697 = (((_e6690 * _e6690) + (_e6691 * _e6691)) + (_e6692 * _e6692));
                                                let _e6698 = (_e1417.member.w * _e1417.member.w);
                                                if (_e6697 <= (_e6698 * 1.000001f)) {
                                                    let _e6712 = (_e1413 - _e320.member.x);
                                                    let _e6713 = (_e1414 - _e320.member.y);
                                                    let _e6714 = (_e1415 - _e320.member.z);
                                                    let _e6719 = (((_e6712 * _e6712) + (_e6713 * _e6713)) + (_e6714 * _e6714));
                                                    phi_30961_ = (_e6719 / (((abs((((_e965.member_2.x * _e6712) + (_e965.member_2.y * _e6713)) + (_e965.member_2.z * _e6714))) / sqrt(_e6719)) * 12.566371f) * _e6698));
                                                } else {
                                                    let _e6701 = (_e6698 / _e6697);
                                                    if (_e6701 != _e6701) {
                                                        phi_30980_ = true;
                                                    } else {
                                                        phi_30980_ = (1f <= _e6701);
                                                    }
                                                    let _e6705 = phi_30980_;
                                                    let _e6706 = select(_e6701, 1f, _e6705);
                                                    phi_30961_ = (0.15915494f / (_e6706 / (1f + sqrt((1f - _e6706)))));
                                                }
                                                let _e6735 = phi_30961_;
                                                phi_13814_ = _e6735;
                                            } else {
                                                let _e6619 = (_e1413 - _e320.member.x);
                                                let _e6620 = (_e1414 - _e320.member.y);
                                                let _e6621 = (_e1415 - _e320.member.z);
                                                let _e6626 = (((_e6619 * _e6619) + (_e6620 * _e6620)) + (_e6621 * _e6621));
                                                let _e6637 = (abs((((_e965.member_2.x * _e6619) + (_e965.member_2.y * _e6620)) + (_e965.member_2.z * _e6621))) / sqrt(_e6626));
                                                if (_e6637 <= 0f) {
                                                    phi_13813_ = 0f;
                                                } else {
                                                    if (_e1417.member_6.x == 0u) {
                                                        let _e6652 = (_e1417.member_1.x - _e1417.member.x);
                                                        let _e6653 = (_e1417.member_1.y - _e1417.member.y);
                                                        let _e6654 = (_e1417.member_1.z - _e1417.member.z);
                                                        let _e6659 = (_e1417.member_2.x - _e1417.member.x);
                                                        let _e6660 = (_e1417.member_2.y - _e1417.member.y);
                                                        let _e6661 = (_e1417.member_2.z - _e1417.member.z);
                                                        let _e6664 = ((_e6653 * _e6661) - (_e6660 * _e6654));
                                                        let _e6667 = ((_e6654 * _e6659) - (_e6661 * _e6652));
                                                        let _e6670 = ((_e6652 * _e6660) - (_e6659 * _e6653));
                                                        phi_13810_ = (sqrt((((_e6664 * _e6664) + (_e6667 * _e6667)) + (_e6670 * _e6670))) * 0.5f);
                                                    } else {
                                                        phi_13810_ = ((12.566371f * _e1417.member.w) * _e1417.member.w);
                                                    }
                                                    let _e6679 = phi_13810_;
                                                    phi_13813_ = (_e6626 / (_e6637 * _e6679));
                                                }
                                                let _e6683 = phi_13813_;
                                                phi_13814_ = _e6683;
                                            }
                                            let _e6737 = phi_13814_;
                                            let _e6738 = ((_e6611 - _e6613) * _e6737);
                                            if (_e320.member_1.w >= 300000000000000000000000000000000000000f) {
                                                phi_31001_ = 1f;
                                            } else {
                                                let _e6740 = (_e320.member_1.w * _e320.member_1.w);
                                                let _e6742 = (_e6740 + (_e6738 * _e6738));
                                                if (_e6742 == 0f) {
                                                    phi_30998_ = 0f;
                                                } else {
                                                    phi_30998_ = (_e6740 / _e6742);
                                                }
                                                let _e6746 = phi_30998_;
                                                phi_31001_ = _e6746;
                                            }
                                            let _e6748 = phi_31001_;
                                            phi_13817_ = _e6748;
                                        } else {
                                            phi_13817_ = 1f;
                                        }
                                        let _e6750 = phi_13817_;
                                        phi_13818_ = _e6750;
                                    }
                                    let _e6752 = phi_13818_;
                                    phi_13819_ = _e6752;
                                }
                                let _e6754 = phi_13819_;
                                phi_19318_ = type_69(type_35(_e320.member, _e320.member_1, vec4<f32>(_e1406.x, _e1406.y, _e1406.z, f32(_e347)), vec4<f32>((_e320.member_3.x + ((_e1406.x * _e6598.x) * _e6754)), (_e320.member_3.y + ((_e1406.y * _e6598.y) * _e6754)), (_e320.member_3.z + ((_e1406.z * _e6598.z) * _e6754)), f32(_e966)), vec4<u32>(_e320.member_4.x, (_e320.member_4.y & 4294967294u), 0u, 0u)), _e1408, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                phi_19319_ = type_69();
                                phi_19320_ = true;
                            } else {
                                let _e1483 = -(_e320.member_1.x);
                                let _e1484 = -(_e320.member_1.y);
                                let _e1485 = -(_e320.member_1.z);
                                if ((((_e965.member_2.x * _e1483) + (_e965.member_2.y * _e1484)) + (_e965.member_2.z * _e1485)) < 0f) {
                                    phi_13858_ = -(_e965.member_2);
                                } else {
                                    phi_13858_ = _e965.member_2;
                                }
                                let _e1498 = phi_13858_;
                                if ((((_e965.member_3.x * _e1483) + (_e965.member_3.y * _e1484)) + (_e965.member_3.z * _e1485)) < 0f) {
                                    phi_13875_ = -(_e965.member_3);
                                } else {
                                    phi_13875_ = _e965.member_3;
                                }
                                let _e1511 = phi_13875_;
                                let _e1514 = global_1.member.member_4[2u];
                                let _e1515 = abs(_e1413);
                                let _e1516 = abs(_e1414);
                                let _e1517 = abs(_e1415);
                                let _e1519 = select(_e1516, _e1515, (_e1515 > _e1516));
                                let _e1523 = (0.00002f * (1f + select(_e1517, _e1519, (_e1519 > _e1517))));
                                if (_e1514 != _e1514) {
                                    phi_25622_ = true;
                                } else {
                                    phi_25622_ = (_e1523 >= _e1514);
                                }
                                let _e1527 = phi_25622_;
                                let _e1529 = (select(_e1514, _e1523, _e1527) * 2f);
                                let _e1536 = (_e1413 + (_e1498.x * _e1529));
                                let _e1537 = (_e1414 + (_e1498.y * _e1529));
                                let _e1538 = (_e1415 + (_e1498.z * _e1529));
                                let _e1539 = vec3<f32>(_e1536, _e1537, _e1538);
                                if ((_e320.member_4.y & 4u) == 0u) {
                                    phi_13908_ = 0f;
                                } else {
                                    let _e1544 = global_1.member.member_4[0u];
                                    phi_13908_ = _e1544;
                                }
                                let _e1546 = phi_13908_;
                                let _e1549 = select(-1f, 1f, (_e1511.z >= 0f));
                                let _e1551 = (-1f / (_e1549 + _e1511.z));
                                let _e1555 = ((_e1511.x * _e1511.y) * _e1551);
                                let _e1559 = (1f + (((_e1549 * _e1511.x) * _e1511.x) * _e1551));
                                let _e1560 = (_e1549 * _e1555);
                                let _e1562 = (-(_e1549) * _e1511.x);
                                let _e1565 = (_e1549 + ((_e1511.y * _e1511.y) * _e1551));
                                let _e1566 = -(_e1511.y);
                                if (_e1480.member == 0u) {
                                    phi_32983_ = vec3<f32>(0f, 0f, 0f);
                                    phi_32791_ = 1.5f;
                                    phi_32712_ = vec3<f32>(0f, 0f, 0f);
                                    phi_32635_ = vec3<f32>(_e1480.member_4.x, _e1480.member_4.y, _e1480.member_4.z);
                                    phi_32556_ = 0u;
                                    phi_32474_ = 1f;
                                    phi_32397_ = 0u;
                                    phi_32388_ = _e320.member_4.x;
                                    phi_14587_ = type_69();
                                    phi_14588_ = true;
                                } else {
                                    if (_e1480.member == 1u) {
                                        let _e2115 = (_e1480.member_6.y * _e1480.member_6.y);
                                        if (_e2115 != _e2115) {
                                            phi_26314_ = true;
                                        } else {
                                            phi_26314_ = (_e1546 >= _e2115);
                                        }
                                        let _e2119 = phi_26314_;
                                        let _e2120 = select(_e2115, _e1546, _e2119);
                                        let _e2121 = (_e2120 >= 0.001f);
                                        if _e2121 {
                                            phi_33040_ = vec3<f32>(_e1480.member_4.x, _e1480.member_4.y, _e1480.member_4.z);
                                            phi_32834_ = _e1480.member_6.x;
                                            phi_32687_ = vec3<f32>(0f, 0f, 0f);
                                            phi_32375_ = _e320.member_4.x;
                                            phi_13983_ = vec3<f32>();
                                            phi_13984_ = true;
                                        } else {
                                            let _e2123 = ((_e320.member_4.x * 747796405u) + 2891336453u);
                                            let _e2131 = (((_e2123 >> bitcast<u32>((((_e2123 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2123) * 277803737u);
                                            let _e2134 = ((_e2131 >> bitcast<u32>(22i)) ^ _e2131);
                                            let _e2144 = abs((((_e1483 * _e1511.x) + (_e1484 * _e1511.y)) + (_e1485 * _e1511.z)));
                                            let _e2147 = (1f / _e1480.member_6.x);
                                            let _e2151 = ((_e2147 * _e2147) * (1f - (_e2144 * _e2144)));
                                            if (_e2151 >= 1f) {
                                                phi_26354_ = 1f;
                                            } else {
                                                let _e2154 = sqrt((1f - _e2151));
                                                if (_e2144 != _e2144) {
                                                    phi_26365_ = true;
                                                } else {
                                                    phi_26365_ = (_e2154 <= _e2144);
                                                }
                                                let _e2158 = phi_26365_;
                                                let _e2163 = pow(((1f - _e1480.member_6.x) / (1f + _e1480.member_6.x)), 2f);
                                                phi_26354_ = (_e2163 + ((1f - _e2163) * pow((1f - select(_e2144, _e2154, _e2158)), 5f)));
                                            }
                                            let _e2170 = phi_26354_;
                                            let _e2171 = ((f32((_e2134 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e2170);
                                            if _e2171 {
                                                let _e2184 = (((_e1511.x * _e320.member_1.x) + (_e1511.y * _e320.member_1.y)) + (_e1511.z * _e320.member_1.z));
                                                phi_32688_ = vec3<f32>(0f, 0f, 0f);
                                                phi_13981_ = (_e336 - vec3<f32>(((_e1511.x * 2f) * _e2184), ((_e1511.y * 2f) * _e2184), ((_e1511.z * 2f) * _e2184)));
                                            } else {
                                                phi_32688_ = vec3<f32>(_e1480.member_4.x, _e1480.member_4.y, _e1480.member_4.z);
                                                phi_13981_ = vec3<f32>();
                                            }
                                            let _e2191 = phi_32688_;
                                            let _e2193 = phi_13981_;
                                            phi_33040_ = vec3<f32>(0f, 0f, 0f);
                                            phi_32834_ = 1.5f;
                                            phi_32687_ = _e2191;
                                            phi_32375_ = _e2134;
                                            phi_13983_ = _e2193;
                                            phi_13984_ = select(true, false, _e2171);
                                        }
                                        let _e2203 = phi_33040_;
                                        let _e2205 = phi_32834_;
                                        let _e2207 = phi_32687_;
                                        let _e2209 = phi_32375_;
                                        let _e2211 = phi_13983_;
                                        let _e2213 = phi_13984_;
                                        phi_32986_ = _e2203;
                                        phi_32794_ = _e2205;
                                        phi_32715_ = vec3<f32>(0f, 0f, 0f);
                                        phi_32638_ = _e2207;
                                        phi_32559_ = select(0u, 1u, _e2121);
                                        phi_32477_ = select(1f, _e2120, _e2121);
                                        phi_32400_ = select(0u, 2u, _e2121);
                                        phi_32326_ = _e2209;
                                        phi_14457_ = _e1539;
                                        phi_14458_ = _e2211;
                                        phi_14459_ = vec3<f32>(1f, 1f, 1f);
                                        phi_14460_ = _e2213;
                                    } else {
                                        if (_e1480.member == 3u) {
                                            phi_32329_ = _e320.member_4.x;
                                            phi_14136_ = vec3<f32>();
                                            phi_14137_ = vec3<f32>();
                                            phi_14138_ = vec3<f32>();
                                            phi_14139_ = true;
                                            phi_14140_ = false;
                                        } else {
                                            let _e1570 = (_e1480.member == 5u);
                                            if _e1570 {
                                                phi_32330_ = _e320.member_4.x;
                                                phi_14132_ = vec3<f32>();
                                                phi_14133_ = vec3<f32>();
                                                phi_14134_ = vec3<f32>();
                                                phi_14135_ = false;
                                            } else {
                                                let _e1571 = (_e1480.member == 2u);
                                                if _e1571 {
                                                    let _e1576 = (((_e320.member_1.x * _e965.member_3.x) + (_e320.member_1.y * _e965.member_3.y)) + (_e320.member_1.z * _e965.member_3.z));
                                                    let _e1577 = (_e1576 > 0f);
                                                    if _e1577 {
                                                        phi_14017_ = -(_e965.member_3);
                                                    } else {
                                                        phi_14017_ = _e965.member_3;
                                                    }
                                                    let _e1580 = phi_14017_;
                                                    if _e1577 {
                                                        phi_14023_ = _e1480.member_6.x;
                                                    } else {
                                                        phi_14023_ = (1f / _e1480.member_6.x);
                                                    }
                                                    let _e1585 = phi_14023_;
                                                    let _e1586 = abs(_e1576);
                                                    let _e1587 = (_e1585 * _e1585);
                                                    let _e1590 = (_e1587 * (1f - (_e1586 * _e1586)));
                                                    if (_e1590 >= 1f) {
                                                        phi_25684_ = 1f;
                                                    } else {
                                                        let _e1593 = sqrt((1f - _e1590));
                                                        if (_e1586 != _e1586) {
                                                            phi_25695_ = true;
                                                        } else {
                                                            phi_25695_ = (_e1593 <= _e1586);
                                                        }
                                                        let _e1597 = phi_25695_;
                                                        let _e1602 = pow(((1f - _e1480.member_6.x) / (1f + _e1480.member_6.x)), 2f);
                                                        phi_25684_ = (_e1602 + ((1f - _e1602) * pow((1f - select(_e1586, _e1593, _e1597)), 5f)));
                                                    }
                                                    let _e1609 = phi_25684_;
                                                    let _e1617 = (((_e320.member_1.x * _e1580.x) + (_e320.member_1.y * _e1580.y)) + (_e320.member_1.z * _e1580.z));
                                                    let _e1621 = (1f - (_e1587 * (1f - (_e1617 * _e1617))));
                                                    if (_e1621 > 0f) {
                                                        let _e1624 = ((_e320.member_4.x * 747796405u) + 2891336453u);
                                                        let _e1632 = (((_e1624 >> bitcast<u32>((((_e1624 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1624) * 277803737u);
                                                        let _e1635 = ((_e1632 >> bitcast<u32>(22i)) ^ _e1632);
                                                        let _e1640 = ((f32((_e1635 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1609);
                                                        if _e1640 {
                                                            let _e1650 = sqrt(_e1621);
                                                            let _e1654 = (((_e320.member_1.x - (_e1580.x * _e1617)) * _e1585) - (_e1580.x * _e1650));
                                                            let _e1655 = (((_e320.member_1.y - (_e1580.y * _e1617)) * _e1585) - (_e1580.y * _e1650));
                                                            let _e1656 = (((_e320.member_1.z - (_e1580.z * _e1617)) * _e1585) - (_e1580.z * _e1650));
                                                            let _e1663 = (1f / sqrt((((_e1654 * _e1654) + (_e1655 * _e1655)) + (_e1656 * _e1656))));
                                                            phi_14104_ = vec3<f32>((_e1413 - (_e1580.x * _e1529)), (_e1414 - (_e1580.y * _e1529)), (_e1415 - (_e1580.z * _e1529)));
                                                            phi_14105_ = vec3<f32>((_e1654 * _e1663), (_e1655 * _e1663), (_e1656 * _e1663));
                                                            phi_14106_ = vec3<f32>(_e1480.member_4.x, _e1480.member_4.y, _e1480.member_4.z);
                                                        } else {
                                                            phi_14104_ = vec3<f32>();
                                                            phi_14105_ = vec3<f32>();
                                                            phi_14106_ = vec3<f32>();
                                                        }
                                                        let _e1681 = phi_14104_;
                                                        let _e1683 = phi_14105_;
                                                        let _e1685 = phi_14106_;
                                                        phi_32333_ = _e1635;
                                                        phi_14108_ = _e1681;
                                                        phi_14109_ = _e1683;
                                                        phi_14110_ = _e1685;
                                                        phi_14111_ = select(true, false, _e1640);
                                                    } else {
                                                        phi_32333_ = _e320.member_4.x;
                                                        phi_14108_ = vec3<f32>();
                                                        phi_14109_ = vec3<f32>();
                                                        phi_14110_ = vec3<f32>();
                                                        phi_14111_ = true;
                                                    }
                                                    let _e1688 = phi_32333_;
                                                    let _e1690 = phi_14108_;
                                                    let _e1692 = phi_14109_;
                                                    let _e1694 = phi_14110_;
                                                    let _e1696 = phi_14111_;
                                                    if _e1696 {
                                                        let _e1704 = (((_e965.member_3.x * _e320.member_1.x) + (_e965.member_3.y * _e320.member_1.y)) + (_e965.member_3.z * _e320.member_1.z));
                                                        phi_14126_ = vec3<f32>((_e1413 + (_e1580.x * _e1529)), (_e1414 + (_e1580.y * _e1529)), (_e1415 + (_e1580.z * _e1529)));
                                                        phi_14127_ = (_e336 - vec3<f32>(((_e965.member_3.x * 2f) * _e1704), ((_e965.member_3.y * 2f) * _e1704), ((_e965.member_3.z * 2f) * _e1704)));
                                                    } else {
                                                        phi_14126_ = _e1690;
                                                        phi_14127_ = _e1692;
                                                    }
                                                    let _e1718 = phi_14126_;
                                                    let _e1720 = phi_14127_;
                                                    phi_32331_ = _e1688;
                                                    phi_14129_ = _e1718;
                                                    phi_14130_ = _e1720;
                                                    phi_14131_ = select(_e1694, vec3<f32>(1f, 1f, 1f), vec3(_e1696));
                                                } else {
                                                    phi_32331_ = _e320.member_4.x;
                                                    phi_14129_ = vec3<f32>();
                                                    phi_14130_ = vec3<f32>();
                                                    phi_14131_ = vec3<f32>();
                                                }
                                                let _e1724 = phi_32331_;
                                                let _e1726 = phi_14129_;
                                                let _e1728 = phi_14130_;
                                                let _e1730 = phi_14131_;
                                                phi_32330_ = _e1724;
                                                phi_14132_ = _e1726;
                                                phi_14133_ = _e1728;
                                                phi_14134_ = _e1730;
                                                phi_14135_ = _e1571;
                                            }
                                            let _e1732 = phi_32330_;
                                            let _e1734 = phi_14132_;
                                            let _e1736 = phi_14133_;
                                            let _e1738 = phi_14134_;
                                            let _e1740 = phi_14135_;
                                            phi_32329_ = _e1732;
                                            phi_14136_ = _e1734;
                                            phi_14137_ = _e1736;
                                            phi_14138_ = _e1738;
                                            phi_14139_ = _e1570;
                                            phi_14140_ = _e1740;
                                        }
                                        let _e1742 = phi_32329_;
                                        let _e1744 = phi_14136_;
                                        let _e1746 = phi_14137_;
                                        let _e1748 = phi_14138_;
                                        let _e1750 = phi_14139_;
                                        let _e1752 = phi_14140_;
                                        if _e1750 {
                                            let _e1755 = (_e1480.member_6.y * _e1480.member_6.y);
                                            if (_e1755 != _e1755) {
                                                phi_25768_ = true;
                                            } else {
                                                phi_25768_ = (_e1546 >= _e1755);
                                            }
                                            let _e1759 = phi_25768_;
                                            let _e1760 = select(_e1755, _e1546, _e1759);
                                            let _e1772 = select(2u, 0u, (_e1480.member == 5u));
                                            let _e1773 = (_e1760 < 0.001f);
                                            if _e1773 {
                                                let _e1781 = (((_e1511.x * _e320.member_1.x) + (_e1511.y * _e320.member_1.y)) + (_e1511.z * _e320.member_1.z));
                                                let _e1792 = abs((((_e1483 * _e1511.x) + (_e1484 * _e1511.y)) + (_e1485 * _e1511.z)));
                                                if (_e1772 == 0u) {
                                                    let _e2074 = (1f - _e1792);
                                                    if (_e2074 != _e2074) {
                                                        phi_26299_ = true;
                                                    } else {
                                                        phi_26299_ = (0f >= _e2074);
                                                    }
                                                    let _e2078 = phi_26299_;
                                                    let _e2080 = pow(select(_e2074, 0f, _e2078), 5f);
                                                    let _e2081 = (1f - _e2080);
                                                    phi_14443_ = vec3<f32>(((_e1480.member_4.x * _e2081) + _e2080), ((_e1480.member_4.y * _e2081) + _e2080), ((_e1480.member_4.z * _e2081) + _e2080));
                                                } else {
                                                    if (_e1772 == 1u) {
                                                        let _e2056 = (0.44444448f * (1f - (_e1792 * _e1792)));
                                                        if (_e2056 >= 1f) {
                                                            phi_26260_ = 1f;
                                                        } else {
                                                            let _e2059 = sqrt((1f - _e2056));
                                                            if (_e1792 != _e1792) {
                                                                phi_26271_ = true;
                                                            } else {
                                                                phi_26271_ = (_e2059 <= _e1792);
                                                            }
                                                            let _e2063 = phi_26271_;
                                                            phi_26260_ = (0.040000003f + (0.96f * pow((1f - select(_e1792, _e2059, _e2063)), 5f)));
                                                        }
                                                        let _e2070 = phi_26260_;
                                                        phi_14442_ = vec3(_e2070);
                                                    } else {
                                                        let _e1796 = select(_e1792, 0f, (_e1792 < 0f));
                                                        let _e1798 = select(_e1796, 1f, (_e1796 > 1f));
                                                        let _e1800 = (1f - (_e1798 * _e1798));
                                                        let _e1807 = ((_e1480.member_4.x * _e1480.member_4.x) - (_e1480.member_5.x * _e1480.member_5.x));
                                                        let _e1808 = ((_e1480.member_4.y * _e1480.member_4.y) - (_e1480.member_5.y * _e1480.member_5.y));
                                                        let _e1809 = ((_e1480.member_4.z * _e1480.member_4.z) - (_e1480.member_5.z * _e1480.member_5.z));
                                                        let _e1811 = (_e1480.member_4.x * (_e1480.member_5.x + _e1480.member_5.x));
                                                        let _e1813 = (_e1480.member_4.y * (_e1480.member_5.y + _e1480.member_5.y));
                                                        let _e1815 = (_e1480.member_4.z * (_e1480.member_5.z + _e1480.member_5.z));
                                                        let _e1822 = ((_e1807 * _e1807) + (_e1811 * _e1811));
                                                        let _e1823 = ((_e1808 * _e1808) + (_e1813 * _e1813));
                                                        let _e1824 = ((_e1809 * _e1809) + (_e1815 * _e1815));
                                                        let _e1829 = ((_e1800 * _e1807) / _e1822);
                                                        let _e1830 = ((_e1800 * _e1808) / _e1823);
                                                        let _e1831 = ((_e1800 * _e1809) / _e1824);
                                                        let _e1832 = (1f - _e1829);
                                                        let _e1833 = (1f - _e1830);
                                                        let _e1834 = (1f - _e1831);
                                                        let _e1836 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1829, _e1830, _e1831));
                                                        let _e1837 = (_e1800 * _e1811);
                                                        let _e1838 = (_e1800 * _e1813);
                                                        let _e1839 = (_e1800 * _e1815);
                                                        let _e1844 = (vec3<f32>(_e1837, _e1838, _e1839) / vec3<f32>(_e1822, _e1823, _e1824));
                                                        let _e1857 = ((_e1836.x * _e1836.x) + (_e1844.x * _e1844.x));
                                                        let _e1858 = ((_e1836.y * _e1836.y) + (_e1844.y * _e1844.y));
                                                        let _e1859 = ((_e1836.z * _e1836.z) + (_e1844.z * _e1844.z));
                                                        if (_e1857 != _e1857) {
                                                            phi_25930_ = true;
                                                        } else {
                                                            phi_25930_ = (0f >= _e1857);
                                                        }
                                                        let _e1863 = phi_25930_;
                                                        let _e1865 = sqrt(select(_e1857, 0f, _e1863));
                                                        if (_e1858 != _e1858) {
                                                            phi_25945_ = true;
                                                        } else {
                                                            phi_25945_ = (0f >= _e1858);
                                                        }
                                                        let _e1869 = phi_25945_;
                                                        let _e1871 = sqrt(select(_e1858, 0f, _e1869));
                                                        if (_e1859 != _e1859) {
                                                            phi_25960_ = true;
                                                        } else {
                                                            phi_25960_ = (0f >= _e1859);
                                                        }
                                                        let _e1875 = phi_25960_;
                                                        let _e1877 = sqrt(select(_e1859, 0f, _e1875));
                                                        let _e1881 = ((_e1865 + _e1832) * 0.5f);
                                                        let _e1882 = ((_e1871 + _e1833) * 0.5f);
                                                        let _e1883 = ((_e1877 + _e1834) * 0.5f);
                                                        if (_e1881 != _e1881) {
                                                            phi_25987_ = true;
                                                        } else {
                                                            phi_25987_ = (0f >= _e1881);
                                                        }
                                                        let _e1887 = phi_25987_;
                                                        let _e1889 = sqrt(select(_e1881, 0f, _e1887));
                                                        if (_e1882 != _e1882) {
                                                            phi_26002_ = true;
                                                        } else {
                                                            phi_26002_ = (0f >= _e1882);
                                                        }
                                                        let _e1893 = phi_26002_;
                                                        let _e1895 = sqrt(select(_e1882, 0f, _e1893));
                                                        if (_e1883 != _e1883) {
                                                            phi_26017_ = true;
                                                        } else {
                                                            phi_26017_ = (0f >= _e1883);
                                                        }
                                                        let _e1899 = phi_26017_;
                                                        let _e1901 = sqrt(select(_e1883, 0f, _e1899));
                                                        let _e1902 = vec3<f32>(_e1889, _e1895, _e1901);
                                                        let _e1906 = ((_e1865 - _e1832) * 0.5f);
                                                        let _e1907 = ((_e1871 - _e1833) * 0.5f);
                                                        let _e1908 = ((_e1877 - _e1834) * 0.5f);
                                                        if (_e1906 != _e1906) {
                                                            phi_26044_ = true;
                                                        } else {
                                                            phi_26044_ = (0f >= _e1906);
                                                        }
                                                        let _e1912 = phi_26044_;
                                                        let _e1914 = sqrt(select(_e1906, 0f, _e1912));
                                                        if (_e1907 != _e1907) {
                                                            phi_26059_ = true;
                                                        } else {
                                                            phi_26059_ = (0f >= _e1907);
                                                        }
                                                        let _e1918 = phi_26059_;
                                                        let _e1920 = sqrt(select(_e1907, 0f, _e1918));
                                                        if (_e1908 != _e1908) {
                                                            phi_26074_ = true;
                                                        } else {
                                                            phi_26074_ = (0f >= _e1908);
                                                        }
                                                        let _e1924 = phi_26074_;
                                                        let _e1926 = sqrt(select(_e1908, 0f, _e1924));
                                                        if ((_e1837 / _e1822) < 0f) {
                                                            phi_14319_ = -(_e1914);
                                                        } else {
                                                            phi_14319_ = _e1914;
                                                        }
                                                        let _e1930 = phi_14319_;
                                                        if ((_e1838 / _e1823) < 0f) {
                                                            phi_14327_ = -(_e1920);
                                                        } else {
                                                            phi_14327_ = _e1920;
                                                        }
                                                        let _e1934 = phi_14327_;
                                                        if ((_e1839 / _e1824) < 0f) {
                                                            phi_14335_ = -(_e1926);
                                                        } else {
                                                            phi_14335_ = _e1926;
                                                        }
                                                        let _e1938 = phi_14335_;
                                                        let _e1942 = (_e1480.member_5.x * _e1798);
                                                        let _e1943 = (_e1480.member_5.y * _e1798);
                                                        let _e1944 = (_e1480.member_5.z * _e1798);
                                                        let _e1954 = vec3<f32>(((_e1480.member_4.x * _e1889) - (_e1480.member_5.x * _e1930)), ((_e1480.member_4.y * _e1895) - (_e1480.member_5.y * _e1934)), ((_e1480.member_4.z * _e1901) - (_e1480.member_5.z * _e1938)));
                                                        let _e1961 = ((_e1480.member_4.x * _e1930) + (_e1480.member_5.x * _e1889));
                                                        let _e1962 = ((_e1480.member_4.y * _e1934) + (_e1480.member_5.y * _e1895));
                                                        let _e1963 = ((_e1480.member_4.z * _e1938) + (_e1480.member_5.z * _e1901));
                                                        let _e1965 = vec3<f32>((_e1480.member_4.x * _e1798), (_e1480.member_4.y * _e1798), (_e1480.member_4.z * _e1798));
                                                        let _e1966 = (_e1965 - _e1902);
                                                        let _e1967 = (_e1942 - _e1930);
                                                        let _e1968 = (_e1943 - _e1934);
                                                        let _e1969 = (_e1944 - _e1938);
                                                        let _e1982 = (_e1965 + _e1902);
                                                        let _e1983 = (_e1942 + _e1930);
                                                        let _e1984 = (_e1943 + _e1934);
                                                        let _e1985 = (_e1944 + _e1938);
                                                        let _e1995 = ((_e1982.x * _e1982.x) + (_e1983 * _e1983));
                                                        let _e1996 = ((_e1982.y * _e1982.y) + (_e1984 * _e1984));
                                                        let _e1997 = ((_e1982.z * _e1982.z) + (_e1985 * _e1985));
                                                        let _e2007 = vec3(_e1798);
                                                        let _e2008 = (_e2007 - _e1954);
                                                        let _e2009 = -(vec3<f32>(_e1961, _e1962, _e1963));
                                                        let _e2025 = (_e2007 + _e1954);
                                                        let _e2035 = ((_e2025.x * _e2025.x) + (_e1961 * _e1961));
                                                        let _e2036 = ((_e2025.y * _e2025.y) + (_e1962 * _e1962));
                                                        let _e2037 = ((_e2025.z * _e2025.z) + (_e1963 * _e1963));
                                                        phi_14442_ = vec3<f32>((((((_e1966.x * _e1966.x) + (_e1967 * _e1967)) / select(0.000000000000000000000000000001f, _e1995, (_e1995 > 0.000000000000000000000000000001f))) + (((_e2008.x * _e2008.x) + (_e2009.x * _e2009.x)) / select(0.000000000000000000000000000001f, _e2035, (_e2035 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1966.y * _e1966.y) + (_e1968 * _e1968)) / select(0.000000000000000000000000000001f, _e1996, (_e1996 > 0.000000000000000000000000000001f))) + (((_e2008.y * _e2008.y) + (_e2009.y * _e2009.y)) / select(0.000000000000000000000000000001f, _e2036, (_e2036 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1966.z * _e1966.z) + (_e1969 * _e1969)) / select(0.000000000000000000000000000001f, _e1997, (_e1997 > 0.000000000000000000000000000001f))) + (((_e2008.z * _e2008.z) + (_e2009.z * _e2009.z)) / select(0.000000000000000000000000000001f, _e2037, (_e2037 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e2073 = phi_14442_;
                                                    phi_14443_ = _e2073;
                                                }
                                                let _e2090 = phi_14443_;
                                                phi_14444_ = (_e336 - vec3<f32>(((_e1511.x * 2f) * _e1781), ((_e1511.y * 2f) * _e1781), ((_e1511.z * 2f) * _e1781)));
                                                phi_14445_ = _e2090;
                                            } else {
                                                phi_14444_ = vec3<f32>();
                                                phi_14445_ = vec3<f32>();
                                            }
                                            let _e2092 = phi_14444_;
                                            let _e2094 = phi_14445_;
                                            phi_32717_ = vec3<f32>(_e1480.member_5.x, _e1480.member_5.y, _e1480.member_5.z);
                                            phi_32640_ = vec3<f32>(_e1480.member_4.x, _e1480.member_4.y, _e1480.member_4.z);
                                            phi_32561_ = _e1772;
                                            phi_32479_ = _e1760;
                                            phi_14447_ = _e2092;
                                            phi_14448_ = _e2094;
                                            phi_14449_ = _e1773;
                                        } else {
                                            phi_32717_ = vec3<f32>(0f, 0f, 0f);
                                            phi_32640_ = vec3<f32>(0f, 0f, 0f);
                                            phi_32561_ = 0u;
                                            phi_32479_ = 1f;
                                            phi_14447_ = _e1746;
                                            phi_14448_ = _e1748;
                                            phi_14449_ = _e1752;
                                        }
                                        let _e2096 = phi_32717_;
                                        let _e2098 = phi_32640_;
                                        let _e2100 = phi_32561_;
                                        let _e2102 = phi_32479_;
                                        let _e2104 = phi_14447_;
                                        let _e2106 = phi_14448_;
                                        let _e2108 = phi_14449_;
                                        phi_32986_ = vec3<f32>(0f, 0f, 0f);
                                        phi_32794_ = 1.5f;
                                        phi_32715_ = _e2096;
                                        phi_32638_ = _e2098;
                                        phi_32559_ = _e2100;
                                        phi_32477_ = _e2102;
                                        phi_32400_ = select(0u, 1u, _e1750);
                                        phi_32326_ = _e1742;
                                        phi_14457_ = select(_e1744, _e1539, vec3(_e1750));
                                        phi_14458_ = _e2104;
                                        phi_14459_ = _e2106;
                                        phi_14460_ = select(true, false, _e2108);
                                    }
                                    let _e2218 = phi_32986_;
                                    let _e2220 = phi_32794_;
                                    let _e2222 = phi_32715_;
                                    let _e2224 = phi_32638_;
                                    let _e2226 = phi_32559_;
                                    let _e2228 = phi_32477_;
                                    let _e2230 = phi_32400_;
                                    let _e2232 = phi_32326_;
                                    let _e2234 = phi_14457_;
                                    let _e2236 = phi_14458_;
                                    let _e2238 = phi_14459_;
                                    let _e2240 = phi_14460_;
                                    let _e2241 = select(true, false, _e2240);
                                    if _e2241 {
                                        if (_e347 >= _e363) {
                                            phi_32385_ = _e2232;
                                            phi_14576_ = _e1406;
                                            phi_14577_ = type_69();
                                            phi_14578_ = true;
                                        } else {
                                            let _e2254 = global_1.member.member_4[1u];
                                            if (sqrt((((_e2238.x * _e2238.x) + (_e2238.y * _e2238.y)) + (_e2238.z * _e2238.z))) <= _e2254) {
                                                phi_32380_ = _e2232;
                                                phi_14540_ = _e1406;
                                                phi_14541_ = false;
                                            } else {
                                                let _e2258 = global_1.member.member_7[1u];
                                                if (_e2258 == 1u) {
                                                    let _e2263 = global_1.member.member_4[3u];
                                                    if (f32(_e347) >= _e2263) {
                                                        let _e2266 = select(_e2238.y, _e2238.x, (_e2238.x > _e2238.y));
                                                        let _e2268 = select(_e2238.z, _e2266, (_e2266 > _e2238.z));
                                                        if (_e2268 != _e2268) {
                                                            phi_26431_ = true;
                                                        } else {
                                                            phi_26431_ = (1f <= _e2268);
                                                        }
                                                        let _e2272 = phi_26431_;
                                                        let _e2273 = select(_e2268, 1f, _e2272);
                                                        let _e2275 = ((_e2232 * 747796405u) + 2891336453u);
                                                        let _e2283 = (((_e2275 >> bitcast<u32>((((_e2275 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2275) * 277803737u);
                                                        let _e2286 = ((_e2283 >> bitcast<u32>(22i)) ^ _e2283);
                                                        let _e2291 = ((f32((_e2286 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e2273);
                                                        if _e2291 {
                                                            phi_14529_ = vec3<f32>();
                                                        } else {
                                                            phi_14529_ = vec3<f32>((_e1406.x / _e2273), (_e1406.y / _e2273), (_e1406.z / _e2273));
                                                        }
                                                        let _e2300 = phi_14529_;
                                                        phi_32383_ = _e2286;
                                                        phi_14531_ = _e2300;
                                                        phi_14532_ = select(true, false, _e2291);
                                                    } else {
                                                        phi_32383_ = _e2232;
                                                        phi_14531_ = _e1406;
                                                        phi_14532_ = true;
                                                    }
                                                    let _e2303 = phi_32383_;
                                                    let _e2305 = phi_14531_;
                                                    let _e2307 = phi_14532_;
                                                    phi_32382_ = _e2303;
                                                    phi_14533_ = _e2305;
                                                    phi_14534_ = _e2307;
                                                } else {
                                                    phi_32382_ = _e2232;
                                                    phi_14533_ = _e1406;
                                                    phi_14534_ = true;
                                                }
                                                let _e2309 = phi_32382_;
                                                let _e2311 = phi_14533_;
                                                let _e2313 = phi_14534_;
                                                phi_32380_ = _e2309;
                                                phi_14540_ = select(_e1406, _e2311, vec3(_e2313));
                                                phi_14541_ = select(false, true, _e2313);
                                            }
                                            let _e2318 = phi_32380_;
                                            let _e2320 = phi_14540_;
                                            let _e2322 = phi_14541_;
                                            if _e2322 {
                                                let _e2323 = (_e2320 * _e2238);
                                                let _e2334 = (_e2236 * (1f / sqrt((((_e2236.x * _e2236.x) + (_e2236.y * _e2236.y)) + (_e2236.z * _e2236.z)))));
                                                phi_14574_ = type_69(type_35(vec4<f32>(_e2234.x, _e2234.y, _e2234.z, _e320.member.w), vec4<f32>(_e2334.x, _e2334.y, _e2334.z, _e320.member_1.w), vec4<f32>(_e2323.x, _e2323.y, _e2323.z, f32((_e347 + 1u))), vec4<f32>(_e320.member_3.x, _e320.member_3.y, _e320.member_3.z, f32(_e966)), vec4<u32>(_e2318, (_e320.member_4.y | 2u), 0u, 0u)), _e1408, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                            } else {
                                                phi_14574_ = type_69();
                                            }
                                            let _e2357 = phi_14574_;
                                            phi_32385_ = _e2318;
                                            phi_14576_ = _e2320;
                                            phi_14577_ = _e2357;
                                            phi_14578_ = select(true, false, _e2322);
                                        }
                                        let _e2360 = phi_32385_;
                                        let _e2362 = phi_14576_;
                                        let _e2364 = phi_14577_;
                                        let _e2366 = phi_14578_;
                                        if _e2366 {
                                            phi_14584_ = type_69(type_35(_e320.member, _e320.member_1, vec4<f32>(_e2362.x, _e2362.y, _e2362.z, f32(_e347)), vec4<f32>(_e320.member_3.x, _e320.member_3.y, _e320.member_3.z, f32(_e966)), vec4<u32>(_e2360, (_e320.member_4.y & 4294967294u), 0u, 0u)), _e1408, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14584_ = _e2364;
                                        }
                                        let _e2379 = phi_14584_;
                                        phi_32389_ = _e2360;
                                        phi_14585_ = _e2379;
                                    } else {
                                        phi_32389_ = _e2232;
                                        phi_14585_ = type_69();
                                    }
                                    let _e2381 = phi_32389_;
                                    let _e2383 = phi_14585_;
                                    phi_32983_ = _e2218;
                                    phi_32791_ = _e2220;
                                    phi_32712_ = _e2222;
                                    phi_32635_ = _e2224;
                                    phi_32556_ = _e2226;
                                    phi_32474_ = _e2228;
                                    phi_32397_ = _e2230;
                                    phi_32388_ = _e2381;
                                    phi_14587_ = _e2383;
                                    phi_14588_ = select(true, false, _e2241);
                                }
                                let _e2391 = phi_32983_;
                                let _e2393 = phi_32791_;
                                let _e2395 = phi_32712_;
                                let _e2397 = phi_32635_;
                                let _e2399 = phi_32556_;
                                let _e2401 = phi_32474_;
                                let _e2403 = phi_32397_;
                                let _e2405 = phi_32388_;
                                let _e2407 = phi_14587_;
                                let _e2409 = phi_14588_;
                                if _e2409 {
                                    let _e2411 = ((_e320.member_4.y | 4u) & 4294967293u);
                                    let _e2414 = global_1.member.member_1[3u];
                                    phi_34320_ = _e1404;
                                    phi_32387_ = _e2405;
                                    phi_14599_ = _e351;
                                    phi_14602_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                    phi_14604_ = 0u;
                                    phi_14606_ = _e966;
                                    loop {
                                        let _e2416 = phi_34320_;
                                        let _e2418 = phi_32387_;
                                        let _e2420 = phi_14599_;
                                        let _e2422 = phi_14602_;
                                        let _e2424 = phi_14604_;
                                        let _e2426 = phi_14606_;
                                        local_85 = _e2426;
                                        local_86 = _e2420;
                                        local_87 = _e2420;
                                        local_88 = _e2420;
                                        local_89 = _e2422;
                                        local_90 = _e2426;
                                        local_91 = _e2420;
                                        local_92 = _e2420;
                                        local_93 = _e2420;
                                        local_94 = _e2422;
                                        local_95 = _e2426;
                                        local_96 = _e2420;
                                        local_97 = _e2420;
                                        local_98 = _e2420;
                                        local_99 = _e2422;
                                        local_100 = _e2426;
                                        local_101 = _e2420;
                                        local_102 = _e2420;
                                        local_103 = _e2420;
                                        local_104 = _e2422;
                                        let _e2428 = (_e2424 < (_e2414 + 3u));
                                        if _e2428 {
                                            if (_e2424 < _e2414) {
                                                let _e4413 = global_5.member[_e2424];
                                                let _e4418 = (_e4413.member.x - _e1536);
                                                let _e4419 = (_e4413.member.y - _e1537);
                                                let _e4420 = (_e4413.member.z - _e1538);
                                                let _e4425 = (((_e4418 * _e4418) + (_e4419 * _e4419)) + (_e4420 * _e4420));
                                                let _e4426 = sqrt(_e4425);
                                                let _e4427 = (_e4418 / _e4426);
                                                let _e4428 = (_e4419 / _e4426);
                                                let _e4429 = (_e4420 / _e4426);
                                                if (_e4413.member.w < 0.5f) {
                                                    phi_14703_ = 1f;
                                                } else {
                                                    let _e4453 = (((_e4413.member_2.x * -(_e4427)) + (_e4413.member_2.y * -(_e4428))) + (_e4413.member_2.z * -(_e4429)));
                                                    if (_e4413.member_3.x == _e4413.member_2.w) {
                                                        phi_14702_ = select(1f, 0f, (_e4453 < _e4413.member_3.x));
                                                    } else {
                                                        let _e4457 = ((_e4453 - _e4413.member_3.x) / (_e4413.member_2.w - _e4413.member_3.x));
                                                        let _e4459 = select(_e4457, 0f, (_e4457 < 0f));
                                                        let _e4461 = select(_e4459, 1f, (_e4459 > 1f));
                                                        phi_14702_ = ((_e4461 * _e4461) * (3f - (2f * _e4461)));
                                                    }
                                                    let _e4469 = phi_14702_;
                                                    phi_14703_ = _e4469;
                                                }
                                                let _e4471 = phi_14703_;
                                                phi_34681_ = _e2418;
                                                phi_17641_ = 0f;
                                                phi_17642_ = vec3<f32>(_e4427, _e4428, _e4429);
                                                phi_17643_ = vec3<f32>(((_e4413.member_1.x * _e4471) / _e4425), ((_e4413.member_1.y * _e4471) / _e4425), ((_e4413.member_1.z * _e4471) / _e4425));
                                                phi_17644_ = _e4426;
                                            } else {
                                                if (_e2424 == _e2414) {
                                                    if ((_e366 & 4u) == 0u) {
                                                        phi_34725_ = _e2418;
                                                        phi_16745_ = 0f;
                                                        phi_16746_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_16747_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e3165 = global_1.member.member_5;
                                                        let _e3173 = global_1.member.member_6[3u];
                                                        let _e3175 = sin((_e3173 * 0.5f));
                                                        let _e3177 = ((12.566371f * _e3175) * _e3175);
                                                        let _e3182 = (((_e3165.x * _e1511.x) + (_e3165.y * _e1511.y)) + (_e3165.z * _e1511.z));
                                                        if (_e3182 != _e3182) {
                                                            phi_26970_ = true;
                                                        } else {
                                                            phi_26970_ = (0f >= _e3182);
                                                        }
                                                        let _e3186 = phi_26970_;
                                                        let _e3188 = (select(_e3182, 0f, _e3186) * 0.31830987f);
                                                        if (_e2403 == 0u) {
                                                            if (_e3182 <= 0f) {
                                                                phi_27767_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_27767_ = (_e2397 * (_e3182 * 0.31830987f));
                                                            }
                                                            let _e3664 = phi_27767_;
                                                            phi_15113_ = type_57(_e3664, _e3188);
                                                            phi_15114_ = type_57();
                                                            phi_15115_ = true;
                                                        } else {
                                                            let _e3194 = (((_e1483 * _e1559) + (_e1484 * _e1560)) + (_e1485 * _e1562));
                                                            let _e3199 = (((_e1483 * _e1555) + (_e1484 * _e1565)) + (_e1485 * _e1566));
                                                            let _e3204 = (((_e1483 * _e1511.x) + (_e1484 * _e1511.y)) + (_e1485 * _e1511.z));
                                                            let _e3209 = (((_e3165.x * _e1559) + (_e3165.y * _e1560)) + (_e3165.z * _e1562));
                                                            let _e3214 = (((_e3165.x * _e1555) + (_e3165.y * _e1565)) + (_e3165.z * _e1566));
                                                            if (_e3204 <= 0f) {
                                                                phi_15077_ = type_57();
                                                                phi_15078_ = true;
                                                            } else {
                                                                let _e3216 = (_e3182 <= 0f);
                                                                if _e3216 {
                                                                    phi_15076_ = type_57();
                                                                } else {
                                                                    let _e3217 = (_e3194 + _e3209);
                                                                    let _e3218 = (_e3199 + _e3214);
                                                                    let _e3219 = (_e3204 + _e3182);
                                                                    let _e3226 = (1f / sqrt((((_e3217 * _e3217) + (_e3218 * _e3218)) + (_e3219 * _e3219))));
                                                                    let _e3229 = (_e3219 * _e3226);
                                                                    if (_e3229 <= 0f) {
                                                                        phi_27080_ = 0f;
                                                                    } else {
                                                                        let _e3231 = (_e2401 * _e2401);
                                                                        let _e3235 = ((((_e3231 - 1f) * _e3229) * _e3229) + 1f);
                                                                        phi_27080_ = (_e3231 / ((3.1415927f * _e3235) * _e3235));
                                                                    }
                                                                    let _e3240 = phi_27080_;
                                                                    let _e3241 = (_e3204 * _e3204);
                                                                    if (_e3241 >= 1f) {
                                                                        phi_27108_ = 0f;
                                                                    } else {
                                                                        phi_27108_ = ((sqrt((1f + (((_e2401 * _e2401) * ((_e3194 * _e3194) + (_e3199 * _e3199))) / _e3241))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3254 = phi_27108_;
                                                                    let _e3255 = (1f + _e3254);
                                                                    let _e3256 = (_e3182 * _e3182);
                                                                    if (_e3256 >= 1f) {
                                                                        phi_27136_ = 0f;
                                                                    } else {
                                                                        phi_27136_ = ((sqrt((1f + (((_e2401 * _e2401) * ((_e3209 * _e3209) + (_e3214 * _e3214))) / _e3256))) - 1f) * 0.5f);
                                                                    }
                                                                    let _e3269 = phi_27136_;
                                                                    let _e3276 = (((_e3194 * (_e3217 * _e3226)) + (_e3199 * (_e3218 * _e3226))) + (_e3204 * _e3229));
                                                                    if (_e2399 == 0u) {
                                                                        let _e3571 = (1f - _e3276);
                                                                        if (_e3571 != _e3571) {
                                                                            phi_27639_ = true;
                                                                        } else {
                                                                            phi_27639_ = (0f >= _e3571);
                                                                        }
                                                                        let _e3575 = phi_27639_;
                                                                        let _e3577 = pow(select(_e3571, 0f, _e3575), 5f);
                                                                        let _e3578 = (1f - _e3577);
                                                                        phi_15064_ = vec3<f32>(((_e2397.x * _e3578) + _e3577), ((_e2397.y * _e3578) + _e3577), ((_e2397.z * _e3578) + _e3577));
                                                                    } else {
                                                                        if (_e2399 == 1u) {
                                                                            let _e3544 = (1f / _e2393);
                                                                            let _e3548 = ((_e3544 * _e3544) * (1f - (_e3276 * _e3276)));
                                                                            if (_e3548 >= 1f) {
                                                                                phi_27600_ = 1f;
                                                                            } else {
                                                                                let _e3551 = sqrt((1f - _e3548));
                                                                                if (_e3276 != _e3276) {
                                                                                    phi_27611_ = true;
                                                                                } else {
                                                                                    phi_27611_ = (_e3551 <= _e3276);
                                                                                }
                                                                                let _e3555 = phi_27611_;
                                                                                let _e3560 = pow(((1f - _e2393) / (1f + _e2393)), 2f);
                                                                                phi_27600_ = (_e3560 + ((1f - _e3560) * pow((1f - select(_e3276, _e3551, _e3555)), 5f)));
                                                                            }
                                                                            let _e3567 = phi_27600_;
                                                                            phi_15063_ = vec3(_e3567);
                                                                        } else {
                                                                            let _e3280 = select(_e3276, 0f, (_e3276 < 0f));
                                                                            let _e3282 = select(_e3280, 1f, (_e3280 > 1f));
                                                                            let _e3284 = (1f - (_e3282 * _e3282));
                                                                            let _e3297 = ((_e2397.x * _e2397.x) - (_e2395.x * _e2395.x));
                                                                            let _e3298 = ((_e2397.y * _e2397.y) - (_e2395.y * _e2395.y));
                                                                            let _e3299 = ((_e2397.z * _e2397.z) - (_e2395.z * _e2395.z));
                                                                            let _e3301 = (_e2397.x * (_e2395.x + _e2395.x));
                                                                            let _e3303 = (_e2397.y * (_e2395.y + _e2395.y));
                                                                            let _e3305 = (_e2397.z * (_e2395.z + _e2395.z));
                                                                            let _e3312 = ((_e3297 * _e3297) + (_e3301 * _e3301));
                                                                            let _e3313 = ((_e3298 * _e3298) + (_e3303 * _e3303));
                                                                            let _e3314 = ((_e3299 * _e3299) + (_e3305 * _e3305));
                                                                            let _e3319 = ((_e3284 * _e3297) / _e3312);
                                                                            let _e3320 = ((_e3284 * _e3298) / _e3313);
                                                                            let _e3321 = ((_e3284 * _e3299) / _e3314);
                                                                            let _e3322 = (1f - _e3319);
                                                                            let _e3323 = (1f - _e3320);
                                                                            let _e3324 = (1f - _e3321);
                                                                            let _e3326 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3319, _e3320, _e3321));
                                                                            let _e3327 = (_e3284 * _e3301);
                                                                            let _e3328 = (_e3284 * _e3303);
                                                                            let _e3329 = (_e3284 * _e3305);
                                                                            let _e3334 = (vec3<f32>(_e3327, _e3328, _e3329) / vec3<f32>(_e3312, _e3313, _e3314));
                                                                            let _e3347 = ((_e3326.x * _e3326.x) + (_e3334.x * _e3334.x));
                                                                            let _e3348 = ((_e3326.y * _e3326.y) + (_e3334.y * _e3334.y));
                                                                            let _e3349 = ((_e3326.z * _e3326.z) + (_e3334.z * _e3334.z));
                                                                            if (_e3347 != _e3347) {
                                                                                phi_27270_ = true;
                                                                            } else {
                                                                                phi_27270_ = (0f >= _e3347);
                                                                            }
                                                                            let _e3353 = phi_27270_;
                                                                            let _e3355 = sqrt(select(_e3347, 0f, _e3353));
                                                                            if (_e3348 != _e3348) {
                                                                                phi_27285_ = true;
                                                                            } else {
                                                                                phi_27285_ = (0f >= _e3348);
                                                                            }
                                                                            let _e3359 = phi_27285_;
                                                                            let _e3361 = sqrt(select(_e3348, 0f, _e3359));
                                                                            if (_e3349 != _e3349) {
                                                                                phi_27300_ = true;
                                                                            } else {
                                                                                phi_27300_ = (0f >= _e3349);
                                                                            }
                                                                            let _e3365 = phi_27300_;
                                                                            let _e3367 = sqrt(select(_e3349, 0f, _e3365));
                                                                            let _e3371 = ((_e3355 + _e3322) * 0.5f);
                                                                            let _e3372 = ((_e3361 + _e3323) * 0.5f);
                                                                            let _e3373 = ((_e3367 + _e3324) * 0.5f);
                                                                            if (_e3371 != _e3371) {
                                                                                phi_27327_ = true;
                                                                            } else {
                                                                                phi_27327_ = (0f >= _e3371);
                                                                            }
                                                                            let _e3377 = phi_27327_;
                                                                            let _e3379 = sqrt(select(_e3371, 0f, _e3377));
                                                                            if (_e3372 != _e3372) {
                                                                                phi_27342_ = true;
                                                                            } else {
                                                                                phi_27342_ = (0f >= _e3372);
                                                                            }
                                                                            let _e3383 = phi_27342_;
                                                                            let _e3385 = sqrt(select(_e3372, 0f, _e3383));
                                                                            if (_e3373 != _e3373) {
                                                                                phi_27357_ = true;
                                                                            } else {
                                                                                phi_27357_ = (0f >= _e3373);
                                                                            }
                                                                            let _e3389 = phi_27357_;
                                                                            let _e3391 = sqrt(select(_e3373, 0f, _e3389));
                                                                            let _e3392 = vec3<f32>(_e3379, _e3385, _e3391);
                                                                            let _e3396 = ((_e3355 - _e3322) * 0.5f);
                                                                            let _e3397 = ((_e3361 - _e3323) * 0.5f);
                                                                            let _e3398 = ((_e3367 - _e3324) * 0.5f);
                                                                            if (_e3396 != _e3396) {
                                                                                phi_27384_ = true;
                                                                            } else {
                                                                                phi_27384_ = (0f >= _e3396);
                                                                            }
                                                                            let _e3402 = phi_27384_;
                                                                            let _e3404 = sqrt(select(_e3396, 0f, _e3402));
                                                                            if (_e3397 != _e3397) {
                                                                                phi_27399_ = true;
                                                                            } else {
                                                                                phi_27399_ = (0f >= _e3397);
                                                                            }
                                                                            let _e3408 = phi_27399_;
                                                                            let _e3410 = sqrt(select(_e3397, 0f, _e3408));
                                                                            if (_e3398 != _e3398) {
                                                                                phi_27414_ = true;
                                                                            } else {
                                                                                phi_27414_ = (0f >= _e3398);
                                                                            }
                                                                            let _e3414 = phi_27414_;
                                                                            let _e3416 = sqrt(select(_e3398, 0f, _e3414));
                                                                            if ((_e3327 / _e3312) < 0f) {
                                                                                phi_14940_ = -(_e3404);
                                                                            } else {
                                                                                phi_14940_ = _e3404;
                                                                            }
                                                                            let _e3420 = phi_14940_;
                                                                            if ((_e3328 / _e3313) < 0f) {
                                                                                phi_14948_ = -(_e3410);
                                                                            } else {
                                                                                phi_14948_ = _e3410;
                                                                            }
                                                                            let _e3424 = phi_14948_;
                                                                            if ((_e3329 / _e3314) < 0f) {
                                                                                phi_14956_ = -(_e3416);
                                                                            } else {
                                                                                phi_14956_ = _e3416;
                                                                            }
                                                                            let _e3428 = phi_14956_;
                                                                            let _e3432 = (_e2395.x * _e3282);
                                                                            let _e3433 = (_e2395.y * _e3282);
                                                                            let _e3434 = (_e2395.z * _e3282);
                                                                            let _e3444 = vec3<f32>(((_e2397.x * _e3379) - (_e2395.x * _e3420)), ((_e2397.y * _e3385) - (_e2395.y * _e3424)), ((_e2397.z * _e3391) - (_e2395.z * _e3428)));
                                                                            let _e3451 = ((_e2397.x * _e3420) + (_e2395.x * _e3379));
                                                                            let _e3452 = ((_e2397.y * _e3424) + (_e2395.y * _e3385));
                                                                            let _e3453 = ((_e2397.z * _e3428) + (_e2395.z * _e3391));
                                                                            let _e3455 = vec3<f32>((_e2397.x * _e3282), (_e2397.y * _e3282), (_e2397.z * _e3282));
                                                                            let _e3456 = (_e3455 - _e3392);
                                                                            let _e3457 = (_e3432 - _e3420);
                                                                            let _e3458 = (_e3433 - _e3424);
                                                                            let _e3459 = (_e3434 - _e3428);
                                                                            let _e3472 = (_e3455 + _e3392);
                                                                            let _e3473 = (_e3432 + _e3420);
                                                                            let _e3474 = (_e3433 + _e3424);
                                                                            let _e3475 = (_e3434 + _e3428);
                                                                            let _e3485 = ((_e3472.x * _e3472.x) + (_e3473 * _e3473));
                                                                            let _e3486 = ((_e3472.y * _e3472.y) + (_e3474 * _e3474));
                                                                            let _e3487 = ((_e3472.z * _e3472.z) + (_e3475 * _e3475));
                                                                            let _e3497 = vec3(_e3282);
                                                                            let _e3498 = (_e3497 - _e3444);
                                                                            let _e3499 = -(vec3<f32>(_e3451, _e3452, _e3453));
                                                                            let _e3515 = (_e3497 + _e3444);
                                                                            let _e3525 = ((_e3515.x * _e3515.x) + (_e3451 * _e3451));
                                                                            let _e3526 = ((_e3515.y * _e3515.y) + (_e3452 * _e3452));
                                                                            let _e3527 = ((_e3515.z * _e3515.z) + (_e3453 * _e3453));
                                                                            phi_15063_ = vec3<f32>((((((_e3456.x * _e3456.x) + (_e3457 * _e3457)) / select(0.000000000000000000000000000001f, _e3485, (_e3485 > 0.000000000000000000000000000001f))) + (((_e3498.x * _e3498.x) + (_e3499.x * _e3499.x)) / select(0.000000000000000000000000000001f, _e3525, (_e3525 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3456.y * _e3456.y) + (_e3458 * _e3458)) / select(0.000000000000000000000000000001f, _e3486, (_e3486 > 0.000000000000000000000000000001f))) + (((_e3498.y * _e3498.y) + (_e3499.y * _e3499.y)) / select(0.000000000000000000000000000001f, _e3526, (_e3526 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3456.z * _e3456.z) + (_e3459 * _e3459)) / select(0.000000000000000000000000000001f, _e3487, (_e3487 > 0.000000000000000000000000000001f))) + (((_e3498.z * _e3498.z) + (_e3499.z * _e3499.z)) / select(0.000000000000000000000000000001f, _e3527, (_e3527 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                        }
                                                                        let _e3570 = phi_15063_;
                                                                        phi_15064_ = _e3570;
                                                                    }
                                                                    let _e3590 = phi_15064_;
                                                                    let _e3592 = (4f * _e3204);
                                                                    phi_15076_ = type_57((_e3590 * (((1f / (_e3255 + _e3269)) * _e3240) / _e3592)), ((_e3240 / _e3255) / _e3592));
                                                                }
                                                                let _e3599 = phi_15076_;
                                                                phi_15077_ = _e3599;
                                                                phi_15078_ = _e3216;
                                                            }
                                                            let _e3601 = phi_15077_;
                                                            let _e3603 = phi_15078_;
                                                            if _e3603 {
                                                                phi_15083_ = type_57(vec3<f32>(0f, 0f, 0f), 0f);
                                                            } else {
                                                                phi_15083_ = _e3601;
                                                            }
                                                            let _e3605 = phi_15083_;
                                                            let _e3608 = (_e2403 == 1u);
                                                            if _e3608 {
                                                                phi_15111_ = _e3605;
                                                                phi_15112_ = type_57();
                                                            } else {
                                                                if (_e3204 != _e3204) {
                                                                    phi_27676_ = true;
                                                                } else {
                                                                    phi_27676_ = (0f >= _e3204);
                                                                }
                                                                let _e3612 = phi_27676_;
                                                                let _e3613 = select(_e3204, 0f, _e3612);
                                                                let _e3614 = (1f / _e2393);
                                                                let _e3618 = ((_e3614 * _e3614) * (1f - (_e3613 * _e3613)));
                                                                if (_e3618 >= 1f) {
                                                                    phi_27695_ = 1f;
                                                                } else {
                                                                    let _e3621 = sqrt((1f - _e3618));
                                                                    if (_e3613 != _e3613) {
                                                                        phi_27706_ = true;
                                                                    } else {
                                                                        phi_27706_ = (_e3621 <= _e3613);
                                                                    }
                                                                    let _e3625 = phi_27706_;
                                                                    let _e3630 = pow(((1f - _e2393) / (1f + _e2393)), 2f);
                                                                    phi_27695_ = (_e3630 + ((1f - _e3630) * pow((1f - select(_e3613, _e3621, _e3625)), 5f)));
                                                                }
                                                                let _e3637 = phi_27695_;
                                                                if (_e3182 <= 0f) {
                                                                    phi_27745_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    phi_27745_ = (_e2391 * (_e3182 * 0.31830987f));
                                                                }
                                                                let _e3642 = phi_27745_;
                                                                let _e3643 = (1f - _e3637);
                                                                phi_15111_ = type_57();
                                                                phi_15112_ = type_57((_e3605.member + vec3<f32>((_e3642.x * _e3643), (_e3642.y * _e3643), (_e3642.z * _e3643))), ((_e3637 * _e3605.member_1) + (_e3643 * _e3188)));
                                                            }
                                                            let _e3657 = phi_15111_;
                                                            let _e3659 = phi_15112_;
                                                            phi_15113_ = _e3657;
                                                            phi_15114_ = _e3659;
                                                            phi_15115_ = _e3608;
                                                        }
                                                        let _e3667 = phi_15113_;
                                                        let _e3669 = phi_15114_;
                                                        let _e3671 = phi_15115_;
                                                        if _e3671 {
                                                            phi_15119_ = _e3667;
                                                        } else {
                                                            phi_15119_ = _e3669;
                                                        }
                                                        let _e3673 = phi_15119_;
                                                        if ((_e3673.member_1 * _e3177) < 0.001f) {
                                                            let _e4100 = global_1.member.member_6;
                                                            let _e4106 = global_1.member.member_8[2u];
                                                            if (_e4106 == 4294967295u) {
                                                                phi_15858_ = vec3<f32>(1f, 1f, 1f);
                                                            } else {
                                                                let _e4109 = global_6.member[_e4106];
                                                                let _e4112 = global_6.member[(_e4106 + 1u)];
                                                                let _e4115 = global_6.member[(_e4106 + 2u)];
                                                                let _e4118 = global_6.member[(_e4106 + 3u)];
                                                                let _e4121 = global_6.member[(_e4106 + 4u)];
                                                                let _e4124 = global_6.member[(_e4106 + 5u)];
                                                                let _e4127 = global_6.member[(_e4106 + 6u)];
                                                                let _e4130 = global_6.member[(_e4106 + 7u)];
                                                                let _e4133 = global_6.member[(_e4106 + 8u)];
                                                                let _e4136 = global_6.member[(_e4106 + 9u)];
                                                                let _e4139 = global_6.member[(_e4106 + 13u)];
                                                                let _e4142 = global_6.member[(_e4106 + 14u)];
                                                                let _e4145 = global_6.member[(_e4106 + 15u)];
                                                                let _e4148 = global_6.member[(_e4106 + 16u)];
                                                                let _e4151 = global_6.member[(_e4106 + 18u)];
                                                                let _e4154 = global_6.member[(_e4106 + 19u)];
                                                                let _e4157 = global_6.member[(_e4106 + 20u)];
                                                                let _e4160 = global_6.member[(_e4106 + 29u)];
                                                                let _e4161 = bitcast<u32>(_e4160);
                                                                let _e4174 = (((((_e1536 * _e1536) + (_e1537 * _e1537)) + (_e1538 * _e1538)) - (2f * (((_e1536 * _e4109) + (_e1537 * _e4112)) + (_e1538 * _e4115)))) + _e4127);
                                                                if (_e4174 > 0f) {
                                                                    let _e4183 = ((((_e1536 - _e4109) * _e4130) + ((_e1537 - _e4112) * _e4133)) + ((_e1538 - _e4115) * _e4136));
                                                                    let _e4185 = ((_e4183 * _e4183) - _e4174);
                                                                    if (_e4183 >= 0f) {
                                                                        phi_15558_ = vec3<f32>();
                                                                        phi_15559_ = true;
                                                                    } else {
                                                                        let _e4187 = (_e4185 <= 0f);
                                                                        if _e4187 {
                                                                            phi_15557_ = vec3<f32>();
                                                                        } else {
                                                                            let _e4190 = (-(_e4183) - sqrt(_e4185));
                                                                            phi_15557_ = vec3<f32>((_e1536 + (_e4130 * _e4190)), (_e1537 + (_e4133 * _e4190)), (_e1538 + (_e4136 * _e4190)));
                                                                        }
                                                                        let _e4199 = phi_15557_;
                                                                        phi_15558_ = _e4199;
                                                                        phi_15559_ = _e4187;
                                                                    }
                                                                    let _e4201 = phi_15558_;
                                                                    let _e4203 = phi_15559_;
                                                                    phi_15564_ = _e4201;
                                                                    phi_15565_ = select(true, false, _e4203);
                                                                } else {
                                                                    phi_15564_ = _e1539;
                                                                    phi_15565_ = true;
                                                                }
                                                                let _e4206 = phi_15564_;
                                                                let _e4208 = phi_15565_;
                                                                if _e4208 {
                                                                    let _e4210 = (_e4206.x - _e4109);
                                                                    let _e4212 = (_e4206.y - _e4112);
                                                                    let _e4214 = (_e4206.z - _e4115);
                                                                    let _e4220 = sqrt((((_e4210 * _e4210) + (_e4212 * _e4212)) + (_e4214 * _e4214)));
                                                                    let _e4221 = (1f / _e4220);
                                                                    let _e4239 = ((((((_e4206.x * _e4206.x) + (_e4206.y * _e4206.y)) + (_e4206.z * _e4206.z)) - (2f * (((_e4206.x * _e4109) + (_e4206.y * _e4112)) + (_e4206.z * _e4115)))) + _e4124) / (_e4220 + _e4118));
                                                                    if (_e4239 != _e4239) {
                                                                        phi_28201_ = true;
                                                                    } else {
                                                                        phi_28201_ = (0f >= _e4239);
                                                                    }
                                                                    let _e4243 = phi_28201_;
                                                                    let _e4247 = sqrt((select(_e4239, 0f, _e4243) / (_e4121 - _e4118)));
                                                                    if (_e4247 != _e4247) {
                                                                        phi_28216_ = true;
                                                                    } else {
                                                                        phi_28216_ = (1f <= _e4247);
                                                                    }
                                                                    let _e4251 = phi_28216_;
                                                                    let _e4253 = (select(_e4247, 1f, _e4251) * 63f);
                                                                    let _e4258 = ((((_e4210 * _e4221) * _e4130) + ((_e4212 * _e4221) * _e4133)) + ((_e4214 * _e4221) * _e4136));
                                                                    let _e4260 = select(_e4258, -1f, (_e4258 < -1f));
                                                                    let _e4264 = ((select(_e4260, 1f, (_e4260 > 1f)) + 1f) * 127.5f);
                                                                    let _e4269 = select(select(u32(_e4253), 0u, (_e4253 < 0f)), 4294967295u, (_e4253 > 4294967000f));
                                                                    let _e4271 = select(_e4269, 62u, (62u < _e4269));
                                                                    let _e4276 = select(select(u32(_e4264), 0u, (_e4264 < 0f)), 4294967295u, (_e4264 > 4294967000f));
                                                                    let _e4278 = select(_e4276, 254u, (254u < _e4276));
                                                                    let _e4280 = (_e4253 - f32(_e4271));
                                                                    let _e4282 = (_e4264 - f32(_e4278));
                                                                    let _e4286 = (_e4161 + (3u * ((_e4271 * 256u) + _e4278)));
                                                                    let _e4291 = (_e4161 + (3u * (((_e4271 + 1u) * 256u) + _e4278)));
                                                                    let _e4293 = global_6.member[_e4286];
                                                                    let _e4296 = global_6.member[(_e4286 + 1u)];
                                                                    let _e4299 = global_6.member[(_e4286 + 2u)];
                                                                    let _e4300 = (1f - _e4282);
                                                                    let _e4306 = global_6.member[(_e4286 + 3u)];
                                                                    let _e4309 = global_6.member[(_e4286 + 4u)];
                                                                    let _e4312 = global_6.member[(_e4286 + 5u)];
                                                                    let _e4319 = (1f - _e4280);
                                                                    let _e4324 = global_6.member[_e4291];
                                                                    let _e4327 = global_6.member[(_e4291 + 1u)];
                                                                    let _e4330 = global_6.member[(_e4291 + 2u)];
                                                                    let _e4336 = global_6.member[(_e4291 + 3u)];
                                                                    let _e4339 = global_6.member[(_e4291 + 4u)];
                                                                    let _e4342 = global_6.member[(_e4291 + 5u)];
                                                                    phi_15856_ = vec3<f32>(((((_e4293 * _e4300) + (_e4306 * _e4282)) * _e4319) + (((_e4324 * _e4300) + (_e4336 * _e4282)) * _e4280)), ((((_e4296 * _e4300) + (_e4309 * _e4282)) * _e4319) + (((_e4327 * _e4300) + (_e4339 * _e4282)) * _e4280)), ((((_e4299 * _e4300) + (_e4312 * _e4282)) * _e4319) + (((_e4330 * _e4300) + (_e4342 * _e4282)) * _e4280)));
                                                                } else {
                                                                    phi_15856_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e4357 = phi_15856_;
                                                                let _e4364 = ((_e4148 * 1.1f) * _e4357.y);
                                                                phi_15858_ = vec3<f32>(exp(-((((_e4139 * _e4357.x) + _e4364) + (_e4151 * _e4357.z)))), exp(-((((_e4142 * _e4357.x) + _e4364) + (_e4154 * _e4357.z)))), exp(-((((_e4145 * _e4357.x) + _e4364) + (_e4157 * _e4357.z)))));
                                                            }
                                                            let _e4383 = phi_15858_;
                                                            phi_34726_ = _e2418;
                                                            phi_16742_ = 0f;
                                                            phi_16743_ = vec3<f32>(_e3165.x, _e3165.y, _e3165.z);
                                                            phi_16744_ = (vec3<f32>(_e4100.x, _e4100.y, _e4100.z) * _e4383);
                                                        } else {
                                                            let _e3678 = ((_e2418 * 747796405u) + 2891336453u);
                                                            let _e3686 = (((_e3678 >> bitcast<u32>((((_e3678 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3678) * 277803737u);
                                                            let _e3689 = ((_e3686 >> bitcast<u32>(22i)) ^ _e3686);
                                                            let _e3695 = (((f32((_e3689 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e3177) * 0.15915494f);
                                                            let _e3696 = (1f - _e3695);
                                                            let _e3698 = (_e3695 * (2f - _e3695));
                                                            if (_e3698 != _e3698) {
                                                                phi_27799_ = true;
                                                            } else {
                                                                phi_27799_ = (0f >= _e3698);
                                                            }
                                                            let _e3702 = phi_27799_;
                                                            let _e3704 = sqrt(select(_e3698, 0f, _e3702));
                                                            let _e3706 = ((_e3689 * 747796405u) + 2891336453u);
                                                            let _e3714 = (((_e3706 >> bitcast<u32>((((_e3706 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3706) * 277803737u);
                                                            let _e3717 = ((_e3714 >> bitcast<u32>(22i)) ^ _e3714);
                                                            let _e3721 = (f32((_e3717 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                            let _e3723 = select(-1f, 1f, (_e3165.z >= 0f));
                                                            let _e3725 = (-1f / (_e3723 + _e3165.z));
                                                            let _e3727 = ((_e3165.x * _e3165.y) * _e3725);
                                                            let _e3740 = (_e3704 * cos(_e3721));
                                                            let _e3745 = (_e3704 * sin(_e3721));
                                                            let _e3755 = ((((1f + (((_e3723 * _e3165.x) * _e3165.x) * _e3725)) * _e3740) + (_e3727 * _e3745)) + (_e3165.x * _e3696));
                                                            let _e3756 = ((((_e3723 * _e3727) * _e3740) + ((_e3723 + ((_e3165.y * _e3165.y) * _e3725)) * _e3745)) + (_e3165.y * _e3696));
                                                            let _e3757 = ((((-(_e3723) * _e3165.x) * _e3740) + (-(_e3165.y) * _e3745)) + (_e3165.z * _e3696));
                                                            let _e3759 = (_e3755 - _e3165.x);
                                                            let _e3760 = (_e3756 - _e3165.y);
                                                            let _e3761 = (_e3757 - _e3165.z);
                                                            let _e3768 = (sqrt((((_e3759 * _e3759) + (_e3760 * _e3760)) + (_e3761 * _e3761))) * 0.5f);
                                                            let _e3769 = (_e3768 != _e3768);
                                                            if _e3769 {
                                                                phi_27870_ = true;
                                                            } else {
                                                                phi_27870_ = (1f <= _e3768);
                                                            }
                                                            let _e3772 = phi_27870_;
                                                            if ((2f * asin(select(_e3768, 1f, _e3772))) >= _e3173) {
                                                                phi_15939_ = 0f;
                                                            } else {
                                                                phi_15939_ = (1f / _e3177);
                                                            }
                                                            let _e3779 = phi_15939_;
                                                            if (_e3779 > 0f) {
                                                                if _e3769 {
                                                                    phi_27885_ = true;
                                                                } else {
                                                                    phi_27885_ = (1f <= _e3768);
                                                                }
                                                                let _e3783 = phi_27885_;
                                                                let _e3786 = (2f * asin(select(_e3768, 1f, _e3783)));
                                                                if (_e3786 >= _e3173) {
                                                                    phi_16740_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    let _e3790 = (1f - pow((_e3786 / _e3173), 2f));
                                                                    if (_e3790 != _e3790) {
                                                                        phi_27900_ = true;
                                                                    } else {
                                                                        phi_27900_ = (0f >= _e3790);
                                                                    }
                                                                    let _e3794 = phi_27900_;
                                                                    let _e3797 = global_1.member.member_6;
                                                                    let _e3803 = global_1.member.member_8[2u];
                                                                    if (_e3803 == 4294967295u) {
                                                                        phi_16713_ = vec3<f32>(1f, 1f, 1f);
                                                                    } else {
                                                                        let _e3806 = global_6.member[_e3803];
                                                                        let _e3809 = global_6.member[(_e3803 + 1u)];
                                                                        let _e3812 = global_6.member[(_e3803 + 2u)];
                                                                        let _e3815 = global_6.member[(_e3803 + 3u)];
                                                                        let _e3818 = global_6.member[(_e3803 + 4u)];
                                                                        let _e3821 = global_6.member[(_e3803 + 5u)];
                                                                        let _e3824 = global_6.member[(_e3803 + 6u)];
                                                                        let _e3827 = global_6.member[(_e3803 + 7u)];
                                                                        let _e3830 = global_6.member[(_e3803 + 8u)];
                                                                        let _e3833 = global_6.member[(_e3803 + 9u)];
                                                                        let _e3836 = global_6.member[(_e3803 + 13u)];
                                                                        let _e3839 = global_6.member[(_e3803 + 14u)];
                                                                        let _e3842 = global_6.member[(_e3803 + 15u)];
                                                                        let _e3845 = global_6.member[(_e3803 + 16u)];
                                                                        let _e3848 = global_6.member[(_e3803 + 18u)];
                                                                        let _e3851 = global_6.member[(_e3803 + 19u)];
                                                                        let _e3854 = global_6.member[(_e3803 + 20u)];
                                                                        let _e3857 = global_6.member[(_e3803 + 29u)];
                                                                        let _e3858 = bitcast<u32>(_e3857);
                                                                        let _e3871 = (((((_e1536 * _e1536) + (_e1537 * _e1537)) + (_e1538 * _e1538)) - (2f * (((_e1536 * _e3806) + (_e1537 * _e3809)) + (_e1538 * _e3812)))) + _e3824);
                                                                        if (_e3871 > 0f) {
                                                                            let _e3880 = ((((_e1536 - _e3806) * _e3827) + ((_e1537 - _e3809) * _e3830)) + ((_e1538 - _e3812) * _e3833));
                                                                            let _e3882 = ((_e3880 * _e3880) - _e3871);
                                                                            if (_e3880 >= 0f) {
                                                                                phi_16413_ = vec3<f32>();
                                                                                phi_16414_ = true;
                                                                            } else {
                                                                                let _e3884 = (_e3882 <= 0f);
                                                                                if _e3884 {
                                                                                    phi_16412_ = vec3<f32>();
                                                                                } else {
                                                                                    let _e3887 = (-(_e3880) - sqrt(_e3882));
                                                                                    phi_16412_ = vec3<f32>((_e1536 + (_e3827 * _e3887)), (_e1537 + (_e3830 * _e3887)), (_e1538 + (_e3833 * _e3887)));
                                                                                }
                                                                                let _e3896 = phi_16412_;
                                                                                phi_16413_ = _e3896;
                                                                                phi_16414_ = _e3884;
                                                                            }
                                                                            let _e3898 = phi_16413_;
                                                                            let _e3900 = phi_16414_;
                                                                            phi_16419_ = _e3898;
                                                                            phi_16420_ = select(true, false, _e3900);
                                                                        } else {
                                                                            phi_16419_ = _e1539;
                                                                            phi_16420_ = true;
                                                                        }
                                                                        let _e3903 = phi_16419_;
                                                                        let _e3905 = phi_16420_;
                                                                        if _e3905 {
                                                                            let _e3907 = (_e3903.x - _e3806);
                                                                            let _e3909 = (_e3903.y - _e3809);
                                                                            let _e3911 = (_e3903.z - _e3812);
                                                                            let _e3917 = sqrt((((_e3907 * _e3907) + (_e3909 * _e3909)) + (_e3911 * _e3911)));
                                                                            let _e3918 = (1f / _e3917);
                                                                            let _e3936 = ((((((_e3903.x * _e3903.x) + (_e3903.y * _e3903.y)) + (_e3903.z * _e3903.z)) - (2f * (((_e3903.x * _e3806) + (_e3903.y * _e3809)) + (_e3903.z * _e3812)))) + _e3821) / (_e3917 + _e3815));
                                                                            if (_e3936 != _e3936) {
                                                                                phi_28024_ = true;
                                                                            } else {
                                                                                phi_28024_ = (0f >= _e3936);
                                                                            }
                                                                            let _e3940 = phi_28024_;
                                                                            let _e3944 = sqrt((select(_e3936, 0f, _e3940) / (_e3818 - _e3815)));
                                                                            if (_e3944 != _e3944) {
                                                                                phi_28039_ = true;
                                                                            } else {
                                                                                phi_28039_ = (1f <= _e3944);
                                                                            }
                                                                            let _e3948 = phi_28039_;
                                                                            let _e3950 = (select(_e3944, 1f, _e3948) * 63f);
                                                                            let _e3955 = ((((_e3907 * _e3918) * _e3827) + ((_e3909 * _e3918) * _e3830)) + ((_e3911 * _e3918) * _e3833));
                                                                            let _e3957 = select(_e3955, -1f, (_e3955 < -1f));
                                                                            let _e3961 = ((select(_e3957, 1f, (_e3957 > 1f)) + 1f) * 127.5f);
                                                                            let _e3966 = select(select(u32(_e3950), 0u, (_e3950 < 0f)), 4294967295u, (_e3950 > 4294967000f));
                                                                            let _e3968 = select(_e3966, 62u, (62u < _e3966));
                                                                            let _e3973 = select(select(u32(_e3961), 0u, (_e3961 < 0f)), 4294967295u, (_e3961 > 4294967000f));
                                                                            let _e3975 = select(_e3973, 254u, (254u < _e3973));
                                                                            let _e3977 = (_e3950 - f32(_e3968));
                                                                            let _e3979 = (_e3961 - f32(_e3975));
                                                                            let _e3983 = (_e3858 + (3u * ((_e3968 * 256u) + _e3975)));
                                                                            let _e3988 = (_e3858 + (3u * (((_e3968 + 1u) * 256u) + _e3975)));
                                                                            let _e3990 = global_6.member[_e3983];
                                                                            let _e3993 = global_6.member[(_e3983 + 1u)];
                                                                            let _e3996 = global_6.member[(_e3983 + 2u)];
                                                                            let _e3997 = (1f - _e3979);
                                                                            let _e4003 = global_6.member[(_e3983 + 3u)];
                                                                            let _e4006 = global_6.member[(_e3983 + 4u)];
                                                                            let _e4009 = global_6.member[(_e3983 + 5u)];
                                                                            let _e4016 = (1f - _e3977);
                                                                            let _e4021 = global_6.member[_e3988];
                                                                            let _e4024 = global_6.member[(_e3988 + 1u)];
                                                                            let _e4027 = global_6.member[(_e3988 + 2u)];
                                                                            let _e4033 = global_6.member[(_e3988 + 3u)];
                                                                            let _e4036 = global_6.member[(_e3988 + 4u)];
                                                                            let _e4039 = global_6.member[(_e3988 + 5u)];
                                                                            phi_16711_ = vec3<f32>(((((_e3990 * _e3997) + (_e4003 * _e3979)) * _e4016) + (((_e4021 * _e3997) + (_e4033 * _e3979)) * _e3977)), ((((_e3993 * _e3997) + (_e4006 * _e3979)) * _e4016) + (((_e4024 * _e3997) + (_e4036 * _e3979)) * _e3977)), ((((_e3996 * _e3997) + (_e4009 * _e3979)) * _e4016) + (((_e4027 * _e3997) + (_e4039 * _e3979)) * _e3977)));
                                                                        } else {
                                                                            phi_16711_ = vec3<f32>(0f, 0f, 0f);
                                                                        }
                                                                        let _e4054 = phi_16711_;
                                                                        let _e4061 = ((_e3845 * 1.1f) * _e4054.y);
                                                                        phi_16713_ = vec3<f32>(exp(-((((_e3836 * _e4054.x) + _e4061) + (_e3848 * _e4054.z)))), exp(-((((_e3839 * _e4054.x) + _e4061) + (_e3851 * _e4054.z)))), exp(-((((_e3842 * _e4054.x) + _e4061) + (_e3854 * _e4054.z)))));
                                                                    }
                                                                    let _e4080 = phi_16713_;
                                                                    let _e4091 = (((1f - (0.6f * (1f - sqrt(select(_e3790, 0f, _e3794))))) * 1.25f) / _e3177);
                                                                    phi_16740_ = vec3<f32>(((_e3797.x * _e4080.x) * _e4091), ((_e3797.y * _e4080.y) * _e4091), ((_e3797.z * _e4080.z) * _e4091));
                                                                }
                                                                let _e4097 = phi_16740_;
                                                                phi_16741_ = _e4097;
                                                            } else {
                                                                phi_16741_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e4099 = phi_16741_;
                                                            phi_34726_ = _e3717;
                                                            phi_16742_ = _e3779;
                                                            phi_16743_ = vec3<f32>(_e3755, _e3756, _e3757);
                                                            phi_16744_ = _e4099;
                                                        }
                                                        let _e4387 = phi_34726_;
                                                        let _e4389 = phi_16742_;
                                                        let _e4391 = phi_16743_;
                                                        let _e4393 = phi_16744_;
                                                        phi_34725_ = _e4387;
                                                        phi_16745_ = _e4389;
                                                        phi_16746_ = _e4391;
                                                        phi_16747_ = _e4393;
                                                    }
                                                    let _e4395 = phi_34725_;
                                                    let _e4397 = phi_16745_;
                                                    let _e4399 = phi_16746_;
                                                    let _e4401 = phi_16747_;
                                                    phi_34682_ = _e4395;
                                                    phi_17637_ = _e4397;
                                                    phi_17638_ = _e4399;
                                                    phi_17639_ = _e4401;
                                                    phi_17640_ = 1000000000000000000000000000000f;
                                                } else {
                                                    if (_e2424 == (_e2414 + 1u)) {
                                                        if ((_e366 & 1u) == 0u) {
                                                            phi_34691_ = _e2418;
                                                            phi_17271_ = 0f;
                                                            phi_17272_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17273_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17274_ = 1000000000000000000000000000000f;
                                                        } else {
                                                            if (_e369 > 0u) {
                                                                let _e2655 = global_1.member.member_2[1u];
                                                                let _e2657 = ((_e2418 * 747796405u) + 2891336453u);
                                                                let _e2665 = (((_e2657 >> bitcast<u32>((((_e2657 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2657) * 277803737u);
                                                                let _e2668 = ((_e2665 >> bitcast<u32>(22i)) ^ _e2665);
                                                                phi_16768_ = 0u;
                                                                phi_16771_ = _e369;
                                                                loop {
                                                                    let _e2674 = phi_16768_;
                                                                    let _e2676 = phi_16771_;
                                                                    local_83 = _e2674;
                                                                    local_84 = _e2674;
                                                                    let _e2678 = ((_e2674 + 1u) < _e2676);
                                                                    if _e2678 {
                                                                        let _e2680 = ((_e2674 + _e2676) / 2u);
                                                                        let _e2683 = global_6.member[(_e2655 + _e2680)];
                                                                        let _e2684 = (_e2683 <= (f32((_e2668 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                        phi_16769_ = select(_e2674, _e2680, _e2684);
                                                                        phi_16772_ = select(_e2680, _e2676, _e2684);
                                                                    } else {
                                                                        phi_16769_ = u32();
                                                                        phi_16772_ = u32();
                                                                    }
                                                                    let _e2688 = phi_16769_;
                                                                    let _e2690 = phi_16772_;
                                                                    phi_16768_ = _e2688;
                                                                    phi_16771_ = _e2690;
                                                                    if !(_e2678) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e2693 = local_83;
                                                                let _e2694 = (_e2655 + _e2693);
                                                                let _e2697 = global_6.member[(_e2694 + 1u)];
                                                                let _e2699 = global_6.member[_e2694];
                                                                let _e2703 = global_1.member.member_2[0u];
                                                                let _e2705 = local_84;
                                                                let _e2708 = global_6.member[(_e2703 + _e2705)];
                                                                let _e2711 = ((_e2668 * 747796405u) + 2891336453u);
                                                                let _e2719 = (((_e2711 >> bitcast<u32>((((_e2711 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2711) * 277803737u);
                                                                let _e2722 = ((_e2719 >> bitcast<u32>(22i)) ^ _e2719);
                                                                let _e2725 = f32((_e2722 >> bitcast<u32>(8i)));
                                                                let _e2726 = (_e2725 * 0.000000059604645f);
                                                                let _e2728 = ((_e2722 * 747796405u) + 2891336453u);
                                                                let _e2736 = (((_e2728 >> bitcast<u32>((((_e2728 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2728) * 277803737u);
                                                                let _e2739 = ((_e2736 >> bitcast<u32>(22i)) ^ _e2736);
                                                                let _e2742 = f32((_e2739 >> bitcast<u32>(8i)));
                                                                let _e2745 = global_2.member[bitcast<u32>(_e2708)];
                                                                let _e2748 = (_e2745.member_6.x == 1u);
                                                                if _e2748 {
                                                                    let _e2890 = (_e2745.member.x - _e1536);
                                                                    let _e2891 = (_e2745.member.y - _e1537);
                                                                    let _e2892 = (_e2745.member.z - _e1538);
                                                                    let _e2897 = (((_e2890 * _e2890) + (_e2891 * _e2891)) + (_e2892 * _e2892));
                                                                    let _e2898 = (_e2745.member.w * _e2745.member.w);
                                                                    let _e2900 = (_e2897 <= (_e2898 * 1.000001f));
                                                                    if _e2900 {
                                                                        let _e3009 = (_e2725 * 0.00000037450704f);
                                                                        let _e3011 = ((_e2742 * 0.00000011920929f) - 1f);
                                                                        let _e3013 = (1f - (_e3011 * _e3011));
                                                                        if (_e3013 != _e3013) {
                                                                            phi_26867_ = true;
                                                                        } else {
                                                                            phi_26867_ = (0f >= _e3013);
                                                                        }
                                                                        let _e3017 = phi_26867_;
                                                                        let _e3019 = sqrt(select(_e3013, 0f, _e3017));
                                                                        let _e3021 = (_e3019 * cos(_e3009));
                                                                        let _e3023 = (_e3019 * sin(_e3009));
                                                                        let _e3028 = (_e2745.member.x + (_e3021 * _e2745.member.w));
                                                                        let _e3029 = (_e2745.member.y + (_e3023 * _e2745.member.w));
                                                                        let _e3030 = (_e2745.member.z + (_e3011 * _e2745.member.w));
                                                                        if _e2900 {
                                                                            let _e3043 = (_e3028 - _e1536);
                                                                            let _e3044 = (_e3029 - _e1537);
                                                                            let _e3045 = (_e3030 - _e1538);
                                                                            let _e3050 = (((_e3043 * _e3043) + (_e3044 * _e3044)) + (_e3045 * _e3045));
                                                                            phi_26936_ = (_e3050 / (((abs((((_e3021 * _e3043) + (_e3023 * _e3044)) + (_e3011 * _e3045))) / sqrt(_e3050)) * 12.566371f) * _e2898));
                                                                        } else {
                                                                            let _e3032 = (_e2898 / _e2897);
                                                                            if (_e3032 != _e3032) {
                                                                                phi_26955_ = true;
                                                                            } else {
                                                                                phi_26955_ = (1f <= _e3032);
                                                                            }
                                                                            let _e3036 = phi_26955_;
                                                                            let _e3037 = select(_e3032, 1f, _e3036);
                                                                            phi_26936_ = (0.15915494f / (_e3037 / (1f + sqrt((1f - _e3037)))));
                                                                        }
                                                                        let _e3063 = phi_26936_;
                                                                        phi_16984_ = type_72(vec3<f32>(_e3028, _e3029, _e3030), vec3<f32>(_e3021, _e3023, _e3011), _e3063);
                                                                    } else {
                                                                        let _e2901 = (_e2898 / _e2897);
                                                                        let _e2902 = (_e2901 != _e2901);
                                                                        if _e2902 {
                                                                            phi_26719_ = true;
                                                                        } else {
                                                                            phi_26719_ = (1f <= _e2901);
                                                                        }
                                                                        let _e2905 = phi_26719_;
                                                                        let _e2906 = select(_e2901, 1f, _e2905);
                                                                        if _e2902 {
                                                                            phi_26742_ = true;
                                                                        } else {
                                                                            phi_26742_ = (1f <= _e2901);
                                                                        }
                                                                        let _e2910 = phi_26742_;
                                                                        let _e2911 = select(_e2901, 1f, _e2910);
                                                                        let _e2915 = (_e2911 / (1f + sqrt((1f - _e2911))));
                                                                        if (_e2906 < 0.00068523f) {
                                                                            phi_16919_ = (_e2906 * _e2726);
                                                                        } else {
                                                                            let _e2918 = (1f - (_e2726 * _e2915));
                                                                            let _e2920 = (1f - (_e2918 * _e2918));
                                                                            if (_e2920 != _e2920) {
                                                                                phi_26757_ = true;
                                                                            } else {
                                                                                phi_26757_ = (0f >= _e2920);
                                                                            }
                                                                            let _e2924 = phi_26757_;
                                                                            phi_16919_ = select(_e2920, 0f, _e2924);
                                                                        }
                                                                        let _e2928 = phi_16919_;
                                                                        let _e2929 = (1f - _e2928);
                                                                        if (_e2929 != _e2929) {
                                                                            phi_26772_ = true;
                                                                        } else {
                                                                            phi_26772_ = (0f >= _e2929);
                                                                        }
                                                                        let _e2933 = phi_26772_;
                                                                        let _e2938 = (1f - (_e2928 / _e2906));
                                                                        if (_e2938 != _e2938) {
                                                                            phi_26787_ = true;
                                                                        } else {
                                                                            phi_26787_ = (0f >= _e2938);
                                                                        }
                                                                        let _e2942 = phi_26787_;
                                                                        let _e2946 = ((_e2928 / sqrt(_e2906)) + (sqrt(select(_e2929, 0f, _e2933)) * sqrt(select(_e2938, 0f, _e2942))));
                                                                        let _e2948 = (1f - (_e2946 * _e2946));
                                                                        if (_e2948 != _e2948) {
                                                                            phi_26802_ = true;
                                                                        } else {
                                                                            phi_26802_ = (0f >= _e2948);
                                                                        }
                                                                        let _e2952 = phi_26802_;
                                                                        let _e2954 = sqrt(select(_e2948, 0f, _e2952));
                                                                        let _e2955 = (_e2742 * 0.00000037450704f);
                                                                        let _e2956 = sqrt(_e2897);
                                                                        let _e2957 = (_e2890 / _e2956);
                                                                        let _e2958 = (_e2891 / _e2956);
                                                                        let _e2959 = (_e2892 / _e2956);
                                                                        let _e2961 = select(-1f, 1f, (_e2959 >= 0f));
                                                                        let _e2963 = (-1f / (_e2961 + _e2959));
                                                                        let _e2965 = ((_e2957 * _e2958) * _e2963);
                                                                        let _e2978 = (_e2954 * cos(_e2955));
                                                                        let _e2983 = (_e2954 * sin(_e2955));
                                                                        let _e2996 = -(((((1f + (((_e2961 * _e2957) * _e2957) * _e2963)) * _e2978) + (_e2965 * _e2983)) + (_e2957 * _e2946)));
                                                                        let _e2997 = -(((((_e2961 * _e2965) * _e2978) + ((_e2961 + ((_e2958 * _e2958) * _e2963)) * _e2983)) + (_e2958 * _e2946)));
                                                                        let _e2998 = -(((((-(_e2961) * _e2957) * _e2978) + (-(_e2958) * _e2983)) + (_e2959 * _e2946)));
                                                                        phi_16984_ = type_72(vec3<f32>((_e2745.member.x + (_e2996 * _e2745.member.w)), (_e2745.member.y + (_e2997 * _e2745.member.w)), (_e2745.member.z + (_e2998 * _e2745.member.w))), vec3<f32>(_e2996, _e2997, _e2998), (0.15915494f / _e2915));
                                                                    }
                                                                    let _e3066 = phi_16984_;
                                                                    phi_17170_ = _e3066;
                                                                } else {
                                                                    let _e2749 = sqrt(_e2726);
                                                                    let _e2750 = (1f - _e2749);
                                                                    let _e2751 = ((_e2742 * 0.000000059604645f) * _e2749);
                                                                    let _e2773 = (_e2749 - _e2751);
                                                                    let _e2777 = (((_e2745.member.x * _e2750) + (_e2745.member_1.x * _e2751)) + (_e2745.member_2.x * _e2773));
                                                                    let _e2778 = (((_e2745.member.y * _e2750) + (_e2745.member_1.y * _e2751)) + (_e2745.member_2.y * _e2773));
                                                                    let _e2779 = (((_e2745.member.z * _e2750) + (_e2745.member_1.z * _e2751)) + (_e2745.member_2.z * _e2773));
                                                                    let _e2781 = (_e2745.member_1.x - _e2745.member.x);
                                                                    let _e2782 = (_e2745.member_1.y - _e2745.member.y);
                                                                    let _e2783 = (_e2745.member_1.z - _e2745.member.z);
                                                                    let _e2784 = (_e2745.member_2.x - _e2745.member.x);
                                                                    let _e2785 = (_e2745.member_2.y - _e2745.member.y);
                                                                    let _e2786 = (_e2745.member_2.z - _e2745.member.z);
                                                                    let _e2789 = ((_e2782 * _e2786) - (_e2785 * _e2783));
                                                                    let _e2792 = ((_e2783 * _e2784) - (_e2786 * _e2781));
                                                                    let _e2795 = ((_e2781 * _e2785) - (_e2784 * _e2782));
                                                                    let _e2801 = sqrt((((_e2789 * _e2789) + (_e2792 * _e2792)) + (_e2795 * _e2795)));
                                                                    let _e2802 = (1f / _e2801);
                                                                    let _e2803 = (_e2789 * _e2802);
                                                                    let _e2804 = (_e2792 * _e2802);
                                                                    let _e2805 = (_e2795 * _e2802);
                                                                    if _e2748 {
                                                                        let _e2838 = (_e2745.member.x - _e1536);
                                                                        let _e2839 = (_e2745.member.y - _e1537);
                                                                        let _e2840 = (_e2745.member.z - _e1538);
                                                                        let _e2845 = (((_e2838 * _e2838) + (_e2839 * _e2839)) + (_e2840 * _e2840));
                                                                        let _e2846 = (_e2745.member.w * _e2745.member.w);
                                                                        if (_e2845 <= (_e2846 * 1.000001f)) {
                                                                            let _e2860 = (_e2777 - _e1536);
                                                                            let _e2861 = (_e2778 - _e1537);
                                                                            let _e2862 = (_e2779 - _e1538);
                                                                            let _e2867 = (((_e2860 * _e2860) + (_e2861 * _e2861)) + (_e2862 * _e2862));
                                                                            phi_26685_ = (_e2867 / (((abs((((_e2803 * _e2860) + (_e2804 * _e2861)) + (_e2805 * _e2862))) / sqrt(_e2867)) * 12.566371f) * _e2846));
                                                                        } else {
                                                                            let _e2849 = (_e2846 / _e2845);
                                                                            if (_e2849 != _e2849) {
                                                                                phi_26704_ = true;
                                                                            } else {
                                                                                phi_26704_ = (1f <= _e2849);
                                                                            }
                                                                            let _e2853 = phi_26704_;
                                                                            let _e2854 = select(_e2849, 1f, _e2853);
                                                                            phi_26685_ = (0.15915494f / (_e2854 / (1f + sqrt((1f - _e2854)))));
                                                                        }
                                                                        let _e2880 = phi_26685_;
                                                                        phi_17168_ = _e2880;
                                                                    } else {
                                                                        let _e2807 = (_e2777 - _e1536);
                                                                        let _e2808 = (_e2778 - _e1537);
                                                                        let _e2809 = (_e2779 - _e1538);
                                                                        let _e2814 = (((_e2807 * _e2807) + (_e2808 * _e2808)) + (_e2809 * _e2809));
                                                                        let _e2822 = (abs((((_e2803 * _e2807) + (_e2804 * _e2808)) + (_e2805 * _e2809))) / sqrt(_e2814));
                                                                        if (_e2822 <= 0f) {
                                                                            phi_17167_ = 0f;
                                                                        } else {
                                                                            if (_e2745.member_6.x == 0u) {
                                                                                phi_17164_ = (_e2801 * 0.5f);
                                                                            } else {
                                                                                phi_17164_ = ((12.566371f * _e2745.member.w) * _e2745.member.w);
                                                                            }
                                                                            let _e2831 = phi_17164_;
                                                                            phi_17167_ = (_e2814 / (_e2822 * _e2831));
                                                                        }
                                                                        let _e2835 = phi_17167_;
                                                                        phi_17168_ = _e2835;
                                                                    }
                                                                    let _e2882 = phi_17168_;
                                                                    phi_17170_ = type_72(vec3<f32>(_e2777, _e2778, _e2779), vec3<f32>(_e2803, _e2804, _e2805), _e2882);
                                                                }
                                                                let _e3068 = phi_17170_;
                                                                let _e3073 = (_e3068.member.x - _e1536);
                                                                let _e3075 = (_e3068.member.y - _e1537);
                                                                let _e3077 = (_e3068.member.z - _e1538);
                                                                let _e3083 = sqrt((((_e3073 * _e3073) + (_e3075 * _e3075)) + (_e3077 * _e3077)));
                                                                let _e3084 = (_e3073 / _e3083);
                                                                let _e3085 = (_e3075 / _e3083);
                                                                let _e3086 = (_e3077 / _e3083);
                                                                if (_e3068.member_2 > 0f) {
                                                                    let _e3098 = global_4.member[_e2745.member_6.y];
                                                                    if (_e3098.member == 4u) {
                                                                        if (_e3098.member_1 == 1u) {
                                                                            phi_17248_ = true;
                                                                        } else {
                                                                            phi_17248_ = ((((-(_e3084) * _e3068.member_1.x) + (-(_e3085) * _e3068.member_1.y)) + (-(_e3086) * _e3068.member_1.z)) > 0f);
                                                                        }
                                                                        let _e3113 = phi_17248_;
                                                                        if _e3113 {
                                                                            phi_17257_ = vec3<f32>(_e3098.member_4.x, _e3098.member_4.y, _e3098.member_4.z);
                                                                        } else {
                                                                            phi_17257_ = vec3<f32>();
                                                                        }
                                                                        let _e3120 = phi_17257_;
                                                                        phi_17259_ = _e3120;
                                                                        phi_17260_ = select(true, false, _e3113);
                                                                    } else {
                                                                        phi_17259_ = vec3<f32>();
                                                                        phi_17260_ = true;
                                                                    }
                                                                    let _e3123 = phi_17259_;
                                                                    let _e3125 = phi_17260_;
                                                                    phi_17265_ = ((_e2697 - _e2699) * _e3068.member_2);
                                                                    phi_17266_ = select(_e3123, vec3<f32>(0f, 0f, 0f), vec3(_e3125));
                                                                } else {
                                                                    phi_17265_ = 0f;
                                                                    phi_17266_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e3129 = phi_17265_;
                                                                let _e3131 = phi_17266_;
                                                                phi_34692_ = _e2739;
                                                                phi_17267_ = _e3129;
                                                                phi_17268_ = vec3<f32>(_e3084, _e3085, _e3086);
                                                                phi_17269_ = _e3131;
                                                                phi_17270_ = ((_e3083 * 0.9999f) - _e1529);
                                                            } else {
                                                                phi_34692_ = _e2418;
                                                                phi_17267_ = 0f;
                                                                phi_17268_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_17269_ = vec3<f32>(0f, 0f, 0f);
                                                                phi_17270_ = 1000000000000000000000000000000f;
                                                            }
                                                            let _e3133 = phi_34692_;
                                                            let _e3135 = phi_17267_;
                                                            let _e3137 = phi_17268_;
                                                            let _e3139 = phi_17269_;
                                                            let _e3141 = phi_17270_;
                                                            phi_34691_ = _e3133;
                                                            phi_17271_ = _e3135;
                                                            phi_17272_ = _e3137;
                                                            phi_17273_ = _e3139;
                                                            phi_17274_ = _e3141;
                                                        }
                                                        let _e3143 = phi_34691_;
                                                        let _e3145 = phi_17271_;
                                                        let _e3147 = phi_17272_;
                                                        let _e3149 = phi_17273_;
                                                        let _e3151 = phi_17274_;
                                                        phi_34683_ = _e3143;
                                                        phi_17633_ = _e3145;
                                                        phi_17634_ = _e3147;
                                                        phi_17635_ = _e3149;
                                                        phi_17636_ = _e3151;
                                                    } else {
                                                        if ((_e366 & 2u) == 0u) {
                                                            phi_34684_ = _e2418;
                                                            phi_17630_ = 0f;
                                                            phi_17631_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17632_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e2436 = ((_e2418 * 747796405u) + 2891336453u);
                                                            let _e2444 = (((_e2436 >> bitcast<u32>((((_e2436 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2436) * 277803737u);
                                                            let _e2447 = ((_e2444 >> bitcast<u32>(22i)) ^ _e2444);
                                                            let _e2451 = (f32((_e2447 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2453 = ((_e2447 * 747796405u) + 2891336453u);
                                                            let _e2461 = (((_e2453 >> bitcast<u32>((((_e2453 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2453) * 277803737u);
                                                            let _e2464 = ((_e2461 >> bitcast<u32>(22i)) ^ _e2461);
                                                            let _e2468 = (f32((_e2464 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                            let _e2471 = global_1.member.member_3[1u];
                                                            let _e2474 = global_1.member.member_3[2u];
                                                            let _e2477 = global_1.member.member_2[3u];
                                                            phi_17295_ = 0u;
                                                            phi_17298_ = _e2474;
                                                            loop {
                                                                let _e2479 = phi_17295_;
                                                                let _e2481 = phi_17298_;
                                                                local_78 = _e2479;
                                                                local_81 = _e2479;
                                                                local_82 = _e2479;
                                                                let _e2483 = ((_e2479 + 1u) < _e2481);
                                                                if _e2483 {
                                                                    let _e2485 = ((_e2479 + _e2481) / 2u);
                                                                    let _e2488 = global_6.member[(_e2477 + _e2485)];
                                                                    let _e2489 = (_e2488 <= _e2451);
                                                                    phi_17296_ = select(_e2479, _e2485, _e2489);
                                                                    phi_17299_ = select(_e2485, _e2481, _e2489);
                                                                } else {
                                                                    phi_17296_ = u32();
                                                                    phi_17299_ = u32();
                                                                }
                                                                let _e2493 = phi_17296_;
                                                                let _e2495 = phi_17299_;
                                                                phi_17295_ = _e2493;
                                                                phi_17298_ = _e2495;
                                                                if !(_e2483) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2499 = global_1.member.member_3[0u];
                                                            let _e2500 = (_e2471 + 1u);
                                                            let _e2502 = local_78;
                                                            let _e2504 = (_e2499 + (_e2502 * _e2500));
                                                            phi_17336_ = 0u;
                                                            phi_17339_ = _e2471;
                                                            loop {
                                                                let _e2506 = phi_17336_;
                                                                let _e2508 = phi_17339_;
                                                                local_79 = _e2506;
                                                                local_80 = _e2506;
                                                                let _e2510 = ((_e2506 + 1u) < _e2508);
                                                                if _e2510 {
                                                                    let _e2512 = ((_e2506 + _e2508) / 2u);
                                                                    let _e2515 = global_6.member[(_e2504 + _e2512)];
                                                                    let _e2516 = (_e2515 <= _e2468);
                                                                    phi_17337_ = select(_e2506, _e2512, _e2516);
                                                                    phi_17340_ = select(_e2512, _e2508, _e2516);
                                                                } else {
                                                                    phi_17337_ = u32();
                                                                    phi_17340_ = u32();
                                                                }
                                                                let _e2520 = phi_17337_;
                                                                let _e2522 = phi_17340_;
                                                                phi_17336_ = _e2520;
                                                                phi_17339_ = _e2522;
                                                                if !(_e2510) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2525 = local_79;
                                                            let _e2528 = local_80;
                                                            let _e2529 = (_e2504 + _e2528);
                                                            let _e2531 = global_6.member[_e2529];
                                                            let _e2534 = global_6.member[(_e2529 + 1u)];
                                                            if (_e2534 > _e2531) {
                                                                let _e2538 = ((_e2468 - _e2531) / (_e2534 - _e2531));
                                                                let _e2540 = select(_e2538, 0f, (_e2538 < 0f));
                                                                phi_17416_ = select(_e2540, 0.99999f, (_e2540 > 0.99999f));
                                                            } else {
                                                                phi_17416_ = 0.5f;
                                                            }
                                                            let _e2544 = phi_17416_;
                                                            let _e2546 = f32(_e2471);
                                                            let _e2549 = local_81;
                                                            let _e2552 = local_82;
                                                            let _e2553 = (_e2477 + _e2552);
                                                            let _e2555 = global_6.member[_e2553];
                                                            let _e2558 = global_6.member[(_e2553 + 1u)];
                                                            if (_e2558 > _e2555) {
                                                                let _e2562 = ((_e2451 - _e2555) / (_e2558 - _e2555));
                                                                let _e2564 = select(_e2562, 0f, (_e2562 < 0f));
                                                                phi_17465_ = select(_e2564, 0.99999f, (_e2564 > 0.99999f));
                                                            } else {
                                                                phi_17465_ = 0.5f;
                                                            }
                                                            let _e2568 = phi_17465_;
                                                            let _e2570 = f32(_e2474);
                                                            let _e2573 = ((((f32(_e2525) + _e2544) / _e2546) - 0.5f) * 6.2831855f);
                                                            let _e2574 = (((f32(_e2549) + _e2568) / _e2570) * 3.1415927f);
                                                            let _e2575 = sin(_e2574);
                                                            let _e2577 = (_e2575 * cos(_e2573));
                                                            let _e2578 = cos(_e2574);
                                                            let _e2580 = (_e2575 * sin(_e2573));
                                                            let _e2583 = ((atan2(_e2580, _e2577) * 0.15915494f) + 0.5f);
                                                            let _e2587 = select(_e2578, -1f, (_e2578 < -1f));
                                                            let _e2590 = acos(select(_e2587, 1f, (_e2587 > 1f)));
                                                            let _e2592 = ((_e2583 - floor(_e2583)) * _e2546);
                                                            let _e2597 = select(select(u32(_e2592), 0u, (_e2592 < 0f)), 4294967295u, (_e2592 > 4294967000f));
                                                            let _e2598 = (_e2471 - 1u);
                                                            let _e2601 = ((_e2590 * 0.31830987f) * _e2570);
                                                            let _e2606 = select(select(u32(_e2601), 0u, (_e2601 < 0f)), 4294967295u, (_e2601 > 4294967000f));
                                                            let _e2607 = (_e2474 - 1u);
                                                            let _e2609 = select(_e2606, _e2607, (_e2607 < _e2606));
                                                            let _e2610 = sin(_e2590);
                                                            if (_e2610 <= 0f) {
                                                                phi_17621_ = 0f;
                                                            } else {
                                                                let _e2614 = (_e2477 + _e2609);
                                                                let _e2617 = global_6.member[(_e2614 + 1u)];
                                                                let _e2619 = global_6.member[_e2614];
                                                                let _e2622 = ((_e2499 + (_e2609 * _e2500)) + select(_e2597, _e2598, (_e2598 < _e2597)));
                                                                let _e2625 = global_6.member[(_e2622 + 1u)];
                                                                let _e2627 = global_6.member[_e2622];
                                                                phi_17621_ = ((((_e2617 - _e2619) * _e2570) * ((_e2625 - _e2627) * _e2546)) / (19.73921f * _e2610));
                                                            }
                                                            let _e2634 = phi_17621_;
                                                            let _e2635 = (_e2634 > 0f);
                                                            if _e2635 {
                                                                phi_17628_ = vec3<f32>(_e2577, _e2578, _e2580);
                                                            } else {
                                                                phi_17628_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2638 = phi_17628_;
                                                            phi_34684_ = _e2464;
                                                            phi_17630_ = select(0f, _e2634, _e2635);
                                                            phi_17631_ = _e2638;
                                                            phi_17632_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2635));
                                                        }
                                                        let _e2643 = phi_34684_;
                                                        let _e2645 = phi_17630_;
                                                        let _e2647 = phi_17631_;
                                                        let _e2649 = phi_17632_;
                                                        phi_34683_ = _e2643;
                                                        phi_17633_ = _e2645;
                                                        phi_17634_ = _e2647;
                                                        phi_17635_ = _e2649;
                                                        phi_17636_ = 1000000000000000000000000000000f;
                                                    }
                                                    let _e3153 = phi_34683_;
                                                    let _e3155 = phi_17633_;
                                                    let _e3157 = phi_17634_;
                                                    let _e3159 = phi_17635_;
                                                    let _e3161 = phi_17636_;
                                                    phi_34682_ = _e3153;
                                                    phi_17637_ = _e3155;
                                                    phi_17638_ = _e3157;
                                                    phi_17639_ = _e3159;
                                                    phi_17640_ = _e3161;
                                                }
                                                let _e4403 = phi_34682_;
                                                let _e4405 = phi_17637_;
                                                let _e4407 = phi_17638_;
                                                let _e4409 = phi_17639_;
                                                let _e4411 = phi_17640_;
                                                phi_34681_ = _e4403;
                                                phi_17641_ = _e4405;
                                                phi_17642_ = _e4407;
                                                phi_17643_ = _e4409;
                                                phi_17644_ = _e4411;
                                            }
                                            let _e4480 = phi_34681_;
                                            let _e4482 = phi_17641_;
                                            let _e4484 = phi_17642_;
                                            let _e4486 = phi_17643_;
                                            let _e4488 = phi_17644_;
                                            if ((((_e4486.x * _e4486.x) + (_e4486.y * _e4486.y)) + (_e4486.z * _e4486.z)) > 0f) {
                                                let _e4505 = (((_e4484.x * _e1511.x) + (_e4484.y * _e1511.y)) + (_e4484.z * _e1511.z));
                                                if (_e4505 != _e4505) {
                                                    phi_28269_ = true;
                                                } else {
                                                    phi_28269_ = (0f >= _e4505);
                                                }
                                                let _e4509 = phi_28269_;
                                                let _e4511 = (select(_e4505, 0f, _e4509) * 0.31830987f);
                                                if (_e2403 == 0u) {
                                                    if (_e4505 <= 0f) {
                                                        phi_29066_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_29066_ = (_e2397 * (_e4505 * 0.31830987f));
                                                    }
                                                    let _e4987 = phi_29066_;
                                                    phi_18040_ = type_57(_e4987, _e4511);
                                                    phi_18041_ = type_57();
                                                    phi_18042_ = true;
                                                } else {
                                                    let _e4517 = (((_e1483 * _e1559) + (_e1484 * _e1560)) + (_e1485 * _e1562));
                                                    let _e4522 = (((_e1483 * _e1555) + (_e1484 * _e1565)) + (_e1485 * _e1566));
                                                    let _e4527 = (((_e1483 * _e1511.x) + (_e1484 * _e1511.y)) + (_e1485 * _e1511.z));
                                                    let _e4532 = (((_e4484.x * _e1559) + (_e4484.y * _e1560)) + (_e4484.z * _e1562));
                                                    let _e4537 = (((_e4484.x * _e1555) + (_e4484.y * _e1565)) + (_e4484.z * _e1566));
                                                    if (_e4527 <= 0f) {
                                                        phi_18004_ = type_57();
                                                        phi_18005_ = true;
                                                    } else {
                                                        let _e4539 = (_e4505 <= 0f);
                                                        if _e4539 {
                                                            phi_18003_ = type_57();
                                                        } else {
                                                            let _e4540 = (_e4517 + _e4532);
                                                            let _e4541 = (_e4522 + _e4537);
                                                            let _e4542 = (_e4527 + _e4505);
                                                            let _e4549 = (1f / sqrt((((_e4540 * _e4540) + (_e4541 * _e4541)) + (_e4542 * _e4542))));
                                                            let _e4552 = (_e4542 * _e4549);
                                                            if (_e4552 <= 0f) {
                                                                phi_28379_ = 0f;
                                                            } else {
                                                                let _e4554 = (_e2401 * _e2401);
                                                                let _e4558 = ((((_e4554 - 1f) * _e4552) * _e4552) + 1f);
                                                                phi_28379_ = (_e4554 / ((3.1415927f * _e4558) * _e4558));
                                                            }
                                                            let _e4563 = phi_28379_;
                                                            let _e4564 = (_e4527 * _e4527);
                                                            if (_e4564 >= 1f) {
                                                                phi_28407_ = 0f;
                                                            } else {
                                                                phi_28407_ = ((sqrt((1f + (((_e2401 * _e2401) * ((_e4517 * _e4517) + (_e4522 * _e4522))) / _e4564))) - 1f) * 0.5f);
                                                            }
                                                            let _e4577 = phi_28407_;
                                                            let _e4578 = (1f + _e4577);
                                                            let _e4579 = (_e4505 * _e4505);
                                                            if (_e4579 >= 1f) {
                                                                phi_28435_ = 0f;
                                                            } else {
                                                                phi_28435_ = ((sqrt((1f + (((_e2401 * _e2401) * ((_e4532 * _e4532) + (_e4537 * _e4537))) / _e4579))) - 1f) * 0.5f);
                                                            }
                                                            let _e4592 = phi_28435_;
                                                            let _e4599 = (((_e4517 * (_e4540 * _e4549)) + (_e4522 * (_e4541 * _e4549))) + (_e4527 * _e4552));
                                                            if (_e2399 == 0u) {
                                                                let _e4894 = (1f - _e4599);
                                                                if (_e4894 != _e4894) {
                                                                    phi_28938_ = true;
                                                                } else {
                                                                    phi_28938_ = (0f >= _e4894);
                                                                }
                                                                let _e4898 = phi_28938_;
                                                                let _e4900 = pow(select(_e4894, 0f, _e4898), 5f);
                                                                let _e4901 = (1f - _e4900);
                                                                phi_17991_ = vec3<f32>(((_e2397.x * _e4901) + _e4900), ((_e2397.y * _e4901) + _e4900), ((_e2397.z * _e4901) + _e4900));
                                                            } else {
                                                                if (_e2399 == 1u) {
                                                                    let _e4867 = (1f / _e2393);
                                                                    let _e4871 = ((_e4867 * _e4867) * (1f - (_e4599 * _e4599)));
                                                                    if (_e4871 >= 1f) {
                                                                        phi_28899_ = 1f;
                                                                    } else {
                                                                        let _e4874 = sqrt((1f - _e4871));
                                                                        if (_e4599 != _e4599) {
                                                                            phi_28910_ = true;
                                                                        } else {
                                                                            phi_28910_ = (_e4874 <= _e4599);
                                                                        }
                                                                        let _e4878 = phi_28910_;
                                                                        let _e4883 = pow(((1f - _e2393) / (1f + _e2393)), 2f);
                                                                        phi_28899_ = (_e4883 + ((1f - _e4883) * pow((1f - select(_e4599, _e4874, _e4878)), 5f)));
                                                                    }
                                                                    let _e4890 = phi_28899_;
                                                                    phi_17990_ = vec3(_e4890);
                                                                } else {
                                                                    let _e4603 = select(_e4599, 0f, (_e4599 < 0f));
                                                                    let _e4605 = select(_e4603, 1f, (_e4603 > 1f));
                                                                    let _e4607 = (1f - (_e4605 * _e4605));
                                                                    let _e4620 = ((_e2397.x * _e2397.x) - (_e2395.x * _e2395.x));
                                                                    let _e4621 = ((_e2397.y * _e2397.y) - (_e2395.y * _e2395.y));
                                                                    let _e4622 = ((_e2397.z * _e2397.z) - (_e2395.z * _e2395.z));
                                                                    let _e4624 = (_e2397.x * (_e2395.x + _e2395.x));
                                                                    let _e4626 = (_e2397.y * (_e2395.y + _e2395.y));
                                                                    let _e4628 = (_e2397.z * (_e2395.z + _e2395.z));
                                                                    let _e4635 = ((_e4620 * _e4620) + (_e4624 * _e4624));
                                                                    let _e4636 = ((_e4621 * _e4621) + (_e4626 * _e4626));
                                                                    let _e4637 = ((_e4622 * _e4622) + (_e4628 * _e4628));
                                                                    let _e4642 = ((_e4607 * _e4620) / _e4635);
                                                                    let _e4643 = ((_e4607 * _e4621) / _e4636);
                                                                    let _e4644 = ((_e4607 * _e4622) / _e4637);
                                                                    let _e4645 = (1f - _e4642);
                                                                    let _e4646 = (1f - _e4643);
                                                                    let _e4647 = (1f - _e4644);
                                                                    let _e4649 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4642, _e4643, _e4644));
                                                                    let _e4650 = (_e4607 * _e4624);
                                                                    let _e4651 = (_e4607 * _e4626);
                                                                    let _e4652 = (_e4607 * _e4628);
                                                                    let _e4657 = (vec3<f32>(_e4650, _e4651, _e4652) / vec3<f32>(_e4635, _e4636, _e4637));
                                                                    let _e4670 = ((_e4649.x * _e4649.x) + (_e4657.x * _e4657.x));
                                                                    let _e4671 = ((_e4649.y * _e4649.y) + (_e4657.y * _e4657.y));
                                                                    let _e4672 = ((_e4649.z * _e4649.z) + (_e4657.z * _e4657.z));
                                                                    if (_e4670 != _e4670) {
                                                                        phi_28569_ = true;
                                                                    } else {
                                                                        phi_28569_ = (0f >= _e4670);
                                                                    }
                                                                    let _e4676 = phi_28569_;
                                                                    let _e4678 = sqrt(select(_e4670, 0f, _e4676));
                                                                    if (_e4671 != _e4671) {
                                                                        phi_28584_ = true;
                                                                    } else {
                                                                        phi_28584_ = (0f >= _e4671);
                                                                    }
                                                                    let _e4682 = phi_28584_;
                                                                    let _e4684 = sqrt(select(_e4671, 0f, _e4682));
                                                                    if (_e4672 != _e4672) {
                                                                        phi_28599_ = true;
                                                                    } else {
                                                                        phi_28599_ = (0f >= _e4672);
                                                                    }
                                                                    let _e4688 = phi_28599_;
                                                                    let _e4690 = sqrt(select(_e4672, 0f, _e4688));
                                                                    let _e4694 = ((_e4678 + _e4645) * 0.5f);
                                                                    let _e4695 = ((_e4684 + _e4646) * 0.5f);
                                                                    let _e4696 = ((_e4690 + _e4647) * 0.5f);
                                                                    if (_e4694 != _e4694) {
                                                                        phi_28626_ = true;
                                                                    } else {
                                                                        phi_28626_ = (0f >= _e4694);
                                                                    }
                                                                    let _e4700 = phi_28626_;
                                                                    let _e4702 = sqrt(select(_e4694, 0f, _e4700));
                                                                    if (_e4695 != _e4695) {
                                                                        phi_28641_ = true;
                                                                    } else {
                                                                        phi_28641_ = (0f >= _e4695);
                                                                    }
                                                                    let _e4706 = phi_28641_;
                                                                    let _e4708 = sqrt(select(_e4695, 0f, _e4706));
                                                                    if (_e4696 != _e4696) {
                                                                        phi_28656_ = true;
                                                                    } else {
                                                                        phi_28656_ = (0f >= _e4696);
                                                                    }
                                                                    let _e4712 = phi_28656_;
                                                                    let _e4714 = sqrt(select(_e4696, 0f, _e4712));
                                                                    let _e4715 = vec3<f32>(_e4702, _e4708, _e4714);
                                                                    let _e4719 = ((_e4678 - _e4645) * 0.5f);
                                                                    let _e4720 = ((_e4684 - _e4646) * 0.5f);
                                                                    let _e4721 = ((_e4690 - _e4647) * 0.5f);
                                                                    if (_e4719 != _e4719) {
                                                                        phi_28683_ = true;
                                                                    } else {
                                                                        phi_28683_ = (0f >= _e4719);
                                                                    }
                                                                    let _e4725 = phi_28683_;
                                                                    let _e4727 = sqrt(select(_e4719, 0f, _e4725));
                                                                    if (_e4720 != _e4720) {
                                                                        phi_28698_ = true;
                                                                    } else {
                                                                        phi_28698_ = (0f >= _e4720);
                                                                    }
                                                                    let _e4731 = phi_28698_;
                                                                    let _e4733 = sqrt(select(_e4720, 0f, _e4731));
                                                                    if (_e4721 != _e4721) {
                                                                        phi_28713_ = true;
                                                                    } else {
                                                                        phi_28713_ = (0f >= _e4721);
                                                                    }
                                                                    let _e4737 = phi_28713_;
                                                                    let _e4739 = sqrt(select(_e4721, 0f, _e4737));
                                                                    if ((_e4650 / _e4635) < 0f) {
                                                                        phi_17867_ = -(_e4727);
                                                                    } else {
                                                                        phi_17867_ = _e4727;
                                                                    }
                                                                    let _e4743 = phi_17867_;
                                                                    if ((_e4651 / _e4636) < 0f) {
                                                                        phi_17875_ = -(_e4733);
                                                                    } else {
                                                                        phi_17875_ = _e4733;
                                                                    }
                                                                    let _e4747 = phi_17875_;
                                                                    if ((_e4652 / _e4637) < 0f) {
                                                                        phi_17883_ = -(_e4739);
                                                                    } else {
                                                                        phi_17883_ = _e4739;
                                                                    }
                                                                    let _e4751 = phi_17883_;
                                                                    let _e4755 = (_e2395.x * _e4605);
                                                                    let _e4756 = (_e2395.y * _e4605);
                                                                    let _e4757 = (_e2395.z * _e4605);
                                                                    let _e4767 = vec3<f32>(((_e2397.x * _e4702) - (_e2395.x * _e4743)), ((_e2397.y * _e4708) - (_e2395.y * _e4747)), ((_e2397.z * _e4714) - (_e2395.z * _e4751)));
                                                                    let _e4774 = ((_e2397.x * _e4743) + (_e2395.x * _e4702));
                                                                    let _e4775 = ((_e2397.y * _e4747) + (_e2395.y * _e4708));
                                                                    let _e4776 = ((_e2397.z * _e4751) + (_e2395.z * _e4714));
                                                                    let _e4778 = vec3<f32>((_e2397.x * _e4605), (_e2397.y * _e4605), (_e2397.z * _e4605));
                                                                    let _e4779 = (_e4778 - _e4715);
                                                                    let _e4780 = (_e4755 - _e4743);
                                                                    let _e4781 = (_e4756 - _e4747);
                                                                    let _e4782 = (_e4757 - _e4751);
                                                                    let _e4795 = (_e4778 + _e4715);
                                                                    let _e4796 = (_e4755 + _e4743);
                                                                    let _e4797 = (_e4756 + _e4747);
                                                                    let _e4798 = (_e4757 + _e4751);
                                                                    let _e4808 = ((_e4795.x * _e4795.x) + (_e4796 * _e4796));
                                                                    let _e4809 = ((_e4795.y * _e4795.y) + (_e4797 * _e4797));
                                                                    let _e4810 = ((_e4795.z * _e4795.z) + (_e4798 * _e4798));
                                                                    let _e4820 = vec3(_e4605);
                                                                    let _e4821 = (_e4820 - _e4767);
                                                                    let _e4822 = -(vec3<f32>(_e4774, _e4775, _e4776));
                                                                    let _e4838 = (_e4820 + _e4767);
                                                                    let _e4848 = ((_e4838.x * _e4838.x) + (_e4774 * _e4774));
                                                                    let _e4849 = ((_e4838.y * _e4838.y) + (_e4775 * _e4775));
                                                                    let _e4850 = ((_e4838.z * _e4838.z) + (_e4776 * _e4776));
                                                                    phi_17990_ = vec3<f32>((((((_e4779.x * _e4779.x) + (_e4780 * _e4780)) / select(0.000000000000000000000000000001f, _e4808, (_e4808 > 0.000000000000000000000000000001f))) + (((_e4821.x * _e4821.x) + (_e4822.x * _e4822.x)) / select(0.000000000000000000000000000001f, _e4848, (_e4848 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4779.y * _e4779.y) + (_e4781 * _e4781)) / select(0.000000000000000000000000000001f, _e4809, (_e4809 > 0.000000000000000000000000000001f))) + (((_e4821.y * _e4821.y) + (_e4822.y * _e4822.y)) / select(0.000000000000000000000000000001f, _e4849, (_e4849 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4779.z * _e4779.z) + (_e4782 * _e4782)) / select(0.000000000000000000000000000001f, _e4810, (_e4810 > 0.000000000000000000000000000001f))) + (((_e4821.z * _e4821.z) + (_e4822.z * _e4822.z)) / select(0.000000000000000000000000000001f, _e4850, (_e4850 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                }
                                                                let _e4893 = phi_17990_;
                                                                phi_17991_ = _e4893;
                                                            }
                                                            let _e4913 = phi_17991_;
                                                            let _e4915 = (4f * _e4527);
                                                            phi_18003_ = type_57((_e4913 * (((1f / (_e4578 + _e4592)) * _e4563) / _e4915)), ((_e4563 / _e4578) / _e4915));
                                                        }
                                                        let _e4922 = phi_18003_;
                                                        phi_18004_ = _e4922;
                                                        phi_18005_ = _e4539;
                                                    }
                                                    let _e4924 = phi_18004_;
                                                    let _e4926 = phi_18005_;
                                                    if _e4926 {
                                                        phi_18010_ = type_57(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        phi_18010_ = _e4924;
                                                    }
                                                    let _e4928 = phi_18010_;
                                                    let _e4931 = (_e2403 == 1u);
                                                    if _e4931 {
                                                        phi_18038_ = _e4928;
                                                        phi_18039_ = type_57();
                                                    } else {
                                                        if (_e4527 != _e4527) {
                                                            phi_28975_ = true;
                                                        } else {
                                                            phi_28975_ = (0f >= _e4527);
                                                        }
                                                        let _e4935 = phi_28975_;
                                                        let _e4936 = select(_e4527, 0f, _e4935);
                                                        let _e4937 = (1f / _e2393);
                                                        let _e4941 = ((_e4937 * _e4937) * (1f - (_e4936 * _e4936)));
                                                        if (_e4941 >= 1f) {
                                                            phi_28994_ = 1f;
                                                        } else {
                                                            let _e4944 = sqrt((1f - _e4941));
                                                            if (_e4936 != _e4936) {
                                                                phi_29005_ = true;
                                                            } else {
                                                                phi_29005_ = (_e4944 <= _e4936);
                                                            }
                                                            let _e4948 = phi_29005_;
                                                            let _e4953 = pow(((1f - _e2393) / (1f + _e2393)), 2f);
                                                            phi_28994_ = (_e4953 + ((1f - _e4953) * pow((1f - select(_e4936, _e4944, _e4948)), 5f)));
                                                        }
                                                        let _e4960 = phi_28994_;
                                                        if (_e4505 <= 0f) {
                                                            phi_29044_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_29044_ = (_e2391 * (_e4505 * 0.31830987f));
                                                        }
                                                        let _e4965 = phi_29044_;
                                                        let _e4966 = (1f - _e4960);
                                                        phi_18038_ = type_57();
                                                        phi_18039_ = type_57((_e4928.member + vec3<f32>((_e4965.x * _e4966), (_e4965.y * _e4966), (_e4965.z * _e4966))), ((_e4960 * _e4928.member_1) + (_e4966 * _e4511)));
                                                    }
                                                    let _e4980 = phi_18038_;
                                                    let _e4982 = phi_18039_;
                                                    phi_18040_ = _e4980;
                                                    phi_18041_ = _e4982;
                                                    phi_18042_ = _e4931;
                                                }
                                                let _e4990 = phi_18040_;
                                                let _e4992 = phi_18041_;
                                                let _e4994 = phi_18042_;
                                                if _e4994 {
                                                    phi_18046_ = _e4990;
                                                } else {
                                                    phi_18046_ = _e4992;
                                                }
                                                let _e4996 = phi_18046_;
                                                if (_e4482 > 0f) {
                                                    if (_e4482 >= 300000000000000000000000000000000000000f) {
                                                        phi_29083_ = 1f;
                                                    } else {
                                                        let _e5001 = (_e4482 * _e4482);
                                                        let _e5003 = (_e5001 + (_e4996.member_1 * _e4996.member_1));
                                                        if (_e5003 == 0f) {
                                                            phi_29080_ = 0f;
                                                        } else {
                                                            phi_29080_ = (_e5001 / _e5003);
                                                        }
                                                        let _e5007 = phi_29080_;
                                                        phi_29083_ = _e5007;
                                                    }
                                                    let _e5009 = phi_29083_;
                                                    phi_18055_ = (_e5009 / _e4482);
                                                } else {
                                                    phi_18055_ = 1f;
                                                }
                                                let _e5012 = phi_18055_;
                                                if ((((_e4996.member.x * _e4996.member.x) + (_e4996.member.y * _e4996.member.y)) + (_e4996.member.z * _e4996.member.z)) > 0f) {
                                                    let _e5024 = (vec3<f32>(1f, 1f, 1f) / _e4484);
                                                    local_67 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                    phi_18089_ = _e372;
                                                    phi_18092_ = _e379;
                                                    phi_18094_ = _e376;
                                                    phi_18098_ = type_64(_e4488, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                    loop {
                                                        let _e5026 = phi_18089_;
                                                        let _e5028 = phi_18092_;
                                                        let _e5030 = phi_18094_;
                                                        let _e5032 = phi_18098_;
                                                        local_106 = _e5032;
                                                        if (_e5026 < _e5030) {
                                                            let _e5201 = global_2.member[_e5026];
                                                            if (_e5201.member_6.x == 0u) {
                                                                let _e5307 = (_e5201.member_1.x - _e5201.member.x);
                                                                let _e5308 = (_e5201.member_1.y - _e5201.member.y);
                                                                let _e5309 = (_e5201.member_1.z - _e5201.member.z);
                                                                let _e5314 = (_e5201.member_2.x - _e5201.member.x);
                                                                let _e5315 = (_e5201.member_2.y - _e5201.member.y);
                                                                let _e5316 = (_e5201.member_2.z - _e5201.member.z);
                                                                let _e5319 = ((_e4484.y * _e5316) - (_e5315 * _e4484.z));
                                                                let _e5322 = ((_e4484.z * _e5314) - (_e5316 * _e4484.x));
                                                                let _e5325 = ((_e4484.x * _e5315) - (_e5314 * _e4484.y));
                                                                let _e5330 = (((_e5307 * _e5319) + (_e5308 * _e5322)) + (_e5309 * _e5325));
                                                                if (abs(_e5330) < 0.000000000001f) {
                                                                    phi_18265_ = vec4<f32>();
                                                                    phi_18266_ = true;
                                                                } else {
                                                                    let _e5333 = (1f / _e5330);
                                                                    let _e5334 = (_e1536 - _e5201.member.x);
                                                                    let _e5335 = (_e1537 - _e5201.member.y);
                                                                    let _e5336 = (_e1538 - _e5201.member.z);
                                                                    let _e5342 = ((((_e5334 * _e5319) + (_e5335 * _e5322)) + (_e5336 * _e5325)) * _e5333);
                                                                    if (_e5342 < 0f) {
                                                                        phi_18255_ = vec4<f32>();
                                                                        phi_18256_ = true;
                                                                        phi_18257_ = false;
                                                                    } else {
                                                                        let _e5344 = (_e5342 > 1f);
                                                                        if _e5344 {
                                                                            phi_18253_ = vec4<f32>();
                                                                            phi_18254_ = false;
                                                                        } else {
                                                                            let _e5347 = ((_e5335 * _e5309) - (_e5308 * _e5336));
                                                                            let _e5350 = ((_e5336 * _e5307) - (_e5309 * _e5334));
                                                                            let _e5353 = ((_e5334 * _e5308) - (_e5307 * _e5335));
                                                                            let _e5359 = ((((_e4484.x * _e5347) + (_e4484.y * _e5350)) + (_e4484.z * _e5353)) * _e5333);
                                                                            if (_e5359 < 0f) {
                                                                                phi_18243_ = vec4<f32>();
                                                                                phi_18244_ = true;
                                                                                phi_18245_ = false;
                                                                            } else {
                                                                                let _e5362 = ((_e5342 + _e5359) > 1f);
                                                                                if _e5362 {
                                                                                    phi_18241_ = vec4<f32>();
                                                                                    phi_18242_ = false;
                                                                                } else {
                                                                                    let _e5368 = ((((_e5314 * _e5347) + (_e5315 * _e5350)) + (_e5316 * _e5353)) * _e5333);
                                                                                    if (_e5368 <= 0f) {
                                                                                        phi_18236_ = vec4<f32>();
                                                                                        phi_18237_ = true;
                                                                                    } else {
                                                                                        let _e5370 = (_e5368 >= _e5032.member);
                                                                                        if _e5370 {
                                                                                            phi_18235_ = vec4<f32>();
                                                                                        } else {
                                                                                            phi_18235_ = vec4<f32>(_e5368, _e5342, _e5359, 1f);
                                                                                        }
                                                                                        let _e5373 = phi_18235_;
                                                                                        phi_18236_ = _e5373;
                                                                                        phi_18237_ = _e5370;
                                                                                    }
                                                                                    let _e5375 = phi_18236_;
                                                                                    let _e5377 = phi_18237_;
                                                                                    phi_18241_ = _e5375;
                                                                                    phi_18242_ = _e5377;
                                                                                }
                                                                                let _e5379 = phi_18241_;
                                                                                let _e5381 = phi_18242_;
                                                                                phi_18243_ = _e5379;
                                                                                phi_18244_ = _e5362;
                                                                                phi_18245_ = _e5381;
                                                                            }
                                                                            let _e5383 = phi_18243_;
                                                                            let _e5385 = phi_18244_;
                                                                            let _e5387 = phi_18245_;
                                                                            phi_18253_ = _e5383;
                                                                            phi_18254_ = select(_e5387, true, _e5385);
                                                                        }
                                                                        let _e5390 = phi_18253_;
                                                                        let _e5392 = phi_18254_;
                                                                        phi_18255_ = _e5390;
                                                                        phi_18256_ = _e5344;
                                                                        phi_18257_ = _e5392;
                                                                    }
                                                                    let _e5394 = phi_18255_;
                                                                    let _e5396 = phi_18256_;
                                                                    let _e5398 = phi_18257_;
                                                                    phi_18265_ = _e5394;
                                                                    phi_18266_ = select(_e5398, true, _e5396);
                                                                }
                                                                let _e5401 = phi_18265_;
                                                                let _e5403 = phi_18266_;
                                                                phi_18412_ = _e5401;
                                                                phi_18413_ = _e5403;
                                                            } else {
                                                                if (_e5201.member_6.x == 1u) {
                                                                    let _e5229 = (_e1536 - _e5201.member.x);
                                                                    let _e5230 = (_e1537 - _e5201.member.y);
                                                                    let _e5231 = (_e1538 - _e5201.member.z);
                                                                    let _e5236 = (((_e5229 * _e4484.x) + (_e5230 * _e4484.y)) + (_e5231 * _e4484.z));
                                                                    if (_e5236 > 0f) {
                                                                        phi_18306_ = select(true, false, ((((_e5229 * _e5229) + (_e5230 * _e5230)) + (_e5231 * _e5231)) > (_e5201.member.w * _e5201.member.w)));
                                                                    } else {
                                                                        phi_18306_ = true;
                                                                    }
                                                                    let _e5249 = phi_18306_;
                                                                    if _e5249 {
                                                                        let _e5253 = (_e5229 - (_e4484.x * _e5236));
                                                                        let _e5254 = (_e5230 - (_e4484.y * _e5236));
                                                                        let _e5255 = (_e5231 - (_e4484.z * _e5236));
                                                                        let _e5264 = ((_e5201.member.w * _e5201.member.w) - (((_e5253 * _e5253) + (_e5254 * _e5254)) + (_e5255 * _e5255)));
                                                                        if (_e5264 < 0f) {
                                                                            phi_18365_ = vec4<f32>();
                                                                            phi_18366_ = true;
                                                                        } else {
                                                                            let _e5266 = sqrt(_e5264);
                                                                            let _e5267 = -(_e5236);
                                                                            let _e5268 = (_e5267 - _e5266);
                                                                            if (_e5268 <= 0f) {
                                                                                phi_18343_ = true;
                                                                            } else {
                                                                                phi_18343_ = (_e5268 >= _e5032.member);
                                                                            }
                                                                            let _e5272 = phi_18343_;
                                                                            if _e5272 {
                                                                                phi_18349_ = (_e5267 + _e5266);
                                                                            } else {
                                                                                phi_18349_ = _e5268;
                                                                            }
                                                                            let _e5275 = phi_18349_;
                                                                            if (_e5275 <= 0f) {
                                                                                phi_18360_ = vec4<f32>();
                                                                                phi_18361_ = true;
                                                                            } else {
                                                                                let _e5277 = (_e5275 >= _e5032.member);
                                                                                if _e5277 {
                                                                                    phi_18359_ = vec4<f32>();
                                                                                } else {
                                                                                    phi_18359_ = vec4<f32>(_e5275, 0f, 0f, 1f);
                                                                                }
                                                                                let _e5280 = phi_18359_;
                                                                                phi_18360_ = _e5280;
                                                                                phi_18361_ = _e5277;
                                                                            }
                                                                            let _e5282 = phi_18360_;
                                                                            let _e5284 = phi_18361_;
                                                                            phi_18365_ = _e5282;
                                                                            phi_18366_ = _e5284;
                                                                        }
                                                                        let _e5286 = phi_18365_;
                                                                        let _e5288 = phi_18366_;
                                                                        phi_18370_ = _e5286;
                                                                        phi_18371_ = _e5288;
                                                                    } else {
                                                                        phi_18370_ = vec4<f32>();
                                                                        phi_18371_ = true;
                                                                    }
                                                                    let _e5290 = phi_18370_;
                                                                    let _e5292 = phi_18371_;
                                                                    phi_18402_ = _e5290;
                                                                    phi_18403_ = _e5292;
                                                                } else {
                                                                    if (abs(_e4484.y) < 0.000000000001f) {
                                                                        phi_18400_ = vec4<f32>();
                                                                        phi_18401_ = true;
                                                                    } else {
                                                                        let _e5211 = ((_e5201.member.x - _e1537) / _e4484.y);
                                                                        if (_e5211 <= 0f) {
                                                                            phi_18395_ = vec4<f32>();
                                                                            phi_18396_ = true;
                                                                        } else {
                                                                            let _e5213 = (_e5211 >= _e5032.member);
                                                                            if _e5213 {
                                                                                phi_18394_ = vec4<f32>();
                                                                            } else {
                                                                                phi_18394_ = vec4<f32>(_e5211, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5216 = phi_18394_;
                                                                            phi_18395_ = _e5216;
                                                                            phi_18396_ = _e5213;
                                                                        }
                                                                        let _e5218 = phi_18395_;
                                                                        let _e5220 = phi_18396_;
                                                                        phi_18400_ = _e5218;
                                                                        phi_18401_ = _e5220;
                                                                    }
                                                                    let _e5222 = phi_18400_;
                                                                    let _e5224 = phi_18401_;
                                                                    phi_18402_ = _e5222;
                                                                    phi_18403_ = _e5224;
                                                                }
                                                                let _e5294 = phi_18402_;
                                                                let _e5296 = phi_18403_;
                                                                phi_18412_ = _e5294;
                                                                phi_18413_ = select(true, false, select(true, false, _e5296));
                                                            }
                                                            let _e5405 = phi_18412_;
                                                            let _e5407 = phi_18413_;
                                                            let _e5410 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5405, vec4(select(true, false, _e5407)));
                                                            let _e5412 = (_e5410.w > 0f);
                                                            if _e5412 {
                                                                let _e5418 = type_64(_e5410.x, _e5032.member_1, _e5032.member_2, _e5032.member_3);
                                                                let _e5423 = type_64(_e5418.member, _e5026, _e5418.member_2, _e5418.member_3);
                                                                phi_18434_ = _e5423;
                                                                phi_18436_ = _e5423;
                                                            } else {
                                                                phi_18434_ = type_64();
                                                                phi_18436_ = _e5032;
                                                            }
                                                            let _e5425 = phi_18434_;
                                                            let _e5427 = phi_18436_;
                                                            let _e5428 = select(true, false, _e5412);
                                                            if _e5428 {
                                                                phi_18442_ = (_e5026 + 1u);
                                                            } else {
                                                                phi_18442_ = u32();
                                                            }
                                                            let _e5431 = phi_18442_;
                                                            phi_18090_ = _e5431;
                                                            phi_18093_ = _e5028;
                                                            phi_18095_ = _e5030;
                                                            phi_18099_ = _e5427;
                                                            phi_18515_ = _e5425;
                                                            phi_18516_ = _e5428;
                                                            phi_18517_ = false;
                                                        } else {
                                                            let _e5034 = (_e5028 == 0u);
                                                            if _e5034 {
                                                                phi_18511_ = u32();
                                                                phi_18512_ = u32();
                                                                phi_18513_ = u32();
                                                            } else {
                                                                let _e5035 = (_e5028 - 1u);
                                                                let _e5036 = (_e5035 < 100u);
                                                                if _e5036 {
                                                                } else {
                                                                    phi_34240_ = true;
                                                                    phi_20379_ = bool();
                                                                    phi_20378_ = type_64();
                                                                    break;
                                                                }
                                                                let _e5038 = local_67[_e5035];
                                                                let _e5040 = global_3.member[_e5038];
                                                                let _e5051 = ((_e5040.member.x - _e1536) * _e5024.x);
                                                                let _e5053 = ((_e5040.member.y - _e1537) * _e5024.y);
                                                                let _e5055 = ((_e5040.member.z - _e1538) * _e5024.z);
                                                                let _e5062 = ((_e5040.member_2.x - _e1536) * _e5024.x);
                                                                let _e5063 = ((_e5040.member_2.y - _e1537) * _e5024.y);
                                                                let _e5064 = ((_e5040.member_2.z - _e1538) * _e5024.z);
                                                                if (_e5051 == _e5051) {
                                                                    let _e5066 = (_e5062 == _e5062);
                                                                    if _e5066 {
                                                                        let _e5067 = (_e5051 != _e5051);
                                                                        if _e5067 {
                                                                            phi_29180_ = true;
                                                                        } else {
                                                                            phi_29180_ = (_e5062 <= _e5051);
                                                                        }
                                                                        let _e5070 = phi_29180_;
                                                                        let _e5071 = select(_e5051, _e5062, _e5070);
                                                                        if _e5067 {
                                                                            phi_29210_ = true;
                                                                        } else {
                                                                            phi_29210_ = (_e5062 >= _e5051);
                                                                        }
                                                                        let _e5076 = phi_29210_;
                                                                        let _e5077 = select(_e5051, _e5062, _e5076);
                                                                        phi_29157_ = type_54(select(-300000000000000000000000000000000000000f, _e5071, (_e5071 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5077, (_e5077 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_29157_ = type_54();
                                                                    }
                                                                    let _e5082 = phi_29157_;
                                                                    phi_29160_ = _e5082;
                                                                    phi_29161_ = select(true, false, _e5066);
                                                                } else {
                                                                    phi_29160_ = type_54();
                                                                    phi_29161_ = true;
                                                                }
                                                                let _e5085 = phi_29160_;
                                                                let _e5087 = phi_29161_;
                                                                if _e5087 {
                                                                    phi_29166_ = type_54(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_29166_ = _e5085;
                                                                }
                                                                let _e5089 = phi_29166_;
                                                                if (_e5053 == _e5053) {
                                                                    let _e5093 = (_e5063 == _e5063);
                                                                    if _e5093 {
                                                                        let _e5094 = (_e5053 != _e5053);
                                                                        if _e5094 {
                                                                            phi_29267_ = true;
                                                                        } else {
                                                                            phi_29267_ = (_e5063 <= _e5053);
                                                                        }
                                                                        let _e5097 = phi_29267_;
                                                                        let _e5098 = select(_e5053, _e5063, _e5097);
                                                                        if (_e5089.member != _e5089.member) {
                                                                            phi_29282_ = true;
                                                                        } else {
                                                                            phi_29282_ = (_e5098 >= _e5089.member);
                                                                        }
                                                                        let _e5102 = phi_29282_;
                                                                        if _e5094 {
                                                                            phi_29297_ = true;
                                                                        } else {
                                                                            phi_29297_ = (_e5063 >= _e5053);
                                                                        }
                                                                        let _e5106 = phi_29297_;
                                                                        let _e5107 = select(_e5053, _e5063, _e5106);
                                                                        if (_e5089.member_1 != _e5089.member_1) {
                                                                            phi_29312_ = true;
                                                                        } else {
                                                                            phi_29312_ = (_e5107 <= _e5089.member_1);
                                                                        }
                                                                        let _e5111 = phi_29312_;
                                                                        phi_29244_ = type_54(select(_e5089.member, _e5098, _e5102), select(_e5089.member_1, _e5107, _e5111));
                                                                    } else {
                                                                        phi_29244_ = type_54();
                                                                    }
                                                                    let _e5115 = phi_29244_;
                                                                    phi_29247_ = _e5115;
                                                                    phi_29248_ = select(true, false, _e5093);
                                                                } else {
                                                                    phi_29247_ = type_54();
                                                                    phi_29248_ = true;
                                                                }
                                                                let _e5118 = phi_29247_;
                                                                let _e5120 = phi_29248_;
                                                                if _e5120 {
                                                                    phi_29253_ = _e5089;
                                                                } else {
                                                                    phi_29253_ = _e5118;
                                                                }
                                                                let _e5122 = phi_29253_;
                                                                if (_e5055 == _e5055) {
                                                                    let _e5126 = (_e5064 == _e5064);
                                                                    if _e5126 {
                                                                        let _e5127 = (_e5055 != _e5055);
                                                                        if _e5127 {
                                                                            phi_29354_ = true;
                                                                        } else {
                                                                            phi_29354_ = (_e5064 <= _e5055);
                                                                        }
                                                                        let _e5130 = phi_29354_;
                                                                        let _e5131 = select(_e5055, _e5064, _e5130);
                                                                        if (_e5122.member != _e5122.member) {
                                                                            phi_29369_ = true;
                                                                        } else {
                                                                            phi_29369_ = (_e5131 >= _e5122.member);
                                                                        }
                                                                        let _e5135 = phi_29369_;
                                                                        if _e5127 {
                                                                            phi_29384_ = true;
                                                                        } else {
                                                                            phi_29384_ = (_e5064 >= _e5055);
                                                                        }
                                                                        let _e5139 = phi_29384_;
                                                                        let _e5140 = select(_e5055, _e5064, _e5139);
                                                                        if (_e5122.member_1 != _e5122.member_1) {
                                                                            phi_29399_ = true;
                                                                        } else {
                                                                            phi_29399_ = (_e5140 <= _e5122.member_1);
                                                                        }
                                                                        let _e5144 = phi_29399_;
                                                                        phi_29331_ = type_54(select(_e5122.member, _e5131, _e5135), select(_e5122.member_1, _e5140, _e5144));
                                                                    } else {
                                                                        phi_29331_ = type_54();
                                                                    }
                                                                    let _e5148 = phi_29331_;
                                                                    phi_29334_ = _e5148;
                                                                    phi_29335_ = select(true, false, _e5126);
                                                                } else {
                                                                    phi_29334_ = type_54();
                                                                    phi_29335_ = true;
                                                                }
                                                                let _e5151 = phi_29334_;
                                                                let _e5153 = phi_29335_;
                                                                if _e5153 {
                                                                    phi_29340_ = _e5122;
                                                                } else {
                                                                    phi_29340_ = _e5151;
                                                                }
                                                                let _e5155 = phi_29340_;
                                                                if (_e5155.member_1 < 0f) {
                                                                    phi_29136_ = bool();
                                                                    phi_29137_ = true;
                                                                } else {
                                                                    let _e5159 = (_e5155.member > _e5155.member_1);
                                                                    if _e5159 {
                                                                        phi_29133_ = bool();
                                                                    } else {
                                                                        phi_29133_ = (_e5155.member > _e5032.member);
                                                                    }
                                                                    let _e5162 = phi_29133_;
                                                                    phi_29136_ = _e5162;
                                                                    phi_29137_ = _e5159;
                                                                }
                                                                let _e5164 = phi_29136_;
                                                                let _e5166 = phi_29137_;
                                                                if !(select(_e5164, true, _e5166)) {
                                                                    if (_e5040.member_3 > 0u) {
                                                                        phi_18505_ = _e5040.member_1;
                                                                        phi_18506_ = _e5035;
                                                                        phi_18507_ = (_e5040.member_1 + _e5040.member_3);
                                                                    } else {
                                                                        let _e5171 = (_e5028 + 1u);
                                                                        let _e5172 = (_e5171 <= 100u);
                                                                        if _e5172 {
                                                                            if _e5036 {
                                                                            } else {
                                                                                phi_34240_ = true;
                                                                                phi_20379_ = bool();
                                                                                phi_20378_ = type_64();
                                                                                break;
                                                                            }
                                                                            local_67[_e5035] = _e5040.member_1;
                                                                            if (_e5028 < 100u) {
                                                                            } else {
                                                                                phi_34240_ = true;
                                                                                phi_20379_ = bool();
                                                                                phi_20378_ = type_64();
                                                                                break;
                                                                            }
                                                                            local_67[_e5028] = (_e5038 + 1u);
                                                                        }
                                                                        phi_18505_ = _e5026;
                                                                        phi_18506_ = select(_e5035, _e5171, _e5172);
                                                                        phi_18507_ = _e5030;
                                                                    }
                                                                    let _e5181 = phi_18505_;
                                                                    let _e5183 = phi_18506_;
                                                                    let _e5185 = phi_18507_;
                                                                    phi_18508_ = _e5181;
                                                                    phi_18509_ = _e5183;
                                                                    phi_18510_ = _e5185;
                                                                } else {
                                                                    phi_18508_ = _e5026;
                                                                    phi_18509_ = _e5035;
                                                                    phi_18510_ = _e5030;
                                                                }
                                                                let _e5187 = phi_18508_;
                                                                let _e5189 = phi_18509_;
                                                                let _e5191 = phi_18510_;
                                                                phi_18511_ = _e5187;
                                                                phi_18512_ = _e5189;
                                                                phi_18513_ = _e5191;
                                                            }
                                                            let _e5193 = phi_18511_;
                                                            let _e5195 = phi_18512_;
                                                            let _e5197 = phi_18513_;
                                                            phi_18090_ = _e5193;
                                                            phi_18093_ = _e5195;
                                                            phi_18095_ = _e5197;
                                                            phi_18099_ = _e5032;
                                                            phi_18515_ = type_64();
                                                            phi_18516_ = select(true, false, _e5034);
                                                            phi_18517_ = _e5034;
                                                        }
                                                        let _e5433 = phi_18090_;
                                                        let _e5435 = phi_18093_;
                                                        let _e5437 = phi_18095_;
                                                        let _e5439 = phi_18099_;
                                                        let _e5441 = phi_18515_;
                                                        let _e5443 = phi_18516_;
                                                        let _e5445 = phi_18517_;
                                                        phi_18089_ = _e5433;
                                                        phi_18092_ = _e5435;
                                                        phi_18094_ = _e5437;
                                                        phi_18098_ = _e5439;
                                                        phi_34240_ = _e2416;
                                                        phi_20379_ = _e5445;
                                                        phi_20378_ = _e5441;
                                                        if !(_e5443) {
                                                            break;
                                                        }
                                                    }
                                                    let _e5448 = phi_34240_;
                                                    let _e5450 = phi_20379_;
                                                    let _e5452 = phi_20378_;
                                                    phi_34639_ = _e4480;
                                                    phi_34625_ = _e5448;
                                                    if _e5448 {
                                                        break;
                                                    }
                                                    if _e5450 {
                                                        let _e7982 = local_106;
                                                        phi_18689_ = _e7982;
                                                    } else {
                                                        phi_18689_ = type_64();
                                                    }
                                                    let _e5454 = phi_18689_;
                                                    if select(true, false, _e5450) {
                                                        phi_18694_ = _e5452;
                                                    } else {
                                                        phi_18694_ = _e5454;
                                                    }
                                                    let _e5457 = phi_18694_;
                                                    if (_e5457.member_1 != 4294967295u) {
                                                        phi_18748_ = _e2420;
                                                        phi_18749_ = _e2422;
                                                    } else {
                                                        if (_e2424 == (_e2414 + 2u)) {
                                                            phi_18746_ = _e2420;
                                                            phi_18747_ = type_39(vec4<f32>(_e1536, _e1537, _e1538, -1f), vec4<f32>(_e4484.x, _e4484.y, _e4484.z, 0f), vec4<f32>(((_e1406.x * _e4996.member.x) * _e5012), ((_e1406.y * _e4996.member.y) * _e5012), ((_e1406.z * _e4996.member.z) * _e5012), 0f));
                                                        } else {
                                                            phi_18746_ = vec3<f32>((_e2420.x + (((_e1406.x * _e4996.member.x) * _e4486.x) * _e5012)), (_e2420.y + (((_e1406.y * _e4996.member.y) * _e4486.y) * _e5012)), (_e2420.z + (((_e1406.z * _e4996.member.z) * _e4486.z) * _e5012)));
                                                            phi_18747_ = _e2422;
                                                        }
                                                        let _e5495 = phi_18746_;
                                                        let _e5497 = phi_18747_;
                                                        phi_18748_ = _e5495;
                                                        phi_18749_ = _e5497;
                                                    }
                                                    let _e5499 = phi_18748_;
                                                    let _e5501 = phi_18749_;
                                                    phi_34628_ = _e5448;
                                                    phi_18750_ = _e5499;
                                                    phi_18751_ = _e5501;
                                                    phi_18752_ = (_e2426 + 1u);
                                                } else {
                                                    phi_34628_ = _e2416;
                                                    phi_18750_ = _e2420;
                                                    phi_18751_ = _e2422;
                                                    phi_18752_ = _e2426;
                                                }
                                                let _e5503 = phi_34628_;
                                                let _e5505 = phi_18750_;
                                                let _e5507 = phi_18751_;
                                                let _e5509 = phi_18752_;
                                                phi_34627_ = _e5503;
                                                phi_18753_ = _e5505;
                                                phi_18754_ = _e5507;
                                                phi_18755_ = _e5509;
                                            } else {
                                                phi_34627_ = _e2416;
                                                phi_18753_ = _e2420;
                                                phi_18754_ = _e2422;
                                                phi_18755_ = _e2426;
                                            }
                                            let _e5511 = phi_34627_;
                                            let _e5513 = phi_18753_;
                                            let _e5515 = phi_18754_;
                                            let _e5517 = phi_18755_;
                                            phi_34870_ = _e4480;
                                            phi_34626_ = _e5511;
                                            phi_14600_ = _e5513;
                                            phi_14603_ = _e5515;
                                            phi_14605_ = (_e2424 + 1u);
                                            phi_14607_ = _e5517;
                                        } else {
                                            phi_34870_ = _e2418;
                                            phi_34626_ = _e2416;
                                            phi_14600_ = vec3<f32>();
                                            phi_14603_ = type_39();
                                            phi_14605_ = u32();
                                            phi_14607_ = u32();
                                        }
                                        let _e5520 = phi_34870_;
                                        let _e5522 = phi_34626_;
                                        let _e5524 = phi_14600_;
                                        let _e5526 = phi_14603_;
                                        let _e5528 = phi_14605_;
                                        let _e5530 = phi_14607_;
                                        phi_34320_ = _e5522;
                                        phi_32387_ = _e5520;
                                        phi_14599_ = _e5524;
                                        phi_14602_ = _e5526;
                                        phi_14604_ = _e5528;
                                        phi_14606_ = _e5530;
                                        phi_34639_ = _e5520;
                                        phi_34625_ = _e5522;
                                        if !(_e2428) {
                                            break;
                                        }
                                    }
                                    let _e5533 = phi_34639_;
                                    let _e5535 = phi_34625_;
                                    if _e5535 {
                                        break;
                                    }
                                    if (_e347 >= _e363) {
                                        let _e6539 = local_100;
                                        let _e6542 = local_101;
                                        let _e6545 = local_102;
                                        let _e6548 = local_103;
                                        let _e6554 = local_104;
                                        phi_19306_ = type_69(type_35(_e320.member, _e320.member_1, vec4<f32>(_e1406.x, _e1406.y, _e1406.z, f32(_e347)), vec4<f32>(_e6542.x, _e6545.y, _e6548.z, f32(_e6539)), vec4<u32>(_e5533, (_e2411 & 4294967294u), 0u, 0u)), _e1408, _e6554);
                                        phi_19307_ = type_69();
                                        phi_19308_ = true;
                                    } else {
                                        let _e5538 = ((_e5533 * 747796405u) + 2891336453u);
                                        let _e5546 = (((_e5538 >> bitcast<u32>((((_e5538 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5538) * 277803737u);
                                        let _e5549 = ((_e5546 >> bitcast<u32>(22i)) ^ _e5546);
                                        let _e5552 = f32((_e5549 >> bitcast<u32>(8i)));
                                        let _e5555 = ((_e5549 * 747796405u) + 2891336453u);
                                        let _e5563 = (((_e5555 >> bitcast<u32>((((_e5555 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5555) * 277803737u);
                                        let _e5566 = ((_e5563 >> bitcast<u32>(22i)) ^ _e5563);
                                        let _e5569 = f32((_e5566 >> bitcast<u32>(8i)));
                                        let _e5571 = ((_e5566 * 747796405u) + 2891336453u);
                                        let _e5579 = (((_e5571 >> bitcast<u32>((((_e5571 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5571) * 277803737u);
                                        let _e5582 = ((_e5579 >> bitcast<u32>(22i)) ^ _e5579);
                                        let _e5587 = (_e2403 == 0u);
                                        if _e5587 {
                                            phi_29509_ = vec3<f32>();
                                            phi_29510_ = true;
                                        } else {
                                            if (_e2403 == 2u) {
                                                let _e5593 = (((_e1483 * _e1511.x) + (_e1484 * _e1511.y)) + (_e1485 * _e1511.z));
                                                if (_e5593 != _e5593) {
                                                    phi_29550_ = true;
                                                } else {
                                                    phi_29550_ = (0f >= _e5593);
                                                }
                                                let _e5597 = phi_29550_;
                                                let _e5598 = select(_e5593, 0f, _e5597);
                                                let _e5599 = (1f / _e2393);
                                                let _e5603 = ((_e5599 * _e5599) * (1f - (_e5598 * _e5598)));
                                                if (_e5603 >= 1f) {
                                                    phi_29569_ = 1f;
                                                } else {
                                                    let _e5606 = sqrt((1f - _e5603));
                                                    if (_e5598 != _e5598) {
                                                        phi_29580_ = true;
                                                    } else {
                                                        phi_29580_ = (_e5606 <= _e5598);
                                                    }
                                                    let _e5610 = phi_29580_;
                                                    let _e5615 = pow(((1f - _e2393) / (1f + _e2393)), 2f);
                                                    phi_29569_ = (_e5615 + ((1f - _e5615) * pow((1f - select(_e5598, _e5606, _e5610)), 5f)));
                                                }
                                                let _e5622 = phi_29569_;
                                                phi_29500_ = select(true, false, ((f32((_e5582 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5622));
                                            } else {
                                                phi_29500_ = true;
                                            }
                                            let _e5626 = phi_29500_;
                                            if _e5626 {
                                                let _e5631 = (((_e1483 * _e1559) + (_e1484 * _e1560)) + (_e1485 * _e1562));
                                                let _e5636 = (((_e1483 * _e1555) + (_e1484 * _e1565)) + (_e1485 * _e1566));
                                                let _e5641 = (((_e1483 * _e1511.x) + (_e1484 * _e1511.y)) + (_e1485 * _e1511.z));
                                                if (_e5641 <= 0f) {
                                                    phi_29734_ = vec3<f32>();
                                                    phi_29735_ = true;
                                                } else {
                                                    let _e5644 = (_e2401 * _e5631);
                                                    let _e5645 = (_e2401 * _e5636);
                                                    let _e5652 = (1f / sqrt((((_e5644 * _e5644) + (_e5645 * _e5645)) + (_e5641 * _e5641))));
                                                    let _e5653 = (_e5644 * _e5652);
                                                    let _e5654 = (_e5645 * _e5652);
                                                    let _e5655 = (_e5641 * _e5652);
                                                    let _e5658 = ((_e5653 * _e5653) + (_e5654 * _e5654));
                                                    if (_e5658 > 0f) {
                                                        let _e5661 = sqrt(_e5658);
                                                        phi_29636_ = vec3<f32>((-(_e5654) / _e5661), (_e5653 / _e5661), 0f);
                                                    } else {
                                                        phi_29636_ = vec3<f32>(1f, 0f, 0f);
                                                    }
                                                    let _e5666 = phi_29636_;
                                                    let _e5679 = sqrt((_e5552 * 0.000000059604645f));
                                                    let _e5680 = (_e5569 * 0.00000037450704f);
                                                    let _e5682 = (_e5679 * cos(_e5680));
                                                    let _e5684 = (0.5f * (1f + _e5655));
                                                    let _e5687 = (1f - (_e5682 * _e5682));
                                                    if (_e5687 != _e5687) {
                                                        phi_29792_ = true;
                                                    } else {
                                                        phi_29792_ = (0f >= _e5687);
                                                    }
                                                    let _e5691 = phi_29792_;
                                                    let _e5698 = (((1f - _e5684) * sqrt(select(_e5687, 0f, _e5691))) + ((_e5684 * _e5679) * sin(_e5680)));
                                                    let _e5709 = (_e5687 - (_e5698 * _e5698));
                                                    if (_e5709 != _e5709) {
                                                        phi_29807_ = true;
                                                    } else {
                                                        phi_29807_ = (0f >= _e5709);
                                                    }
                                                    let _e5713 = phi_29807_;
                                                    let _e5715 = sqrt(select(_e5709, 0f, _e5713));
                                                    let _e5721 = (((_e5666.z * _e5682) + (((_e5653 * _e5666.y) - (_e5666.x * _e5654)) * _e5698)) + (_e5655 * _e5715));
                                                    let _e5722 = (_e2401 * (((_e5666.x * _e5682) + (((_e5654 * _e5666.z) - (_e5666.y * _e5655)) * _e5698)) + (_e5653 * _e5715)));
                                                    let _e5723 = (_e2401 * (((_e5666.y * _e5682) + (((_e5655 * _e5666.x) - (_e5666.z * _e5653)) * _e5698)) + (_e5654 * _e5715)));
                                                    if (_e5721 != _e5721) {
                                                        phi_29822_ = true;
                                                    } else {
                                                        phi_29822_ = (0f >= _e5721);
                                                    }
                                                    let _e5727 = phi_29822_;
                                                    let _e5728 = select(_e5721, 0f, _e5727);
                                                    let _e5735 = (1f / sqrt((((_e5722 * _e5722) + (_e5723 * _e5723)) + (_e5728 * _e5728))));
                                                    let _e5736 = (_e5722 * _e5735);
                                                    let _e5737 = (_e5723 * _e5735);
                                                    let _e5738 = (_e5728 * _e5735);
                                                    let _e5744 = (2f * (((_e5631 * _e5736) + (_e5636 * _e5737)) + (_e5641 * _e5738)));
                                                    let _e5747 = (_e5738 * _e5744);
                                                    let _e5750 = (vec3<f32>((_e5736 * _e5744), (_e5737 * _e5744), _e5747) - vec3<f32>(_e5631, _e5636, _e5641));
                                                    let _e5751 = ((_e5747 - _e5641) <= 0f);
                                                    if _e5751 {
                                                        phi_29731_ = vec3<f32>();
                                                    } else {
                                                        phi_29731_ = vec3<f32>((((_e1559 * _e5750.x) + (_e1555 * _e5750.y)) + (_e1511.x * _e5750.z)), (((_e1560 * _e5750.x) + (_e1565 * _e5750.y)) + (_e1511.y * _e5750.z)), (((_e1562 * _e5750.x) + (_e1566 * _e5750.y)) + (_e1511.z * _e5750.z)));
                                                    }
                                                    let _e5772 = phi_29731_;
                                                    phi_29734_ = _e5772;
                                                    phi_29735_ = _e5751;
                                                }
                                                let _e5774 = phi_29734_;
                                                let _e5776 = phi_29735_;
                                                phi_29505_ = select(_e5774, vec3<f32>(0f, 0f, 0f), vec3(_e5776));
                                            } else {
                                                phi_29505_ = vec3<f32>();
                                            }
                                            let _e5780 = phi_29505_;
                                            phi_29509_ = _e5780;
                                            phi_29510_ = select(true, false, _e5626);
                                        }
                                        let _e5783 = phi_29509_;
                                        let _e5785 = phi_29510_;
                                        if _e5785 {
                                            let _e5786 = (_e5552 * 0.00000037450704f);
                                            let _e5788 = ((_e5569 * 0.00000011920929f) - 1f);
                                            let _e5790 = (1f - (_e5788 * _e5788));
                                            if (_e5790 != _e5790) {
                                                phi_29922_ = true;
                                            } else {
                                                phi_29922_ = (0f >= _e5790);
                                            }
                                            let _e5794 = phi_29922_;
                                            let _e5796 = sqrt(select(_e5790, 0f, _e5794));
                                            let _e5801 = (_e1511.x + (_e5796 * cos(_e5786)));
                                            let _e5802 = (_e1511.y + (_e5796 * sin(_e5786)));
                                            let _e5803 = (_e1511.z + _e5788);
                                            let _e5809 = sqrt((((_e5801 * _e5801) + (_e5802 * _e5802)) + (_e5803 * _e5803)));
                                            if (_e5809 < 0.000001f) {
                                                phi_29896_ = _e1511;
                                            } else {
                                                let _e5811 = (1f / _e5809);
                                                phi_29896_ = vec3<f32>((_e5801 * _e5811), (_e5802 * _e5811), (_e5803 * _e5811));
                                            }
                                            let _e5817 = phi_29896_;
                                            phi_29517_ = _e5817;
                                        } else {
                                            phi_29517_ = _e5783;
                                        }
                                        let _e5819 = phi_29517_;
                                        let _e5827 = (((_e5819.x * _e1511.x) + (_e5819.y * _e1511.y)) + (_e5819.z * _e1511.z));
                                        if (_e5827 != _e5827) {
                                            phi_29937_ = true;
                                        } else {
                                            phi_29937_ = (0f >= _e5827);
                                        }
                                        let _e5831 = phi_29937_;
                                        let _e5833 = (select(_e5827, 0f, _e5831) * 0.31830987f);
                                        if _e5587 {
                                            if (_e5827 <= 0f) {
                                                phi_30734_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_30734_ = (_e2397 * (_e5827 * 0.31830987f));
                                            }
                                            let _e6308 = phi_30734_;
                                            phi_19147_ = type_57(_e6308, _e5833);
                                            phi_19148_ = type_57();
                                            phi_19149_ = true;
                                        } else {
                                            let _e5838 = (((_e1483 * _e1559) + (_e1484 * _e1560)) + (_e1485 * _e1562));
                                            let _e5843 = (((_e1483 * _e1555) + (_e1484 * _e1565)) + (_e1485 * _e1566));
                                            let _e5848 = (((_e1483 * _e1511.x) + (_e1484 * _e1511.y)) + (_e1485 * _e1511.z));
                                            let _e5853 = (((_e5819.x * _e1559) + (_e5819.y * _e1560)) + (_e5819.z * _e1562));
                                            let _e5858 = (((_e5819.x * _e1555) + (_e5819.y * _e1565)) + (_e5819.z * _e1566));
                                            if (_e5848 <= 0f) {
                                                phi_19111_ = type_57();
                                                phi_19112_ = true;
                                            } else {
                                                let _e5860 = (_e5827 <= 0f);
                                                if _e5860 {
                                                    phi_19110_ = type_57();
                                                } else {
                                                    let _e5861 = (_e5838 + _e5853);
                                                    let _e5862 = (_e5843 + _e5858);
                                                    let _e5863 = (_e5848 + _e5827);
                                                    let _e5870 = (1f / sqrt((((_e5861 * _e5861) + (_e5862 * _e5862)) + (_e5863 * _e5863))));
                                                    let _e5873 = (_e5863 * _e5870);
                                                    if (_e5873 <= 0f) {
                                                        phi_30047_ = 0f;
                                                    } else {
                                                        let _e5875 = (_e2401 * _e2401);
                                                        let _e5879 = ((((_e5875 - 1f) * _e5873) * _e5873) + 1f);
                                                        phi_30047_ = (_e5875 / ((3.1415927f * _e5879) * _e5879));
                                                    }
                                                    let _e5884 = phi_30047_;
                                                    let _e5885 = (_e5848 * _e5848);
                                                    if (_e5885 >= 1f) {
                                                        phi_30075_ = 0f;
                                                    } else {
                                                        phi_30075_ = ((sqrt((1f + (((_e2401 * _e2401) * ((_e5838 * _e5838) + (_e5843 * _e5843))) / _e5885))) - 1f) * 0.5f);
                                                    }
                                                    let _e5898 = phi_30075_;
                                                    let _e5899 = (1f + _e5898);
                                                    let _e5900 = (_e5827 * _e5827);
                                                    if (_e5900 >= 1f) {
                                                        phi_30103_ = 0f;
                                                    } else {
                                                        phi_30103_ = ((sqrt((1f + (((_e2401 * _e2401) * ((_e5853 * _e5853) + (_e5858 * _e5858))) / _e5900))) - 1f) * 0.5f);
                                                    }
                                                    let _e5913 = phi_30103_;
                                                    let _e5920 = (((_e5838 * (_e5861 * _e5870)) + (_e5843 * (_e5862 * _e5870))) + (_e5848 * _e5873));
                                                    if (_e2399 == 0u) {
                                                        let _e6215 = (1f - _e5920);
                                                        if (_e6215 != _e6215) {
                                                            phi_30606_ = true;
                                                        } else {
                                                            phi_30606_ = (0f >= _e6215);
                                                        }
                                                        let _e6219 = phi_30606_;
                                                        let _e6221 = pow(select(_e6215, 0f, _e6219), 5f);
                                                        let _e6222 = (1f - _e6221);
                                                        phi_19098_ = vec3<f32>(((_e2397.x * _e6222) + _e6221), ((_e2397.y * _e6222) + _e6221), ((_e2397.z * _e6222) + _e6221));
                                                    } else {
                                                        if (_e2399 == 1u) {
                                                            let _e6188 = (1f / _e2393);
                                                            let _e6192 = ((_e6188 * _e6188) * (1f - (_e5920 * _e5920)));
                                                            if (_e6192 >= 1f) {
                                                                phi_30567_ = 1f;
                                                            } else {
                                                                let _e6195 = sqrt((1f - _e6192));
                                                                if (_e5920 != _e5920) {
                                                                    phi_30578_ = true;
                                                                } else {
                                                                    phi_30578_ = (_e6195 <= _e5920);
                                                                }
                                                                let _e6199 = phi_30578_;
                                                                let _e6204 = pow(((1f - _e2393) / (1f + _e2393)), 2f);
                                                                phi_30567_ = (_e6204 + ((1f - _e6204) * pow((1f - select(_e5920, _e6195, _e6199)), 5f)));
                                                            }
                                                            let _e6211 = phi_30567_;
                                                            phi_19097_ = vec3(_e6211);
                                                        } else {
                                                            let _e5924 = select(_e5920, 0f, (_e5920 < 0f));
                                                            let _e5926 = select(_e5924, 1f, (_e5924 > 1f));
                                                            let _e5928 = (1f - (_e5926 * _e5926));
                                                            let _e5941 = ((_e2397.x * _e2397.x) - (_e2395.x * _e2395.x));
                                                            let _e5942 = ((_e2397.y * _e2397.y) - (_e2395.y * _e2395.y));
                                                            let _e5943 = ((_e2397.z * _e2397.z) - (_e2395.z * _e2395.z));
                                                            let _e5945 = (_e2397.x * (_e2395.x + _e2395.x));
                                                            let _e5947 = (_e2397.y * (_e2395.y + _e2395.y));
                                                            let _e5949 = (_e2397.z * (_e2395.z + _e2395.z));
                                                            let _e5956 = ((_e5941 * _e5941) + (_e5945 * _e5945));
                                                            let _e5957 = ((_e5942 * _e5942) + (_e5947 * _e5947));
                                                            let _e5958 = ((_e5943 * _e5943) + (_e5949 * _e5949));
                                                            let _e5963 = ((_e5928 * _e5941) / _e5956);
                                                            let _e5964 = ((_e5928 * _e5942) / _e5957);
                                                            let _e5965 = ((_e5928 * _e5943) / _e5958);
                                                            let _e5966 = (1f - _e5963);
                                                            let _e5967 = (1f - _e5964);
                                                            let _e5968 = (1f - _e5965);
                                                            let _e5970 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5963, _e5964, _e5965));
                                                            let _e5971 = (_e5928 * _e5945);
                                                            let _e5972 = (_e5928 * _e5947);
                                                            let _e5973 = (_e5928 * _e5949);
                                                            let _e5978 = (vec3<f32>(_e5971, _e5972, _e5973) / vec3<f32>(_e5956, _e5957, _e5958));
                                                            let _e5991 = ((_e5970.x * _e5970.x) + (_e5978.x * _e5978.x));
                                                            let _e5992 = ((_e5970.y * _e5970.y) + (_e5978.y * _e5978.y));
                                                            let _e5993 = ((_e5970.z * _e5970.z) + (_e5978.z * _e5978.z));
                                                            if (_e5991 != _e5991) {
                                                                phi_30237_ = true;
                                                            } else {
                                                                phi_30237_ = (0f >= _e5991);
                                                            }
                                                            let _e5997 = phi_30237_;
                                                            let _e5999 = sqrt(select(_e5991, 0f, _e5997));
                                                            if (_e5992 != _e5992) {
                                                                phi_30252_ = true;
                                                            } else {
                                                                phi_30252_ = (0f >= _e5992);
                                                            }
                                                            let _e6003 = phi_30252_;
                                                            let _e6005 = sqrt(select(_e5992, 0f, _e6003));
                                                            if (_e5993 != _e5993) {
                                                                phi_30267_ = true;
                                                            } else {
                                                                phi_30267_ = (0f >= _e5993);
                                                            }
                                                            let _e6009 = phi_30267_;
                                                            let _e6011 = sqrt(select(_e5993, 0f, _e6009));
                                                            let _e6015 = ((_e5999 + _e5966) * 0.5f);
                                                            let _e6016 = ((_e6005 + _e5967) * 0.5f);
                                                            let _e6017 = ((_e6011 + _e5968) * 0.5f);
                                                            if (_e6015 != _e6015) {
                                                                phi_30294_ = true;
                                                            } else {
                                                                phi_30294_ = (0f >= _e6015);
                                                            }
                                                            let _e6021 = phi_30294_;
                                                            let _e6023 = sqrt(select(_e6015, 0f, _e6021));
                                                            if (_e6016 != _e6016) {
                                                                phi_30309_ = true;
                                                            } else {
                                                                phi_30309_ = (0f >= _e6016);
                                                            }
                                                            let _e6027 = phi_30309_;
                                                            let _e6029 = sqrt(select(_e6016, 0f, _e6027));
                                                            if (_e6017 != _e6017) {
                                                                phi_30324_ = true;
                                                            } else {
                                                                phi_30324_ = (0f >= _e6017);
                                                            }
                                                            let _e6033 = phi_30324_;
                                                            let _e6035 = sqrt(select(_e6017, 0f, _e6033));
                                                            let _e6036 = vec3<f32>(_e6023, _e6029, _e6035);
                                                            let _e6040 = ((_e5999 - _e5966) * 0.5f);
                                                            let _e6041 = ((_e6005 - _e5967) * 0.5f);
                                                            let _e6042 = ((_e6011 - _e5968) * 0.5f);
                                                            if (_e6040 != _e6040) {
                                                                phi_30351_ = true;
                                                            } else {
                                                                phi_30351_ = (0f >= _e6040);
                                                            }
                                                            let _e6046 = phi_30351_;
                                                            let _e6048 = sqrt(select(_e6040, 0f, _e6046));
                                                            if (_e6041 != _e6041) {
                                                                phi_30366_ = true;
                                                            } else {
                                                                phi_30366_ = (0f >= _e6041);
                                                            }
                                                            let _e6052 = phi_30366_;
                                                            let _e6054 = sqrt(select(_e6041, 0f, _e6052));
                                                            if (_e6042 != _e6042) {
                                                                phi_30381_ = true;
                                                            } else {
                                                                phi_30381_ = (0f >= _e6042);
                                                            }
                                                            let _e6058 = phi_30381_;
                                                            let _e6060 = sqrt(select(_e6042, 0f, _e6058));
                                                            if ((_e5971 / _e5956) < 0f) {
                                                                phi_18974_ = -(_e6048);
                                                            } else {
                                                                phi_18974_ = _e6048;
                                                            }
                                                            let _e6064 = phi_18974_;
                                                            if ((_e5972 / _e5957) < 0f) {
                                                                phi_18982_ = -(_e6054);
                                                            } else {
                                                                phi_18982_ = _e6054;
                                                            }
                                                            let _e6068 = phi_18982_;
                                                            if ((_e5973 / _e5958) < 0f) {
                                                                phi_18990_ = -(_e6060);
                                                            } else {
                                                                phi_18990_ = _e6060;
                                                            }
                                                            let _e6072 = phi_18990_;
                                                            let _e6076 = (_e2395.x * _e5926);
                                                            let _e6077 = (_e2395.y * _e5926);
                                                            let _e6078 = (_e2395.z * _e5926);
                                                            let _e6088 = vec3<f32>(((_e2397.x * _e6023) - (_e2395.x * _e6064)), ((_e2397.y * _e6029) - (_e2395.y * _e6068)), ((_e2397.z * _e6035) - (_e2395.z * _e6072)));
                                                            let _e6095 = ((_e2397.x * _e6064) + (_e2395.x * _e6023));
                                                            let _e6096 = ((_e2397.y * _e6068) + (_e2395.y * _e6029));
                                                            let _e6097 = ((_e2397.z * _e6072) + (_e2395.z * _e6035));
                                                            let _e6099 = vec3<f32>((_e2397.x * _e5926), (_e2397.y * _e5926), (_e2397.z * _e5926));
                                                            let _e6100 = (_e6099 - _e6036);
                                                            let _e6101 = (_e6076 - _e6064);
                                                            let _e6102 = (_e6077 - _e6068);
                                                            let _e6103 = (_e6078 - _e6072);
                                                            let _e6116 = (_e6099 + _e6036);
                                                            let _e6117 = (_e6076 + _e6064);
                                                            let _e6118 = (_e6077 + _e6068);
                                                            let _e6119 = (_e6078 + _e6072);
                                                            let _e6129 = ((_e6116.x * _e6116.x) + (_e6117 * _e6117));
                                                            let _e6130 = ((_e6116.y * _e6116.y) + (_e6118 * _e6118));
                                                            let _e6131 = ((_e6116.z * _e6116.z) + (_e6119 * _e6119));
                                                            let _e6141 = vec3(_e5926);
                                                            let _e6142 = (_e6141 - _e6088);
                                                            let _e6143 = -(vec3<f32>(_e6095, _e6096, _e6097));
                                                            let _e6159 = (_e6141 + _e6088);
                                                            let _e6169 = ((_e6159.x * _e6159.x) + (_e6095 * _e6095));
                                                            let _e6170 = ((_e6159.y * _e6159.y) + (_e6096 * _e6096));
                                                            let _e6171 = ((_e6159.z * _e6159.z) + (_e6097 * _e6097));
                                                            phi_19097_ = vec3<f32>((((((_e6100.x * _e6100.x) + (_e6101 * _e6101)) / select(0.000000000000000000000000000001f, _e6129, (_e6129 > 0.000000000000000000000000000001f))) + (((_e6142.x * _e6142.x) + (_e6143.x * _e6143.x)) / select(0.000000000000000000000000000001f, _e6169, (_e6169 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6100.y * _e6100.y) + (_e6102 * _e6102)) / select(0.000000000000000000000000000001f, _e6130, (_e6130 > 0.000000000000000000000000000001f))) + (((_e6142.y * _e6142.y) + (_e6143.y * _e6143.y)) / select(0.000000000000000000000000000001f, _e6170, (_e6170 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6100.z * _e6100.z) + (_e6103 * _e6103)) / select(0.000000000000000000000000000001f, _e6131, (_e6131 > 0.000000000000000000000000000001f))) + (((_e6142.z * _e6142.z) + (_e6143.z * _e6143.z)) / select(0.000000000000000000000000000001f, _e6171, (_e6171 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e6214 = phi_19097_;
                                                        phi_19098_ = _e6214;
                                                    }
                                                    let _e6234 = phi_19098_;
                                                    let _e6236 = (4f * _e5848);
                                                    phi_19110_ = type_57((_e6234 * (((1f / (_e5899 + _e5913)) * _e5884) / _e6236)), ((_e5884 / _e5899) / _e6236));
                                                }
                                                let _e6243 = phi_19110_;
                                                phi_19111_ = _e6243;
                                                phi_19112_ = _e5860;
                                            }
                                            let _e6245 = phi_19111_;
                                            let _e6247 = phi_19112_;
                                            if _e6247 {
                                                phi_19117_ = type_57(vec3<f32>(0f, 0f, 0f), 0f);
                                            } else {
                                                phi_19117_ = _e6245;
                                            }
                                            let _e6249 = phi_19117_;
                                            let _e6252 = (_e2403 == 1u);
                                            if _e6252 {
                                                phi_19145_ = _e6249;
                                                phi_19146_ = type_57();
                                            } else {
                                                if (_e5848 != _e5848) {
                                                    phi_30643_ = true;
                                                } else {
                                                    phi_30643_ = (0f >= _e5848);
                                                }
                                                let _e6256 = phi_30643_;
                                                let _e6257 = select(_e5848, 0f, _e6256);
                                                let _e6258 = (1f / _e2393);
                                                let _e6262 = ((_e6258 * _e6258) * (1f - (_e6257 * _e6257)));
                                                if (_e6262 >= 1f) {
                                                    phi_30662_ = 1f;
                                                } else {
                                                    let _e6265 = sqrt((1f - _e6262));
                                                    if (_e6257 != _e6257) {
                                                        phi_30673_ = true;
                                                    } else {
                                                        phi_30673_ = (_e6265 <= _e6257);
                                                    }
                                                    let _e6269 = phi_30673_;
                                                    let _e6274 = pow(((1f - _e2393) / (1f + _e2393)), 2f);
                                                    phi_30662_ = (_e6274 + ((1f - _e6274) * pow((1f - select(_e6257, _e6265, _e6269)), 5f)));
                                                }
                                                let _e6281 = phi_30662_;
                                                if (_e5827 <= 0f) {
                                                    phi_30712_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_30712_ = (_e2391 * (_e5827 * 0.31830987f));
                                                }
                                                let _e6286 = phi_30712_;
                                                let _e6287 = (1f - _e6281);
                                                phi_19145_ = type_57();
                                                phi_19146_ = type_57((_e6249.member + vec3<f32>((_e6286.x * _e6287), (_e6286.y * _e6287), (_e6286.z * _e6287))), ((_e6281 * _e6249.member_1) + (_e6287 * _e5833)));
                                            }
                                            let _e6301 = phi_19145_;
                                            let _e6303 = phi_19146_;
                                            phi_19147_ = _e6301;
                                            phi_19148_ = _e6303;
                                            phi_19149_ = _e6252;
                                        }
                                        let _e6311 = phi_19147_;
                                        let _e6313 = phi_19148_;
                                        let _e6315 = phi_19149_;
                                        if _e6315 {
                                            phi_19153_ = _e6311;
                                        } else {
                                            phi_19153_ = _e6313;
                                        }
                                        let _e6317 = phi_19153_;
                                        if ((((_e5819.x * _e5819.x) + (_e5819.y * _e5819.y)) + (_e5819.z * _e5819.z)) == 0f) {
                                            phi_36132_ = _e5582;
                                            phi_19292_ = type_69();
                                            phi_19293_ = type_69();
                                            phi_19294_ = true;
                                            phi_19295_ = false;
                                        } else {
                                            let _e6326 = (_e6317.member_1 <= 0f);
                                            if _e6326 {
                                                phi_36133_ = _e5582;
                                                phi_19289_ = type_69();
                                                phi_19290_ = type_69();
                                                phi_19291_ = false;
                                            } else {
                                                let _e6328 = (_e6317.member.x / _e6317.member_1);
                                                let _e6330 = (_e6317.member.y / _e6317.member_1);
                                                let _e6332 = (_e6317.member.z / _e6317.member_1);
                                                let _e6341 = global_1.member.member_4[1u];
                                                if (sqrt((((_e6328 * _e6328) + (_e6330 * _e6330)) + (_e6332 * _e6332))) <= _e6341) {
                                                    phi_36126_ = _e5582;
                                                    phi_19240_ = _e1406;
                                                    phi_19241_ = false;
                                                } else {
                                                    let _e6345 = global_1.member.member_7[1u];
                                                    if (_e6345 == 1u) {
                                                        let _e6350 = global_1.member.member_4[3u];
                                                        if (f32(_e347) >= _e6350) {
                                                            let _e6353 = select(_e6330, _e6328, (_e6328 > _e6330));
                                                            let _e6355 = select(_e6332, _e6353, (_e6353 > _e6332));
                                                            if (_e6355 != _e6355) {
                                                                phi_30759_ = true;
                                                            } else {
                                                                phi_30759_ = (1f <= _e6355);
                                                            }
                                                            let _e6359 = phi_30759_;
                                                            let _e6360 = select(_e6355, 1f, _e6359);
                                                            let _e6362 = ((_e5582 * 747796405u) + 2891336453u);
                                                            let _e6370 = (((_e6362 >> bitcast<u32>((((_e6362 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6362) * 277803737u);
                                                            let _e6373 = ((_e6370 >> bitcast<u32>(22i)) ^ _e6370);
                                                            let _e6378 = ((f32((_e6373 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6360);
                                                            if _e6378 {
                                                                phi_19229_ = vec3<f32>();
                                                            } else {
                                                                phi_19229_ = vec3<f32>((_e1406.x / _e6360), (_e1406.y / _e6360), (_e1406.z / _e6360));
                                                            }
                                                            let _e6387 = phi_19229_;
                                                            phi_36129_ = _e6373;
                                                            phi_19231_ = _e6387;
                                                            phi_19232_ = select(true, false, _e6378);
                                                        } else {
                                                            phi_36129_ = _e5582;
                                                            phi_19231_ = _e1406;
                                                            phi_19232_ = true;
                                                        }
                                                        let _e6390 = phi_36129_;
                                                        let _e6392 = phi_19231_;
                                                        let _e6394 = phi_19232_;
                                                        phi_36128_ = _e6390;
                                                        phi_19233_ = _e6392;
                                                        phi_19234_ = _e6394;
                                                    } else {
                                                        phi_36128_ = _e5582;
                                                        phi_19233_ = _e1406;
                                                        phi_19234_ = true;
                                                    }
                                                    let _e6396 = phi_36128_;
                                                    let _e6398 = phi_19233_;
                                                    let _e6400 = phi_19234_;
                                                    phi_36126_ = _e6396;
                                                    phi_19240_ = select(_e1406, _e6398, vec3(_e6400));
                                                    phi_19241_ = select(false, true, _e6400);
                                                }
                                                let _e6405 = phi_36126_;
                                                let _e6407 = phi_19240_;
                                                let _e6409 = phi_19241_;
                                                if _e6409 {
                                                    if ((((_e5819.x * _e1498.x) + (_e5819.y * _e1498.y)) + (_e5819.z * _e1498.z)) < 0f) {
                                                        phi_19270_ = -(_e1498);
                                                    } else {
                                                        phi_19270_ = _e1498;
                                                    }
                                                    let _e6448 = phi_19270_;
                                                    let _e6465 = local_90;
                                                    let _e6468 = local_91;
                                                    let _e6471 = local_92;
                                                    let _e6474 = local_93;
                                                    let _e6480 = local_94;
                                                    phi_19286_ = type_69(type_35(vec4<f32>((_e1413 + (_e6448.x * _e1529)), (_e1414 + (_e6448.y * _e1529)), (_e1415 + (_e6448.z * _e1529)), _e320.member.w), vec4<f32>(_e5819.x, _e5819.y, _e5819.z, _e6317.member_1), vec4<f32>((_e6407.x * _e6328), (_e6407.y * _e6330), (_e6407.z * _e6332), f32((_e347 + 1u))), vec4<f32>(_e6468.x, _e6471.y, _e6474.z, f32(_e6465)), vec4<u32>(_e6405, _e2411, 0u, 0u)), _e1408, _e6480);
                                                    phi_19287_ = type_69();
                                                } else {
                                                    let _e6417 = local_85;
                                                    let _e6420 = local_86;
                                                    let _e6423 = local_87;
                                                    let _e6426 = local_88;
                                                    let _e6432 = local_89;
                                                    phi_19286_ = type_69();
                                                    phi_19287_ = type_69(type_35(_e320.member, _e320.member_1, vec4<f32>(_e6407.x, _e6407.y, _e6407.z, f32(_e347)), vec4<f32>(_e6420.x, _e6423.y, _e6426.z, f32(_e6417)), vec4<u32>(_e6405, (_e2411 & 4294967294u), 0u, 0u)), _e1408, _e6432);
                                                }
                                                let _e6483 = phi_19286_;
                                                let _e6485 = phi_19287_;
                                                phi_36133_ = _e6405;
                                                phi_19289_ = _e6483;
                                                phi_19290_ = _e6485;
                                                phi_19291_ = select(true, false, _e6409);
                                            }
                                            let _e6488 = phi_36133_;
                                            let _e6490 = phi_19289_;
                                            let _e6492 = phi_19290_;
                                            let _e6494 = phi_19291_;
                                            phi_36132_ = _e6488;
                                            phi_19292_ = _e6490;
                                            phi_19293_ = _e6492;
                                            phi_19294_ = _e6326;
                                            phi_19295_ = _e6494;
                                        }
                                        let _e6496 = phi_36132_;
                                        let _e6498 = phi_19292_;
                                        let _e6500 = phi_19293_;
                                        let _e6502 = phi_19294_;
                                        let _e6504 = phi_19295_;
                                        if _e6502 {
                                            let _e6512 = local_95;
                                            let _e6515 = local_96;
                                            let _e6518 = local_97;
                                            let _e6521 = local_98;
                                            let _e6527 = local_99;
                                            phi_19301_ = type_69(type_35(_e320.member, _e320.member_1, vec4<f32>(_e1406.x, _e1406.y, _e1406.z, f32(_e347)), vec4<f32>(_e6515.x, _e6518.y, _e6521.z, f32(_e6512)), vec4<u32>(_e6496, (_e2411 & 4294967294u), 0u, 0u)), _e1408, _e6527);
                                        } else {
                                            phi_19301_ = _e6500;
                                        }
                                        let _e6530 = phi_19301_;
                                        phi_19306_ = _e6530;
                                        phi_19307_ = _e6498;
                                        phi_19308_ = select(_e6504, true, _e6502);
                                    }
                                    let _e6557 = phi_19306_;
                                    let _e6559 = phi_19307_;
                                    let _e6561 = phi_19308_;
                                    phi_19312_ = _e6557;
                                    phi_19313_ = _e6559;
                                    phi_19314_ = _e6561;
                                } else {
                                    phi_19312_ = _e2407;
                                    phi_19313_ = type_69();
                                    phi_19314_ = true;
                                }
                                let _e6563 = phi_19312_;
                                let _e6565 = phi_19313_;
                                let _e6567 = phi_19314_;
                                phi_19318_ = _e6563;
                                phi_19319_ = _e6565;
                                phi_19320_ = _e6567;
                            }
                            let _e6779 = phi_19318_;
                            let _e6781 = phi_19319_;
                            let _e6783 = phi_19320_;
                            phi_19324_ = _e6779;
                            phi_19325_ = _e6781;
                            phi_19326_ = _e6783;
                        }
                        let _e7291 = phi_19324_;
                        let _e7293 = phi_19325_;
                        let _e7295 = phi_19326_;
                        if _e7295 {
                            phi_19330_ = _e7291;
                        } else {
                            phi_19330_ = _e7293;
                        }
                        let _e7297 = phi_19330_;
                        if ((((_e7297.member_1.member_2.x * _e7297.member_1.member_2.x) + (_e7297.member_1.member_2.y * _e7297.member_1.member_2.y)) + (_e7297.member_1.member_2.z * _e7297.member_1.member_2.z)) > 0f) {
                            phi_19368_ = true;
                        } else {
                            phi_19368_ = ((((_e7297.member_2.member_2.x * _e7297.member_2.member_2.x) + (_e7297.member_2.member_2.y * _e7297.member_2.member_2.y)) + (_e7297.member_2.member_2.z * _e7297.member_2.member_2.z)) > 0f);
                        }
                        let _e7322 = phi_19368_;
                        if _e7322 {
                            phi_19375_ = type_35(_e7297.member.member, _e7297.member.member_1, _e7297.member.member_2, _e7297.member.member_3, vec4<u32>(_e7297.member.member_4.x, (_e7297.member.member_4.y | 16u), _e7297.member.member_4.z, _e7297.member.member_4.w));
                        } else {
                            phi_19375_ = _e7297.member;
                        }
                        let _e7338 = phi_19375_;
                        phi_19376_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_19377_ = _e7297.member_2;
                        phi_19378_ = _e7297.member_1;
                        phi_19379_ = _e7338;
                    }
                    let _e7385 = phi_19376_;
                    let _e7387 = phi_19377_;
                    let _e7389 = phi_19378_;
                    let _e7391 = phi_19379_;
                    if _e310 {
                    } else {
                        break;
                    }
                    global_7.member[_e305.member_1] = _e7391;
                    if ((_e7391.member_4.y & 16u) == 0u) {
                    } else {
                        let _e7396 = (2u * _e305.member_1);
                        if (_e7396 < _e285) {
                        } else {
                            break;
                        }
                        global_8.member[_e7396] = _e7389;
                        let _e7400 = (_e7396 + 1u);
                        if (_e7400 < _e285) {
                        } else {
                            break;
                        }
                        global_8.member[_e7400] = _e7387;
                    }
                    if (_e7385.w > 0f) {
                        if (_e305.member_1 < arrayLength((&global_9.member))) {
                        } else {
                            break;
                        }
                        let _e7411 = global_9.member[_e305.member_1][0u];
                        global_9.member[_e305.member_1][0u] = (_e7411 + _e7385.x);
                        let _e7417 = global_9.member[_e305.member_1][1u];
                        global_9.member[_e305.member_1][1u] = (_e7417 + _e7385.y);
                        let _e7423 = global_9.member[_e305.member_1][2u];
                        global_9.member[_e305.member_1][2u] = (_e7423 + _e7385.z);
                        let _e7428 = global_9.member[_e305.member_1][3u];
                        global_9.member[_e305.member_1][3u] = (_e7428 + _e7385.w);
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
